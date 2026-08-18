//! Shared Core ABI support for Endstone event facade modules.

#[allow(unused_imports)]
pub(crate) use crate::abi::AEGILEX_NOT_FOUND;
#[allow(unused_imports)]
pub(crate) use crate::core_host::imports::*;
#[allow(unused_imports)]
pub(crate) use crate::core_host::{
    BlockFaceBlockFace, DamageSourceDamageSource, EquipmentSlotEquipmentSlot, GameModeGameMode,
    LocationLocation, MessageMessage, PacketPacketDirection, PlayerInteractEventInteractAction,
    ServerLoadEventServerLoadType, SocketAddressSocketAddress, TypesHostError, VectorVector,
};
#[allow(unused_imports)]
pub(crate) use crate::cxx_runtime::ffi as cxx_event;
#[allow(unused_imports)]
pub(crate) use crate::host::endstone::inventory::resources::item_stack_resource_handle;
#[allow(unused_imports)]
pub(crate) use crate::host::endstone::support::*;
#[allow(unused_imports)]
pub(crate) use crate::host::runtime::core::{check_capability, map_core_host_error};
#[allow(unused_imports)]
pub(crate) use crate::host::runtime::handles::{ResourceKind, ResourceLifetime};
#[allow(unused_imports)]
pub(crate) use crate::host::runtime::native::HostError;
#[allow(unused_imports)]
pub(crate) use crate::host::runtime::store_shared::not_found;
#[allow(unused_imports)]
pub(crate) use crate::runtime::PluginStoreState;

/// Borrowed-handle check shared by every event interface: the token rep of a
/// host-borrowed event resource is its table key (`slot rep == token rep`).
pub(crate) fn event_handle(
    state: &PluginStoreState,
    event: u32,
    kind: ResourceKind,
) -> Result<u64, HostError> {
    state.resource_slot(event, kind).and_then(|slot| {
        (slot.lifetime == ResourceLifetime::HostBorrowed)
            .then_some(slot.handle)
            .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
    })
}

pub(crate) fn location_from_cxx(value: cxx_event::LocationData) -> LocationLocation {
    LocationLocation {
        dimension: value.dimension,
        x: value.x,
        y: value.y,
        z: value.z,
        pitch: value.pitch,
        yaw: value.yaw,
    }
}

pub(crate) fn location_to_cxx(value: LocationLocation) -> cxx_event::LocationData {
    cxx_event::LocationData {
        dimension: value.dimension,
        x: value.x,
        y: value.y,
        z: value.z,
        pitch: value.pitch,
        yaw: value.yaw,
    }
}

pub(crate) fn vector_from_cxx(value: cxx_event::VectorData) -> VectorVector {
    VectorVector {
        x: value.x,
        y: value.y,
        z: value.z,
    }
}

pub(crate) fn vector_to_cxx(value: VectorVector) -> cxx_event::VectorData {
    cxx_event::VectorData {
        x: value.x,
        y: value.y,
        z: value.z,
    }
}

pub(crate) fn block_face(face: u8) -> BlockFaceBlockFace {
    match face {
        0 => BlockFaceBlockFace::Down,
        1 => BlockFaceBlockFace::Up,
        2 => BlockFaceBlockFace::North,
        3 => BlockFaceBlockFace::South,
        4 => BlockFaceBlockFace::West,
        5 => BlockFaceBlockFace::East,
        _ => BlockFaceBlockFace::East,
    }
}
