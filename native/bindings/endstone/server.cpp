#include "server.h"
#include "../../aegilex_types.h"
#include "plugin.h"

#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/ban/ban_list.h"
#include "bindings/endstone/boss/boss_bar.h"
#include "bindings/endstone/command_sender.h"
#include "bindings/endstone/inventory/item_stack.h"
#include "bindings/endstone/inventory/item_meta.h"
#include "bindings/endstone/inventory/item_type.h"
#include "bindings/endstone/map/map.h"
#include "bindings/endstone/permissions/permission_definition.h"
#include "bindings/endstone/scoreboard/scoreboard.h"
#include "../../wasm_plugin_loader.h"

#include <aegilex-runtime/src/cxx_host_admin.rs.h>
#include <aegilex-runtime/src/cxx_host_server.rs.h>

#include <endstone/command/console_command_sender.h>
#include <endstone/command/plugin_command.h>
#include <endstone/enchantments/enchantment.h>
#include <endstone/inventory/item_type.h>
#include <endstone/lang/language.h>
#include <endstone/message.h>
#include <endstone/player.h>
#include <endstone/permissions/permission.h>
#include <endstone/permissions/permission_default.h>
#include <endstone/plugin/plugin_manager.h>
#include <endstone/server.h>

#include <algorithm>
#include <chrono>
#include <cstring>
#include <string>
#include <string_view>
#include <unordered_map>
#include <vector>

namespace aegilex::native::server {

class Server::impl {
  public:
    explicit impl(endstone::Server *server, ::aegilex::native::WasmPluginLoader *wasm_loader) noexcept
        : server(server), wasm_loader(wasm_loader)
    {
    }

    endstone::Server *server;
    ::aegilex::native::WasmPluginLoader *wasm_loader;

    // Registry caches hold sorted, copied rows so repeated guest listing does
    // not re-walk the Endstone registries. The shared collections are
    // immutable after build (non-destructive take) and are cleared when the
    // owning plugin is disabled.
    std::shared_ptr<ItemTypeCollection> item_type_cache;
    std::shared_ptr<EnchantmentCollection> enchantment_cache;
};

class ItemType::impl {
  public:
    explicit impl(const endstone::ItemType *item_type) noexcept : item_type(item_type)
    {
    }

    const endstone::ItemType *item_type;
};

class Enchantment::impl {
  public:
    explicit impl(const endstone::Enchantment *enchantment) noexcept : enchantment(enchantment)
    {
    }

    const endstone::Enchantment *enchantment;
};

namespace {

[[nodiscard]] bool valid_permission_name(const std::string_view permission) noexcept
{
    if (permission.empty()) {
        return false;
    }
    for (std::size_t index = 0; index < permission.size(); ++index) {
        const auto value = static_cast<std::uint8_t>(permission.data()[index]);
        const auto valid = (value >= static_cast<std::uint8_t>('a') && value <= static_cast<std::uint8_t>('z')) ||
                           (value >= static_cast<std::uint8_t>('0') && value <= static_cast<std::uint8_t>('9')) ||
                           value == static_cast<std::uint8_t>('.') || value == static_cast<std::uint8_t>('_') ||
                           value == static_cast<std::uint8_t>('-');
        if (!valid) {
            return false;
        }
    }
    return true;
}

[[nodiscard]] bool valid_permission(const rust::Str permission) noexcept
{
    return valid_permission_name(std::string_view(permission.data(), permission.size()));
}

[[nodiscard]] bool permission_default(const std::uint8_t raw, endstone::PermissionDefault &out) noexcept
{
    switch (raw) {
    case aegilex::kPermissionDefaultTrue:
        out = endstone::PermissionDefault::True;
        return true;
    case aegilex::kPermissionDefaultFalse:
        out = endstone::PermissionDefault::False;
        return true;
    case aegilex::kPermissionDefaultOperator:
        out = endstone::PermissionDefault::Operator;
        return true;
    case aegilex::kPermissionDefaultNotOperator:
        out = endstone::PermissionDefault::NotOperator;
        return true;
    case aegilex::kPermissionDefaultConsole:
        out = endstone::PermissionDefault::Console;
        return true;
    default:
        return false;
    }
}

[[nodiscard]] bool valid_item_type_identifier(const rust::Str identifier) noexcept
{
    if (identifier.empty()) {
        return false;
    }
    std::size_t colons = 0;
    for (std::size_t index = 0; index < identifier.size(); ++index) {
        const auto value = static_cast<std::uint8_t>(identifier.data()[index]);
        const auto valid = (value >= static_cast<std::uint8_t>('a') && value <= static_cast<std::uint8_t>('z')) ||
                           (value >= static_cast<std::uint8_t>('0') && value <= static_cast<std::uint8_t>('9')) ||
                           value == static_cast<std::uint8_t>('_') || value == static_cast<std::uint8_t>(':');
        if (!valid) {
            return false;
        }
        colons += value == static_cast<std::uint8_t>(':') ? 1U : 0U;
    }
    if (colons > 1 || identifier.data()[0] == ':' || identifier.data()[identifier.size() - 1] == ':') {
        return false;
    }
    return true;
}

} // namespace

Server::Server(endstone::Server *server, ::aegilex::native::WasmPluginLoader *wasm_loader) noexcept
    : impl_(std::make_shared<class Server::impl>(server, wasm_loader))
{
}

endstone::Server *Server::native() const noexcept
{
    return impl_ == nullptr ? nullptr : impl_->server;
}

std::unique_ptr<::aegilex::native::level::Level> Server::getLevel() const
{
    try {
        auto *level = impl_ == nullptr || impl_->server == nullptr ? nullptr : impl_->server->getLevel();
        return level == nullptr ? std::unique_ptr<::aegilex::native::level::Level>()
                                : std::make_unique<::aegilex::native::level::Level>(level, impl_->server);
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::level::Level>();
    }
}

std::unique_ptr<::aegilex::native::inventory::ItemType> Server::getItemType(const rust::Str type_id) const
{
    try {
        if (impl_ == nullptr || impl_->server == nullptr || !valid_item_type_identifier(type_id)) {
            return std::unique_ptr<::aegilex::native::inventory::ItemType>();
        }
        const auto *item_type = impl_->server->getRegistry<endstone::ItemType>().get(
            endstone::ItemTypeId{std::string(type_id.data(), type_id.size())});
        return item_type == nullptr ? std::unique_ptr<::aegilex::native::inventory::ItemType>()
                                    : std::make_unique<::aegilex::native::inventory::ItemType>(item_type);
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::inventory::ItemType>();
    }
}

std::unique_ptr<::aegilex::native::ui::BossBar>
Server::createBossBar(const rust::Str title, const std::uint32_t color, const std::uint32_t style,
                      const rust::Slice<const std::uint32_t> flags) const
{
    return ::aegilex::native::ui::BossBar::create(*this, title, color, style, flags);
}

std::unique_ptr<::aegilex::native::ui::Scoreboard> Server::getScoreboard() const
{
    try {
        auto *scoreboard = native() == nullptr ? nullptr : native()->getScoreboard();
        return scoreboard == nullptr ? std::unique_ptr<::aegilex::native::ui::Scoreboard>()
                                     : std::make_unique<::aegilex::native::ui::Scoreboard>(scoreboard);
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::ui::Scoreboard>();
    }
}

std::unique_ptr<::aegilex::native::ui::Scoreboard> Server::createScoreboard() const
{
    try {
        if (native() == nullptr) {
            return std::unique_ptr<::aegilex::native::ui::Scoreboard>();
        }
        auto scoreboard = native()->createScoreboard();
        return scoreboard == nullptr ? std::unique_ptr<::aegilex::native::ui::Scoreboard>()
                                     : std::make_unique<::aegilex::native::ui::Scoreboard>(std::move(scoreboard));
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::ui::Scoreboard>();
    }
}

std::unique_ptr<::aegilex::native::ui::Map> Server::getMap(const std::int64_t id) const
{
    try {
        auto *map = native() == nullptr ? nullptr : native()->getMap(id);
        return map == nullptr ? std::unique_ptr<::aegilex::native::ui::Map>()
                              : std::make_unique<::aegilex::native::ui::Map>(map, native());
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::ui::Map>();
    }
}

std::unique_ptr<::aegilex::native::ui::Map> Server::createMap(const rust::Str dimension) const
{
    try {
        if (native() == nullptr) {
            return std::unique_ptr<::aegilex::native::ui::Map>();
        }
        const std::string_view dimension_value(dimension.data(), dimension.size());
        if (dimension_value.empty()) {
            return std::unique_ptr<::aegilex::native::ui::Map>();
        }
        auto *level = native()->getLevel();
        auto *target = level == nullptr ? nullptr : level->getDimension(std::string(dimension_value));
        if (target == nullptr) {
            return std::unique_ptr<::aegilex::native::ui::Map>();
        }
        const auto &map = native()->createMap(*target);
        return std::make_unique<::aegilex::native::ui::Map>(const_cast<endstone::MapView *>(&map), native());
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::ui::Map>();
    }
}

std::unique_ptr<Plugin> Server::getPlugin(const rust::Str name) const
{
    try {
        if (impl_ == nullptr || impl_->wasm_loader == nullptr) {
            return std::unique_ptr<Plugin>();
        }
        auto *plugin = impl_->wasm_loader->find_plugin(std::string_view(name.data(), name.size()));
        return plugin == nullptr ? std::unique_ptr<Plugin>() : std::make_unique<Plugin>(plugin);
    }
    catch (...) {
        return std::unique_ptr<Plugin>();
    }
}

bool Server::dispatchConsoleCommand(const rust::Str command_line) const
{
    try {
        return impl_ != nullptr && impl_->server != nullptr &&
               impl_->server->dispatchCommand(impl_->server->getCommandSender(),
                                              std::string(command_line.data(), command_line.size()));
    }
    catch (...) {
        return false;
    }
}

rust::String Server::getName() const
{
    try {
        return rust::String(impl_->server->getName());
    }
    catch (...) {
        return rust::String();
    }
}

rust::String Server::getVersion() const
{
    try {
        return rust::String(impl_->server->getVersion());
    }
    catch (...) {
        return rust::String();
    }
}

rust::String Server::getMinecraftVersion() const
{
    try {
        return rust::String(impl_->server->getMinecraftVersion());
    }
    catch (...) {
        return rust::String();
    }
}

std::int32_t Server::getProtocolVersion() const
{
    try {
        return impl_->server->getProtocolVersion();
    }
    catch (...) {
        return 0;
    }
}

std::int32_t Server::getMaxPlayers() const
{
    try {
        return impl_->server->getMaxPlayers();
    }
    catch (...) {
        return 0;
    }
}

std::int32_t Server::getPort() const
{
    try {
        return impl_->server->getPort();
    }
    catch (...) {
        return 0;
    }
}

std::int32_t Server::getPortV6() const
{
    try {
        return impl_->server->getPortV6();
    }
    catch (...) {
        return 0;
    }
}

bool Server::getOnlineMode() const
{
    try {
        return impl_->server->getOnlineMode();
    }
    catch (...) {
        return false;
    }
}

bool Server::isPrimaryThread() const
{
    try {
        return impl_->server->isPrimaryThread();
    }
    catch (...) {
        return false;
    }
}

float Server::getCurrentMillisecondsPerTick() const
{
    try {
        return impl_->server->getCurrentMillisecondsPerTick();
    }
    catch (...) {
        return 0.0F;
    }
}

float Server::getAverageMillisecondsPerTick() const
{
    try {
        return impl_->server->getAverageMillisecondsPerTick();
    }
    catch (...) {
        return 0.0F;
    }
}

float Server::getCurrentTicksPerSecond() const
{
    try {
        return impl_->server->getCurrentTicksPerSecond();
    }
    catch (...) {
        return 0.0F;
    }
}

float Server::getAverageTicksPerSecond() const
{
    try {
        return impl_->server->getAverageTicksPerSecond();
    }
    catch (...) {
        return 0.0F;
    }
}

float Server::getCurrentTickUsage() const
{
    try {
        return impl_->server->getCurrentTickUsage();
    }
    catch (...) {
        return 0.0F;
    }
}

float Server::getAverageTickUsage() const
{
    try {
        return impl_->server->getAverageTickUsage();
    }
    catch (...) {
        return 0.0F;
    }
}

std::int64_t Server::getStartTimeMilliseconds() const
{
    try {
        const auto start = impl_->server->getStartTime();
        return static_cast<std::int64_t>(
            std::chrono::duration_cast<std::chrono::milliseconds>(start.time_since_epoch()).count());
    }
    catch (...) {
        return 0;
    }
}

std::unique_ptr<::aegilex::native::admin::BanList> Server::getPlayerBanList() const
{
    try {
        return native() == nullptr ? std::unique_ptr<::aegilex::native::admin::BanList>()
                                   : std::make_unique<::aegilex::native::admin::BanList>(&native()->getBanList());
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::admin::BanList>();
    }
}

std::unique_ptr<::aegilex::native::admin::BanList> Server::getIpBanList() const
{
    try {
        return native() == nullptr ? std::unique_ptr<::aegilex::native::admin::BanList>()
                                   : std::make_unique<::aegilex::native::admin::BanList>(&native()->getIpBanList());
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::admin::BanList>();
    }
}

std::unique_ptr<::aegilex::native::admin::PermissionDefinition>
Server::getPermissionDefinition(const rust::Str name) const
{
    try {
        if (native() == nullptr || !valid_permission(name)) {
            return {};
        }
        auto *definition = native()->getPluginManager().getPermission(std::string(name));
        return definition == nullptr ? std::unique_ptr<::aegilex::native::admin::PermissionDefinition>()
                                     : std::make_unique<::aegilex::native::admin::PermissionDefinition>(definition);
    }
    catch (...) {
        return {};
    }
}

std::unique_ptr<::aegilex::native::admin::PermissionDefinition>
Server::addPermissionDefinition(const rust::Str name, const bool has_description, const rust::Str description,
                                const bool has_default, const std::uint8_t default_value,
                                const rust::Vec<::aegilex::native::admin::PermissionChild> &children) const
{
    try {
        if (native() == nullptr || !valid_permission(name)) {
            return {};
        }
        endstone::PermissionDefault default_value_native = endstone::PermissionDefault::Operator;
        if (has_default && !permission_default(default_value, default_value_native)) {
            return {};
        }
        auto &plugin_manager = native()->getPluginManager();
        if (plugin_manager.getPermission(std::string(name)) != nullptr) {
            return {};
        }
        std::unordered_map<std::string, bool> children_map;
        for (const auto &child : children) {
            if (!valid_permission_name(std::string_view(child.name.data(), child.name.size()))) {
                return {};
            }
            children_map.emplace(std::string(child.name), child.value);
        }
        auto &definition = plugin_manager.addPermission(std::make_unique<endstone::Permission>(
            std::string(name), has_description ? std::string(description) : std::string(), default_value_native,
            std::move(children_map)));
        return std::make_unique<::aegilex::native::admin::PermissionDefinition>(&definition);
    }
    catch (...) {
        return {};
    }
}

bool Server::removePermissionDefinitionByName(const rust::Str name) const
{
    try {
        if (native() == nullptr || !valid_permission(name)) {
            return false;
        }
        auto &plugin_manager = native()->getPluginManager();
        if (plugin_manager.getPermission(std::string(name)) == nullptr) {
            return false;
        }
        plugin_manager.removePermission(std::string(name));
        return true;
    }
    catch (...) {
        return false;
    }
}

std::unique_ptr<::aegilex::native::admin::PermissionDefinitionCollection>
Server::listDefaultPermissionDefinitions(const std::uint8_t level) const
{
    try {
        if (native() == nullptr) {
            return {};
        }
        endstone::PermissionLevel level_native = endstone::PermissionLevel::Default;
        switch (level) {
        case aegilex::kPermissionDefault:
            break;
        case aegilex::kPermissionOperator:
            level_native = endstone::PermissionLevel::Operator;
            break;
        case aegilex::kPermissionConsole:
            level_native = endstone::PermissionLevel::Console;
            break;
        default:
            return {};
        }
        std::vector<std::unique_ptr<::aegilex::native::admin::PermissionDefinition>> definitions;
        for (auto *definition : native()->getPluginManager().getDefaultPermissions(level_native)) {
            if (definition != nullptr) {
                definitions.push_back(std::make_unique<::aegilex::native::admin::PermissionDefinition>(definition));
            }
        }
        return std::make_unique<::aegilex::native::admin::PermissionDefinitionCollection>(std::move(definitions));
    }
    catch (...) {
        return {};
    }
}

void Server::recalculatePermissionDefaults(const ::aegilex::native::admin::PermissionDefinition &definition) const
{
    try {
        if (native() != nullptr && definition.native() != nullptr) {
            native()->getPluginManager().recalculatePermissionDefaults(*definition.native());
        }
    }
    catch (...) {
    }
}

bool Server::createItemMetaForType(const rust::Str type_id, ::aegilex::native::inventory::ItemMeta &out,
                                   ::aegilex::native::inventory::ItemStackCollection &projectiles) const
{
    return ::aegilex::native::inventory::detail::create_item_meta_for_type(*this, type_id, out, projectiles);
}

bool Server::isItemMetaApplicable(const rust::Str type_id, const ::aegilex::native::inventory::ItemMeta &meta,
                                  const ::aegilex::native::inventory::ItemStackCollection &projectiles, bool &out) const
{
    return ::aegilex::native::inventory::detail::is_item_meta_applicable(*this, type_id, meta, projectiles, out);
}

bool Server::areItemMetasEqual(const bool has_a, const ::aegilex::native::inventory::ItemMeta &a,
                               const ::aegilex::native::inventory::ItemStackCollection &a_projectiles, const bool has_b,
                               const ::aegilex::native::inventory::ItemMeta &b,
                               const ::aegilex::native::inventory::ItemStackCollection &b_projectiles, bool &out) const
{
    return ::aegilex::native::inventory::detail::are_item_metas_equal(*this, has_a, a, a_projectiles, has_b, b,
                                                                      b_projectiles, out);
}

bool Server::convertItemMetaForType(const rust::Str type_id, const ::aegilex::native::inventory::ItemMeta &meta,
                                    const ::aegilex::native::inventory::ItemStackCollection &projectiles,
                                    ::aegilex::native::inventory::ItemMeta &out,
                                    ::aegilex::native::inventory::ItemStackCollection &converted_projectiles) const
{
    return ::aegilex::native::inventory::detail::convert_item_meta_for_type(*this, type_id, meta, projectiles, out,
                                                                            converted_projectiles);
}

ItemType::ItemType(const endstone::ItemType *item_type) noexcept
    : impl_(std::make_shared<class ItemType::impl>(item_type))
{
}

const endstone::ItemType *ItemType::native() const noexcept
{
    return impl_ == nullptr ? nullptr : impl_->item_type;
}

rust::String ItemType::getId() const
{
    try {
        return rust::String(std::string(impl_->item_type->getId()));
    }
    catch (...) {
        return rust::String();
    }
}

rust::String ItemType::getTranslationKey() const
{
    try {
        return rust::String(impl_->item_type->getTranslationKey());
    }
    catch (...) {
        return rust::String();
    }
}

std::int32_t ItemType::getMaxStackSize() const
{
    try {
        return impl_->item_type->getMaxStackSize();
    }
    catch (...) {
        return 0;
    }
}

std::int32_t ItemType::getMaxDurability() const
{
    try {
        return impl_->item_type->getMaxDurability();
    }
    catch (...) {
        return 0;
    }
}

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

std::unique_ptr<PlayerCollection> Server::listOnlinePlayers() const
{
    try {
        if (native() == nullptr) {
            return std::unique_ptr<PlayerCollection>();
        }
        std::vector<std::unique_ptr<::aegilex::native::player::Player>> players;
        for (auto *player : native()->getOnlinePlayers()) {
            if (player == nullptr) {
                continue;
            }
            players.push_back(std::make_unique<::aegilex::native::player::Player>(player));
        }
        return std::make_unique<PlayerCollection>(std::move(players));
    }
    catch (...) {
        return std::unique_ptr<PlayerCollection>();
    }
}

std::unique_ptr<::aegilex::native::player::Player>
Server::findPlayerByUuid(const rust::Slice<const std::uint8_t> id) const
{
    try {
        if (native() == nullptr || id.size() != endstone::UUID::size()) {
            return std::unique_ptr<::aegilex::native::player::Player>();
        }
        endstone::UUID uuid;
        std::memcpy(uuid.data, id.data(), uuid.size());
        auto *player = native()->getPlayer(uuid);
        return player == nullptr ? std::unique_ptr<::aegilex::native::player::Player>()
                                 : std::make_unique<::aegilex::native::player::Player>(player);
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::player::Player>();
    }
}

std::unique_ptr<::aegilex::native::player::Player> Server::findPlayerByName(const rust::Str name) const
{
    try {
        if (native() == nullptr) {
            return std::unique_ptr<::aegilex::native::player::Player>();
        }
        auto *player = native()->getPlayer(std::string(name.data(), name.size()));
        return player == nullptr ? std::unique_ptr<::aegilex::native::player::Player>()
                                 : std::make_unique<::aegilex::native::player::Player>(player);
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::player::Player>();
    }
}

std::unique_ptr<::aegilex::native::host::CommandSender> Server::getCommandSender() const
{
    try {
        if (native() == nullptr) {
            return std::unique_ptr<::aegilex::native::host::CommandSender>();
        }
        return std::make_unique<::aegilex::native::host::CommandSender>(
            &static_cast<endstone::CommandSender &>(native()->getCommandSender()), native());
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::host::CommandSender>();
    }
}

std::uint32_t Server::setMaxPlayers(const std::int32_t max_players) const
{
    try {
        if (native() == nullptr) {
            return aegilex::kInvalidArgument;
        }
        if (max_players < 0) {
            return aegilex::kInvalidArgument;
        }
        native()->setMaxPlayers(max_players);
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

std::uint32_t Server::broadcast(const rust::Str message, const bool has_permission, const rust::Str permission) const
{
    try {
        if (native() == nullptr) {
            return aegilex::kInvalidArgument;
        }
        if (has_permission && !valid_permission(permission)) {
            return aegilex::kInvalidArgument;
        }
        const std::string text(message.data(), message.size());
        if (!has_permission) {
            native()->broadcastMessage(endstone::Message{text});
        }
        else {
            const std::string permission_text(permission.data(), permission.size());
            native()->broadcast(endstone::Message{text}, permission_text);
        }
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

std::uint32_t Server::translate(const rust::Str key, rust::Vec<rust::String> args, TranslateResult &out) const
{
    out.value = rust::String();
    try {
        if (native() == nullptr) {
            return aegilex::kInvalidArgument;
        }
        std::vector<std::string> params;
        params.reserve(args.size());
        for (const auto &arg : args) {
            params.emplace_back(arg.data(), arg.size());
        }
        out.value = rust::String(native()->getLanguage().translate(std::string(key.data(), key.size()), params));
        return aegilex::kOk;
    }
    catch (...) {
        out.value = rust::String();
        return aegilex::kHostError;
    }
}

std::unique_ptr<PluginCommand> Server::getPluginCommand(const rust::Str name) const
{
    try {
        if (native() == nullptr) {
            return std::unique_ptr<PluginCommand>();
        }
        auto *command = native()->getPluginCommand(std::string(name.data(), name.size()));
        return command == nullptr ? std::unique_ptr<PluginCommand>() : std::make_unique<PluginCommand>(command);
    }
    catch (...) {
        return std::unique_ptr<PluginCommand>();
    }
}

std::unique_ptr<ItemType> Server::getRegistryItemType(const rust::Str type_id) const
{
    try {
        if (native() == nullptr || !valid_item_type_identifier(type_id)) {
            return std::unique_ptr<ItemType>();
        }
        const auto *item_type = native()->getRegistry<endstone::ItemType>().get(
            endstone::ItemTypeId{std::string(type_id.data(), type_id.size())});
        return item_type == nullptr ? std::unique_ptr<ItemType>() : std::make_unique<ItemType>(item_type);
    }
    catch (...) {
        return std::unique_ptr<ItemType>();
    }
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

std::shared_ptr<ItemTypeCollection> Server::listRegistryItemTypes() const
{
    try {
        if (native() == nullptr || impl_ == nullptr) {
            return std::shared_ptr<ItemTypeCollection>();
        }
        if (impl_->item_type_cache == nullptr) {
            std::vector<const endstone::ItemType *> entries;
            native()->getRegistry<endstone::ItemType>().forEach([&entries](const endstone::ItemType &type) {
                entries.push_back(&type);
                return true;
            });
            std::sort(entries.begin(), entries.end(),
                      [](const endstone::ItemType *left, const endstone::ItemType *right) {
                          return std::string(left->getId()) < std::string(right->getId());
                      });
            impl_->item_type_cache = std::make_shared<ItemTypeCollection>(std::move(entries));
        }
        return impl_->item_type_cache;
    }
    catch (...) {
        return std::shared_ptr<ItemTypeCollection>();
    }
}

std::unique_ptr<Enchantment> Server::getRegistryEnchantment(const rust::Str id) const
{
    try {
        if (native() == nullptr || !valid_item_type_identifier(id)) {
            return std::unique_ptr<Enchantment>();
        }
        const auto *enchantment = native()->getRegistry<endstone::Enchantment>().get(
            endstone::EnchantmentId{std::string(id.data(), id.size())});
        return enchantment == nullptr ? std::unique_ptr<Enchantment>() : std::make_unique<Enchantment>(enchantment);
    }
    catch (...) {
        return std::unique_ptr<Enchantment>();
    }
}

std::shared_ptr<EnchantmentCollection> Server::listRegistryEnchantments() const
{
    try {
        if (native() == nullptr || impl_ == nullptr) {
            return std::shared_ptr<EnchantmentCollection>();
        }
        if (impl_->enchantment_cache == nullptr) {
            std::vector<const endstone::Enchantment *> entries;
            native()->getRegistry<endstone::Enchantment>().forEach(
                [&entries](const endstone::Enchantment &enchantment) {
                    entries.push_back(&enchantment);
                    return true;
                });
            std::sort(entries.begin(), entries.end(),
                      [](const endstone::Enchantment *left, const endstone::Enchantment *right) {
                          return std::string(left->getId()) < std::string(right->getId());
                      });
            impl_->enchantment_cache = std::make_shared<EnchantmentCollection>(std::move(entries));
        }
        return impl_->enchantment_cache;
    }
    catch (...) {
        return std::shared_ptr<EnchantmentCollection>();
    }
}

void Server::clearRegistryCaches() noexcept
{
    if (impl_ != nullptr) {
        impl_->item_type_cache.reset();
        impl_->enchantment_cache.reset();
    }
}

Enchantment::Enchantment(const endstone::Enchantment *enchantment) noexcept
    : impl_(std::make_shared<class Enchantment::impl>(enchantment))
{
}

const endstone::Enchantment *Enchantment::native() const noexcept
{
    return impl_ == nullptr ? nullptr : impl_->enchantment;
}

rust::String Enchantment::getId() const
{
    try {
        return native() == nullptr ? rust::String() : rust::String(std::string(native()->getId()));
    }
    catch (...) {
        return rust::String();
    }
}

rust::String Enchantment::getTranslationKey() const
{
    try {
        return native() == nullptr ? rust::String() : rust::String(native()->getTranslationKey());
    }
    catch (...) {
        return rust::String();
    }
}

std::int32_t Enchantment::getMaxLevel() const
{
    try {
        return native() == nullptr ? 0 : native()->getMaxLevel();
    }
    catch (...) {
        return 0;
    }
}

std::int32_t Enchantment::getStartLevel() const
{
    try {
        return native() == nullptr ? 0 : native()->getStartLevel();
    }
    catch (...) {
        return 0;
    }
}

bool Enchantment::canEnchantItem(const ::aegilex::native::inventory::ItemStack &item) const
{
    try {
        return native() != nullptr && item.native() != nullptr && native()->canEnchantItem(*item.native());
    }
    catch (...) {
        return false;
    }
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

} // namespace aegilex::native::server
