wit_bindgen::generate!({
    path: "wit",
    world: "plugin",
});

use std::cell::RefCell;

use aegilex::endstone::{
    actor, bar_color, bar_style, boss_bar, item_type, location, logger, message, permission_default,
    player, plugin_metadata, task, types,
};
use exports::aegilex::endstone::{commands as exported_commands, events as exported_events};

const EVENT_NAMES: &[&str] = &[
    "player-login", "player-join", "player-emote", "player-interact", "player-interact-actor",
    "player-kick", "player-command", "player-quit", "player-chat", "player-game-mode-change",
    "player-jump", "player-move", "player-teleport", "player-portal", "player-death",
    "player-respawn", "player-item-consume", "player-item-held", "player-drop-item",
    "player-pickup-item", "player-bed-enter", "player-bed-leave", "player-skin-change",
    "player-dimension-change", "actor-damage", "actor-death", "actor-explode", "actor-knockback",
    "actor-remove", "actor-spawn", "actor-teleport", "block-break", "block-cook", "block-place",
    "block-explode", "block-form", "block-from-to", "block-grow", "block-piston-extend",
    "block-piston-retract", "leaves-decay", "chunk-load", "chunk-unload", "weather-change",
    "thunder-change", "server-command", "broadcast-message", "server-list-ping", "plugin-enable",
    "plugin-disable", "server-load", "packet-send", "packet-receive", "map-initialize", "script-message",
];

struct EventState {
    boss_bar: Option<boss_bar::BossBar>,
    triggered: [bool; EVENT_NAMES.len()],
}

impl EventState {
    const fn new() -> Self {
        Self {
            boss_bar: None,
            triggered: [false; EVENT_NAMES.len()],
        }
    }
}

thread_local! {
    static EVENT_STATE: RefCell<EventState> = const { RefCell::new(EventState::new()) };
}

struct EventTestPlugin;

impl Guest for EventTestPlugin {
    fn metadata() -> plugin_metadata::Metadata {
        plugin_metadata::Metadata {
            name: "event_test".to_owned(),
            version: "0.1.0".to_owned(),
            description: "Endstone event test component for Aegilex.".to_owned(),
            load_order: plugin_metadata::LoadOrder::PostWorld,
            authors: vec!["Aegilex Contributors".to_owned()],
            contributors: Vec::new(),
            website: String::new(),
            prefix: "EventTest".to_owned(),
            provides: Vec::new(),
            depend: Vec::new(),
            soft_depend: Vec::new(),
            load_before: Vec::new(),
            default_permission: permission_default::PermissionDefault::Operator,
            commands: vec![plugin_metadata::Command {
                name: "test".to_owned(),
                description: Some("Show the event-test component status.".to_owned()),
                aliases: vec!["eventtest".to_owned()],
                usages: vec![
                    "/test".to_owned(),
                    "/test status".to_owned(),
                    "/test count <event: string>".to_owned(),
                    "/test form <message|action|modal>".to_owned(),
                    "/test spawn <entity: entity_type>".to_owned(),
                    "/test block <block: block> [blockStates: block_states]".to_owned(),
                    "/test broadcast [args: message]".to_owned(),
                ],
                permissions: vec!["aegilex.eventtest.command.test".to_owned()],
            }],
            permissions: vec![plugin_metadata::PluginPermission {
                name: "aegilex.eventtest.command.test".to_owned(),
                description: Some("Allow using the /test command.".to_owned()),
                default_value: Some(permission_default::PermissionDefault::True),
                children: Vec::new(),
            }],
            subscriptions: EVENT_NAMES
                .iter()
                .map(|event_name| (*event_name).to_owned())
                .collect(),
        }
    }

    fn on_load() -> Result<(), String> {
        Ok(())
    }

    fn on_enable() -> Result<(), String> {
        let boss_bar = boss_bar::create(
            &format!("Events: 0/{}", EVENT_NAMES.len()),
            bar_color::BossBarColor::Green,
            bar_style::BossBarStyle::Segmented10,
            &[],
        )
        .map_err(|error| format!("boss-bar.create failed: {error:?}"))?;
        EVENT_STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.triggered.fill(false);
            state.boss_bar = Some(boss_bar);
        });
        log("event_test: on_enable is called!")
    }

    fn on_disable() {
        let boss_bar = EVENT_STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.triggered.fill(false);
            state.boss_bar.take()
        });
        if let Some(boss_bar) = boss_bar {
            let _ = boss_bar.remove_all_players();
        }
        let _ = log("event_test: on_disable is called!");
    }
}
impl exports::aegilex::endstone::events::Guest for EventTestPlugin {
    fn on_event(event: exported_events::Event<'_>) -> Result<(), String> {
        let event_index = event_index(&event);
        match event {
            exported_events::Event::PlayerLogin(login) => {
                let player = login
                    .get_player()
                    .map_err(|error| format!("player-login.get-player failed: {error:?}"))?;
                let message = login
                    .get_kick_message()
                    .map_err(|error| format!("player-login.get-kick-message failed: {error:?}"))?;
                log(&format!("{} logged in. kick message: {}", player_name(&player)?, message))?;
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
                log(&format!(
                    "{} joined the server.",
                    player_name(&player)?
                ))?;
                add_boss_bar_player(&player)?;
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
                let clicked_position = interact
                    .get_clicked_position()
                    .map_err(|error| format!("player-interact.get-clicked-position failed: {error:?}"))?;
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
                let player = interact
                    .get_player()
                    .map_err(|error| format!("player-interact-actor.get-player failed: {error:?}"))?;
                let actor = interact
                    .get_actor()
                    .map_err(|error| format!("player-interact-actor.get-actor failed: {error:?}"))?;
                log(&format!(
                    "{} interacts with actor {}",
                    player_name(&player)?,
                    actor_name(&actor)?
                ))?;
                let cancelled = interact
                    .is_cancelled()
                    .map_err(|error| format!("player-interact-actor.is-cancelled failed: {error:?}"))?;
                interact
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("player-interact-actor.set-cancelled failed: {error:?}"))?;
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
                let player = game_mode_change
                    .get_player()
                    .map_err(|error| format!("player-game-mode-change.get-player failed: {error:?}"))?;
                let mode = game_mode_change
                    .get_new_game_mode()
                    .map_err(|error| format!("player-game-mode-change.get-new-game-mode failed: {error:?}"))?;
                log(&format!(
                    "{} changed game mode to {:?}",
                    player_name(&player)?,
                    mode
                ))?;
                let cancelled = game_mode_change
                    .is_cancelled()
                    .map_err(|error| format!("player-game-mode-change.is-cancelled failed: {error:?}"))?;
                game_mode_change
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("player-game-mode-change.set-cancelled failed: {error:?}"))?;
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
                let cancelled = consume
                    .is_cancelled()
                    .map_err(|error| format!("player-item-consume.is-cancelled failed: {error:?}"))?;
                consume
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("player-item-consume.set-cancelled failed: {error:?}"))?;
            }
            exported_events::Event::PlayerItemHeld(held) => {
                let player = held
                    .get_player()
                    .map_err(|error| format!("player-item-held.get-player failed: {error:?}"))?;
                let previous_slot = held
                    .get_previous_slot()
                    .map_err(|error| format!("player-item-held.get-previous-slot failed: {error:?}"))?;
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
                log(&format!(
                    "{} picks up an item.",
                    player_name(&player)?
                ))?;
                let actor = pickup
                    .get_item_actor()
                    .map_err(|error| format!("player-pickup-item.get-item-actor failed: {error:?}"))?;
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
                let cancelled = pickup
                    .is_cancelled()
                    .map_err(|error| format!("player-pickup-item.is-cancelled failed: {error:?}"))?;
                pickup
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("player-pickup-item.set-cancelled failed: {error:?}"))?;
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
                let message = skin_change
                    .get_skin_change_message()
                    .map_err(|error| format!("player-skin-change.get-skin-change-message failed: {error:?}"))?;
                skin_change
                    .set_skin_change_message(message.as_ref())
                    .map_err(|error| format!("player-skin-change.set-skin-change-message failed: {error:?}"))?;
                let cancelled = skin_change
                    .is_cancelled()
                    .map_err(|error| format!("player-skin-change.is-cancelled failed: {error:?}"))?;
                skin_change
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("player-skin-change.set-cancelled failed: {error:?}"))?;
                log(&format!(
                    "{} changes skin.",
                    player_name(&player)?,
                ))?;
            }
            exported_events::Event::PlayerDimensionChange(dimension_change) => {
                let player = dimension_change
                    .get_player()
                    .map_err(|error| format!("player-dimension-change.get-player failed: {error:?}"))?;
                let from = dimension_change
                    .get_from_dimension()
                    .map_err(|error| format!("player-dimension-change.get-from-dimension failed: {error:?}"))?;
                let to = dimension_change
                    .get_to_dimension()
                    .map_err(|error| format!("player-dimension-change.get-to-dimension failed: {error:?}"))?;
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
                    block.get_type().map_err(|error| format!("block.get-type failed: {error:?}"))?
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
                    cook
                        .set_result(&apple)
                        .map_err(|error| format!("block-cook.set-result failed: {error:?}"))?;
                }
                let cancelled = cook
                    .is_cancelled()
                    .map_err(|error| format!("block-cook.is-cancelled failed: {error:?}"))?;
                cook
                    .set_cancelled(cancelled)
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
                    replaced.get_type().map_err(|error| format!("block.get-type failed: {error:?}"))?,
                    against.get_type().map_err(|error| format!("block.get-type failed: {error:?}"))?
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
                let block = grow.get_block().map_err(|error| format!("block-form.get-block failed: {error:?}"))?;
                let cancelled = grow.is_cancelled().map_err(|error| format!("block-form.is-cancelled failed: {error:?}"))?;
                grow.set_cancelled(cancelled).map_err(|error| format!("block-form.set-cancelled failed: {error:?}"))?;
                log(&format!("{} formed", block.get_type().map_err(|error| format!("block.get-type failed: {error:?}"))?))?;
            }
            exported_events::Event::BlockFromTo(from_to) => {
                let block = from_to.get_block().map_err(|error| format!("block-from-to.get-block failed: {error:?}"))?;
                let to_block = from_to.get_to_block().map_err(|error| format!("block-from-to.get-to-block failed: {error:?}"))?;
                let cancelled = from_to.is_cancelled().map_err(|error| format!("block-from-to.is-cancelled failed: {error:?}"))?;
                from_to.set_cancelled(cancelled).map_err(|error| format!("block-from-to.set-cancelled failed: {error:?}"))?;
                log(&format!("{} flowed to {}", block.get_type().map_err(|error| format!("block.get-type failed: {error:?}"))?, to_block.get_type().map_err(|error| format!("block.get-type failed: {error:?}"))?))?;
            }
            exported_events::Event::BlockGrow(grow) => {
                let block = grow.get_block().map_err(|error| format!("block-grow.get-block failed: {error:?}"))?;
                let cancelled = grow.is_cancelled().map_err(|error| format!("block-grow.is-cancelled failed: {error:?}"))?;
                grow.set_cancelled(cancelled).map_err(|error| format!("block-grow.set-cancelled failed: {error:?}"))?;
                log(&format!("{} grew", block.get_type().map_err(|error| format!("block.get-type failed: {error:?}"))?))?;
            }
            exported_events::Event::BlockPistonExtend(piston) => {
                let block = piston.get_block().map_err(|error| format!("block-piston-extend.get-block failed: {error:?}"))?;
                let direction = piston.get_direction().map_err(|error| format!("block-piston-extend.get-direction failed: {error:?}"))?;
                let cancelled = piston.is_cancelled().map_err(|error| format!("block-piston-extend.is-cancelled failed: {error:?}"))?;
                piston.set_cancelled(cancelled).map_err(|error| format!("block-piston-extend.set-cancelled failed: {error:?}"))?;
                log(&format!("Piston ({}) expands towards {direction:?}", block.get_type().map_err(|error| format!("block.get-type failed: {error:?}"))?))?;
            }
            exported_events::Event::BlockPistonRetract(piston) => {
                let block = piston.get_block().map_err(|error| format!("block-piston-retract.get-block failed: {error:?}"))?;
                let direction = piston.get_direction().map_err(|error| format!("block-piston-retract.get-direction failed: {error:?}"))?;
                let cancelled = piston.is_cancelled().map_err(|error| format!("block-piston-retract.is-cancelled failed: {error:?}"))?;
                piston.set_cancelled(cancelled).map_err(|error| format!("block-piston-retract.set-cancelled failed: {error:?}"))?;
                log(&format!("Piston ({}) retracts towards {direction:?}", block.get_type().map_err(|error| format!("block.get-type failed: {error:?}"))?))?;
            }
            exported_events::Event::LeavesDecay(leaves) => {
                let block = leaves.get_block().map_err(|error| format!("leaves-decay.get-block failed: {error:?}"))?;
                let cancelled = leaves.is_cancelled().map_err(|error| format!("leaves-decay.is-cancelled failed: {error:?}"))?;
                leaves.set_cancelled(cancelled).map_err(|error| format!("leaves-decay.set-cancelled failed: {error:?}"))?;
                log(&format!("Leaves ({}) decayed", block.get_type().map_err(|error| format!("block.get-type failed: {error:?}"))?))?;
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
                message
                    .set_cancelled(cancelled)
                    .map_err(|error| format!("broadcast-message.set-cancelled failed: {error:?}"))?;
            }
            exported_events::Event::ServerListPing(ping) => {
                let motd = ping
                    .get_motd()
                    .map_err(|error| format!("server-list-ping.get-motd failed: {error:?}"))?;
                let server_guid = ping
                    .get_server_guid()
                    .map_err(|error| format!("server-list-ping.get-server-guid failed: {error:?}"))?;
                let local_port = ping
                    .get_local_port()
                    .map_err(|error| format!("server-list-ping.get-local-port failed: {error:?}"))?;
                log(&format!(
                    "ServerListPingEvent is called (motd: {}, port: {})",
                    motd, local_port
                ))?;
                ping.set_motd(&format!("**{motd}**"))
                    .map_err(|error| format!("server-list-ping.set-motd failed: {error:?}"))?;
                ping.set_server_guid(&server_guid)
                    .map_err(|error| format!("server-list-ping.set-server-guid failed: {error:?}"))?;
                ping.set_local_port(local_port)
                    .map_err(|error| format!("server-list-ping.set-local-port failed: {error:?}"))?;
                let local_port_v6 = ping
                    .get_local_port_v6()
                    .map_err(|error| format!("server-list-ping.get-local-port-v6 failed: {error:?}"))?;
                ping.set_local_port_v6(local_port_v6)
                    .map_err(|error| format!("server-list-ping.set-local-port-v6 failed: {error:?}"))?;
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
        record_event(event_index)
    }
}


impl exports::aegilex::endstone::commands::Guest for EventTestPlugin {
    fn on_command(
        command: exported_commands::Invocation,
    ) -> Result<exported_commands::Outcome, String> {
        if command.subcommand != "test" {
            return Ok(exported_commands::Outcome {
                handled: false,
                reply: None,
                error: None,
            });
        }
        let reply = match command.args.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
            [] | ["status"] => Some(event_status()),
            ["count", event] => Some(format!("received {event} argument via the event-test command")),
            ["form", form_type] => Some(format!("form type {form_type} accepted")),
            ["spawn", entity] => Some(format!("spawn request for {entity} accepted")),
            ["block", ..] => Some("block argument accepted".to_owned()),
            ["broadcast", ..] => Some("broadcast request accepted".to_owned()),
            args => Some(format!("unknown event-test arguments: {args:?}")),
        };
        Ok(exported_commands::Outcome {
            handled: true,
            reply,
            error: None,
        })
    }
}

impl exports::aegilex::endstone::tasks::Guest for EventTestPlugin {
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

impl exports::aegilex::endstone::form_callbacks::Guest for EventTestPlugin {
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

impl exports::aegilex::endstone::service_callbacks::Guest for EventTestPlugin {
    fn on_service_request(
        _request: exports::aegilex::endstone::service_callbacks::ServiceRequest,
    ) -> Result<exports::aegilex::endstone::service_callbacks::ServiceResponse, String> {
        Err("not implemented by event-test-component".to_owned())
    }
}

impl exports::aegilex::endstone::map_renderer_callbacks::Guest for EventTestPlugin {
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

fn log(message: &str) -> Result<(), String> {
    let logger =
        logger::get_logger().map_err(|error| format!("get logger failed: {error:?}"))?;
    logger
        .log(logger::LogLevel::Info, message)
        .map_err(|error| format!("log failed: {error:?}"))
}

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

fn message_text(message: &message::Message) -> &str {
    match message {
        message::Message::PlainText(text) => text,
        message::Message::Translatable(_) => "<translatable>",
    }
}

fn event_index(event: &exported_events::Event<'_>) -> usize {
    match event {
        exported_events::Event::PlayerLogin(_) => 0,
        exported_events::Event::PlayerJoin(_) => 1,
        exported_events::Event::PlayerEmote(_) => 2,
        exported_events::Event::PlayerInteract(_) => 3,
        exported_events::Event::PlayerInteractActor(_) => 4,
        exported_events::Event::PlayerKick(_) => 5,
        exported_events::Event::PlayerCommand(_) => 6,
        exported_events::Event::PlayerQuit(_) => 7,
        exported_events::Event::PlayerChat(_) => 8,
        exported_events::Event::PlayerGameModeChange(_) => 9,
        exported_events::Event::PlayerJump(_) => 10,
        exported_events::Event::PlayerMove(_) => 11,
        exported_events::Event::PlayerTeleport(_) => 12,
        exported_events::Event::PlayerPortal(_) => 13,
        exported_events::Event::PlayerDeath(_) => 14,
        exported_events::Event::PlayerRespawn(_) => 15,
        exported_events::Event::PlayerItemConsume(_) => 16,
        exported_events::Event::PlayerItemHeld(_) => 17,
        exported_events::Event::PlayerDropItem(_) => 18,
        exported_events::Event::PlayerPickupItem(_) => 19,
        exported_events::Event::PlayerBedEnter(_) => 20,
        exported_events::Event::PlayerBedLeave(_) => 21,
        exported_events::Event::PlayerSkinChange(_) => 22,
        exported_events::Event::PlayerDimensionChange(_) => 23,
        exported_events::Event::ActorDamage(_) => 24,
        exported_events::Event::ActorDeath(_) => 25,
        exported_events::Event::ActorExplode(_) => 26,
        exported_events::Event::ActorKnockback(_) => 27,
        exported_events::Event::ActorRemove(_) => 28,
        exported_events::Event::ActorSpawn(_) => 29,
        exported_events::Event::ActorTeleport(_) => 30,
        exported_events::Event::BlockBreak(_) => 31,
        exported_events::Event::BlockCook(_) => 32,
        exported_events::Event::BlockPlace(_) => 33,
        exported_events::Event::BlockExplode(_) => 34,
        exported_events::Event::BlockForm(_) => 35,
        exported_events::Event::BlockFromTo(_) => 36,
        exported_events::Event::BlockGrow(_) => 37,
        exported_events::Event::BlockPistonExtend(_) => 38,
        exported_events::Event::BlockPistonRetract(_) => 39,
        exported_events::Event::LeavesDecay(_) => 40,
        exported_events::Event::ChunkLoad(_) => 41,
        exported_events::Event::ChunkUnload(_) => 42,
        exported_events::Event::WeatherChange(_) => 43,
        exported_events::Event::ThunderChange(_) => 44,
        exported_events::Event::ServerCommand(_) => 45,
        exported_events::Event::BroadcastMessage(_) => 46,
        exported_events::Event::ServerListPing(_) => 47,
        exported_events::Event::PluginEnable(_) => 48,
        exported_events::Event::PluginDisable(_) => 49,
        exported_events::Event::ServerLoad(_) => 50,
        exported_events::Event::PacketSend(_) => 51,
        exported_events::Event::PacketReceive(_) => 52,
        exported_events::Event::MapInitialize(_) => 53,
        exported_events::Event::ScriptMessage(_) => 54,
    }
}

fn add_boss_bar_player(player: &player::Player) -> Result<(), String> {
    EVENT_STATE.with(|state| {
        state
            .borrow()
            .boss_bar
            .as_ref()
            .ok_or_else(|| "event boss bar is unavailable".to_owned())?
            .add_player(player)
            .map_err(|error| format!("boss-bar.add-player failed: {error:?}"))
    })
}

fn event_status() -> String {
    EVENT_STATE.with(|state| {
        let state = state.borrow();
        let triggered = state.triggered.iter().filter(|triggered| **triggered).count();
        let next = state
            .triggered
            .iter()
            .position(|triggered| !triggered)
            .map(|index| EVENT_NAMES[index]);
        match next {
            Some(next) => format!("event-test: {triggered}/{} event kinds triggered; next: {next}", EVENT_NAMES.len()),
            None => format!("event-test: all {} event kinds triggered", EVENT_NAMES.len()),
        }
    })
}

fn record_event(event_index: usize) -> Result<(), String> {
    EVENT_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.triggered[event_index] = true;
        let triggered = state.triggered.iter().filter(|triggered| **triggered).count();
        let next = state
            .triggered
            .iter()
            .position(|triggered| !triggered)
            .map(|index| EVENT_NAMES[index]);
        let title = match next {
            Some(next) => format!(
                "Events: {triggered}/{}, Last: {}, Next: {next}",
                EVENT_NAMES.len(),
                EVENT_NAMES[event_index]
            ),
            None => format!("All {} events triggered! Last: {}", EVENT_NAMES.len(), EVENT_NAMES[event_index]),
        };
        let boss_bar = state
            .boss_bar
            .as_ref()
            .ok_or_else(|| "event boss bar is unavailable".to_owned())?;
        boss_bar
            .set_progress(triggered as f32 / EVENT_NAMES.len() as f32)
            .map_err(|error| format!("boss-bar.set-progress failed: {error:?}"))?;
        boss_bar
            .set_title(&title)
            .map_err(|error| format!("boss-bar.set-title failed: {error:?}"))
    })
}

export!(EventTestPlugin);
