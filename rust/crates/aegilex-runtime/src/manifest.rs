mod metadata;
mod module;
mod policy;
#[path = "manifest_tests.rs"]
#[cfg(test)]
mod tests;

pub(crate) use metadata::validate_metadata;
pub(crate) use module::inspect_module_path;
#[cfg(test)]
pub(crate) use policy::is_known_capability;
pub(crate) use policy::load_plugin_policy;
use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use crate::config::RuntimeConfig;

const MODULE_NAME: &str = "plugin.wasm";
const POLICY_NAME: &str = "aegilex.toml";
const KNOWN_SUBSCRIPTIONS: [&str; 55] = [
    "player-join",
    "player-quit",
    "player-chat",
    "player-kick",
    "player-command",
    "player-game-mode-change",
    "player-teleport",
    "player-drop-item",
    "block-break",
    "block-place",
    "actor-damage",
    "server-command",
    "player-bed-enter",
    "player-bed-leave",
    "player-dimension-change",
    "player-emote",
    "player-interact",
    "player-interact-actor",
    "player-item-consume",
    "player-item-held",
    "player-jump",
    "player-login",
    "player-move",
    "player-pickup-item",
    "player-portal",
    "player-respawn",
    "player-skin-change",
    "block-cook",
    "block-explode",
    "block-form",
    "block-from-to",
    "block-grow",
    "block-piston-extend",
    "block-piston-retract",
    "leaves-decay",
    "chunk-load",
    "chunk-unload",
    "weather-change",
    "thunder-change",
    "actor-death",
    "actor-explode",
    "actor-knockback",
    "actor-remove",
    "actor-spawn",
    "actor-teleport",
    "player-death",
    "broadcast-message",
    "server-list-ping",
    "plugin-enable",
    "plugin-disable",
    "server-load",
    "packet-send",
    "packet-receive",
    "map-initialize",
    "script-message",
];
mod capabilities {
    include!(concat!(env!("OUT_DIR"), "/capabilities.rs"));
}

use capabilities::KNOWN_CAPABILITIES;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct PluginPolicy {
    pub(crate) capabilities: Vec<String>,
    pub(crate) paths: Vec<PreopenedPath>,
    pub(crate) network: Vec<NetworkRule>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PreopenedPath {
    pub(crate) host_path: PathBuf,
    pub(crate) guest_path: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NetworkProtocol {
    Tcp,
    Udp,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct NetworkRule {
    pub(crate) protocol: NetworkProtocol,
    pub(crate) address: SocketAddr,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PluginMetadata {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) description: String,
    pub(crate) load_order: PluginLoadOrder,
    pub(crate) authors: Vec<String>,
    pub(crate) contributors: Vec<String>,
    pub(crate) website: String,
    pub(crate) prefix: String,
    pub(crate) provides: Vec<String>,
    pub(crate) depend: Vec<String>,
    pub(crate) soft_depend: Vec<String>,
    pub(crate) load_before: Vec<String>,
    pub(crate) default_permission: u32,
    pub(crate) commands: Vec<CommandSpec>,
    pub(crate) permissions: Vec<PermissionSpec>,
    pub(crate) subscriptions: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CommandSpec {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) aliases: Vec<String>,
    pub(crate) usages: Vec<String>,
    pub(crate) permissions: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PermissionSpec {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) default_value: Option<u32>,
    pub(crate) children: Vec<PermissionChild>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PermissionChild {
    pub(crate) name: String,
    pub(crate) value: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PluginLoadOrder {
    Startup,
    PostWorld,
}
