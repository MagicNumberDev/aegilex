use super::*;
use wasmtime::{Instance, Linker as CoreLinker, Module, Store as CoreStore};
use wasmtime_wasi::preview1::{self};

/// Builds the linker for a Core Wasm plugin module: Preview1 WASI plus every
/// generated canonical-ABI host import for the canonical world.
fn build_core_linker(engine: &Engine) -> Result<CoreLinker<PluginStoreState>, String> {
    let mut linker = CoreLinker::new(engine);
    preview1::add_to_linker_sync(&mut linker, |state: &mut PluginStoreState| &mut state.wasi)
        .map_err(|error| format!("cannot configure WASI preview1 imports: {error}"))?;
    crate::core_host::imports::add_all_imports(&mut linker)
        .map_err(|error| format!("cannot configure generated host imports: {error}"))?;
    Ok(linker)
}

/// Loads a Core module, instantiates it with a deny-most store, and resolves
/// its generated exports. Used for discovery and for the live plugin store.
fn instantiate_core(
    engine: &Engine,
    module_path: &Path,
    linker: &CoreLinker<PluginStoreState>,
    store: &mut CoreStore<PluginStoreState>,
) -> Result<(Instance, crate::core_host::CoreExports), String> {
    let module = Module::from_file(engine, module_path)
        .map_err(|error| format!("{}: invalid Core module: {error}", module_path.display()))?;
    let instance = linker
        .instantiate(&mut *store, &module)
        .map_err(|error| format!("cannot instantiate: {error}"))?;
    let exports = crate::core_host::CoreExports::new(&instance, &mut *store)
        .map_err(|error| format!("cannot resolve guest exports: {error}"))?;
    Ok((instance, exports))
}

impl Runtime {
    pub(super) fn load_metadata(&self, module_path: &Path) -> Result<PluginMetadata, String> {
        let module_path = manifest::inspect_module_path(module_path, self.config)?;
        // Validate policy at discovery time without allowing metadata to invoke host APIs.
        let _policy = manifest::load_plugin_policy(&module_path)?;
        let linker = build_core_linker(&self.engine)?;
        let mut store = self.new_store("aegilex".to_owned(), PluginPolicy::default())?;
        store
            .set_fuel(ENABLE_FUEL)
            .map_err(|error| format!("cannot configure fuel: {error}"))?;
        let (instance, exports) =
            instantiate_core(&self.engine, &module_path, &linker, &mut store)?;
        let metadata = exports
            .call_metadata(&instance, &mut store)
            .map_err(|error| format!("metadata trapped: {error}"))?;
        manifest::validate_metadata(&module_path, metadata)
    }

    pub(super) fn prepare(&mut self, module_path: &Path) -> Result<(), String> {
        let module_path = manifest::inspect_module_path(module_path, self.config)?;
        let policy = manifest::load_plugin_policy(&module_path)?;
        let linker = build_core_linker(&self.engine)?;
        let metadata = {
            // Metadata is discovery data, so resolve it in a deny-all store.
            let mut store = self.new_store("aegilex".to_owned(), PluginPolicy::default())?;
            store
                .set_fuel(ENABLE_FUEL)
                .map_err(|error| format!("cannot configure fuel: {error}"))?;
            let (instance, exports) =
                instantiate_core(&self.engine, &module_path, &linker, &mut store)?;
            let metadata = exports
                .call_metadata(&instance, &mut store)
                .map_err(|error| format!("metadata trapped: {error}"))?;
            manifest::validate_metadata(&module_path, metadata)?
        };
        if self.plugins.iter().any(|plugin| plugin.id == metadata.name) {
            return Ok(());
        }

        let mut store = self.new_store(metadata.name.clone(), policy)?;
        store
            .set_fuel(ENABLE_FUEL)
            .map_err(|error| format!("cannot configure fuel: {error}"))?;
        let (instance, exports) =
            instantiate_core(&self.engine, &module_path, &linker, &mut store)?;
        let invocation_id = self.host.next_invocation_id();
        match call_with_invocation(&mut store, invocation_id, |store| {
            exports.call_on_load(&instance, store)
        }) {
            Ok(Ok(())) => {}
            Ok(Err(error)) => return Err(format!("{}: on-load rejected: {error}", metadata.name)),
            Err(error) => return Err(format!("{}: on-load trapped: {error}", metadata.name)),
        }

        store.data_mut().subscriptions = metadata.subscriptions.clone();
        store.data_mut().commands = metadata.commands.clone();
        store.data_mut().instance = Some(instance);
        self.plugins.push(LoadedPlugin {
            id: metadata.name.clone(),
            metadata: metadata.clone(),
            store,
            instance,
            exports,
            enabled: false,
            subscriptions: metadata.subscriptions.clone(),
            commands: metadata.commands.clone(),
        });
        Ok(())
    }

    fn new_store(
        &self,
        plugin_id: String,
        policy: PluginPolicy,
    ) -> Result<Store<PluginStoreState>, String> {
        let wasi = build_wasi(&policy)?;
        Ok(Store::new(
            &self.engine,
            PluginStoreState {
                host: self.host.clone(),
                handles: GuestHandles::new(),
                plugin_id,
                invocation_id: 0,
                invocation_frames: Vec::new(),
                subscriptions: Vec::new(),
                commands: Vec::new(),
                instance: None,
                policy,
                config: self.config,
                wasi,
                resources: CoreResourceTable::new(),
                resource_slot_count: 0,
                forms: std::collections::HashMap::new(),
                service_providers: std::collections::HashMap::new(),
                service_calls: std::collections::HashMap::new(),
                map_renderers: std::collections::HashMap::new(),
                host_borrowed_slots: Vec::new(),
                plugin_owned_slots: Vec::new(),
            },
        ))
    }
}
