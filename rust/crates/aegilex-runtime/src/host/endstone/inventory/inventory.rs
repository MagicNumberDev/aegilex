//! Core ABI implementation for `native/bindings/endstone/inventory/inventory.h`.

use crate::host::endstone::inventory::resources::*;
use crate::host::endstone::support::*;

impl crate::core_host::imports::HostInventory for PluginStoreState {
    fn inventory_get_size(&mut self, self_: u32) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "inventory.inventory.get-size")?;
            self.handles
                .inventory(
                    self.invocation_id,
                    inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                )
                .map(|inventory| inventory.getSize())
                .ok_or_else(not_found)
                .map_err(map_core_host_error)
        })())
    }

    fn inventory_get_item(
        &mut self,
        self_: u32,
        index: i32,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "inventory.inventory.get-item")?;
            let invocation_id = self.invocation_id;
            let item = self
                .handles
                .inventory(
                    invocation_id,
                    inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                )
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?
                .getItem(index);
            if item.is_null() {
                return Ok(None);
            }
            Ok(Some(
                self.insert_item_stack_resource(item)
                    .map_err(map_core_host_error)?,
            ))
        })())
    }

    fn inventory_set_item(
        &mut self,
        self_: u32,
        index: i32,
        item: Option<u32>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "inventory.inventory.set-item")?;
            let invocation_id = self.invocation_id;
            let inventory = self
                .handles
                .inventory(
                    invocation_id,
                    inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                )
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            if let Some(item) = item {
                let handle = item_stack_resource_handle(self, item).map_err(map_core_host_error)?;
                let item = self
                    .handles
                    .item_stack(invocation_id, handle)
                    .ok_or_else(not_found)
                    .map_err(map_core_host_error)?;
                inventory.setItem(index, item);
            } else {
                inventory.clearIndex(index);
            }
            Ok(())
        })())
    }

    fn inventory_get_contents(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<InventoryInventoryItem>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "inventory.inventory.get-contents")?;
            let invocation_id = self.invocation_id;
            let items = {
                let inventory = self
                    .handles
                    .inventory(
                        invocation_id,
                        inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                    )
                    .ok_or_else(not_found)
                    .map_err(map_core_host_error)?;
                (0..inventory.getSize())
                    .map(|index| (index, inventory.getItem(index)))
                    .collect::<Vec<_>>()
            };
            let mut contents = Vec::new();
            for (index, item) in items {
                let item = if item.is_null() {
                    None
                } else {
                    Some(
                        self.insert_item_stack_resource(item)
                            .map_err(map_core_host_error)?,
                    )
                };
                contents.push(InventoryInventoryItem { index, item });
            }
            Ok(contents)
        })())
    }

    fn inventory_set_contents(
        &mut self,
        self_: u32,
        items: Vec<Option<u32>>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "inventory.inventory.set-contents")?;
            let invocation_id = self.invocation_id;
            let inventory = self
                .handles
                .inventory(
                    invocation_id,
                    inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                )
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            if items.len() > inventory.getSize() as usize {
                return Err(map_core_host_error(not_found()));
            }
            inventory.clear();
            for (index, item) in items.into_iter().enumerate() {
                if let Some(item) = item {
                    let handle =
                        item_stack_resource_handle(self, item).map_err(map_core_host_error)?;
                    let item = self
                        .handles
                        .item_stack(invocation_id, handle)
                        .ok_or_else(not_found)
                        .map_err(map_core_host_error)?;
                    inventory.setItem(index as i32, item);
                }
            }
            Ok(())
        })())
    }

    fn inventory_clear(&mut self, self_: u32) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "inventory.inventory.clear")?;
            self.handles
                .inventory(
                    self.invocation_id,
                    inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                )
                .map(|inventory| inventory.clear())
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn inventory_clear_index(
        &mut self,
        self_: u32,
        index: i32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "inventory.inventory.clear-index")?;
            self.handles
                .inventory(
                    self.invocation_id,
                    inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                )
                .map(|inventory| inventory.clearIndex(index))
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn inventory_get_max_stack_size(
        &mut self,
        self_: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "inventory.inventory.get-max-stack-size")?;
            self.handles
                .inventory(
                    self.invocation_id,
                    inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                )
                .map(|inventory| inventory.getMaxStackSize())
                .ok_or_else(not_found)
                .map_err(map_core_host_error)
        })())
    }

    fn inventory_add_item(
        &mut self,
        self_: u32,
        items: Vec<u32>,
    ) -> Result<Result<Vec<InventoryItemStackResult>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "inventory.inventory.add-item")?;
            let invocation_id = self.invocation_id;
            let native_leftovers = {
                let inventory = self
                    .handles
                    .inventory(
                        invocation_id,
                        inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                    )
                    .ok_or_else(not_found)
                    .map_err(map_core_host_error)?;
                let mut native_leftovers = Vec::new();
                for (slot, resource) in items.into_iter().enumerate() {
                    let handle =
                        item_stack_resource_handle(self, resource).map_err(map_core_host_error)?;
                    let item = self
                        .handles
                        .item_stack(invocation_id, handle)
                        .ok_or_else(not_found)
                        .map_err(map_core_host_error)?;
                    let leftover = inventory.addItem(item);
                    if !leftover.is_null() {
                        native_leftovers.push((slot, leftover));
                    }
                }
                native_leftovers
            };
            let mut leftovers = Vec::new();
            for (slot, leftover) in native_leftovers {
                leftovers.push(InventoryItemStackResult {
                    slot: slot as i32,
                    item: self
                        .insert_item_stack_resource(leftover)
                        .map_err(map_core_host_error)?,
                });
            }
            Ok(leftovers)
        })())
    }

    fn inventory_remove_item(
        &mut self,
        self_: u32,
        items: Vec<u32>,
    ) -> Result<Result<Vec<InventoryItemStackResult>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "inventory.inventory.remove-item")?;
            let invocation_id = self.invocation_id;
            let native_removed = {
                let inventory = self
                    .handles
                    .inventory(
                        invocation_id,
                        inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                    )
                    .ok_or_else(not_found)
                    .map_err(map_core_host_error)?;
                let mut native_removed = Vec::new();
                for (slot, resource) in items.into_iter().enumerate() {
                    let handle =
                        item_stack_resource_handle(self, resource).map_err(map_core_host_error)?;
                    let item = self
                        .handles
                        .item_stack(invocation_id, handle)
                        .ok_or_else(not_found)
                        .map_err(map_core_host_error)?;
                    let removed = inventory.removeItem(item);
                    if !removed.is_null() {
                        native_removed.push((slot, removed));
                    }
                }
                native_removed
            };
            let mut removed = Vec::new();
            for (slot, leftover) in native_removed {
                removed.push(InventoryItemStackResult {
                    slot: slot as i32,
                    item: self
                        .insert_item_stack_resource(leftover)
                        .map_err(map_core_host_error)?,
                });
            }
            Ok(removed)
        })())
    }

    fn inventory_first_slot(
        &mut self,
        self_: u32,
        request: InventoryItemRequest,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "inventory.inventory.first-slot")?;
            let invocation_id = self.invocation_id;
            let inventory = self
                .handles
                .inventory(
                    invocation_id,
                    inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                )
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            let (stack, type_id) = match request.matcher {
                InventoryItemMatcher::Stack(resource) => (
                    Some(
                        self.handles
                            .item_stack(
                                invocation_id,
                                item_stack_resource_handle(self, resource)
                                    .map_err(map_core_host_error)?,
                            )
                            .ok_or_else(not_found)
                            .map_err(map_core_host_error)?,
                    ),
                    None,
                ),
                InventoryItemMatcher::TypeId(type_id) => (None, Some(type_id)),
            };
            for slot in 0..inventory.getSize() {
                let item = inventory.getItem(slot);
                let Some(item_ref) = item.as_ref() else {
                    continue;
                };
                let matches = type_id
                    .as_ref()
                    .is_some_and(|type_id| item_ref.getType() == *type_id)
                    || stack.is_some_and(|stack| item_ref.equals(stack));
                if matches {
                    return Ok(Some(slot as u32));
                }
            }
            Ok(None)
        })())
    }

    fn inventory_contains(
        &mut self,
        self_: u32,
        request: InventoryItemRequest,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "inventory.inventory.contains")?;
            let invocation_id = self.invocation_id;
            let inventory = self
                .handles
                .inventory(
                    invocation_id,
                    inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                )
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            match request.matcher {
                InventoryItemMatcher::TypeId(type_id) => Ok(request.amount.map_or_else(
                    || inventory.containsType(&type_id),
                    |amount| inventory.containsAtLeastType(&type_id, amount),
                )),
                InventoryItemMatcher::Stack(item) => {
                    let handle =
                        item_stack_resource_handle(self, item).map_err(map_core_host_error)?;
                    let item = self
                        .handles
                        .item_stack(invocation_id, handle)
                        .ok_or_else(not_found)
                        .map_err(map_core_host_error)?;
                    Ok(request.amount.map_or_else(
                        || inventory.containsStack(item),
                        |amount| inventory.containsAtLeastStack(item, amount),
                    ))
                }
            }
        })())
    }

    fn inventory_list_matching(
        &mut self,
        self_: u32,
        request: InventoryItemRequest,
    ) -> Result<Result<Vec<InventoryItemStackResult>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "inventory.inventory.list-matching")?;
            let invocation_id = self.invocation_id;
            let matched_items = {
                let inventory = self
                    .handles
                    .inventory(
                        invocation_id,
                        inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                    )
                    .ok_or_else(not_found)
                    .map_err(map_core_host_error)?;
                let (stack, type_id) = match request.matcher {
                    InventoryItemMatcher::Stack(resource) => (
                        Some(
                            self.handles
                                .item_stack(
                                    invocation_id,
                                    item_stack_resource_handle(self, resource)
                                        .map_err(map_core_host_error)?,
                                )
                                .ok_or_else(not_found)
                                .map_err(map_core_host_error)?,
                        ),
                        None,
                    ),
                    InventoryItemMatcher::TypeId(type_id) => (None, Some(type_id)),
                };
                let mut matched_items = Vec::new();
                for slot in 0..inventory.getSize() {
                    let item = inventory.getItem(slot);
                    let Some(item_ref) = item.as_ref() else {
                        continue;
                    };
                    let matches = type_id
                        .as_ref()
                        .is_some_and(|type_id| item_ref.getType() == *type_id)
                        || stack.is_some_and(|stack| item_ref.equals(stack));
                    if matches {
                        matched_items.push((slot, item));
                    }
                }
                matched_items
            };
            let mut result = Vec::new();
            for (slot, item) in matched_items {
                result.push(InventoryItemStackResult {
                    slot,
                    item: self
                        .insert_item_stack_resource(item)
                        .map_err(map_core_host_error)?,
                });
            }
            Ok(result)
        })())
    }

    fn inventory_first_empty(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "inventory.inventory.first-empty")?;
            let slot = self
                .handles
                .inventory(
                    self.invocation_id,
                    inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                )
                .map(|inventory| inventory.firstEmpty())
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            Ok((slot >= 0).then_some(slot as u32))
        })())
    }

    fn inventory_is_empty(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "inventory.inventory.is-empty")?;
            self.handles
                .inventory(
                    self.invocation_id,
                    inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                )
                .map(|inventory| inventory.isEmpty())
                .ok_or_else(not_found)
                .map_err(map_core_host_error)
        })())
    }

    fn inventory_remove_matching(
        &mut self,
        self_: u32,
        request: InventoryItemRequest,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "inventory.inventory.remove-matching")?;
            let invocation_id = self.invocation_id;
            let inventory = self
                .handles
                .inventory(
                    invocation_id,
                    inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                )
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            match request.matcher {
                InventoryItemMatcher::TypeId(type_id) => inventory.removeType(&type_id),
                InventoryItemMatcher::Stack(item) => {
                    let handle =
                        item_stack_resource_handle(self, item).map_err(map_core_host_error)?;
                    let item = self
                        .handles
                        .item_stack(invocation_id, handle)
                        .ok_or_else(not_found)
                        .map_err(map_core_host_error)?;
                    inventory.removeStack(item);
                }
            }
            Ok(())
        })())
    }
}
