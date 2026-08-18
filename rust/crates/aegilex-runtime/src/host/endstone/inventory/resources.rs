use crate::cxx_host_inventory::ffi as cxx_inventory;
use crate::host::runtime::handles::ResourceKind;
use crate::host::runtime::native::HostError;
use crate::host::runtime::store_shared::not_found;
use crate::runtime::PluginStoreState;

pub(crate) fn item_stack_resource_handle(
    state: &PluginStoreState,
    item: u32,
) -> Result<u64, HostError> {
    state
        .resource_slot(item, ResourceKind::ItemStack)
        .map(|slot| slot.handle)
}

pub(crate) fn item_stack_ref_resource_handle(
    state: &PluginStoreState,
    item: u32,
) -> Result<u64, HostError> {
    state
        .resource_slot(item, ResourceKind::ItemStackRef)
        .map(|slot| slot.handle)
}

pub(crate) fn inventory_resource_handle(
    state: &PluginStoreState,
    inventory: u32,
) -> Result<u64, HostError> {
    state
        .resource_slot(inventory, ResourceKind::Inventory)
        .map(|slot| slot.handle)
}

pub(crate) fn player_inventory_resource_handle(
    state: &PluginStoreState,
    inventory: u32,
) -> Result<u64, HostError> {
    state
        .resource_slot(inventory, ResourceKind::PlayerInventory)
        .map(|slot| slot.handle)
}

pub(crate) fn resolve_item_stack_ref(
    state: &PluginStoreState,
    handle: u64,
) -> Result<&cxx_inventory::ItemStackRef, HostError> {
    state
        .handles
        .item_stack_ref(state.invocation_id, handle)
        .ok_or_else(not_found)
}

pub(crate) fn player_get_slot<F>(
    state: &mut PluginStoreState,
    inventory: u64,
    get: F,
) -> Result<Option<u32>, HostError>
where
    F: FnOnce(&cxx_inventory::PlayerInventory) -> cxx::UniquePtr<cxx_inventory::ItemStack>,
{
    let invocation_id = state.invocation_id;
    let inventory = state
        .handles
        .player_inventory(invocation_id, inventory)
        .ok_or_else(not_found)?;
    let item = get(inventory);
    if item.is_null() {
        return Ok(None);
    }
    state.insert_item_stack_resource(item).map(Some)
}

pub(crate) fn player_set_slot<F, G>(
    state: &PluginStoreState,
    inventory: u64,
    item: Option<u32>,
    set: F,
    clear: G,
) -> Result<(), HostError>
where
    F: FnOnce(&cxx_inventory::PlayerInventory, &cxx_inventory::ItemStack),
    G: FnOnce(&cxx_inventory::PlayerInventory),
{
    let invocation_id = state.invocation_id;
    let inventory = state
        .handles
        .player_inventory(invocation_id, inventory)
        .ok_or_else(not_found)?;
    if let Some(item) = item {
        let handle = item_stack_resource_handle(state, item)?;
        let item = state
            .handles
            .item_stack(invocation_id, handle)
            .ok_or_else(not_found)?;
        set(inventory, item);
    } else {
        clear(inventory);
    }
    Ok(())
}
