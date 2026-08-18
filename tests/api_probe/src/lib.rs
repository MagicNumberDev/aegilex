wit_bindgen::generate!({
    path: "wit",
    world: "plugin",
});

use aegilex::endstone::{
    form, item_type, level, logger, map_canvas, map_renderer, message, nbt, permission_default,
    player, plugin_context, plugin_metadata, server, task, types,
};
use exports::aegilex::endstone::{
    commands as exported_commands, events as exported_events,
    form_callbacks as exported_form_callbacks, service_callbacks as exported_service_callbacks,
};

struct ApiProbe;

impl Guest for ApiProbe {
    fn metadata() -> plugin_metadata::Metadata {
        plugin_metadata::Metadata {
            name: "api_probe".to_owned(),
            version: "0.1.0".to_owned(),
            description: "Resource-first Aegilex host API probe.".to_owned(),
            load_order: plugin_metadata::LoadOrder::PostWorld,
            authors: vec!["Aegilex Contributors".to_owned()],
            contributors: Vec::new(),
            website: String::new(),
            prefix: "API Probe".to_owned(),
            provides: Vec::new(),
            depend: Vec::new(),
            soft_depend: Vec::new(),
            load_before: Vec::new(),
            default_permission: permission_default::PermissionDefault::Operator,
            commands: vec![plugin_metadata::Command {
                name: "ping".to_owned(),
                description: Some("Reply with pong.".to_owned()),
                aliases: Vec::new(),
                usages: vec!["/ping".to_owned()],
                permissions: vec!["aegilex.probe.ping".to_owned()],
            }],
            permissions: vec![plugin_metadata::PluginPermission {
                name: "aegilex.probe.ping".to_owned(),
                description: Some("Allow using the /ping command.".to_owned()),
                default_value: Some(permission_default::PermissionDefault::Operator),
                children: Vec::new(),
            }],
            subscriptions: vec!["player-join".to_owned()],
        }
    }

    fn on_load() -> Result<(), String> {
        let name = plugin_context::get_name()
            .map_err(|error| format!("plugin-context.get-name failed: {error:?}"))?;
        if name != "api_probe" {
            return Err(format!("plugin context reported unexpected name: {name}"));
        }
        Ok(())
    }

    fn on_enable() -> Result<(), String> {
        log("api_probe: starting resource-first host API checks")?;

        if !plugin_context::list_commands()
            .map_err(|error| format!("plugin-context.list-commands failed: {error:?}"))?
            .iter()
            .any(|command| command == "ping")
        {
            return Err("plugin context does not include the declared ping command".to_owned());
        }

        let server_name = server::get_name()
            .map_err(|error| format!("server.get-name failed: {error:?}"))?;
        if server_name.is_empty() {
            return Err("server returned an empty name".to_owned());
        }

        let world = level::get_level()
            .map_err(|error| format!("level.get-level failed: {error:?}"))?;
        if world.name.is_empty() {
            return Err("level returned an empty name".to_owned());
        }

        let stack = item_type::create_item_stack("minecraft:apple", Some(1))
            .map_err(|error| format!("item-type.create-item-stack failed: {error:?}"))?;
        if stack
            .get_type_id()
            .map_err(|error| format!("item-stack.get-type-id failed: {error:?}"))?
            != "minecraft:apple"
        {
            return Err("item stack type did not match its requested type".to_owned());
        }
        let answer = nbt::from_int(42)
            .map_err(|error| format!("nbt.from-int failed: {error:?}"))?;
        let root = nbt::from_compound(&[nbt::CompoundEntry {
            key: "answer".to_owned(),
            value: &answer,
        }])
        .map_err(|error| format!("nbt.from-compound failed: {error:?}"))?;
        stack
            .set_nbt(&root)
            .map_err(|error| format!("item-stack.set-nbt failed: {error:?}"))?;
        let root = stack
            .get_nbt()
            .map_err(|error| format!("item-stack.get-nbt failed: {error:?}"))?;
        if root
            .get_type()
            .map_err(|error| format!("nbt.get-type failed: {error:?}"))?
            != nbt::TagType::Compound
        {
            return Err("item stack NBT root is not a compound".to_owned());
        }
        let value = root
            .get_compound("answer")
            .map_err(|error| format!("nbt.get-compound failed: {error:?}"))?
            .ok_or_else(|| "item stack NBT missing the answer key".to_owned())?;
        if value
            .get_int()
            .map_err(|error| format!("nbt.get-int failed: {error:?}"))?
            != 42
        {
            return Err("item stack NBT answer is not 42".to_owned());
        }

        for actor in level::list_actors(None)
            .map_err(|error| format!("level.list-actors failed: {error:?}"))?
        {
            actor
                .get_actor_name()
                .map_err(|error| format!("actor.get-actor-name failed: {error:?}"))?;
            if let Some(mob) = actor
                .as_mob()
                .map_err(|error| format!("actor.as-mob failed: {error:?}"))?
            {
                mob.as_actor()
                    .map_err(|error| format!("mob.as-actor failed: {error:?}"))?;
            }
            if let Some(player) = actor
                .as_player()
                .map_err(|error| format!("actor.as-player failed: {error:?}"))?
            {
                player
                    .as_actor()
                    .map_err(|error| format!("player.as-actor failed: {error:?}"))?;
            }
            if let Some(item_actor) = actor
                .as_item_actor()
                .map_err(|error| format!("actor.as-item-actor failed: {error:?}"))?
            {
                item_actor
                    .as_actor()
                    .map_err(|error| format!("item-actor.as-actor failed: {error:?}"))?;
                let item = item_actor
                    .get_item_stack()
                    .map_err(|error| format!("item-actor.get-item-stack failed: {error:?}"))?;
                item.get_amount()
                    .map_err(|error| format!("item-stack.get-amount failed: {error:?}"))?;
            }
        }

        let broadcast = message::Message::PlainText("api_probe: host API checks passed".to_owned());
        server::broadcast(&broadcast, None)
            .map_err(|error| format!("server.broadcast failed: {error:?}"))?;
        log("api_probe: resource-first host API checks passed")
    }

    fn on_disable() {}
}

impl exports::aegilex::endstone::events::Guest for ApiProbe {
    fn on_event(_event: exported_events::Event<'_>) -> Result<(), String> {
        Ok(())
    }
}

impl exports::aegilex::endstone::commands::Guest for ApiProbe {
    fn on_command(
        command: exported_commands::Invocation,
    ) -> Result<exported_commands::Outcome, String> {
        if command.subcommand == "ping" {
            Ok(exported_commands::Outcome {
                handled: true,
                reply: Some("pong".to_owned()),
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

impl exports::aegilex::endstone::tasks::Guest for ApiProbe {
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

impl exports::aegilex::endstone::form_callbacks::Guest for ApiProbe {
    fn on_form_submit(
        _form: &form::Form,
        _player: &player::Player,
        _response: exported_form_callbacks::FormResponse,
    ) -> Result<(), String> {
        Ok(())
    }

    fn on_form_close(_form: &form::Form, _player: &player::Player) -> Result<(), String> {
        Ok(())
    }
}

impl exports::aegilex::endstone::service_callbacks::Guest for ApiProbe {
    fn on_service_request(
        _request: exported_service_callbacks::ServiceRequest,
    ) -> Result<exported_service_callbacks::ServiceResponse, String> {
        Ok(exported_service_callbacks::ServiceResponse::Rejected(
            "api probe does not provide services".to_owned(),
        ))
    }
}

impl exports::aegilex::endstone::map_renderer_callbacks::Guest for ApiProbe {
    fn on_map_initialize(
        _renderer: &map_renderer::MapRenderer,
        _map_id: i64,
    ) -> Result<(), String> {
        Ok(())
    }

    fn on_map_render(
        _renderer: &map_renderer::MapRenderer,
        _map_id: i64,
        _player: &player::Player,
    ) -> Result<Vec<map_canvas::MapDrawCommand>, String> {
        Ok(Vec::new())
    }
}

fn log(message: &str) -> Result<(), String> {
    let logger =
        logger::get_logger().map_err(|error| format!("logger.get-logger failed: {error:?}"))?;
    logger
        .log(logger::LogLevel::Info, message)
        .map_err(|error| format!("logger.log failed: {error:?}"))
}

export!(ApiProbe);
