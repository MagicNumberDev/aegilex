use super::*;

mod cases {
    use super::*;

    fn metadata(name: &str) -> crate::core_host::PluginMetadataMetadata {
        crate::core_host::PluginMetadataMetadata {
            name: name.to_owned(),
            version: "1.0.0".to_owned(),
            description: String::new(),
            load_order: crate::core_host::PluginTypesLoadOrder::Startup,
            authors: Vec::new(),
            contributors: Vec::new(),
            website: String::new(),
            prefix: String::new(),
            provides: Vec::new(),
            depend: Vec::new(),
            soft_depend: Vec::new(),
            load_before: Vec::new(),
            default_permission: crate::core_host::PermissionDefaultPermissionDefault::Operator,
            commands: Vec::new(),
            permissions: Vec::new(),
            subscriptions: Vec::new(),
        }
    }

    fn command(name: &str) -> crate::core_host::PluginTypesCommand {
        crate::core_host::PluginTypesCommand {
            name: name.to_owned(),
            description: None,
            aliases: Vec::new(),
            usages: vec![format!("/{name}")],
            permissions: Vec::new(),
        }
    }

    #[test]
    fn metadata_name_must_match_module_directory() {
        let error = validate_metadata(
            Path::new("plugins/example_hello/plugin.wasm"),
            metadata("other_plugin"),
        )
        .unwrap_err();

        assert_eq!(error, "metadata name must match the module directory name");
    }

    #[test]
    fn metadata_normalizes_routing_lists() {
        let mut input = metadata("example_hello");
        input.commands = vec![
            crate::core_host::PluginTypesCommand {
                name: "hello".to_owned(),
                description: Some("Say hello".to_owned()),
                aliases: vec!["hi".to_owned()],
                usages: vec!["/hello".to_owned(), "/hello <name: string>".to_owned()],
                permissions: vec!["aegilex.example.hello".to_owned()],
            },
            command("status"),
        ];
        input.subscriptions = vec![
            "player-quit".to_owned(),
            "player-join".to_owned(),
            "player-join".to_owned(),
        ];

        let metadata =
            validate_metadata(Path::new("plugins/example_hello/plugin.wasm"), input).unwrap();

        assert_eq!(
            metadata.commands,
            [
                CommandSpec {
                    name: "hello".to_owned(),
                    description: Some("Say hello".to_owned()),
                    aliases: vec!["hi".to_owned()],
                    usages: vec!["/hello".to_owned(), "/hello <name: string>".to_owned()],
                    permissions: vec!["aegilex.example.hello".to_owned()],
                },
                CommandSpec {
                    name: "status".to_owned(),
                    description: None,
                    aliases: Vec::new(),
                    usages: vec!["/status".to_owned()],
                    permissions: Vec::new(),
                },
            ]
        );
        assert_eq!(metadata.subscriptions, ["player-join", "player-quit"]);
    }

    #[test]
    fn metadata_accepts_endstone_test_command_usages() {
        let mut input = metadata("endstone_test");
        input.commands = vec![crate::core_host::PluginTypesCommand {
            name: "test".to_owned(),
            description: Some("Run the test command".to_owned()),
            aliases: vec!["t".to_owned()],
            usages: vec![
                "/test form <message|action|modal>".to_owned(),
                "/test sender".to_owned(),
                "/test player <toast|title|kick|particle|sound>".to_owned(),
                "/test block <block: block> [blockStates: block_states]".to_owned(),
                "/test broadcast".to_owned(),
                "/test inv <mainhand|offhand|meta>".to_owned(),
                "/test spawn <entity: entity_type>".to_owned(),
            ],
            permissions: vec!["endstone_test.command.test".to_owned()],
        }];
        input.permissions = vec![crate::core_host::PluginTypesPluginPermission {
            name: "endstone_test.command.test".to_owned(),
            description: Some("Allow users to use the /test command.".to_owned()),
            default_value: Some(crate::core_host::PermissionDefaultPermissionDefault::True),
            children: vec![crate::core_host::PermissionAttachmentPermissionChild {
                name: "endstone_test.command.admin".to_owned(),
                value: true,
            }],
        }];

        let metadata =
            validate_metadata(Path::new("plugins/endstone_test/plugin.wasm"), input).unwrap();

        assert_eq!(metadata.commands.len(), 1);
        assert_eq!(metadata.commands[0].usages.len(), 7);
        assert_eq!(metadata.commands[0].aliases, ["t"]);
        assert_eq!(
            metadata.commands[0].permissions,
            ["endstone_test.command.test"]
        );
        assert_eq!(metadata.permissions.len(), 1);
        assert_eq!(
            metadata.permissions[0],
            PermissionSpec {
                name: "endstone_test.command.test".to_owned(),
                description: Some("Allow users to use the /test command.".to_owned()),
                default_value: Some(0),
                children: vec![PermissionChild {
                    name: "endstone_test.command.admin".to_owned(),
                    value: true,
                }],
            }
        );
    }

    #[test]
    fn metadata_rejects_duplicate_commands() {
        let mut input = metadata("example_hello");
        input.commands = vec![command("hello"), command("hello")];

        let error =
            validate_metadata(Path::new("plugins/example_hello/plugin.wasm"), input).unwrap_err();

        assert_eq!(error, "duplicate command name: hello");
    }

    #[test]
    fn metadata_rejects_alias_collisions() {
        let mut input = metadata("example_hello");
        input.commands = vec![
            crate::core_host::PluginTypesCommand {
                name: "hello".to_owned(),
                description: None,
                aliases: vec!["status".to_owned()],
                usages: vec!["/hello".to_owned()],
                permissions: Vec::new(),
            },
            command("status"),
        ];

        let error =
            validate_metadata(Path::new("plugins/example_hello/plugin.wasm"), input).unwrap_err();

        assert_eq!(
            error,
            "command name or alias is claimed more than once: status"
        );
    }

    #[test]
    fn metadata_defers_usage_validation_to_endstone() {
        let mut input = metadata("example_hello");
        input.commands = vec![crate::core_host::PluginTypesCommand {
            name: "hello".to_owned(),
            description: None,
            aliases: Vec::new(),
            usages: vec!["/other <text: string>".to_owned()],
            permissions: Vec::new(),
        }];

        let metadata =
            validate_metadata(Path::new("plugins/example_hello/plugin.wasm"), input).unwrap();

        assert_eq!(metadata.commands[0].usages, ["/other <text: string>"]);
    }

    #[test]
    fn metadata_rejects_duplicate_permissions() {
        let mut input = metadata("example_hello");
        input.permissions = vec![
            crate::core_host::PluginTypesPluginPermission {
                name: "aegilex.example.hello".to_owned(),
                description: None,
                default_value: None,
                children: Vec::new(),
            },
            crate::core_host::PluginTypesPluginPermission {
                name: "aegilex.example.hello".to_owned(),
                description: None,
                default_value: None,
                children: Vec::new(),
            },
        ];

        let error =
            validate_metadata(Path::new("plugins/example_hello/plugin.wasm"), input).unwrap_err();

        assert_eq!(error, "duplicate permission name: aegilex.example.hello");
    }

    #[test]
    fn metadata_defers_permission_name_validation_to_endstone() {
        let mut input = metadata("example_hello");
        input.permissions = vec![crate::core_host::PluginTypesPluginPermission {
            name: "aegilex.example.hello".to_owned(),
            description: None,
            default_value: None,
            children: vec![crate::core_host::PermissionAttachmentPermissionChild {
                name: "Not Valid!".to_owned(),
                value: true,
            }],
        }];

        let metadata =
            validate_metadata(Path::new("plugins/example_hello/plugin.wasm"), input).unwrap();

        assert_eq!(metadata.permissions[0].children[0].name, "Not Valid!");
    }

    #[test]
    fn metadata_accepts_new_runtime_subscriptions() {
        let mut input = metadata("example_hello");
        input.subscriptions = vec![
            "packet-send".to_owned(),
            "packet-receive".to_owned(),
            "map-initialize".to_owned(),
        ];

        let metadata =
            validate_metadata(Path::new("plugins/example_hello/plugin.wasm"), input).unwrap();

        assert_eq!(
            metadata.subscriptions,
            ["map-initialize", "packet-receive", "packet-send"]
        );
    }

    #[test]
    fn metadata_accepts_long_command_description() {
        let mut input = metadata("example_hello");
        input.commands = vec![crate::core_host::PluginTypesCommand {
            name: "hello".to_owned(),
            description: Some("x".repeat(257)),
            aliases: Vec::new(),
            usages: vec!["/hello".to_owned()],
            permissions: Vec::new(),
        }];

        let metadata =
            validate_metadata(Path::new("plugins/example_hello/plugin.wasm"), input).unwrap();

        assert_eq!(
            metadata.commands[0].description.as_deref(),
            Some("x".repeat(257).as_str())
        );
    }

    #[test]
    fn metadata_defers_command_permission_validation_to_endstone() {
        let mut input = metadata("example_hello");
        input.commands = vec![crate::core_host::PluginTypesCommand {
            name: "hello".to_owned(),
            description: None,
            aliases: Vec::new(),
            usages: vec!["/hello".to_owned()],
            permissions: vec!["Not Valid!".to_owned()],
        }];

        let metadata =
            validate_metadata(Path::new("plugins/example_hello/plugin.wasm"), input).unwrap();

        assert_eq!(metadata.commands[0].permissions, ["Not Valid!"]);
    }

    #[test]
    fn metadata_rejects_reserved_commands() {
        let mut input = metadata("example_hello");
        input.commands = vec![command("aegilex")];

        let error =
            validate_metadata(Path::new("plugins/example_hello/plugin.wasm"), input).unwrap_err();

        assert_eq!(error, "command name is reserved: aegilex");
    }

    #[test]
    fn metadata_rejects_reserved_aliases() {
        let mut input = metadata("example_hello");
        input.commands = vec![crate::core_host::PluginTypesCommand {
            name: "hello".to_owned(),
            description: None,
            aliases: vec!["aegilex".to_owned()],
            usages: vec!["/hello".to_owned()],
            permissions: Vec::new(),
        }];

        let error =
            validate_metadata(Path::new("plugins/example_hello/plugin.wasm"), input).unwrap_err();

        assert_eq!(error, "command alias is reserved: aegilex");
    }

    #[test]
    fn metadata_accepts_empty_usages() {
        let mut input = metadata("example_hello");
        input.commands = vec![crate::core_host::PluginTypesCommand {
            name: "hello".to_owned(),
            description: None,
            aliases: Vec::new(),
            usages: Vec::new(),
            permissions: Vec::new(),
        }];

        let metadata =
            validate_metadata(Path::new("plugins/example_hello/plugin.wasm"), input).unwrap();

        assert!(metadata.commands[0].usages.is_empty());
    }

    #[test]
    fn metadata_defers_extended_usage_to_endstone() {
        let mut input = metadata("example_hello");
        input.commands = vec![crate::core_host::PluginTypesCommand {
            name: "hello".to_owned(),
            description: None,
            aliases: Vec::new(),
            usages: vec!["/hellohello".to_owned()],
            permissions: Vec::new(),
        }];

        let metadata =
            validate_metadata(Path::new("plugins/example_hello/plugin.wasm"), input).unwrap();

        assert_eq!(metadata.commands[0].usages, ["/hellohello"]);
    }

    #[test]
    fn metadata_maps_all_permission_defaults() {
        let mut input = metadata("example_hello");
        input.permissions = vec![
            crate::core_host::PluginTypesPluginPermission {
                name: "aegilex.example.true".to_owned(),
                description: None,
                default_value: Some(crate::core_host::PermissionDefaultPermissionDefault::True),
                children: Vec::new(),
            },
            crate::core_host::PluginTypesPluginPermission {
                name: "aegilex.example.false".to_owned(),
                description: None,
                default_value: Some(crate::core_host::PermissionDefaultPermissionDefault::False),
                children: Vec::new(),
            },
            crate::core_host::PluginTypesPluginPermission {
                name: "aegilex.example.operator".to_owned(),
                description: None,
                default_value: Some(crate::core_host::PermissionDefaultPermissionDefault::Operator),
                children: Vec::new(),
            },
            crate::core_host::PluginTypesPluginPermission {
                name: "aegilex.example.not-operator".to_owned(),
                description: None,
                default_value: Some(
                    crate::core_host::PermissionDefaultPermissionDefault::NotOperator,
                ),
                children: Vec::new(),
            },
            crate::core_host::PluginTypesPluginPermission {
                name: "aegilex.example.console".to_owned(),
                description: None,
                default_value: Some(crate::core_host::PermissionDefaultPermissionDefault::Console),
                children: Vec::new(),
            },
            crate::core_host::PluginTypesPluginPermission {
                name: "aegilex.example.unspecified".to_owned(),
                description: None,
                default_value: None,
                children: Vec::new(),
            },
        ];

        let metadata =
            validate_metadata(Path::new("plugins/example_hello/plugin.wasm"), input).unwrap();

        assert_eq!(
            metadata
                .permissions
                .iter()
                .map(|permission| (permission.name.as_str(), permission.default_value))
                .collect::<Vec<_>>(),
            [
                ("aegilex.example.console", Some(4)),
                ("aegilex.example.false", Some(1)),
                ("aegilex.example.not-operator", Some(3)),
                ("aegilex.example.operator", Some(2)),
                ("aegilex.example.true", Some(0)),
                ("aegilex.example.unspecified", None),
            ]
        );
    }

    fn policy_fixture(policy: &str) -> (PathBuf, PathBuf) {
        use std::sync::atomic::{AtomicUsize, Ordering};

        static NEXT_FIXTURE: AtomicUsize = AtomicUsize::new(0);

        let root = std::env::temp_dir().join(format!(
            "aegilex-policy-{}-{}",
            std::process::id(),
            NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed)
        ));
        let plugin_dir = root.join("plugins/example_hello");
        fs::create_dir_all(root.join("data")).unwrap();
        fs::create_dir_all(&plugin_dir).unwrap();
        let module = plugin_dir.join(MODULE_NAME);
        fs::write(&module, []).unwrap();
        fs::write(plugin_dir.join(POLICY_NAME), policy).unwrap();
        (root, module)
    }

    #[test]
    fn policy_accepts_exact_and_interface_capabilities() {
        let (root, module) = policy_fixture(
            r#"
                capabilities = ["logger.get-logger", "logger.logger.log", "level.*", "actor.actor.*"]
                paths = ["data"]
                network = ["tcp:203.0.113.10:443", "udp:[2001:db8::10]:3478"]
            "#,
        );

        let policy = load_plugin_policy(&module.canonicalize().unwrap()).unwrap();
        assert_eq!(
            policy.capabilities,
            [
                "logger.get-logger",
                "logger.logger.log",
                "level.*",
                "actor.actor.*"
            ]
        );
        assert_eq!(policy.paths.len(), 1);
        assert_eq!(policy.paths[0].guest_path, "/data");
        assert_eq!(policy.network.len(), 2);
        assert_eq!(policy.network[0].protocol, NetworkProtocol::Tcp);
        assert_eq!(
            policy.network[0].address,
            "203.0.113.10:443".parse().unwrap()
        );
        assert_eq!(policy.network[1].protocol, NetworkProtocol::Udp);
        assert_eq!(
            policy.network[1].address,
            "[2001:db8::10]:3478".parse().unwrap()
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn policy_accepts_canonical_resource_capabilities() {
        assert!(is_known_capability("item-stack.item-stack.get-amount"));
        assert!(is_known_capability("item-stack.item-stack.clone"));
        assert!(is_known_capability("item-stack.item-stack-ref.get-amount"));
        assert!(is_known_capability("item-stack.item-stack-ref.clone"));
        assert!(is_known_capability("actor.item-actor.get-item-stack"));
        assert!(is_known_capability("actor.item-actor.set-item-stack"));
        assert!(!is_known_capability("actor.item-actor.missing"));
        assert!(!is_known_capability("item-stack.item-stack-ref.set-amount"));
    }

    #[test]
    fn policy_accepts_script_message_event_capabilities() {
        for capability in [
            "script-message-event.script-message-event.get-message-id",
            "script-message-event.script-message-event.get-message",
            "script-message-event.script-message-event.get-sender",
        ] {
            assert!(is_known_capability(capability), "{capability}");
        }
    }

    #[test]
    fn policy_accepts_block_type_capabilities() {
        assert!(is_known_capability("block-type.has-item-type"));
        assert!(is_known_capability("block-type.create-block-data"));
        assert!(!is_known_capability("block-type.get-item"));
    }

    #[test]
    fn policy_accepts_item_stack_nbt_capabilities() {
        for capability in [
            "item-stack.item-stack.get-nbt",
            "item-stack.item-stack.set-nbt",
            "item-stack.item-stack-ref.get-nbt",
        ] {
            assert!(is_known_capability(capability), "{capability}");
        }
    }

    #[test]
    fn policy_accepts_logger_capabilities() {
        for capability in [
            "logger.get-logger",
            "logger.logger.log",
            "logger.logger.get-name",
            "logger.logger.get-level",
            "logger.logger.set-level",
        ] {
            assert!(is_known_capability(capability), "{capability}");
        }
        assert!(!is_known_capability("host.get-logger"));
    }

    #[test]
    fn policy_accepts_nbt_tag_capabilities() {
        for capability in [
            "nbt.from-int",
            "nbt.from-list",
            "nbt.from-compound",
            "nbt.tag.get-type",
            "nbt.tag.get-compound",
            "nbt.tag.list-set",
            "nbt.tag.set-int",
        ] {
            assert!(is_known_capability(capability), "{capability}");
        }
        for capability in [
            "nbt.from-binary",
            "nbt.tag-missing",
            "nbt.tag-compound-entry",
        ] {
            assert!(!is_known_capability(capability), "{capability}");
        }
    }

    #[test]
    fn policy_accepts_server_console_dispatch_capability() {
        assert!(is_known_capability("server.dispatch-console-command"));
        assert!(!is_known_capability("admin.dispatch-console-command"));
    }

    #[test]
    fn policy_accepts_pickup_event_capabilities() {
        for capability in [
            "player-pickup-item-event.player-pickup-item-event.get-player",
            "player-pickup-item-event.player-pickup-item-event.get-item-actor",
            "player-pickup-item-event.player-pickup-item-event.is-cancelled",
            "player-pickup-item-event.player-pickup-item-event.set-cancelled",
        ] {
            assert!(is_known_capability(capability), "{capability}");
        }
    }

    #[test]
    fn policy_accepts_game_mode_change_event_capabilities() {
        for capability in [
            "player-game-mode-change-event.player-game-mode-change-event.get-player",
            "player-game-mode-change-event.player-game-mode-change-event.get-new-game-mode",
            "player-game-mode-change-event.player-game-mode-change-event.is-cancelled",
            "player-game-mode-change-event.player-game-mode-change-event.set-cancelled",
        ] {
            assert!(is_known_capability(capability), "{capability}");
        }
    }

    #[test]
    fn policy_accepts_emote_event_capabilities() {
        for capability in [
            "player-emote-event.player-emote-event.get-player",
            "player-emote-event.player-emote-event.get-emote-id",
            "player-emote-event.player-emote-event.is-muted",
            "player-emote-event.player-emote-event.set-muted",
            "player-emote-event.player-emote-event.is-cancelled",
            "player-emote-event.player-emote-event.set-cancelled",
        ] {
            assert!(is_known_capability(capability), "{capability}");
        }
    }

    #[test]
    fn policy_accepts_dimension_change_event_capabilities() {
        for capability in [
            "player-dimension-change-event.player-dimension-change-event.get-player",
            "player-dimension-change-event.player-dimension-change-event.get-from-dimension",
            "player-dimension-change-event.player-dimension-change-event.get-to-dimension",
        ] {
            assert!(is_known_capability(capability), "{capability}");
        }
    }

    #[test]
    fn policy_accepts_respawn_event_capabilities() {
        assert!(is_known_capability(
            "player-respawn-event.player-respawn-event.get-player"
        ));
    }

    #[test]
    fn policy_accepts_join_and_quit_event_capabilities() {
        for capability in [
            "player-join-event.player-join-event.get-player",
            "player-join-event.player-join-event.get-join-message",
            "player-join-event.player-join-event.set-join-message",
            "player-quit-event.player-quit-event.get-player",
            "player-quit-event.player-quit-event.get-quit-message",
            "player-quit-event.player-quit-event.set-quit-message",
        ] {
            assert!(is_known_capability(capability), "{capability}");
        }
    }
    #[test]
    fn policy_accepts_item_held_event_capabilities() {
        for capability in [
            "player-item-held-event.player-item-held-event.get-player",
            "player-item-held-event.player-item-held-event.get-previous-slot",
            "player-item-held-event.player-item-held-event.get-new-slot",
            "player-item-held-event.player-item-held-event.is-cancelled",
            "player-item-held-event.player-item-held-event.set-cancelled",
        ] {
            assert!(is_known_capability(capability), "{capability}");
        }
    }

    #[test]
    fn policy_rejects_unknown_capability_and_unsafe_path() {
        let (root, module) = policy_fixture(
            r#"
                capabilities = ["player.write"]
                paths = ["../outside"]
            "#,
        );

        let error = load_plugin_policy(&module.canonicalize().unwrap()).unwrap_err();
        assert!(error.contains("unknown capability"));

        fs::write(
            module.parent().unwrap().join(POLICY_NAME),
            "paths = [\"../outside\"]",
        )
        .unwrap();
        let error = load_plugin_policy(&module.canonicalize().unwrap()).unwrap_err();
        assert!(error.contains("must be a non-empty relative path"));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn policy_rejects_unknown_keys_and_non_literal_network_addresses() {
        let (root, module) = policy_fixture(
            r#"
                capabilities = ["*"]
                network = ["tcp:example.com:443"]
                metadata = "guest-controlled"
            "#,
        );

        let error = load_plugin_policy(&module.canonicalize().unwrap()).unwrap_err();
        assert!(error.contains("unknown authorization policy key"));

        fs::write(
            module.parent().unwrap().join(POLICY_NAME),
            "network = [\"tcp:example.com:443\"]",
        )
        .unwrap();
        let error = load_plugin_policy(&module.canonicalize().unwrap()).unwrap_err();
        assert!(error.contains("literal IP and single port"));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn direct_resource_conversion_capabilities_and_wit_surface_are_exact() {
        for capability in [
            "actor.actor.as-mob",
            "actor.actor.as-item-actor",
            "actor.actor.as-player",
            "actor.mob.as-actor",
            "actor.item-actor.as-actor",
            "actor.player.as-actor",
        ] {
            assert!(is_known_capability(capability));
        }
        for capability in [
            "command-sender.command-sender.as-console",
            "command-sender.command-sender.as-block",
            "command-sender.command-sender.as-actor",
            "command-sender.command-sender.as-player",
            "actor.actor.as-item-stack",
            "actor.mob.as-player",
            "actor.mob.as-item-actor",
            "actor.item-actor.as-player",
            "actor.player.as-mob",
            "actor.player.as-item-actor",
        ] {
            assert!(!is_known_capability(capability));
        }

        let command_sender = include_str!("../wit/command-command-sender.wit");
        assert!(!command_sender.contains("as-console: func("));
        assert!(!command_sender.contains("as-block: func("));
        assert!(!command_sender.contains("as-actor: func("));
        assert!(!command_sender.contains("as-player: func("));
        let actor = include_str!("../wit/actor-actor.wit");
        let item_actor = include_str!("../wit/actor-item.wit");
        assert!(actor.contains("as-mob: func("));
        assert!(actor.contains("as-item-actor: func("));
        assert!(actor.contains("as-player: func("));
        assert!(actor.contains("resource mob {"));
        assert!(actor.contains("as-actor: func("));
        assert!(actor.contains("resource item-actor {"));
        assert!(actor.contains("resource player {"));
        assert!(item_actor.contains("use actor.{item-actor};"));
        assert!(include_str!("../wit/player.wit").contains("use actor.{player};"));
        assert!(!actor.contains("as-item: func("));
        assert!(!actor.contains("update-item-actor: func("));
    }
}
