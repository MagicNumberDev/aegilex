#pragma once

#include "rust/cxx.h"
#include "level/level.h"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

namespace endstone {
class CommandSender;
class Enchantment;
class ItemType;
class Player;
class Server;
} // namespace endstone

namespace aegilex::native {
class WasmPluginLoader;
}

namespace aegilex::native::host {
class CommandSender;
}

namespace aegilex::native::admin {
class BanList;
class PermissionDefinition;
class PermissionDefinitionCollection;
struct PermissionChild;
} // namespace aegilex::native::admin

namespace aegilex::native::player {
class Player;
}

namespace aegilex::native::inventory {
struct ItemMeta;
class ItemStack;
class ItemStackCollection;
class ItemType;
struct ItemMeta;
} // namespace aegilex::native::inventory

namespace aegilex::native::ui {
class BossBar;
class Map;
class Scoreboard;
} // namespace aegilex::native::ui

namespace aegilex::native::server {

struct TranslateResult;

class Server;
class ItemType;
class Plugin;
class PluginCommand;
class PlayerCollection;
class ItemTypeCollection;
class Enchantment;
class EnchantmentCollection;

// OOP/Pimpl facade over endstone::Server. The impl holds only a non-owning
// endstone::Server* (the host owns the server); mirrors endstone/server.h.
class Server {
  public:
    explicit Server(endstone::Server *server, ::aegilex::native::WasmPluginLoader *wasm_loader = nullptr) noexcept;
    ~Server() noexcept = default;

    Server(const Server &) = delete;
    Server &operator=(const Server &) = delete;

    rust::String getName() const;
    rust::String getVersion() const;
    rust::String getMinecraftVersion() const;
    std::int32_t getProtocolVersion() const;
    std::int32_t getMaxPlayers() const;
    std::int32_t getPort() const;
    std::int32_t getPortV6() const;
    bool getOnlineMode() const;
    bool isPrimaryThread() const;
    float getCurrentMillisecondsPerTick() const;
    float getAverageMillisecondsPerTick() const;
    float getCurrentTicksPerSecond() const;
    float getAverageTicksPerSecond() const;
    float getCurrentTickUsage() const;
    float getAverageTickUsage() const;
    std::int64_t getStartTimeMilliseconds() const;
    [[nodiscard]] std::unique_ptr<PlayerCollection> listOnlinePlayers() const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player>
    findPlayerByUuid(rust::Slice<const std::uint8_t> id) const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> findPlayerByName(rust::Str name) const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::host::CommandSender> getCommandSender() const;
    [[nodiscard]] std::uint32_t setMaxPlayers(std::int32_t max_players) const;
    [[nodiscard]] std::uint32_t broadcast(rust::Str message, bool has_permission, rust::Str permission) const;
    [[nodiscard]] std::uint32_t translate(rust::Str key, rust::Vec<rust::String> args, TranslateResult &out) const;
    [[nodiscard]] std::unique_ptr<PluginCommand> getPluginCommand(rust::Str name) const;
    [[nodiscard]] std::unique_ptr<ItemType> getRegistryItemType(rust::Str type_id) const;
    // Returns a shared reference to the cached full sorted registry list; the
    // cache is refilled lazily and cleared at plugin disable.
    [[nodiscard]] std::shared_ptr<ItemTypeCollection> listRegistryItemTypes() const;
    [[nodiscard]] std::unique_ptr<Enchantment> getRegistryEnchantment(rust::Str id) const;
    [[nodiscard]] std::shared_ptr<EnchantmentCollection> listRegistryEnchantments() const;
    // Invalidates the cached registry lists; called when the owning plugin is
    // disabled.
    void clearRegistryCaches() noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::level::Level> getLevel() const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::inventory::ItemType> getItemType(rust::Str type_id) const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::ui::BossBar>
    createBossBar(rust::Str title, std::uint32_t color, std::uint32_t style,
                  rust::Slice<const std::uint32_t> flags) const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::ui::Scoreboard> getScoreboard() const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::ui::Scoreboard> createScoreboard() const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::ui::Map> getMap(std::int64_t id) const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::ui::Map> createMap(rust::Str dimension) const;
    [[nodiscard]] std::unique_ptr<Plugin> getPlugin(rust::Str name) const;
    [[nodiscard]] bool dispatchConsoleCommand(rust::Str command_line) const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::admin::BanList> getPlayerBanList() const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::admin::BanList> getIpBanList() const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::admin::PermissionDefinition>
    getPermissionDefinition(rust::Str name) const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::admin::PermissionDefinition>
    addPermissionDefinition(rust::Str name, bool has_description, rust::Str description, bool has_default,
                            std::uint8_t default_value,
                            const rust::Vec<::aegilex::native::admin::PermissionChild> &children) const;
    [[nodiscard]] bool removePermissionDefinitionByName(rust::Str name) const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::admin::PermissionDefinitionCollection>
    listDefaultPermissionDefinitions(std::uint8_t level) const;
    void recalculatePermissionDefaults(const ::aegilex::native::admin::PermissionDefinition &definition) const;
    [[nodiscard]] bool createItemMetaForType(rust::Str type_id, ::aegilex::native::inventory::ItemMeta &out,
                                             ::aegilex::native::inventory::ItemStackCollection &projectiles) const;
    [[nodiscard]] bool isItemMetaApplicable(rust::Str type_id, const ::aegilex::native::inventory::ItemMeta &meta,
                                            const ::aegilex::native::inventory::ItemStackCollection &projectiles,
                                            bool &out) const;
    [[nodiscard]] bool areItemMetasEqual(bool has_a, const ::aegilex::native::inventory::ItemMeta &a,
                                         const ::aegilex::native::inventory::ItemStackCollection &a_projectiles,
                                         bool has_b, const ::aegilex::native::inventory::ItemMeta &b,
                                         const ::aegilex::native::inventory::ItemStackCollection &b_projectiles,
                                         bool &out) const;
    [[nodiscard]] bool
    convertItemMetaForType(rust::Str type_id, const ::aegilex::native::inventory::ItemMeta &meta,
                           const ::aegilex::native::inventory::ItemStackCollection &projectiles,
                           ::aegilex::native::inventory::ItemMeta &out,
                           ::aegilex::native::inventory::ItemStackCollection &converted_projectiles) const;
    [[nodiscard]] endstone::Server *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl_;
};

// OOP/Pimpl facade over endstone::ItemType (a registry entry, so the impl
// holds a non-owning pointer to a const entry). Mirrors
// endstone/inventory/item_type.h.
class ItemType {
  public:
    explicit ItemType(const endstone::ItemType *item_type) noexcept;
    ~ItemType() noexcept = default;

    ItemType(const ItemType &) = delete;
    ItemType &operator=(const ItemType &) = delete;

    rust::String getId() const;
    rust::String getTranslationKey() const;
    std::int32_t getMaxStackSize() const;
    std::int32_t getMaxDurability() const;
    [[nodiscard]] const endstone::ItemType *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl_;
};

class PlayerCollection {
  public:
    explicit PlayerCollection(std::vector<std::unique_ptr<::aegilex::native::player::Player>> players) noexcept;
    ~PlayerCollection() noexcept = default;

    PlayerCollection(const PlayerCollection &) = delete;
    PlayerCollection &operator=(const PlayerCollection &) = delete;

    [[nodiscard]] std::size_t len() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> takePlayer(std::size_t index) noexcept;

  private:
    std::vector<std::unique_ptr<::aegilex::native::player::Player>> players_;
};

class ItemTypeCollection {
  public:
    explicit ItemTypeCollection(std::vector<const endstone::ItemType *> item_types) noexcept;
    ~ItemTypeCollection() noexcept = default;

    ItemTypeCollection(const ItemTypeCollection &) = delete;
    ItemTypeCollection &operator=(const ItemTypeCollection &) = delete;

    [[nodiscard]] std::size_t len() const noexcept;
    // Non-destructive: each call builds a fresh facade over the stored native
    // entry so shared cached collections stay valid for every reader.
    [[nodiscard]] std::unique_ptr<ItemType> takeItemType(std::size_t index) const noexcept;

  private:
    std::vector<const endstone::ItemType *> item_types_;
};

// OOP/Pimpl facade over endstone::Enchantment (a registry entry, so the impl
// holds a non-owning pointer to a const entry). Mirrors
// endstone/enchantments/enchantment.h.
class Enchantment {
  public:
    explicit Enchantment(const endstone::Enchantment *enchantment) noexcept;
    ~Enchantment() noexcept = default;

    Enchantment(const Enchantment &) = delete;
    Enchantment &operator=(const Enchantment &) = delete;

    rust::String getId() const;
    rust::String getTranslationKey() const;
    std::int32_t getMaxLevel() const;
    std::int32_t getStartLevel() const;
    [[nodiscard]] bool canEnchantItem(const ::aegilex::native::inventory::ItemStack &item) const;
    [[nodiscard]] const endstone::Enchantment *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl_;
};

class EnchantmentCollection {
  public:
    explicit EnchantmentCollection(std::vector<const endstone::Enchantment *> enchantments) noexcept;
    ~EnchantmentCollection() noexcept = default;

    EnchantmentCollection(const EnchantmentCollection &) = delete;
    EnchantmentCollection &operator=(const EnchantmentCollection &) = delete;

    [[nodiscard]] std::size_t len() const noexcept;
    // Non-destructive: each call builds a fresh facade over the stored native
    // entry so shared cached collections stay valid for every reader.
    [[nodiscard]] std::unique_ptr<Enchantment> takeEnchantment(std::size_t index) const noexcept;

  private:
    std::vector<const endstone::Enchantment *> enchantments_;
};

} // namespace aegilex::native::server
