//! Host-import trampoline generator driven by `wit-bindgen-core` instructions.
//!
//! Translates the canonical-ABI instruction stream for an imported function
//! (`AbiVariant::GuestImport`, `LowerArgsLiftResults`) into Rust host code:
//! lift flat wasm parameters through `Caller` memory into typed Rust values,
//! invoke a generated host trait method, and lower the result through the
//! guest's `cabi_realloc` into the return area.

use std::fmt::Write as _;

use wit_bindgen_core::abi::{Bitcast, Instruction};
use wit_parser::abi::WasmType;
use wit_parser::{ArchitectureSize, Function, Resolve, SizeAlign, Type, TypeDefKind, TypeOwner};

pub(crate) struct HostImportBindgen<'a> {
    resolve: &'a Resolve,
    sizes: &'a SizeAlign,
    pub(crate) out: String,
    blocks: Vec<String>,
    tmp: usize,
    trait_name: String,
    module_name: String,
    function_name: String,
    function: &'a Function,
    arg_names: Vec<String>,
    retptr: Option<String>,
}

impl<'a> HostImportBindgen<'a> {
    pub(crate) fn new(
        resolve: &'a Resolve,
        sizes: &'a SizeAlign,
        trait_name: &str,
        module_name: &str,
        function: &'a Function,
        arg_count: usize,
    ) -> Self {
        Self {
            resolve,
            sizes,
            out: String::new(),
            blocks: Vec::new(),
            tmp: 0,
            trait_name: trait_name.to_owned(),
            module_name: module_name.to_owned(),
            function_name: match &function.kind {
                wit_parser::FunctionKind::Method(resource) => {
                    let resource_name = resolve.types[*resource]
                        .name
                        .as_deref()
                        .expect("resource name")
                        .replace('-', "_");
                    format!(
                        "{}_{}",
                        resource_name,
                        function.item_name().replace('-', "_")
                    )
                }
                _ => function.item_name().replace('-', "_"),
            },
            function,
            arg_names: (0..arg_count).map(|index| format!("arg{index}")).collect(),
            retptr: None,
        }
    }

    fn tmp(&mut self) -> usize {
        let tmp = self.tmp;
        self.tmp += 1;
        tmp
    }

    fn bind(&mut self, expression: String) -> String {
        let tmp = self.tmp();
        let name = format!("t{tmp}");
        let _ = writeln!(self.out, "let {name} = {expression};");
        name
    }

    fn emit_statement(&mut self, statement: String) {
        let _ = writeln!(self.out, "{statement}");
    }

    fn size32(&self, ty: &Type) -> usize {
        self.sizes.size(ty).size_wasm32()
    }

    fn align32(&self, ty: &Type) -> usize {
        self.sizes.align(ty).align_wasm32()
    }

    fn offset32(&self, offset: &ArchitectureSize) -> usize {
        offset.size_wasm32()
    }

    fn sanitize_field(name: &str) -> String {
        let snake = name.replace('-', "_");
        match snake.as_str() {
            "type" | "match" | "ref" | "move" | "box" | "as" | "if" | "else" | "in" | "for"
            | "loop" | "while" | "where" | "impl" | "trait" | "fn" | "let" | "static" | "const"
            | "pub" | "use" | "mod" | "struct" | "enum" | "unsafe" | "async" | "await" | "dyn"
            | "super" | "crate" | "return" | "break" | "continue" => {
                format!("r#{snake}")
            }
            "self" => "self_".to_owned(),
            _ => snake,
        }
    }

    fn type_name(&self, id: wit_parser::TypeId) -> String {
        let name = self.resolve.types[id].name.clone().expect("named type");
        let interface = match self.resolve.types[id].owner {
            TypeOwner::Interface(iface) => self.resolve.interfaces[iface]
                .name
                .clone()
                .expect("interface name"),
            _ => {
                // Search world imports/exports for the owning interface.
                let mut found = None;
                for (_, world) in self.resolve.worlds.iter() {
                    for item in world.imports.values().chain(world.exports.values()) {
                        if let wit_parser::WorldItem::Interface { id: iface, .. } = item
                            && self.resolve.interfaces[*iface]
                                .types
                                .values()
                                .any(|ty| *ty == id)
                        {
                            found = Some(
                                self.resolve.interfaces[*iface]
                                    .name
                                    .clone()
                                    .expect("interface name"),
                            );
                            break;
                        }
                    }
                }
                found.expect("type belongs to an interface")
            }
        };
        let camel = |part: &str| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        };
        format!(
            "{}{}",
            interface.split(['-', '_']).map(camel).collect::<String>(),
            name.split(['-', '_']).map(camel).collect::<String>()
        )
    }

    fn rust_type(&self, ty: &Type) -> String {
        match ty {
            Type::Bool => "bool".to_owned(),
            Type::U8 => "u8".to_owned(),
            Type::U16 => "u16".to_owned(),
            Type::U32 => "u32".to_owned(),
            Type::U64 => "u64".to_owned(),
            Type::S8 => "i8".to_owned(),
            Type::S16 => "i16".to_owned(),
            Type::S32 => "i32".to_owned(),
            Type::S64 => "i64".to_owned(),
            Type::F32 => "f32".to_owned(),
            Type::F64 => "f64".to_owned(),
            Type::Char => "char".to_owned(),
            Type::ErrorContext => "u32".to_owned(),
            Type::String => "String".to_owned(),
            Type::Id(id) => match &self.resolve.types[*id].kind {
                TypeDefKind::Type(inner) => self.rust_type(inner),
                TypeDefKind::List(inner) => format!("Vec<{}>", self.rust_type(inner)),
                TypeDefKind::Option(inner) => format!("Option<{}>", self.rust_type(inner)),
                TypeDefKind::Result(result) => {
                    let ok = result
                        .ok
                        .as_ref()
                        .map(|ty| self.rust_type(ty))
                        .unwrap_or_else(|| "()".to_owned());
                    let err = result
                        .err
                        .as_ref()
                        .map(|ty| self.rust_type(ty))
                        .unwrap_or_else(|| "()".to_owned());
                    format!("Result<{ok}, {err}>")
                }
                TypeDefKind::Tuple(tuple) => {
                    let fields = tuple
                        .types
                        .iter()
                        .map(|ty| self.rust_type(ty))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("({fields})")
                }
                TypeDefKind::Handle(_) => "u32".to_owned(),
                _ => self.type_name(*id),
            },
        }
    }

    fn load(&mut self, wasm: WasmType, ptr: &str, offset: usize, signed: bool) -> String {
        let read = match (wasm, signed) {
            (WasmType::I32, false) => format!("read_i32_at(&mut caller, &mem, {ptr}, {offset})?"),
            (WasmType::I32, true) => format!("read_i32_at(&mut caller, &mem, {ptr}, {offset})?"),
            (WasmType::I64, _) => format!("read_i64_at(&mut caller, &mem, {ptr}, {offset})?"),
            (WasmType::F32, _) => format!("read_f32_at(&mut caller, &mem, {ptr}, {offset})?"),
            (WasmType::F64, _) => format!("read_f64_at(&mut caller, &mem, {ptr}, {offset})?"),
            (WasmType::Pointer, _) => format!("read_i32_at(&mut caller, &mem, {ptr}, {offset})?"),
            (WasmType::Length, _) => format!("read_i32_at(&mut caller, &mem, {ptr}, {offset})?"),
            (WasmType::PointerOrI64, _) => {
                format!("read_i64_at(&mut caller, &mem, {ptr}, {offset})?")
            }
        };
        let value = self.bind(read);
        match wasm {
            WasmType::I32 | WasmType::Pointer | WasmType::Length => value,
            WasmType::I64 | WasmType::PointerOrI64 => value,
            WasmType::F32 => value,
            WasmType::F64 => value,
        }
    }

    fn store(&mut self, wasm: WasmType, ptr: &str, offset: usize, value: &str) {
        let statement = match wasm {
            WasmType::I32 | WasmType::Pointer | WasmType::Length => {
                format!("write_i32_at(&mut caller, &mem, {ptr}, {offset}, {value})?;")
            }
            WasmType::I64 | WasmType::PointerOrI64 => {
                format!("write_i64_at(&mut caller, &mem, {ptr}, {offset}, {value})?;")
            }
            WasmType::F32 => format!("write_f32_at(&mut caller, &mem, {ptr}, {offset}, {value})?;"),
            WasmType::F64 => format!("write_f64_at(&mut caller, &mem, {ptr}, {offset}, {value})?;"),
        };
        self.emit_statement(statement);
    }
}

impl wit_bindgen_core::abi::Bindgen for HostImportBindgen<'_> {
    type Operand = String;

    fn push_block(&mut self) {
        let previous = std::mem::take(&mut self.out);
        self.blocks.push(previous);
    }

    fn finish_block(&mut self, operands: &mut Vec<String>) {
        let previous = self.blocks.pop().expect("block stack");
        let src = std::mem::replace(&mut self.out, previous);
        let expression = match operands.len() {
            0 => "()".to_owned(),
            1 => operands[0].clone(),
            _ => format!("({})", operands.join(", ")),
        };
        if src.is_empty() {
            self.blocks.push(expression);
        } else if operands.is_empty() {
            self.blocks.push(format!("{{\n{src}}}"));
        } else {
            self.blocks.push(format!("{{\n{src}\n{expression}\n}}"));
        }
    }

    fn return_pointer(&mut self, _size: ArchitectureSize, _align: wit_parser::Alignment) -> String {
        let retptr = self
            .arg_names
            .last()
            .cloned()
            .expect("return pointer is the last flat argument");
        self.retptr = Some(retptr.clone());
        retptr
    }

    fn sizes(&self) -> &SizeAlign {
        self.sizes
    }

    fn is_list_canonical(&self, _resolve: &Resolve, _element: &Type) -> bool {
        false
    }

    fn emit(
        &mut self,
        _resolve: &Resolve,
        inst: &Instruction<'_>,
        operands: &mut Vec<String>,
        results: &mut Vec<String>,
    ) {
        let mut pop = || operands.pop().expect("operand");
        match inst {
            Instruction::GetArg { nth } => results.push(self.arg_names[*nth].clone()),

            Instruction::I32Const { val } => results.push(format!("{val}")),

            Instruction::Bitcasts { casts } => {
                let mut casted = Vec::with_capacity(casts.len());
                for (operand, cast) in operands.drain(..).zip(casts.iter()) {
                    casted.push(match cast {
                        Bitcast::None => operand,
                        Bitcast::I32ToI64 => format!("({operand} as i64)"),
                        Bitcast::F32ToI32 => format!("({operand}).to_bits() as i32"),
                        Bitcast::F64ToI64 => format!("({operand}).to_bits() as i64"),
                        Bitcast::I64ToI32 => format!("({operand} as i32)"),
                        Bitcast::I32ToF32 => format!("f32::from_bits({operand} as u32)"),
                        Bitcast::I64ToF64 => format!("f64::from_bits({operand} as u64)"),
                        Bitcast::F32ToI64 => format!("i64::from(({operand}).to_bits())"),
                        Bitcast::I64ToF32 => format!("f32::from_bits(({operand}) as u32)"),
                        Bitcast::I64ToP64
                        | Bitcast::PToP64
                        | Bitcast::P64ToI64
                        | Bitcast::P64ToP => format!("({operand} as i64)"),
                        Bitcast::I32ToP | Bitcast::LToP => format!("({operand} as i32)"),
                        Bitcast::PToI32 | Bitcast::LToI32 => format!("({operand} as i32)"),
                        Bitcast::I32ToL | Bitcast::I64ToL | Bitcast::PToL => {
                            format!("({operand} as i32)")
                        }
                        Bitcast::LToI64 => format!("({operand} as i64)"),
                        Bitcast::Sequence(sequence) => {
                            let [first, _] = &**sequence;
                            let _ = first;
                            "(operand as i64)".to_string()
                        }
                    });
                }
                results.extend(casted);
            }

            Instruction::ConstZero { tys } => {
                for ty in tys.iter() {
                    results.push(match ty {
                        WasmType::I32 | WasmType::Pointer | WasmType::Length => "0".to_owned(),
                        WasmType::I64 | WasmType::PointerOrI64 => "0".to_owned(),
                        WasmType::F32 => "0.0f32".to_owned(),
                        WasmType::F64 => "0.0f64".to_owned(),
                    });
                }
            }

            Instruction::I32Load { offset } => {
                let ptr = pop();
                results.push(self.load(WasmType::I32, &ptr, self.offset32(offset), false));
            }
            Instruction::I32Load8U { offset } => {
                let ptr = pop();
                let value = self.bind(format!(
                    "read_i32_at(&mut caller, &mem, {ptr}, {})? & 0xff",
                    self.offset32(offset)
                ));
                results.push(value);
            }
            Instruction::I32Load8S { offset } => {
                let ptr = pop();
                let value = self.bind(format!(
                    "read_i32_at(&mut caller, &mem, {ptr}, {})? as i8 as i32",
                    self.offset32(offset)
                ));
                results.push(value);
            }
            Instruction::I32Load16U { offset } => {
                let ptr = pop();
                let value = self.bind(format!(
                    "read_i32_at(&mut caller, &mem, {ptr}, {})? & 0xffff",
                    self.offset32(offset)
                ));
                results.push(value);
            }
            Instruction::I32Load16S { offset } => {
                let ptr = pop();
                let value = self.bind(format!(
                    "read_i32_at(&mut caller, &mem, {ptr}, {})? as i16 as i32",
                    self.offset32(offset)
                ));
                results.push(value);
            }
            Instruction::I64Load { offset } => {
                let ptr = pop();
                results.push(self.load(WasmType::I64, &ptr, self.offset32(offset), false));
            }
            Instruction::F32Load { offset } => {
                let ptr = pop();
                results.push(self.load(WasmType::F32, &ptr, self.offset32(offset), false));
            }
            Instruction::F64Load { offset } => {
                let ptr = pop();
                results.push(self.load(WasmType::F64, &ptr, self.offset32(offset), false));
            }
            Instruction::PointerLoad { offset } => {
                let ptr = pop();
                results.push(self.load(WasmType::Pointer, &ptr, self.offset32(offset), false));
            }
            Instruction::LengthLoad { offset } => {
                let ptr = pop();
                results.push(self.load(WasmType::Length, &ptr, self.offset32(offset), false));
            }

            Instruction::I32Store { offset } => {
                let ptr = pop();
                let value = pop();
                self.store(WasmType::I32, &ptr, self.offset32(offset), &value);
            }
            Instruction::I32Store8 { offset } => {
                let ptr = pop();
                let value = pop();
                self.emit_statement(format!(
                    "write_u8_at(&mut caller, &mem, {ptr}, {}, {value})?;",
                    self.offset32(offset)
                ));
            }
            Instruction::I32Store16 { offset } => {
                let ptr = pop();
                let value = pop();
                self.emit_statement(format!(
                    "write_u16_at(&mut caller, &mem, {ptr}, {}, {value})?;",
                    self.offset32(offset)
                ));
            }
            Instruction::I64Store { offset } => {
                let ptr = pop();
                let value = pop();
                self.store(WasmType::I64, &ptr, self.offset32(offset), &value);
            }
            Instruction::F32Store { offset } => {
                let ptr = pop();
                let value = pop();
                self.store(WasmType::F32, &ptr, self.offset32(offset), &value);
            }
            Instruction::F64Store { offset } => {
                let ptr = pop();
                let value = pop();
                self.store(WasmType::F64, &ptr, self.offset32(offset), &value);
            }
            Instruction::PointerStore { offset } => {
                let ptr = pop();
                let value = pop();
                self.store(WasmType::Pointer, &ptr, self.offset32(offset), &value);
            }
            Instruction::LengthStore { offset } => {
                let ptr = pop();
                let value = pop();
                self.store(WasmType::Length, &ptr, self.offset32(offset), &value);
            }

            // Scalar conversions: interface value <-> wasm value.
            Instruction::I32FromChar
            | Instruction::I32FromU32
            | Instruction::I32FromS32
            | Instruction::I32FromU16
            | Instruction::I32FromS16
            | Instruction::I32FromU8
            | Instruction::I32FromS8 => {
                let operand = pop();
                results.push(format!("({operand} as i32)"));
            }
            Instruction::I64FromU64 | Instruction::I64FromS64 => {
                let operand = pop();
                results.push(format!("({operand} as i64)"));
            }
            Instruction::CoreF32FromF32 | Instruction::F32FromCoreF32 => {
                let operand = pop();
                results.push(format!("({operand} as f32)"));
            }
            Instruction::CoreF64FromF64 | Instruction::F64FromCoreF64 => {
                let operand = pop();
                results.push(format!("({operand} as f64)"));
            }
            Instruction::S8FromI32 => {
                let operand = pop();
                results.push(format!("({operand} as i8)"));
            }
            Instruction::U8FromI32 => {
                let operand = pop();
                results.push(format!("({operand} as u8)"));
            }
            Instruction::S16FromI32 => {
                let operand = pop();
                results.push(format!("({operand} as i16)"));
            }
            Instruction::U16FromI32 => {
                let operand = pop();
                results.push(format!("({operand} as u16)"));
            }
            Instruction::S32FromI32 => {
                let operand = pop();
                results.push(format!("({operand} as i32)"));
            }
            Instruction::U32FromI32 => {
                let operand = pop();
                results.push(format!("({operand} as u32)"));
            }
            Instruction::S64FromI64 => {
                let operand = pop();
                results.push(format!("({operand} as i64)"));
            }
            Instruction::U64FromI64 => {
                let operand = pop();
                results.push(format!("({operand} as u64)"));
            }
            Instruction::CharFromI32 => {
                let operand = pop();
                let value = self.bind(format!(
                    "char::from_u32({operand} as u32).ok_or_else(|| trap(\"invalid char\"))?"
                ));
                results.push(value);
            }
            Instruction::BoolFromI32 => {
                let operand = pop();
                results.push(format!("({operand} != 0)"));
            }
            Instruction::I32FromBool => {
                let operand = pop();
                results.push(format!("if {operand} {{ 1 }} else {{ 0 }}"));
            }

            Instruction::StringLift => {
                let len = pop();
                let ptr = pop();
                let value = self.bind(format!("lift_string(&mut caller, &mem, {ptr}, {len})?"));
                results.push(value);
            }

            Instruction::StringLower { realloc } => {
                let value = pop();
                let value = self.bind(value);
                let ptr = self.bind(format!("alloc(&mut caller, &mem, {value}.len(), 1)?"));
                self.emit_statement(format!(
                    "mem.data_mut(caller.as_context_mut()).get_mut({ptr} as usize..).and_then(|slice| slice.get_mut(..{value}.len())).ok_or_else(|| trap(\"allocation out of bounds\"))?.copy_from_slice({value}.as_bytes());"
                ));
                let len = self.bind(format!("{value}.len() as i32"));
                let _ = realloc;
                results.push(ptr);
                results.push(len);
            }

            Instruction::ListCanonLift { element, .. } => {
                let len = pop();
                let ptr = pop();
                let value = self.bind(format!("read_bytes(caller, &mem, {ptr}, {len})?"));
                let _ = element;
                results.push(value);
            }

            Instruction::ListLift { element, .. } => {
                let body = self.blocks.pop().expect("list block");
                let len = pop();
                let ptr = pop();
                let tmp = self.tmp();
                let size = self.size32(element);
                let _ = writeln!(self.out, "let base{tmp} = {ptr};");
                let _ = writeln!(self.out, "let len{tmp} = {len};");
                let _ = writeln!(
                    self.out,
                    "let mut vec{tmp}: Vec<{}> = Vec::with_capacity(len{tmp} as usize);",
                    self.rust_type(element)
                );
                let _ = writeln!(self.out, "for i{tmp} in 0..len{tmp} {{");
                let _ = writeln!(self.out, "let base = base{tmp} + i{tmp} * {size};");
                let _ = writeln!(self.out, "let e{tmp} = {body};");
                let _ = writeln!(self.out, "vec{tmp}.push(e{tmp});");
                let _ = writeln!(self.out, "}}");
                results.push(format!("vec{tmp}"));
            }

            Instruction::ListCanonLower { element, .. } => {
                let value = pop();
                let _ = element;
                let value = self.bind(value);
                let ptr = self.bind(format!("alloc(&mut caller, &mem, {value}.len(), 1)?"));
                self.emit_statement(format!(
                    "mem.data_mut(caller.as_context_mut()).get_mut({ptr} as usize..).and_then(|slice| slice.get_mut(..{value}.len())).ok_or_else(|| trap(\"allocation out of bounds\"))?.copy_from_slice(&{value});"
                ));
                let len = self.bind(format!("{value}.len() as i32"));
                results.push(ptr);
                results.push(len);
            }

            Instruction::ListLower { element, realloc } => {
                let body = self.blocks.pop().expect("list block");
                let value = pop();
                let tmp = self.tmp();
                let size = self.size32(element);
                let align = self.align32(element);
                let _ = writeln!(self.out, "let vec{tmp} = {value};");
                let _ = writeln!(self.out, "let len{tmp} = vec{tmp}.len();");
                let ptr = self.bind(format!(
                    "alloc(&mut caller, &mem, len{tmp}.checked_mul({size}).ok_or_else(|| trap(\"list size overflow\"))?, {align})?"
                ));
                let _ = writeln!(
                    self.out,
                    "for (i{tmp}, e) in vec{tmp}.into_iter().enumerate() {{"
                );
                let _ = writeln!(self.out, "let base = {ptr} + (i{tmp} * {size}) as i32;");
                let _ = writeln!(self.out, "{body}");
                let _ = writeln!(self.out, "}}");
                let len = self.bind(format!("len{tmp} as i32"));
                let _ = realloc;
                results.push(ptr);
                results.push(len);
            }

            Instruction::RecordLift { record, ty, .. } => {
                let field_count = record.fields.len();
                let values = operands.split_off(operands.len() - field_count);
                let struct_name = self.type_name(*ty);
                let fields = record
                    .fields
                    .iter()
                    .zip(values)
                    .map(|(field, value)| format!("{}: {value}", Self::sanitize_field(&field.name)))
                    .collect::<Vec<_>>()
                    .join(", ");
                let value = self.bind(format!("{struct_name} {{ {fields} }}"));
                results.push(value);
            }

            Instruction::RecordLower {
                record, name, ty, ..
            } => {
                let value = pop();
                let _ = name;
                let _ = ty;
                for field in &record.fields {
                    results.push(format!("({value}).{}", Self::sanitize_field(&field.name)));
                }
            }

            Instruction::HandleLift { .. } => {
                let operand = pop();
                results.push(format!("({operand} as u32)"));
            }
            Instruction::HandleLower { .. } => {
                let operand = pop();
                results.push(format!("({operand} as i32)"));
            }

            Instruction::VariantPayloadName => results.push("e".to_owned()),

            Instruction::IterElem { .. } => results.push("e".to_owned()),

            Instruction::IterBasePointer => results.push("base".to_owned()),

            Instruction::VariantLift { variant, ty, .. } => {
                let case_count = variant.cases.len();
                let blocks = self.blocks.split_off(self.blocks.len() - case_count);
                let tag = pop();
                let tmp = self.tmp();
                let enum_name = self.type_name(*ty);
                let _ = writeln!(self.out, "let v{tmp} = match {tag} {{");
                for (index, (case, block)) in variant.cases.iter().zip(blocks).enumerate() {
                    let case_name = case
                        .name
                        .split(['-', '_'])
                        .map(|part| {
                            let mut chars = part.chars();
                            match chars.next() {
                                Some(first) => {
                                    first.to_uppercase().collect::<String>() + chars.as_str()
                                }
                                None => String::new(),
                            }
                        })
                        .collect::<String>();
                    if case.ty.is_some() {
                        let _ = writeln!(self.out, "{index} => {enum_name}::{case_name}({block}),");
                    } else {
                        let _ = writeln!(self.out, "{index} => {enum_name}::{case_name},");
                    }
                }
                let _ = writeln!(
                    self.out,
                    "_ => return Err(trap(\"invalid variant discriminant\")),"
                );
                let _ = writeln!(self.out, "}};");
                results.push(format!("v{tmp}"));
            }

            Instruction::VariantLower {
                variant,
                results: result_types,
                ty,
                ..
            } => {
                let case_count = variant.cases.len();
                let blocks = self.blocks.split_off(self.blocks.len() - case_count);
                let value = pop();
                let tmp = self.tmp();
                let enum_name = self.type_name(*ty);
                if result_types.is_empty() {
                    let _ = writeln!(self.out, "match {value} {{");
                } else {
                    let _ = writeln!(self.out, "let v{tmp} = match {value} {{");
                }
                for (case, block) in variant.cases.iter().zip(blocks) {
                    let case_name = case
                        .name
                        .split(['-', '_'])
                        .map(|part| {
                            let mut chars = part.chars();
                            match chars.next() {
                                Some(first) => {
                                    first.to_uppercase().collect::<String>() + chars.as_str()
                                }
                                None => String::new(),
                            }
                        })
                        .collect::<String>();
                    if case.ty.is_some() {
                        let _ = writeln!(self.out, "{enum_name}::{case_name}(e) => {{ {block} }},");
                    } else {
                        let _ = writeln!(self.out, "{enum_name}::{case_name} => {{ {block} }},");
                    }
                }
                let _ = writeln!(self.out, "}};");
                if !result_types.is_empty() {
                    results.push(format!("v{tmp}"));
                }
            }

            Instruction::OptionLift { payload, .. } => {
                let blocks = self.blocks.split_off(self.blocks.len() - 2);
                let tag = pop();
                let tmp = self.tmp();
                let has_payload = self.size32(payload) > 0;
                let _ = writeln!(self.out, "let v{tmp} = if {tag} != 0 {{");
                if has_payload {
                    let _ = writeln!(self.out, "Some({})", blocks[1]);
                } else {
                    let _ = writeln!(self.out, "Some(())");
                }
                let _ = writeln!(self.out, "}} else {{ None }};");
                results.push(format!("v{tmp}"));
            }

            Instruction::OptionLower {
                payload,
                results: result_types,
                ..
            } => {
                let blocks = self.blocks.split_off(self.blocks.len() - 2);
                let value = pop();
                let tmp = self.tmp();
                let has_payload = self.size32(payload) > 0;
                if result_types.is_empty() {
                    let _ = writeln!(self.out, "match {value} {{");
                } else {
                    let _ = writeln!(self.out, "let v{tmp} = match {value} {{");
                }
                if has_payload {
                    let _ = writeln!(self.out, "Some(e) => {{ {} }}", blocks[1]);
                } else {
                    let _ = writeln!(self.out, "Some(()) => {{ {} }}", blocks[1]);
                }
                let _ = writeln!(self.out, "None => {{ {} }}", blocks[0]);
                let _ = writeln!(self.out, "}};");
                if !result_types.is_empty() {
                    results.push(format!("v{tmp}"));
                }
            }

            Instruction::ResultLift { result, .. } => {
                let blocks = self.blocks.split_off(self.blocks.len() - 2);
                let tag = pop();
                let tmp = self.tmp();
                let _ = writeln!(self.out, "let v{tmp} = if {tag} == 0 {{");
                if result.ok.as_ref().is_some_and(|ty| self.size32(ty) > 0) {
                    let _ = writeln!(self.out, "Ok({})", blocks[0]);
                } else {
                    let _ = writeln!(self.out, "Ok(())");
                }
                let _ = writeln!(self.out, "}} else {{");
                if result.err.as_ref().is_some_and(|ty| self.size32(ty) > 0) {
                    let _ = writeln!(self.out, "Err({})", blocks[1]);
                } else {
                    let _ = writeln!(self.out, "Err(())");
                }
                let _ = writeln!(self.out, "}};");
                results.push(format!("v{tmp}"));
            }

            Instruction::ResultLower {
                result,
                results: result_types,
                ..
            } => {
                let blocks = self.blocks.split_off(self.blocks.len() - 2);
                let value = pop();
                let tmp = self.tmp();
                if result_types.is_empty() {
                    let _ = writeln!(self.out, "match {value} {{");
                } else {
                    let _ = writeln!(self.out, "let v{tmp} = match {value} {{");
                }
                if result.ok.as_ref().is_some_and(|ty| self.size32(ty) > 0) {
                    let _ = writeln!(self.out, "Ok(e) => {{ {} }}", blocks[0]);
                } else {
                    let _ = writeln!(self.out, "Ok(()) => {{ {} }}", blocks[0]);
                }
                if result.err.as_ref().is_some_and(|ty| self.size32(ty) > 0) {
                    let _ = writeln!(self.out, "Err(e) => {{ {} }}", blocks[1]);
                } else {
                    let _ = writeln!(self.out, "Err(()) => {{ {} }}", blocks[1]);
                }
                let _ = writeln!(self.out, "}};");
                if !result_types.is_empty() {
                    results.push(format!("v{tmp}"));
                }
            }

            Instruction::EnumLift { enum_, ty, .. } => {
                let tag = pop();
                let tmp = self.tmp();
                let enum_name = self.type_name(*ty);
                let _ = writeln!(self.out, "let v{tmp} = match {tag} {{");
                for (index, case) in enum_.cases.iter().enumerate() {
                    let case_name = case
                        .name
                        .split(['-', '_'])
                        .map(|part| {
                            let mut chars = part.chars();
                            match chars.next() {
                                Some(first) => {
                                    first.to_uppercase().collect::<String>() + chars.as_str()
                                }
                                None => String::new(),
                            }
                        })
                        .collect::<String>();
                    let _ = writeln!(self.out, "{index} => {enum_name}::{case_name},");
                }
                let _ = writeln!(
                    self.out,
                    "_ => return Err(trap(\"invalid enum discriminant\")),"
                );
                let _ = writeln!(self.out, "}};");
                results.push(format!("v{tmp}"));
            }

            Instruction::EnumLower { .. } => {
                let operand = pop();
                results.push(format!("({operand} as i32)"));
            }

            Instruction::CallWasm { .. } => {
                let args = std::mem::take(operands);
                let call = format!(
                    "crate::reentry::with_guard(&mut caller, |caller| <T as {}>::{}(caller.as_context_mut().data_mut(), {}))",
                    self.trait_name,
                    self.function_name,
                    args.join(", ")
                );
                let mapped = format!(
                    "{call}.map_err(|error| trap(format!(\"{{}}::{{}} failed: {{error}}\", {:?}, {:?})))",
                    self.module_name, self.function_name
                );
                if self.function.result.is_some() {
                    let value = self.bind(format!("{mapped}?"));
                    results.push(value);
                } else {
                    self.emit_statement(format!("{mapped}?;"));
                }
            }

            Instruction::CallInterface { func, .. } => {
                let args = std::mem::take(operands);
                let call = format!(
                    "crate::reentry::with_guard(&mut caller, |caller| <T as {}>::{}(caller.as_context_mut().data_mut(), {}))",
                    self.trait_name,
                    self.function_name,
                    args.join(", ")
                );
                let mapped = format!(
                    "{call}.map_err(|error| trap(format!(\"{{}}::{{}} failed: {{error}}\", {:?}, {:?})))",
                    self.module_name, self.function_name
                );
                if func.result.is_some() {
                    let value = self.bind(format!("{mapped}?"));
                    results.push(value);
                } else {
                    self.emit_statement(format!("{mapped}?;"));
                }
            }

            Instruction::Return { amt, .. } => {
                if let Some(retptr) = &self.retptr {
                    let _ = retptr;
                    let _ = writeln!(self.out, "return Ok(());");
                } else if *amt > 0 {
                    let values = operands.split_off(operands.len() - amt);
                    let _ = writeln!(self.out, "return Ok({});", values.join(", "));
                } else {
                    let _ = writeln!(self.out, "return Ok(());");
                }
            }

            Instruction::Flush { amt } => {
                let values = operands.split_off(operands.len() - amt);
                for value in values {
                    results.push(self.bind(value));
                }
            }

            other => {
                let _ = writeln!(
                    self.out,
                    "return Err(trap(\"unsupported instruction: {other:?}\"));"
                );
            }
        }
    }
}

impl<'a> HostImportBindgen<'a> {}
