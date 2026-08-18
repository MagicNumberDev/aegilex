//! Core ABI implementation for `native/bindings/endstone/inventory/player_inventory.h`.

use crate::host::endstone::inventory::resources::*;
use crate::host::endstone::support::*;

impl crate::core_host::imports::HostPlayerInventory for PluginStoreState {
    fn player_inventory_get_inventory(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-inventory.player-inventory.get-inventory")?;
            let handle =
                player_inventory_resource_handle(self, self_).map_err(map_core_host_error)?;
            let inventory = self
                .handles
                .player_inventory(self.invocation_id, handle)
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?
                .asInventory();
            self.insert_inventory_resource(inventory)
                .map_err(map_core_host_error)
        })())
    }

    fn player_inventory_get_helmet(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-inventory.player-inventory.get-helmet")?;
            player_get_slot(
                self,
                player_inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                |inventory| inventory.getHelmet(),
            )
            .map_err(map_core_host_error)
        })())
    }

    fn player_inventory_set_helmet(
        &mut self,
        self_: u32,
        item: Option<u32>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-inventory.player-inventory.set-helmet")?;
            player_set_slot(
                self,
                player_inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                item,
                |inventory, item| inventory.setHelmet(item),
                |inventory| inventory.clearHelmet(),
            )
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_inventory_get_chestplate(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-inventory.player-inventory.get-chestplate")?;
            player_get_slot(
                self,
                player_inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                |inventory| inventory.getChestplate(),
            )
            .map_err(map_core_host_error)
        })())
    }

    fn player_inventory_set_chestplate(
        &mut self,
        self_: u32,
        item: Option<u32>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-inventory.player-inventory.set-chestplate")?;
            player_set_slot(
                self,
                player_inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                item,
                |inventory, item| inventory.setChestplate(item),
                |inventory| inventory.clearChestplate(),
            )
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_inventory_get_leggings(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-inventory.player-inventory.get-leggings")?;
            player_get_slot(
                self,
                player_inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                |inventory| inventory.getLeggings(),
            )
            .map_err(map_core_host_error)
        })())
    }

    fn player_inventory_set_leggings(
        &mut self,
        self_: u32,
        item: Option<u32>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-inventory.player-inventory.set-leggings")?;
            player_set_slot(
                self,
                player_inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                item,
                |inventory, item| inventory.setLeggings(item),
                |inventory| inventory.clearLeggings(),
            )
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_inventory_get_boots(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-inventory.player-inventory.get-boots")?;
            player_get_slot(
                self,
                player_inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                |inventory| inventory.getBoots(),
            )
            .map_err(map_core_host_error)
        })())
    }

    fn player_inventory_set_boots(
        &mut self,
        self_: u32,
        item: Option<u32>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-inventory.player-inventory.set-boots")?;
            player_set_slot(
                self,
                player_inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                item,
                |inventory, item| inventory.setBoots(item),
                |inventory| inventory.clearBoots(),
            )
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_inventory_get_item_in_main_hand(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-inventory.player-inventory.get-item-in-main-hand",
            )?;
            player_get_slot(
                self,
                player_inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                |inventory| inventory.getItemInMainHand(),
            )
            .map_err(map_core_host_error)
        })())
    }

    fn player_inventory_set_item_in_main_hand(
        &mut self,
        self_: u32,
        item: Option<u32>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-inventory.player-inventory.set-item-in-main-hand",
            )?;
            player_set_slot(
                self,
                player_inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                item,
                |inventory, item| inventory.setItemInMainHand(item),
                |inventory| inventory.clearItemInMainHand(),
            )
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_inventory_get_item_in_off_hand(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-inventory.player-inventory.get-item-in-off-hand",
            )?;
            player_get_slot(
                self,
                player_inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                |inventory| inventory.getItemInOffHand(),
            )
            .map_err(map_core_host_error)
        })())
    }

    fn player_inventory_set_item_in_off_hand(
        &mut self,
        self_: u32,
        item: Option<u32>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "player-inventory.player-inventory.set-item-in-off-hand",
            )?;
            player_set_slot(
                self,
                player_inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                item,
                |inventory, item| inventory.setItemInOffHand(item),
                |inventory| inventory.clearItemInOffHand(),
            )
            .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn player_inventory_get_held_item_slot(
        &mut self,
        self_: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-inventory.player-inventory.get-held-item-slot")?;
            self.handles
                .player_inventory(
                    self.invocation_id,
                    player_inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                )
                .map(|inventory| inventory.getHeldItemSlot())
                .ok_or_else(not_found)
                .map_err(map_core_host_error)
        })())
    }

    fn player_inventory_set_held_item_slot(
        &mut self,
        self_: u32,
        slot: i32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-inventory.player-inventory.set-held-item-slot")?;
            self.handles
                .player_inventory(
                    self.invocation_id,
                    player_inventory_resource_handle(self, self_).map_err(map_core_host_error)?,
                )
                .map(|inventory| inventory.setHeldItemSlot(slot))
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }
}
