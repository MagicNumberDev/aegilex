//! Core ABI implementation for `native/bindings/endstone/inventory/item_meta.h`.

use crate::host::endstone::inventory::resources::*;
use crate::host::endstone::support::*;

fn resolve_server(state: &PluginStoreState) -> Result<&cxx_server::Server, HostError> {
    state.host.server()
}

fn resolve_plugin_command(
    state: &PluginStoreState,
    handle: u64,
) -> Result<&cxx_server::PluginCommand, HostError> {
    state
        .handles
        .plugin_command(state.invocation_id, handle)
        .ok_or_else(not_found)
}

impl HostItemMeta for PluginStoreState {
    fn item_meta_get_type(
        &mut self,
        self_: u32,
    ) -> Result<Result<ItemMetaItemMetaType, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.get-type")?;
            item_meta_value(self, self_)
                .map(|meta| item_meta_type(&meta.meta))
                .map_err(map_core_host_error)
        })())
    }

    fn item_meta_has_display_name(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.has-display-name")?;
            item_meta_value(self, self_)
                .map(|meta| meta.meta.base.has_display_name)
                .map_err(map_core_host_error)
        })())
    }

    fn item_meta_get_display_name(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.get-display-name")?;
            let meta = item_meta_value(self, self_).map_err(map_core_host_error)?;
            meta.meta
                .base
                .has_display_name
                .then(|| meta.meta.base.display_name.clone())
                .ok_or_else(|| map_core_host_error(not_found()))
        })())
    }

    fn item_meta_set_display_name(
        &mut self,
        self_: u32,
        name: Option<String>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.set-display-name")?;
            let meta = self
                .owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?;
            meta.meta.base.has_display_name = name.is_some();
            meta.meta.base.display_name = name.unwrap_or_default();
            Ok(())
        })())
    }

    fn item_meta_has_lore(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.has-lore")?;
            item_meta_value(self, self_)
                .map(|meta| meta.meta.base.has_lore)
                .map_err(map_core_host_error)
        })())
    }

    fn item_meta_get_lore(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<String>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.get-lore")?;
            let meta = item_meta_value(self, self_).map_err(map_core_host_error)?;
            meta.meta
                .base
                .has_lore
                .then(|| meta.meta.base.lore.clone())
                .ok_or_else(|| map_core_host_error(not_found()))
        })())
    }

    fn item_meta_set_lore(
        &mut self,
        self_: u32,
        lore: Option<Vec<String>>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.set-lore")?;
            let meta = self
                .owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?;
            meta.meta.base.has_lore = lore.is_some();
            meta.meta.base.lore = lore.unwrap_or_default();
            Ok(())
        })())
    }

    fn item_meta_has_enchants(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.has-enchants")?;
            item_meta_value(self, self_)
                .map(|meta| !meta.meta.base.enchants.is_empty())
                .map_err(map_core_host_error)
        })())
    }

    fn item_meta_has_enchant(
        &mut self,
        self_: u32,
        type_id: String,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.has-enchant")?;
            item_meta_value(self, self_)
                .map(|meta| {
                    meta.meta
                        .base
                        .enchants
                        .iter()
                        .any(|enchant| enchant.type_id == type_id)
                })
                .map_err(map_core_host_error)
        })())
    }

    fn item_meta_get_enchant_level(
        &mut self,
        self_: u32,
        type_id: String,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.get-enchant-level")?;
            item_meta_value(self, self_)
                .map_err(map_core_host_error)?
                .meta
                .base
                .enchants
                .iter()
                .find(|enchant| enchant.type_id == type_id)
                .map(|enchant| enchant.level)
                .ok_or_else(|| map_core_host_error(not_found()))
        })())
    }

    fn item_meta_get_enchants(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<ItemMetaEnchantment>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.get-enchants")?;
            item_meta_value(self, self_)
                .map(|meta| {
                    meta.meta
                        .base
                        .enchants
                        .iter()
                        .map(|enchant| ItemMetaEnchantment {
                            type_id: enchant.type_id.clone(),
                            level: enchant.level,
                        })
                        .collect()
                })
                .map_err(map_core_host_error)
        })())
    }

    fn item_meta_add_enchant(
        &mut self,
        self_: u32,
        type_id: String,
        level: i32,
        force: bool,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.add-enchant")?;
            let meta = self
                .owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?;
            if let Some(enchant) = meta
                .meta
                .base
                .enchants
                .iter_mut()
                .find(|enchant| enchant.type_id == type_id)
            {
                if !force {
                    return Ok(false);
                }
                enchant.level = level;
            } else {
                meta.meta
                    .base
                    .enchants
                    .push(cxx_inventory::Enchantment { type_id, level });
            }
            Ok(true)
        })())
    }

    fn item_meta_remove_enchant(
        &mut self,
        self_: u32,
        type_id: String,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.remove-enchant")?;
            let meta = self
                .owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?;
            let length = meta.meta.base.enchants.len();
            meta.meta
                .base
                .enchants
                .retain(|enchant| enchant.type_id != type_id);
            Ok(meta.meta.base.enchants.len() != length)
        })())
    }

    fn item_meta_remove_enchants(
        &mut self,
        self_: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.remove-enchants")?;
            self.owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?
                .meta
                .base
                .enchants
                .clear();
            Ok(())
        })())
    }

    fn item_meta_is_unbreakable(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.is-unbreakable")?;
            item_meta_value(self, self_)
                .map(|meta| meta.meta.base.unbreakable)
                .map_err(map_core_host_error)
        })())
    }

    fn item_meta_set_unbreakable(
        &mut self,
        self_: u32,
        unbreakable: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.set-unbreakable")?;
            self.owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?
                .meta
                .base
                .unbreakable = unbreakable;
            Ok(())
        })())
    }

    fn item_meta_has_damage(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.has-damage")?;
            item_meta_value(self, self_)
                .map(|meta| meta.meta.base.has_damage)
                .map_err(map_core_host_error)
        })())
    }

    fn item_meta_get_damage(&mut self, self_: u32) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.get-damage")?;
            let meta = item_meta_value(self, self_).map_err(map_core_host_error)?;
            meta.meta
                .base
                .has_damage
                .then_some(meta.meta.base.damage)
                .ok_or_else(|| map_core_host_error(not_found()))
        })())
    }

    fn item_meta_set_damage(
        &mut self,
        self_: u32,
        damage: i32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.set-damage")?;
            let meta = self
                .owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?;
            meta.meta.base.has_damage = true;
            meta.meta.base.damage = damage;
            Ok(())
        })())
    }

    fn item_meta_has_repair_cost(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.has-repair-cost")?;
            item_meta_value(self, self_)
                .map(|meta| meta.meta.base.has_repair_cost)
                .map_err(map_core_host_error)
        })())
    }

    fn item_meta_get_repair_cost(
        &mut self,
        self_: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.get-repair-cost")?;
            let meta = item_meta_value(self, self_).map_err(map_core_host_error)?;
            meta.meta
                .base
                .has_repair_cost
                .then_some(meta.meta.base.repair_cost)
                .ok_or_else(|| map_core_host_error(not_found()))
        })())
    }

    fn item_meta_set_repair_cost(
        &mut self,
        self_: u32,
        cost: i32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.set-repair-cost")?;
            let meta = self
                .owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?;
            meta.meta.base.has_repair_cost = true;
            meta.meta.base.repair_cost = cost;
            Ok(())
        })())
    }

    fn item_meta_clone(&mut self, self_: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-meta.item-meta.clone")?;
            clone_item_meta_resource(self, self_).map_err(map_core_host_error)
        })())
    }
}

impl HostItemFactory for PluginStoreState {
    fn create_for_type(&mut self, type_id: String) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-factory.create-for-type")?;
            let mut meta = cxx_meta_defaults();
            let mut projectiles =
                cxx_inventory::ItemStackCollection::create_item_stack_collection();
            let server = resolve_server(self).map_err(map_core_host_error)?;
            if projectiles.is_null()
                || !server.createItemMetaForType(&type_id, &mut meta, projectiles.pin_mut())
            {
                return Err(map_core_host_error(not_found()));
            }
            insert_item_meta_resource(self, meta, projectiles).map_err(map_core_host_error)
        })())
    }

    fn is_applicable(
        &mut self,
        type_id: String,
        meta: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-factory.is-applicable")?;
            let meta = item_meta_value(self, meta).map_err(map_core_host_error)?;
            let mut applicable = false;
            if !resolve_server(self)
                .map_err(map_core_host_error)?
                .isItemMetaApplicable(&type_id, &meta.meta, &meta.projectiles, &mut applicable)
            {
                return Err(map_core_host_error(not_found()));
            }
            Ok(applicable)
        })())
    }

    fn equals(
        &mut self,
        a: Option<u32>,
        b: Option<u32>,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-factory.equals")?;
            let empty_meta = cxx_meta_defaults();
            let empty_projectiles =
                cxx_inventory::ItemStackCollection::create_item_stack_collection();
            if empty_projectiles.is_null() {
                return Err(map_core_host_error(not_found()));
            }
            let a = a
                .map(|meta| item_meta_value(self, meta))
                .transpose()
                .map_err(map_core_host_error)?;
            let b = b
                .map(|meta| item_meta_value(self, meta))
                .transpose()
                .map_err(map_core_host_error)?;
            let (has_a, a_meta, a_projectiles) = a
                .map_or((false, &empty_meta, &empty_projectiles), |meta| {
                    (true, &meta.meta, &meta.projectiles)
                });
            let (has_b, b_meta, b_projectiles) = b
                .map_or((false, &empty_meta, &empty_projectiles), |meta| {
                    (true, &meta.meta, &meta.projectiles)
                });
            let mut equal = false;
            if !resolve_server(self)
                .map_err(map_core_host_error)?
                .areItemMetasEqual(
                    has_a,
                    a_meta,
                    a_projectiles,
                    has_b,
                    b_meta,
                    b_projectiles,
                    &mut equal,
                )
            {
                return Err(map_core_host_error(not_found()));
            }
            Ok(equal)
        })())
    }

    fn convert_for_type(
        &mut self,
        type_id: String,
        meta: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-factory.convert-for-type")?;
            let meta = item_meta_value(self, meta).map_err(map_core_host_error)?;
            let mut converted = cxx_meta_defaults();
            let mut projectiles =
                cxx_inventory::ItemStackCollection::create_item_stack_collection();
            if projectiles.is_null()
                || !resolve_server(self)
                    .map_err(map_core_host_error)?
                    .convertItemMetaForType(
                        &type_id,
                        &meta.meta,
                        &meta.projectiles,
                        &mut converted,
                        projectiles.pin_mut(),
                    )
            {
                return Err(map_core_host_error(not_found()));
            }
            insert_item_meta_resource(self, converted, projectiles).map_err(map_core_host_error)
        })())
    }
}

impl HostWritableBookMeta for PluginStoreState {
    fn writable_book_meta_has_pages(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "writable-book-meta.writable-book-meta.has-pages")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_WRITABLE_BOOK)
                .map_err(map_core_host_error)?;
            Ok(!item_meta_value(self, self_)
                .map_err(map_core_host_error)?
                .meta
                .pages
                .is_empty())
        })())
    }

    fn writable_book_meta_get_page(
        &mut self,
        self_: u32,
        page: i32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "writable-book-meta.writable-book-meta.get-page")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_WRITABLE_BOOK)
                .map_err(map_core_host_error)?;
            let page = usize::try_from(page).map_err(|_| map_core_host_error(not_found()))?;
            item_meta_value(self, self_)
                .map_err(map_core_host_error)?
                .meta
                .pages
                .get(page)
                .cloned()
                .ok_or_else(|| map_core_host_error(not_found()))
        })())
    }

    fn writable_book_meta_set_page(
        &mut self,
        self_: u32,
        page: i32,
        data: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "writable-book-meta.writable-book-meta.set-page")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_WRITABLE_BOOK)
                .map_err(map_core_host_error)?;
            let page = usize::try_from(page).map_err(|_| map_core_host_error(not_found()))?;
            self.owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?
                .meta
                .pages
                .get_mut(page)
                .map(|value| *value = data)
                .ok_or_else(|| map_core_host_error(not_found()))
        })())
    }

    fn writable_book_meta_get_pages(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<String>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "writable-book-meta.writable-book-meta.get-pages")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_WRITABLE_BOOK)
                .map_err(map_core_host_error)?;
            Ok(item_meta_value(self, self_)
                .map_err(map_core_host_error)?
                .meta
                .pages
                .clone())
        })())
    }

    fn writable_book_meta_set_pages(
        &mut self,
        self_: u32,
        pages: Vec<String>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "writable-book-meta.writable-book-meta.set-pages")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_WRITABLE_BOOK)
                .map_err(map_core_host_error)?;
            self.owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?
                .meta
                .pages = pages;
            Ok(())
        })())
    }

    fn writable_book_meta_add_pages(
        &mut self,
        self_: u32,
        pages: Vec<String>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "writable-book-meta.writable-book-meta.add-pages")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_WRITABLE_BOOK)
                .map_err(map_core_host_error)?;
            self.owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?
                .meta
                .pages
                .extend(pages);
            Ok(())
        })())
    }

    fn writable_book_meta_get_page_count(
        &mut self,
        self_: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "writable-book-meta.writable-book-meta.get-page-count")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_WRITABLE_BOOK)
                .map_err(map_core_host_error)?;
            Ok(item_meta_value(self, self_)
                .map_err(map_core_host_error)?
                .meta
                .pages
                .len() as i32)
        })())
    }

    fn from_item_meta(&mut self, meta: u32) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "writable-book-meta.from-item-meta")?;
            if require_item_meta_kind(self, meta, ITEM_META_KIND_WRITABLE_BOOK).is_err() {
                return Ok(None);
            }
            Ok(Some(
                clone_item_meta_resource(self, meta).map_err(map_core_host_error)?,
            ))
        })())
    }

    fn as_item_meta(&mut self, meta: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "writable-book-meta.as-item-meta")?;
            require_item_meta_kind(self, meta, ITEM_META_KIND_WRITABLE_BOOK)
                .map_err(map_core_host_error)?;
            clone_item_meta_resource(self, meta).map_err(map_core_host_error)
        })())
    }
}

impl HostBookMeta for PluginStoreState {
    fn book_meta_has_pages(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "book-meta.book-meta.has-pages")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_BOOK)
                .map_err(map_core_host_error)?;
            Ok(!item_meta_value(self, self_)
                .map_err(map_core_host_error)?
                .meta
                .pages
                .is_empty())
        })())
    }

    fn book_meta_get_page(
        &mut self,
        self_: u32,
        page: i32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "book-meta.book-meta.get-page")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_BOOK)
                .map_err(map_core_host_error)?;
            let page = usize::try_from(page).map_err(|_| map_core_host_error(not_found()))?;
            item_meta_value(self, self_)
                .map_err(map_core_host_error)?
                .meta
                .pages
                .get(page)
                .cloned()
                .ok_or_else(|| map_core_host_error(not_found()))
        })())
    }

    fn book_meta_set_page(
        &mut self,
        self_: u32,
        page: i32,
        data: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "book-meta.book-meta.set-page")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_BOOK)
                .map_err(map_core_host_error)?;
            let page = usize::try_from(page).map_err(|_| map_core_host_error(not_found()))?;
            self.owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?
                .meta
                .pages
                .get_mut(page)
                .map(|value| *value = data)
                .ok_or_else(|| map_core_host_error(not_found()))
        })())
    }

    fn book_meta_get_pages(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<String>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "book-meta.book-meta.get-pages")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_BOOK)
                .map_err(map_core_host_error)?;
            Ok(item_meta_value(self, self_)
                .map_err(map_core_host_error)?
                .meta
                .pages
                .clone())
        })())
    }

    fn book_meta_set_pages(
        &mut self,
        self_: u32,
        pages: Vec<String>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "book-meta.book-meta.set-pages")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_BOOK)
                .map_err(map_core_host_error)?;
            self.owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?
                .meta
                .pages = pages;
            Ok(())
        })())
    }

    fn book_meta_add_pages(
        &mut self,
        self_: u32,
        pages: Vec<String>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "book-meta.book-meta.add-pages")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_BOOK)
                .map_err(map_core_host_error)?;
            self.owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?
                .meta
                .pages
                .extend(pages);
            Ok(())
        })())
    }

    fn book_meta_get_page_count(
        &mut self,
        self_: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "book-meta.book-meta.get-page-count")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_BOOK)
                .map_err(map_core_host_error)?;
            Ok(item_meta_value(self, self_)
                .map_err(map_core_host_error)?
                .meta
                .pages
                .len() as i32)
        })())
    }

    fn book_meta_has_title(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "book-meta.book-meta.has-title")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_BOOK)
                .map_err(map_core_host_error)?;
            Ok(item_meta_value(self, self_)
                .map_err(map_core_host_error)?
                .meta
                .has_title)
        })())
    }

    fn book_meta_get_title(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "book-meta.book-meta.get-title")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_BOOK)
                .map_err(map_core_host_error)?;
            let meta = item_meta_value(self, self_).map_err(map_core_host_error)?;
            meta.meta
                .has_title
                .then(|| meta.meta.title.clone())
                .ok_or_else(|| map_core_host_error(not_found()))
        })())
    }

    fn book_meta_set_title(
        &mut self,
        self_: u32,
        title: Option<String>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "book-meta.book-meta.set-title")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_BOOK)
                .map_err(map_core_host_error)?;
            let meta = self
                .owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?;
            meta.meta.has_title = title.is_some();
            meta.meta.title = title.unwrap_or_default();
            Ok(())
        })())
    }

    fn book_meta_has_author(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "book-meta.book-meta.has-author")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_BOOK)
                .map_err(map_core_host_error)?;
            Ok(item_meta_value(self, self_)
                .map_err(map_core_host_error)?
                .meta
                .has_author)
        })())
    }

    fn book_meta_get_author(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "book-meta.book-meta.get-author")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_BOOK)
                .map_err(map_core_host_error)?;
            let meta = item_meta_value(self, self_).map_err(map_core_host_error)?;
            meta.meta
                .has_author
                .then(|| meta.meta.author.clone())
                .ok_or_else(|| map_core_host_error(not_found()))
        })())
    }

    fn book_meta_set_author(
        &mut self,
        self_: u32,
        author: Option<String>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "book-meta.book-meta.set-author")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_BOOK)
                .map_err(map_core_host_error)?;
            let meta = self
                .owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?;
            meta.meta.has_author = author.is_some();
            meta.meta.author = author.unwrap_or_default();
            Ok(())
        })())
    }

    fn book_meta_has_generation(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "book-meta.book-meta.has-generation")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_BOOK)
                .map_err(map_core_host_error)?;
            Ok(item_meta_value(self, self_)
                .map_err(map_core_host_error)?
                .meta
                .has_generation)
        })())
    }

    fn book_meta_get_generation(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<BookMetaBookGeneration>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "book-meta.book-meta.get-generation")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_BOOK)
                .map_err(map_core_host_error)?;
            let meta = item_meta_value(self, self_).map_err(map_core_host_error)?;
            if !meta.meta.has_generation {
                return Ok(None);
            }
            match meta.meta.generation {
                0 => Ok(Some(BookMetaBookGeneration::Original)),
                1 => Ok(Some(BookMetaBookGeneration::CopyOfOriginal)),
                2 => Ok(Some(BookMetaBookGeneration::CopyOfCopy)),
                _ => Err(map_core_host_error(HostError::from_status(
                    AEGILEX_INVALID_ARGUMENT,
                ))),
            }
        })())
    }

    fn book_meta_set_generation(
        &mut self,
        self_: u32,
        generation: Option<BookMetaBookGeneration>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "book-meta.book-meta.set-generation")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_BOOK)
                .map_err(map_core_host_error)?;
            let meta = self
                .owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?;
            meta.meta.has_generation = generation.is_some();
            meta.meta.generation = match generation {
                Some(BookMetaBookGeneration::Original) => 0,
                Some(BookMetaBookGeneration::CopyOfOriginal) => 1,
                Some(BookMetaBookGeneration::CopyOfCopy) => 2,
                None => 0,
            };
            Ok(())
        })())
    }

    fn from_item_meta(&mut self, meta: u32) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "book-meta.from-item-meta")?;
            if require_item_meta_kind(self, meta, ITEM_META_KIND_BOOK).is_err() {
                return Ok(None);
            }
            Ok(Some(
                clone_item_meta_resource(self, meta).map_err(map_core_host_error)?,
            ))
        })())
    }

    fn as_item_meta(&mut self, meta: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "book-meta.as-item-meta")?;
            require_item_meta_kind(self, meta, ITEM_META_KIND_BOOK).map_err(map_core_host_error)?;
            clone_item_meta_resource(self, meta).map_err(map_core_host_error)
        })())
    }
}

impl HostCrossbowMeta for PluginStoreState {
    fn crossbow_meta_has_charged_projectiles(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "crossbow-meta.crossbow-meta.has-charged-projectiles")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_CROSSBOW)
                .map_err(map_core_host_error)?;
            Ok(item_meta_value(self, self_)
                .map_err(map_core_host_error)?
                .projectiles
                .len()
                != 0)
        })())
    }

    fn crossbow_meta_get_charged_projectiles(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "crossbow-meta.crossbow-meta.get-charged-projectiles")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_CROSSBOW)
                .map_err(map_core_host_error)?;
            let count = item_meta_value(self, self_)
                .map_err(map_core_host_error)?
                .projectiles
                .len();
            let mut result = Vec::with_capacity(count);
            for index in 0..count {
                let projectile = item_meta_value(self, self_)
                    .map_err(map_core_host_error)?
                    .projectiles
                    .get(index);
                if projectile.is_null() {
                    return Err(map_core_host_error(not_found()));
                }
                result.push(
                    self.insert_item_stack_resource(projectile)
                        .map_err(map_core_host_error)?,
                );
            }
            Ok(result)
        })())
    }

    fn crossbow_meta_set_charged_projectiles(
        &mut self,
        self_: u32,
        projectiles: Vec<u32>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "crossbow-meta.crossbow-meta.set-charged-projectiles")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_CROSSBOW)
                .map_err(map_core_host_error)?;
            let projectiles = projectiles.into_iter().collect::<Vec<_>>();
            let projectiles =
                projectiles_from_resources(self, &projectiles).map_err(map_core_host_error)?;
            self.owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?
                .projectiles = projectiles;
            Ok(())
        })())
    }

    fn crossbow_meta_add_charged_projectile(
        &mut self,
        self_: u32,
        projectile: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "crossbow-meta.crossbow-meta.add-charged-projectile")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_CROSSBOW)
                .map_err(map_core_host_error)?;
            let handle =
                item_stack_resource_handle(self, projectile).map_err(map_core_host_error)?;
            let projectile = self
                .handles
                .item_stack(self.invocation_id, handle)
                .ok_or_else(|| map_core_host_error(not_found()))?
                .cloneItemStack();
            if projectile.is_null() {
                return Err(map_core_host_error(not_found()));
            }
            self.owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?
                .projectiles
                .pin_mut()
                .push(&projectile);
            Ok(())
        })())
    }

    fn from_item_meta(&mut self, meta: u32) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "crossbow-meta.from-item-meta")?;
            if require_item_meta_kind(self, meta, ITEM_META_KIND_CROSSBOW).is_err() {
                return Ok(None);
            }
            Ok(Some(
                clone_item_meta_resource(self, meta).map_err(map_core_host_error)?,
            ))
        })())
    }

    fn as_item_meta(&mut self, meta: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "crossbow-meta.as-item-meta")?;
            require_item_meta_kind(self, meta, ITEM_META_KIND_CROSSBOW)
                .map_err(map_core_host_error)?;
            clone_item_meta_resource(self, meta).map_err(map_core_host_error)
        })())
    }
}

impl HostMapMeta for PluginStoreState {
    fn map_meta_has_map_id(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "map-meta.map-meta.has-map-id")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_MAP).map_err(map_core_host_error)?;
            Ok(item_meta_value(self, self_)
                .map_err(map_core_host_error)?
                .meta
                .has_map_id)
        })())
    }

    fn map_meta_get_map_id(&mut self, self_: u32) -> Result<Result<i64, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "map-meta.map-meta.get-map-id")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_MAP).map_err(map_core_host_error)?;
            let meta = item_meta_value(self, self_).map_err(map_core_host_error)?;
            meta.meta
                .has_map_id
                .then_some(meta.meta.map_id)
                .ok_or_else(|| map_core_host_error(not_found()))
        })())
    }

    fn map_meta_set_map_id(
        &mut self,
        self_: u32,
        id: i64,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "map-meta.map-meta.set-map-id")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_MAP).map_err(map_core_host_error)?;
            let meta = self
                .owned_resource_mut::<ItemMetaValue>(self_)
                .map_err(map_core_host_error)?;
            meta.meta.has_map_id = true;
            meta.meta.map_id = id;
            Ok(())
        })())
    }

    fn map_meta_has_map_view(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "map-meta.map-meta.has-map-view")?;
            require_item_meta_kind(self, self_, ITEM_META_KIND_MAP).map_err(map_core_host_error)?;
            Ok(item_meta_value(self, self_)
                .map_err(map_core_host_error)?
                .meta
                .has_map_view)
        })())
    }

    fn from_item_meta(&mut self, meta: u32) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "map-meta.from-item-meta")?;
            if require_item_meta_kind(self, meta, ITEM_META_KIND_MAP).is_err() {
                return Ok(None);
            }
            Ok(Some(
                clone_item_meta_resource(self, meta).map_err(map_core_host_error)?,
            ))
        })())
    }

    fn as_item_meta(&mut self, meta: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "map-meta.as-item-meta")?;
            require_item_meta_kind(self, meta, ITEM_META_KIND_MAP).map_err(map_core_host_error)?;
            clone_item_meta_resource(self, meta).map_err(map_core_host_error)
        })())
    }
}

impl HostPluginCommand for PluginStoreState {
    fn plugin_command_get_data(
        &mut self,
        self_: u32,
    ) -> Result<Result<PluginCommandPluginCommandData, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "plugin-command.plugin-command.get-data")?;
            let handle = self
                .resource_slot(self_, ResourceKind::PluginCommand)
                .map_err(map_core_host_error)?
                .handle;
            let command = resolve_plugin_command(self, handle).map_err(map_core_host_error)?;
            Ok(PluginCommandPluginCommandData {
                name: command.getName(),
                description: command.getDescription(),
                aliases: command.getAliases(),
                usages: command.getUsages(),
                permissions: command.getPermissions(),
            })
        })())
    }
}

/// Shared body of `language.translate`/`translate-text`/`translate-text-with-parameters`.
fn translate_with_parameters(
    this: &mut PluginStoreState,
    text: String,
    parameters: Vec<String>,
) -> Result<String, TypesHostError> {
    check_capability(this, "language.translate")?;
    let mut out = cxx_server::TranslateResult {
        status: 0,
        value: String::new(),
    };
    let server = resolve_server(this).map_err(map_core_host_error)?;
    native::status_result(server.translate(&text, parameters, &mut out))
        .map_err(map_core_host_error)?;
    Ok(out.value)
}

impl HostLanguage for PluginStoreState {
    fn translate_text(&mut self, text: String) -> Result<Result<String, TypesHostError>, String> {
        Ok(translate_with_parameters(self, text, Vec::new()))
    }

    fn translate_text_with_parameters(
        &mut self,
        text: String,
        parameters: Vec<String>,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok(translate_with_parameters(self, text, parameters))
    }

    fn translate(
        &mut self,
        message: TranslatableTranslatable,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok(translate_with_parameters(
            self,
            message.text,
            message.parameters,
        ))
    }
}
