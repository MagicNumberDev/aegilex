wit_bindgen::generate!({
    path: "wit",
    world: "plugin",
});

use aegilex::endstone::{
    logger, permission_default, plugin_metadata, task, types,
};
use exports::aegilex::endstone::{commands as exported_commands, events as exported_events};

struct HelloPlugin;

impl Guest for HelloPlugin {
    fn metadata() -> plugin_metadata::Metadata {
        plugin_metadata::Metadata {
            name: "example_hello".to_owned(),
            version: "0.1.0".to_owned(),
            description: "Hello-world Wasm component for Aegilex.".to_owned(),
            load_order: plugin_metadata::LoadOrder::PostWorld,
            authors: vec!["Aegilex Contributors".to_owned()],
            contributors: Vec::new(),
            website: String::new(),
            prefix: "Hello".to_owned(),
            provides: Vec::new(),
            depend: Vec::new(),
            soft_depend: Vec::new(),
            load_before: Vec::new(),
            default_permission: permission_default::PermissionDefault::Operator,
            commands: vec![plugin_metadata::Command {
                name: "hello".to_owned(),
                description: Some("Reply with a greeting.".to_owned()),
                aliases: vec!["hi".to_owned()],
                usages: vec!["/hello".to_owned()],
                permissions: vec!["aegilex.example.hello".to_owned()],
            }],
            permissions: vec![plugin_metadata::PluginPermission {
                name: "aegilex.example.hello".to_owned(),
                description: Some("Allow using the /hello command.".to_owned()),
                default_value: Some(permission_default::PermissionDefault::Operator),
                children: Vec::new(),
            }],
            subscriptions: vec!["player-join".to_owned(), "player-kick".to_owned()],
        }
    }

    fn on_load() -> Result<(), String> { Ok(()) }

    fn on_enable() -> Result<(), String> {
        log(
            logger::LogLevel::Info,
            "Hello from the Aegilex Wasm component.",
        )
        .map_err(|error| format!("host logging failed: {error:?}"))
    }

    fn on_disable() {}
}

impl exports::aegilex::endstone::events::Guest for HelloPlugin {
    fn on_event(event: exported_events::Event<'_>) -> Result<(), String> {
        if let exported_events::Event::PlayerJoin(join) = event {
            let player = join
                .get_player()
                .map_err(|error| format!("player-join.get-player failed: {error:?}"))?;
            let player_name = player
                .get_name()
                .map_err(|error| format!("player.get-name failed: {error:?}"))?;
            log(
                logger::LogLevel::Info,
                &format!("Event listener test: {player_name} joined the server."),
            )
            .map_err(|error| format!("player-join event logging failed: {error:?}"))?;
        }
        Ok(())
    }
}

impl exports::aegilex::endstone::commands::Guest for HelloPlugin {
    fn on_command(
        command: exported_commands::Invocation,
    ) -> Result<exported_commands::Outcome, String> {
        if command.subcommand == "hello" {
            Ok(exported_commands::Outcome {
                handled: true,
                reply: Some("Hello from the Aegilex Wasm component.".to_owned()),
                error: None,
            })
        } else {
            Ok(exported_commands::Outcome {
                handled: false,
                reply: None,
                error: None,
            })
        }
    }
}

impl exports::aegilex::endstone::tasks::Guest for HelloPlugin {
    fn schedule_now() -> Result<u64, types::HostError> {
        Err(types::HostError::Denied)
    }

    fn schedule_after(_delay_ticks: u64) -> Result<u64, types::HostError> {
        Err(types::HostError::Denied)
    }

    fn schedule_every(
        _initial_delay_ticks: u64,
        _period_ticks: u64,
    ) -> Result<u64, types::HostError> {
        Err(types::HostError::Denied)
    }

    fn cancel(_task_id: u64) -> Result<(), types::HostError> {
        Err(types::HostError::Denied)
    }

    fn get_task(_task_id: u64) -> Result<task::Task, types::HostError> {
        Err(types::HostError::Denied)
    }

    fn task_is_running(_task_id: u64) -> Result<bool, types::HostError> {
        Err(types::HostError::Denied)
    }

    fn task_is_queued(_task_id: u64) -> Result<bool, types::HostError> {
        Err(types::HostError::Denied)
    }

    fn task_list_pending() -> Result<Vec<task::Task>, types::HostError> {
        Err(types::HostError::Denied)
    }

    fn on_task(_task_id: u64) -> Result<(), String> {
        Ok(())
    }
}

impl exports::aegilex::endstone::form_callbacks::Guest for HelloPlugin {
    fn on_form_submit(
        _form: &aegilex::endstone::form::Form,
        _player: &aegilex::endstone::player::Player,
        _response: exports::aegilex::endstone::form_callbacks::FormResponse,
    ) -> Result<(), String> {
        Ok(())
    }

    fn on_form_close(
        _form: &aegilex::endstone::form::Form,
        _player: &aegilex::endstone::player::Player,
    ) -> Result<(), String> {
        Ok(())
    }
}

impl exports::aegilex::endstone::service_callbacks::Guest for HelloPlugin {
    fn on_service_request(
        _request: exports::aegilex::endstone::service_callbacks::ServiceRequest,
    ) -> Result<exports::aegilex::endstone::service_callbacks::ServiceResponse, String> {
        Err("not implemented by hello-component".to_owned())
    }
}

impl exports::aegilex::endstone::map_renderer_callbacks::Guest for HelloPlugin {
    fn on_map_initialize(
        _renderer: &aegilex::endstone::map_renderer::MapRenderer,
        _map_id: i64,
    ) -> Result<(), String> {
        Ok(())
    }

    fn on_map_render(
        _renderer: &aegilex::endstone::map_renderer::MapRenderer,
        _map_id: i64,
        _player: &aegilex::endstone::player::Player,
    ) -> Result<Vec<aegilex::endstone::map_canvas::MapDrawCommand>, String> {
        Ok(Vec::new())
    }
}

fn log(level: logger::LogLevel, message: &str) -> Result<(), String> {
    let logger =
        logger::get_logger().map_err(|error| format!("get logger failed: {error:?}"))?;
    logger
        .log(level, message)
        .map_err(|error| format!("host logging failed: {error:?}"))
}

export!(HelloPlugin);
