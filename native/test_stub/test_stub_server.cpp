// Test-only typed server/plugin bridge stubs. Never linked into the plugin.

#include <aegilex-runtime/src/cxx_host_server.rs.h>
#include <aegilex-runtime/src/cxx_host_admin.rs.h>
#include <aegilex-runtime/src/cxx_host_inventory.rs.h>

#include "bindings/endstone/server.h"
#include "bindings/endstone/plugin.h"
#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/ban/ban_list.h"
#include "bindings/endstone/boss/boss_bar.h"
#include "bindings/endstone/command_sender.h"
#include "bindings/endstone/inventory/item_type.h"
#include "bindings/endstone/map/map.h"
#include "bindings/endstone/permissions/permission_definition.h"
#include "bindings/endstone/scoreboard/scoreboard.h"

#include <string>
#include <utility>
#include <vector>

namespace aegilex::native::server {

class Server::impl {
  public:
    impl() noexcept = default;
};

class ItemType::impl {
  public:
    impl() noexcept = default;
};

class Enchantment::impl {
  public:
    impl() noexcept = default;
};

class Plugin::impl {
  public:
    impl() noexcept = default;
};

class PluginCommand::impl {
  public:
    impl() noexcept = default;
};

PlayerCollection::PlayerCollection(std::vector<std::unique_ptr<::aegilex::native::player::Player>> players) noexcept
    : players_(std::move(players))
{
}

std::size_t PlayerCollection::len() const noexcept
{
    return players_.size();
}

std::unique_ptr<::aegilex::native::player::Player> PlayerCollection::takePlayer(const std::size_t index) noexcept
{
    return index < players_.size() ? std::move(players_[index]) : std::unique_ptr<::aegilex::native::player::Player>();
}

ItemTypeCollection::ItemTypeCollection(std::vector<const endstone::ItemType *> item_types) noexcept
    : item_types_(std::move(item_types))
{
}

std::size_t ItemTypeCollection::len() const noexcept
{
    return item_types_.size();
}

std::unique_ptr<ItemType> ItemTypeCollection::takeItemType(const std::size_t index) const noexcept
{
    return index < item_types_.size() && item_types_[index] != nullptr ? std::make_unique<ItemType>(item_types_[index])
                                                                       : std::unique_ptr<ItemType>();
}

Server::Server(endstone::Server *, ::aegilex::native::WasmPluginLoader *) noexcept
    : impl_(std::make_shared<class Server::impl>())
{
}

rust::String Server::getName() const
{
    return rust::String("Aegilex");
}

rust::String Server::getVersion() const
{
    return rust::String("0.1.0");
}

rust::String Server::getMinecraftVersion() const
{
    return rust::String("1.21.0");
}

std::int32_t Server::getProtocolVersion() const
{
    return 729;
}

std::int32_t Server::getMaxPlayers() const
{
    return 10;
}

std::int32_t Server::getPort() const
{
    return 19132;
}

std::int32_t Server::getPortV6() const
{
    return 19133;
}

bool Server::getOnlineMode() const
{
    return true;
}

bool Server::isPrimaryThread() const
{
    return true;
}

float Server::getCurrentMillisecondsPerTick() const
{
    return 25.0F;
}

float Server::getAverageMillisecondsPerTick() const
{
    return 24.0F;
}

float Server::getCurrentTicksPerSecond() const
{
    return 20.0F;
}

float Server::getAverageTicksPerSecond() const
{
    return 19.9F;
}

float Server::getCurrentTickUsage() const
{
    return 0.5F;
}

float Server::getAverageTickUsage() const
{
    return 0.48F;
}

std::int64_t Server::getStartTimeMilliseconds() const
{
    return 1700000000000;
}

std::unique_ptr<::aegilex::native::level::Level> Server::getLevel() const
{
    return std::make_unique<::aegilex::native::level::Level>(nullptr);
}

std::unique_ptr<::aegilex::native::inventory::ItemType> Server::getItemType(rust::Str) const
{
    return std::make_unique<::aegilex::native::inventory::ItemType>(nullptr);
}

std::unique_ptr<::aegilex::native::ui::BossBar> Server::createBossBar(rust::Str, std::uint32_t, std::uint32_t,
                                                                      rust::Slice<const std::uint32_t>) const
{
    return std::make_unique<::aegilex::native::ui::BossBar>(nullptr);
}

std::unique_ptr<::aegilex::native::ui::Scoreboard> Server::getScoreboard() const
{
    return std::make_unique<::aegilex::native::ui::Scoreboard>(nullptr);
}

std::unique_ptr<::aegilex::native::ui::Scoreboard> Server::createScoreboard() const
{
    return std::make_unique<::aegilex::native::ui::Scoreboard>(nullptr);
}

std::unique_ptr<::aegilex::native::ui::Map> Server::getMap(std::int64_t) const
{
    return std::make_unique<::aegilex::native::ui::Map>(nullptr);
}

std::unique_ptr<::aegilex::native::ui::Map> Server::createMap(rust::Str) const
{
    return std::make_unique<::aegilex::native::ui::Map>(nullptr);
}

std::unique_ptr<Plugin> Server::getPlugin(rust::Str) const
{
    return std::make_unique<Plugin>(nullptr);
}

bool Server::dispatchConsoleCommand(rust::Str) const
{
    return true;
}

std::unique_ptr<::aegilex::native::admin::BanList> Server::getPlayerBanList() const
{
    return std::make_unique<::aegilex::native::admin::BanList>(static_cast<endstone::PlayerBanList *>(nullptr));
}

std::unique_ptr<::aegilex::native::admin::BanList> Server::getIpBanList() const
{
    return std::make_unique<::aegilex::native::admin::BanList>(static_cast<endstone::IpBanList *>(nullptr));
}

std::unique_ptr<::aegilex::native::admin::PermissionDefinition> Server::getPermissionDefinition(rust::Str) const
{
    return std::make_unique<::aegilex::native::admin::PermissionDefinition>(nullptr);
}

std::unique_ptr<::aegilex::native::admin::PermissionDefinition>
Server::addPermissionDefinition(rust::Str, bool, rust::Str, bool, std::uint8_t,
                                const rust::Vec<::aegilex::native::admin::PermissionChild> &) const
{
    return std::make_unique<::aegilex::native::admin::PermissionDefinition>(nullptr);
}

bool Server::removePermissionDefinitionByName(rust::Str) const
{
    return true;
}

std::unique_ptr<::aegilex::native::admin::PermissionDefinitionCollection>
Server::listDefaultPermissionDefinitions(std::uint8_t) const
{
    std::vector<std::unique_ptr<::aegilex::native::admin::PermissionDefinition>> definitions;
    definitions.push_back(std::make_unique<::aegilex::native::admin::PermissionDefinition>(nullptr));
    return std::make_unique<::aegilex::native::admin::PermissionDefinitionCollection>(std::move(definitions));
}

void Server::recalculatePermissionDefaults(const ::aegilex::native::admin::PermissionDefinition &) const
{
}

bool Server::createItemMetaForType(rust::Str, ::aegilex::native::inventory::ItemMeta &out,
                                   ::aegilex::native::inventory::ItemStackCollection &) const
{
    out = ::aegilex::native::inventory::ItemMeta{};
    return true;
}

bool Server::isItemMetaApplicable(rust::Str, const ::aegilex::native::inventory::ItemMeta &,
                                  const ::aegilex::native::inventory::ItemStackCollection &, bool &out) const
{
    out = true;
    return true;
}

bool Server::areItemMetasEqual(bool, const ::aegilex::native::inventory::ItemMeta &,
                               const ::aegilex::native::inventory::ItemStackCollection &, bool,
                               const ::aegilex::native::inventory::ItemMeta &,
                               const ::aegilex::native::inventory::ItemStackCollection &, bool &out) const
{
    out = true;
    return true;
}

bool Server::convertItemMetaForType(rust::Str, const ::aegilex::native::inventory::ItemMeta &,
                                    const ::aegilex::native::inventory::ItemStackCollection &,
                                    ::aegilex::native::inventory::ItemMeta &out,
                                    ::aegilex::native::inventory::ItemStackCollection &) const
{
    out = ::aegilex::native::inventory::ItemMeta{};
    return true;
}

ItemType::ItemType(const endstone::ItemType *) noexcept : impl_(std::make_shared<class ItemType::impl>())
{
}

rust::String ItemType::getId() const
{
    return rust::String("minecraft:apple");
}

rust::String ItemType::getTranslationKey() const
{
    return rust::String("item.apple.name");
}

std::int32_t ItemType::getMaxStackSize() const
{
    return 64;
}

std::int32_t ItemType::getMaxDurability() const
{
    return 0;
}

Enchantment::Enchantment(const endstone::Enchantment *) noexcept : impl_(std::make_shared<class Enchantment::impl>())
{
}

rust::String Enchantment::getId() const
{
    return rust::String("minecraft:sharpness");
}

rust::String Enchantment::getTranslationKey() const
{
    return rust::String("enchantment.sharpness.name");
}

std::int32_t Enchantment::getMaxLevel() const
{
    return 5;
}

std::int32_t Enchantment::getStartLevel() const
{
    return 1;
}

bool Enchantment::canEnchantItem(const ::aegilex::native::inventory::ItemStack &) const
{
    return true;
}

EnchantmentCollection::EnchantmentCollection(std::vector<const endstone::Enchantment *> enchantments) noexcept
    : enchantments_(std::move(enchantments))
{
}

std::size_t EnchantmentCollection::len() const noexcept
{
    return enchantments_.size();
}

std::unique_ptr<Enchantment> EnchantmentCollection::takeEnchantment(const std::size_t index) const noexcept
{
    return index < enchantments_.size() && enchantments_[index] != nullptr
               ? std::make_unique<Enchantment>(enchantments_[index])
               : std::unique_ptr<Enchantment>();
}

std::unique_ptr<Enchantment> Server::getRegistryEnchantment(rust::Str) const
{
    return std::make_unique<Enchantment>(nullptr);
}

std::shared_ptr<EnchantmentCollection> Server::listRegistryEnchantments() const
{
    static const endstone::Enchantment *enchantment = reinterpret_cast<const endstone::Enchantment *>(1);
    static const auto cached =
        std::make_shared<EnchantmentCollection>(std::vector<const endstone::Enchantment *>{enchantment});
    return cached;
}

Plugin::Plugin(endstone::Plugin *) noexcept : impl_(std::make_shared<class Plugin::impl>())
{
}

rust::String Plugin::getName() const
{
    return rust::String("hello");
}

rust::String Plugin::getVersion() const
{
    return rust::String("1.0.0");
}

rust::String Plugin::getFullName() const
{
    return rust::String("hello v1.0.0");
}

rust::String Plugin::getApiVersion() const
{
    return rust::String("0.11");
}

rust::String Plugin::getDescription() const
{
    return rust::String("Hello plugin");
}

std::uint8_t Plugin::getLoadOrder() const
{
    return 1;
}

rust::Vec<rust::String> Plugin::listAuthors() const
{
    rust::Vec<rust::String> authors;
    authors.push_back(rust::String("Aegilex"));
    return authors;
}

rust::Vec<rust::String> Plugin::listContributors() const
{
    rust::Vec<rust::String> contributors;
    return contributors;
}

rust::String Plugin::getWebsite() const
{
    return rust::String("https://aegilex.dev");
}

rust::String Plugin::getPrefix() const
{
    return rust::String("Hello");
}

rust::Vec<rust::String> Plugin::listProvides() const
{
    rust::Vec<rust::String> provides;
    return provides;
}

rust::Vec<rust::String> Plugin::listDepend() const
{
    rust::Vec<rust::String> depend;
    return depend;
}

rust::Vec<rust::String> Plugin::listSoftDepend() const
{
    rust::Vec<rust::String> soft_depend;
    return soft_depend;
}

rust::Vec<rust::String> Plugin::listLoadBefore() const
{
    rust::Vec<rust::String> load_before;
    return load_before;
}

std::uint8_t Plugin::getDefaultPermission() const
{
    return 2;
}

rust::Vec<rust::String> Plugin::listCommands() const
{
    rust::Vec<rust::String> commands;
    commands.push_back(rust::String("greet"));
    return commands;
}

bool Plugin::isEnabled() const
{
    return true;
}

rust::String Plugin::getDataFolder() const
{
    return rust::String("plugins/hello");
}

rust::Vec<rust::String> Plugin::listLoaderFileFilters() const
{
    rust::Vec<rust::String> filters;
    filters.push_back(rust::String("*.wasm"));
    return filters;
}

std::unique_ptr<PluginCommand> Plugin::getCommand(rust::Str) const
{
    return std::unique_ptr<PluginCommand>(new PluginCommand(nullptr));
}

PluginCommand::PluginCommand(endstone::PluginCommand *) noexcept : impl_(std::make_shared<class PluginCommand::impl>())
{
}

rust::String PluginCommand::getName() const
{
    return rust::String("greet");
}

rust::String PluginCommand::getDescription() const
{
    return rust::String("Greets the player");
}

rust::Vec<rust::String> PluginCommand::getAliases() const
{
    rust::Vec<rust::String> aliases;
    aliases.push_back(rust::String("hello"));
    return aliases;
}

rust::Vec<rust::String> PluginCommand::getUsages() const
{
    rust::Vec<rust::String> usages;
    usages.push_back(rust::String("/greet <name>"));
    return usages;
}

rust::Vec<rust::String> PluginCommand::getPermissions() const
{
    rust::Vec<rust::String> permissions;
    permissions.push_back(rust::String("hello.greet"));
    return permissions;
}

std::unique_ptr<PlayerCollection> Server::listOnlinePlayers() const
{
    std::vector<std::unique_ptr<::aegilex::native::player::Player>> players;
    players.push_back(std::make_unique<::aegilex::native::player::Player>(nullptr));
    return std::make_unique<PlayerCollection>(std::move(players));
}

std::unique_ptr<::aegilex::native::player::Player> Server::findPlayerByUuid(rust::Slice<const std::uint8_t>) const
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

std::unique_ptr<::aegilex::native::player::Player> Server::findPlayerByName(rust::Str) const
{
    return std::make_unique<::aegilex::native::player::Player>(nullptr);
}

std::unique_ptr<::aegilex::native::host::CommandSender> Server::getCommandSender() const
{
    return std::make_unique<::aegilex::native::host::CommandSender>(nullptr);
}

std::uint32_t Server::setMaxPlayers(std::int32_t) const
{
    return 0;
}

std::uint32_t Server::broadcast(rust::Str, bool, rust::Str) const
{
    return 0;
}

std::uint32_t Server::translate(rust::Str key, rust::Vec<rust::String>, TranslateResult &out) const
{
    out.value = rust::String(std::string(key.data(), key.size()));
    return 0;
}

std::unique_ptr<PluginCommand> Server::getPluginCommand(rust::Str) const
{
    return std::make_unique<PluginCommand>(nullptr);
}

std::unique_ptr<ItemType> Server::getRegistryItemType(rust::Str) const
{
    return std::make_unique<ItemType>(nullptr);
}

std::shared_ptr<ItemTypeCollection> Server::listRegistryItemTypes() const
{
    static const auto cached = std::make_shared<ItemTypeCollection>(std::vector<const endstone::ItemType *>{});
    return cached;
}

void Server::clearRegistryCaches() noexcept
{
}

} // namespace aegilex::native::server
