//! Core ABI implementation for `native/bindings/endstone/events/packet_receive_event_facade.h`.

use super::support::*;



fn resolve_packet_receive_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::PacketReceiveEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::PacketReceiveEvent)?;
    state
        .handles
        .packet_receive_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_packet_receive_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::PacketReceiveEventFacade>, HostError> {
    let handle = event_handle(state, event, ResourceKind::PacketReceiveEvent)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .packet_receive_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostPacketReceiveEvent for PluginStoreState {
    fn packet_receive_event_get_direction(
        &mut self,
        self_: u32,
    ) -> Result<Result<PacketPacketDirection, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "packet-receive-event.packet-receive-event.get-direction",
            )?;
            self.resource_slot(self_, ResourceKind::PacketReceiveEvent)
                .map(|_| PacketPacketDirection::Receive)
                .map_err(map_core_host_error)
        })())
    }

    fn packet_receive_event_get_packet_id(
        &mut self,
        self_: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "packet-receive-event.packet-receive-event.get-packet-id",
            )?;
            resolve_packet_receive_event(self, self_)
                .map(|event| event.getPacketId())
                .map_err(map_core_host_error)
        })())
    }

    fn packet_receive_event_get_payload(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<u8>, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "packet-receive-event.packet-receive-event.get-payload",
            )?;
            resolve_packet_receive_event(self, self_)
                .map(|event| event.getPayloadForRust())
                .map_err(map_core_host_error)
        })())
    }

    fn packet_receive_event_set_payload(
        &mut self,
        self_: u32,
        payload: Vec<u8>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "packet-receive-event.packet-receive-event.set-payload",
            )?;
            resolve_packet_receive_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setPayloadForRust(&payload)
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn packet_receive_event_get_player(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "packet-receive-event.packet-receive-event.get-player")?;
            let player = resolve_packet_receive_event(self, self_)
                .map(|event| event.getPlayer())
                .map_err(map_core_host_error)?;
            if player.is_null() {
                return Ok(None);
            }
            Ok(Some(
                self.insert_player_resource(player)
                    .map_err(map_core_host_error)?,
            ))
        })())
    }

    fn packet_receive_event_get_address(
        &mut self,
        self_: u32,
    ) -> Result<Result<SocketAddressSocketAddress, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "packet-receive-event.packet-receive-event.get-address",
            )?;
            resolve_packet_receive_event(self, self_)
                .map(|event| {
                    let address = event.getAddress();
                    SocketAddressSocketAddress {
                        hostname: address.hostname,
                        port: address.port,
                    }
                })
                .map_err(map_core_host_error)
        })())
    }

    fn packet_receive_event_get_sub_client_id(
        &mut self,
        self_: u32,
    ) -> Result<Result<u8, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "packet-receive-event.packet-receive-event.get-sub-client-id",
            )?;
            resolve_packet_receive_event(self, self_)
                .map(|event| event.getSubClientId())
                .map_err(map_core_host_error)
        })())
    }

    fn packet_receive_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "packet-receive-event.packet-receive-event.is-cancelled",
            )?;
            resolve_packet_receive_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn packet_receive_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "packet-receive-event.packet-receive-event.set-cancelled",
            )?;
            resolve_packet_receive_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setCancelled(cancelled)
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }
}
