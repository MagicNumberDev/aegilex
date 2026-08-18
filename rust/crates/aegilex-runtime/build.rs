use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use wit_parser::{FunctionKind, Resolve};

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing manifest directory"));
    let root = manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .and_then(|path| path.parent())
        .expect("runtime crate must live below the workspace root");
    let output = PathBuf::from(env::var("OUT_DIR").expect("missing build output directory"));
    generate_capabilities(&manifest_dir, &output);
    generate_core_bindings(&manifest_dir, &output);
    generate_core_abi_probe_bindings(root, &output);

    generate_store_impls(&manifest_dir, &output);

    let mut cxx = cxx_build::bridges([
        "src/cxx_host.rs",
        "src/cxx_runtime.rs",
        "src/cxx_host_server.rs",
        "src/cxx_host_player.rs",
        "src/cxx_host_inventory.rs",
        "src/cxx_host_level.rs",
        "src/cxx_host_actor.rs",
        "src/cxx_host_admin.rs",
        "src/cxx_host_ui.rs",
        "src/cxx_host_common.rs",
    ]);
    cxx.include(root.join("native"))
        .std("c++17")
        .compile("aegilex-runtime-cxx");
    for bridge in [
        "src/cxx_host.rs",
        "src/cxx_runtime.rs",
        "src/cxx_host_server.rs",
        "src/cxx_host_player.rs",
        "src/cxx_host_inventory.rs",
        "src/cxx_host_level.rs",
        "src/cxx_host_actor.rs",
        "src/cxx_host_admin.rs",
        "src/cxx_host_ui.rs",
        "src/cxx_host_common.rs",
    ] {
        println!("cargo::rerun-if-changed={bridge}");
    }

    build_test_stub(root, &output);
}

fn generate_capabilities(manifest_dir: &Path, output: &Path) {
    let wit_dir = manifest_dir.join("wit");
    let mut resolve = Resolve::default();
    let (package, sources) = resolve.push_dir(&wit_dir).unwrap_or_else(|error| {
        panic!(
            "failed to parse canonical WIT {}: {error}",
            wit_dir.display()
        )
    });
    let world = *resolve.packages[package]
        .worlds
        .get("plugin")
        .expect("canonical WIT package must define world plugin");
    let mut capabilities = BTreeSet::new();

    for item in resolve.worlds[world].imports.values() {
        let wit_parser::WorldItem::Interface { id, .. } = item else {
            continue;
        };
        let interface = &resolve.interfaces[*id];
        let interface_name = interface
            .name
            .as_deref()
            .expect("world imports must name their interfaces");
        for function in interface.functions.values() {
            match function.kind {
                FunctionKind::Freestanding => {
                    capabilities.insert(format!("{interface_name}.{}", function.name));
                }
                FunctionKind::Method(resource)
                | FunctionKind::Static(resource)
                | FunctionKind::Constructor(resource) => {
                    let resource_name = resolve.types[resource]
                        .name
                        .as_deref()
                        .expect("resource functions must name their resource");
                    capabilities.insert(format!(
                        "{interface_name}.{resource_name}.{}",
                        function.item_name()
                    ));
                }
            }
        }
    }

    let mut generated = String::from("// Generated from wit/world.wit by build.rs. Do not edit.\n");
    generated.push_str("pub(crate) const KNOWN_CAPABILITIES: &[&str] = &[\n");
    for capability in capabilities {
        generated.push_str("    ");
        generated.push_str(&format!("{capability:?},\n"));
    }
    generated.push_str("];\n");
    fs::write(output.join("capabilities.rs"), generated)
        .expect("failed to write generated capability registry");

    for path in sources.paths() {
        println!("cargo::rerun-if-changed={}", path.display());
    }
}
fn generate_core_bindings(manifest_dir: &Path, output: &Path) {
    let wit_dir = manifest_dir.join("wit");
    let (resolve, world) = aegilex_core_bindgen::parse_world(&wit_dir, "plugin")
        .unwrap_or_else(|error| panic!("failed to parse canonical WIT for Core bindings: {error}"));
    let mut bindgen = aegilex_core_bindgen::Bindgen::new(resolve, world);
    fs::write(output.join("core_bindings.rs"), bindgen.generate())
        .expect("failed to write generated Core bindings");
    for path in fs::read_dir(&wit_dir)
        .expect("failed to read canonical WIT directory")
        .map(|entry| entry.expect("WIT entry").path())
    {
        if path.extension().is_some_and(|extension| extension == "wit") {
            println!("cargo::rerun-if-changed={}", path.display());
        }
    }
}

fn generate_core_abi_probe_bindings(root: &Path, output: &Path) {
    let probe_dir = root.join("tests/core-abi-probe/wit");
    let bindings = aegilex_core_bindgen::generate_world(&probe_dir, "probe-world")
        .unwrap_or_else(|error| panic!("failed to generate Core ABI probe bindings: {error}"));
    fs::write(output.join("core_abi_probe_bindings.rs"), bindings)
        .expect("failed to write generated Core ABI probe bindings");
    println!("cargo::rerun-if-changed={}", probe_dir.display());
}
fn generate_store_impls(manifest_dir: &Path, output: &Path) {
    let wit_dir = manifest_dir.join("wit");
    let (resolve, world) = aegilex_core_bindgen::parse_world(&wit_dir, "plugin")
        .unwrap_or_else(|error| panic!("failed to parse canonical WIT for store impls: {error}"));
    let mut bindgen = aegilex_core_bindgen::Bindgen::new(resolve, world);
    // Interfaces with real generated-trait implementations in
    // src/host/store_impls.rs are excluded from the default-denied set.
    const EXCLUDED: &[&str] = &[
        "logger",
        "server",
        "actor",
        "player",
        "command-sender",
        "block-command-sender",
        "plugin-context",
        "plugin-manager",
        "ban-list",
        "permission-attachment",
        "permissible",
        "permission-definition",
        "tasks",
        "scheduler",
        "service-bus",
        "player-form",
        "form",
        "map-renderer",
        "level",
        "dimension",
        "block",
        "block-data",
        "block-state",
        "block-type",
        "inventory",
        "item-type",
        "player-inventory",
        "item-stack",
        "item-actor",
        "boss-bar",
        "map-view",
        "scoreboard",
        "score-entry",
        "objective",
        "player-join-event",
        "player-quit-event",
        "player-chat-event",
        "player-kick-event",
        "player-command-event",
        "player-game-mode-change-event",
        "player-drop-item-event",
        "player-bed-enter-event",
        "player-bed-leave-event",
        "player-dimension-change-event",
        "player-emote-event",
        "player-interact-event",
        "player-interact-actor-event",
        "player-item-consume-event",
        "player-item-held-event",
        "player-login-event",
        "player-move-event",
        "player-pickup-item-event",
        "player-respawn-event",
        "player-skin-change-event",
        "player-death-event",
        "actor-damage-event",
        "actor-death-event",
        "actor-explode-event",
        "actor-knockback-event",
        "actor-remove-event",
        "actor-spawn-event",
        "actor-teleport-event",
        "block-break-event",
        "block-place-event",
        "block-cook-event",
        "block-explode-event",
        "block-form-event",
        "block-from-to-event",
        "block-grow-event",
        "block-piston-extend-event",
        "block-piston-retract-event",
        "leaves-decay-event",
        "chunk-load-event",
        "chunk-unload-event",
        "weather-change-event",
        "thunder-change-event",
        "server-command-event",
        "server-list-ping-event",
        "server-load-event",
        "broadcast-message-event",
        "script-message-event",
        "map-initialize-event",
        "plugin-enable-event",
        "plugin-disable-event",
        "packet-send-event",
        "packet-receive-event",
        "nbt",
        "book-meta",
        "crossbow-meta",
        "item-factory",
        "item-meta",
        "language",
        "map-meta",
        "plugin-command",
        "writable-book-meta",
        "player-jump-event",
        "player-portal-event",
        "player-teleport-event",
    ];
    fs::write(
        output.join("store_denied_impls.rs"),
        bindgen.generate_store_impls("PluginStoreState", EXCLUDED),
    )
    .expect("failed to write generated store trait impls");
}
fn build_test_stub(root: &std::path::Path, output: &std::path::Path) {
    let build = output.join("aegilex-test-stub");
    let native = root.join("native");
    let mut configure = Command::new("cmake");
    configure
        .arg("-S")
        .arg(&native)
        .arg("-B")
        .arg(&build)
        .args(["-G", "Ninja"])
        .args([
            "-DAEGILEX_BUILD_PLUGIN=OFF",
            "-DCMAKE_BUILD_TYPE=Debug",
            "-DCMAKE_MSVC_RUNTIME_LIBRARY=MultiThreadedDLL",
        ])
        .arg(format!(
            "-DAEGILEX_RUNTIME_CXX_INCLUDE={}",
            output.join("cxxbridge/include").display()
        ));
    if cfg!(windows) {
        configure
            .arg("-DCMAKE_CXX_COMPILER=clang-cl")
            .env("CXX", "clang-cl");
    }
    run(&mut configure, "configure the native test stub");

    let mut compile = Command::new("cmake");
    compile
        .arg("--build")
        .arg(&build)
        .args(["--target", "aegilex_test_stub"]);
    run(&mut compile, "build the native test stub");

    println!(
        "cargo::rustc-link-search=native={}",
        build.join("lib").display()
    );
    println!(
        "cargo::rerun-if-changed={}",
        native.join("CMakeLists.txt").display()
    );
    for relative in [
        "host_context.h",
        "bridge/form_bridge.h",
        "bridge/task_bridge.h",
        "bridge/map_renderer_bridge.h",
        "test_stub/test_stub_actor.cpp",
        "test_stub/test_stub_item_actor.cpp",
        "test_stub/test_stub_player.cpp",
        "test_stub/test_stub_inventory.cpp",
        "test_stub/test_stub_level.cpp",
        "test_stub/test_stub_server.cpp",
        "test_stub/test_stub_ui.cpp",
        "test_stub/test_stub_admin.cpp",
        "test_stub/test_stub_common.cpp",
        "test_stub/test_stub_logger.cpp",
        "test_stub/test_stub_event.cpp",
        "test_stub/test_stub_event.h",
        "bindings/endstone/actor/actor.h",
        "bindings/endstone/actor/item_actor.h",
        "bindings/endstone/actor/mob.h",
        "bindings/endstone/actor/player.h",
        "bindings/endstone/ban/ban_list.h",
        "bindings/endstone/boss/boss_bar.h",
        "bindings/endstone/command_sender.h",
        "bindings/endstone/inventory/inventory.h",
        "bindings/endstone/inventory/item_stack.h",
        "bindings/endstone/inventory/item_type.h",
        "bindings/endstone/inventory/player_inventory.h",
        "bindings/endstone/inventory/item_meta.h",
        "bindings/endstone/level/level.h",
        "bindings/endstone/level/dimension.h",
        "bindings/endstone/level/chunk.h",
        "bindings/endstone/level/block.h",
        "bindings/endstone/map/map.h",
        "bindings/endstone/permissions/permissible.h",
        "bindings/endstone/permissions/permission.h",
        "bindings/endstone/permissions/permission_definition.h",
        "bindings/endstone/permissions/permission_attachment.h",
        "bindings/endstone/scoreboard/scoreboard.h",
        "bindings/endstone/scoreboard/objective.h",
        "bindings/endstone/server.h",
        "bindings/endstone/plugin.h",
        "bindings/endstone/logger.h",
        "bindings/endstone/events/player_chat_event_facade.h",
        "bindings/endstone/events/player_join_event_facade.h",
        "bindings/endstone/events/player_quit_event_facade.h",
        "bindings/endstone/events/actor_damage_event_facade.h",
        "bindings/endstone/events/actor_death_event_facade.h",
        "bindings/endstone/events/block_explode_event_facade.h",
        "bindings/endstone/events/actor_explode_event_facade.h",
        "bindings/endstone/events/actor_knockback_event_facade.h",
        "bindings/endstone/events/actor_remove_event_facade.h",
        "bindings/endstone/events/plugin_lifecycle_event_facade.h",
        "bindings/endstone/events/server_load_event_facade.h",
        "bindings/endstone/events/actor_spawn_event_facade.h",
        "bindings/endstone/events/actor_teleport_event_facade.h",
        "bindings/endstone/events/broadcast_message_event_facade.h",
        "bindings/endstone/events/packet_send_event_facade.h",
        "bindings/endstone/events/packet_receive_event_facade.h",
        "bindings/endstone/events/map_initialize_event_facade.h",
        "bindings/endstone/events/script_message_event_facade.h",
        "bindings/endstone/events/player_command_event_facade.h",
        "bindings/endstone/events/player_kick_event_facade.h",
        "bindings/endstone/events/player_login_event_facade.h",
        "bindings/endstone/events/player_game_mode_change_event_facade.h",
        "bindings/endstone/events/player_emote_event_facade.h",
        "bindings/endstone/events/player_dimension_change_event_facade.h",
        "bindings/endstone/events/player_bed_enter_event_facade.h",
        "bindings/endstone/events/player_bed_leave_event_facade.h",
        "bindings/endstone/events/player_respawn_event_facade.h",
        "bindings/endstone/events/player_item_held_event_facade.h",
        "bindings/endstone/events/player_move_event_facade.h",
        "bindings/endstone/events/player_drop_item_event_facade.h",
        "bindings/endstone/events/block_cook_event_facade.h",
        "bindings/endstone/events/block_from_to_event_facade.h",
        "bindings/endstone/events/block_grow_event_facade.h",
        "bindings/endstone/events/block_piston_event_facade.h",
        "bindings/endstone/events/leaves_decay_event_facade.h",
        "bindings/endstone/events/player_interact_event_facade.h",
        "bindings/endstone/events/player_interact_actor_event_facade.h",
        "bindings/endstone/events/player_item_consume_event_facade.h",
        "bindings/endstone/events/player_pickup_item_event_facade.h",
        "bindings/endstone/events/server_command_event_facade.h",
        "bindings/endstone/events/server_list_ping_event_facade.h",
        "bindings/endstone/events/weather_change_event_facade.h",
        "bindings/endstone/events/thunder_change_event_facade.h",
    ] {
        println!(
            "cargo::rerun-if-changed={}",
            native.join(relative).display()
        );
    }
}

fn run(command: &mut Command, action: &str) {
    let status = command
        .status()
        .unwrap_or_else(|error| panic!("failed to {action}: {error}"));
    assert!(status.success(), "failed to {action}: {status}");
}
