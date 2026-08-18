//! Core ABI implementation for `native/bindings/endstone/inventory/item_stack.h`.

use crate::host::endstone::inventory::resources::*;
use crate::host::endstone::support::*;

fn empty_nbt_node(kind: u8) -> cxx_inventory::NbtNode {
    cxx_inventory::NbtNode {
        kind,
        byte_value: 0,
        short_value: 0,
        int_value: 0,
        long_value: 0,
        float_value: 0.0,
        double_value: 0.0,
        string_value: String::new(),
        byte_array: Vec::new(),
        int_array: Vec::new(),
        child_indices: Vec::new(),
        entries: Vec::new(),
    }
}

fn nbt_node_kind(node: &cxx_inventory::NbtNode) -> u8 {
    node.kind
}

fn append_nbt_node(
    tag: &NbtTag,
    nodes: &mut Vec<cxx_inventory::NbtNode>,
) -> Result<u32, HostError> {
    let index = u32::try_from(nodes.len()).map_err(|_| limit_exceeded())?;
    nodes.push(empty_nbt_node(NBT_END));
    let node = match tag {
        NbtTag::End => empty_nbt_node(NBT_END),
        NbtTag::Byte(value) => cxx_inventory::NbtNode {
            kind: NBT_BYTE,
            byte_value: *value,
            ..empty_nbt_node(NBT_BYTE)
        },
        NbtTag::Short(value) => cxx_inventory::NbtNode {
            kind: NBT_SHORT,
            short_value: *value,
            ..empty_nbt_node(NBT_SHORT)
        },
        NbtTag::Int(value) => cxx_inventory::NbtNode {
            kind: NBT_INT,
            int_value: *value,
            ..empty_nbt_node(NBT_INT)
        },
        NbtTag::Long(value) => cxx_inventory::NbtNode {
            kind: NBT_LONG,
            long_value: *value,
            ..empty_nbt_node(NBT_LONG)
        },
        NbtTag::Float(value) => cxx_inventory::NbtNode {
            kind: NBT_FLOAT,
            float_value: *value,
            ..empty_nbt_node(NBT_FLOAT)
        },
        NbtTag::Double(value) => cxx_inventory::NbtNode {
            kind: NBT_DOUBLE,
            double_value: *value,
            ..empty_nbt_node(NBT_DOUBLE)
        },
        NbtTag::ByteArray(value) => cxx_inventory::NbtNode {
            kind: NBT_BYTE_ARRAY,
            byte_array: value.clone(),
            ..empty_nbt_node(NBT_BYTE_ARRAY)
        },
        NbtTag::StringValue(value) => cxx_inventory::NbtNode {
            kind: NBT_STRING,
            string_value: value.clone(),
            ..empty_nbt_node(NBT_STRING)
        },
        NbtTag::List(values) => cxx_inventory::NbtNode {
            kind: NBT_LIST,
            child_indices: values
                .iter()
                .map(|value| append_nbt_node(value, nodes))
                .collect::<Result<_, _>>()?,
            ..empty_nbt_node(NBT_LIST)
        },
        NbtTag::Compound(entries) => cxx_inventory::NbtNode {
            kind: NBT_COMPOUND,
            entries: entries
                .iter()
                .map(|(key, value)| {
                    Ok(cxx_inventory::NbtEntry {
                        key: key.clone(),
                        value_index: append_nbt_node(value, nodes)?,
                    })
                })
                .collect::<Result<_, HostError>>()?,
            ..empty_nbt_node(NBT_COMPOUND)
        },
        NbtTag::IntArray(value) => cxx_inventory::NbtNode {
            kind: NBT_INT_ARRAY,
            int_array: value.clone(),
            ..empty_nbt_node(NBT_INT_ARRAY)
        },
    };
    nodes[index as usize] = node;
    Ok(index)
}

fn validate_nbt_node(
    nodes: &[cxx_inventory::NbtNode],
    index: usize,
    depth: usize,
    visit_state: &mut [u8],
    config: &crate::config::RuntimeConfig,
) -> bool {
    if exceeds_limit(depth, config.max_nbt_depth) || index >= nodes.len() || visit_state[index] != 0
    {
        return false;
    }
    visit_state[index] = 1;
    let valid = match nodes[index].kind {
        NBT_LIST => nodes[index].child_indices.iter().all(|child| {
            usize::try_from(*child).ok().is_some_and(|child| {
                validate_nbt_node(nodes, child, depth + 1, visit_state, config)
            })
        }),
        NBT_COMPOUND => nodes[index].entries.iter().all(|entry| {
            usize::try_from(entry.value_index)
                .ok()
                .is_some_and(|child| {
                    validate_nbt_node(nodes, child, depth + 1, visit_state, config)
                })
        }),
        _ => true,
    };
    if valid {
        visit_state[index] = 2;
    }
    valid
}

fn validate_nbt_tree(
    value: &cxx_inventory::Nbt,
    state: &PluginStoreState,
) -> Result<(), HostError> {
    if value.nodes.is_empty() {
        return Err(invalid_input());
    }
    if exceeds_limit(value.nodes.len(), state.config.max_nbt_nodes) {
        return Err(limit_exceeded());
    }
    let root = usize::try_from(value.root_index).map_err(|_| invalid_input())?;
    if root >= value.nodes.len() {
        return Err(invalid_input());
    }
    for node in &value.nodes {
        match node.kind {
            NBT_BYTE_ARRAY
                if exceeds_limit(node.byte_array.len(), state.config.max_nbt_array_bytes) =>
            {
                return Err(limit_exceeded());
            }
            NBT_INT_ARRAY
                if exceeds_limit(
                    node.int_array
                        .len()
                        .saturating_mul(std::mem::size_of::<i32>()),
                    state.config.max_nbt_array_bytes,
                ) =>
            {
                return Err(limit_exceeded());
            }
            NBT_STRING
                if exceeds_limit(node.string_value.len(), state.config.max_nbt_string_bytes) =>
            {
                return Err(limit_exceeded());
            }
            NBT_LIST => {
                let mut element_kind = None;
                for child in &node.child_indices {
                    let child = usize::try_from(*child).map_err(|_| invalid_input())?;
                    let Some(child) = value.nodes.get(child) else {
                        return Err(invalid_input());
                    };
                    if nbt_node_kind(child) == NBT_END {
                        return Err(invalid_input());
                    }
                    if let Some(kind) = element_kind {
                        if kind != nbt_node_kind(child) {
                            return Err(invalid_input());
                        }
                    } else {
                        element_kind = Some(nbt_node_kind(child));
                    }
                }
            }
            NBT_COMPOUND => {
                if exceeds_limit(node.entries.len(), state.config.max_nbt_compound_entries) {
                    return Err(limit_exceeded());
                }
                let mut keys = std::collections::HashSet::with_capacity(node.entries.len());
                for entry in &node.entries {
                    if exceeds_limit(entry.key.len(), state.config.max_nbt_string_bytes) {
                        return Err(limit_exceeded());
                    }
                    if !keys.insert(entry.key.as_str())
                        || usize::try_from(entry.value_index)
                            .ok()
                            .is_none_or(|child| child >= value.nodes.len())
                    {
                        return Err(invalid_input());
                    }
                }
            }
            _ => {}
        }
    }
    let mut visit_state = vec![0; value.nodes.len()];
    if !validate_nbt_node(&value.nodes, root, 0, &mut visit_state, &state.config)
        || visit_state.iter().any(|state| *state != 2)
    {
        return Err(invalid_input());
    }
    Ok(())
}

fn nbt_to_cxx(value: &NbtTag, state: &PluginStoreState) -> Result<cxx_inventory::Nbt, HostError> {
    if !matches!(value, NbtTag::Compound(_)) {
        return Err(invalid_input());
    }
    validate_nbt_tag_root(value, state)?;
    let mut nodes = Vec::new();
    let root_index = append_nbt_node(value, &mut nodes)?;
    Ok(cxx_inventory::Nbt { root_index, nodes })
}

fn nbt_from_node(nodes: &[cxx_inventory::NbtNode], index: usize) -> Result<NbtTag, HostError> {
    let node = &nodes[index];
    match node.kind {
        NBT_END => Ok(NbtTag::End),
        NBT_BYTE => Ok(NbtTag::Byte(node.byte_value)),
        NBT_SHORT => Ok(NbtTag::Short(node.short_value)),
        NBT_INT => Ok(NbtTag::Int(node.int_value)),
        NBT_LONG => Ok(NbtTag::Long(node.long_value)),
        NBT_FLOAT => Ok(NbtTag::Float(node.float_value)),
        NBT_DOUBLE => Ok(NbtTag::Double(node.double_value)),
        NBT_BYTE_ARRAY => Ok(NbtTag::ByteArray(node.byte_array.clone())),
        NBT_STRING => Ok(NbtTag::StringValue(node.string_value.clone())),
        NBT_LIST => node
            .child_indices
            .iter()
            .map(|child| nbt_from_node(nodes, *child as usize))
            .collect::<Result<Vec<_>, _>>()
            .map(NbtTag::List),
        NBT_COMPOUND => node
            .entries
            .iter()
            .map(|entry| {
                Ok((
                    entry.key.clone(),
                    nbt_from_node(nodes, entry.value_index as usize)?,
                ))
            })
            .collect::<Result<BTreeMap<_, _>, HostError>>()
            .map(NbtTag::Compound),
        NBT_INT_ARRAY => Ok(NbtTag::IntArray(node.int_array.clone())),
        _ => Err(invalid_input()),
    }
}

fn nbt_from_cxx(value: cxx_inventory::Nbt, state: &PluginStoreState) -> Result<NbtTag, HostError> {
    if value.nodes.is_empty() {
        return Err(invalid_input());
    }
    if exceeds_limit(value.nodes.len(), state.config.max_nbt_nodes) {
        return Err(limit_exceeded());
    }
    let root = usize::try_from(value.root_index).map_err(|_| invalid_input())?;
    if root >= value.nodes.len() {
        return Err(invalid_input());
    }
    validate_nbt_tree(&value, state)?;
    nbt_from_node(&value.nodes, root)
}

impl crate::core_host::imports::HostItemStack for PluginStoreState {
    fn item_stack_get_type_id(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack.get-type-id")?;
            let handle = item_stack_resource_handle(self, self_).map_err(map_core_host_error)?;
            self.handles
                .item_stack(self.invocation_id, handle)
                .map(|item| item.getType())
                .ok_or_else(not_found)
                .map_err(map_core_host_error)
        })())
    }

    fn item_stack_set_type(
        &mut self,
        self_: u32,
        type_id: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack.set-type")?;
            let handle = item_stack_resource_handle(self, self_).map_err(map_core_host_error)?;
            self.handles
                .item_stack(self.invocation_id, handle)
                .and_then(|item| item.setType(&type_id).then_some(()))
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn item_stack_get_amount(&mut self, self_: u32) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack.get-amount")?;
            let handle = item_stack_resource_handle(self, self_).map_err(map_core_host_error)?;
            self.handles
                .item_stack(self.invocation_id, handle)
                .map(|item| item.getAmount())
                .ok_or_else(not_found)
                .map_err(map_core_host_error)
        })())
    }

    fn item_stack_set_amount(
        &mut self,
        self_: u32,
        amount: i32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack.set-amount")?;
            let handle = item_stack_resource_handle(self, self_).map_err(map_core_host_error)?;
            self.handles
                .item_stack(self.invocation_id, handle)
                .map(|item| item.setAmount(amount))
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn item_stack_get_data(&mut self, self_: u32) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack.get-data")?;
            let handle = item_stack_resource_handle(self, self_).map_err(map_core_host_error)?;
            self.handles
                .item_stack(self.invocation_id, handle)
                .map(|item| item.getData())
                .ok_or_else(not_found)
                .map_err(map_core_host_error)
        })())
    }

    fn item_stack_set_data(
        &mut self,
        self_: u32,
        data: i32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack.set-data")?;
            let handle = item_stack_resource_handle(self, self_).map_err(map_core_host_error)?;
            self.handles
                .item_stack(self.invocation_id, handle)
                .map(|item| item.setData(data))
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }

    fn item_stack_get_translation_key(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack.get-translation-key")?;
            let handle = item_stack_resource_handle(self, self_).map_err(map_core_host_error)?;
            self.handles
                .item_stack(self.invocation_id, handle)
                .map(|item| item.getTranslationKey())
                .ok_or_else(not_found)
                .map_err(map_core_host_error)
        })())
    }

    fn item_stack_get_max_stack_size(
        &mut self,
        self_: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack.get-max-stack-size")?;
            let handle = item_stack_resource_handle(self, self_).map_err(map_core_host_error)?;
            self.handles
                .item_stack(self.invocation_id, handle)
                .map(|item| item.getMaxStackSize())
                .ok_or_else(not_found)
                .map_err(map_core_host_error)
        })())
    }

    fn item_stack_clone(&mut self, self_: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack.clone")?;
            let handle = item_stack_resource_handle(self, self_).map_err(map_core_host_error)?;
            let copy = self
                .handles
                .item_stack(self.invocation_id, handle)
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?
                .cloneItemStack();
            if copy.is_null() {
                return Err(map_core_host_error(not_found()));
            }
            self.insert_item_stack_resource(copy)
                .map_err(map_core_host_error)
        })())
    }

    fn item_stack_is_similar(
        &mut self,
        self_: u32,
        other: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack.is-similar")?;
            let item_handle =
                item_stack_resource_handle(self, self_).map_err(map_core_host_error)?;
            let other_handle =
                item_stack_resource_handle(self, other).map_err(map_core_host_error)?;
            let item = self
                .handles
                .item_stack(self.invocation_id, item_handle)
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            let other = self
                .handles
                .item_stack(self.invocation_id, other_handle)
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            Ok(item.isSimilar(other))
        })())
    }

    fn item_stack_get_meta(&mut self, self_: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack.get-meta")?;
            let handle = item_stack_resource_handle(self, self_).map_err(map_core_host_error)?;
            let mut meta = cxx_meta_defaults();
            let projectiles = {
                let item = self
                    .handles
                    .item_stack(self.invocation_id, handle)
                    .ok_or_else(not_found)
                    .map_err(map_core_host_error)?;
                if !item.getMeta(&mut meta) {
                    return Err(map_core_host_error(not_found()));
                }
                item.getChargedProjectiles()
            };
            if projectiles.is_null() {
                return Err(map_core_host_error(not_found()));
            }
            insert_item_meta_resource(self, meta, projectiles).map_err(map_core_host_error)
        })())
    }

    fn item_stack_has_meta(&mut self, self_: u32) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack.has-meta")?;
            let handle = item_stack_resource_handle(self, self_).map_err(map_core_host_error)?;
            self.handles
                .item_stack(self.invocation_id, handle)
                .map(|item| item.hasItemMeta())
                .ok_or_else(not_found)
                .map_err(map_core_host_error)
        })())
    }

    fn item_stack_set_meta(
        &mut self,
        self_: u32,
        meta: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack.set-meta")?;
            let meta = item_meta_value(self, meta).map_err(map_core_host_error)?;
            let handle = item_stack_resource_handle(self, self_).map_err(map_core_host_error)?;
            let item = self
                .handles
                .item_stack(self.invocation_id, handle)
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            let mut out = false;
            let server = resolve_server(self).map_err(map_core_host_error)?;
            if !item.setMeta(server, &meta.meta, &meta.projectiles, &mut out) {
                return Err(map_core_host_error(not_found()));
            }
            Ok(out)
        })())
    }

    fn item_stack_get_nbt(&mut self, self_: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack.get-nbt")?;
            let handle = item_stack_resource_handle(self, self_).map_err(map_core_host_error)?;
            let mut nbt = cxx_inventory::Nbt {
                root_index: 0,
                nodes: Vec::new(),
            };
            let item = self
                .handles
                .item_stack(self.invocation_id, handle)
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            if !item.getNbt(&mut nbt) {
                return Err(map_core_host_error(not_found()));
            }
            let value = nbt_from_cxx(nbt, self).map_err(map_core_host_error)?;
            insert_nbt_tag_resource(self, value).map_err(map_core_host_error)
        })())
    }

    fn item_stack_set_nbt(
        &mut self,
        self_: u32,
        value: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack.set-nbt")?;
            let value = nbt_tag_value(self, value).map_err(map_core_host_error)?;
            let value = nbt_to_cxx(value, self).map_err(map_core_host_error)?;
            let handle = item_stack_resource_handle(self, self_).map_err(map_core_host_error)?;
            let item = self
                .handles
                .item_stack(self.invocation_id, handle)
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            if !item.setNbt(&value) {
                return Err(map_core_host_error(invalid_input()));
            }
            Ok(())
        })())
    }

    fn item_stack_can_enchant(
        &mut self,
        self_: u32,
        enchantment: String,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack.can-enchant")?;
            let handle = item_stack_resource_handle(self, self_).map_err(map_core_host_error)?;
            let item = self
                .handles
                .item_stack(self.invocation_id, handle)
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            let server = resolve_server(self).map_err(map_core_host_error)?;
            let enchantment = server.getRegistryEnchantment(&enchantment);
            if enchantment.is_null() {
                return Err(map_core_host_error(not_found()));
            }
            Ok(enchantment.canEnchantItem(item))
        })())
    }

    fn item_stack_ref_get_type_id(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack-ref.get-type-id")?;
            resolve_item_stack_ref(
                self,
                item_stack_ref_resource_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|item| item.getType())
            .map_err(map_core_host_error)
        })())
    }

    fn item_stack_ref_get_amount(
        &mut self,
        self_: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack-ref.get-amount")?;
            resolve_item_stack_ref(
                self,
                item_stack_ref_resource_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|item| item.getAmount())
            .map_err(map_core_host_error)
        })())
    }

    fn item_stack_ref_get_data(
        &mut self,
        self_: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack-ref.get-data")?;
            resolve_item_stack_ref(
                self,
                item_stack_ref_resource_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|item| item.getData())
            .map_err(map_core_host_error)
        })())
    }

    fn item_stack_ref_get_translation_key(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack-ref.get-translation-key")?;
            resolve_item_stack_ref(
                self,
                item_stack_ref_resource_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|item| item.getTranslationKey())
            .map_err(map_core_host_error)
        })())
    }

    fn item_stack_ref_get_max_stack_size(
        &mut self,
        self_: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack-ref.get-max-stack-size")?;
            resolve_item_stack_ref(
                self,
                item_stack_ref_resource_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|item| item.getMaxStackSize())
            .map_err(map_core_host_error)
        })())
    }

    fn item_stack_ref_clone(&mut self, self_: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack-ref.clone")?;
            let copy = resolve_item_stack_ref(
                self,
                item_stack_ref_resource_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|item| item.cloneItemStack())
            .map_err(map_core_host_error)?;
            if copy.is_null() {
                return Err(map_core_host_error(not_found()));
            }
            self.insert_item_stack_resource(copy)
                .map_err(map_core_host_error)
        })())
    }

    fn item_stack_ref_is_similar(
        &mut self,
        self_: u32,
        other: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack-ref.is-similar")?;
            let item = resolve_item_stack_ref(
                self,
                item_stack_ref_resource_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            let other = self
                .handles
                .item_stack(
                    self.invocation_id,
                    item_stack_resource_handle(self, other).map_err(map_core_host_error)?,
                )
                .ok_or_else(not_found)
                .map_err(map_core_host_error)?;
            Ok(item.isSimilar(other))
        })())
    }

    fn item_stack_ref_get_meta(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack-ref.get-meta")?;
            let mut meta = cxx_meta_defaults();
            let item = resolve_item_stack_ref(
                self,
                item_stack_ref_resource_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            if !item.getMeta(&mut meta) {
                return Err(map_core_host_error(not_found()));
            }
            let projectiles = item.getChargedProjectiles();
            if projectiles.is_null() {
                return Err(map_core_host_error(not_found()));
            }
            insert_item_meta_resource(self, meta, projectiles).map_err(map_core_host_error)
        })())
    }

    fn item_stack_ref_has_meta(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack-ref.has-meta")?;
            resolve_item_stack_ref(
                self,
                item_stack_ref_resource_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|item| item.hasItemMeta())
            .map_err(map_core_host_error)
        })())
    }

    fn item_stack_ref_get_nbt(
        &mut self,
        self_: u32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "item-stack.item-stack-ref.get-nbt")?;
            let mut nbt = cxx_inventory::Nbt {
                root_index: 0,
                nodes: Vec::new(),
            };
            let item = resolve_item_stack_ref(
                self,
                item_stack_ref_resource_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            if !item.getNbt(&mut nbt) {
                return Err(map_core_host_error(not_found()));
            }
            let value = nbt_from_cxx(nbt, self).map_err(map_core_host_error)?;
            insert_nbt_tag_resource(self, value).map_err(map_core_host_error)
        })())
    }
    fn drop_item_stack(&mut self, handle: u32) -> Result<(), String> {
        self.drop_resource(handle, ResourceKind::ItemStack)
            .map_err(|error| format!("{error:?}"))
    }
}
