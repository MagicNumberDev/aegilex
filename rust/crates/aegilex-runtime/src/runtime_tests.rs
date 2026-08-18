use super::*;

mod cases {
    use super::*;

    unsafe extern "C" {
        fn aegilex_test_make_server_list_ping_event_facade()
        -> *mut cxx_event::ServerListPingEventFacade;
        fn aegilex_test_make_player_drop_item_event_facade()
        -> *mut cxx_event::PlayerDropItemEventFacade;
        fn aegilex_test_make_block_break_event_facade() -> *mut cxx_event::BlockBreakEventFacade;
        fn aegilex_test_make_block_cook_event_facade() -> *mut cxx_event::BlockCookEventFacade;
        fn aegilex_test_make_leaves_decay_event_facade() -> *mut cxx_event::LeavesDecayEventFacade;
        fn aegilex_test_make_block_from_to_event_facade() -> *mut cxx_event::BlockFromToEventFacade;
        fn aegilex_test_make_block_grow_event_facade() -> *mut cxx_event::BlockGrowEventFacade;
        fn aegilex_test_make_block_piston_event_facade() -> *mut cxx_event::BlockPistonEventFacade;
        fn aegilex_test_make_block_place_event_facade() -> *mut cxx_event::BlockPlaceEventFacade;
        fn aegilex_test_make_player_interact_event_facade()
        -> *mut cxx_event::PlayerInteractEventFacade;
        fn aegilex_test_make_player_interact_actor_event_facade()
        -> *mut cxx_event::PlayerInteractActorEventFacade;
        fn aegilex_test_make_actor_damage_event_facade() -> *mut cxx_event::ActorDamageEventFacade;
        fn aegilex_test_make_actor_explode_event_facade() -> *mut cxx_event::ActorExplodeEventFacade;
        fn aegilex_test_make_block_explode_event_facade() -> *mut cxx_event::BlockExplodeEventFacade;
        fn aegilex_test_make_actor_knockback_event_facade(
            has_source: bool,
        ) -> *mut cxx_event::ActorKnockbackEventFacade;
        fn aegilex_test_make_actor_death_event_facade() -> *mut cxx_event::ActorDeathEventFacade;
        fn aegilex_test_make_actor_remove_event_facade() -> *mut cxx_event::ActorRemoveEventFacade;
        fn aegilex_test_make_actor_spawn_event_facade() -> *mut cxx_event::ActorSpawnEventFacade;
        fn aegilex_test_make_actor_teleport_event_facade()
        -> *mut cxx_event::ActorTeleportEventFacade;
        fn aegilex_test_make_player_death_event_facade() -> *mut cxx_event::PlayerDeathEventFacade;
        fn aegilex_test_make_player_join_event_facade() -> *mut cxx_event::PlayerJoinEventFacade;
        fn aegilex_test_make_player_quit_event_facade() -> *mut cxx_event::PlayerQuitEventFacade;
        fn aegilex_test_make_player_item_consume_event_facade()
        -> *mut cxx_event::PlayerItemConsumeEventFacade;
        fn aegilex_test_make_player_game_mode_change_event_facade()
        -> *mut cxx_event::PlayerGameModeChangeEventFacade;
        fn aegilex_test_make_player_emote_event_facade() -> *mut cxx_event::PlayerEmoteEventFacade;
        fn aegilex_test_make_player_skin_change_event_facade()
        -> *mut cxx_event::PlayerSkinChangeEventFacade;
        fn aegilex_test_make_player_dimension_change_event_facade()
        -> *mut cxx_event::PlayerDimensionChangeEventFacade;
        fn aegilex_test_make_player_bed_enter_event_facade()
        -> *mut cxx_event::PlayerBedEnterEventFacade;
        fn aegilex_test_make_player_bed_leave_event_facade()
        -> *mut cxx_event::PlayerBedLeaveEventFacade;
        fn aegilex_test_make_player_respawn_event_facade()
        -> *mut cxx_event::PlayerRespawnEventFacade;
        fn aegilex_test_make_player_item_held_event_facade()
        -> *mut cxx_event::PlayerItemHeldEventFacade;
        fn aegilex_test_make_player_pickup_item_event_facade()
        -> *mut cxx_event::PlayerPickupItemEventFacade;
        fn aegilex_test_make_player_move_event_facade() -> *mut cxx_event::PlayerMoveEventFacade;
        fn aegilex_test_make_weather_change_event_facade()
        -> *mut cxx_event::WeatherChangeEventFacade;
        fn aegilex_test_make_thunder_change_event_facade()
        -> *mut cxx_event::ThunderChangeEventFacade;
        fn aegilex_test_make_plugin_lifecycle_event_facade()
        -> *mut cxx_event::PluginLifecycleEventFacade;
        fn aegilex_test_make_server_load_event_facade() -> *mut cxx_event::ServerLoadEventFacade;
        fn aegilex_test_make_chunk_event_facade() -> *mut cxx_event::ChunkEventFacade;
    }

    fn test_server_list_ping_event_facade() -> cxx::UniquePtr<cxx_event::ServerListPingEventFacade>
    {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_server_list_ping_event_facade()) }
    }

    fn test_player_pickup_item_event_facade()
    -> cxx::UniquePtr<cxx_event::PlayerPickupItemEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_player_pickup_item_event_facade()) }
    }

    fn test_actor_teleport_event_facade() -> cxx::UniquePtr<cxx_event::ActorTeleportEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_actor_teleport_event_facade()) }
    }

    fn test_player_drop_item_event_facade() -> cxx::UniquePtr<cxx_event::PlayerDropItemEventFacade>
    {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_player_drop_item_event_facade()) }
    }

    fn test_block_break_event_facade() -> cxx::UniquePtr<cxx_event::BlockBreakEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_block_break_event_facade()) }
    }

    fn test_block_cook_event_facade() -> cxx::UniquePtr<cxx_event::BlockCookEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_block_cook_event_facade()) }
    }

    fn test_leaves_decay_event_facade() -> cxx::UniquePtr<cxx_event::LeavesDecayEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_leaves_decay_event_facade()) }
    }

    fn test_block_from_to_event_facade() -> cxx::UniquePtr<cxx_event::BlockFromToEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_block_from_to_event_facade()) }
    }

    fn test_block_grow_event_facade() -> cxx::UniquePtr<cxx_event::BlockGrowEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_block_grow_event_facade()) }
    }

    fn test_block_piston_event_facade() -> cxx::UniquePtr<cxx_event::BlockPistonEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_block_piston_event_facade()) }
    }

    fn test_block_place_event_facade() -> cxx::UniquePtr<cxx_event::BlockPlaceEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_block_place_event_facade()) }
    }

    fn test_player_move_event_facade() -> cxx::UniquePtr<cxx_event::PlayerMoveEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_player_move_event_facade()) }
    }

    fn test_player_interact_event_facade() -> cxx::UniquePtr<cxx_event::PlayerInteractEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_player_interact_event_facade()) }
    }

    fn test_player_interact_actor_event_facade()
    -> cxx::UniquePtr<cxx_event::PlayerInteractActorEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_player_interact_actor_event_facade()) }
    }

    fn test_actor_damage_event_facade() -> cxx::UniquePtr<cxx_event::ActorDamageEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_actor_damage_event_facade()) }
    }

    fn test_actor_explode_event_facade() -> cxx::UniquePtr<cxx_event::ActorExplodeEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_actor_explode_event_facade()) }
    }

    fn test_block_explode_event_facade() -> cxx::UniquePtr<cxx_event::BlockExplodeEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_block_explode_event_facade()) }
    }

    fn test_actor_knockback_event_facade(
        has_source: bool,
    ) -> cxx::UniquePtr<cxx_event::ActorKnockbackEventFacade> {
        unsafe {
            cxx::UniquePtr::from_raw(aegilex_test_make_actor_knockback_event_facade(has_source))
        }
    }

    fn test_actor_death_event_facade() -> cxx::UniquePtr<cxx_event::ActorDeathEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_actor_death_event_facade()) }
    }

    fn test_plugin_lifecycle_event_facade() -> cxx::UniquePtr<cxx_event::PluginLifecycleEventFacade>
    {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_plugin_lifecycle_event_facade()) }
    }

    fn test_server_load_event_facade() -> cxx::UniquePtr<cxx_event::ServerLoadEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_server_load_event_facade()) }
    }

    fn test_chunk_event_facade() -> cxx::UniquePtr<cxx_event::ChunkEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_chunk_event_facade()) }
    }

    fn test_actor_remove_event_facade() -> cxx::UniquePtr<cxx_event::ActorRemoveEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_actor_remove_event_facade()) }
    }

    fn test_actor_spawn_event_facade() -> cxx::UniquePtr<cxx_event::ActorSpawnEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_actor_spawn_event_facade()) }
    }

    fn test_player_death_event_facade() -> cxx::UniquePtr<cxx_event::PlayerDeathEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_player_death_event_facade()) }
    }

    fn test_player_join_event_facade() -> cxx::UniquePtr<cxx_event::PlayerJoinEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_player_join_event_facade()) }
    }

    fn test_player_quit_event_facade() -> cxx::UniquePtr<cxx_event::PlayerQuitEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_player_quit_event_facade()) }
    }

    fn test_player_item_consume_event_facade()
    -> cxx::UniquePtr<cxx_event::PlayerItemConsumeEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_player_item_consume_event_facade()) }
    }

    fn test_player_game_mode_change_event_facade()
    -> cxx::UniquePtr<cxx_event::PlayerGameModeChangeEventFacade> {
        unsafe {
            cxx::UniquePtr::from_raw(aegilex_test_make_player_game_mode_change_event_facade())
        }
    }

    fn test_player_emote_event_facade() -> cxx::UniquePtr<cxx_event::PlayerEmoteEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_player_emote_event_facade()) }
    }

    fn test_player_skin_change_event_facade()
    -> cxx::UniquePtr<cxx_event::PlayerSkinChangeEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_player_skin_change_event_facade()) }
    }

    fn test_player_dimension_change_event_facade()
    -> cxx::UniquePtr<cxx_event::PlayerDimensionChangeEventFacade> {
        unsafe {
            cxx::UniquePtr::from_raw(aegilex_test_make_player_dimension_change_event_facade())
        }
    }

    fn test_player_bed_enter_event_facade() -> cxx::UniquePtr<cxx_event::PlayerBedEnterEventFacade>
    {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_player_bed_enter_event_facade()) }
    }

    fn test_player_bed_leave_event_facade() -> cxx::UniquePtr<cxx_event::PlayerBedLeaveEventFacade>
    {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_player_bed_leave_event_facade()) }
    }

    fn test_player_respawn_event_facade() -> cxx::UniquePtr<cxx_event::PlayerRespawnEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_player_respawn_event_facade()) }
    }

    fn test_player_item_held_event_facade() -> cxx::UniquePtr<cxx_event::PlayerItemHeldEventFacade>
    {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_player_item_held_event_facade()) }
    }

    fn test_weather_change_event_facade() -> cxx::UniquePtr<cxx_event::WeatherChangeEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_weather_change_event_facade()) }
    }

    fn test_thunder_change_event_facade() -> cxx::UniquePtr<cxx_event::ThunderChangeEventFacade> {
        unsafe { cxx::UniquePtr::from_raw(aegilex_test_make_thunder_change_event_facade()) }
    }
    use crate::abi::AEGILEX_NOT_FOUND;

    fn stub_host_context() -> HostContext {
        HostContext::new(crate::cxx_host::ffi::HostContext::test_stub()).unwrap()
    }

    fn store() -> PluginStoreState {
        PluginStoreState {
            host: stub_host_context(),
            handles: GuestHandles::new(),
            plugin_id: "hello".to_owned(),
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
        }
    }

    fn store_with_capabilities(capabilities: &[&str]) -> PluginStoreState {
        let mut store = store();
        store.policy.capabilities = capabilities
            .iter()
            .map(|value| (*value).to_owned())
            .collect();
        store
    }

    fn test_config() -> RuntimeConfig {
        RuntimeConfig {
            max_module_bytes: 0,
            max_nested_dispatch_depth: crate::config::DEFAULT_MAX_NESTED_DISPATCH_DEPTH,
            max_nbt_depth: 0,
            max_nbt_nodes: 0,
            max_nbt_compound_entries: 0,
            max_nbt_string_bytes: 0,
            max_nbt_array_bytes: 0,
            max_invocation_native_resources: 0,
            max_plugin_resource_slots: 0,
        }
    }

    #[test]
    fn denies_server_info_without_capabilities() {
        let mut store = store();

        assert_eq!(
            crate::core_host::imports::HostServer::get_name(&mut store),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        );
    }

    #[test]
    fn console_dispatch_requires_its_server_capability() {
        assert_eq!(
            crate::core_host::imports::HostServer::dispatch_console_command(
                &mut store(),
                "say test".to_owned()
            ),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        );
        assert!(
            crate::core_host::imports::HostServer::dispatch_console_command(
                &mut store_with_capabilities(&["server.dispatch-console-command"]),
                "say test".to_owned(),
            )
            .is_ok()
        );
    }

    #[test]
    fn registry_enchantment_list_round_trips_nonempty_records() {
        let mut store = store_with_capabilities(&["server.registry-enchantment-list"]);
        let values = crate::core_host::imports::HostServer::registry_enchantment_list(&mut store)
            .unwrap()
            .unwrap();

        assert_eq!(values.len(), 1);
        assert_eq!(values[0].id, "minecraft:sharpness");
        assert_eq!(values[0].max_level, 5);
    }

    #[test]
    fn list_plugins_requires_capability() {
        assert_eq!(
            crate::core_host::imports::HostPluginManager::list_plugins(&mut store()),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        );
    }

    #[test]
    fn list_providers_requires_capability() {
        assert_eq!(
            crate::core_host::imports::HostServiceBus::list_providers(
                &mut store(),
                "echo".to_owned(),
            ),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        );
    }

    #[test]
    fn list_plugins_returns_full_summary_list() {
        let mut store = store_with_capabilities(&["plugin-manager.list-plugins"]);
        let plugins = crate::core_host::imports::HostPluginManager::list_plugins(&mut store)
            .unwrap()
            .unwrap();

        assert_eq!(plugins[0].metadata.name, "hello");
    }

    #[test]
    fn list_providers_returns_id_and_spec() {
        let mut store = store_with_capabilities(&["service-bus.list-providers"]);
        let providers = crate::core_host::imports::HostServiceBus::list_providers(
            &mut store,
            "echo".to_owned(),
        )
        .unwrap()
        .unwrap();

        assert_eq!(providers.len(), 1);
        assert_ne!(providers[0].id, 0);
        assert_eq!(providers[0].spec.name, "echo");
    }

    #[test]
    fn matches_exact_interface_and_global_capabilities() {
        assert!(
            store_with_capabilities(&["logger.get-logger"])
                .require_capability("logger.get-logger")
                .is_ok()
        );
        assert_eq!(
            store_with_capabilities(&["logger.get-logger"]).require_capability("logger.get-name"),
            Err(crate::core_host::TypesHostError::Denied)
        );
        assert!(
            store_with_capabilities(&["dimension.*"])
                .require_capability("dimension.get-block")
                .is_ok()
        );
        assert_eq!(
            store_with_capabilities(&["dimension.*"]).require_capability("actor.player.kick"),
            Err(crate::core_host::TypesHostError::Denied)
        );
        assert!(
            store_with_capabilities(&["*"])
                .require_capability("actor.player.kick")
                .is_ok()
        );
    }

    #[test]
    fn network_policy_allows_only_matching_outbound_endpoints() {
        let rules = vec![
            manifest::NetworkRule {
                protocol: NetworkProtocol::Tcp,
                address: "203.0.113.10:443".parse().unwrap(),
            },
            manifest::NetworkRule {
                protocol: NetworkProtocol::Udp,
                address: "198.51.100.20:3478".parse().unwrap(),
            },
        ];

        assert!(network_allows(
            &rules,
            "203.0.113.10:443".parse().unwrap(),
            SocketAddrUse::TcpConnect
        ));
        assert!(!network_allows(
            &rules,
            "203.0.113.10:443".parse().unwrap(),
            SocketAddrUse::UdpConnect
        ));
        assert!(network_allows(
            &rules,
            "198.51.100.20:3478".parse().unwrap(),
            SocketAddrUse::UdpOutgoingDatagram
        ));
        assert!(!network_allows(
            &rules,
            "203.0.113.10:443".parse().unwrap(),
            SocketAddrUse::TcpBind
        ));
        assert!(!network_allows(
            &rules,
            "198.51.100.20:3478".parse().unwrap(),
            SocketAddrUse::UdpBind
        ));
    }

    #[test]
    fn denies_inventory_ops_without_capabilities() {
        let mut store = store();
        store.invocation_id = 42;
        let mut players = store.host.server().unwrap().listOnlinePlayers();
        let inventory = store
            .insert_inventory_resource(players.pin_mut().takePlayer(0).getInventory().asInventory())
            .unwrap();

        assert!(matches!(
            crate::core_host::imports::HostItemType::get_item_type(
                &mut store,
                "minecraft:apple".to_owned()
            ),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        ));
        assert_eq!(
            crate::core_host::imports::HostInventory::inventory_set_item(
                &mut store, inventory, 0, None,
            ),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        );
    }

    #[test]
    fn denies_item_stack_resource_methods_without_capabilities() {
        let mut store = store();
        store.invocation_id = 42;
        let item_type = store.host.server().unwrap().getItemType("minecraft:apple");
        let item = store
            .insert_item_stack_resource(item_type.createItemStack(1))
            .unwrap();

        assert_eq!(
            crate::core_host::imports::HostItemStack::item_stack_get_amount(&mut store, item),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        );
    }

    #[test]
    fn denies_inventory_wave4_ops_without_capabilities() {
        let mut store = store();
        store.invocation_id = 42;
        let mut players = store.host.server().unwrap().listOnlinePlayers();
        let inventory = store
            .insert_inventory_resource(players.pin_mut().takePlayer(0).getInventory().asInventory())
            .unwrap();
        let mut players = store.host.server().unwrap().listOnlinePlayers();
        let player_inventory = store
            .insert_player_inventory_resource(players.pin_mut().takePlayer(0).getInventory())
            .unwrap();

        assert_eq!(
            crate::core_host::imports::HostInventory::inventory_get_max_stack_size(
                &mut store, inventory,
            ),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        );
        assert!(matches!(
            crate::core_host::imports::HostInventory::inventory_add_item(
                &mut store,
                inventory,
                Vec::new(),
            ),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        ));
        assert_eq!(
            crate::core_host::imports::HostPlayerInventory::player_inventory_set_held_item_slot(
                &mut store,
                player_inventory,
                0,
            ),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        );
        assert_eq!(
            crate::core_host::imports::HostInventory::inventory_first_empty(&mut store, inventory,),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        );
        assert_eq!(
            crate::core_host::imports::HostInventory::inventory_is_empty(&mut store, inventory,),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        );
        assert!(matches!(
            crate::core_host::imports::HostItemType::create_item_stack(
                &mut store,
                "minecraft:apple".to_owned(),
                None
            ),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        ));
        assert_eq!(
            crate::core_host::imports::HostItemStack::item_stack_get_amount(&mut store, 0),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        );
    }

    #[test]
    fn guest_handles_are_scoped_to_the_invocation() {
        let mut store = store_with_capabilities(&["inventory.inventory.get-size"]);
        store.invocation_id = 42;
        let mut players = store.host.server().unwrap().listOnlinePlayers();
        let inventory = players.pin_mut().takePlayer(0).getInventory().asInventory();
        let handle = store
            .handles
            .insert_inventory(store.invocation_id, inventory);
        let inventory = store
            .resource_from_handle(ResourceKind::Inventory, handle)
            .unwrap();

        assert_ne!(handle, 0);
        assert_eq!(
            crate::core_host::imports::HostInventory::inventory_get_size(&mut store, inventory),
            Ok(Ok(36))
        );

        store.handles.clear_invocation(store.invocation_id);
        assert_eq!(
            crate::core_host::imports::HostInventory::inventory_get_size(&mut store, inventory),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );
    }

    #[test]
    fn player_resources_become_tombstones_after_invocation_cleanup() {
        let mut store = store_with_capabilities(&["actor.player.get-name"]);
        store.invocation_id = 42;
        let mut players = store.host.server().unwrap().listOnlinePlayers();
        let player = store
            .insert_player_resource(players.pin_mut().takePlayer(0))
            .unwrap();

        let player_rep = player;
        let player_handle = store
            .resource_slot(player_rep, ResourceKind::Player)
            .unwrap()
            .handle;
        assert_eq!(store.resource_slot_count, 1);
        assert_eq!(
            store
                .resources
                .get_raw::<crate::host::runtime::handles::ResourceSlot>(
                    crate::core_resources::ResourceToken::from_rep(player_rep)
                )
                .unwrap()
                .lifetime,
            ResourceLifetime::GuestOwned
        );

        store.clear_invocation_resources(store.invocation_id);

        assert_eq!(store.resource_slot_count, 1);
        assert!(
            store
                .resources
                .get_raw::<crate::host::runtime::handles::ResourceSlot>(
                    crate::core_resources::ResourceToken::from_rep(player_rep)
                )
                .is_ok()
        );
        assert!(store.handles.player(42, player_handle).is_none());

        store.invocation_id = 43;
        assert_eq!(
            crate::core_host::imports::HostActor::player_get_name(&mut store, player_rep),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );

        crate::core_host::imports::HostActor::drop_player(&mut store, player_rep).unwrap();
        assert_eq!(store.resource_slot_count, 0);
    }

    #[test]
    fn direct_conversions_produce_typed_resource_slots() {
        let mut store = store_with_capabilities(&[
            "actor.actor.as-mob",
            "actor.actor.as-player",
            "actor.mob.as-actor",
            "actor.player.as-actor",
        ]);
        store.invocation_id = 42;
        let actor = store.host.server().unwrap().getLevel().getActors("").get(0);
        let actor = store.insert_actor_resource(actor).unwrap();

        let mob = crate::core_host::imports::HostActor::actor_as_mob(&mut store, actor)
            .unwrap()
            .unwrap()
            .unwrap();
        assert_eq!(store.resource_slot_count, 2);
        assert!(store.resource_slot(mob, ResourceKind::Mob).is_ok());

        let converted_actor = crate::core_host::imports::HostActor::mob_as_actor(&mut store, mob)
            .unwrap()
            .unwrap();
        assert_eq!(store.resource_slot_count, 3);
        assert!(
            store
                .resource_slot(converted_actor, ResourceKind::Actor)
                .is_ok()
        );

        let player = crate::core_host::imports::HostActor::actor_as_player(&mut store, actor)
            .unwrap()
            .unwrap()
            .unwrap();
        assert_eq!(store.resource_slot_count, 4);
        assert!(store.resource_slot(player, ResourceKind::Player).is_ok());

        let player_actor =
            crate::core_host::imports::HostActor::player_as_actor(&mut store, player)
                .unwrap()
                .unwrap();
        assert_eq!(store.resource_slot_count, 5);
        assert!(
            store
                .resource_slot(player_actor, ResourceKind::Actor)
                .is_ok()
        );
    }

    #[test]
    fn denied_conversion_does_not_allocate_a_resource_slot() {
        let mut store = store();
        store.invocation_id = 42;
        let actor = store.host.server().unwrap().getLevel().getActors("").get(0);
        let actor = store.insert_actor_resource(actor).unwrap();
        let slots_before = store.resource_slot_count;

        assert!(matches!(
            crate::core_host::imports::HostActor::actor_as_mob(&mut store, actor,),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        ));
        assert_eq!(store.resource_slot_count, slots_before);

        assert!(matches!(
            crate::core_host::imports::HostActor::actor_as_player(&mut store, actor,),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        ));
        assert_eq!(store.resource_slot_count, slots_before);

        let mut player_store = store_with_capabilities(&["actor.actor.as-player"]);
        player_store.invocation_id = 42;
        let actor = player_store
            .host
            .server()
            .unwrap()
            .getLevel()
            .getActors("")
            .get(0);
        let actor = player_store.insert_actor_resource(actor).unwrap();
        let player =
            crate::core_host::imports::HostActor::actor_as_player(&mut player_store, actor)
                .unwrap()
                .unwrap()
                .unwrap();
        let slots_before = player_store.resource_slot_count;
        assert!(matches!(
            crate::core_host::imports::HostActor::player_as_actor(&mut player_store, player,),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        ));
        assert_eq!(player_store.resource_slot_count, slots_before);

        let mut item_store = store_with_capabilities(&["actor.actor.as-item-actor"]);
        item_store.invocation_id = 42;
        let actor = item_store
            .host
            .server()
            .unwrap()
            .getLevel()
            .getActors("")
            .get(0);
        let actor = item_store.insert_actor_resource(actor).unwrap();
        let item_actor =
            crate::core_host::imports::HostActor::actor_as_item_actor(&mut item_store, actor)
                .unwrap()
                .unwrap()
                .unwrap();
        let slots_before = item_store.resource_slot_count;
        assert!(matches!(
            crate::core_host::imports::HostActor::item_actor_as_actor(&mut item_store, item_actor,),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        ));
        assert_eq!(item_store.resource_slot_count, slots_before);
    }

    #[test]
    fn item_actor_conversion_is_typed_and_rejects_non_item_actors() {
        let mut store = store_with_capabilities(&[
            "actor.actor.as-mob",
            "actor.actor.as-item-actor",
            "actor.item-actor.as-actor",
        ]);
        store.invocation_id = 42;
        let item_actor_source = store.host.server().unwrap().getLevel().getActors("").get(0);
        let item_actor_source = store.insert_actor_resource(item_actor_source).unwrap();

        let item_actor = crate::core_host::imports::HostActor::actor_as_item_actor(
            &mut store,
            item_actor_source,
        )
        .unwrap()
        .unwrap()
        .unwrap();
        assert_eq!(store.resource_slot_count, 2);
        assert_eq!(
            store
                .resource_slot(item_actor, ResourceKind::ItemActor)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );

        let converted_actor =
            crate::core_host::imports::HostActor::item_actor_as_actor(&mut store, item_actor)
                .unwrap()
                .unwrap();
        assert_eq!(store.resource_slot_count, 3);
        assert_eq!(
            store
                .resource_slot(converted_actor, ResourceKind::Actor)
                .unwrap()
                .lifetime,
            ResourceLifetime::GuestOwned
        );

        let mob = crate::core_host::imports::HostActor::actor_as_mob(&mut store, item_actor_source)
            .unwrap()
            .unwrap()
            .unwrap();
        let non_item_native = store
            .handles
            .mob(
                store.invocation_id,
                store.resource_slot(mob, ResourceKind::Mob).unwrap().handle,
            )
            .unwrap()
            .asActor();
        let non_item = store.insert_actor_resource(non_item_native).unwrap();
        assert!(matches!(
            crate::core_host::imports::HostActor::actor_as_item_actor(&mut store, non_item,),
            Ok(Ok(None))
        ));
    }

    #[test]
    fn item_actor_mutations_validate_and_stack_is_an_owned_snapshot() {
        let mut store = store_with_capabilities(&[
            "actor.actor.as-item-actor",
            "actor.item-actor.get-item-stack",
            "actor.item-actor.set-item-stack",
            "actor.item-actor.get-pickup-delay",
            "actor.item-actor.set-pickup-delay",
            "actor.item-actor.is-unlimited-lifetime",
            "actor.item-actor.set-unlimited-lifetime",
            "actor.item-actor.get-thrower",
            "actor.item-actor.set-thrower",
            "item-stack.item-stack.get-amount",
            "item-stack.item-stack.set-amount",
        ]);
        store.invocation_id = 42;
        let actor = store.host.server().unwrap().getLevel().getActors("").get(0);
        let actor = store.insert_actor_resource(actor).unwrap();
        let item_actor =
            crate::core_host::imports::HostActor::actor_as_item_actor(&mut store, actor)
                .unwrap()
                .unwrap()
                .unwrap();

        assert_eq!(
            crate::core_host::imports::HostActor::item_actor_set_pickup_delay(
                &mut store, item_actor, -1,
            ),
            Ok(Err(crate::core_host::TypesHostError::InvalidInput))
        );
        crate::core_host::imports::HostActor::item_actor_set_pickup_delay(
            &mut store, item_actor, 20,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostActor::item_actor_get_pickup_delay(
                &mut store, item_actor,
            ),
            Ok(Ok(20))
        );
        crate::core_host::imports::HostActor::item_actor_set_unlimited_lifetime(
            &mut store, item_actor, true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostActor::item_actor_is_unlimited_lifetime(
                &mut store, item_actor,
            ),
            Ok(Ok(true))
        );
        crate::core_host::imports::HostActor::item_actor_set_thrower(
            &mut store,
            item_actor,
            Some(7),
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostActor::item_actor_get_thrower(&mut store, item_actor,),
            Ok(Ok(Some(7)))
        );
        crate::core_host::imports::HostActor::item_actor_set_thrower(&mut store, item_actor, None)
            .unwrap()
            .unwrap();
        assert_eq!(
            crate::core_host::imports::HostActor::item_actor_get_thrower(&mut store, item_actor,),
            Ok(Ok(None))
        );

        let stack =
            crate::core_host::imports::HostActor::item_actor_get_item_stack(&mut store, item_actor)
                .unwrap()
                .unwrap();
        crate::core_host::imports::HostItemStack::item_stack_set_amount(&mut store, stack, 8)
            .unwrap()
            .unwrap();
        assert_eq!(
            crate::core_host::imports::HostItemStack::item_stack_get_amount(&mut store, stack,),
            Ok(Ok(8))
        );
        let fresh_stack =
            crate::core_host::imports::HostActor::item_actor_get_item_stack(&mut store, item_actor)
                .unwrap()
                .unwrap();
        assert_eq!(
            crate::core_host::imports::HostItemStack::item_stack_get_amount(
                &mut store,
                fresh_stack,
            ),
            Ok(Ok(1))
        );

        let replacement = store
            .host
            .server()
            .unwrap()
            .getItemType("minecraft:apple")
            .createItemStack(3);
        let replacement = store.insert_item_stack_resource(replacement).unwrap();
        crate::core_host::imports::HostItemStack::item_stack_set_amount(&mut store, replacement, 3)
            .unwrap()
            .unwrap();
        crate::core_host::imports::HostActor::item_actor_set_item_stack(
            &mut store,
            item_actor,
            replacement,
        )
        .unwrap()
        .unwrap();
        crate::core_host::imports::HostItemStack::item_stack_set_amount(&mut store, replacement, 8)
            .unwrap()
            .unwrap();
        crate::core_host::imports::HostItemStack::drop_item_stack(&mut store, replacement).unwrap();
        let replaced_stack =
            crate::core_host::imports::HostActor::item_actor_get_item_stack(&mut store, item_actor)
                .unwrap()
                .unwrap();
        assert_eq!(
            crate::core_host::imports::HostItemStack::item_stack_get_amount(
                &mut store,
                replaced_stack,
            ),
            Ok(Ok(3))
        );
    }

    #[test]
    fn item_actor_cleanup_and_denial_do_not_leave_live_resources() {
        let mut denied = store();
        denied.invocation_id = 42;
        let actor = denied
            .host
            .server()
            .unwrap()
            .getLevel()
            .getActors("")
            .get(0);
        let actor = denied.insert_actor_resource(actor).unwrap();
        let slots_before = denied.resource_slot_count;
        assert!(matches!(
            crate::core_host::imports::HostActor::actor_as_item_actor(&mut denied, actor,),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        ));
        assert_eq!(denied.resource_slot_count, slots_before);

        let mut denied_set = store_with_capabilities(&[
            "actor.actor.as-item-actor",
            "actor.item-actor.get-item-stack",
            "item-stack.item-stack.get-amount",
        ]);
        denied_set.invocation_id = 42;
        let actor = denied_set
            .host
            .server()
            .unwrap()
            .getLevel()
            .getActors("")
            .get(0);
        let actor = denied_set.insert_actor_resource(actor).unwrap();
        let item_actor =
            crate::core_host::imports::HostActor::actor_as_item_actor(&mut denied_set, actor)
                .unwrap()
                .unwrap()
                .unwrap();
        let replacement = denied_set
            .host
            .server()
            .unwrap()
            .getItemType("minecraft:apple")
            .createItemStack(3);
        let replacement = denied_set.insert_item_stack_resource(replacement).unwrap();
        let slots_before = denied_set.resource_slot_count;
        assert_eq!(
            crate::core_host::imports::HostActor::item_actor_set_item_stack(
                &mut denied_set,
                item_actor,
                replacement,
            ),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        );
        assert_eq!(denied_set.resource_slot_count, slots_before);
        let stack = crate::core_host::imports::HostActor::item_actor_get_item_stack(
            &mut denied_set,
            item_actor,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostItemStack::item_stack_get_amount(&mut denied_set, stack,),
            Ok(Ok(1))
        );

        let mut store = store_with_capabilities(&[
            "actor.actor.as-item-actor",
            "actor.item-actor.get-pickup-delay",
        ]);
        store.invocation_id = 42;
        let actor = store.host.server().unwrap().getLevel().getActors("").get(0);
        let actor = store.insert_actor_resource(actor).unwrap();
        let item_actor =
            crate::core_host::imports::HostActor::actor_as_item_actor(&mut store, actor)
                .unwrap()
                .unwrap()
                .unwrap();
        let item_actor_rep = item_actor;

        assert_eq!(
            crate::core_host::imports::HostActor::item_actor_get_pickup_delay(&mut store, actor,),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );
        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        assert_eq!(
            crate::core_host::imports::HostActor::item_actor_get_pickup_delay(
                &mut store,
                item_actor_rep,
            ),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );
        assert!(
            store
                .resources
                .get_raw::<crate::host::runtime::handles::ResourceSlot>(
                    crate::core_resources::ResourceToken::from_rep(item_actor_rep)
                )
                .is_err()
        );
    }

    #[test]
    fn item_stack_resources_drop_to_tombstones_and_reject_invalid_slots() {
        let mut store = store_with_capabilities(&["item-stack.item-stack.get-amount"]);
        store.invocation_id = 42;
        let item_type = store.host.server().unwrap().getItemType("minecraft:apple");
        let item = store
            .insert_item_stack_resource(item_type.createItemStack(1))
            .unwrap();

        let item_rep = item;
        assert_eq!(
            crate::core_host::imports::HostItemStack::item_stack_get_amount(&mut store, item_rep,),
            Ok(Ok(1))
        );

        crate::core_host::imports::HostItemStack::drop_item_stack(&mut store, item_rep).unwrap();
        assert_eq!(store.resource_slot_count, 0);
        assert!(matches!(
            crate::core_host::imports::HostItemStack::item_stack_get_amount(&mut store, item_rep,),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        ));

        assert!(matches!(
            crate::core_host::imports::HostItemStack::item_stack_get_amount(&mut store, u32::MAX,),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        ));
    }

    #[test]
    fn item_stack_clone_is_independent_and_inventory_copies_resources() {
        let mut store = store_with_capabilities(&[
            "item-type.create-item-stack",
            "item-stack.item-stack.get-amount",
            "item-stack.item-stack.set-amount",
            "item-stack.item-stack.clone",
            "inventory.inventory.get-item",
            "inventory.inventory.set-item",
        ]);
        store.invocation_id = 42;
        let item = crate::core_host::imports::HostItemType::create_item_stack(
            &mut store,
            "minecraft:apple".to_owned(),
            Some(1),
        )
        .unwrap()
        .unwrap();
        let copy = crate::core_host::imports::HostItemStack::item_stack_clone(&mut store, item)
            .unwrap()
            .unwrap();
        crate::core_host::imports::HostItemStack::item_stack_set_amount(&mut store, copy, 7)
            .unwrap()
            .unwrap();
        assert_eq!(
            crate::core_host::imports::HostItemStack::item_stack_get_amount(&mut store, item,),
            Ok(Ok(1))
        );
        assert_eq!(
            crate::core_host::imports::HostItemStack::item_stack_get_amount(&mut store, copy,),
            Ok(Ok(7))
        );

        let mut players = store.host.server().unwrap().listOnlinePlayers();
        let inventory = store
            .insert_inventory_resource(players.pin_mut().takePlayer(0).getInventory().asInventory())
            .unwrap();
        crate::core_host::imports::HostInventory::inventory_set_item(
            &mut store,
            inventory,
            0,
            Some(item),
        )
        .unwrap()
        .unwrap();
        let slot =
            crate::core_host::imports::HostInventory::inventory_get_item(&mut store, inventory, 0)
                .unwrap()
                .unwrap();
        assert_ne!(slot, Some(item));
        assert_eq!(
            crate::core_host::imports::HostItemStack::item_stack_get_amount(
                &mut store,
                slot.unwrap()
            ),
            Ok(Ok(1))
        );
    }

    #[test]
    fn item_stack_nbt_round_trips_and_rejects_malformed_trees() {
        let mut store = store_with_capabilities(&[
            "item-type.create-item-stack",
            "item-stack.item-stack.get-nbt",
            "item-stack.item-stack.set-nbt",
            "nbt.from-end",
            "nbt.from-int",
            "nbt.from-string",
            "nbt.from-byte-array",
            "nbt.from-int-array",
            "nbt.from-list",
            "nbt.from-compound",
            "nbt.tag.get-type",
            "nbt.tag.get-int",
            "nbt.tag.get-compound-keys",
            "nbt.tag.get-compound",
        ]);
        store.invocation_id = 42;
        let item = crate::core_host::imports::HostItemType::create_item_stack(
            &mut store,
            "minecraft:apple".to_owned(),
            Some(1),
        )
        .unwrap()
        .unwrap();
        let item_rep = item;
        let answer = crate::core_host::imports::HostNbt::from_int(&mut store, 42)
            .unwrap()
            .unwrap();
        let root = crate::core_host::imports::HostNbt::from_compound(
            &mut store,
            vec![crate::core_host::NbtCompoundEntry {
                key: "answer".to_owned(),
                value: answer,
            }],
        )
        .unwrap()
        .unwrap();
        crate::core_host::imports::HostItemStack::item_stack_set_nbt(&mut store, item_rep, root)
            .unwrap()
            .unwrap();
        let root = crate::core_host::imports::HostItemStack::item_stack_get_nbt(&mut store, item)
            .unwrap()
            .unwrap();
        let root_rep = root;
        assert_eq!(
            crate::core_host::imports::HostNbt::tag_get_type(&mut store, root_rep,),
            Ok(Ok(crate::core_host::NbtTagType::Compound))
        );
        assert_eq!(
            crate::core_host::imports::HostNbt::tag_get_compound_keys(&mut store, root_rep,),
            Ok(Ok(vec!["answer".to_owned()]))
        );
        let answer = crate::core_host::imports::HostNbt::tag_get_compound(
            &mut store,
            root_rep,
            "answer".to_owned(),
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostNbt::tag_get_int(&mut store, answer.unwrap()),
            Ok(Ok(42))
        );

        // A non-compound root is rejected by item-stack.set-nbt.
        let scalar = crate::core_host::imports::HostNbt::from_int(&mut store, 7)
            .unwrap()
            .unwrap();
        assert_eq!(
            crate::core_host::imports::HostItemStack::item_stack_set_nbt(
                &mut store, item_rep, scalar,
            ),
            Ok(Err(crate::core_host::TypesHostError::InvalidInput))
        );

        // Heterogeneous and End-containing lists are rejected at construction.
        let int_tag = crate::core_host::imports::HostNbt::from_int(&mut store, 1)
            .unwrap()
            .unwrap();
        let string_tag =
            crate::core_host::imports::HostNbt::from_string(&mut store, "not an int".to_owned())
                .unwrap()
                .unwrap();
        assert!(matches!(
            crate::core_host::imports::HostNbt::from_list(&mut store, vec![int_tag, string_tag]),
            Ok(Err(crate::core_host::TypesHostError::InvalidInput))
        ));
        let end_tag = crate::core_host::imports::HostNbt::from_end(&mut store)
            .unwrap()
            .unwrap();
        assert!(matches!(
            crate::core_host::imports::HostNbt::from_list(&mut store, vec![end_tag]),
            Ok(Err(crate::core_host::TypesHostError::InvalidInput))
        ));

        // Duplicate compound keys are rejected at construction.
        let one = crate::core_host::imports::HostNbt::from_int(&mut store, 1)
            .unwrap()
            .unwrap();
        let two = crate::core_host::imports::HostNbt::from_int(&mut store, 2)
            .unwrap()
            .unwrap();
        assert!(matches!(
            crate::core_host::imports::HostNbt::from_compound(
                &mut store,
                vec![
                    crate::core_host::NbtCompoundEntry {
                        key: "dup".to_owned(),
                        value: one,
                    },
                    crate::core_host::NbtCompoundEntry {
                        key: "dup".to_owned(),
                        value: two,
                    },
                ],
            ),
            Ok(Err(crate::core_host::TypesHostError::InvalidInput))
        ));

        // Default RuntimeConfig does not impose arbitrary NBT size quotas.
        let mut deep = crate::core_host::imports::HostNbt::from_int(&mut store, 42)
            .unwrap()
            .unwrap();
        for _ in 0..33 {
            deep = crate::core_host::imports::HostNbt::from_compound(
                &mut store,
                vec![crate::core_host::NbtCompoundEntry {
                    key: "child".to_owned(),
                    value: deep,
                }],
            )
            .unwrap()
            .unwrap();
        }
        assert!(matches!(
            crate::core_host::imports::HostNbt::tag_get_type(&mut store, deep),
            Ok(Ok(crate::core_host::NbtTagType::Compound))
        ));
    }

    #[test]
    fn runtime_config_enforces_nbt_quotas_when_configured() {
        let mut store = store_with_capabilities(&["nbt.from-int", "nbt.from-compound"]);
        store.config.max_nbt_depth = 1;
        let leaf = crate::core_host::imports::HostNbt::from_int(&mut store, 42)
            .unwrap()
            .unwrap();
        let child = crate::core_host::imports::HostNbt::from_compound(
            &mut store,
            vec![crate::core_host::NbtCompoundEntry {
                key: "child".to_owned(),
                value: leaf,
            }],
        )
        .unwrap()
        .unwrap();

        assert!(matches!(
            crate::core_host::imports::HostNbt::from_compound(
                &mut store,
                vec![crate::core_host::NbtCompoundEntry {
                    key: "root".to_owned(),
                    value: child,
                }],
            ),
            Ok(Err(crate::core_host::TypesHostError::LimitExceeded))
        ));
    }

    #[test]
    fn item_stack_ref_is_host_borrowed_invalidated_and_clones_owned_stack() {
        let mut store = store_with_capabilities(&[
            "item-type.create-item-stack",
            "item-stack.item-stack-ref.get-type-id",
            "item-stack.item-stack-ref.get-nbt",
            "item-stack.item-stack-ref.clone",
            "item-stack.item-stack.get-amount",
            "item-stack.item-stack.set-amount",
            "nbt.tag.get-type",
            "nbt.tag.get-compound-keys",
        ]);
        store.invocation_id = 42;
        let source = crate::core_host::imports::HostItemType::create_item_stack(
            &mut store,
            "minecraft:apple".to_owned(),
            Some(1),
        )
        .unwrap()
        .unwrap();
        let source_handle = store
            .resource_slot(source, ResourceKind::ItemStack)
            .unwrap()
            .handle;
        let native_source = store
            .handles
            .item_stack(store.invocation_id, source_handle)
            .unwrap();
        let item_ref = cxx_inventory::borrow_item_stack(native_source);
        let item_ref = store.insert_item_stack_ref_resource(item_ref).unwrap();
        let item_ref_rep = item_ref;

        assert_eq!(store.resource_slot_count, 2);
        assert_eq!(
            store
                .resource_slot(item_ref_rep, ResourceKind::ItemStackRef)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            crate::core_host::imports::HostItemStack::item_stack_ref_get_type_id(
                &mut store, item_ref
            ),
            Ok(Ok("minecraft:apple".to_owned()))
        );
        let nbt = crate::core_host::imports::HostItemStack::item_stack_ref_get_nbt(
            &mut store,
            item_ref_rep,
        )
        .unwrap()
        .unwrap();
        let nbt_rep = nbt;
        assert_eq!(
            crate::core_host::imports::HostNbt::tag_get_type(&mut store, nbt_rep,),
            Ok(Ok(crate::core_host::NbtTagType::Compound))
        );
        assert_eq!(
            crate::core_host::imports::HostNbt::tag_get_compound_keys(&mut store, nbt_rep,),
            Ok(Ok(Vec::new()))
        );

        let clone = crate::core_host::imports::HostItemStack::item_stack_ref_clone(
            &mut store,
            item_ref_rep,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            store
                .resource_slot(clone, ResourceKind::ItemStack)
                .unwrap()
                .lifetime,
            ResourceLifetime::GuestOwned
        );
        crate::core_host::imports::HostItemStack::item_stack_set_amount(&mut store, clone, 7)
            .unwrap()
            .unwrap();
        assert_eq!(
            crate::core_host::imports::HostItemStack::item_stack_get_amount(&mut store, source,),
            Ok(Ok(1))
        );

        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        assert_eq!(store.resource_slot_count, 3);
        assert_eq!(
            crate::core_host::imports::HostItemStack::item_stack_ref_get_type_id(
                &mut store,
                item_ref_rep,
            ),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );
    }

    #[test]
    fn block_resources_are_scoped_and_reclaimed() {
        let mut store = store_with_capabilities(&["block.block.get-type"]);
        store.invocation_id = 42;
        let block = store
            .host
            .server()
            .unwrap()
            .getLevel()
            .getBlock("overworld", 0, 0, 0);
        let block = store.insert_block_resource(block).unwrap();
        let block_rep = block;

        assert!(crate::core_host::imports::HostBlock::block_get_type(&mut store, block).is_ok());
        assert_eq!(store.resource_slot_count, 1);

        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        assert_eq!(
            crate::core_host::imports::HostBlock::block_get_type(&mut store, block_rep),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );

        assert!(crate::core_host::imports::HostBlock::drop_block(&mut store, block_rep).is_ok());
        assert_eq!(store.resource_slot_count, 0);
    }

    #[test]
    fn block_type_creates_block_data_from_a_type_id() {
        let mut denied = store();
        assert_eq!(
            crate::core_host::imports::HostBlockType::has_item_type(
                &mut denied,
                "minecraft:stone".to_owned(),
            ),
            Ok(Err(crate::core_host::TypesHostError::Denied))
        );

        let mut store = store_with_capabilities(&[
            "block-type.has-item-type",
            "block-type.create-block-data",
            "block-data.block-data.get-type",
            "block-data.block-data.get-runtime-id",
        ]);
        assert_eq!(
            crate::core_host::imports::HostBlockType::has_item_type(
                &mut store,
                "minecraft:stone".to_owned()
            ),
            Ok(Ok(true))
        );
        let data = crate::core_host::imports::HostBlockType::create_block_data(
            &mut store,
            "minecraft:stone".to_owned(),
        )
        .unwrap()
        .unwrap();

        assert_eq!(
            crate::core_host::imports::HostBlockData::block_data_get_type(&mut store, data),
            Ok(Ok("minecraft:stone".to_owned()))
        );
    }

    #[test]
    fn block_clone_is_an_independent_owned_block_facade() {
        let mut store = store_with_capabilities(&[
            "block.block.clone",
            "block.block.get-type",
            "block.block.set-type",
        ]);
        store.invocation_id = 42;
        let source = store
            .host
            .server()
            .unwrap()
            .getLevel()
            .getBlock("overworld", 0, 0, 0);
        let source = store.insert_block_resource(source).unwrap();
        let source_rep = source;
        let clone = crate::core_host::imports::HostBlock::block_clone(&mut store, source)
            .unwrap()
            .unwrap();

        let source_handle = store
            .resource_slot(source_rep, ResourceKind::Block)
            .unwrap()
            .handle;
        let clone_handle = store
            .resource_slot(clone, ResourceKind::Block)
            .unwrap()
            .handle;
        assert_ne!(source_handle, clone_handle);
        assert_eq!(
            store
                .resource_slot(clone, ResourceKind::Block)
                .unwrap()
                .lifetime,
            ResourceLifetime::GuestOwned
        );

        crate::core_host::imports::HostBlock::block_set_type(
            &mut store,
            clone,
            "minecraft:dirt".to_owned(),
            None,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostBlock::block_get_type(&mut store, source_rep,),
            Ok(Ok("minecraft:stone".to_owned()))
        );
    }

    #[test]
    fn host_borrowed_resource_slots_are_reclaimed_at_invocation_end() {
        let mut store = store();
        store.invocation_id = 42;
        let guest_handle = store.handles.insert_block(
            store.invocation_id,
            store
                .host
                .server()
                .unwrap()
                .getLevel()
                .getBlock("overworld", 0, 0, 0),
        );
        let guest_resource = store
            .resource_from_handle(ResourceKind::Block, guest_handle)
            .unwrap();
        let host_borrowed_handle = store.handles.insert_block(
            store.invocation_id,
            store
                .host
                .server()
                .unwrap()
                .getLevel()
                .getBlock("overworld", 1, 0, 0),
        );
        let host_borrowed_resource = store
            .host_borrowed_resource_from_handle(ResourceKind::Block, host_borrowed_handle)
            .unwrap();
        let host_borrowed_child_handle = store.handles.insert_block(
            store.invocation_id,
            store
                .host
                .server()
                .unwrap()
                .getLevel()
                .getBlock("overworld", 2, 0, 0),
        );
        let host_borrowed_child_resource = store
            .host_borrowed_child_resource_from_handle(
                ResourceKind::Block,
                host_borrowed_child_handle,
                host_borrowed_resource,
            )
            .unwrap();

        assert_eq!(store.resource_slot_count, 3);
        assert_eq!(store.host_borrowed_slots.len(), 2);
        assert!(
            store
                .handles
                .block(store.invocation_id, guest_handle)
                .is_some()
        );
        assert!(
            store
                .handles
                .block(store.invocation_id, host_borrowed_handle)
                .is_some()
        );

        store.clear_invocation_resources(store.invocation_id);

        assert_eq!(store.resource_slot_count, 1);
        assert!(store.host_borrowed_slots.is_empty());
        assert_eq!(
            store
                .resources
                .get_raw::<crate::host::runtime::handles::ResourceSlot>(
                    crate::core_resources::ResourceToken::from_rep(guest_resource)
                )
                .unwrap()
                .lifetime,
            ResourceLifetime::GuestOwned
        );
        assert!(
            store
                .resources
                .get_raw::<crate::host::runtime::handles::ResourceSlot>(
                    crate::core_resources::ResourceToken::from_rep(host_borrowed_resource)
                )
                .is_err()
        );
        assert!(
            store
                .resources
                .get_raw::<crate::host::runtime::handles::ResourceSlot>(
                    crate::core_resources::ResourceToken::from_rep(host_borrowed_child_resource)
                )
                .is_err()
        );
        assert!(
            store
                .handles
                .block(store.invocation_id, guest_handle)
                .is_none()
        );
        assert!(
            store
                .handles
                .block(store.invocation_id, host_borrowed_handle)
                .is_none()
        );
        assert!(
            store
                .handles
                .block(store.invocation_id, host_borrowed_child_handle)
                .is_none()
        );

        store.clear_invocation_resources(store.invocation_id);
        assert_eq!(store.resource_slot_count, 1);

        store
            .drop_resource(guest_resource, ResourceKind::Block)
            .unwrap();
        assert_eq!(store.resource_slot_count, 0);
    }

    #[test]
    fn player_chat_event_resource_slots_are_invocation_scoped() {
        let mut store = store();
        store.invocation_id = 42;
        let token = store
            .resources
            .insert_value(
                ResourceSlot {
                    invocation_id: store.invocation_id,
                    handle: 1,
                    lifetime: ResourceLifetime::HostBorrowed,
                },
                ResourceKind::PlayerChatEvent as u32,
                RESOURCE_OWNER,
                ResourceLifetime::HostBorrowed,
                store.invocation_id,
            )
            .unwrap();
        store.resource_slot_count = 1;
        store
            .host_borrowed_slots
            .push((store.invocation_id, token.rep()));
        let event = token.rep();

        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerChatEvent)
                .unwrap()
                .handle,
            1
        );
        assert!(store.resource_slot(event, ResourceKind::Player).is_err());

        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        assert!(
            store
                .resource_slot(event, ResourceKind::PlayerChatEvent)
                .is_err()
        );
    }

    #[test]
    fn player_interact_event_children_are_host_borrowed_and_tombstoned() {
        let mut store = store_with_capabilities(&[
            "player-interact-event.player-interact-event.get-player",
            "player-interact-event.player-interact-event.get-action",
            "player-interact-event.player-interact-event.get-item",
            "player-interact-event.player-interact-event.get-block",
            "player-interact-event.player-interact-event.get-block-face",
            "player-interact-event.player-interact-event.get-clicked-position",
            "player-interact-event.player-interact-event.is-cancelled",
            "player-interact-event.player-interact-event.set-cancelled",
            "block.block.get-type",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_player_interact_event_resource(test_player_interact_event_facade())
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerInteractEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        let player =
            crate::core_host::imports::HostPlayerInteractEvent::player_interact_event_get_player(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        let item =
            crate::core_host::imports::HostPlayerInteractEvent::player_interact_event_get_item(
                &mut store, event,
            )
            .unwrap()
            .unwrap()
            .unwrap();
        let block =
            crate::core_host::imports::HostPlayerInteractEvent::player_interact_event_get_block(
                &mut store, event,
            )
            .unwrap()
            .unwrap()
            .unwrap();
        assert_eq!(
            store
                .resource_slot(player, ResourceKind::Player)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            store
                .resource_slot(item, ResourceKind::ItemStackRef)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            store
                .resource_slot(block, ResourceKind::Block)
                .unwrap()
                .lifetime,
            ResourceLifetime::GuestOwned
        );
        assert_eq!(
            crate::core_host::imports::HostPlayerInteractEvent::player_interact_event_get_action(
                &mut store, event,
            ),
            Ok(Ok(
                crate::core_host::PlayerInteractEventInteractAction::LeftClickBlock
            ))
        );
        assert!(matches!(
            crate::core_host::imports::HostPlayerInteractEvent::player_interact_event_get_clicked_position(
                &mut store,
                event,
            ),
            Ok(Ok(None))
        ));
        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        for (rep, kind) in [
            (event, ResourceKind::PlayerInteractEvent),
            (player, ResourceKind::Player),
            (item, ResourceKind::ItemStackRef),
        ] {
            assert!(store.resource_slot(rep, kind).is_err());
        }
        assert!(store.resource_slot(block, ResourceKind::Block).is_ok());
        assert_eq!(
            crate::core_host::imports::HostBlock::block_get_type(&mut store, block),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );
        assert_eq!(
            crate::core_host::imports::HostPlayerInteractEvent::player_interact_event_is_cancelled(
                &mut store, event,
            ),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );
    }

    #[test]
    fn player_interact_actor_event_children_are_host_borrowed_and_tombstoned() {
        let mut store = store_with_capabilities(&[
            "player-interact-actor-event.player-interact-actor-event.get-player",
            "player-interact-actor-event.player-interact-actor-event.get-actor",
            "player-interact-actor-event.player-interact-actor-event.is-cancelled",
            "player-interact-actor-event.player-interact-actor-event.set-cancelled",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_player_interact_actor_event_resource(test_player_interact_actor_event_facade())
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerInteractActorEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        let player = crate::core_host::imports::HostPlayerInteractActorEvent::player_interact_actor_event_get_player(
            &mut store,
            event,
        )
        .unwrap()
        .unwrap();
        let actor = crate::core_host::imports::HostPlayerInteractActorEvent::player_interact_actor_event_get_actor(
            &mut store,
            event,
        )
        .unwrap()
        .unwrap();
        for (resource, kind) in [(player, ResourceKind::Player), (actor, ResourceKind::Actor)] {
            assert_eq!(
                store.resource_slot(resource, kind).unwrap().lifetime,
                ResourceLifetime::HostBorrowed
            );
        }
        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        for (resource, kind) in [
            (event, ResourceKind::PlayerInteractActorEvent),
            (player, ResourceKind::Player),
            (actor, ResourceKind::Actor),
        ] {
            assert!(store.resource_slot(resource, kind).is_err());
        }
        assert_eq!(
            crate::core_host::imports::HostPlayerInteractActorEvent::player_interact_actor_event_is_cancelled(
                &mut store,
                event,
            ),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );
    }

    #[test]
    fn actor_damage_event_actor_child_is_host_borrowed_and_tombstoned() {
        let mut store =
            store_with_capabilities(&["actor-damage-event.actor-damage-event.get-actor"]);
        store.invocation_id = 42;
        let event = store
            .insert_actor_damage_event_resource(test_actor_damage_event_facade())
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::ActorDamageEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        let actor = crate::core_host::imports::HostActorDamageEvent::actor_damage_event_get_actor(
            &mut store, event,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            store
                .resource_slot(actor, ResourceKind::Actor)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );

        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        for (resource, kind) in [
            (event, ResourceKind::ActorDamageEvent),
            (actor, ResourceKind::Actor),
        ] {
            assert!(store.resource_slot(resource, kind).is_err());
        }
        assert!(matches!(
            crate::core_host::imports::HostActorDamageEvent::actor_damage_event_get_actor(
                &mut store, event,
            ),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        ));
    }

    #[test]
    fn player_pickup_item_event_resources_are_scoped_and_typed() {
        let mut store = store_with_capabilities(&[
            "player-pickup-item-event.player-pickup-item-event.get-player",
            "player-pickup-item-event.player-pickup-item-event.get-item-actor",
            "player-pickup-item-event.player-pickup-item-event.is-cancelled",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_player_pickup_item_event_resource(test_player_pickup_item_event_facade())
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerPickupItemEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        let player = crate::core_host::imports::HostPlayerPickupItemEvent::player_pickup_item_event_get_player(
            &mut store,
            event,
        )
        .unwrap()
        .unwrap();
        let actor = crate::core_host::imports::HostPlayerPickupItemEvent::player_pickup_item_event_get_item_actor(
            &mut store,
            event,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            store
                .resource_slot(player, ResourceKind::Player)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            store
                .resource_slot(actor, ResourceKind::Actor)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert!(store.resource_slot(event, ResourceKind::Player).is_err());
        assert_eq!(
            crate::core_host::imports::HostPlayerPickupItemEvent::player_pickup_item_event_is_cancelled(
                &mut store,
                player,
            ),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );

        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        assert!(
            store
                .resource_slot(event, ResourceKind::PlayerPickupItemEvent)
                .is_err()
        );
        assert!(store.resource_slot(player, ResourceKind::Player).is_err());
        assert!(store.resource_slot(actor, ResourceKind::Actor).is_err());
    }

    #[test]
    fn player_drop_item_event_children_are_host_borrowed_and_expire() {
        let mut store = store_with_capabilities(&[
            "player-drop-item-event.player-drop-item-event.get-player",
            "player-drop-item-event.player-drop-item-event.get-item",
            "player-drop-item-event.player-drop-item-event.is-cancelled",
            "item-stack.item-stack-ref.get-type-id",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_player_drop_item_event_resource(test_player_drop_item_event_facade())
            .unwrap();
        let player =
            crate::core_host::imports::HostPlayerDropItemEvent::player_drop_item_event_get_player(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        let item =
            crate::core_host::imports::HostPlayerDropItemEvent::player_drop_item_event_get_item(
                &mut store, event,
            )
            .unwrap()
            .unwrap();

        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerDropItemEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            store
                .resource_slot(player, ResourceKind::Player)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            store
                .resource_slot(item, ResourceKind::ItemStackRef)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert!(crate::core_host::imports::HostPlayerDropItemEvent::player_drop_item_event_is_cancelled(
            &mut store,
            item,
        )
        .unwrap()
        .is_err());

        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        assert!(
            store
                .resource_slot(event, ResourceKind::PlayerDropItemEvent)
                .is_err()
        );
        assert!(store.resource_slot(player, ResourceKind::Player).is_err());
        assert!(
            store
                .resource_slot(item, ResourceKind::ItemStackRef)
                .is_err()
        );
        assert_eq!(
            crate::core_host::imports::HostItemStack::item_stack_ref_get_type_id(&mut store, item,),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );
    }

    #[test]
    fn block_break_event_children_are_host_borrowed_and_expire() {
        let mut store = store_with_capabilities(&[
            "block-break-event.block-break-event.get-player",
            "block-break-event.block-break-event.get-block",
            "block-break-event.block-break-event.is-cancelled",
            "block.block.get-type",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_block_break_event_resource(test_block_break_event_facade())
            .unwrap();
        let player = crate::core_host::imports::HostBlockBreakEvent::block_break_event_get_player(
            &mut store, event,
        )
        .unwrap()
        .unwrap();
        let block = crate::core_host::imports::HostBlockBreakEvent::block_break_event_get_block(
            &mut store, event,
        )
        .unwrap()
        .unwrap();
        for (rep, kind) in [
            (event, ResourceKind::BlockBreakEvent),
            (player, ResourceKind::Player),
        ] {
            assert_eq!(
                store.resource_slot(rep, kind).unwrap().lifetime,
                ResourceLifetime::HostBorrowed
            );
        }
        assert_eq!(
            store
                .resource_slot(block, ResourceKind::Block)
                .unwrap()
                .lifetime,
            ResourceLifetime::GuestOwned
        );
        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        for (rep, kind) in [
            (event, ResourceKind::BlockBreakEvent),
            (player, ResourceKind::Player),
        ] {
            assert!(store.resource_slot(rep, kind).is_err());
        }
        assert!(store.resource_slot(block, ResourceKind::Block).is_ok());
        assert_eq!(
            crate::core_host::imports::HostBlock::block_get_type(&mut store, block),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );
    }

    #[test]
    fn block_cook_event_children_are_host_borrowed_and_expire() {
        let mut store = store_with_capabilities(&[
            "block-cook-event.block-cook-event.get-block",
            "block-cook-event.block-cook-event.get-source",
            "block-cook-event.block-cook-event.get-result",
            "block.block.get-type",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_block_cook_event_resource(test_block_cook_event_facade())
            .unwrap();
        let block = crate::core_host::imports::HostBlockCookEvent::block_cook_event_get_block(
            &mut store, event,
        )
        .unwrap()
        .unwrap();
        let source = crate::core_host::imports::HostBlockCookEvent::block_cook_event_get_source(
            &mut store, event,
        )
        .unwrap()
        .unwrap();
        let result = crate::core_host::imports::HostBlockCookEvent::block_cook_event_get_result(
            &mut store, event,
        )
        .unwrap()
        .unwrap();
        for (rep, kind) in [
            (event, ResourceKind::BlockCookEvent),
            (source, ResourceKind::ItemStackRef),
            (result, ResourceKind::ItemStackRef),
        ] {
            assert_eq!(
                store.resource_slot(rep, kind).unwrap().lifetime,
                ResourceLifetime::HostBorrowed
            );
        }
        assert_eq!(
            store
                .resource_slot(block, ResourceKind::Block)
                .unwrap()
                .lifetime,
            ResourceLifetime::GuestOwned
        );
        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        for (rep, kind) in [
            (event, ResourceKind::BlockCookEvent),
            (source, ResourceKind::ItemStackRef),
            (result, ResourceKind::ItemStackRef),
        ] {
            assert!(store.resource_slot(rep, kind).is_err());
        }
        assert!(store.resource_slot(block, ResourceKind::Block).is_ok());
        assert_eq!(
            crate::core_host::imports::HostBlock::block_get_type(&mut store, block),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );
    }

    #[test]
    fn block_callback_resources_apply_cancellation_and_expire_children() {
        let mut store = store_with_capabilities(&[
            "block-from-to-event.block-from-to-event.get-block",
            "block-grow-event.block-grow-event.get-block",
            "block-piston-extend-event.block-piston-extend-event.get-block",
            "leaves-decay-event.leaves-decay-event.get-block",
            "block-from-to-event.block-from-to-event.get-to-block",
            "block-piston-extend-event.block-piston-extend-event.get-direction",
            "block-from-to-event.block-from-to-event.is-cancelled",
            "block-grow-event.block-grow-event.is-cancelled",
            "block-piston-extend-event.block-piston-extend-event.is-cancelled",
            "leaves-decay-event.leaves-decay-event.is-cancelled",
            "block-from-to-event.block-from-to-event.set-cancelled",
            "block-grow-event.block-grow-event.set-cancelled",
            "block-piston-extend-event.block-piston-extend-event.set-cancelled",
            "leaves-decay-event.leaves-decay-event.set-cancelled",
            "block.block.get-type",
        ]);

        store.invocation_id = 42;
        let leaves = store
            .insert_leaves_decay_event_resource(test_leaves_decay_event_facade())
            .unwrap();
        let leaves_block =
            crate::core_host::imports::HostLeavesDecayEvent::leaves_decay_event_get_block(
                &mut store, leaves,
            )
            .unwrap()
            .unwrap();
        crate::core_host::imports::HostLeavesDecayEvent::leaves_decay_event_set_cancelled(
            &mut store, leaves, true,
        )
        .unwrap()
        .unwrap();

        let from_to = store
            .insert_block_from_to_event_resource(test_block_from_to_event_facade())
            .unwrap();
        let source =
            crate::core_host::imports::HostBlockFromToEvent::block_from_to_event_get_block(
                &mut store, from_to,
            )
            .unwrap()
            .unwrap();
        let destination =
            crate::core_host::imports::HostBlockFromToEvent::block_from_to_event_get_to_block(
                &mut store, from_to,
            )
            .unwrap()
            .unwrap();
        crate::core_host::imports::HostBlockFromToEvent::block_from_to_event_set_cancelled(
            &mut store, from_to, true,
        )
        .unwrap()
        .unwrap();

        let grow = store
            .insert_block_grow_event_resource(test_block_grow_event_facade())
            .unwrap();
        let grow_block = crate::core_host::imports::HostBlockGrowEvent::block_grow_event_get_block(
            &mut store, grow,
        )
        .unwrap()
        .unwrap();
        crate::core_host::imports::HostBlockGrowEvent::block_grow_event_set_cancelled(
            &mut store, grow, true,
        )
        .unwrap()
        .unwrap();

        let piston = store
            .insert_block_piston_event_resource(test_block_piston_event_facade())
            .unwrap();
        let piston_block =
            crate::core_host::imports::HostBlockPistonExtendEvent::block_piston_extend_event_get_block(
                &mut store,
                piston,
            )
            .unwrap()
            .unwrap();
        assert_eq!(
            crate::core_host::imports::HostBlockPistonExtendEvent::block_piston_extend_event_get_direction(
                &mut store,
                piston,
            )
            .unwrap()
            .unwrap(),
            crate::core_host::BlockFaceBlockFace::Up
        );
        crate::core_host::imports::HostBlockPistonExtendEvent::block_piston_extend_event_set_cancelled(
            &mut store,
            piston,
            true,
        )
        .unwrap()
        .unwrap();

        store.clear_invocation_resources(42);
        store.invocation_id = 43;
        for (rep, kind) in [
            (leaves, ResourceKind::LeavesDecayEvent),
            (from_to, ResourceKind::BlockFromToEvent),
            (grow, ResourceKind::BlockGrowEvent),
            (piston, ResourceKind::BlockPistonEvent),
        ] {
            assert!(store.resource_slot(rep, kind).is_err());
        }
        for block in [leaves_block, source, destination, grow_block, piston_block] {
            assert!(store.resource_slot(block, ResourceKind::Block).is_ok());
            assert_eq!(
                crate::core_host::imports::HostBlock::block_get_type(&mut store, block),
                Ok(Err(crate::core_host::TypesHostError::NotFound))
            );
        }
        assert_eq!(
            crate::core_host::imports::HostLeavesDecayEvent::leaves_decay_event_is_cancelled(
                &mut store, leaves,
            ),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );

        store.invocation_id = 44;
        let expiring = store
            .insert_block_grow_event_resource(test_block_grow_event_facade())
            .unwrap();
        crate::core_host::imports::HostBlockGrowEvent::block_grow_event_set_cancelled(
            &mut store, expiring, true,
        )
        .unwrap()
        .unwrap();
        store.clear_invocation_resources(44);
        assert_eq!(
            crate::core_host::imports::HostBlockGrowEvent::block_grow_event_is_cancelled(
                &mut store, expiring,
            ),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );
    }

    #[test]
    fn block_place_event_children_are_host_borrowed_and_expire() {
        let mut store = store_with_capabilities(&[
            "block-place-event.block-place-event.get-player",
            "block-place-event.block-place-event.get-block-replaced",
            "block-place-event.block-place-event.get-block-against",
            "block-place-event.block-place-event.is-cancelled",
            "block.block.get-type",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_block_place_event_resource(test_block_place_event_facade())
            .unwrap();
        let player = crate::core_host::imports::HostBlockPlaceEvent::block_place_event_get_player(
            &mut store, event,
        )
        .unwrap()
        .unwrap();
        let replaced =
            crate::core_host::imports::HostBlockPlaceEvent::block_place_event_get_block_replaced(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        let against =
            crate::core_host::imports::HostBlockPlaceEvent::block_place_event_get_block_against(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        for (rep, kind) in [
            (event, ResourceKind::BlockPlaceEvent),
            (player, ResourceKind::Player),
        ] {
            assert_eq!(
                store.resource_slot(rep, kind).unwrap().lifetime,
                ResourceLifetime::HostBorrowed
            );
        }
        for block in [replaced, against] {
            assert_eq!(
                store
                    .resource_slot(block, ResourceKind::Block)
                    .unwrap()
                    .lifetime,
                ResourceLifetime::GuestOwned
            );
        }
        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        for (rep, kind) in [
            (event, ResourceKind::BlockPlaceEvent),
            (player, ResourceKind::Player),
        ] {
            assert!(store.resource_slot(rep, kind).is_err());
        }
        for block in [replaced, against] {
            assert!(store.resource_slot(block, ResourceKind::Block).is_ok());
            assert_eq!(
                crate::core_host::imports::HostBlock::block_get_type(&mut store, block),
                Ok(Err(crate::core_host::TypesHostError::NotFound))
            );
        }
    }

    #[test]
    fn player_item_consume_event_children_are_host_borrowed_and_expire() {
        let mut store = store_with_capabilities(&[
            "player-item-consume-event.player-item-consume-event.get-player",
            "player-item-consume-event.player-item-consume-event.get-item",
            "player-item-consume-event.player-item-consume-event.get-hand",
            "player-item-consume-event.player-item-consume-event.is-cancelled",
            "item-stack.item-stack-ref.get-type-id",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_player_item_consume_event_resource(test_player_item_consume_event_facade())
            .unwrap();
        let player = crate::core_host::imports::HostPlayerItemConsumeEvent::player_item_consume_event_get_player(
            &mut store,
            event,
        )
        .unwrap()
        .unwrap();
        let item = crate::core_host::imports::HostPlayerItemConsumeEvent::player_item_consume_event_get_item(
            &mut store,
            event,
        )
        .unwrap()
        .unwrap();

        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerItemConsumeEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            store
                .resource_slot(player, ResourceKind::Player)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            store
                .resource_slot(item, ResourceKind::ItemStackRef)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            crate::core_host::imports::HostPlayerItemConsumeEvent::player_item_consume_event_get_hand(
                &mut store,
                event,
            ),
            Ok(Ok(crate::core_host::EquipmentSlotEquipmentSlot::Hand))
        );

        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        assert!(
            store
                .resource_slot(event, ResourceKind::PlayerItemConsumeEvent)
                .is_err()
        );
        assert!(store.resource_slot(player, ResourceKind::Player).is_err());
        assert!(
            store
                .resource_slot(item, ResourceKind::ItemStackRef)
                .is_err()
        );
        assert_eq!(
            crate::core_host::imports::HostItemStack::item_stack_ref_get_type_id(&mut store, item,),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );
    }

    #[test]
    fn broadcast_message_event_resource_slots_are_invocation_scoped() {
        let mut store = store();
        store.invocation_id = 42;
        let token = store
            .resources
            .insert_value(
                ResourceSlot {
                    invocation_id: store.invocation_id,
                    handle: 1,
                    lifetime: ResourceLifetime::HostBorrowed,
                },
                ResourceKind::BroadcastMessageEvent as u32,
                RESOURCE_OWNER,
                ResourceLifetime::HostBorrowed,
                store.invocation_id,
            )
            .unwrap();
        store.resource_slot_count = 1;
        store
            .host_borrowed_slots
            .push((store.invocation_id, token.rep()));
        let event = token.rep();

        assert_eq!(
            store
                .resource_slot(event, ResourceKind::BroadcastMessageEvent)
                .unwrap()
                .handle,
            1
        );
        assert!(store.resource_slot(event, ResourceKind::Player).is_err());

        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        assert!(
            store
                .resource_slot(event, ResourceKind::BroadcastMessageEvent)
                .is_err()
        );
    }

    #[test]
    fn weather_change_event_resource_slots_are_invocation_scoped() {
        let mut store = store();
        store.invocation_id = 42;
        let token = store
            .resources
            .insert_value(
                ResourceSlot {
                    invocation_id: store.invocation_id,
                    handle: 1,
                    lifetime: ResourceLifetime::HostBorrowed,
                },
                ResourceKind::WeatherChangeEvent as u32,
                RESOURCE_OWNER,
                ResourceLifetime::HostBorrowed,
                store.invocation_id,
            )
            .unwrap();
        store.resource_slot_count = 1;
        store
            .host_borrowed_slots
            .push((store.invocation_id, token.rep()));
        let event = token.rep();

        assert_eq!(
            store
                .resource_slot(event, ResourceKind::WeatherChangeEvent)
                .unwrap()
                .handle,
            1
        );
        assert!(store.resource_slot(event, ResourceKind::Player).is_err());

        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        assert!(
            store
                .resource_slot(event, ResourceKind::WeatherChangeEvent)
                .is_err()
        );
    }

    #[test]
    fn thunder_change_event_resource_slots_are_invocation_scoped() {
        let mut store = store();
        store.invocation_id = 42;
        let token = store
            .resources
            .insert_value(
                ResourceSlot {
                    invocation_id: store.invocation_id,
                    handle: 1,
                    lifetime: ResourceLifetime::HostBorrowed,
                },
                ResourceKind::ThunderChangeEvent as u32,
                RESOURCE_OWNER,
                ResourceLifetime::HostBorrowed,
                store.invocation_id,
            )
            .unwrap();
        store.resource_slot_count = 1;
        store
            .host_borrowed_slots
            .push((store.invocation_id, token.rep()));
        let event = token.rep();

        assert_eq!(
            store
                .resource_slot(event, ResourceKind::ThunderChangeEvent)
                .unwrap()
                .handle,
            1
        );
        assert!(store.resource_slot(event, ResourceKind::Player).is_err());

        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        assert!(
            store
                .resource_slot(event, ResourceKind::ThunderChangeEvent)
                .is_err()
        );
    }

    #[test]
    fn player_kick_event_resource_slots_are_invocation_scoped() {
        let mut store = store();
        store.invocation_id = 42;
        let token = store
            .resources
            .insert_value(
                ResourceSlot {
                    invocation_id: store.invocation_id,
                    handle: 1,
                    lifetime: ResourceLifetime::HostBorrowed,
                },
                ResourceKind::PlayerKickEvent as u32,
                RESOURCE_OWNER,
                ResourceLifetime::HostBorrowed,
                store.invocation_id,
            )
            .unwrap();
        store.resource_slot_count = 1;
        store
            .host_borrowed_slots
            .push((store.invocation_id, token.rep()));
        let event = token.rep();

        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerKickEvent)
                .unwrap()
                .handle,
            1
        );
        assert!(store.resource_slot(event, ResourceKind::Player).is_err());

        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        assert!(
            store
                .resource_slot(event, ResourceKind::PlayerKickEvent)
                .is_err()
        );
    }

    #[test]
    fn player_login_event_resource_slots_are_invocation_scoped() {
        let mut store = store();
        store.invocation_id = 42;
        let token = store
            .resources
            .insert_value(
                ResourceSlot {
                    invocation_id: store.invocation_id,
                    handle: 1,
                    lifetime: ResourceLifetime::HostBorrowed,
                },
                ResourceKind::PlayerLoginEvent as u32,
                RESOURCE_OWNER,
                ResourceLifetime::HostBorrowed,
                store.invocation_id,
            )
            .unwrap();
        store.resource_slot_count = 1;
        store
            .host_borrowed_slots
            .push((store.invocation_id, token.rep()));
        let event = token.rep();

        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerLoginEvent)
                .unwrap()
                .handle,
            1
        );
        assert!(store.resource_slot(event, ResourceKind::Player).is_err());

        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        assert!(
            store
                .resource_slot(event, ResourceKind::PlayerLoginEvent)
                .is_err()
        );
    }

    #[test]
    fn player_command_event_resource_slots_are_invocation_scoped() {
        let mut store = store();
        store.invocation_id = 42;
        let token = store
            .resources
            .insert_value(
                ResourceSlot {
                    invocation_id: store.invocation_id,
                    handle: 1,
                    lifetime: ResourceLifetime::HostBorrowed,
                },
                ResourceKind::PlayerCommandEvent as u32,
                RESOURCE_OWNER,
                ResourceLifetime::HostBorrowed,
                store.invocation_id,
            )
            .unwrap();
        store.resource_slot_count = 1;
        store
            .host_borrowed_slots
            .push((store.invocation_id, token.rep()));
        let event = token.rep();

        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerCommandEvent)
                .unwrap()
                .handle,
            1
        );
        assert!(store.resource_slot(event, ResourceKind::Player).is_err());

        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        assert!(
            store
                .resource_slot(event, ResourceKind::PlayerCommandEvent)
                .is_err()
        );
    }

    #[test]
    fn server_command_event_resource_slots_are_invocation_scoped() {
        let mut store = store();
        store.invocation_id = 42;
        let token = store
            .resources
            .insert_value(
                ResourceSlot {
                    invocation_id: store.invocation_id,
                    handle: 1,
                    lifetime: ResourceLifetime::HostBorrowed,
                },
                ResourceKind::ServerCommandEvent as u32,
                RESOURCE_OWNER,
                ResourceLifetime::HostBorrowed,
                store.invocation_id,
            )
            .unwrap();
        store.resource_slot_count = 1;
        store
            .host_borrowed_slots
            .push((store.invocation_id, token.rep()));
        let event = token.rep();

        assert_eq!(
            store
                .resource_slot(event, ResourceKind::ServerCommandEvent)
                .unwrap()
                .handle,
            1
        );
        assert!(store.resource_slot(event, ResourceKind::Player).is_err());

        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        assert!(
            store
                .resource_slot(event, ResourceKind::ServerCommandEvent)
                .is_err()
        );
    }

    #[test]
    fn server_list_ping_event_resource_slots_are_invocation_scoped() {
        let mut store = store();
        store.invocation_id = 42;
        let token = store
            .resources
            .insert_value(
                ResourceSlot {
                    invocation_id: store.invocation_id,
                    handle: 1,
                    lifetime: ResourceLifetime::HostBorrowed,
                },
                ResourceKind::ServerListPingEvent as u32,
                RESOURCE_OWNER,
                ResourceLifetime::HostBorrowed,
                store.invocation_id,
            )
            .unwrap();
        store.resource_slot_count = 1;
        store
            .host_borrowed_slots
            .push((store.invocation_id, token.rep()));
        let event = token.rep();

        assert_eq!(
            store
                .resource_slot(event, ResourceKind::ServerListPingEvent)
                .unwrap()
                .handle,
            1
        );
        assert!(store.resource_slot(event, ResourceKind::Player).is_err());

        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        assert!(
            store
                .resource_slot(event, ResourceKind::ServerListPingEvent)
                .is_err()
        );
    }

    #[test]
    fn server_list_ping_event_mutates_immediately_and_expires() {
        let mut store = store_with_capabilities(&[
            "server-list-ping-event.server-list-ping-event.get-motd",
            "server-list-ping-event.server-list-ping-event.set-motd",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_server_list_ping_event_resource(test_server_list_ping_event_facade())
            .unwrap();
        crate::core_host::imports::HostServerListPingEvent::server_list_ping_event_set_motd(
            &mut store,
            event,
            String::new(),
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostServerListPingEvent::server_list_ping_event_get_motd(
                &mut store, event,
            ),
            Ok(Ok(String::new()))
        );
        store.clear_invocation_resources(store.invocation_id);

        store.invocation_id = 43;
        let event = store
            .insert_server_list_ping_event_resource(test_server_list_ping_event_facade())
            .unwrap();
        crate::core_host::imports::HostServerListPingEvent::server_list_ping_event_set_motd(
            &mut store,
            event,
            "discarded".to_owned(),
        )
        .unwrap()
        .unwrap();
        // Cleanup invalidates the callback-scoped facade after a direct setter call.
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(event, ResourceKind::ServerListPingEvent)
                .is_err()
        );
    }
    #[test]
    fn player_pickup_item_event_cancellation_mutates_immediately() {
        let mut store = store_with_capabilities(&[
            "player-pickup-item-event.player-pickup-item-event.is-cancelled",
            "player-pickup-item-event.player-pickup-item-event.set-cancelled",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_player_pickup_item_event_resource(test_player_pickup_item_event_facade())
            .unwrap();
        crate::core_host::imports::HostPlayerPickupItemEvent::player_pickup_item_event_set_cancelled(
            &mut store,
            event,
            true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostPlayerPickupItemEvent::player_pickup_item_event_is_cancelled(
                &mut store,
                event,
            ),
            Ok(Ok(true))
        );
        store.clear_invocation_resources(store.invocation_id);

        store.invocation_id = 43;
        let event = store
            .insert_player_pickup_item_event_resource(test_player_pickup_item_event_facade())
            .unwrap();
        crate::core_host::imports::HostPlayerPickupItemEvent::player_pickup_item_event_set_cancelled(
            &mut store,
            event,
            true,
        )
        .unwrap()
        .unwrap();
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(event, ResourceKind::PlayerPickupItemEvent)
                .is_err()
        );
    }

    #[test]
    fn player_interact_event_cancellation_mutates_immediately() {
        let mut store = store_with_capabilities(&[
            "player-interact-event.player-interact-event.is-cancelled",
            "player-interact-event.player-interact-event.set-cancelled",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_player_interact_event_resource(test_player_interact_event_facade())
            .unwrap();
        crate::core_host::imports::HostPlayerInteractEvent::player_interact_event_set_cancelled(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostPlayerInteractEvent::player_interact_event_is_cancelled(
                &mut store, event,
            ),
            Ok(Ok(true))
        );
        store.clear_invocation_resources(store.invocation_id);

        store.invocation_id = 43;
        let expiring = store
            .insert_player_interact_event_resource(test_player_interact_event_facade())
            .unwrap();
        crate::core_host::imports::HostPlayerInteractEvent::player_interact_event_set_cancelled(
            &mut store, expiring, true,
        )
        .unwrap()
        .unwrap();
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(expiring, ResourceKind::PlayerInteractEvent)
                .is_err()
        );
    }

    #[test]
    fn player_interact_actor_event_mutates_immediately_and_expires() {
        let mut store = store_with_capabilities(&[
            "player-interact-actor-event.player-interact-actor-event.is-cancelled",
            "player-interact-actor-event.player-interact-actor-event.set-cancelled",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_player_interact_actor_event_resource(test_player_interact_actor_event_facade())
            .unwrap();
        crate::core_host::imports::HostPlayerInteractActorEvent::player_interact_actor_event_set_cancelled(
            &mut store,
            event,
            true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostPlayerInteractActorEvent::player_interact_actor_event_is_cancelled(
                &mut store,
                event,
            ),
            Ok(Ok(true))
        );
        store.clear_invocation_resources(store.invocation_id);

        store.invocation_id = 43;
        let expiring = store
            .insert_player_interact_actor_event_resource(test_player_interact_actor_event_facade())
            .unwrap();
        crate::core_host::imports::HostPlayerInteractActorEvent::player_interact_actor_event_set_cancelled(
            &mut store,
            expiring,
            true,
        )
        .unwrap()
        .unwrap();
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(expiring, ResourceKind::PlayerInteractActorEvent)
                .is_err()
        );
    }

    #[test]
    fn actor_damage_event_mutates_immediately_and_expires() {
        let mut store = store_with_capabilities(&[
            "actor-damage-event.actor-damage-event.get-damage",
            "actor-damage-event.actor-damage-event.set-damage",
            "actor-damage-event.actor-damage-event.is-cancelled",
            "actor-damage-event.actor-damage-event.set-cancelled",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_actor_damage_event_resource(test_actor_damage_event_facade())
            .unwrap();
        assert_eq!(
            crate::core_host::imports::HostActorDamageEvent::actor_damage_event_get_damage(
                &mut store, event,
            ),
            Ok(Ok(2.5))
        );
        crate::core_host::imports::HostActorDamageEvent::actor_damage_event_set_damage(
            &mut store, event, 7.5,
        )
        .unwrap()
        .unwrap();
        crate::core_host::imports::HostActorDamageEvent::actor_damage_event_set_cancelled(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostActorDamageEvent::actor_damage_event_get_damage(
                &mut store, event,
            ),
            Ok(Ok(7.5))
        );
        assert_eq!(
            crate::core_host::imports::HostActorDamageEvent::actor_damage_event_is_cancelled(
                &mut store, event,
            ),
            Ok(Ok(true))
        );
        store.clear_invocation_resources(store.invocation_id);
    }

    #[test]
    fn actor_explode_event_cancellation_mutates_immediately() {
        let mut store = store_with_capabilities(&[
            "actor-explode-event.actor-explode-event.is-cancelled",
            "actor-explode-event.actor-explode-event.set-cancelled",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_actor_explode_event_resource(test_actor_explode_event_facade())
            .unwrap();
        assert_eq!(
            crate::core_host::imports::HostActorExplodeEvent::actor_explode_event_is_cancelled(
                &mut store, event,
            ),
            Ok(Ok(false))
        );
        crate::core_host::imports::HostActorExplodeEvent::actor_explode_event_set_cancelled(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostActorExplodeEvent::actor_explode_event_is_cancelled(
                &mut store, event,
            ),
            Ok(Ok(true))
        );
        store.clear_invocation_resources(store.invocation_id);

        for invocation_id in [43, 44] {
            store.invocation_id = invocation_id;
            let event = store
                .insert_actor_explode_event_resource(test_actor_explode_event_facade())
                .unwrap();
            crate::core_host::imports::HostActorExplodeEvent::actor_explode_event_set_cancelled(
                &mut store, event, true,
            )
            .unwrap()
            .unwrap();
            store.clear_invocation_resources(invocation_id);
            assert!(
                store
                    .resource_slot(event, ResourceKind::ActorExplodeEvent)
                    .is_err()
            );
        }
    }

    #[test]
    fn actor_explode_event_returns_owned_blocks_and_expires_event_and_actor() {
        let mut store = store_with_capabilities(&[
            "actor.actor.get-actor-name",
            "actor-explode-event.actor-explode-event.get-actor",
            "actor-explode-event.actor-explode-event.get-block-list",
            "actor-explode-event.actor-explode-event.get-location",
            "block.block.get-type",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_actor_explode_event_resource(test_actor_explode_event_facade())
            .unwrap();
        let actor =
            crate::core_host::imports::HostActorExplodeEvent::actor_explode_event_get_actor(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        for (rep, kind) in [
            (event, ResourceKind::ActorExplodeEvent),
            (actor, ResourceKind::Actor),
        ] {
            assert_eq!(
                store.resource_slot(rep, kind).unwrap().lifetime,
                ResourceLifetime::HostBorrowed
            );
        }
        let location =
            crate::core_host::imports::HostActorExplodeEvent::actor_explode_event_get_location(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        assert_eq!(
            (
                location.dimension.as_str(),
                location.x,
                location.y,
                location.z,
                location.pitch,
                location.yaw,
            ),
            ("test", 1.0, 2.0, 3.0, 4.0, 5.0)
        );
        let blocks =
            crate::core_host::imports::HostActorExplodeEvent::actor_explode_event_get_block_list(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(
            store
                .resource_slot(blocks[0], ResourceKind::Block)
                .unwrap()
                .lifetime,
            ResourceLifetime::GuestOwned
        );
        assert_eq!(
            crate::core_host::imports::HostBlock::block_get_type(&mut store, blocks[0]),
            Ok(Ok("minecraft:stone".to_owned()))
        );

        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        for (rep, kind) in [
            (event, ResourceKind::ActorExplodeEvent),
            (actor, ResourceKind::Actor),
        ] {
            assert!(store.resource_slot(rep, kind).is_err());
        }
        assert!(matches!(
            crate::core_host::imports::HostActorExplodeEvent::actor_explode_event_get_location(
                &mut store, event,
            ),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        ));
        assert_eq!(
            crate::core_host::imports::HostActor::actor_get_actor_name(&mut store, actor),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );
        assert!(store.resource_slot(blocks[0], ResourceKind::Block).is_ok());
        assert_eq!(
            crate::core_host::imports::HostBlock::block_get_type(&mut store, blocks[0]),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );
    }

    #[test]
    fn block_explode_event_returns_owned_blocks_and_mutates_immediately() {
        let mut store = store_with_capabilities(&[
            "block-explode-event.block-explode-event.get-block",
            "block-explode-event.block-explode-event.get-block-list",
            "block-explode-event.block-explode-event.is-cancelled",
            "block-explode-event.block-explode-event.set-cancelled",
            "block.block.get-type",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_block_explode_event_resource(test_block_explode_event_facade())
            .unwrap();
        let block =
            crate::core_host::imports::HostBlockExplodeEvent::block_explode_event_get_block(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::BlockExplodeEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            store
                .resource_slot(block, ResourceKind::Block)
                .unwrap()
                .lifetime,
            ResourceLifetime::GuestOwned
        );
        assert_eq!(
            crate::core_host::imports::HostBlock::block_get_type(&mut store, block),
            Ok(Ok("minecraft:stone".to_owned()))
        );
        let blocks =
            crate::core_host::imports::HostBlockExplodeEvent::block_explode_event_get_block_list(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(
            store
                .resource_slot(blocks[0], ResourceKind::Block)
                .unwrap()
                .lifetime,
            ResourceLifetime::GuestOwned
        );
        assert_eq!(
            crate::core_host::imports::HostBlock::block_get_type(&mut store, blocks[0]),
            Ok(Ok("minecraft:stone".to_owned()))
        );
        assert_eq!(
            crate::core_host::imports::HostBlockExplodeEvent::block_explode_event_is_cancelled(
                &mut store, event,
            ),
            Ok(Ok(false))
        );
        crate::core_host::imports::HostBlockExplodeEvent::block_explode_event_set_cancelled(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();

        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        assert!(
            store
                .resource_slot(event, ResourceKind::BlockExplodeEvent)
                .is_err()
        );
        assert!(store.resource_slot(block, ResourceKind::Block).is_ok());
        assert!(store.resource_slot(blocks[0], ResourceKind::Block).is_ok());
        assert!(matches!(
            crate::core_host::imports::HostBlockExplodeEvent::block_explode_event_get_block_list(
                &mut store, event,
            ),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        ));
        assert_eq!(
            crate::core_host::imports::HostBlock::block_get_type(&mut store, block),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );
        assert_eq!(
            crate::core_host::imports::HostBlock::block_get_type(&mut store, blocks[0]),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );

        for invocation_id in [44, 45, 46, 47] {
            store.invocation_id = invocation_id;
            let expiring = store
                .insert_block_explode_event_resource(test_block_explode_event_facade())
                .unwrap();
            crate::core_host::imports::HostBlockExplodeEvent::block_explode_event_set_cancelled(
                &mut store, expiring, true,
            )
            .unwrap()
            .unwrap();
            store.clear_invocation_resources(invocation_id);
            assert!(
                store
                    .resource_slot(expiring, ResourceKind::BlockExplodeEvent)
                    .is_err()
            );
        }

        store.invocation_id = 48;
        store.config.max_plugin_resource_slots = 1;
        store.resource_slot_count = 1;
        assert!(
            store
                .insert_block_explode_event_resource(test_block_explode_event_facade())
                .is_err()
        );
        store.resource_slot_count = 0;
        store.config.max_plugin_resource_slots = 0;
    }

    #[test]
    fn actor_knockback_event_mutates_immediately_and_expires_all_children() {
        let mut store = store_with_capabilities(&[
            "actor.actor.get-actor-name",
            "actor-knockback-event.actor-knockback-event.get-actor",
            "actor-knockback-event.actor-knockback-event.get-source",
            "actor-knockback-event.actor-knockback-event.get-knockback",
            "actor-knockback-event.actor-knockback-event.set-knockback",
            "actor-knockback-event.actor-knockback-event.is-cancelled",
            "actor-knockback-event.actor-knockback-event.set-cancelled",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_actor_knockback_event_resource(test_actor_knockback_event_facade(true))
            .unwrap();
        let actor =
            crate::core_host::imports::HostActorKnockbackEvent::actor_knockback_event_get_actor(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        let source =
            crate::core_host::imports::HostActorKnockbackEvent::actor_knockback_event_get_source(
                &mut store, event,
            )
            .unwrap()
            .unwrap()
            .unwrap();
        for (rep, kind) in [
            (event, ResourceKind::ActorKnockbackEvent),
            (actor, ResourceKind::Actor),
            (source, ResourceKind::Actor),
        ] {
            assert_eq!(
                store.resource_slot(rep, kind).unwrap().lifetime,
                ResourceLifetime::HostBorrowed
            );
        }
        let vector = crate::core_host::imports::HostActorKnockbackEvent::actor_knockback_event_get_knockback(
            &mut store,
            event,
        )
        .unwrap()
        .unwrap();
        assert_eq!((vector.x, vector.y, vector.z), (1.0, 2.0, 3.0));
        crate::core_host::imports::HostActorKnockbackEvent::actor_knockback_event_set_knockback(
            &mut store,
            event,
            crate::core_host::VectorVector {
                x: 7.0,
                y: 8.0,
                z: 9.0,
            },
        )
        .unwrap()
        .unwrap();
        crate::core_host::imports::HostActorKnockbackEvent::actor_knockback_event_set_cancelled(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();

        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        for (rep, kind) in [
            (event, ResourceKind::ActorKnockbackEvent),
            (actor, ResourceKind::Actor),
            (source, ResourceKind::Actor),
        ] {
            assert!(store.resource_slot(rep, kind).is_err());
        }
        assert!(matches!(
            crate::core_host::imports::HostActorKnockbackEvent::actor_knockback_event_get_knockback(
                &mut store, event,
            ),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        ));
        for child in [actor, source] {
            assert_eq!(
                crate::core_host::imports::HostActor::actor_get_actor_name(&mut store, child),
                Ok(Err(crate::core_host::TypesHostError::NotFound))
            );
        }

        store.invocation_id = 44;
        let no_source = store
            .insert_actor_knockback_event_resource(test_actor_knockback_event_facade(false))
            .unwrap();
        assert!(matches!(
            crate::core_host::imports::HostActorKnockbackEvent::actor_knockback_event_get_source(
                &mut store, no_source,
            ),
            Ok(Ok(None))
        ));
        store.clear_invocation_resources(store.invocation_id);

        // Cleanup invalidates the callback-scoped facade after direct setter calls.
        for invocation_id in [45, 46, 47, 48] {
            store.invocation_id = invocation_id;
            let expiring = store
                .insert_actor_knockback_event_resource(test_actor_knockback_event_facade(true))
                .unwrap();
            crate::core_host::imports::HostActorKnockbackEvent::actor_knockback_event_set_knockback(
                &mut store,
                expiring,
                crate::core_host::VectorVector {
                    x: 99.0,
                    y: 0.0,
                    z: 0.0,
                },
            )
            .unwrap()
            .unwrap();
            crate::core_host::imports::HostActorKnockbackEvent::actor_knockback_event_set_cancelled(
                &mut store,
                expiring,
                true,
            )
            .unwrap()
            .unwrap();
            store.clear_invocation_resources(invocation_id);
            assert!(
                store
                    .resource_slot(expiring, ResourceKind::ActorKnockbackEvent)
                    .is_err()
            );
        }

        store.invocation_id = 49;
        store.config.max_plugin_resource_slots = 1;
        store.resource_slot_count = 1;
        assert!(
            store
                .insert_actor_knockback_event_resource(test_actor_knockback_event_facade(true))
                .is_err()
        );
        store.resource_slot_count = 0;
        store.config.max_plugin_resource_slots = 0;
    }

    #[test]
    fn actor_death_event_is_borrowed_and_expires_after_callback() {
        let mut store = store_with_capabilities(&["actor-death-event.actor-death-event.get-actor"]);
        store.invocation_id = 42;

        let event = store
            .insert_actor_death_event_resource(test_actor_death_event_facade())
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::ActorDeathEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        let actor = crate::core_host::imports::HostActorDeathEvent::actor_death_event_get_actor(
            &mut store, event,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            store
                .resource_slot(actor, ResourceKind::Actor)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(event, ResourceKind::ActorDeathEvent)
                .is_err()
        );
        assert!(store.resource_slot(actor, ResourceKind::Actor).is_err());
        // Clearing an invocation invalidates a terminal event facade.
        for invocation_id in [43, 44, 45, 46] {
            store.invocation_id = invocation_id;
            let expiring = store
                .insert_actor_death_event_resource(test_actor_death_event_facade())
                .unwrap();
            store.clear_invocation_resources(invocation_id);
            assert!(
                store
                    .resource_slot(expiring, ResourceKind::ActorDeathEvent)
                    .is_err()
            );
        }
    }

    #[test]
    fn copied_lifecycle_resources_expire_after_callback() {
        let mut store = store_with_capabilities(&[
            "plugin-enable-event.plugin-enable-event.get-plugin-name",
            "server-load-event.server-load-event.get-load-type",
        ]);
        store.invocation_id = 42;

        let lifecycle = store
            .insert_plugin_lifecycle_event_resource(test_plugin_lifecycle_event_facade())
            .unwrap();
        assert_eq!(
            store
                .resource_slot(lifecycle, ResourceKind::PluginLifecycleEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            crate::core_host::imports::HostPluginEnableEvent::plugin_enable_event_get_plugin_name(
                &mut store, lifecycle,
            ),
            Ok(Ok("example-plugin".to_owned()))
        );

        let server_load = store
            .insert_server_load_event_resource(test_server_load_event_facade())
            .unwrap();
        assert_eq!(
            crate::core_host::imports::HostServerLoadEvent::server_load_event_get_load_type(
                &mut store,
                server_load,
            ),
            Ok(Ok(crate::core_host::ServerLoadEventServerLoadType::Reload))
        );

        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(lifecycle, ResourceKind::PluginLifecycleEvent)
                .is_err()
        );
        assert_eq!(
            crate::core_host::imports::HostPluginEnableEvent::plugin_enable_event_get_plugin_name(
                &mut store, lifecycle,
            ),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );
        assert!(
            store
                .resource_slot(server_load, ResourceKind::ServerLoadEvent)
                .is_err()
        );
        assert_eq!(
            crate::core_host::imports::HostServerLoadEvent::server_load_event_get_load_type(
                &mut store,
                server_load,
            ),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );

        // Clearing callbacks invalidates their terminal facades.
        for invocation_id in [43, 44, 45] {
            store.invocation_id = invocation_id;
            let expiring = store
                .insert_plugin_lifecycle_event_resource(test_plugin_lifecycle_event_facade())
                .unwrap();
            store.clear_invocation_resources(invocation_id);
            assert!(
                store
                    .resource_slot(expiring, ResourceKind::PluginLifecycleEvent)
                    .is_err()
            );
        }
        for invocation_id in [46, 47, 48] {
            store.invocation_id = invocation_id;
            let expiring = store
                .insert_server_load_event_resource(test_server_load_event_facade())
                .unwrap();
            store.clear_invocation_resources(invocation_id);
            assert!(
                store
                    .resource_slot(expiring, ResourceKind::ServerLoadEvent)
                    .is_err()
            );
        }
    }

    #[test]
    fn chunk_event_is_borrowed_and_expires_after_callback() {
        let mut store = store_with_capabilities(&[
            "chunk-load-event.chunk-load-event.get-chunk-x",
            "chunk-load-event.chunk-load-event.get-chunk-z",
            "chunk-load-event.chunk-load-event.get-dimension",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_chunk_event_resource(test_chunk_event_facade())
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::ChunkEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            crate::core_host::imports::HostChunkLoadEvent::chunk_load_event_get_chunk_x(
                &mut store, event,
            ),
            Ok(Ok(3))
        );
        assert_eq!(
            crate::core_host::imports::HostChunkLoadEvent::chunk_load_event_get_chunk_z(
                &mut store, event,
            ),
            Ok(Ok(-7))
        );
        assert_eq!(
            crate::core_host::imports::HostChunkLoadEvent::chunk_load_event_get_dimension(
                &mut store, event,
            ),
            Ok(Ok("overworld".to_owned()))
        );

        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(event, ResourceKind::ChunkEvent)
                .is_err()
        );
        assert_eq!(
            crate::core_host::imports::HostChunkLoadEvent::chunk_load_event_get_dimension(
                &mut store, event,
            ),
            Ok(Err(crate::core_host::TypesHostError::NotFound))
        );

        // Clearing callbacks invalidates their terminal facades.
        for invocation_id in [43, 44, 45] {
            store.invocation_id = invocation_id;
            let expiring = store
                .insert_chunk_event_resource(test_chunk_event_facade())
                .unwrap();
            store.clear_invocation_resources(invocation_id);
            assert!(
                store
                    .resource_slot(expiring, ResourceKind::ChunkEvent)
                    .is_err()
            );
        }
    }

    #[test]
    fn copied_lifecycle_resources_require_read_capabilities() {
        let mut store = store();
        store.invocation_id = 42;
        let lifecycle = store
            .insert_plugin_lifecycle_event_resource(test_plugin_lifecycle_event_facade())
            .unwrap();
        let server_load = store
            .insert_server_load_event_resource(test_server_load_event_facade())
            .unwrap();

        assert_eq!(
            crate::core_host::imports::HostPluginEnableEvent::plugin_enable_event_get_plugin_name(
                &mut store, lifecycle
            )
            .unwrap(),
            Err(crate::core_host::TypesHostError::Denied)
        );
        assert_eq!(
            crate::core_host::imports::HostServerLoadEvent::server_load_event_get_load_type(
                &mut store,
                server_load
            )
            .unwrap(),
            Err(crate::core_host::TypesHostError::Denied)
        );
    }

    #[test]
    fn actor_remove_event_children_expire_after_callback() {
        let mut store = store_with_capabilities(&[
            "actor.actor.get-actor-name",
            "actor-remove-event.actor-remove-event.get-actor",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_actor_remove_event_resource(test_actor_remove_event_facade())
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::ActorRemoveEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        let actor = crate::core_host::imports::HostActorRemoveEvent::actor_remove_event_get_actor(
            &mut store, event,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            store
                .resource_slot(actor, ResourceKind::Actor)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(event, ResourceKind::ActorRemoveEvent)
                .is_err()
        );
        assert!(store.resource_slot(actor, ResourceKind::Actor).is_err());
        assert!(matches!(
            crate::core_host::imports::HostActorRemoveEvent::actor_remove_event_get_actor(
                &mut store, event
            )
            .unwrap(),
            Err(crate::core_host::TypesHostError::NotFound)
        ));
        assert_eq!(
            crate::core_host::imports::HostActor::actor_get_actor_name(&mut store, actor).unwrap(),
            Err(crate::core_host::TypesHostError::NotFound)
        );
    }

    #[test]
    fn actor_spawn_event_cancellation_mutates_immediately_and_expires_children() {
        let mut store = store_with_capabilities(&[
            "actor.actor.get-actor-name",
            "actor-spawn-event.actor-spawn-event.get-actor",
            "actor-spawn-event.actor-spawn-event.is-cancelled",
            "actor-spawn-event.actor-spawn-event.set-cancelled",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_actor_spawn_event_resource(test_actor_spawn_event_facade())
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::ActorSpawnEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        let actor = crate::core_host::imports::HostActorSpawnEvent::actor_spawn_event_get_actor(
            &mut store, event,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            store
                .resource_slot(actor, ResourceKind::Actor)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            crate::core_host::imports::HostActorSpawnEvent::actor_spawn_event_is_cancelled(
                &mut store, event
            )
            .unwrap(),
            Ok(false)
        );
        crate::core_host::imports::HostActorSpawnEvent::actor_spawn_event_set_cancelled(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostActorSpawnEvent::actor_spawn_event_is_cancelled(
                &mut store, event
            )
            .unwrap(),
            Ok(true)
        );
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(event, ResourceKind::ActorSpawnEvent)
                .is_err()
        );
        assert!(store.resource_slot(actor, ResourceKind::Actor).is_err());
        assert_eq!(
            crate::core_host::imports::HostActorSpawnEvent::actor_spawn_event_is_cancelled(
                &mut store, event
            )
            .unwrap(),
            Err(crate::core_host::TypesHostError::NotFound)
        );
        assert_eq!(
            crate::core_host::imports::HostActor::actor_get_actor_name(&mut store, actor).unwrap(),
            Err(crate::core_host::TypesHostError::NotFound)
        );

        // Cleanup invalidates the facade after a direct cancellation write.
        for invocation_id in [43, 44, 45] {
            store.invocation_id = invocation_id;
            let expiring = store
                .insert_actor_spawn_event_resource(test_actor_spawn_event_facade())
                .unwrap();
            crate::core_host::imports::HostActorSpawnEvent::actor_spawn_event_set_cancelled(
                &mut store, expiring, true,
            )
            .unwrap()
            .unwrap();
            store.clear_invocation_resources(invocation_id);
            assert!(
                store
                    .resource_slot(expiring, ResourceKind::ActorSpawnEvent)
                    .is_err()
            );
        }
    }

    #[test]
    fn actor_teleport_event_mutates_locations_and_expires_children() {
        let mut store = store_with_capabilities(&[
            "actor.actor.get-actor-name",
            "actor-teleport-event.actor-teleport-event.get-actor",
            "actor-teleport-event.actor-teleport-event.get-from",
            "actor-teleport-event.actor-teleport-event.set-from",
            "actor-teleport-event.actor-teleport-event.get-to",
            "actor-teleport-event.actor-teleport-event.set-to",
            "actor-teleport-event.actor-teleport-event.is-cancelled",
            "actor-teleport-event.actor-teleport-event.set-cancelled",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_actor_teleport_event_resource(test_actor_teleport_event_facade())
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::ActorTeleportEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        let actor =
            crate::core_host::imports::HostActorTeleportEvent::actor_teleport_event_get_actor(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        assert_eq!(
            store
                .resource_slot(actor, ResourceKind::Actor)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        let mut from =
            crate::core_host::imports::HostActorTeleportEvent::actor_teleport_event_get_from(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        let mut to =
            crate::core_host::imports::HostActorTeleportEvent::actor_teleport_event_get_to(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        assert_eq!(from.dimension, "overworld");
        assert_eq!(from.x, 1.0);
        assert_eq!(to.dimension, "overworld");
        assert_eq!(to.x, 6.0);
        from.x = 11.0;
        to.x = 22.0;
        crate::core_host::imports::HostActorTeleportEvent::actor_teleport_event_set_from(
            &mut store, event, from,
        )
        .unwrap()
        .unwrap();
        crate::core_host::imports::HostActorTeleportEvent::actor_teleport_event_set_to(
            &mut store, event, to,
        )
        .unwrap()
        .unwrap();
        crate::core_host::imports::HostActorTeleportEvent::actor_teleport_event_set_cancelled(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostActorTeleportEvent::actor_teleport_event_get_from(
                &mut store, event
            )
            .unwrap()
            .unwrap()
            .x,
            11.0
        );
        assert_eq!(
            crate::core_host::imports::HostActorTeleportEvent::actor_teleport_event_get_to(
                &mut store, event
            )
            .unwrap()
            .unwrap()
            .x,
            22.0
        );
        assert_eq!(
            crate::core_host::imports::HostActorTeleportEvent::actor_teleport_event_is_cancelled(
                &mut store, event
            )
            .unwrap(),
            Ok(true)
        );
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(event, ResourceKind::ActorTeleportEvent)
                .is_err()
        );
        assert!(store.resource_slot(actor, ResourceKind::Actor).is_err());
        assert!(matches!(
            crate::core_host::imports::HostActorTeleportEvent::actor_teleport_event_get_from(
                &mut store, event
            )
            .unwrap(),
            Err(crate::core_host::TypesHostError::NotFound)
        ));
        assert_eq!(
            crate::core_host::imports::HostActor::actor_get_actor_name(&mut store, actor).unwrap(),
            Err(crate::core_host::TypesHostError::NotFound)
        );

        // Cleanup invalidates the callback-scoped facade after a direct cancellation write.
        for invocation_id in [43, 44, 45] {
            store.invocation_id = invocation_id;
            let expiring = store
                .insert_actor_teleport_event_resource(test_actor_teleport_event_facade())
                .unwrap();
            crate::core_host::imports::HostActorTeleportEvent::actor_teleport_event_set_cancelled(
                &mut store, expiring, false,
            )
            .unwrap()
            .unwrap();
            store.clear_invocation_resources(invocation_id);
            assert!(
                store
                    .resource_slot(expiring, ResourceKind::ActorTeleportEvent)
                    .is_err()
            );
        }

        store.invocation_id = 46;
        store.config.max_plugin_resource_slots = 1;
        store.resource_slot_count = 1;
        assert_eq!(
            store
                .insert_actor_teleport_event_resource(test_actor_teleport_event_facade())
                .unwrap_err()
                .status(),
            crate::abi::AEGILEX_LIMIT_EXCEEDED
        );
        assert_eq!(store.resource_slot_count, 1);
        store.config.max_plugin_resource_slots = 0;
        assert!(store.host_borrowed_slots.is_empty());
    }

    #[test]
    fn player_death_event_message_mutates_immediately_and_expires() {
        let mut store = store_with_capabilities(&[
            "player-death-event.player-death-event.get-player",
            "player-death-event.player-death-event.get-death-message",
            "player-death-event.player-death-event.set-death-message",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_player_death_event_resource(test_player_death_event_facade())
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerDeathEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        let player =
            crate::core_host::imports::HostPlayerDeathEvent::player_death_event_get_player(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        assert_eq!(
            store
                .resource_slot(player, ResourceKind::Player)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert!(matches!(
            crate::core_host::imports::HostPlayerDeathEvent::player_death_event_get_death_message(&mut store, event).unwrap(),
            Ok(Some(crate::core_host::MessageMessage::PlainText(message))) if message == "test death"
        ));
        crate::core_host::imports::HostPlayerDeathEvent::player_death_event_set_death_message(
            &mut store, event, None,
        )
        .unwrap()
        .unwrap();
        assert!(matches!(
            crate::core_host::imports::HostPlayerDeathEvent::player_death_event_get_death_message(
                &mut store, event
            )
            .unwrap(),
            Ok(None)
        ));
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(event, ResourceKind::PlayerDeathEvent)
                .is_err()
        );
        assert!(store.resource_slot(player, ResourceKind::Player).is_err());
    }

    #[test]
    fn player_join_event_message_mutates_immediately_and_expires() {
        let mut store = store_with_capabilities(&[
            "player-join-event.player-join-event.get-player",
            "player-join-event.player-join-event.get-join-message",
            "player-join-event.player-join-event.set-join-message",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_player_join_event_resource(test_player_join_event_facade())
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerJoinEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        let player = crate::core_host::imports::HostPlayerJoinEvent::player_join_event_get_player(
            &mut store, event,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            store
                .resource_slot(player, ResourceKind::Player)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert!(matches!(
            crate::core_host::imports::HostPlayerJoinEvent::player_join_event_get_join_message(&mut store, event).unwrap(),
            Ok(Some(crate::core_host::MessageMessage::PlainText(message))) if message == "test join"
        ));
        crate::core_host::imports::HostPlayerJoinEvent::player_join_event_set_join_message(
            &mut store, event, None,
        )
        .unwrap()
        .unwrap();
        store.clear_invocation_resources(store.invocation_id);
        assert!(matches!(
            crate::core_host::imports::HostPlayerJoinEvent::player_join_event_get_join_message(
                &mut store, event
            )
            .unwrap(),
            Err(crate::core_host::TypesHostError::NotFound)
        ));
        assert!(store.resource_slot(player, ResourceKind::Player).is_err());
    }

    #[test]
    fn player_quit_event_message_mutates_immediately_and_expires() {
        let mut store = store_with_capabilities(&[
            "player-quit-event.player-quit-event.get-player",
            "player-quit-event.player-quit-event.get-quit-message",
            "player-quit-event.player-quit-event.set-quit-message",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_player_quit_event_resource(test_player_quit_event_facade())
            .unwrap();
        assert!(matches!(
            crate::core_host::imports::HostPlayerQuitEvent::player_quit_event_get_quit_message(&mut store, event).unwrap(),
            Ok(Some(crate::core_host::MessageMessage::PlainText(message))) if message == "test quit"
        ));
        crate::core_host::imports::HostPlayerQuitEvent::player_quit_event_set_quit_message(
            &mut store,
            event,
            Some(crate::core_host::MessageMessage::PlainText(
                "goodbye".to_owned(),
            )),
        )
        .unwrap()
        .unwrap();
        store.clear_invocation_resources(store.invocation_id);
        assert_eq!(
            crate::core_host::imports::HostPlayerQuitEvent::player_quit_event_set_quit_message(
                &mut store, event, None
            )
            .unwrap(),
            Err(crate::core_host::TypesHostError::NotFound)
        );
    }

    #[test]
    fn player_drop_item_event_cancellation_mutates_immediately() {
        let mut store = store_with_capabilities(&[
            "player-drop-item-event.player-drop-item-event.is-cancelled",
            "player-drop-item-event.player-drop-item-event.set-cancelled",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_player_drop_item_event_resource(test_player_drop_item_event_facade())
            .unwrap();
        crate::core_host::imports::HostPlayerDropItemEvent::player_drop_item_event_set_cancelled(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostPlayerDropItemEvent::player_drop_item_event_is_cancelled(&mut store, event).unwrap(),
            Ok(true)
        );
        store.clear_invocation_resources(store.invocation_id);

        store.invocation_id = 43;
        let event = store
            .insert_player_drop_item_event_resource(test_player_drop_item_event_facade())
            .unwrap();
        crate::core_host::imports::HostPlayerDropItemEvent::player_drop_item_event_set_cancelled(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(event, ResourceKind::PlayerDropItemEvent)
                .is_err()
        );
    }

    #[test]
    fn block_break_event_cancellation_mutates_immediately() {
        let mut store = store_with_capabilities(&[
            "block-break-event.block-break-event.is-cancelled",
            "block-break-event.block-break-event.set-cancelled",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_block_break_event_resource(test_block_break_event_facade())
            .unwrap();
        crate::core_host::imports::HostBlockBreakEvent::block_break_event_set_cancelled(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostBlockBreakEvent::block_break_event_is_cancelled(
                &mut store, event
            )
            .unwrap(),
            Ok(true)
        );
        store.clear_invocation_resources(store.invocation_id);

        store.invocation_id = 43;
        let expiring = store
            .insert_block_break_event_resource(test_block_break_event_facade())
            .unwrap();
        crate::core_host::imports::HostBlockBreakEvent::block_break_event_set_cancelled(
            &mut store, expiring, true,
        )
        .unwrap()
        .unwrap();
        store.clear_invocation_resources(store.invocation_id);
    }

    #[test]
    fn block_cook_event_mutates_result_and_cancellation_immediately() {
        let mut store = store_with_capabilities(&[
            "block-cook-event.block-cook-event.set-result",
            "block-cook-event.block-cook-event.is-cancelled",
            "block-cook-event.block-cook-event.set-cancelled",
            "item-stack.item-stack.set-amount",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_block_cook_event_resource(test_block_cook_event_facade())
            .unwrap();
        let item = store
            .host
            .server()
            .unwrap()
            .getItemType("minecraft:apple")
            .createItemStack(3);
        let result = store.insert_item_stack_resource(item).unwrap();
        crate::core_host::imports::HostItemStack::item_stack_set_amount(&mut store, result, 3)
            .unwrap()
            .unwrap();
        crate::core_host::imports::HostBlockCookEvent::block_cook_event_set_result(
            &mut store, event, result,
        )
        .unwrap()
        .unwrap();
        crate::core_host::imports::HostItemStack::item_stack_set_amount(&mut store, result, 8)
            .unwrap()
            .unwrap();
        crate::core_host::imports::HostBlockCookEvent::block_cook_event_set_cancelled(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostBlockCookEvent::block_cook_event_is_cancelled(
                &mut store, event
            )
            .unwrap(),
            Ok(true)
        );
        store.clear_invocation_resources(store.invocation_id);

        store.invocation_id = 43;
        let expiring = store
            .insert_block_cook_event_resource(test_block_cook_event_facade())
            .unwrap();
        crate::core_host::imports::HostBlockCookEvent::block_cook_event_set_cancelled(
            &mut store, expiring, true,
        )
        .unwrap()
        .unwrap();
        store.clear_invocation_resources(store.invocation_id);
    }

    #[test]
    fn block_place_event_cancellation_mutates_immediately() {
        let mut store = store_with_capabilities(&[
            "block-place-event.block-place-event.is-cancelled",
            "block-place-event.block-place-event.set-cancelled",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_block_place_event_resource(test_block_place_event_facade())
            .unwrap();
        crate::core_host::imports::HostBlockPlaceEvent::block_place_event_set_cancelled(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostBlockPlaceEvent::block_place_event_is_cancelled(
                &mut store, event
            )
            .unwrap(),
            Ok(true)
        );
        store.clear_invocation_resources(store.invocation_id);

        store.invocation_id = 43;
        let expiring = store
            .insert_block_place_event_resource(test_block_place_event_facade())
            .unwrap();
        crate::core_host::imports::HostBlockPlaceEvent::block_place_event_set_cancelled(
            &mut store, expiring, true,
        )
        .unwrap()
        .unwrap();
        store.clear_invocation_resources(store.invocation_id);
    }

    #[test]
    fn player_item_consume_event_cancellation_mutates_immediately() {
        let mut store = store_with_capabilities(&[
            "player-item-consume-event.player-item-consume-event.is-cancelled",
            "player-item-consume-event.player-item-consume-event.set-cancelled",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_player_item_consume_event_resource(test_player_item_consume_event_facade())
            .unwrap();
        crate::core_host::imports::HostPlayerItemConsumeEvent::player_item_consume_event_set_cancelled(&mut store, event, true)
        .unwrap().unwrap();
        assert_eq!(
            crate::core_host::imports::HostPlayerItemConsumeEvent::player_item_consume_event_is_cancelled(&mut store, event).unwrap(),
            Ok(true)
        );
        store.clear_invocation_resources(store.invocation_id);

        store.invocation_id = 43;
        let event = store
            .insert_player_item_consume_event_resource(test_player_item_consume_event_facade())
            .unwrap();
        crate::core_host::imports::HostPlayerItemConsumeEvent::player_item_consume_event_set_cancelled(&mut store, event, true)
        .unwrap().unwrap();
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(event, ResourceKind::PlayerItemConsumeEvent)
                .is_err()
        );
    }

    #[test]
    fn player_game_mode_change_event_is_borrowed_and_mutates_immediately() {
        let mut store = store_with_capabilities(&[
            "player-game-mode-change-event.player-game-mode-change-event.get-player",
            "player-game-mode-change-event.player-game-mode-change-event.get-new-game-mode",
            "player-game-mode-change-event.player-game-mode-change-event.is-cancelled",
            "player-game-mode-change-event.player-game-mode-change-event.set-cancelled",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_player_game_mode_change_event_resource(
                test_player_game_mode_change_event_facade(),
            )
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerGameModeChangeEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        let player = crate::core_host::imports::HostPlayerGameModeChangeEvent::player_game_mode_change_event_get_player(&mut store, event)
        .unwrap().unwrap();
        assert_eq!(
            store
                .resource_slot(player, ResourceKind::Player)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            crate::core_host::imports::HostPlayerGameModeChangeEvent::player_game_mode_change_event_get_new_game_mode(&mut store, event).unwrap(),
            Ok(crate::core_host::GameModeGameMode::Creative)
        );
        crate::core_host::imports::HostPlayerGameModeChangeEvent::player_game_mode_change_event_set_cancelled(&mut store, event, true)
        .unwrap().unwrap();
        assert_eq!(
            crate::core_host::imports::HostPlayerGameModeChangeEvent::player_game_mode_change_event_is_cancelled(&mut store, event).unwrap(),
            Ok(true)
        );
        store.clear_invocation_resources(store.invocation_id);

        store.invocation_id = 43;
        let expiring = store
            .insert_player_game_mode_change_event_resource(
                test_player_game_mode_change_event_facade(),
            )
            .unwrap();
        crate::core_host::imports::HostPlayerGameModeChangeEvent::player_game_mode_change_event_set_cancelled(&mut store, expiring, true)
        .unwrap().unwrap();
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(expiring, ResourceKind::PlayerGameModeChangeEvent)
                .is_err()
        );
    }

    #[test]
    fn player_emote_event_mutates_immediately_and_expires_children() {
        let mut store = store_with_capabilities(&[
            "player-emote-event.player-emote-event.get-player",
            "player-emote-event.player-emote-event.get-emote-id",
            "player-emote-event.player-emote-event.is-muted",
            "player-emote-event.player-emote-event.set-muted",
            "player-emote-event.player-emote-event.is-cancelled",
            "player-emote-event.player-emote-event.set-cancelled",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_player_emote_event_resource(test_player_emote_event_facade())
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerEmoteEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        let player =
            crate::core_host::imports::HostPlayerEmoteEvent::player_emote_event_get_player(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        assert_eq!(
            store
                .resource_slot(player, ResourceKind::Player)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            crate::core_host::imports::HostPlayerEmoteEvent::player_emote_event_get_emote_id(
                &mut store, event
            )
            .unwrap(),
            Ok("test-emote".to_owned())
        );
        crate::core_host::imports::HostPlayerEmoteEvent::player_emote_event_set_muted(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();
        crate::core_host::imports::HostPlayerEmoteEvent::player_emote_event_set_cancelled(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostPlayerEmoteEvent::player_emote_event_is_muted(
                &mut store, event
            )
            .unwrap(),
            Ok(true)
        );
        assert_eq!(
            crate::core_host::imports::HostPlayerEmoteEvent::player_emote_event_is_cancelled(
                &mut store, event
            )
            .unwrap(),
            Ok(true)
        );
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(event, ResourceKind::PlayerEmoteEvent)
                .is_err()
        );
        assert!(store.resource_slot(player, ResourceKind::Player).is_err());

        store.invocation_id = 43;
        let expiring = store
            .insert_player_emote_event_resource(test_player_emote_event_facade())
            .unwrap();
        crate::core_host::imports::HostPlayerEmoteEvent::player_emote_event_set_muted(
            &mut store, expiring, true,
        )
        .unwrap()
        .unwrap();
        crate::core_host::imports::HostPlayerEmoteEvent::player_emote_event_set_cancelled(
            &mut store, expiring, true,
        )
        .unwrap()
        .unwrap();
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(expiring, ResourceKind::PlayerEmoteEvent)
                .is_err()
        );
    }

    #[test]
    fn player_skin_change_event_mutates_immediately_and_expires_children() {
        let mut store = store_with_capabilities(&[
            "player-skin-change-event.player-skin-change-event.get-player",
            "player-skin-change-event.player-skin-change-event.get-skin-change-message",
            "player-skin-change-event.player-skin-change-event.set-skin-change-message",
            "player-skin-change-event.player-skin-change-event.is-cancelled",
            "player-skin-change-event.player-skin-change-event.set-cancelled",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_player_skin_change_event_resource(test_player_skin_change_event_facade())
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerSkinChangeEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        let player = crate::core_host::imports::HostPlayerSkinChangeEvent::player_skin_change_event_get_player(&mut store, event)
        .unwrap().unwrap();
        assert_eq!(
            store
                .resource_slot(player, ResourceKind::Player)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert!(matches!(
            crate::core_host::imports::HostPlayerSkinChangeEvent::player_skin_change_event_get_skin_change_message(&mut store, event).unwrap(),
            Ok(None)
        ));
        crate::core_host::imports::HostPlayerSkinChangeEvent::player_skin_change_event_set_skin_change_message(&mut store, event, Some(crate::core_host::MessageMessage::PlainText( "skin changed".to_owned(), )))
        .unwrap().unwrap();
        crate::core_host::imports::HostPlayerSkinChangeEvent::player_skin_change_event_set_cancelled(&mut store, event, true)
        .unwrap().unwrap();
        assert!(matches!(
            crate::core_host::imports::HostPlayerSkinChangeEvent::player_skin_change_event_get_skin_change_message(&mut store, event).unwrap(),
            Ok(Some(crate::core_host::MessageMessage::PlainText(message))) if message == "skin changed"
        ));
        assert_eq!(
            crate::core_host::imports::HostPlayerSkinChangeEvent::player_skin_change_event_is_cancelled(&mut store, event).unwrap(),
            Ok(true)
        );
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(event, ResourceKind::PlayerSkinChangeEvent)
                .is_err()
        );
        assert!(store.resource_slot(player, ResourceKind::Player).is_err());
    }

    #[test]
    fn player_item_held_event_is_borrowed_and_mutates_immediately() {
        let mut store = store_with_capabilities(&[
            "player-item-held-event.player-item-held-event.get-player",
            "player-item-held-event.player-item-held-event.get-previous-slot",
            "player-item-held-event.player-item-held-event.get-new-slot",
            "player-item-held-event.player-item-held-event.is-cancelled",
            "player-item-held-event.player-item-held-event.set-cancelled",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_player_item_held_event_resource(test_player_item_held_event_facade())
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerItemHeldEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        let player =
            crate::core_host::imports::HostPlayerItemHeldEvent::player_item_held_event_get_player(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        assert_eq!(
            store
                .resource_slot(player, ResourceKind::Player)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            crate::core_host::imports::HostPlayerItemHeldEvent::player_item_held_event_get_previous_slot(&mut store, event).unwrap(),
            Ok(2)
        );
        assert_eq!(
            crate::core_host::imports::HostPlayerItemHeldEvent::player_item_held_event_get_new_slot(&mut store, event).unwrap(),
            Ok(5)
        );
        crate::core_host::imports::HostPlayerItemHeldEvent::player_item_held_event_set_cancelled(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostPlayerItemHeldEvent::player_item_held_event_is_cancelled(&mut store, event).unwrap(),
            Ok(true)
        );
        store.clear_invocation_resources(store.invocation_id);

        store.invocation_id = 43;
        let expiring = store
            .insert_player_item_held_event_resource(test_player_item_held_event_facade())
            .unwrap();
        crate::core_host::imports::HostPlayerItemHeldEvent::player_item_held_event_set_cancelled(
            &mut store, expiring, true,
        )
        .unwrap()
        .unwrap();
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(expiring, ResourceKind::PlayerItemHeldEvent)
                .is_err()
        );
    }

    #[test]
    fn player_dimension_change_event_children_expire_after_callback() {
        let mut store = store_with_capabilities(&[
            "player-dimension-change-event.player-dimension-change-event.get-player",
            "player-dimension-change-event.player-dimension-change-event.get-from-dimension",
            "player-dimension-change-event.player-dimension-change-event.get-to-dimension",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_player_dimension_change_event_resource(
                test_player_dimension_change_event_facade(),
            )
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerDimensionChangeEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        let player = crate::core_host::imports::HostPlayerDimensionChangeEvent::player_dimension_change_event_get_player(&mut store, event)
        .unwrap().unwrap();
        assert_eq!(
            store
                .resource_slot(player, ResourceKind::Player)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            crate::core_host::imports::HostPlayerDimensionChangeEvent::player_dimension_change_event_get_from_dimension(&mut store, event).unwrap(),
            Ok("overworld".to_owned())
        );
        assert_eq!(
            crate::core_host::imports::HostPlayerDimensionChangeEvent::player_dimension_change_event_get_to_dimension(&mut store, event).unwrap(),
            Ok("nether".to_owned())
        );
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(event, ResourceKind::PlayerDimensionChangeEvent)
                .is_err()
        );
        assert!(store.resource_slot(player, ResourceKind::Player).is_err());

        store.invocation_id = 43;
        let expiring = store
            .insert_player_dimension_change_event_resource(
                test_player_dimension_change_event_facade(),
            )
            .unwrap();
        let expiring_player = crate::core_host::imports::HostPlayerDimensionChangeEvent::player_dimension_change_event_get_player(&mut store, expiring)
        .unwrap().unwrap();
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(expiring, ResourceKind::PlayerDimensionChangeEvent)
                .is_err()
        );
        assert!(
            store
                .resource_slot(expiring_player, ResourceKind::Player)
                .is_err()
        );
    }

    #[test]
    fn player_bed_enter_event_cancellation_mutates_and_expires_live_children() {
        let mut store = store_with_capabilities(&[
            "player-bed-enter-event.player-bed-enter-event.get-player",
            "player-bed-enter-event.player-bed-enter-event.get-bed",
            "player-bed-enter-event.player-bed-enter-event.is-cancelled",
            "player-bed-enter-event.player-bed-enter-event.set-cancelled",
            "actor.player.get-name",
            "block.block.get-type",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_player_bed_enter_event_resource(test_player_bed_enter_event_facade())
            .unwrap();
        let player =
            crate::core_host::imports::HostPlayerBedEnterEvent::player_bed_enter_event_get_player(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        let bed =
            crate::core_host::imports::HostPlayerBedEnterEvent::player_bed_enter_event_get_bed(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerBedEnterEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            store
                .resource_slot(player, ResourceKind::Player)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            store
                .resource_slot(bed, ResourceKind::Block)
                .unwrap()
                .lifetime,
            ResourceLifetime::GuestOwned
        );
        assert_eq!(
            crate::core_host::imports::HostPlayerBedEnterEvent::player_bed_enter_event_is_cancelled(
                &mut store, event,
            )
            .unwrap(),
            Ok(false)
        );
        crate::core_host::imports::HostPlayerBedEnterEvent::player_bed_enter_event_set_cancelled(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            crate::core_host::imports::HostPlayerBedEnterEvent::player_bed_enter_event_is_cancelled(
                &mut store, event,
            )
            .unwrap(),
            Ok(true)
        );
        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        assert_eq!(
            crate::core_host::imports::HostPlayerBedEnterEvent::player_bed_enter_event_is_cancelled(
                &mut store, event,
            )
            .unwrap(),
            Err(crate::core_host::TypesHostError::NotFound)
        );
        assert_eq!(
            crate::core_host::imports::HostActor::player_get_name(&mut store, player).unwrap(),
            Err(crate::core_host::TypesHostError::NotFound)
        );
        assert!(store.resource_slot(bed, ResourceKind::Block).is_ok());
        assert_eq!(
            crate::core_host::imports::HostBlock::block_get_type(&mut store, bed).unwrap(),
            Err(crate::core_host::TypesHostError::NotFound)
        );
    }

    #[test]
    fn player_bed_leave_event_expires_live_children_and_has_no_mutators() {
        let mut store = store_with_capabilities(&[
            "player-bed-leave-event.player-bed-leave-event.get-player",
            "player-bed-leave-event.player-bed-leave-event.get-bed",
            "actor.player.get-name",
            "block.block.get-type",
        ]);
        store.invocation_id = 42;
        let event = store
            .insert_player_bed_leave_event_resource(test_player_bed_leave_event_facade())
            .unwrap();
        let player =
            crate::core_host::imports::HostPlayerBedLeaveEvent::player_bed_leave_event_get_player(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        let bed =
            crate::core_host::imports::HostPlayerBedLeaveEvent::player_bed_leave_event_get_bed(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerBedLeaveEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            store
                .resource_slot(player, ResourceKind::Player)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        assert_eq!(
            store
                .resource_slot(bed, ResourceKind::Block)
                .unwrap()
                .lifetime,
            ResourceLifetime::GuestOwned
        );
        store.clear_invocation_resources(store.invocation_id);
        store.invocation_id = 43;
        assert!(matches!(
            crate::core_host::imports::HostPlayerBedLeaveEvent::player_bed_leave_event_get_player(
                &mut store, event,
            )
            .unwrap(),
            Err(crate::core_host::TypesHostError::NotFound)
        ));
        assert_eq!(
            crate::core_host::imports::HostActor::player_get_name(&mut store, player).unwrap(),
            Err(crate::core_host::TypesHostError::NotFound)
        );
        assert!(store.resource_slot(bed, ResourceKind::Block).is_ok());
        assert_eq!(
            crate::core_host::imports::HostBlock::block_get_type(&mut store, bed).unwrap(),
            Err(crate::core_host::TypesHostError::NotFound)
        );
    }
    #[test]
    fn player_respawn_event_children_expire_after_callback() {
        let mut store =
            store_with_capabilities(&["player-respawn-event.player-respawn-event.get-player"]);
        store.invocation_id = 42;

        let event = store
            .insert_player_respawn_event_resource(test_player_respawn_event_facade())
            .unwrap();
        assert_eq!(
            store
                .resource_slot(event, ResourceKind::PlayerRespawnEvent)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        let player =
            crate::core_host::imports::HostPlayerRespawnEvent::player_respawn_event_get_player(
                &mut store, event,
            )
            .unwrap()
            .unwrap();
        assert_eq!(
            store
                .resource_slot(player, ResourceKind::Player)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(event, ResourceKind::PlayerRespawnEvent)
                .is_err()
        );
        assert!(store.resource_slot(player, ResourceKind::Player).is_err());

        store.invocation_id = 43;
        let expiring = store
            .insert_player_respawn_event_resource(test_player_respawn_event_facade())
            .unwrap();
        let expiring_player =
            crate::core_host::imports::HostPlayerRespawnEvent::player_respawn_event_get_player(
                &mut store, expiring,
            )
            .unwrap()
            .unwrap();
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(expiring, ResourceKind::PlayerRespawnEvent)
                .is_err()
        );
        assert!(
            store
                .resource_slot(expiring_player, ResourceKind::Player)
                .is_err()
        );
    }

    #[test]
    fn player_move_event_mutates_locations_and_cancellation_immediately() {
        let mut store = store_with_capabilities(&[
            "player-move-event.player-move-event.get-player",
            "player-move-event.player-move-event.get-from",
            "player-move-event.player-move-event.set-from",
            "player-move-event.player-move-event.get-to",
            "player-move-event.player-move-event.set-to",
            "player-move-event.player-move-event.is-cancelled",
            "player-move-event.player-move-event.set-cancelled",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_player_move_event_resource(test_player_move_event_facade())
            .unwrap();
        let player = crate::core_host::imports::HostPlayerMoveEvent::player_move_event_get_player(
            &mut store, event,
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            store
                .resource_slot(player, ResourceKind::Player)
                .unwrap()
                .lifetime,
            ResourceLifetime::HostBorrowed
        );
        let from = crate::core_host::imports::HostPlayerMoveEvent::player_move_event_get_from(
            &mut store, event,
        )
        .unwrap()
        .unwrap();
        assert_eq!(from.dimension, "overworld");
        crate::core_host::imports::HostPlayerMoveEvent::player_move_event_set_from(
            &mut store, event, from,
        )
        .unwrap()
        .unwrap();
        let to = crate::core_host::imports::HostPlayerMoveEvent::player_move_event_get_to(
            &mut store, event,
        )
        .unwrap()
        .unwrap();
        crate::core_host::imports::HostPlayerMoveEvent::player_move_event_set_to(
            &mut store, event, to,
        )
        .unwrap()
        .unwrap();
        crate::core_host::imports::HostPlayerMoveEvent::player_move_event_set_cancelled(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();
        store.clear_invocation_resources(store.invocation_id);

        store.invocation_id = 43;
        let expiring = store
            .insert_player_move_event_resource(test_player_move_event_facade())
            .unwrap();
        crate::core_host::imports::HostPlayerMoveEvent::player_move_event_set_cancelled(
            &mut store, expiring, true,
        )
        .unwrap()
        .unwrap();
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(expiring, ResourceKind::PlayerMoveEvent)
                .is_err()
        );
    }

    #[test]
    fn weather_change_event_cancellation_mutates_immediately() {
        let mut store = store_with_capabilities(&[
            "weather-change-event.weather-change-event.get-to-weather",
            "weather-change-event.weather-change-event.is-cancelled",
            "weather-change-event.weather-change-event.set-cancelled",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_weather_change_event_resource(test_weather_change_event_facade())
            .unwrap();
        assert_eq!(
            crate::core_host::imports::HostWeatherChangeEvent::weather_change_event_get_to_weather(
                &mut store, event
            )
            .unwrap(),
            Ok(false)
        );
        crate::core_host::imports::HostWeatherChangeEvent::weather_change_event_set_cancelled(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();
        store.clear_invocation_resources(store.invocation_id);

        store.invocation_id = 43;
        let event = store
            .insert_weather_change_event_resource(test_weather_change_event_facade())
            .unwrap();
        // Cleanup invalidates the callback-scoped facade.
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(event, ResourceKind::WeatherChangeEvent)
                .is_err()
        );
    }

    #[test]
    fn thunder_change_event_cancellation_mutates_immediately() {
        let mut store = store_with_capabilities(&[
            "thunder-change-event.thunder-change-event.get-to-thunder",
            "thunder-change-event.thunder-change-event.is-cancelled",
            "thunder-change-event.thunder-change-event.set-cancelled",
        ]);
        store.invocation_id = 42;

        let event = store
            .insert_thunder_change_event_resource(test_thunder_change_event_facade())
            .unwrap();
        assert_eq!(
            crate::core_host::imports::HostThunderChangeEvent::thunder_change_event_get_to_thunder(
                &mut store, event
            )
            .unwrap(),
            Ok(false)
        );
        crate::core_host::imports::HostThunderChangeEvent::thunder_change_event_set_cancelled(
            &mut store, event, true,
        )
        .unwrap()
        .unwrap();
        store.clear_invocation_resources(store.invocation_id);

        store.invocation_id = 43;
        let event = store
            .insert_thunder_change_event_resource(test_thunder_change_event_facade())
            .unwrap();
        // Cleanup invalidates the callback-scoped facade.
        store.clear_invocation_resources(store.invocation_id);
        assert!(
            store
                .resource_slot(event, ResourceKind::ThunderChangeEvent)
                .is_err()
        );
    }

    #[test]
    fn host_borrowed_resource_quota_failure_rolls_back_its_handle() {
        let mut store = store();
        store.invocation_id = 42;
        store.config.max_plugin_resource_slots = 1;
        store.resource_slot_count = 1;
        let handle = store.handles.insert_block(
            store.invocation_id,
            store
                .host
                .server()
                .unwrap()
                .getLevel()
                .getBlock("overworld", 0, 0, 0),
        );

        assert_eq!(
            store
                .host_borrowed_resource_from_handle(ResourceKind::Block, handle)
                .unwrap_err()
                .status(),
            crate::abi::AEGILEX_LIMIT_EXCEEDED
        );
        assert_eq!(store.resource_slot_count, 1);
        assert!(store.host_borrowed_slots.is_empty());
        assert!(store.handles.block(store.invocation_id, handle).is_none());
    }

    #[test]
    fn block_resource_slot_failure_rolls_back_its_native_facade() {
        let mut store = store();
        store.invocation_id = 42;
        let handle = store.handles.insert_block(
            store.invocation_id,
            store
                .host
                .server()
                .unwrap()
                .getLevel()
                .getBlock("overworld", 0, 0, 0),
        );
        store.config.max_plugin_resource_slots = 1;
        store.resource_slot_count = 1;

        assert_eq!(
            store
                .resource_from_handle(ResourceKind::Block, handle)
                .unwrap_err()
                .status(),
            crate::abi::AEGILEX_LIMIT_EXCEEDED
        );
        assert!(store.host_borrowed_slots.is_empty());
        assert!(store.handles.block(store.invocation_id, handle).is_none());
    }

    #[test]
    fn block_resource_quota_rejects_the_next_native_facade() {
        let mut store = store_with_capabilities(&["dimension.get-block"]);
        store.invocation_id = 42;
        store.config.max_invocation_native_resources = 2;
        let mut blocks = Vec::with_capacity(2);
        for _ in 0..2 {
            let block = store
                .host
                .server()
                .unwrap()
                .getLevel()
                .getBlock("overworld", 0, 0, 0);
            blocks.push(store.insert_block_resource(block).unwrap());
        }

        let next = store
            .host
            .server()
            .unwrap()
            .getLevel()
            .getBlock("overworld", 0, 0, 0);
        assert_eq!(
            store.insert_block_resource(next).unwrap_err().status(),
            crate::abi::AEGILEX_LIMIT_EXCEEDED
        );
        assert_eq!(blocks.len(), 2);
    }

    #[test]
    fn player_inventory_can_produce_a_base_inventory_handle() {
        let mut store = store_with_capabilities(&[
            "actor.player.get-inventory",
            "player-inventory.player-inventory.get-inventory",
            "inventory.inventory.get-size",
        ]);
        store.invocation_id = 42;
        let mut players = store.host.server().unwrap().listOnlinePlayers();
        let player = store
            .insert_player_resource(players.pin_mut().takePlayer(0))
            .unwrap();

        let player_inventory =
            crate::core_host::imports::HostActor::player_get_inventory(&mut store, player)
                .unwrap()
                .unwrap();
        let inventory =
            crate::core_host::imports::HostPlayerInventory::player_inventory_get_inventory(
                &mut store,
                player_inventory,
            )
            .unwrap()
            .unwrap();

        assert_ne!(player_inventory, inventory);
        assert_eq!(
            crate::core_host::imports::HostInventory::inventory_get_size(&mut store, inventory)
                .unwrap(),
            Ok(36)
        );
    }

    #[test]
    fn denies_actor_mutation_without_capabilities() {
        let mut store = store();

        let actor = 0;
        assert_eq!(
            crate::core_host::imports::HostActor::actor_set_rotation(&mut store, actor, 90.0, 0.0)
                .unwrap(),
            Err(crate::core_host::TypesHostError::Denied)
        );
    }

    #[test]
    fn denies_player_ops_without_capabilities() {
        let mut store = store();
        let player = 0;

        assert_eq!(
            crate::core_host::imports::HostActor::player_set_operator(&mut store, player, true)
                .unwrap(),
            Err(crate::core_host::TypesHostError::Denied)
        );
        assert_eq!(
            crate::core_host::imports::HostActor::player_get_game_mode(&mut store, 0).unwrap(),
            Err(crate::core_host::TypesHostError::Denied)
        );
    }

    #[test]
    fn denies_logger_ops_without_capabilities() {
        let mut store = store();

        assert!(matches!(
            crate::core_host::imports::HostLogger::get_logger(&mut store),
            Err(error) if error == "Denied"
        ));
        assert_eq!(
            crate::core_host::imports::HostLogger::logger_set_level(
                &mut store,
                0,
                crate::core_host::LoggerLogLevel::Critical
            ),
            Err("Denied".to_owned())
        );
    }

    #[test]
    fn denies_set_max_players_without_capabilities() {
        let mut store = store();

        assert_eq!(
            crate::core_host::imports::HostServer::set_max_players(&mut store, 30).unwrap(),
            Err(crate::core_host::TypesHostError::Denied)
        );
    }

    #[test]
    fn denies_task_scheduling_without_capabilities() {
        let mut store = store();

        assert_eq!(
            crate::core_host::imports::HostTasks::schedule_after(&mut store, 1).unwrap(),
            Err(crate::core_host::TypesHostError::Denied)
        );
        assert_eq!(
            crate::core_host::imports::HostTasks::schedule_every(&mut store, 1, 2).unwrap(),
            Err(crate::core_host::TypesHostError::Denied)
        );
        assert_eq!(
            crate::core_host::imports::HostTasks::cancel(&mut store, 7).unwrap(),
            Err(crate::core_host::TypesHostError::Denied)
        );
    }

    #[test]
    fn dispatch_rejects_unknown_plugins() {
        let mut runtime = Runtime::new(stub_host_context(), test_config()).unwrap();
        let invocation = crate::core_host::CommandsInvocation {
            subcommand: "ping".to_owned(),
            args: Vec::new(),
            sender: 0,
        };
        assert!(matches!(
            runtime.dispatch_wit_command("missing", 1, invocation),
            Err(AEGILEX_NOT_FOUND)
        ));
        assert_eq!(runtime.dispatch_task("missing", 1), Err(AEGILEX_NOT_FOUND));
    }

    #[test]
    fn event_subscription_check_rejects_missing_plugins() {
        let runtime = Runtime::new(stub_host_context(), test_config()).unwrap();

        assert!(!runtime.should_dispatch_event("missing", "chunk-load"));
    }
}
