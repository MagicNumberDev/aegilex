//! Shared resource resolution and ABI conversion for Endstone facade modules.
//!
//! Concrete `Host*` implementations live beside the matching Endstone header.

#[allow(unused_imports)]
pub(crate) use std::collections::BTreeMap;

#[allow(unused_imports)]
pub(crate) use crate::abi::{AEGILEX_INVALID_ARGUMENT, AEGILEX_NOT_FOUND};
#[allow(unused_imports)]
pub(crate) use crate::core_host::imports::*;
#[allow(unused_imports)]
pub(crate) use crate::core_host::{
    ActionFormActionControl, BanListBanEntry, BanListBanListKind, BarColorBossBarColor,
    BarFlagBossBarFlag, BarStyleBossBarStyle, BlockDataBlockStatePair, BlockDataBlockStateValue,
    BookMetaBookGeneration, ChunkChunk, CriteriaCriteria, DimensionDimension,
    DimensionDimensionKind, DisplaySlotDisplaySlot, FormButtonButton, FormDropdownDropdown,
    FormHeaderHeader, FormLabelLabel, FormSliderSlider, FormTextInputTextInput, FormToggleToggle,
    GameModeGameMode, InventoryInventoryItem, InventoryItemMatcher, InventoryItemRequest,
    InventoryItemStackResult, ItemMetaEnchantment, ItemMetaItemMetaType, ItemTypeItemTypeData,
    LevelLevel, LocationLocation, MapViewMapScale, MessageMessage, ModalFormModalControl,
    NbtCompoundEntry, NbtTagType, ObjectiveSortOrderObjectiveSortOrder,
    PermissibleEffectivePermission, PermissionAttachmentPermissionChild,
    PermissionDefaultPermissionDefault, PermissionLevelPermissionLevel, PlayerFormFormSpec,
    PluginCommandPluginCommandData, PluginManagerPluginInfo, PluginTypesLoadOrder,
    RenderTypeRenderType, ScoreScore, ServiceBusServiceCallStatus,
    ServiceBusServiceProviderInfo, ServiceBusServiceResponse, ServiceBusServiceSpec,
    ServicePriorityServicePriority, TaskTask, TranslatableTranslatable, TypesHostError,
    VectorVector,
};
#[allow(unused_imports)]
pub(crate) use crate::cxx_host_actor::ffi as cxx_actor;
#[allow(unused_imports)]
pub(crate) use crate::cxx_host_admin::ffi as cxx_admin;
#[allow(unused_imports)]
pub(crate) use crate::cxx_host_common::ffi as cxx_common;
#[allow(unused_imports)]
pub(crate) use crate::cxx_host_inventory::ffi as cxx_inventory;
#[allow(unused_imports)]
pub(crate) use crate::cxx_host_level::ffi as cxx_level;
#[allow(unused_imports)]
pub(crate) use crate::cxx_host_player::ffi as cxx_player;
#[allow(unused_imports)]
pub(crate) use crate::cxx_host_server::ffi as cxx_server;
#[allow(unused_imports)]
pub(crate) use crate::cxx_host_ui::ffi as cxx_ui;
#[allow(unused_imports)]
pub(crate) use crate::cxx_runtime::ffi as cxx_event;
#[allow(unused_imports)]
pub(crate) use crate::host::runtime::core::{check_capability, map_core_host_error};
#[allow(unused_imports)]
pub(crate) use crate::host::runtime::handles::ResourceKind;
#[allow(unused_imports)]
pub(crate) use crate::host::runtime::native::{self, HostContext, HostError};
#[allow(unused_imports)]
pub(crate) use crate::host::runtime::store_shared::*;
#[allow(unused_imports)]
pub(crate) use crate::runtime::PluginStoreState;

pub(crate) fn resolve_server(state: &PluginStoreState) -> Result<&cxx_server::Server, HostError> {
    state.host.server()
}

pub(crate) fn resolve_actor(
    state: &PluginStoreState,
    handle: u64,
) -> Result<&cxx_actor::Actor, HostError> {
    state
        .handles
        .actor(state.invocation_id, handle)
        .ok_or_else(not_found)
}

pub(crate) fn resolve_mob(
    state: &PluginStoreState,
    handle: u64,
) -> Result<&cxx_actor::Mob, HostError> {
    state
        .handles
        .mob(state.invocation_id, handle)
        .ok_or_else(not_found)
}

pub(crate) fn actor_handle(state: &PluginStoreState, actor: u32) -> Result<u64, HostError> {
    state
        .resource_slot(actor, ResourceKind::Actor)
        .map(|slot| slot.handle)
}

pub(crate) fn mob_handle(state: &PluginStoreState, mob: u32) -> Result<u64, HostError> {
    state
        .resource_slot(mob, ResourceKind::Mob)
        .map(|slot| slot.handle)
}

pub(crate) fn item_actor_handle(
    state: &PluginStoreState,
    item_actor: u32,
) -> Result<u64, HostError> {
    state
        .resource_slot(item_actor, ResourceKind::ItemActor)
        .map(|slot| slot.handle)
}

pub(crate) fn resolve_item_actor(
    state: &PluginStoreState,
    handle: u64,
) -> Result<&cxx_actor::ItemActor, HostError> {
    state
        .handles
        .item_actor(state.invocation_id, handle)
        .ok_or_else(not_found)
}

pub(crate) fn resolve_player(
    state: &PluginStoreState,
    handle: u64,
) -> Result<&cxx_player::Player, HostError> {
    state
        .handles
        .player(state.invocation_id, handle)
        .ok_or_else(not_found)
}

pub(crate) fn player_handle(state: &PluginStoreState, player: u32) -> Result<u64, HostError> {
    state
        .resource_slot(player, ResourceKind::Player)
        .map(|slot| slot.handle)
}

pub(crate) fn to_game_mode(mode: GameModeGameMode) -> cxx_player::GameMode {
    match mode {
        GameModeGameMode::Survival => cxx_player::GameMode::Survival,
        GameModeGameMode::Creative => cxx_player::GameMode::Creative,
        GameModeGameMode::Adventure => cxx_player::GameMode::Adventure,
        GameModeGameMode::Spectator => cxx_player::GameMode::Spectator,
    }
}

pub(crate) fn from_game_mode(mode: cxx_player::GameMode) -> GameModeGameMode {
    match mode {
        cxx_player::GameMode::Survival => GameModeGameMode::Survival,
        cxx_player::GameMode::Creative => GameModeGameMode::Creative,
        cxx_player::GameMode::Adventure => GameModeGameMode::Adventure,
        cxx_player::GameMode::Spectator => GameModeGameMode::Spectator,
        _ => GameModeGameMode::Survival,
    }
}

pub(crate) fn resolve_level(
    host: HostContext,
) -> Result<cxx::UniquePtr<cxx_level::Level>, HostError> {
    let level = host.server()?.getLevel();
    if level.is_null() {
        return Err(HostError::from_status(AEGILEX_NOT_FOUND));
    }
    Ok(level)
}

pub(crate) fn resolve_block(
    state: &PluginStoreState,
    handle: u64,
) -> Result<&cxx_level::Block, HostError> {
    state
        .handles
        .block(state.invocation_id, handle)
        .ok_or_else(not_found)
}

pub(crate) fn block_handle(state: &PluginStoreState, block: u32) -> Result<u64, HostError> {
    state
        .resource_slot(block, ResourceKind::Block)
        .map(|slot| slot.handle)
}

pub(crate) fn resolve_boss_bar(
    state: &PluginStoreState,
    handle: u64,
) -> Result<&cxx_ui::BossBar, HostError> {
    state
        .handles
        .boss_bar(state.invocation_id, handle)
        .ok_or_else(not_found)
}

pub(crate) fn boss_bar_handle(state: &PluginStoreState, bar: u32) -> Result<u64, HostError> {
    state
        .resource_slot(bar, ResourceKind::BossBar)
        .map(|slot| slot.handle)
}

pub(crate) fn resolve_scoreboard(
    state: &PluginStoreState,
    handle: u64,
) -> Result<&cxx_ui::Scoreboard, HostError> {
    state
        .handles
        .scoreboard(state.invocation_id, handle)
        .ok_or_else(not_found)
}

pub(crate) fn scoreboard_handle(
    state: &PluginStoreState,
    scoreboard: u32,
) -> Result<u64, HostError> {
    state
        .resource_slot(scoreboard, ResourceKind::Scoreboard)
        .map(|slot| slot.handle)
}

pub(crate) fn resolve_score_entry(
    state: &PluginStoreState,
    handle: u64,
) -> Result<&cxx_ui::ScoreEntry, HostError> {
    state
        .handles
        .score_entry(state.invocation_id, handle)
        .ok_or_else(not_found)
}

pub(crate) fn score_entry_handle(state: &PluginStoreState, entry: u32) -> Result<u64, HostError> {
    state
        .resource_slot(entry, ResourceKind::ScoreEntry)
        .map(|slot| slot.handle)
}

pub(crate) fn resolve_ban_list(
    state: &PluginStoreState,
    handle: u64,
) -> Result<&cxx_admin::BanList, HostError> {
    state
        .handles
        .ban_list(state.invocation_id, handle)
        .ok_or_else(not_found)
}

pub(crate) fn ban_list_handle(state: &PluginStoreState, ban_list: u32) -> Result<u64, HostError> {
    state
        .resource_slot(ban_list, ResourceKind::BanList)
        .map(|slot| slot.handle)
}

pub(crate) fn resolve_attachment(
    state: &PluginStoreState,
    handle: u64,
) -> Result<&cxx_admin::PermissionAttachment, HostError> {
    state
        .handles
        .permission_attachment(state.invocation_id, handle)
        .ok_or_else(not_found)
}

pub(crate) fn attachment_handle(
    state: &PluginStoreState,
    attachment: u32,
) -> Result<u64, HostError> {
    state
        .resource_slot(attachment, ResourceKind::PermissionAttachment)
        .map(|slot| slot.handle)
}

pub(crate) fn resolve_permissible(
    state: &PluginStoreState,
    handle: u64,
) -> Result<&cxx_common::Permissible, HostError> {
    state
        .handles
        .permissible(state.invocation_id, handle)
        .ok_or_else(not_found)
}

pub(crate) fn permissible_handle(
    state: &PluginStoreState,
    permissible: u32,
) -> Result<u64, HostError> {
    state
        .resource_slot(permissible, ResourceKind::Permissible)
        .map(|slot| slot.handle)
}

pub(crate) fn resolve_definition(
    state: &PluginStoreState,
    handle: u64,
) -> Result<&cxx_admin::PermissionDefinition, HostError> {
    state
        .handles
        .permission_definition(state.invocation_id, handle)
        .ok_or_else(not_found)
}

pub(crate) fn definition_handle(
    state: &PluginStoreState,
    definition: u32,
) -> Result<u64, HostError> {
    state
        .resource_slot(definition, ResourceKind::PermissionDefinition)
        .map(|slot| slot.handle)
}

pub(crate) fn resolve_sender(
    state: &PluginStoreState,
    handle: u64,
) -> Result<&cxx_common::CommandSender, HostError> {
    state
        .handles
        .command_sender(state.invocation_id, handle)
        .ok_or_else(not_found)
}

pub(crate) fn sender_handle(state: &PluginStoreState, sender: u32) -> Result<u64, HostError> {
    state
        .resource_slot(sender, ResourceKind::CommandSender)
        .map(|slot| slot.handle)
}

pub(crate) fn permission_default(
    value: u8,
) -> Result<PermissionDefaultPermissionDefault, HostError> {
    match value {
        0 => Ok(PermissionDefaultPermissionDefault::True),
        1 => Ok(PermissionDefaultPermissionDefault::False),
        2 => Ok(PermissionDefaultPermissionDefault::Operator),
        3 => Ok(PermissionDefaultPermissionDefault::NotOperator),
        4 => Ok(PermissionDefaultPermissionDefault::Console),
        _ => Ok(PermissionDefaultPermissionDefault::False),
    }
}

pub(crate) fn native_permission_default(value: PermissionDefaultPermissionDefault) -> u8 {
    match value {
        PermissionDefaultPermissionDefault::True => 0,
        PermissionDefaultPermissionDefault::False => 1,
        PermissionDefaultPermissionDefault::Operator => 2,
        PermissionDefaultPermissionDefault::NotOperator => 3,
        PermissionDefaultPermissionDefault::Console => 4,
    }
}

pub(crate) fn priority_value(priority: &ServicePriorityServicePriority) -> u32 {
    match priority {
        ServicePriorityServicePriority::Lowest => 0,
        ServicePriorityServicePriority::Low => 1,
        ServicePriorityServicePriority::Normal => 2,
        ServicePriorityServicePriority::High => 3,
        ServicePriorityServicePriority::Highest => 4,
    }
}

pub(crate) fn status_from_value(value: u32) -> ServiceBusServiceCallStatus {
    match value {
        crate::runtime::SERVICE_STATUS_COMPLETED => ServiceBusServiceCallStatus::Completed,
        crate::runtime::SERVICE_STATUS_REJECTED => ServiceBusServiceCallStatus::Rejected,
        crate::runtime::SERVICE_STATUS_FAILED => ServiceBusServiceCallStatus::Failed,
        crate::runtime::SERVICE_STATUS_CANCELLED => ServiceBusServiceCallStatus::Cancelled,
        crate::runtime::SERVICE_STATUS_EXPIRED => ServiceBusServiceCallStatus::Expired,
        _ => ServiceBusServiceCallStatus::Pending,
    }
}

pub(crate) fn ban_entry(entry: cxx_admin::PlayerBanEntry) -> BanListBanEntry {
    BanListBanEntry {
        target: entry.name,
        uuid: entry.has_uuid.then_some(entry.uuid.to_vec()),
        xuid: entry.has_xuid.then_some(entry.xuid),
        reason: entry.reason,
        source: entry.source,
        created_ms: entry.created,
        expiration_ms: entry.has_expires.then_some(entry.expires),
    }
}

pub(crate) fn insert_block_data_resource(
    state: &mut PluginStoreState,
    data: cxx_level::BlockData,
) -> Result<u32, HostError> {
    state.insert_owned_resource(data)
}

pub(crate) fn block_data_value(
    state: &PluginStoreState,
    data: u32,
) -> Result<&cxx_level::BlockData, HostError> {
    state.owned_resource(data)
}

pub(crate) fn insert_block_state_resource(
    state: &mut PluginStoreState,
    snapshot: cxx_level::BlockSnapshot,
) -> Result<u32, HostError> {
    state.insert_owned_resource(snapshot)
}

pub(crate) fn block_state_value(
    state: &PluginStoreState,
    block_state: u32,
) -> Result<&cxx_level::BlockSnapshot, HostError> {
    state.owned_resource(block_state)
}

pub(crate) fn block_state_value_mut(
    state: &mut PluginStoreState,
    block_state: u32,
) -> Result<&mut cxx_level::BlockSnapshot, HostError> {
    state.owned_resource_mut(block_state)
}

pub(crate) fn wit_dimension_kind(raw: u32) -> Result<DimensionDimensionKind, HostError> {
    match raw {
        0 => Ok(DimensionDimensionKind::Overworld),
        1 => Ok(DimensionDimensionKind::Nether),
        2 => Ok(DimensionDimensionKind::TheEnd),
        _ => Err(not_found()),
    }
}

pub(crate) fn service_provider_info_from_cxx(
    provider: cxx_event::ServiceProviderData,
) -> ServiceBusServiceProviderInfo {
    ServiceBusServiceProviderInfo {
        id: provider.id,
        spec: ServiceBusServiceSpec {
            name: provider.name,
            version: provider.version,
            methods: provider.methods,
            priority: match provider.priority {
                0 => ServicePriorityServicePriority::Lowest,
                1 => ServicePriorityServicePriority::Low,
                2 => ServicePriorityServicePriority::Normal,
                3 => ServicePriorityServicePriority::High,
                4 => ServicePriorityServicePriority::Highest,
                _ => ServicePriorityServicePriority::Normal,
            },
        },
    }
}

pub(crate) fn wit_uuid(value: Option<Vec<u8>>) -> Result<(bool, [u8; 16]), TypesHostError> {
    match value {
        Some(value) => Ok((
            true,
            value.try_into().map_err(|_| TypesHostError::InvalidInput)?,
        )),
        None => Ok((false, [0; 16])),
    }
}

/// Resolves the plugin facade of the currently-invoked plugin. The group1
/// header's `resolve_plugin` takes a handle; this one resolves by plugin id.
pub(crate) fn plugin_context_plugin(
    state: &PluginStoreState,
) -> Result<cxx::UniquePtr<cxx_server::Plugin>, HostError> {
    let plugin = state.host.server()?.getPlugin(&state.plugin_id);
    if plugin.is_null() {
        return Err(HostError::from_status(AEGILEX_NOT_FOUND));
    }
    Ok(plugin)
}

pub(crate) fn plugin_load_order(value: u8) -> Result<PluginTypesLoadOrder, HostError> {
    match value {
        0 => Ok(PluginTypesLoadOrder::Startup),
        1 => Ok(PluginTypesLoadOrder::PostWorld),
        _ => Err(HostError::from_status(AEGILEX_INVALID_ARGUMENT)),
    }
}

pub(crate) fn empty_ban_entry() -> cxx_admin::PlayerBanEntry {
    cxx_admin::PlayerBanEntry {
        name: String::new(),
        has_uuid: false,
        uuid: [0; 16],
        has_xuid: false,
        xuid: String::new(),
        has_reason: false,
        reason: String::new(),
        source: String::new(),
        created: 0,
        has_expires: false,
        expires: 0,
    }
}

pub(crate) fn native_block_state_pair(
    pair: &BlockDataBlockStatePair,
) -> Result<cxx_level::BlockStatePair, HostError> {
    if pair.key.is_empty() {
        return Err(invalid_input());
    }
    let (value_kind, boolean, text, integer) = match &pair.value {
        BlockDataBlockStateValue::Boolean(value) => (0, *value, String::new(), 0),
        BlockDataBlockStateValue::Text(value) => (1, false, value.clone(), 0),
        BlockDataBlockStateValue::Integer(value) => (2, false, String::new(), *value),
    };
    Ok(cxx_level::BlockStatePair {
        key: pair.key.clone(),
        value_kind,
        boolean,
        text,
        integer,
    })
}

pub(crate) fn native_block_state_pairs(
    states: &[BlockDataBlockStatePair],
) -> Result<Vec<cxx_level::BlockStatePair>, HostError> {
    states.iter().map(native_block_state_pair).collect()
}

pub(crate) fn wit_block_state_pair_native(
    pair: &cxx_level::BlockStatePair,
) -> BlockDataBlockStatePair {
    let value = match pair.value_kind {
        0 => BlockDataBlockStateValue::Boolean(pair.boolean),
        1 => BlockDataBlockStateValue::Text(pair.text.clone()),
        2 => BlockDataBlockStateValue::Integer(pair.integer),
        _ => BlockDataBlockStateValue::Boolean(false),
    };
    BlockDataBlockStatePair {
        key: pair.key.clone(),
        value,
    }
}

pub(crate) fn copy_native_block_data(data: &cxx_level::BlockData) -> cxx_level::BlockData {
    cxx_level::BlockData {
        type_id: data.type_id.clone(),
        states: data
            .states
            .iter()
            .map(|state| cxx_level::BlockStatePair {
                key: state.key.clone(),
                value_kind: state.value_kind,
                boolean: state.boolean,
                text: state.text.clone(),
                integer: state.integer,
            })
            .collect(),
        runtime_id: data.runtime_id,
    }
}

pub(crate) fn cxx_location_native(location: &LocationLocation) -> cxx_level::Location {
    cxx_level::Location {
        x: location.x,
        y: location.y,
        z: location.z,
        pitch: location.pitch,
        yaw: location.yaw,
        dimension: location.dimension.clone(),
    }
}

pub(crate) fn wit_location_native(location: cxx_level::Location) -> LocationLocation {
    LocationLocation {
        dimension: location.dimension,
        x: location.x,
        y: location.y,
        z: location.z,
        pitch: location.pitch,
        yaw: location.yaw,
    }
}

pub(crate) fn wit_dimension_summary_native(
    dimension: &cxx::UniquePtr<cxx_level::Dimension>,
) -> Result<DimensionDimension, HostError> {
    Ok(DimensionDimension {
        name: dimension.getName(),
        kind: wit_dimension_kind(dimension.getType())?,
        level: dimension.getLevelName(),
    })
}

// ---- ui conversions (current cxx facades) ----

pub(crate) fn from_color(color: u32) -> Result<BarColorBossBarColor, HostError> {
    match color {
        0 => Ok(BarColorBossBarColor::Pink),
        1 => Ok(BarColorBossBarColor::Blue),
        2 => Ok(BarColorBossBarColor::Red),
        3 => Ok(BarColorBossBarColor::Green),
        4 => Ok(BarColorBossBarColor::Yellow),
        5 => Ok(BarColorBossBarColor::Purple),
        6 => Ok(BarColorBossBarColor::RebeccaPurple),
        7 => Ok(BarColorBossBarColor::White),
        _ => Err(invalid_input()),
    }
}

pub(crate) fn from_style(style: u32) -> Result<BarStyleBossBarStyle, HostError> {
    match style {
        0 => Ok(BarStyleBossBarStyle::Solid),
        1 => Ok(BarStyleBossBarStyle::Segmented6),
        2 => Ok(BarStyleBossBarStyle::Segmented10),
        3 => Ok(BarStyleBossBarStyle::Segmented12),
        4 => Ok(BarStyleBossBarStyle::Segmented20),
        _ => Err(invalid_input()),
    }
}

pub(crate) fn from_slot(slot: u32) -> Result<DisplaySlotDisplaySlot, HostError> {
    match slot {
        0 => Ok(DisplaySlotDisplaySlot::BelowName),
        1 => Ok(DisplaySlotDisplaySlot::PlayerList),
        2 => Ok(DisplaySlotDisplaySlot::SideBar),
        _ => Err(invalid_input()),
    }
}

pub(crate) fn from_sort_order(
    order: u32,
) -> Result<ObjectiveSortOrderObjectiveSortOrder, HostError> {
    match order {
        0 => Ok(ObjectiveSortOrderObjectiveSortOrder::Ascending),
        1 => Ok(ObjectiveSortOrderObjectiveSortOrder::Descending),
        _ => Err(invalid_input()),
    }
}

pub(crate) fn to_render_type(render_type: RenderTypeRenderType) -> u8 {
    match render_type {
        RenderTypeRenderType::Integer => 0,
        RenderTypeRenderType::Hearts => 1,
    }
}

pub(crate) fn from_render_type(render_type: u8) -> Result<RenderTypeRenderType, HostError> {
    match render_type {
        0 => Ok(RenderTypeRenderType::Integer),
        1 => Ok(RenderTypeRenderType::Hearts),
        _ => Err(invalid_input()),
    }
}

// ---- score helpers ----

pub(crate) fn get_scores(
    scoreboard: &cxx_ui::Scoreboard,
    entry: &cxx_ui::ScoreEntry,
) -> Result<Vec<ScoreScore>, HostError> {
    let list = scoreboard.getScores(entry);
    native::status_result(list.status)?;
    Ok(list
        .scores
        .into_iter()
        .map(|score| ScoreScore {
            objective_name: score.objective_name,
            value: score.value,
            score_set: score.score_set,
        })
        .collect())
}

pub(crate) fn get_objective_score(
    objective: &cxx_ui::Objective,
    entry: &cxx_ui::ScoreEntry,
) -> Result<i32, HostError> {
    let result = objective.getScoreValue(entry);
    native::status_result(result.status)?;
    Ok(result.value)
}

pub(crate) fn set_objective_score(
    objective: &cxx_ui::Objective,
    entry: &cxx_ui::ScoreEntry,
    value: i32,
) -> Result<(), HostError> {
    let changed = objective.setScoreValue(entry, value);
    changed.then_some(()).ok_or_else(not_found)
}

// ---- map view (ids are server-allocated i64; no resource table) ----

pub(crate) fn resolve_map_native(
    state: &PluginStoreState,
    id: i64,
) -> Result<cxx::UniquePtr<cxx_ui::Map>, HostError> {
    let map = state.host.server()?.getMap(id);
    if map.is_null() {
        return Err(not_found());
    }
    Ok(map)
}

// ---- objective owned resource ----

pub(crate) struct ObjectiveValue {
    pub(crate) scoreboard: u64,
    pub(crate) name: String,
}

pub(crate) fn insert_objective_value(
    state: &mut PluginStoreState,
    scoreboard: u64,
    name: String,
) -> Result<u32, HostError> {
    state.insert_owned_resource(ObjectiveValue { scoreboard, name })
}

pub(crate) fn objective_value(
    state: &PluginStoreState,
    objective: u32,
) -> Result<&ObjectiveValue, HostError> {
    state.owned_resource(objective)
}

pub(crate) fn resolve_objective_value(
    state: &PluginStoreState,
    value: &ObjectiveValue,
) -> Result<cxx::UniquePtr<cxx_ui::Objective>, HostError> {
    let objective = resolve_scoreboard(state, value.scoreboard)?.getObjective(&value.name);
    if objective.is_null() {
        return Err(not_found());
    }
    Ok(objective)
}
