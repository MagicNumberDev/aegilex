//! Core ABI implementation for `native/bindings/endstone/events/packet_send_event_facade.h`.

use super::support::*;



fn resolve_packet_send_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::PacketSendEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::PacketSendEvent)?;
    state
        .handles
        .packet_send_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_packet_send_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::PacketSendEventFacade>, HostError> {
    let handle = event_handle(state, event, ResourceKind::PacketSendEvent)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .packet_send_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostPacketSendEvent for PluginStoreState {
    fn packet_send_event_get_direction(
        &mut self,
        self_: u32,
    ) -> Result<Result<PacketPacketDirection, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "packet-send-event.packet-send-event.get-direction")?;
            self.resource_slot(self_, ResourceKind::PacketSendEvent)
                .map(|_| PacketPacketDirection::Send)
                .map_err(map_core_host_error)
        })())
    }

    fn packet_send_event_get_packet_id(
        &mut self,
        self_: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "packet-send-event.packet-send-event.get-packet-id")?;
            resolve_packet_send_event(self, self_)
                .map(|event| event.getPacketId())
                .map_err(map_core_host_error)
        })())
    }

    fn packet_send_event_get_payload(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<u8>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "packet-send-event.packet-send-event.get-payload")?;
            resolve_packet_send_event(self, self_)
                .map(|event| event.getPayloadForRust())
                .map_err(map_core_host_error)
        })())
    }

    fn packet_send_event_set_payload(
        &mut self,
        self_: u32,
        payload: Vec<u8>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "packet-send-event.packet-send-event.set-payload")?;
            resolve_packet_send_event_mut(self, self_)
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

    fn packet_send_event_get_player(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "packet-send-event.packet-send-event.get-player")?;
            let player = resolve_packet_send_event(self, self_)
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

    fn packet_send_event_get_address(
        &mut self,
        self_: u32,
    ) -> Result<Result<SocketAddressSocketAddress, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "packet-send-event.packet-send-event.get-address")?;
            resolve_packet_send_event(self, self_)
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

    fn packet_send_event_get_sub_client_id(
        &mut self,
        self_: u32,
    ) -> Result<Result<u8, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "packet-send-event.packet-send-event.get-sub-client-id",
            )?;
            resolve_packet_send_event(self, self_)
                .map(|event| event.getSubClientId())
                .map_err(map_core_host_error)
        })())
    }

    fn packet_send_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "packet-send-event.packet-send-event.is-cancelled")?;
            resolve_packet_send_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn packet_send_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "packet-send-event.packet-send-event.set-cancelled")?;
            resolve_packet_send_event_mut(self, self_)
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
