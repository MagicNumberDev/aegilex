#[allow(clippy::too_many_arguments)]
#[cxx::bridge(namespace = "aegilex::native::server")]
pub(crate) mod ffi {
    struct TranslateResult {
        status: u32,
        value: String,
    }

    unsafe extern "C++" {
        include!("bindings/endstone/server.h");
        include!("bindings/endstone/plugin.h");

        type Server;
        type Plugin;
        type PluginCommand;
        type ItemType;
        type Enchantment;
        type EnchantmentCollection;
        type PlayerCollection;
        type ItemTypeCollection;
        #[namespace = "aegilex::native::inventory"]
        type ItemStack = crate::cxx_host_inventory::ffi::ItemStack;

        // Server identity and configuration — mirrors Endstone's Server getters.
        fn getName(self: &Server) -> String;
        fn getVersion(self: &Server) -> String;
        fn getMinecraftVersion(self: &Server) -> String;
        fn getProtocolVersion(self: &Server) -> i32;
        fn getMaxPlayers(self: &Server) -> i32;
        fn getPort(self: &Server) -> i32;
        fn getPortV6(self: &Server) -> i32;
        fn getOnlineMode(self: &Server) -> bool;
        fn isPrimaryThread(self: &Server) -> bool;

        // Performance — mirrors Endstone's performance getters.
        fn getCurrentMillisecondsPerTick(self: &Server) -> f32;
        fn getAverageMillisecondsPerTick(self: &Server) -> f32;
        fn getCurrentTicksPerSecond(self: &Server) -> f32;
        fn getAverageTicksPerSecond(self: &Server) -> f32;
        fn getCurrentTickUsage(self: &Server) -> f32;
        fn getAverageTickUsage(self: &Server) -> f32;
        fn getStartTimeMilliseconds(self: &Server) -> i64;

        fn listOnlinePlayers(self: &Server) -> UniquePtr<PlayerCollection>;
        fn len(self: &PlayerCollection) -> usize;
        fn takePlayer(self: Pin<&mut PlayerCollection>, index: usize) -> UniquePtr<Player>;
        fn findPlayerByUuid(self: &Server, id: &[u8]) -> UniquePtr<Player>;
        fn findPlayerByName(self: &Server, name: &str) -> UniquePtr<Player>;
        fn getCommandSender(self: &Server) -> UniquePtr<CommandSender>;

        // Messaging.
        fn setMaxPlayers(self: &Server, max_players: i32) -> u32;
        fn broadcast(self: &Server, message: &str, has_permission: bool, permission: &str) -> u32;
        fn translate(self: &Server, key: &str, args: Vec<String>, out: &mut TranslateResult)
        -> u32;

        fn getPluginCommand(self: &Server, name: &str) -> UniquePtr<PluginCommand>;
        fn getName(self: &PluginCommand) -> String;
        fn getDescription(self: &PluginCommand) -> String;
        fn getAliases(self: &PluginCommand) -> Vec<String>;
        fn getUsages(self: &PluginCommand) -> Vec<String>;
        fn getPermissions(self: &PluginCommand) -> Vec<String>;

        fn getRegistryItemType(self: &Server, type_id: &str) -> UniquePtr<ItemType>;
        // Returns a shared reference to the cached full registry list.
        fn listRegistryItemTypes(self: &Server) -> SharedPtr<ItemTypeCollection>;
        fn len(self: &ItemTypeCollection) -> usize;
        fn takeItemType(self: &ItemTypeCollection, index: usize) -> UniquePtr<ItemType>;
        fn getId(self: &ItemType) -> String;
        fn getTranslationKey(self: &ItemType) -> String;
        fn getMaxStackSize(self: &ItemType) -> i32;
        fn getMaxDurability(self: &ItemType) -> i32;

        // Enchantment registry — mirrors endstone/enchantments/enchantment.h.
        fn getRegistryEnchantment(self: &Server, id: &str) -> UniquePtr<Enchantment>;
        fn listRegistryEnchantments(self: &Server) -> SharedPtr<EnchantmentCollection>;
        fn len(self: &EnchantmentCollection) -> usize;
        fn takeEnchantment(self: &EnchantmentCollection, index: usize) -> UniquePtr<Enchantment>;
        fn getId(self: &Enchantment) -> String;
        fn getTranslationKey(self: &Enchantment) -> String;
        fn getMaxLevel(self: &Enchantment) -> i32;
        fn getStartLevel(self: &Enchantment) -> i32;
        fn canEnchantItem(self: &Enchantment, item: &ItemStack) -> bool;

        // Plugin facade — mirrors endstone/plugin/plugin.h.
        fn getPlugin(self: &Server, name: &str) -> UniquePtr<Plugin>;
        fn getName(self: &Plugin) -> String;
        fn getVersion(self: &Plugin) -> String;
        fn getFullName(self: &Plugin) -> String;
        fn getApiVersion(self: &Plugin) -> String;
        fn getDescription(self: &Plugin) -> String;
        fn getLoadOrder(self: &Plugin) -> u8;
        fn listAuthors(self: &Plugin) -> Vec<String>;
        fn listContributors(self: &Plugin) -> Vec<String>;
        fn getWebsite(self: &Plugin) -> String;
        fn getPrefix(self: &Plugin) -> String;
        fn listProvides(self: &Plugin) -> Vec<String>;
        fn listDepend(self: &Plugin) -> Vec<String>;
        fn listSoftDepend(self: &Plugin) -> Vec<String>;
        fn listLoadBefore(self: &Plugin) -> Vec<String>;
        fn getDefaultPermission(self: &Plugin) -> u8;
        fn listCommands(self: &Plugin) -> Vec<String>;
        fn isEnabled(self: &Plugin) -> bool;
        fn getDataFolder(self: &Plugin) -> String;
        fn listLoaderFileFilters(self: &Plugin) -> Vec<String>;
        fn getCommand(self: &Plugin, name: &str) -> UniquePtr<PluginCommand>;
    }

    #[namespace = "aegilex::native::player"]
    unsafe extern "C++" {
        include!("bindings/endstone/actor/player.h");

        type Player = crate::cxx_host_player::ffi::Player;
    }

    #[namespace = "aegilex::native::host"]
    unsafe extern "C++" {
        include!("bindings/endstone/command_sender.h");

        type CommandSender = crate::cxx_host_common::ffi::CommandSender;
    }
}
