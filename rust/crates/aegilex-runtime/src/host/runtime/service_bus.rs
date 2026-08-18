use crate::runtime::PluginStoreState;

impl PluginStoreState {
    pub(crate) fn insert_service_provider(&mut self, rep: u32, provider_id: u64) {
        self.service_providers.insert(rep, provider_id);
    }

    pub(crate) fn insert_service_call(&mut self, rep: u32, call_id: u64) {
        self.service_calls.insert(rep, call_id);
    }
}

/// Runtime service-bus ABI implementation.
use crate::host::endstone::support::*;

impl crate::core_host::imports::HostServiceBus for PluginStoreState {
    fn service_provider_get_spec(
        &mut self,
        self_: u32,
    ) -> Result<Result<ServiceBusServiceProviderInfo, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "service-bus.service-provider.get-spec")?;
            let provider_id = self
                .service_providers
                .get(&self_)
                .copied()
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            let list = native::service_list(&self.host, "").map_err(map_core_host_error)?;
            list.providers
                .into_iter()
                .find(|row| row.id == provider_id)
                .map(service_provider_info_from_cxx)
                .ok_or_else(not_found)
                .map_err(map_core_host_error)
        })())
    }

    fn service_provider_unpublish(
        &mut self,
        self_: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "service-bus.service-provider.unpublish")?;
            let provider_id = self
                .service_providers
                .remove(&self_)
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            native::service_unpublish(&self.host, &self.plugin_id, provider_id)
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn service_call_get_status(
        &mut self,
        self_: u32,
    ) -> Result<Result<ServiceBusServiceCallStatus, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "service-bus.service-call.get-status")?;
            let call_id = self
                .service_calls
                .get(&self_)
                .copied()
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            let result =
                native::service_call_status(&self.host, call_id).map_err(map_core_host_error)?;
            Ok(status_from_value(result.call_status))
        })())
    }

    fn service_call_take_response(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<ServiceBusServiceResponse>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "service-bus.service-call.take-response")?;
            let call_id = self
                .service_calls
                .get(&self_)
                .copied()
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            let result =
                native::service_take_response(&self.host, call_id).map_err(map_core_host_error)?;
            Ok(match result.kind {
                crate::runtime::SERVICE_STATUS_COMPLETED => {
                    Some(ServiceBusServiceResponse::Success(result.payload))
                }
                crate::runtime::SERVICE_STATUS_REJECTED | crate::runtime::SERVICE_STATUS_FAILED => {
                    Some(ServiceBusServiceResponse::Rejected(result.error))
                }
                _ => None,
            })
        })())
    }

    fn service_call_cancel(&mut self, self_: u32) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "service-bus.service-call.cancel")?;
            let call_id = self
                .service_calls
                .get(&self_)
                .copied()
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            native::service_cancel(&self.host, call_id).map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn publish(
        &mut self,
        spec: ServiceBusServiceSpec,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "service-bus.publish")?;
            let provider_id = native::service_publish(
                &self.host,
                &self.plugin_id,
                &spec.name,
                &spec.version,
                &spec.methods,
                priority_value(&spec.priority),
            )
            .map_err(map_core_host_error)?;
            let rep =
                u32::try_from(provider_id).map_err(|_| map_core_host_error(limit_exceeded()))?;
            self.insert_service_provider(rep, provider_id);
            Ok(rep)
        })())
    }

    fn list_providers(
        &mut self,
        name: String,
    ) -> Result<Result<Vec<ServiceBusServiceProviderInfo>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "service-bus.list-providers")?;
            let result = native::service_list(&self.host, &name).map_err(map_core_host_error)?;
            Ok(result
                .providers
                .into_iter()
                .map(service_provider_info_from_cxx)
                .collect())
        })())
    }

    fn call(
        &mut self,
        provider_id: u64,
        method: String,
        payload: Vec<u8>,
        timeout: u64,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "service-bus.call")?;
            let call_id = native::service_call(
                &self.host,
                &self.plugin_id,
                provider_id,
                &method,
                &payload,
                timeout,
            )
            .map_err(map_core_host_error)?;
            let rep = u32::try_from(call_id).map_err(|_| map_core_host_error(limit_exceeded()))?;
            self.insert_service_call(rep, call_id);
            Ok(rep)
        })())
    }

    fn drop_service_provider(&mut self, self_: u32) -> Result<(), String> {
        if let Some(provider_id) = self.service_providers.remove(&self_) {
            let _ = native::service_unpublish(&self.host, &self.plugin_id, provider_id);
        }
        Ok(())
    }

    fn drop_service_call(&mut self, self_: u32) -> Result<(), String> {
        if let Some(call_id) = self.service_calls.remove(&self_) {
            let _ = native::service_cancel(&self.host, call_id);
        }
        Ok(())
    }
}
