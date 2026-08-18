//! End-to-end Core loader verification: instantiate a real canonical-world
//! guest (`examples/hello-component`, built for `wasm32-wasip1`) as a Core
//! `Module` with the generated canonical bindings and a default-denied host,
//! then call its lifecycle exports through the generated `CoreExports`.

use crate::core_host::imports::DeniedHost;
use wasmtime::{AsContext, Config, Engine, Instance, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::preview1::{self};

fn hello_wasm_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../../examples/hello-component/target/wasm32-wasip1/release/aegilex_hello_component.wasm",
    )
}

#[test]
fn core_loader_loads_hello_guest_and_runs_on_load() {
    let engine = Engine::new(&Config::new()).expect("engine");
    let module = Module::from_file(&engine, hello_wasm_path())
        .expect("hello component must be built for wasm32-wasip1");

    let mut linker = Linker::new(&engine);
    preview1::add_to_linker_sync(&mut linker, |host: &mut DeniedHost| &mut host.wasi)
        .expect("link WASI preview1");
    crate::core_host::imports::add_all_imports(&mut linker).expect("link generated imports");

    let mut store = Store::new(
        &engine,
        DeniedHost {
            wasi: WasiCtxBuilder::new().build_p1(),
        },
    );
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate Core hello guest");

    let exports = crate::core_host::CoreExports::new(&instance, &mut store).expect("core exports");

    // `on-load: func() -> result<_, string>`; a success writes discriminant 0
    // into the return area.
    let on_load = exports
        .on_load
        .as_ref()
        .expect("on-load export present in hello guest");
    let retptr = on_load.call(&mut store, ()).expect("on-load call");

    let mem = instance
        .get_export(&mut store, "memory")
        .and_then(|export| export.into_memory())
        .expect("memory export");
    let tag = u32::from_le_bytes(
        mem.data(store.as_context())
            .get(retptr as usize..)
            .and_then(|slice| slice.get(..4))
            .expect("return area")
            .try_into()
            .expect("4 bytes"),
    );
    assert_eq!(tag, 0, "on-load must return Ok(())");

    exports
        .on_load_post
        .as_ref()
        .expect("cabi_post_on-load")
        .call(&mut store, retptr)
        .expect("on-load post-return");
}

#[test]
fn core_loader_reads_hello_metadata() {
    let engine = Engine::new(&Config::new()).expect("engine");
    let module = Module::from_file(&engine, hello_wasm_path())
        .expect("hello component must be built for wasm32-wasip1");

    let mut linker = Linker::new(&engine);
    preview1::add_to_linker_sync(&mut linker, |host: &mut DeniedHost| &mut host.wasi)
        .expect("link WASI preview1");
    crate::core_host::imports::add_all_imports(&mut linker).expect("link generated imports");

    let mut store = Store::new(
        &engine,
        DeniedHost {
            wasi: WasiCtxBuilder::new().build_p1(),
        },
    );
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate Core hello guest");
    let exports = crate::core_host::CoreExports::new(&instance, &mut store).expect("core exports");

    let metadata = exports
        .call_metadata(&instance, &mut store)
        .expect("metadata export must lift");
    assert_eq!(metadata.name, "example_hello");
    assert_eq!(metadata.version, "0.1.0");
    assert_eq!(metadata.prefix, "Hello");
    assert_eq!(metadata.subscriptions, vec!["player-join", "player-kick"]);
    assert_eq!(metadata.commands.len(), 1);
    assert_eq!(metadata.commands[0].name, "hello");
    assert_eq!(metadata.commands[0].aliases, vec!["hi"]);
    assert_eq!(metadata.permissions.len(), 1);
    assert_eq!(metadata.permissions[0].name, "aegilex.example.hello");
}

#[test]
fn core_loader_lifecycle_callbacks_propagate_guest_results() {
    let engine = Engine::new(&Config::new()).expect("engine");
    let module = Module::from_file(&engine, hello_wasm_path())
        .expect("hello component must be built for wasm32-wasip1");

    let mut linker = Linker::new(&engine);
    preview1::add_to_linker_sync(&mut linker, |host: &mut DeniedHost| &mut host.wasi)
        .expect("link WASI preview1");
    crate::core_host::imports::add_all_imports(&mut linker).expect("link generated imports");

    let mut store = Store::new(
        &engine,
        DeniedHost {
            wasi: WasiCtxBuilder::new().build_p1(),
        },
    );
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate Core hello guest");
    let exports = crate::core_host::CoreExports::new(&instance, &mut store).expect("core exports");

    // hello's on-load does not call the host: success.
    exports
        .call_on_load(&instance, &mut store)
        .expect("on-load export call must succeed")
        .expect("on-load guest result must succeed");

    // hello's on-enable calls the logger, which the default-denied host
    // rejects; the guest error must propagate out of the export call.
    let enable = exports.call_on_enable(&instance, &mut store);
    // The export call itself succeeds; the guest's on-enable returns Err
    // because the logger import was denied.
    assert!(
        matches!(&enable, Ok(Err(message)) if message.contains("denied")),
        "on-enable must surface the rejected host call, got {enable:?}"
    );

    // hello's on-disable ignores the failed log call and returns success.
    exports
        .call_on_disable(&mut store)
        .expect("on-disable must succeed");
}

fn instantiate_guest(
    engine: &Engine,
    path: std::path::PathBuf,
) -> (Store<DeniedHost>, Instance, crate::core_host::CoreExports) {
    let module = Module::from_file(engine, path).expect("guest wasm");
    let mut linker = Linker::new(engine);
    preview1::add_to_linker_sync(&mut linker, |host: &mut DeniedHost| &mut host.wasi)
        .expect("link WASI preview1");
    crate::core_host::imports::add_all_imports(&mut linker).expect("link generated imports");
    let mut store = Store::new(
        engine,
        DeniedHost {
            wasi: WasiCtxBuilder::new().build_p1(),
        },
    );
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate Core guest");
    let exports = crate::core_host::CoreExports::new(&instance, &mut store).expect("core exports");
    (store, instance, exports)
}

#[test]
fn core_loader_instantiates_largest_canonical_guest() {
    let engine = Engine::new(&Config::new()).expect("engine");
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../../examples/full-test-component/target/wasm32-wasip1/release/aegilex_full_test_component.wasm",
    );
    let (mut store, instance, exports) = instantiate_guest(&engine, path);

    let metadata = exports
        .call_metadata(&instance, &mut store)
        .expect("full-test metadata must lift");
    assert_eq!(metadata.name, "full_test");
    assert_eq!(metadata.version, "0.1.0");
    assert_eq!(metadata.subscriptions.len(), 54);
    assert_eq!(metadata.commands.len(), 1);
    assert_eq!(metadata.permissions.len(), 1);
}

#[test]
fn production_store_enables_hello_guest_with_real_logger() {
    use crate::config::RuntimeConfig;
    use crate::host::runtime::handles::GuestHandles;
    use crate::host::runtime::native::HostContext;
    use crate::manifest::PluginPolicy;
    use crate::runtime::PluginStoreState;
    use wasmtime_wasi::WasiCtxBuilder;

    let engine = Engine::new(&Config::new()).expect("engine");
    let module = Module::from_file(&engine, hello_wasm_path()).expect("hello wasm");
    let mut linker = Linker::new(&engine);
    preview1::add_to_linker_sync(&mut linker, |state: &mut PluginStoreState| &mut state.wasi)
        .expect("link WASI preview1");
    crate::core_host::imports::add_all_imports(&mut linker).expect("link generated imports");

    let mut store = Store::new(
        &engine,
        PluginStoreState {
            host: HostContext::new(crate::cxx_host::ffi::HostContext::test_stub())
                .expect("stub host"),
            handles: GuestHandles::new(),
            plugin_id: "example_hello".to_owned(),
            invocation_id: 0,
            invocation_frames: Vec::new(),
            subscriptions: Vec::new(),
            commands: Vec::new(),
            instance: None,
            policy: PluginPolicy::default(),
            config: RuntimeConfig::default(),
            wasi: WasiCtxBuilder::new().build_p1(),
            resources: crate::core_resources::CoreResourceTable::new(),
            resource_slot_count: 0,
            forms: std::collections::HashMap::new(),
            service_providers: std::collections::HashMap::new(),
            service_calls: std::collections::HashMap::new(),
            map_renderers: std::collections::HashMap::new(),
            host_borrowed_slots: Vec::new(),
            plugin_owned_slots: Vec::new(),
        },
    );
    store.data_mut().policy.capabilities = vec!["logger.*".to_owned()];

    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate Core hello guest");
    let exports = crate::core_host::CoreExports::new(&instance, &mut store).expect("core exports");

    store.data_mut().invocation_id = 42;
    exports
        .call_on_load(&instance, &mut store)
        .expect("on-load export call must succeed")
        .expect("on-load guest result must succeed");

    // hello's on-enable calls the logger import; the real generated-trait
    // implementation must service it (not the default-denied stub).
    let enable = exports.call_on_enable(&instance, &mut store);
    assert!(
        matches!(&enable, Ok(Ok(()))),
        "on-enable must succeed through the real logger host, got {enable:?}"
    );
}

#[test]
fn production_store_runs_full_test_guest_with_all_capabilities() {
    use crate::config::RuntimeConfig;
    use crate::host::runtime::handles::GuestHandles;
    use crate::host::runtime::native::HostContext;
    use crate::manifest::PluginPolicy;
    use crate::runtime::PluginStoreState;
    use wasmtime_wasi::WasiCtxBuilder;

    let engine = Engine::new(&Config::new()).expect("engine");
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../../examples/full-test-component/target/wasm32-wasip1/release/aegilex_full_test_component.wasm",
    );
    let module = Module::from_file(&engine, path).expect("full-test wasm");
    let mut linker = Linker::new(&engine);
    preview1::add_to_linker_sync(&mut linker, |state: &mut PluginStoreState| &mut state.wasi)
        .expect("link WASI preview1");
    crate::core_host::imports::add_all_imports(&mut linker).expect("link generated imports");

    let mut store = Store::new(
        &engine,
        PluginStoreState {
            host: HostContext::new(crate::cxx_host::ffi::HostContext::test_stub())
                .expect("stub host"),
            handles: GuestHandles::new(),
            plugin_id: "full_test".to_owned(),
            invocation_id: 0,
            invocation_frames: Vec::new(),
            subscriptions: Vec::new(),
            commands: Vec::new(),
            instance: None,
            policy: PluginPolicy::default(),
            config: RuntimeConfig::default(),
            wasi: WasiCtxBuilder::new().build_p1(),
            resources: crate::core_resources::CoreResourceTable::new(),
            resource_slot_count: 0,
            forms: std::collections::HashMap::new(),
            service_providers: std::collections::HashMap::new(),
            service_calls: std::collections::HashMap::new(),
            map_renderers: std::collections::HashMap::new(),
            host_borrowed_slots: Vec::new(),
            plugin_owned_slots: Vec::new(),
        },
    );
    store.data_mut().policy.capabilities = vec!["*".to_owned()];
    store.data_mut().invocation_id = 42;

    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate Core full-test guest");
    let exports = crate::core_host::CoreExports::new(&instance, &mut store).expect("core exports");

    exports
        .call_on_load(&instance, &mut store)
        .expect("full-test on-load export call must succeed")
        .expect("full-test on-load guest result must succeed");
    let enable = exports.call_on_enable(&instance, &mut store);
    assert!(
        matches!(&enable, Ok(Ok(()))),
        "full-test on-enable must succeed through migrated host traits, got {enable:?}"
    );
}
