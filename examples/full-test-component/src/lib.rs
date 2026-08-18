wit_bindgen::generate!({
    path: "wit",
    world: "plugin",
});

use aegilex::endstone::{
    actor, ban_list, boss_bar, criteria, form, form_button, form_divider, form_header, form_label,
    game_mode, item_type, language, level, location, logger, map_canvas, map_cursor, map_renderer,
    map_view, message, nbt, permission_default, player, player_form, plugin_manager,
    plugin_metadata, score_entry, scoreboard, server, service_bus, service_priority, task, tasks,
    translatable, types,
};
use exports::aegilex::endstone::{
    commands as exported_commands, events as exported_events,
    form_callbacks as exported_form_callbacks, service_callbacks as exported_service_callbacks,
};

use parking_lot::Mutex;

struct TestState {
    passed: u32,
    failed: u32,
    failures: Vec<String>,
    task_fired: bool,
    form_submitted: bool,
    form_closed: bool,
    map_rendered: bool,
}

static STATE: Mutex<TestState> = Mutex::new(TestState {
    passed: 0,
    failed: 0,
    failures: Vec::new(),
    task_fired: false,
    form_submitted: false,
    form_closed: false,
    map_rendered: false,
});

struct FullTestPlugin;

impl Guest for FullTestPlugin {
    fn metadata() -> plugin_metadata::Metadata {
        plugin_metadata::Metadata {
            name: "full_test".to_owned(),
            version: "0.1.0".to_owned(),
            description: "End-to-end host API test component for Aegilex.".to_owned(),
            load_order: plugin_metadata::LoadOrder::PostWorld,
            authors: vec!["Aegilex Contributors".to_owned()],
            contributors: Vec::new(),
            website: String::new(),
            prefix: "FullTest".to_owned(),
            provides: Vec::new(),
            depend: Vec::new(),
            soft_depend: Vec::new(),
            load_before: Vec::new(),
            default_permission: permission_default::PermissionDefault::Operator,
            commands: vec![plugin_metadata::Command {
                name: "fulltest".to_owned(),
                description: Some("Run the Aegilex full-test component.".to_owned()),
                aliases: Vec::new(),
                usages: vec!["/fulltest".to_owned()],
                permissions: vec!["aegilex.fulltest.command.fulltest".to_owned()],
            }],
            permissions: vec![plugin_metadata::PluginPermission {
                name: "aegilex.fulltest.command.fulltest".to_owned(),
                description: Some("Allow using the /fulltest command.".to_owned()),
                default_value: Some(permission_default::PermissionDefault::Operator),
                children: Vec::new(),
            }],
            subscriptions: vec![
                "player-login".to_owned(),
                "player-join".to_owned(),
                "player-emote".to_owned(),
                "player-interact".to_owned(),
                "player-interact-actor".to_owned(),
                "player-kick".to_owned(),
                "player-command".to_owned(),
                "player-quit".to_owned(),
                "player-chat".to_owned(),
                "player-game-mode-change".to_owned(),
                "player-jump".to_owned(),
                "player-move".to_owned(),
                "player-teleport".to_owned(),
                "player-portal".to_owned(),
                "player-death".to_owned(),
                "player-respawn".to_owned(),
                "player-item-consume".to_owned(),
                "player-item-held".to_owned(),
                "player-drop-item".to_owned(),
                "player-pickup-item".to_owned(),
                "player-bed-enter".to_owned(),
                "player-bed-leave".to_owned(),
                "player-skin-change".to_owned(),
                "player-dimension-change".to_owned(),
                "actor-damage".to_owned(),
                "actor-death".to_owned(),
                "actor-explode".to_owned(),
                "actor-knockback".to_owned(),
                "actor-remove".to_owned(),
                "actor-spawn".to_owned(),
                "actor-teleport".to_owned(),
                "block-break".to_owned(),
                "block-cook".to_owned(),
                "block-place".to_owned(),
                "block-form".to_owned(),
                "block-from-to".to_owned(),
                "block-grow".to_owned(),
                "block-piston-extend".to_owned(),
                "block-piston-retract".to_owned(),
                "leaves-decay".to_owned(),
                "chunk-load".to_owned(),
                "chunk-unload".to_owned(),
                "weather-change".to_owned(),
                "thunder-change".to_owned(),
                "server-command".to_owned(),
                "broadcast-message".to_owned(),
                "server-list-ping".to_owned(),
                "plugin-enable".to_owned(),
                "plugin-disable".to_owned(),
                "server-load".to_owned(),
                "packet-send".to_owned(),
                "packet-receive".to_owned(),
                "map-initialize".to_owned(),
                "script-message".to_owned(),
            ],
        }
    }

    fn on_load() -> Result<(), String> {
        Ok(())
    }

    fn on_enable() -> Result<(), String> {
        log("full_test: on_enable is called!")?;
        run_server_tests()?;
        test_server_external_coverage()
    }

    fn on_disable() {
        let _ = log("full_test: on_disable is called!");
        let state = STATE.lock();
        let _ = log(&format!(
            "full_test summary: {} passed, {} failed{}",
            state.passed,
            state.failed,
            if state.failures.is_empty() {
                String::new()
            } else {
                format!("; failures: {}", state.failures.join(", "))
            }
        ));
    }
}

impl exports::aegilex::endstone::commands::Guest for FullTestPlugin {
    fn on_command(
        command: exported_commands::Invocation,
    ) -> Result<exported_commands::Outcome, String> {
        if command.subcommand != "fulltest" {
            return Ok(exported_commands::Outcome {
                handled: false,
                reply: None,
                error: None,
            });
        }
        let state = STATE.lock();
        let reply = format!(
            "Aegilex full-test: {} passed, {} failed; task={}, form-submit={}, form-close={}, map-render={}{}",
            state.passed,
            state.failed,
            state.task_fired,
            state.form_submitted,
            state.form_closed,
            state.map_rendered,
            if state.failures.is_empty() {
                String::new()
            } else {
                format!("; failures: {}", state.failures.join(", "))
            }
        );
        Ok(exported_commands::Outcome {
            handled: true,
            reply: Some(reply),
            error: None,
        })
    }
}

impl exports::aegilex::endstone::tasks::Guest for FullTestPlugin {
    fn schedule_now() -> Result<u64, types::HostError> {
        tasks::schedule_now()
    }

    fn schedule_after(delay_ticks: u64) -> Result<u64, types::HostError> {
        tasks::schedule_after(delay_ticks)
    }

    fn schedule_every(
        initial_delay_ticks: u64,
        period_ticks: u64,
    ) -> Result<u64, types::HostError> {
        tasks::schedule_every(initial_delay_ticks, period_ticks)
    }

    fn cancel(task_id: u64) -> Result<(), types::HostError> {
        tasks::cancel(task_id)
    }

    fn get_task(task_id: u64) -> Result<task::Task, types::HostError> {
        tasks::get_task(task_id)
    }

    fn task_is_running(task_id: u64) -> Result<bool, types::HostError> {
        tasks::task_is_running(task_id)
    }

    fn task_is_queued(task_id: u64) -> Result<bool, types::HostError> {
        tasks::task_is_queued(task_id)
    }

    fn task_list_pending() -> Result<Vec<task::Task>, types::HostError> {
        tasks::task_list_pending()
    }

    fn on_task(task_id: u64) -> Result<(), String> {
        STATE.lock().task_fired = true;
        log(&format!("full_test: scheduled task {task_id} fired."))
    }
}

impl exports::aegilex::endstone::form_callbacks::Guest for FullTestPlugin {
    fn on_form_submit(
        form: &form::Form,
        player: &player::Player,
        response: exported_form_callbacks::FormResponse,
    ) -> Result<(), String> {
        STATE.lock().form_submitted = true;
        let title = form
            .get_title()
            .map_err(|error| format!("form.get-title failed: {error:?}"))?;
        let player = player
            .get_name()
            .map_err(|error| format!("player.get-name failed: {error:?}"))?;
        let response = match response {
            exported_form_callbacks::FormResponse::Action(action) => {
                format!("action button {}", action.selected_index)
            }
            exported_form_callbacks::FormResponse::Message(message_form) => {
                format!("message {:?}", message_form)
            }
            exported_form_callbacks::FormResponse::Modal(modal) => {
                format!("modal json={}", modal.json)
            }
        };
        log(&format!(
            "full_test: {player} submitted form '{}': {response}",
            message_text(&title)
        ))
    }

    fn on_form_close(form: &form::Form, player: &player::Player) -> Result<(), String> {
        STATE.lock().form_closed = true;
        let title = form
            .get_title()
            .map_err(|error| format!("form.get-title failed: {error:?}"))?;
        let player = player
            .get_name()
            .map_err(|error| format!("player.get-name failed: {error:?}"))?;
        log(&format!(
            "full_test: {player} closed form '{}'.",
            message_text(&title)
        ))
    }
}

impl exports::aegilex::endstone::service_callbacks::Guest for FullTestPlugin {
    fn on_service_request(
        request: exported_service_callbacks::ServiceRequest,
    ) -> Result<exported_service_callbacks::ServiceResponse, String> {
        log(&format!(
            "full_test: service request {}.{} (call {})",
            request.provider_id, request.method, request.call_id
        ))
        .map_err(|error| error)?;
        if request.method == "echo" {
            Ok(exported_service_callbacks::ServiceResponse::Success(
                request.payload,
            ))
        } else if request.method == "reject" {
            Ok(exported_service_callbacks::ServiceResponse::Rejected(
                "rejected by full_test".to_owned(),
            ))
        } else {
            Err(format!("unknown method {}", request.method))
        }
    }
}

impl exports::aegilex::endstone::map_renderer_callbacks::Guest for FullTestPlugin {
    fn on_map_initialize(renderer: &map_renderer::MapRenderer, map_id: i64) -> Result<(), String> {
        let renderer_id = renderer
            .get_map_id()
            .map_err(|error| format!("map-renderer.get-map-id failed: {error:?}"))?;
        log(&format!(
            "full_test: map {map_id} initialized renderer {renderer_id}"
        ))
    }

    fn on_map_render(
        renderer: &map_renderer::MapRenderer,
        map_id: i64,
        player: &player::Player,
    ) -> Result<Vec<map_canvas::MapDrawCommand>, String> {
        STATE.lock().map_rendered = true;
        let contextual = renderer
            .is_contextual()
            .map_err(|error| format!("map-renderer.is-contextual failed: {error:?}"))?;
        let player = player
            .get_name()
            .map_err(|error| format!("player.get-name failed: {error:?}"))?;
        log(&format!(
            "full_test: rendering map {map_id} for {player} (contextual={contextual})"
        ))?;
        Ok(vec![
            map_canvas::MapDrawCommand::SetPixel(map_canvas::MapPixel {
                x: 0,
                y: 0,
                argb: 0xFF00FF00,
            }),
            map_canvas::MapDrawCommand::FillRect(map_canvas::MapRect {
                x: 1,
                y: 1,
                width: 4,
                height: 4,
                argb: 0xFF0000FF,
            }),
            map_canvas::MapDrawCommand::DrawImage(map_canvas::MapImageArgb {
                x: 8,
                y: 8,
                width: 2,
                height: 2,
                pixels: vec![0xFFFF0000, 0xFF00FF00, 0xFF0000FF, 0xFFFFFFFF],
            }),
            map_canvas::MapDrawCommand::SetCursors(vec![map_cursor::MapCursor {
                x: 4,
                y: 4,
                direction: 0,
                cursor_type: map_cursor::MapCursorType::RedMarker,
                visible: true,
                caption: "full_test".to_owned(),
            }]),
        ])
    }
}

impl exports::aegilex::endstone::events::Guest for FullTestPlugin {
    fn on_event(event: exported_events::Event<'_>) -> Result<(), String> {
        match event {
            exported_events::Event::PlayerLogin(login) => {
                let player = login
                    .get_player()
                    .map_err(|error| format!("player-login.get-player failed: {error:?}"))?;
                let message = login
                    .get_kick_message()
                    .map_err(|error| format!("player-login.get-kick-message failed: {error:?}"))?;
                log(&format!(
                    "{} logged in. kick message: {}",
                    player_name(&player)?,
                    message
                ))?;
                login
                    .set_kick_message(&message)
                    .map_err(|error| format!("player-login.set-kick-message failed: {error:?}"))?;
                let cancelled = login
                    .is_cancelled()
                    .map_err(|error| format!("player-login.is-cancelled failed: {error:?}"))?;
                login
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("player-login.set-cancelled failed: {error:?}"))?;
            }
            exported_events::Event::PlayerJoin(join) => {
                let player = join
                    .get_player()
                    .map_err(|error| format!("player-join.get-player failed: {error:?}"))?;
                let join_message = join
                    .get_join_message()
                    .map_err(|error| format!("player-join.get-join-message failed: {error:?}"))?;
                join.set_join_message(join_message.as_ref())
                    .map_err(|error| format!("player-join.set-join-message failed: {error:?}"))?;
                log(&format!("{} joined the server.", player_name(&player)?))?;
                run_player_tests(&player)?;
            }
            exported_events::Event::PlayerEmote(emote) => {
                let player = emote
                    .get_player()
                    .map_err(|error| format!("player-emote.get-player failed: {error:?}"))?;
                let emote_id = emote
                    .get_emote_id()
                    .map_err(|error| format!("player-emote.get-emote-id failed: {error:?}"))?;
                let muted = emote
                    .is_muted()
                    .map_err(|error| format!("player-emote.is-muted failed: {error:?}"))?;
                emote
                    .set_muted(muted)
                    .map_err(|error| format!("player-emote.set-muted failed: {error:?}"))?;
                let cancelled = emote
                    .is_cancelled()
                    .map_err(|error| format!("player-emote.is-cancelled failed: {error:?}"))?;
                emote
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("player-emote.set-cancelled failed: {error:?}"))?;
                log(&format!(
                    "{} sends an emote: {}",
                    player_name(&player)?,
                    emote_id
                ))?;
            }
            exported_events::Event::PlayerInteract(interact) => {
                let player = interact
                    .get_player()
                    .map_err(|error| format!("player-interact.get-player failed: {error:?}"))?;
                let action = interact
                    .get_action()
                    .map_err(|error| format!("player-interact.get-action failed: {error:?}"))?;
                let item = interact
                    .get_item()
                    .map_err(|error| format!("player-interact.get-item failed: {error:?}"))?
                    .map(|item| item.get_type_id())
                    .transpose()
                    .map_err(|error| format!("item-stack-ref.get-type-id failed: {error:?}"))?;
                let block = interact
                    .get_block()
                    .map_err(|error| format!("player-interact.get-block failed: {error:?}"))?
                    .map(|block| block.get_type())
                    .transpose()
                    .map_err(|error| format!("block.get-type failed: {error:?}"))?;
                let face = interact
                    .get_block_face()
                    .map_err(|error| format!("player-interact.get-block-face failed: {error:?}"))?;
                let clicked_position = interact.get_clicked_position().map_err(|error| {
                    format!("player-interact.get-clicked-position failed: {error:?}")
                })?;
                log(&format!(
                    "{} interact ({action:?}) with {item:?}/{block:?} (face={face:?}, position={clicked_position:?})",
                    player_name(&player)?,
                ))?;
                let cancelled = interact
                    .is_cancelled()
                    .map_err(|error| format!("player-interact.is-cancelled failed: {error:?}"))?;
                interact
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("player-interact.set-cancelled failed: {error:?}"))?;
            }
            exported_events::Event::PlayerInteractActor(interact) => {
                let player = interact.get_player().map_err(|error| {
                    format!("player-interact-actor.get-player failed: {error:?}")
                })?;
                let actor = interact.get_actor().map_err(|error| {
                    format!("player-interact-actor.get-actor failed: {error:?}")
                })?;
                log(&format!(
                    "{} interacts with actor {}",
                    player_name(&player)?,
                    actor_name(&actor)?
                ))?;
                let cancelled = interact.is_cancelled().map_err(|error| {
                    format!("player-interact-actor.is-cancelled failed: {error:?}")
                })?;
                interact.set_cancelled(cancelled).map_err(|error| {
                    format!("player-interact-actor.set-cancelled failed: {error:?}")
                })?;
            }
            exported_events::Event::PlayerKick(kick) => {
                let player = kick
                    .get_player()
                    .map_err(|error| format!("player-kick.get-player failed: {error:?}"))?;
                let reason = kick
                    .get_reason()
                    .map_err(|error| format!("player-kick.get-reason failed: {error:?}"))?;
                log(&format!(
                    "{} has been kicked due to {}",
                    player_name(&player)?,
                    reason
                ))?;
                kick.set_reason(&format!("**{reason}**"))
                    .map_err(|error| format!("player-kick.set-reason failed: {error:?}"))?;
                let cancelled = kick
                    .is_cancelled()
                    .map_err(|error| format!("player-kick.is-cancelled failed: {error:?}"))?;
                kick.set_cancelled(cancelled)
                    .map_err(|error| format!("player-kick.set-cancelled failed: {error:?}"))?;
            }
            exported_events::Event::PlayerCommand(command) => {
                let player = command
                    .get_player()
                    .map_err(|error| format!("player-command.get-player failed: {error:?}"))?;
                let command_text = command
                    .get_command()
                    .map_err(|error| format!("player-command.get-command failed: {error:?}"))?;
                log(&format!(
                    "{} executed command: {}",
                    player_name(&player)?,
                    command_text
                ))?;
                command
                    .set_command(&command_text)
                    .map_err(|error| format!("player-command.set-command failed: {error:?}"))?;
                let cancelled = command
                    .is_cancelled()
                    .map_err(|error| format!("player-command.is-cancelled failed: {error:?}"))?;
                command
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("player-command.set-cancelled failed: {error:?}"))?;
            }
            exported_events::Event::PlayerQuit(quit) => {
                let player = quit
                    .get_player()
                    .map_err(|error| format!("player-quit.get-player failed: {error:?}"))?;
                let quit_message = quit
                    .get_quit_message()
                    .map_err(|error| format!("player-quit.get-quit-message failed: {error:?}"))?;
                quit.set_quit_message(quit_message.as_ref())
                    .map_err(|error| format!("player-quit.set-quit-message failed: {error:?}"))?;
                log(&format!("{} quit", player_name(&player)?))?;
            }
            exported_events::Event::PlayerChat(chat) => {
                let player = chat
                    .get_player()
                    .map_err(|error| format!("player-chat.get-player failed: {error:?}"))?;
                let message = chat
                    .get_message()
                    .map_err(|error| format!("player-chat.get-message failed: {error:?}"))?;
                log(&format!("{} says: {}", player_name(&player)?, message))?;
            }
            exported_events::Event::PlayerGameModeChange(game_mode_change) => {
                let player = game_mode_change.get_player().map_err(|error| {
                    format!("player-game-mode-change.get-player failed: {error:?}")
                })?;
                let mode = game_mode_change.get_new_game_mode().map_err(|error| {
                    format!("player-game-mode-change.get-new-game-mode failed: {error:?}")
                })?;
                log(&format!(
                    "{} changed game mode to {:?}",
                    player_name(&player)?,
                    mode
                ))?;
                let cancelled = game_mode_change.is_cancelled().map_err(|error| {
                    format!("player-game-mode-change.is-cancelled failed: {error:?}")
                })?;
                game_mode_change.set_cancelled(cancelled).map_err(|error| {
                    format!("player-game-mode-change.set-cancelled failed: {error:?}")
                })?;
            }
            exported_events::Event::PlayerJump(jump) => {
                let player = jump
                    .get_player()
                    .map_err(|error| format!("player-jump.get-player failed: {error:?}"))?;
                let from = jump
                    .get_from()
                    .map_err(|error| format!("player-jump.get-from failed: {error:?}"))?;
                let to = jump
                    .get_to()
                    .map_err(|error| format!("player-jump.get-to failed: {error:?}"))?;
                log(&format!(
                    "{} jumps from {} to {}",
                    player_name(&player)?,
                    location(&from),
                    location(&to)
                ))?;
            }
            exported_events::Event::PlayerMove(move_event) => {
                let player = move_event
                    .get_player()
                    .map_err(|error| format!("player-move.get-player failed: {error:?}"))?;
                let from = move_event
                    .get_from()
                    .map_err(|error| format!("player-move.get-from failed: {error:?}"))?;
                let to = move_event
                    .get_to()
                    .map_err(|error| format!("player-move.get-to failed: {error:?}"))?;
                log(&format!(
                    "{} moves from {} to {}",
                    player_name(&player)?,
                    location(&from),
                    location(&to)
                ))?;
            }
            exported_events::Event::PlayerTeleport(teleport) => {
                let player = teleport
                    .get_player()
                    .map_err(|error| format!("player-teleport.get-player failed: {error:?}"))?;
                let from = teleport
                    .get_from()
                    .map_err(|error| format!("player-teleport.get-from failed: {error:?}"))?;
                let to = teleport
                    .get_to()
                    .map_err(|error| format!("player-teleport.get-to failed: {error:?}"))?;
                log(&format!(
                    "{} teleported from {} to {}",
                    player_name(&player)?,
                    location(&from),
                    location(&to)
                ))?;
            }
            exported_events::Event::PlayerPortal(portal) => {
                let player = portal
                    .get_player()
                    .map_err(|error| format!("player-portal.get-player failed: {error:?}"))?;
                let from = portal
                    .get_from()
                    .map_err(|error| format!("player-portal.get-from failed: {error:?}"))?;
                let to = portal
                    .get_to()
                    .map_err(|error| format!("player-portal.get-to failed: {error:?}"))?;
                log(&format!(
                    "{} teleported from {} to {} via portal",
                    player_name(&player)?,
                    location(&from),
                    location(&to)
                ))?;
            }
            exported_events::Event::PlayerDeath(death) => {
                let player = death
                    .get_player()
                    .map_err(|error| format!("player-death.get-player failed: {error:?}"))?;
                let death_message = death
                    .get_death_message()
                    .map_err(|error| format!("player-death.get-death-message failed: {error:?}"))?;
                death
                    .set_death_message(death_message.as_ref())
                    .map_err(|error| format!("player-death.set-death-message failed: {error:?}"))?;
                log(&format!(
                    "{} died{}",
                    player_name(&player)?,
                    death_message
                        .as_ref()
                        .map(|message| format!(": {}", message_text(message)))
                        .unwrap_or_default()
                ))?;
            }
            exported_events::Event::PlayerRespawn(respawn) => {
                let player = respawn
                    .get_player()
                    .map_err(|error| format!("player-respawn.get-player failed: {error:?}"))?;
                log(&format!("{} respawned.", player_name(&player)?))?;
            }
            exported_events::Event::PlayerItemConsume(consume) => {
                let player = consume
                    .get_player()
                    .map_err(|error| format!("player-item-consume.get-player failed: {error:?}"))?;
                let item = consume
                    .get_item()
                    .map_err(|error| format!("player-item-consume.get-item failed: {error:?}"))?;
                let type_id = item
                    .get_type_id()
                    .map_err(|error| format!("item-stack-ref.get-type-id failed: {error:?}"))?;
                let hand = consume
                    .get_hand()
                    .map_err(|error| format!("player-item-consume.get-hand failed: {error:?}"))?;
                log(&format!(
                    "{} consumes {} with {:?}.",
                    player_name(&player)?,
                    type_id,
                    hand
                ))?;
                let cancelled = consume.is_cancelled().map_err(|error| {
                    format!("player-item-consume.is-cancelled failed: {error:?}")
                })?;
                consume.set_cancelled(cancelled).map_err(|error| {
                    format!("player-item-consume.set-cancelled failed: {error:?}")
                })?;
            }
            exported_events::Event::PlayerItemHeld(held) => {
                let player = held
                    .get_player()
                    .map_err(|error| format!("player-item-held.get-player failed: {error:?}"))?;
                let previous_slot = held.get_previous_slot().map_err(|error| {
                    format!("player-item-held.get-previous-slot failed: {error:?}")
                })?;
                let new_slot = held
                    .get_new_slot()
                    .map_err(|error| format!("player-item-held.get-new-slot failed: {error:?}"))?;
                log(&format!(
                    "{} changes slot from {} to {}.",
                    player_name(&player)?,
                    previous_slot,
                    new_slot
                ))?;
                let cancelled = held
                    .is_cancelled()
                    .map_err(|error| format!("player-item-held.is-cancelled failed: {error:?}"))?;
                held.set_cancelled(cancelled)
                    .map_err(|error| format!("player-item-held.set-cancelled failed: {error:?}"))?;
            }
            exported_events::Event::PlayerDropItem(drop) => {
                let player = drop
                    .get_player()
                    .map_err(|error| format!("player-drop-item.get-player failed: {error:?}"))?;
                log(&format!("{} drops an item.", player_name(&player)?))?;
                let item = drop
                    .get_item()
                    .map_err(|error| format!("player-drop-item.get-item failed: {error:?}"))?;
                let type_id = item
                    .get_type_id()
                    .map_err(|error| format!("item-stack-ref.get-type-id failed: {error:?}"))?;
                if type_id == "minecraft:apple" {
                    drop.set_cancelled(true).map_err(|error| {
                        format!("player-drop-item.set-cancelled failed: {error:?}")
                    })?;
                }
            }
            exported_events::Event::PlayerPickupItem(pickup) => {
                let player = pickup
                    .get_player()
                    .map_err(|error| format!("player-pickup-item.get-player failed: {error:?}"))?;
                log(&format!("{} picks up an item.", player_name(&player)?))?;
                let actor = pickup.get_item_actor().map_err(|error| {
                    format!("player-pickup-item.get-item-actor failed: {error:?}")
                })?;
                if let Some(item_actor) = actor
                    .as_item_actor()
                    .map_err(|error| format!("actor.as-item-actor failed: {error:?}"))?
                {
                    let item = item_actor
                        .get_item_stack()
                        .map_err(|error| format!("item-actor.get-item-stack failed: {error:?}"))?;
                    item_actor
                        .set_item_stack(&item)
                        .map_err(|error| format!("item-actor.set-item-stack failed: {error:?}"))?;
                    log(&format!(
                        "picked up {}",
                        item.get_type_id()
                            .map_err(|error| format!("item-stack.get-type-id failed: {error:?}"))?
                    ))?;
                }
                let cancelled = pickup.is_cancelled().map_err(|error| {
                    format!("player-pickup-item.is-cancelled failed: {error:?}")
                })?;
                pickup.set_cancelled(cancelled).map_err(|error| {
                    format!("player-pickup-item.set-cancelled failed: {error:?}")
                })?;
            }
            exported_events::Event::PlayerBedEnter(bed) => {
                let player = bed
                    .get_player()
                    .map_err(|error| format!("player-bed-enter.get-player failed: {error:?}"))?;
                let bed_block = bed
                    .get_bed()
                    .map_err(|error| format!("player-bed-enter.get-bed failed: {error:?}"))?;
                let bed_type = bed_block
                    .get_type()
                    .map_err(|error| format!("block.get-type failed: {error:?}"))?;
                let cancelled = bed
                    .is_cancelled()
                    .map_err(|error| format!("player-bed-enter.is-cancelled failed: {error:?}"))?;
                bed.set_cancelled(cancelled)
                    .map_err(|error| format!("player-bed-enter.set-cancelled failed: {error:?}"))?;
                log(&format!(
                    "{} enters bed ({}).",
                    player_name(&player)?,
                    bed_type
                ))?;
            }
            exported_events::Event::PlayerBedLeave(bed) => {
                let player = bed
                    .get_player()
                    .map_err(|error| format!("player-bed-leave.get-player failed: {error:?}"))?;
                let bed_block = bed
                    .get_bed()
                    .map_err(|error| format!("player-bed-leave.get-bed failed: {error:?}"))?;
                let bed_type = bed_block
                    .get_type()
                    .map_err(|error| format!("block.get-type failed: {error:?}"))?;
                log(&format!(
                    "{} leaves bed ({}).",
                    player_name(&player)?,
                    bed_type
                ))?;
            }
            exported_events::Event::PlayerSkinChange(skin_change) => {
                let player = skin_change
                    .get_player()
                    .map_err(|error| format!("player-skin-change.get-player failed: {error:?}"))?;
                let message = skin_change.get_skin_change_message().map_err(|error| {
                    format!("player-skin-change.get-skin-change-message failed: {error:?}")
                })?;
                skin_change
                    .set_skin_change_message(message.as_ref())
                    .map_err(|error| {
                        format!("player-skin-change.set-skin-change-message failed: {error:?}")
                    })?;
                let cancelled = skin_change.is_cancelled().map_err(|error| {
                    format!("player-skin-change.is-cancelled failed: {error:?}")
                })?;
                skin_change.set_cancelled(cancelled).map_err(|error| {
                    format!("player-skin-change.set-cancelled failed: {error:?}")
                })?;
                log(&format!("{} changes skin.", player_name(&player)?,))?;
            }
            exported_events::Event::PlayerDimensionChange(dimension_change) => {
                let player = dimension_change.get_player().map_err(|error| {
                    format!("player-dimension-change.get-player failed: {error:?}")
                })?;
                let from = dimension_change.get_from_dimension().map_err(|error| {
                    format!("player-dimension-change.get-from-dimension failed: {error:?}")
                })?;
                let to = dimension_change.get_to_dimension().map_err(|error| {
                    format!("player-dimension-change.get-to-dimension failed: {error:?}")
                })?;
                log(&format!(
                    "{} has changed dimension from {} to {}.",
                    player_name(&player)?,
                    from,
                    to
                ))?;
            }
            exported_events::Event::ActorDamage(damage) => {
                let actor = damage
                    .get_actor()
                    .map_err(|error| format!("actor-damage.get-actor failed: {error:?}"))?;
                let amount = damage
                    .get_damage()
                    .map_err(|error| format!("actor-damage.get-damage failed: {error:?}"))?;
                log(&format!(
                    "{} hurt (damage: {}).",
                    actor_name(&actor)?,
                    amount
                ))?;
            }
            exported_events::Event::ActorDeath(death) => {
                let actor = death
                    .get_actor()
                    .map_err(|error| format!("actor-death.get-actor failed: {error:?}"))?;
                log(&format!("{} died.", actor_name(&actor)?))?;
            }
            exported_events::Event::ActorExplode(explode) => {
                let actor = explode
                    .get_actor()
                    .map_err(|error| format!("actor-explode.get-actor failed: {error:?}"))?;
                let location = explode
                    .get_location()
                    .map_err(|error| format!("actor-explode.get-location failed: {error:?}"))?;
                let blocks = explode
                    .get_block_list()
                    .map_err(|error| format!("actor-explode.get-block-list failed: {error:?}"))?;
                log(&format!(
                    "{} exploded at {:?}; {} blocks are in the read-only snapshot.",
                    actor_name(&actor)?,
                    location,
                    blocks.len()
                ))?;
            }
            exported_events::Event::ActorKnockback(knockback) => {
                let actor = knockback
                    .get_actor()
                    .map_err(|error| format!("actor-knockback.get-actor failed: {error:?}"))?;
                let source = knockback
                    .get_source()
                    .map_err(|error| format!("actor-knockback.get-source failed: {error:?}"))?;
                let vector = knockback
                    .get_knockback()
                    .map_err(|error| format!("actor-knockback.get-knockback failed: {error:?}"))?;
                knockback
                    .set_knockback(vector)
                    .map_err(|error| format!("actor-knockback.set-knockback failed: {error:?}"))?;
                let cancelled = knockback
                    .is_cancelled()
                    .map_err(|error| format!("actor-knockback.is-cancelled failed: {error:?}"))?;
                knockback
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("actor-knockback.set-cancelled failed: {error:?}"))?;
                log(&format!(
                    "{} is knocked by {:?}{}",
                    actor_name(&actor)?,
                    vector,
                    source
                        .as_ref()
                        .map(actor_name)
                        .transpose()?
                        .map(|source| format!(" (source: {source})"))
                        .unwrap_or_default(),
                ))?;
            }
            exported_events::Event::ActorRemove(remove) => {
                let actor = remove
                    .get_actor()
                    .map_err(|error| format!("actor-remove.get-actor failed: {error:?}"))?;
                log(&format!(
                    "{} is removed from the world.",
                    actor_name(&actor)?
                ))?;
            }
            exported_events::Event::ActorSpawn(spawn) => {
                let actor = spawn
                    .get_actor()
                    .map_err(|error| format!("actor-spawn.get-actor failed: {error:?}"))?;
                let cancelled = spawn
                    .is_cancelled()
                    .map_err(|error| format!("actor-spawn.is-cancelled failed: {error:?}"))?;
                spawn
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("actor-spawn.set-cancelled failed: {error:?}"))?;
                log(&format!("{} just spawned.", actor_name(&actor)?))?;
            }
            exported_events::Event::ActorTeleport(teleport) => {
                let actor = teleport
                    .get_actor()
                    .map_err(|error| format!("actor-teleport.get-actor failed: {error:?}"))?;
                let from = teleport
                    .get_from()
                    .map_err(|error| format!("actor-teleport.get-from failed: {error:?}"))?;
                let to = teleport
                    .get_to()
                    .map_err(|error| format!("actor-teleport.get-to failed: {error:?}"))?;
                teleport
                    .set_from(&from)
                    .map_err(|error| format!("actor-teleport.set-from failed: {error:?}"))?;
                teleport
                    .set_to(&to)
                    .map_err(|error| format!("actor-teleport.set-to failed: {error:?}"))?;
                let cancelled = teleport
                    .is_cancelled()
                    .map_err(|error| format!("actor-teleport.is-cancelled failed: {error:?}"))?;
                teleport
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("actor-teleport.set-cancelled failed: {error:?}"))?;
                log(&format!(
                    "{} teleported from {} to {}",
                    actor_name(&actor)?,
                    location(&from),
                    location(&to)
                ))?;
            }
            exported_events::Event::BlockBreak(block_break) => {
                let player = block_break
                    .get_player()
                    .map_err(|error| format!("block-break.get-player failed: {error:?}"))?;
                let block = block_break
                    .get_block()
                    .map_err(|error| format!("block-break.get-block failed: {error:?}"))?;
                let cancelled = block_break
                    .is_cancelled()
                    .map_err(|error| format!("block-break.is-cancelled failed: {error:?}"))?;
                block_break
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("block-break.set-cancelled failed: {error:?}"))?;
                log(&format!(
                    "{} breaks a block {}",
                    player_name(&player)?,
                    block
                        .get_type()
                        .map_err(|error| format!("block.get-type failed: {error:?}"))?
                ))?;
            }
            exported_events::Event::BlockCook(cook) => {
                let block = cook
                    .get_block()
                    .map_err(|error| format!("block-cook.get-block failed: {error:?}"))?;
                let source = cook
                    .get_source()
                    .map_err(|error| format!("block-cook.get-source failed: {error:?}"))?;
                let result = cook
                    .get_result()
                    .map_err(|error| format!("block-cook.get-result failed: {error:?}"))?;
                log(&format!(
                    "{} cooked {} into {}",
                    block
                        .get_type()
                        .map_err(|error| format!("block.get-type failed: {error:?}"))?,
                    source
                        .get_type_id()
                        .map_err(|error| format!("item-stack-ref.get-type-id failed: {error:?}"))?,
                    result
                        .get_type_id()
                        .map_err(|error| format!("item-stack-ref.get-type-id failed: {error:?}"))?,
                ))?;
                let type_id = result
                    .get_type_id()
                    .map_err(|error| format!("item-stack-ref.get-type-id failed: {error:?}"))?;
                if type_id == "minecraft:cooked_cod" {
                    let apple = item_type::create_item_stack("minecraft:apple", Some(1))
                        .map_err(|error| format!("create-item-stack failed: {error:?}"))?;
                    cook.set_result(&apple)
                        .map_err(|error| format!("block-cook.set-result failed: {error:?}"))?;
                }
                let cancelled = cook
                    .is_cancelled()
                    .map_err(|error| format!("block-cook.is-cancelled failed: {error:?}"))?;
                cook.set_cancelled(cancelled)
                    .map_err(|error| format!("block-cook.set-cancelled failed: {error:?}"))?;
            }
            exported_events::Event::BlockPlace(block_place) => {
                let player = block_place
                    .get_player()
                    .map_err(|error| format!("block-place.get-player failed: {error:?}"))?;
                let replaced = block_place
                    .get_block_replaced()
                    .map_err(|error| format!("block-place.get-block-replaced failed: {error:?}"))?;
                let against = block_place
                    .get_block_against()
                    .map_err(|error| format!("block-place.get-block-against failed: {error:?}"))?;
                let cancelled = block_place
                    .is_cancelled()
                    .map_err(|error| format!("block-place.is-cancelled failed: {error:?}"))?;
                block_place
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("block-place.set-cancelled failed: {error:?}"))?;
                log(&format!(
                    "{} replaces {} against {}",
                    player_name(&player)?,
                    replaced
                        .get_type()
                        .map_err(|error| format!("block.get-type failed: {error:?}"))?,
                    against
                        .get_type()
                        .map_err(|error| format!("block.get-type failed: {error:?}"))?
                ))?;
            }
            exported_events::Event::BlockExplode(explode) => {
                let block = explode
                    .get_block()
                    .map_err(|error| format!("block-explode.get-block failed: {error:?}"))?;
                let blocks = explode
                    .get_block_list()
                    .map_err(|error| format!("block-explode.get-block-list failed: {error:?}"))?;
                let cancelled = explode
                    .is_cancelled()
                    .map_err(|error| format!("block-explode.is-cancelled failed: {error:?}"))?;
                explode
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("block-explode.set-cancelled failed: {error:?}"))?;
                log(&format!(
                    "Block {} exploded ({} blocks affected)",
                    block
                        .get_type()
                        .map_err(|error| format!("block.get-type failed: {error:?}"))?,
                    blocks.len()
                ))?;
            }
            exported_events::Event::BlockForm(grow) => {
                let block = grow
                    .get_block()
                    .map_err(|error| format!("block-form.get-block failed: {error:?}"))?;
                let cancelled = grow
                    .is_cancelled()
                    .map_err(|error| format!("block-form.is-cancelled failed: {error:?}"))?;
                grow.set_cancelled(cancelled)
                    .map_err(|error| format!("block-form.set-cancelled failed: {error:?}"))?;
                log(&format!(
                    "{} formed",
                    block
                        .get_type()
                        .map_err(|error| format!("block.get-type failed: {error:?}"))?
                ))?;
            }
            exported_events::Event::BlockFromTo(from_to) => {
                let block = from_to
                    .get_block()
                    .map_err(|error| format!("block-from-to.get-block failed: {error:?}"))?;
                let to_block = from_to
                    .get_to_block()
                    .map_err(|error| format!("block-from-to.get-to-block failed: {error:?}"))?;
                let cancelled = from_to
                    .is_cancelled()
                    .map_err(|error| format!("block-from-to.is-cancelled failed: {error:?}"))?;
                from_to
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("block-from-to.set-cancelled failed: {error:?}"))?;
                log(&format!(
                    "{} flowed to {}",
                    block
                        .get_type()
                        .map_err(|error| format!("block.get-type failed: {error:?}"))?,
                    to_block
                        .get_type()
                        .map_err(|error| format!("block.get-type failed: {error:?}"))?
                ))?;
            }
            exported_events::Event::BlockGrow(grow) => {
                let block = grow
                    .get_block()
                    .map_err(|error| format!("block-grow.get-block failed: {error:?}"))?;
                let cancelled = grow
                    .is_cancelled()
                    .map_err(|error| format!("block-grow.is-cancelled failed: {error:?}"))?;
                grow.set_cancelled(cancelled)
                    .map_err(|error| format!("block-grow.set-cancelled failed: {error:?}"))?;
                log(&format!(
                    "{} grew",
                    block
                        .get_type()
                        .map_err(|error| format!("block.get-type failed: {error:?}"))?
                ))?;
            }
            exported_events::Event::BlockPistonExtend(piston) => {
                let block = piston
                    .get_block()
                    .map_err(|error| format!("block-piston-extend.get-block failed: {error:?}"))?;
                let direction = piston.get_direction().map_err(|error| {
                    format!("block-piston-extend.get-direction failed: {error:?}")
                })?;
                let cancelled = piston.is_cancelled().map_err(|error| {
                    format!("block-piston-extend.is-cancelled failed: {error:?}")
                })?;
                piston.set_cancelled(cancelled).map_err(|error| {
                    format!("block-piston-extend.set-cancelled failed: {error:?}")
                })?;
                log(&format!(
                    "Piston ({}) expands towards {direction:?}",
                    block
                        .get_type()
                        .map_err(|error| format!("block.get-type failed: {error:?}"))?
                ))?;
            }
            exported_events::Event::BlockPistonRetract(piston) => {
                let block = piston
                    .get_block()
                    .map_err(|error| format!("block-piston-retract.get-block failed: {error:?}"))?;
                let direction = piston.get_direction().map_err(|error| {
                    format!("block-piston-retract.get-direction failed: {error:?}")
                })?;
                let cancelled = piston.is_cancelled().map_err(|error| {
                    format!("block-piston-retract.is-cancelled failed: {error:?}")
                })?;
                piston.set_cancelled(cancelled).map_err(|error| {
                    format!("block-piston-retract.set-cancelled failed: {error:?}")
                })?;
                log(&format!(
                    "Piston ({}) retracts towards {direction:?}",
                    block
                        .get_type()
                        .map_err(|error| format!("block.get-type failed: {error:?}"))?
                ))?;
            }
            exported_events::Event::LeavesDecay(leaves) => {
                let block = leaves
                    .get_block()
                    .map_err(|error| format!("leaves-decay.get-block failed: {error:?}"))?;
                let cancelled = leaves
                    .is_cancelled()
                    .map_err(|error| format!("leaves-decay.is-cancelled failed: {error:?}"))?;
                leaves
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("leaves-decay.set-cancelled failed: {error:?}"))?;
                log(&format!(
                    "Leaves ({}) decayed",
                    block
                        .get_type()
                        .map_err(|error| format!("block.get-type failed: {error:?}"))?
                ))?;
            }
            exported_events::Event::ChunkLoad(chunk) => {
                let chunk_x = chunk
                    .get_chunk_x()
                    .map_err(|error| format!("chunk-load.get-chunk-x failed: {error:?}"))?;
                let chunk_z = chunk
                    .get_chunk_z()
                    .map_err(|error| format!("chunk-load.get-chunk-z failed: {error:?}"))?;
                let dimension = chunk
                    .get_dimension()
                    .map_err(|error| format!("chunk-load.get-dimension failed: {error:?}"))?;
                log(&format!(
                    "Chunk ({}, {}) is loaded in {}",
                    chunk_x, chunk_z, dimension
                ))?;
            }
            exported_events::Event::ChunkUnload(chunk) => {
                let chunk_x = chunk
                    .get_chunk_x()
                    .map_err(|error| format!("chunk-unload.get-chunk-x failed: {error:?}"))?;
                let chunk_z = chunk
                    .get_chunk_z()
                    .map_err(|error| format!("chunk-unload.get-chunk-z failed: {error:?}"))?;
                let dimension = chunk
                    .get_dimension()
                    .map_err(|error| format!("chunk-unload.get-dimension failed: {error:?}"))?;
                log(&format!(
                    "Chunk ({}, {}) is unloaded in {}",
                    chunk_x, chunk_z, dimension
                ))?;
            }
            exported_events::Event::ServerCommand(command) => {
                let sender_name = command
                    .get_sender_name()
                    .map_err(|error| format!("server-command.get-sender-name failed: {error:?}"))?;
                let command_text = command
                    .get_command()
                    .map_err(|error| format!("server-command.get-command failed: {error:?}"))?;
                log(&format!(
                    "{} executed command: {}",
                    sender_name, command_text
                ))?;
                command
                    .set_command(&command_text)
                    .map_err(|error| format!("server-command.set-command failed: {error:?}"))?;
                let cancelled = command
                    .is_cancelled()
                    .map_err(|error| format!("server-command.is-cancelled failed: {error:?}"))?;
                command
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("server-command.set-cancelled failed: {error:?}"))?;
            }
            exported_events::Event::BroadcastMessage(message) => {
                let text = message
                    .get_message()
                    .map_err(|error| format!("broadcast-message.get-message failed: {error:?}"))?;
                log(&format!("Broadcast message: {}", message_text(&text)))?;
                message
                    .set_message(&text)
                    .map_err(|error| format!("broadcast-message.set-message failed: {error:?}"))?;
                let cancelled = message
                    .is_cancelled()
                    .map_err(|error| format!("broadcast-message.is-cancelled failed: {error:?}"))?;
                message.set_cancelled(cancelled).map_err(|error| {
                    format!("broadcast-message.set-cancelled failed: {error:?}")
                })?;
            }
            exported_events::Event::ServerListPing(ping) => {
                let motd = ping
                    .get_motd()
                    .map_err(|error| format!("server-list-ping.get-motd failed: {error:?}"))?;
                let server_guid = ping.get_server_guid().map_err(|error| {
                    format!("server-list-ping.get-server-guid failed: {error:?}")
                })?;
                let local_port = ping.get_local_port().map_err(|error| {
                    format!("server-list-ping.get-local-port failed: {error:?}")
                })?;
                log(&format!(
                    "ServerListPingEvent is called (motd: {}, port: {})",
                    motd, local_port
                ))?;
                ping.set_motd(&format!("**{motd}**"))
                    .map_err(|error| format!("server-list-ping.set-motd failed: {error:?}"))?;
                ping.set_server_guid(&server_guid).map_err(|error| {
                    format!("server-list-ping.set-server-guid failed: {error:?}")
                })?;
                ping.set_local_port(local_port).map_err(|error| {
                    format!("server-list-ping.set-local-port failed: {error:?}")
                })?;
                let local_port_v6 = ping.get_local_port_v6().map_err(|error| {
                    format!("server-list-ping.get-local-port-v6 failed: {error:?}")
                })?;
                ping.set_local_port_v6(local_port_v6).map_err(|error| {
                    format!("server-list-ping.set-local-port-v6 failed: {error:?}")
                })?;
                let cancelled = ping
                    .is_cancelled()
                    .map_err(|error| format!("server-list-ping.is-cancelled failed: {error:?}"))?;
                ping.set_cancelled(cancelled)
                    .map_err(|error| format!("server-list-ping.set-cancelled failed: {error:?}"))?;
            }
            exported_events::Event::WeatherChange(weather) => {
                let to_weather = weather
                    .get_to_weather()
                    .map_err(|error| format!("weather-change.get-to-weather failed: {error:?}"))?;
                log(&format!("Weather state changed to {to_weather}"))?;
                let cancelled = weather
                    .is_cancelled()
                    .map_err(|error| format!("weather-change.is-cancelled failed: {error:?}"))?;
                weather
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("weather-change.set-cancelled failed: {error:?}"))?;
            }
            exported_events::Event::ThunderChange(thunder) => {
                let to_thunder = thunder
                    .get_to_thunder()
                    .map_err(|error| format!("thunder-change.get-to-thunder failed: {error:?}"))?;
                log(&format!("Thunder state changed to {to_thunder}"))?;
                let cancelled = thunder
                    .is_cancelled()
                    .map_err(|error| format!("thunder-change.is-cancelled failed: {error:?}"))?;
                thunder
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("thunder-change.set-cancelled failed: {error:?}"))?;
            }
            exported_events::Event::PluginEnable(plugin) => {
                let name = plugin
                    .get_plugin_name()
                    .map_err(|error| format!("plugin-enable.get-plugin-name failed: {error:?}"))?;
                log(&format!("{name} enabled"))?;
            }
            exported_events::Event::PluginDisable(plugin) => {
                let name = plugin
                    .get_plugin_name()
                    .map_err(|error| format!("plugin-disable.get-plugin-name failed: {error:?}"))?;
                log(&format!("{name} disabled"))?;
            }
            exported_events::Event::ServerLoad(load) => {
                let load_type = load
                    .get_load_type()
                    .map_err(|error| format!("server-load.get-load-type failed: {error:?}"))?;
                log(&format!("Server load event ({load_type:?})"))?;
            }
            exported_events::Event::PacketSend(packet) => {
                let packet_id = packet
                    .get_packet_id()
                    .map_err(|error| format!("packet-send.get-packet-id failed: {error:?}"))?;
                let payload = packet
                    .get_payload()
                    .map_err(|error| format!("packet-send.get-payload failed: {error:?}"))?;
                log(&format!(
                    "Packet {} sent ({} bytes payload)",
                    packet_id,
                    payload.len()
                ))?;
                let cancelled = packet
                    .is_cancelled()
                    .map_err(|error| format!("packet-send.is-cancelled failed: {error:?}"))?;
                packet
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("packet-send.set-cancelled failed: {error:?}"))?;
            }
            exported_events::Event::PacketReceive(packet) => {
                let packet_id = packet
                    .get_packet_id()
                    .map_err(|error| format!("packet-receive.get-packet-id failed: {error:?}"))?;
                let payload = packet
                    .get_payload()
                    .map_err(|error| format!("packet-receive.get-payload failed: {error:?}"))?;
                log(&format!(
                    "Packet {} received ({} bytes payload)",
                    packet_id,
                    payload.len()
                ))?;
                let cancelled = packet
                    .is_cancelled()
                    .map_err(|error| format!("packet-receive.is-cancelled failed: {error:?}"))?;
                packet
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("packet-receive.set-cancelled failed: {error:?}"))?;
            }
            exported_events::Event::MapInitialize(initialize) => {
                let map_id = initialize
                    .get_map_id()
                    .map_err(|error| format!("map-initialize.get-map-id failed: {error:?}"))?;
                log(&format!("Map {map_id} initialized"))?;
            }
            exported_events::Event::ScriptMessage(script) => {
                let message_id = script
                    .get_message_id()
                    .map_err(|error| format!("script-message.get-message-id failed: {error:?}"))?;
                let message = script
                    .get_message()
                    .map_err(|error| format!("script-message.get-message failed: {error:?}"))?;
                let sender = script
                    .get_sender()
                    .map_err(|error| format!("script-message.get-sender failed: {error:?}"))?;
                let sender_name = sender
                    .get_name()
                    .map_err(|error| format!("command-sender.get-name failed: {error:?}"))?;
                let cancelled = script
                    .is_cancelled()
                    .map_err(|error| format!("script-message.is-cancelled failed: {error:?}"))?;
                script
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("script-message.set-cancelled failed: {error:?}"))?;
                log(&format!(
                    "Script message {message_id} from {sender_name}: {message}"
                ))?;
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Server-side tests (on_enable)
// ---------------------------------------------------------------------------

fn run_server_tests() -> Result<(), String> {
    test_server_identity()?;
    test_server_performance()?;
    test_level_and_dimension()?;
    test_registries()?;
    test_language()?;
    test_item_type_and_stack()?;
    test_item_metadata()?;
    test_nbt()?;
    test_scoreboard()?;
    test_boss_bar()?;
    test_ban_list()?;
    test_plugin_manager()?;
    test_service_bus()?;
    test_map_view()?;
    test_map_renderer()?;
    test_tasks()?;
    test_console_command()?;
    Ok(())
}

fn test_item_metadata() -> Result<(), String> {
    let stack = item_type::create_item_stack("minecraft:diamond_sword", None)
        .map_err(|e| fmt_err("item-type.create-item-stack", e))?;
    let meta = stack
        .get_meta()
        .map_err(|e| fmt_err("item-stack.get-meta", e))?;
    let lore = vec!["A powerful blade".to_owned(), "of destiny".to_owned()];
    meta.set_display_name(Some("Excalibur"))
        .map_err(|e| fmt_err("item-meta.set-display-name", e))?;
    meta.set_lore(Some(&lore))
        .map_err(|e| fmt_err("item-meta.set-lore", e))?;
    let added = meta
        .add_enchant("minecraft:sharpness", 3, true)
        .map_err(|e| fmt_err("item-meta.add-enchant", e))?;
    run_test("item-meta.add-enchant", added)?;
    meta.set_damage(5)
        .map_err(|e| fmt_err("item-meta.set-damage", e))?;
    let name = meta
        .get_display_name()
        .map_err(|e| fmt_err("item-meta.get-display-name", e))?;
    let stored_lore = meta
        .get_lore()
        .map_err(|e| fmt_err("item-meta.get-lore", e))?;
    let level = meta
        .get_enchant_level("minecraft:sharpness")
        .map_err(|e| fmt_err("item-meta.get-enchant-level", e))?;
    let damage = meta
        .get_damage()
        .map_err(|e| fmt_err("item-meta.get-damage", e))?;
    run_test(
        "item-meta roundtrip",
        name == "Excalibur" && stored_lore == lore && level == 3 && damage == 5,
    )?;
    stack
        .set_meta(&meta)
        .map_err(|e| fmt_err("item-stack.set-meta", e))?;
    let persisted = stack
        .get_meta()
        .map_err(|e| fmt_err("item-stack.get-meta", e))?;
    run_test(
        "item-meta persisted",
        persisted
            .has_lore()
            .map_err(|e| fmt_err("item-meta.has-lore", e))?
            && persisted
                .has_enchant("minecraft:sharpness")
                .map_err(|e| fmt_err("item-meta.has-enchant", e))?,
    )?;
    Ok(())
}

fn test_server_identity() -> Result<(), String> {
    let name = server::get_name().map_err(|e| fmt_err("server.get-name", e))?;
    let version = server::get_version().map_err(|e| fmt_err("server.get-version", e))?;
    let minecraft =
        server::get_minecraft_version().map_err(|e| fmt_err("server.get-minecraft-version", e))?;
    let protocol =
        server::get_protocol_version().map_err(|e| fmt_err("server.get-protocol-version", e))?;
    let max_players =
        server::get_max_players().map_err(|e| fmt_err("server.get-max-players", e))?;
    let port = server::get_port().map_err(|e| fmt_err("server.get-port", e))?;
    let port_v6 = server::get_port_v6().map_err(|e| fmt_err("server.get-port-v6", e))?;
    let online_mode =
        server::get_online_mode().map_err(|e| fmt_err("server.get-online-mode", e))?;
    let primary_thread =
        server::is_primary_thread().map_err(|e| fmt_err("server.is-primary-thread", e))?;
    let command_sender =
        server::get_command_sender().map_err(|e| fmt_err("server.get-command-sender", e))?;
    let sender_name = command_sender
        .get_name()
        .map_err(|e| fmt_err("command-sender.get-name", e))?;
    let message = message::Message::Translatable(translatable::Translatable {
        text: "item.apple.name".to_owned(),
        parameters: Vec::new(),
    });
    command_sender
        .send_message(&message)
        .map_err(|e| fmt_err("command-sender.send-message", e))?;
    command_sender
        .send_error_message(&message)
        .map_err(|e| fmt_err("command-sender.send-error-message", e))?;

    run_test(
        "server.identity",
        !name.is_empty() && !version.is_empty() && !minecraft.is_empty() && protocol > 0,
    )?;
    log(&format!(
        "full_test: server {name} {version} (mc {minecraft}, protocol {protocol}, max {max_players}, ports {port}/{port_v6}, online-mode {online_mode}, primary-thread {primary_thread}, sender {sender_name})"
    ))
}

fn test_server_performance() -> Result<(), String> {
    let results = [
        (
            "server.get-current-ms-per-tick",
            server::get_current_ms_per_tick().map(|v| v >= 0.0),
        ),
        (
            "server.get-average-ms-per-tick",
            server::get_average_ms_per_tick().map(|v| v >= 0.0),
        ),
        (
            "server.get-current-tps",
            server::get_current_tps().map(|v| v > 0.0),
        ),
        (
            "server.get-average-tps",
            server::get_average_tps().map(|v| v > 0.0),
        ),
        (
            "server.get-current-tick-usage",
            server::get_current_tick_usage().map(|v| v >= 0.0),
        ),
        (
            "server.get-average-tick-usage",
            server::get_average_tick_usage().map(|v| v >= 0.0),
        ),
        (
            "server.get-start-time-ms",
            server::get_start_time_ms().map(|v| v > 0),
        ),
    ];
    let all_ok = results.iter().all(|(_, result)| result.is_ok_and(|ok| ok));
    for (name, result) in results {
        if !result.is_ok_and(|ok| ok) {
            record_failure(name);
        }
    }
    run_test("server.performance", all_ok)?;
    Ok(())
}

fn test_level_and_dimension() -> Result<(), String> {
    let level = level::get_level().map_err(|e| fmt_err("level.get-level", e))?;
    run_test("level.get-level", !level.name.is_empty() && level.seed != 0)?;
    log(&format!(
        "full_test: level {} (seed {}, time {}, {} dimensions)",
        level.name, level.seed, level.time, level.dimension_count
    ))?;

    let dimensions = level::list_dimensions().map_err(|e| fmt_err("level.list-dimensions", e))?;
    run_test(
        "level.list-dimensions",
        dimensions.iter().any(|dimension| {
            dimension.kind == aegilex::endstone::dimension::DimensionKind::Overworld
        }) && dimensions
            .iter()
            .any(|dimension| dimension.kind == aegilex::endstone::dimension::DimensionKind::Nether),
    )?;

    let overworld =
        level::find_dimension("overworld").map_err(|e| fmt_err("level.find-dimension", e))?;
    let nether = level::find_dimension("nether").map_err(|e| fmt_err("level.find-dimension", e))?;
    let end = level::find_dimension("the_end").map_err(|e| fmt_err("level.find-dimension", e))?;
    run_test(
        "level.find-dimension",
        overworld.is_some() && nether.is_some() && end.is_some(),
    )?;

    let time = level::get_time().map_err(|e| fmt_err("level.get-time", e))?;
    level::set_time(time + 100).map_err(|e| fmt_err("level.set-time", e))?;
    let new_time = level::get_time().map_err(|e| fmt_err("level.get-time", e))?;
    run_test("level.set-time", new_time == time + 100)?;

    Ok(())
}

fn test_registries() -> Result<(), String> {
    let apple = server::registry_item_type_get("minecraft:apple")
        .map_err(|e| fmt_err("server.registry-item-type-get", e))?;
    run_test(
        "server.registry-item-type-get",
        apple
            .as_ref()
            .is_some_and(|data| data.max_stack_size == 64 && data.type_id == "minecraft:apple"),
    )?;
    if let Some(apple) = &apple {
        log(&format!(
            "full_test: apple item type '{}' (stack {})",
            apple.translation_key, apple.max_stack_size
        ))?;
    }

    let item_types = server::registry_item_type_list()
        .map_err(|e| fmt_err("server.registry-item-type-list", e))?;
    run_test("server.registry-item-type-list", !item_types.is_empty())?;

    let sharpness = server::registry_enchantment_get("minecraft:sharpness")
        .map_err(|e| fmt_err("server.registry-enchantment-get", e))?;
    run_test(
        "server.registry-enchantment-get",
        sharpness.as_ref().is_some_and(|data| data.max_level >= 5),
    )?;
    // This binding's variable-length record list is covered by the runtime's
    // host test; querying a concrete registry entry exercises the live API.

    Ok(())
}

fn test_language() -> Result<(), String> {
    let translated = language::translate_text("item.apple.name")
        .map_err(|e| fmt_err("language.translate-text", e))?;
    run_test(
        "language.translate-text",
        translated.to_lowercase().contains("apple"),
    )?;
    log(&format!(
        "full_test: translated item.apple.name -> {translated}"
    ))
}

fn test_item_type_and_stack() -> Result<(), String> {
    let info = item_type::get_item_type("minecraft:diamond_sword")
        .map_err(|e| fmt_err("item-type.get-item-type", e))?;
    run_test(
        "item-type.get-item-type",
        info.type_id == "minecraft:diamond_sword" && info.max_durability > 0,
    )?;

    let stack = item_type::create_item_stack("minecraft:apple", Some(2))
        .map_err(|e| fmt_err("item-type.create-item-stack", e))?;
    let type_id = stack
        .get_type_id()
        .map_err(|e| fmt_err("item-stack.get-type-id", e))?;
    let amount = stack
        .get_amount()
        .map_err(|e| fmt_err("item-stack.get-amount", e))?;
    run_test(
        "item-stack.create/get",
        type_id == "minecraft:apple" && amount == 2,
    )?;
    stack
        .set_amount(8)
        .map_err(|e| fmt_err("item-stack.set-amount", e))?;
    let amount = stack
        .get_amount()
        .map_err(|e| fmt_err("item-stack.get-amount", e))?;
    run_test("item-stack.set-amount", amount == 8)?;

    let can_enchant = stack
        .can_enchant("minecraft:sharpness")
        .map_err(|e| fmt_err("item-stack.can-enchant", e))?;
    run_test("item-stack.can-enchant", !can_enchant)?;

    let sword = item_type::create_item_stack("minecraft:diamond_sword", None)
        .map_err(|e| fmt_err("item-type.create-item-stack", e))?;
    let can_enchant_sword = sword
        .can_enchant("minecraft:sharpness")
        .map_err(|e| fmt_err("item-stack.can-enchant", e))?;
    run_test("item-stack.can-enchant (sword)", can_enchant_sword)?;

    let similar = stack
        .is_similar(&stack)
        .map_err(|e| fmt_err("item-stack.is-similar", e))?;
    run_test("item-stack.is-similar", similar)?;

    Ok(())
}

fn test_nbt() -> Result<(), String> {
    let tag = nbt::from_int(42).map_err(|e| fmt_err("nbt.from-int", e))?;
    let value = tag.get_int().map_err(|e| fmt_err("nbt.get-int", e))?;
    run_test("nbt.from-int/get-int", value == 42)?;

    tag.set_string("hello")
        .map_err(|e| fmt_err("nbt.set-string", e))?;
    let text = tag.get_string().map_err(|e| fmt_err("nbt.get-string", e))?;
    run_test("nbt.set-string/get-string", text == "hello")?;

    let tag_type = tag.get_type().map_err(|e| fmt_err("nbt.get-type", e))?;
    run_test("nbt.get-type", tag_type == nbt::TagType::StringValue)?;

    let compound = nbt::from_compound(&[nbt::CompoundEntry {
        key: "answer".to_owned(),
        value: &tag,
    }])
    .map_err(|e| fmt_err("nbt.from-compound", e))?;
    let keys = compound
        .get_compound_keys()
        .map_err(|e| fmt_err("nbt.get-compound-keys", e))?;
    run_test("nbt.from-compound", keys == vec!["answer".to_owned()])?;

    Ok(())
}

fn test_scoreboard() -> Result<(), String> {
    let scoreboard = scoreboard::create().map_err(|e| fmt_err("scoreboard.create", e))?;
    let criteria = criteria::Criteria {
        name: "dummy".to_owned(),
        read_only: false,
        default_render_type: aegilex::endstone::render_type::RenderType::Integer,
    };
    let objective = scoreboard
        .create_objective("fulltest_objective", &criteria, "FullTest", None)
        .map_err(|e| fmt_err("scoreboard.create-objective", e))?;
    let objective_name = objective
        .get_name()
        .map_err(|e| fmt_err("objective.get-name", e))?;
    run_test(
        "scoreboard.create-objective",
        objective_name == "fulltest_objective",
    )?;

    let entry = score_entry::from_text("fulltest_entry")
        .map_err(|e| fmt_err("score-entry.from-text", e))?;
    objective
        .set_score_value(&entry, 42)
        .map_err(|e| fmt_err("objective.set-score-value", e))?;
    let value = objective
        .get_score_value(&entry)
        .map_err(|e| fmt_err("objective.get-score-value", e))?;
    run_test("objective.score-value roundtrip", value == 42)?;

    let scores = scoreboard
        .get_scores(&entry)
        .map_err(|e| fmt_err("scoreboard.get-scores", e))?;
    run_test(
        "scoreboard.get-scores",
        scores.iter().any(|score| score.value == 42),
    )?;

    objective
        .unregister()
        .map_err(|e| fmt_err("objective.unregister", e))?;
    let objective = scoreboard
        .get_objective("fulltest_objective")
        .map_err(|e| fmt_err("scoreboard.get-objective", e))?;
    run_test("objective.unregister removes", objective.is_none())?;

    Ok(())
}

fn test_boss_bar() -> Result<(), String> {
    let bar = boss_bar::create(
        "FullTest",
        aegilex::endstone::bar_color::BossBarColor::Green,
        aegilex::endstone::bar_style::BossBarStyle::Segmented10,
        &[],
    )
    .map_err(|e| fmt_err("boss-bar.create", e))?;
    let title = bar
        .get_title()
        .map_err(|e| fmt_err("boss-bar.get-title", e))?;
    run_test("boss-bar.create/get-title", title == "FullTest")?;

    bar.set_title("FullTest v2")
        .map_err(|e| fmt_err("boss-bar.set-title", e))?;
    bar.set_progress(0.5)
        .map_err(|e| fmt_err("boss-bar.set-progress", e))?;
    bar.set_visible(true)
        .map_err(|e| fmt_err("boss-bar.set-visible", e))?;
    let progress = bar
        .get_progress()
        .map_err(|e| fmt_err("boss-bar.get-progress", e))?;
    let visible = bar
        .is_visible()
        .map_err(|e| fmt_err("boss-bar.is-visible", e))?;
    run_test(
        "boss-bar.mutators",
        (progress - 0.5).abs() < 0.001 && visible,
    )?;

    let color = bar
        .get_color()
        .map_err(|e| fmt_err("boss-bar.get-color", e))?;
    run_test(
        "boss-bar.get-color",
        color == aegilex::endstone::bar_color::BossBarColor::Green,
    )?;

    bar.remove_all_players()
        .map_err(|e| fmt_err("boss-bar.remove-all-players", e))?;
    Ok(())
}

fn test_ban_list() -> Result<(), String> {
    let list =
        ban_list::get(ban_list::BanListKind::Player).map_err(|e| fmt_err("ban-list.get", e))?;
    let banned = list
        .is_banned("nonexistent-fulltest-player")
        .map_err(|e| fmt_err("ban-list.is-banned", e))?;
    run_test("ban-list.is-banned", !banned)?;

    let entry = list
        .get_entry("nonexistent-fulltest-player")
        .map_err(|e| fmt_err("ban-list.get-entry", e))?;
    run_test("ban-list.get-entry (absent)", entry.is_none())?;

    let _entries = list
        .list_entries()
        .map_err(|e| fmt_err("ban-list.list-entries", e))?;
    // A fresh server normally has no bans; reaching the list is the operation
    // being exercised, rather than its current contents.
    run_test("ban-list.list-entries", true)?;
    Ok(())
}

fn test_plugin_manager() -> Result<(), String> {
    let plugins =
        plugin_manager::list_plugins().map_err(|e| fmt_err("plugin-manager.list-plugins", e))?;
    let self_found = plugins.iter().any(|plugin| plugin.metadata.name == "full_test");
    run_test("plugin-manager.list-plugins", self_found)?;
    for plugin in &plugins {
        log(&format!(
            "full_test: plugin {} v{} (enabled: {})",
            plugin.metadata.name, plugin.metadata.version, plugin.enabled
        ))?;
    }

    let summary = plugin_manager::get("full_test").map_err(|e| fmt_err("plugin-manager.get", e))?;
    run_test(
        "plugin-manager.get",
        summary
            .as_ref()
            .is_some_and(|plugin| plugin.metadata.name == "full_test" && plugin.enabled),
    )?;
    Ok(())
}

fn test_service_bus() -> Result<(), String> {
    let provider = service_bus::publish(&service_bus::ServiceSpec {
        name: "fulltest.echo".to_owned(),
        version: "1.0.0".to_owned(),
        methods: vec!["echo".to_owned(), "reject".to_owned()],
        priority: service_priority::ServicePriority::Normal,
    })
    .map_err(|e| fmt_err("service-bus.publish", e))?;

    let info = provider
        .get_spec()
        .map_err(|e| fmt_err("service-bus.get-spec", e))?;
    run_test(
        "service-bus.get-spec",
        info.spec.name == "fulltest.echo" && info.spec.methods.len() == 2,
    )?;
    log(&format!(
        "full_test: published service {} (id {})",
        info.spec.name, info.id
    ))?;

    let providers = service_bus::list_providers("fulltest.echo")
        .map_err(|e| fmt_err("service-bus.list-providers", e))?;
    let found = providers.iter().any(|item| item.id == info.id);
    run_test("service-bus.list-providers", found)?;

    provider
        .unpublish()
        .map_err(|e| fmt_err("service-bus.unpublish", e))?;
    let providers = service_bus::list_providers("fulltest.echo")
        .map_err(|e| fmt_err("service-bus.list-providers", e))?;
    run_test("service-bus.unpublish", providers.is_empty())?;

    Ok(())
}

fn test_map_view() -> Result<(), String> {
    let map_id = map_view::create("overworld").map_err(|e| fmt_err("map-view.create", e))?;
    run_test("map-view.create", map_id != 0)?;

    let exists = map_view::exists(map_id).map_err(|e| fmt_err("map-view.exists", e))?;
    run_test("map-view.exists", exists)?;

    let dimension =
        map_view::get_dimension(map_id).map_err(|e| fmt_err("map-view.get-dimension", e))?;
    run_test("map-view.get-dimension", dimension == "Overworld")?;

    map_view::set_scale(map_id, aegilex::endstone::map_view::MapScale::Normal)
        .map_err(|e| fmt_err("map-view.set-scale", e))?;
    map_view::set_locked(map_id, false).map_err(|e| fmt_err("map-view.set-locked", e))?;
    let scale = map_view::get_scale(map_id).map_err(|e| fmt_err("map-view.get-scale", e))?;
    run_test(
        "map-view.scale roundtrip",
        scale == aegilex::endstone::map_view::MapScale::Normal,
    )?;

    Ok(())
}

fn test_map_renderer() -> Result<(), String> {
    let map_id =
        map_view::create("overworld").map_err(|e| fmt_err("map-view.create renderer", e))?;
    let renderer =
        map_renderer::register(map_id, false).map_err(|e| fmt_err("map-renderer.register", e))?;
    let renderer_map_id = renderer
        .get_map_id()
        .map_err(|e| fmt_err("map-renderer.get-map-id", e))?;
    let contextual = renderer
        .is_contextual()
        .map_err(|e| fmt_err("map-renderer.is-contextual", e))?;
    run_test(
        "map-renderer.register",
        renderer_map_id == map_id && !contextual,
    )?;

    renderer
        .unregister()
        .map_err(|e| fmt_err("map-renderer.unregister", e))?;
    Ok(())
}

fn test_tasks() -> Result<(), String> {
    let task_id = tasks::schedule_now().map_err(|e| fmt_err("tasks.schedule-now", e))?;
    run_test("tasks.schedule-now", task_id != 0)?;

    let task = tasks::get_task(task_id).map_err(|e| fmt_err("tasks.get-task", e))?;
    run_test(
        "tasks.get-task",
        task.task_id == task_id && task.owner == "full_test",
    )?;

    let pending = tasks::task_list_pending().map_err(|e| fmt_err("tasks.task-list-pending", e))?;
    run_test(
        "tasks.task-list-pending",
        pending.iter().any(|task| task.task_id == task_id),
    )?;

    let queued = tasks::task_is_queued(task_id).map_err(|e| fmt_err("tasks.task-is-queued", e))?;
    run_test("tasks.task-is-queued", queued)?;

    let running =
        tasks::task_is_running(task_id).map_err(|e| fmt_err("tasks.task-is-running", e))?;
    run_test("tasks.task-is-running", !running)?;

    tasks::cancel(task_id).map_err(|e| fmt_err("tasks.cancel", e))?;
    let task = tasks::get_task(task_id).map_err(|e| fmt_err("tasks.get-task", e))?;
    run_test("tasks.cancel", task.is_cancelled)?;

    Ok(())
}

fn test_console_command() -> Result<(), String> {
    let dispatched = server::dispatch_console_command("say aegilex full-test is running")
        .map_err(|e| fmt_err("server.dispatch-console-command", e))?;
    run_test("server.dispatch-console-command", dispatched)
}

// ---------------------------------------------------------------------------
// Player-side tests (on player-join)
// ---------------------------------------------------------------------------

fn run_player_tests(player: &player::Player) -> Result<(), String> {
    let name = player
        .get_name()
        .map_err(|e| fmt_err("player.get-name", e))?;
    run_test("player.get-name", !name.is_empty())?;

    let unique_id = player
        .get_unique_id()
        .map_err(|e| fmt_err("player.get-unique-id", e))?;
    run_test("player.get-unique-id", unique_id.len() == 16)?;

    let xuid = player
        .get_xuid()
        .map_err(|e| fmt_err("player.get-xuid", e))?;
    run_test("player.get-xuid", !xuid.is_empty())?;

    let _ping = player
        .get_ping()
        .map_err(|e| fmt_err("player.get-ping", e))?;
    run_test("player.get-ping", true)?;

    let locale = player
        .get_locale()
        .map_err(|e| fmt_err("player.get-locale", e))?;
    run_test("player.get-locale", !locale.is_empty())?;

    let address = player
        .get_address()
        .map_err(|e| fmt_err("player.get-address", e))?;
    run_test(
        "player.get-address",
        !address.hostname.is_empty() && address.port > 0,
    )?;

    let is_op = player
        .is_operator()
        .map_err(|e| fmt_err("player.is-operator", e))?;
    run_test("player.is-operator", is_op)?;

    let mode = player
        .get_game_mode()
        .map_err(|e| fmt_err("player.get-game-mode", e))?;
    let mode_name = match mode {
        game_mode::GameMode::Survival => "survival",
        game_mode::GameMode::Creative => "creative",
        game_mode::GameMode::Adventure => "adventure",
        game_mode::GameMode::Spectator => "spectator",
    };
    log(&format!(
        "full_test: {name} (mode {mode_name}, locale {locale})"
    ))?;

    test_player_skin(player)?;
    test_player_messaging(player)?;
    test_player_inventory(player)?;
    test_player_ender_chest(player)?;
    test_player_permissible(player)?;
    test_player_teleport(player)?;
    test_player_forms(player)?;
    test_player_scoreboard(player)?;
    test_player_boss_bar(player)?;
    Ok(())
}

fn test_player_skin(player: &player::Player) -> Result<(), String> {
    let skin = player
        .get_skin()
        .map_err(|e| fmt_err("player.get-skin", e))?;
    run_test(
        "player.get-skin",
        !skin.id.is_empty() && skin.image.width > 0,
    )?;
    Ok(())
}

fn test_player_messaging(player: &player::Player) -> Result<(), String> {
    let name = player
        .get_name()
        .map_err(|e| fmt_err("player.get-name", e))?;
    player
        .send_message(&message::Message::PlainText(format!(
            "hello {name}, from Aegilex full-test!"
        )))
        .map_err(|e| fmt_err("player.send-message", e))?;
    player
        .send_tip("Aegilex full-test tip")
        .map_err(|e| fmt_err("player.send-tip", e))?;
    player
        .send_toast("Aegilex", "full-test toast")
        .map_err(|e| fmt_err("player.send-toast", e))?;
    player
        .send_popup("Aegilex full-test popup")
        .map_err(|e| fmt_err("player.send-popup", e))?;
    player
        .send_title("Aegilex", "full-test title", Some(5), Some(40), Some(5))
        .map_err(|e| fmt_err("player.send-title", e))?;
    player
        .reset_title()
        .map_err(|e| fmt_err("player.reset-title", e))?;
    player
        .play_sound(
            &aegilex::endstone::location::Location {
                dimension: "overworld".to_owned(),
                x: 0.0,
                y: 64.0,
                z: 0.0,
                pitch: 0.0,
                yaw: 0.0,
            },
            "block.note_block.pling",
            1.0,
            1.0,
        )
        .map_err(|e| fmt_err("player.play-sound", e))?;
    player
        .stop_all_sounds()
        .map_err(|e| fmt_err("player.stop-all-sounds", e))?;
    run_test("player.messaging", true)
}

fn test_player_inventory(player: &player::Player) -> Result<(), String> {
    let player_inventory = player
        .get_inventory()
        .map_err(|e| fmt_err("player.get-inventory", e))?;
    let inventory = player_inventory
        .get_inventory()
        .map_err(|e| fmt_err("player-inventory.get-inventory", e))?;
    let size = inventory
        .get_size()
        .map_err(|e| fmt_err("inventory.get-size", e))?;
    run_test("inventory.get-size", size > 0)?;

    let apple = item_type::create_item_stack("minecraft:apple", Some(3))
        .map_err(|e| fmt_err("item-type.create-item-stack", e))?;
    inventory
        .set_item(0, Some(&apple))
        .map_err(|e| fmt_err("inventory.set-item", e))?;
    let item = inventory
        .get_item(0)
        .map_err(|e| fmt_err("inventory.get-item", e))?;
    let roundtrip = item
        .as_ref()
        .and_then(|item| item.get_amount().ok())
        .is_some_and(|amount| amount == 3);
    run_test("inventory.set/get-item roundtrip", roundtrip)?;

    let held_slot = player_inventory
        .get_held_item_slot()
        .map_err(|e| fmt_err("player-inventory.get-held-item-slot", e))?;
    run_test("player-inventory.get-held-item-slot", held_slot >= 0)?;

    let contents = inventory
        .get_contents()
        .map_err(|e| fmt_err("inventory.get-contents", e))?;
    run_test("inventory.get-contents", contents.len() >= size as usize)?;

    let empty_slot = inventory
        .first_empty()
        .map_err(|e| fmt_err("inventory.first-empty", e))?;
    run_test("inventory.first-empty", empty_slot.is_some())?;

    let first = inventory
        .first_slot(&aegilex::endstone::inventory::ItemRequest {
            matcher: aegilex::endstone::inventory::ItemMatcher::TypeId(
                "minecraft:apple".to_owned(),
            ),
            amount: Some(1),
        })
        .map_err(|e| fmt_err("inventory.first-slot", e))?;
    run_test("inventory.first-slot", first.is_some())?;

    Ok(())
}

fn test_player_ender_chest(player: &player::Player) -> Result<(), String> {
    let ender_chest = player
        .get_ender_chest()
        .map_err(|e| fmt_err("player.get-ender-chest", e))?;
    let size = ender_chest
        .get_size()
        .map_err(|e| fmt_err("inventory.get-size", e))?;
    run_test("player.get-ender-chest", size > 0)?;

    let apple = item_type::create_item_stack("minecraft:apple", Some(1))
        .map_err(|e| fmt_err("item-type.create-item-stack", e))?;
    ender_chest
        .set_item(0, Some(&apple))
        .map_err(|e| fmt_err("inventory.set-item", e))?;
    let item = ender_chest
        .get_item(0)
        .map_err(|e| fmt_err("inventory.get-item", e))?;
    let ok = item
        .as_ref()
        .and_then(|item| item.get_type_id().ok())
        .is_some_and(|type_id| type_id == "minecraft:apple");
    run_test("ender-chest set/get-item roundtrip", ok)?;
    ender_chest
        .clear_index(0)
        .map_err(|e| fmt_err("inventory.clear-index", e))?;
    Ok(())
}

fn test_player_permissible(player: &player::Player) -> Result<(), String> {
    let actor = player
        .as_actor()
        .map_err(|e| fmt_err("player.as-actor", e))?;
    let actor_name = actor
        .get_actor_name()
        .map_err(|e| fmt_err("actor.get-actor-name", e))?;
    let actor_type = actor
        .get_actor_type()
        .map_err(|e| fmt_err("actor.get-actor-type", e))?;
    let valid = actor.is_valid().map_err(|e| fmt_err("actor.is-valid", e))?;
    run_test(
        "player.as-actor",
        !actor_name.is_empty() && !actor_type.is_empty() && valid,
    )?;

    let mob = actor.as_mob().map_err(|e| fmt_err("actor.as-mob", e))?;
    run_test("actor.as-mob", mob.is_some())?;

    let health = mob
        .as_ref()
        .map(|mob| mob.get_health())
        .transpose()
        .map_err(|e| fmt_err("mob.get-health", e))?;
    run_test("mob.get-health", health.is_some_and(|health| health > 0))?;

    let command_sender =
        server::get_command_sender().map_err(|e| fmt_err("server.get-command-sender", e))?;
    let permissible = aegilex::endstone::permissible::get(&command_sender)
        .map_err(|e| fmt_err("permissible.get", e))?;
    let level = permissible
        .get_permission_level()
        .map_err(|e| fmt_err("permissible.get-permission-level", e))?;
    run_test(
        "permissible.get-permission-level",
        level == aegilex::endstone::permission_level::PermissionLevel::Console,
    )?;
    Ok(())
}

fn test_player_teleport(player: &player::Player) -> Result<(), String> {
    let actor = player
        .as_actor()
        .map_err(|e| fmt_err("player.as-actor", e))?;
    let location = actor
        .get_actor_location()
        .map_err(|e| fmt_err("actor.get-actor-location", e))?;
    let teleported = actor
        .teleport(&location)
        .map_err(|e| fmt_err("actor.teleport", e))?;
    run_test("player.teleport", teleported)?;

    player
        .spawn_particle(
            "minecraft:heart_particle",
            location.x,
            location.y + 1.0,
            location.z,
            None,
        )
        .map_err(|e| fmt_err("player.spawn-particle", e))?;
    Ok(())
}

fn test_player_forms(player: &player::Player) -> Result<(), String> {
    let spec = aegilex::endstone::player_form::FormSpec::Action(
        aegilex::endstone::action_form::ActionFormSpec {
            title: message::Message::PlainText("Aegilex full-test form".to_owned()),
            content: message::Message::PlainText(
                "Click a button to test the form callbacks.".to_owned(),
            ),
            controls: vec![
                aegilex::endstone::action_form::ActionControl::Button(form_button::Button {
                    text: message::Message::PlainText("OK".to_owned()),
                    icon: None,
                }),
                aegilex::endstone::action_form::ActionControl::Label(form_label::Label {
                    text: message::Message::PlainText("A label".to_owned()),
                }),
                aegilex::endstone::action_form::ActionControl::Header(form_header::Header {
                    label: message::Message::PlainText("Header".to_owned()),
                }),
                aegilex::endstone::action_form::ActionControl::Divider(
                    form_divider::Divider::Divider,
                ),
            ],
        },
    );
    let form = player_form::show(&player, &spec).map_err(|e| fmt_err("player-form.show", e))?;
    let title = form.get_title().map_err(|e| fmt_err("form.get-title", e))?;
    let title_text = match &title {
        message::Message::PlainText(text) => text.clone(),
        message::Message::Translatable(_) => String::new(),
    };
    run_test("player-form.show", title_text == "Aegilex full-test form")?;
    drop(form);
    Ok(())
}

fn test_player_scoreboard(player: &player::Player) -> Result<(), String> {
    let scoreboard = scoreboard::create().map_err(|e| fmt_err("scoreboard.create", e))?;
    let criteria = criteria::Criteria {
        name: "dummy".to_owned(),
        read_only: false,
        default_render_type: aegilex::endstone::render_type::RenderType::Integer,
    };
    let objective = scoreboard
        .create_objective(
            "fulltest_player_objective",
            &criteria,
            "FullTestPlayer",
            None,
        )
        .map_err(|e| fmt_err("scoreboard.create-objective", e))?;
    let entry =
        score_entry::from_player(player).map_err(|e| fmt_err("score-entry.from-player", e))?;
    objective
        .set_score_value(&entry, 7)
        .map_err(|e| fmt_err("objective.set-score-value", e))?;
    let scores = scoreboard
        .get_scores(&entry)
        .map_err(|e| fmt_err("scoreboard.get-scores", e))?;
    run_test(
        "player scoreboard score",
        scores.iter().any(|score| score.value == 7),
    )?;
    objective
        .unregister()
        .map_err(|e| fmt_err("objective.unregister", e))?;
    Ok(())
}

fn test_player_boss_bar(player: &player::Player) -> Result<(), String> {
    let bar = boss_bar::create(
        "FullTest Player",
        aegilex::endstone::bar_color::BossBarColor::Purple,
        aegilex::endstone::bar_style::BossBarStyle::Solid,
        &[],
    )
    .map_err(|e| fmt_err("boss-bar.create", e))?;
    bar.add_player(&player)
        .map_err(|e| fmt_err("boss-bar.add-player", e))?;
    let players = bar
        .get_players()
        .map_err(|e| fmt_err("boss-bar.get-players", e))?;
    run_test("boss-bar.add-player/get-players", !players.is_empty())?;
    bar.remove_player(&player)
        .map_err(|e| fmt_err("boss-bar.remove-player", e))?;
    bar.remove_all_players()
        .map_err(|e| fmt_err("boss-bar.remove-all-players", e))?;
    Ok(())
}
fn test_server_external_coverage() -> Result<(), String> {
    let current_max_players =
        server::get_max_players().map_err(|e| fmt_err("server.get-max-players", e))?;
    server::set_max_players(current_max_players)
        .map_err(|e| fmt_err("server.set-max-players", e))?;
    let roundtrip = server::get_max_players().map_err(|e| fmt_err("server.get-max-players", e))?
        == current_max_players;
    run_test("server.max-players roundtrip", roundtrip)?;
    let absent = server::find_player_by_name("nonexistent-fulltest-player")
        .map_err(|e| fmt_err("server.find-player-by-name", e))?;
    run_test("server.find-player-by-name absent", absent.is_none())?;
    let dimension = level::find_dimension("overworld")
        .map_err(|e| fmt_err("level.find-dimension", e))?
        .ok_or_else(|| "overworld dimension missing".to_owned())?;
    let block = aegilex::endstone::dimension::get_block(&dimension.name, 0, -64, 0)
        .map_err(|e| fmt_err("dimension.get-block", e))?;
    run_test(
        "dimension.get-block coordinates",
        !block
            .get_type()
            .map_err(|e| fmt_err("block.get-type", e))?
            .is_empty()
            && block.get_x().map_err(|e| fmt_err("block.get-x", e))? == 0
            && block.get_y().map_err(|e| fmt_err("block.get-y", e))? == -64
            && block.get_z().map_err(|e| fmt_err("block.get-z", e))? == 0,
    )?;
    let highest_y = aegilex::endstone::dimension::get_highest_block_y(&dimension.name, 0, 0)
        .map_err(|e| fmt_err("dimension.get-highest-block-y", e))?;
    let highest = aegilex::endstone::dimension::get_highest_block(&dimension.name, 0, 0)
        .map_err(|e| fmt_err("dimension.get-highest-block", e))?;
    run_test(
        "dimension.get-highest-block",
        !highest
            .get_type()
            .map_err(|e| fmt_err("block.get-type", e))?
            .is_empty()
            && highest.get_y().map_err(|e| fmt_err("block.get-y", e))? == highest_y,
    )?;
    let data = aegilex::endstone::block_type::create_block_data("minecraft:standing_sign")
        .map_err(|e| fmt_err("block-type.create-block-data", e))?;
    run_test(
        "block-data.default-state",
        data.get_type()
            .map_err(|e| fmt_err("block-data.get-type", e))?
            == "minecraft:standing_sign",
    )?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Test harness helpers
// ---------------------------------------------------------------------------

fn player_name(player: &player::Player) -> Result<String, String> {
    player
        .get_name()
        .map_err(|error| format!("player.get-name failed: {error:?}"))
}

fn actor_name(actor: &actor::Actor) -> Result<String, String> {
    actor
        .get_actor_name()
        .map_err(|error| format!("actor.get-actor-name failed: {error:?}"))
}

fn location(location: &location::Location) -> String {
    format!(
        "{}({:.1}, {:.1}, {:.1})",
        location.dimension, location.x, location.y, location.z
    )
}

fn fmt_err(operation: &str, error: types::HostError) -> String {
    format!("{operation} failed: {error:?}")
}

fn message_text(message: &message::Message) -> &str {
    match message {
        message::Message::PlainText(text) => text,
        message::Message::Translatable(_) => "<translatable>",
    }
}

fn record_failure(name: &str) {
    let mut state = STATE.lock();
    state.failed += 1;
    if !state.failures.iter().any(|failure| failure == name) {
        state.failures.push(name.to_owned());
    }
}

fn run_test(name: &str, passed: bool) -> Result<(), String> {
    if passed {
        STATE.lock().passed += 1;
        log(&format!("full_test: PASS {name}"))
    } else {
        record_failure(name);
        log(&format!("full_test: FAIL {name}"))
    }
}

fn log(message: &str) -> Result<(), String> {
    let logger = logger::get_logger().map_err(|error| format!("get logger failed: {error:?}"))?;
    logger
        .log(logger::LogLevel::Info, message)
        .map_err(|error| format!("log failed: {error:?}"))
}

export!(FullTestPlugin);
