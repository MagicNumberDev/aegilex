use super::*;

impl Runtime {
    pub(crate) fn service_publish(
        &mut self,
        owner: &str,
        name: &str,
        version: &str,
        methods: Vec<String>,
        priority: u32,
    ) -> Result<u64, u32> {
        if name.is_empty() || methods.is_empty() {
            return Err(AEGILEX_INVALID_ARGUMENT);
        }
        let id = self.next_service_id;
        if id == 0 {
            return Err(AEGILEX_LIMIT_EXCEEDED);
        }
        self.next_service_id = self.next_service_id.wrapping_add(1);
        self.services.insert(
            id,
            ServiceEntry {
                name: name.to_owned(),
                version: version.to_owned(),
                methods,
                priority,
                owner: owner.to_owned(),
            },
        );
        Ok(id)
    }

    pub(crate) fn service_unpublish(&mut self, provider_id: u64, caller: &str) -> u32 {
        let Some(entry) = self.services.get(&provider_id) else {
            return AEGILEX_NOT_FOUND;
        };
        if entry.owner != caller {
            return AEGILEX_DENIED;
        }
        self.services.remove(&provider_id);
        AEGILEX_OK
    }

    pub(crate) fn service_list(&self, name: &str) -> Vec<(u64, String, String, Vec<String>, u32)> {
        let mut matches = self
            .services
            .iter()
            .filter(|(_, entry)| name.is_empty() || entry.name == name)
            .collect::<Vec<_>>();
        matches.sort_by_key(|(_, entry)| entry.name.clone());
        matches
            .into_iter()
            .map(|(&id, entry)| {
                (
                    id,
                    entry.name.clone(),
                    entry.version.clone(),
                    entry.methods.clone(),
                    entry.priority,
                )
            })
            .collect()
    }

    pub(crate) fn service_call(
        &mut self,
        caller: &str,
        provider_id: u64,
        method: &str,
        payload: Vec<u8>,
        _deadline: u64,
    ) -> Result<u64, u32> {
        let Some(entry) = self.services.get(&provider_id) else {
            return Err(AEGILEX_NOT_FOUND);
        };
        if !entry.methods.iter().any(|candidate| candidate == method) {
            return Err(AEGILEX_INVALID_ARGUMENT);
        }
        let provider_plugin = entry.owner.clone();
        let call_id = self.next_service_call_id;
        if call_id == 0 {
            return Err(AEGILEX_LIMIT_EXCEEDED);
        }
        self.next_service_call_id = self.next_service_call_id.wrapping_add(1);

        let Some(index) = self
            .plugins
            .iter()
            .position(|plugin| plugin.id == provider_plugin)
        else {
            return Err(AEGILEX_NOT_FOUND);
        };
        let request = crate::core_host::ServiceBusServiceRequest {
            call_id,
            provider_id,
            method: method.to_owned(),
            payload,
            deadline: _deadline,
        };
        let outcome = {
            let plugin = &mut self.plugins[index];
            if !plugin.enabled {
                return Err(AEGILEX_NOT_FOUND);
            }
            if plugin.store.set_fuel(ENABLE_FUEL).is_err() {
                return Err(AEGILEX_INTERNAL_ERROR);
            }
            let invocation_id = self.host.next_invocation_id();
            call_with_invocation(&mut plugin.store, invocation_id, |store| {
                plugin.exports.call_servicecallbacks_on_service_request(
                    &plugin.instance,
                    store,
                    request,
                )
            })
        };
        let state = match outcome {
            Ok(Ok(crate::core_host::ServiceBusServiceResponse::Success(response))) => {
                ServiceCallState {
                    status: SERVICE_STATUS_COMPLETED,
                    payload: response,
                    error: String::new(),
                    taken: false,
                }
            }
            Ok(Ok(crate::core_host::ServiceBusServiceResponse::Rejected(text))) => {
                ServiceCallState {
                    status: SERVICE_STATUS_REJECTED,
                    payload: Vec::new(),
                    error: text,
                    taken: false,
                }
            }
            Ok(Err(text)) => ServiceCallState {
                status: SERVICE_STATUS_REJECTED,
                payload: Vec::new(),
                error: text,
                taken: false,
            },
            Err(error) => ServiceCallState {
                status: SERVICE_STATUS_FAILED,
                payload: Vec::new(),
                error: format!("provider trapped: {error}"),
                taken: false,
            },
        };
        let _ = caller;
        self.service_calls.insert(call_id, state);
        Ok(call_id)
    }

    pub(crate) fn service_call_status(&self, call_id: u64) -> Result<u32, u32> {
        self.service_calls
            .get(&call_id)
            .map(|call| call.status)
            .ok_or(AEGILEX_NOT_FOUND)
    }

    pub(crate) fn service_take_response(
        &mut self,
        call_id: u64,
    ) -> Result<(u32, Vec<u8>, String), u32> {
        let Some(call) = self.service_calls.get_mut(&call_id) else {
            return Err(AEGILEX_NOT_FOUND);
        };
        if call.taken {
            return Err(AEGILEX_NOT_FOUND);
        }
        call.taken = true;
        Ok((
            call.status,
            std::mem::take(&mut call.payload),
            call.error.clone(),
        ))
    }

    pub(crate) fn service_cancel(&mut self, call_id: u64) -> u32 {
        match self.service_calls.get_mut(&call_id) {
            Some(call) if !call.taken => {
                call.status = SERVICE_STATUS_CANCELLED;
                call.payload.clear();
                call.error.clear();
                AEGILEX_OK
            }
            _ => AEGILEX_NOT_FOUND,
        }
    }
}
