//! Core ABI implementation for `native/bindings/endstone/inventory/nbt.h`.

use crate::host::endstone::support::*;

fn invalid_input() -> HostError {
    HostError::from_status(AEGILEX_INVALID_ARGUMENT)
}

// Container setters copy the borrowed child so a child resource can never
// reference its own parent, which would otherwise allow a cycle.

impl HostNbt for PluginStoreState {
    fn tag_get_type(&mut self, self_: u32) -> Result<Result<NbtTagType, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.get-type")?;
            let tag = self_;
            nbt_tag_value(self, tag)
                .map(nbt_tag_type)
                .map_err(map_core_host_error)
        })())
    }

    fn tag_get_byte(&mut self, self_: u32) -> Result<Result<u8, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.get-byte")?;
            let tag = self_;
            match nbt_tag_value(self, tag).map_err(map_core_host_error)? {
                NbtTag::Byte(value) => Ok(*value),
                _ => Err(map_core_host_error(invalid_input())),
            }
        })())
    }

    fn tag_get_short(&mut self, self_: u32) -> Result<Result<i16, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.get-short")?;
            let tag = self_;
            match nbt_tag_value(self, tag).map_err(map_core_host_error)? {
                NbtTag::Short(value) => Ok(*value),
                _ => Err(map_core_host_error(invalid_input())),
            }
        })())
    }

    fn tag_get_int(&mut self, self_: u32) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.get-int")?;
            let tag = self_;
            match nbt_tag_value(self, tag).map_err(map_core_host_error)? {
                NbtTag::Int(value) => Ok(*value),
                _ => Err(map_core_host_error(invalid_input())),
            }
        })())
    }

    fn tag_get_long(&mut self, self_: u32) -> Result<Result<i64, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.get-long")?;
            let tag = self_;
            match nbt_tag_value(self, tag).map_err(map_core_host_error)? {
                NbtTag::Long(value) => Ok(*value),
                _ => Err(map_core_host_error(invalid_input())),
            }
        })())
    }

    fn tag_get_float(&mut self, self_: u32) -> Result<Result<f32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.get-float")?;
            let tag = self_;
            match nbt_tag_value(self, tag).map_err(map_core_host_error)? {
                NbtTag::Float(value) => Ok(*value),
                _ => Err(map_core_host_error(invalid_input())),
            }
        })())
    }

    fn tag_get_double(&mut self, self_: u32) -> Result<Result<f64, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.get-double")?;
            let tag = self_;
            match nbt_tag_value(self, tag).map_err(map_core_host_error)? {
                NbtTag::Double(value) => Ok(*value),
                _ => Err(map_core_host_error(invalid_input())),
            }
        })())
    }

    fn tag_get_byte_array(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<u8>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.get-byte-array")?;
            let tag = self_;
            match nbt_tag_value(self, tag).map_err(map_core_host_error)? {
                NbtTag::ByteArray(value) => Ok(value.clone()),
                _ => Err(map_core_host_error(invalid_input())),
            }
        })())
    }

    fn tag_get_string(&mut self, self_: u32) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.get-string")?;
            let tag = self_;
            match nbt_tag_value(self, tag).map_err(map_core_host_error)? {
                NbtTag::StringValue(value) => Ok(value.clone()),
                _ => Err(map_core_host_error(invalid_input())),
            }
        })())
    }

    fn tag_get_int_array(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<i32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.get-int-array")?;
            let tag = self_;
            match nbt_tag_value(self, tag).map_err(map_core_host_error)? {
                NbtTag::IntArray(value) => Ok(value.clone()),
                _ => Err(map_core_host_error(invalid_input())),
            }
        })())
    }

    fn tag_get_list_size(&mut self, self_: u32) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.get-list-size")?;
            let tag = self_;
            match nbt_tag_value(self, tag).map_err(map_core_host_error)? {
                NbtTag::List(values) => {
                    i32::try_from(values.len()).map_err(|_| map_core_host_error(limit_exceeded()))
                }
                _ => Err(map_core_host_error(invalid_input())),
            }
        })())
    }

    fn tag_get_list_entry(
        &mut self,
        self_: u32,
        index: i32,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.get-list-entry")?;
            let tag = self_;
            let index = usize::try_from(index).map_err(|_| map_core_host_error(invalid_input()))?;
            let value = match nbt_tag_value(self, tag).map_err(map_core_host_error)? {
                NbtTag::List(values) => values.get(index).cloned(),
                _ => return Err(map_core_host_error(invalid_input())),
            };
            match value {
                Some(value) => insert_nbt_tag_resource(self, value).map_err(map_core_host_error),
                None => Err(map_core_host_error(invalid_input())),
            }
        })())
    }

    fn tag_get_compound_keys(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<String>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.get-compound-keys")?;
            let tag = self_;
            match nbt_tag_value(self, tag).map_err(map_core_host_error)? {
                NbtTag::Compound(entries) => Ok(entries.keys().cloned().collect()),
                _ => Err(map_core_host_error(invalid_input())),
            }
        })())
    }

    fn tag_get_compound(
        &mut self,
        self_: u32,
        key: String,
    ) -> Result<Result<Option<u32>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.get-compound")?;
            let tag = self_;
            let value = match nbt_tag_value(self, tag).map_err(map_core_host_error)? {
                NbtTag::Compound(entries) => entries.get(&key).cloned(),
                _ => return Err(map_core_host_error(invalid_input())),
            };
            match value {
                Some(value) => insert_nbt_tag_resource(self, value)
                    .map(Some)
                    .map_err(map_core_host_error),
                None => Ok(None),
            }
        })())
    }

    fn tag_set_byte(
        &mut self,
        self_: u32,
        value: u8,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.set-byte")?;
            let tag = self_;
            replace_nbt_tag_value(self, tag, NbtTag::Byte(value)).map_err(map_core_host_error)
        })())
    }

    fn tag_set_short(
        &mut self,
        self_: u32,
        value: i16,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.set-short")?;
            let tag = self_;
            replace_nbt_tag_value(self, tag, NbtTag::Short(value)).map_err(map_core_host_error)
        })())
    }

    fn tag_set_int(
        &mut self,
        self_: u32,
        value: i32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.set-int")?;
            let tag = self_;
            replace_nbt_tag_value(self, tag, NbtTag::Int(value)).map_err(map_core_host_error)
        })())
    }

    fn tag_set_long(
        &mut self,
        self_: u32,
        value: i64,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.set-long")?;
            let tag = self_;
            replace_nbt_tag_value(self, tag, NbtTag::Long(value)).map_err(map_core_host_error)
        })())
    }

    fn tag_set_float(
        &mut self,
        self_: u32,
        value: f32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.set-float")?;
            let tag = self_;
            replace_nbt_tag_value(self, tag, NbtTag::Float(value)).map_err(map_core_host_error)
        })())
    }

    fn tag_set_double(
        &mut self,
        self_: u32,
        value: f64,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.set-double")?;
            let tag = self_;
            replace_nbt_tag_value(self, tag, NbtTag::Double(value)).map_err(map_core_host_error)
        })())
    }

    fn tag_set_byte_array(
        &mut self,
        self_: u32,
        value: Vec<u8>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.set-byte-array")?;
            let tag = self_;
            replace_nbt_tag_value(self, tag, NbtTag::ByteArray(value)).map_err(map_core_host_error)
        })())
    }

    fn tag_set_string(
        &mut self,
        self_: u32,
        value: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.set-string")?;
            let tag = self_;
            replace_nbt_tag_value(self, tag, NbtTag::StringValue(value))
                .map_err(map_core_host_error)
        })())
    }

    fn tag_set_int_array(
        &mut self,
        self_: u32,
        value: Vec<i32>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.set-int-array")?;
            let tag = self_;
            replace_nbt_tag_value(self, tag, NbtTag::IntArray(value)).map_err(map_core_host_error)
        })())
    }

    fn tag_set_list(
        &mut self,
        self_: u32,
        values: Vec<u32>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.set-list")?;
            let tag = self_;
            let built = values
                .iter()
                .map(|child| clone_borrowed_nbt_tag(self, tag, *child))
                .collect::<Result<Vec<_>, HostError>>()
                .map_err(map_core_host_error)?;
            replace_nbt_tag_value(self, tag, NbtTag::List(built)).map_err(map_core_host_error)
        })())
    }

    fn tag_list_set(
        &mut self,
        self_: u32,
        index: i32,
        value: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.list-set")?;
            let tag = self_;
            nbt_list_set(self, tag, index, value).map_err(map_core_host_error)
        })())
    }

    fn tag_list_remove(
        &mut self,
        self_: u32,
        index: i32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.list-remove")?;
            let tag = self_;
            nbt_list_remove(self, tag, index).map_err(map_core_host_error)
        })())
    }

    fn tag_list_clear(&mut self, self_: u32) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.list-clear")?;
            let tag = self_;
            nbt_list_clear(self, tag).map_err(map_core_host_error)
        })())
    }

    fn tag_set_compound(
        &mut self,
        self_: u32,
        entries: Vec<NbtCompoundEntry>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.set-compound")?;
            let tag = self_;
            let mut built = BTreeMap::new();
            for entry in entries {
                let value =
                    clone_borrowed_nbt_tag(self, tag, entry.value).map_err(map_core_host_error)?;
                if built.insert(entry.key, value).is_some() {
                    return Err(map_core_host_error(invalid_input()));
                }
            }
            replace_nbt_tag_value(self, tag, NbtTag::Compound(built)).map_err(map_core_host_error)
        })())
    }

    fn tag_compound_set(
        &mut self,
        self_: u32,
        key: String,
        value: u32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.compound-set")?;
            let tag = self_;
            nbt_compound_set(self, tag, key, value).map_err(map_core_host_error)
        })())
    }

    fn tag_compound_remove(
        &mut self,
        self_: u32,
        key: String,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.compound-remove")?;
            let tag = self_;
            nbt_compound_remove(self, tag, key).map_err(map_core_host_error)
        })())
    }

    fn tag_compound_clear(&mut self, self_: u32) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.tag.compound-clear")?;
            let tag = self_;
            nbt_compound_clear(self, tag).map_err(map_core_host_error)
        })())
    }

    fn from_end(&mut self) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.from-end")?;
            insert_nbt_tag_resource(self, NbtTag::End).map_err(map_core_host_error)
        })())
    }

    fn from_byte(&mut self, value: u8) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.from-byte")?;
            insert_nbt_tag_resource(self, NbtTag::Byte(value)).map_err(map_core_host_error)
        })())
    }

    fn from_short(&mut self, value: i16) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.from-short")?;
            insert_nbt_tag_resource(self, NbtTag::Short(value)).map_err(map_core_host_error)
        })())
    }

    fn from_int(&mut self, value: i32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.from-int")?;
            insert_nbt_tag_resource(self, NbtTag::Int(value)).map_err(map_core_host_error)
        })())
    }

    fn from_long(&mut self, value: i64) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.from-long")?;
            insert_nbt_tag_resource(self, NbtTag::Long(value)).map_err(map_core_host_error)
        })())
    }

    fn from_float(&mut self, value: f32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.from-float")?;
            insert_nbt_tag_resource(self, NbtTag::Float(value)).map_err(map_core_host_error)
        })())
    }

    fn from_double(&mut self, value: f64) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.from-double")?;
            insert_nbt_tag_resource(self, NbtTag::Double(value)).map_err(map_core_host_error)
        })())
    }

    fn from_byte_array(&mut self, value: Vec<u8>) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.from-byte-array")?;
            insert_nbt_tag_resource(self, NbtTag::ByteArray(value)).map_err(map_core_host_error)
        })())
    }

    fn from_string(&mut self, value: String) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.from-string")?;
            insert_nbt_tag_resource(self, NbtTag::StringValue(value)).map_err(map_core_host_error)
        })())
    }

    fn from_int_array(&mut self, value: Vec<i32>) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.from-int-array")?;
            insert_nbt_tag_resource(self, NbtTag::IntArray(value)).map_err(map_core_host_error)
        })())
    }

    fn from_list(&mut self, values: Vec<u32>) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.from-list")?;
            let built = values
                .iter()
                .map(|child| nbt_tag_value(self, *child).cloned())
                .collect::<Result<Vec<_>, HostError>>()
                .map_err(map_core_host_error)?;
            insert_nbt_tag_resource(self, NbtTag::List(built)).map_err(map_core_host_error)
        })())
    }

    fn from_compound(
        &mut self,
        entries: Vec<NbtCompoundEntry>,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "nbt.from-compound")?;
            let mut built = BTreeMap::new();
            for entry in entries {
                let value = nbt_tag_value(self, entry.value)
                    .map_err(map_core_host_error)?
                    .clone();
                if built.insert(entry.key, value).is_some() {
                    return Err(map_core_host_error(invalid_input()));
                }
            }
            insert_nbt_tag_resource(self, NbtTag::Compound(built)).map_err(map_core_host_error)
        })())
    }
}
