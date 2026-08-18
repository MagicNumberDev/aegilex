//! Core ABI implementation for `native/bindings/endstone/inventory/item_type.h`.

use crate::host::endstone::support::*;

impl crate::core_host::imports::HostItemType for PluginStoreState {
    fn get_item_type(
        &mut self,
        type_id: String,
    ) -> Result<Result<ItemTypeItemTypeData, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-type.get-item-type")?;
            let item_type = resolve_server(self)
                .map_err(map_core_host_error)?
                .getItemType(&type_id);
            if item_type.is_null() {
                return Err(map_core_host_error(not_found()));
            }
            Ok(ItemTypeItemTypeData {
                type_id: item_type.getTypeId(),
                translation_key: item_type.getTranslationKey(),
                max_stack_size: item_type.getMaxStackSize(),
                max_durability: item_type.getMaxDurability(),
            })
        })())
    }

    fn create_item_stack(
        &mut self,
        type_id: String,
        amount: Option<i32>,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-type.create-item-stack")?;
            let item_type = resolve_server(self)
                .map_err(map_core_host_error)?
                .getItemType(&type_id);
            if item_type.is_null() {
                return Err(map_core_host_error(not_found()));
            }
            let item = item_type.createItemStack(amount.unwrap_or(1));
            if item.is_null() {
                return Err(map_core_host_error(not_found()));
            }
            self.insert_item_stack_resource(item)
                .map_err(map_core_host_error)
        })())
    }
}
