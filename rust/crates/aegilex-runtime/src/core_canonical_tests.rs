//! Compile-time verification that the canonical-world Core bindings generated
//! by `aegilex-core-bindgen` are valid Rust. The full canonical world defines
//! 784 imported functions; this module only includes the generated module so
//! type definitions, host traits, and `add_imports` generic signatures compile.

/// The generated module defines `core_host::imports::{Host<Interface>,
/// add_imports}` for every imported interface and `crate::core_host::CoreExports`
/// for the world's exports. Including it here forces the full canonical world
/// through type checking.
#[test]
fn canonical_bindings_compile() {
    let _ = std::mem::size_of::<crate::core_host::CoreExports>();
}
