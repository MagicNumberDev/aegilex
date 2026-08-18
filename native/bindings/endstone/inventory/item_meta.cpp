#include "item_meta.h"

#include "item_stack.h"

#include "bindings/endstone/server.h"

#include <aegilex-runtime/src/cxx_host_inventory.rs.h>

#include <endstone/enchantments/enchantment.h>
#include <endstone/inventory/item_factory.h>
#include <endstone/inventory/item_stack.h>
#include <endstone/inventory/item_type.h>
#include <endstone/inventory/meta/book_meta.h>
#include <endstone/inventory/meta/crossbow_meta.h>
#include <endstone/inventory/meta/item_meta.h>
#include <endstone/inventory/meta/map_meta.h>
#include <endstone/inventory/meta/writable_book_meta.h>
#include <endstone/server.h>

#include <algorithm>
#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

namespace aegilex::native::inventory {

namespace {

constexpr std::uint8_t kMetaKindItem = 0;
constexpr std::uint8_t kMetaKindBook = 1;
constexpr std::uint8_t kMetaKindCrossbow = 2;
constexpr std::uint8_t kMetaKindMap = 3;
constexpr std::uint8_t kMetaKindWritableBook = 4;

[[nodiscard]] bool valid_type_id(const std::string_view type_id) noexcept
{
    if (type_id.empty()) {
        return false;
    }
    std::size_t colons = 0;
    for (const auto byte : type_id) {
        const auto value = static_cast<std::uint8_t>(byte);
        const auto valid = (value >= static_cast<std::uint8_t>('a') && value <= static_cast<std::uint8_t>('z')) ||
                           (value >= static_cast<std::uint8_t>('0') && value <= static_cast<std::uint8_t>('9')) ||
                           value == static_cast<std::uint8_t>('_') || value == static_cast<std::uint8_t>(':');
        if (!valid) {
            return false;
        }
        colons += value == static_cast<std::uint8_t>(':') ? 1U : 0U;
    }
    return colons <= 1 && type_id.front() != ':' && type_id.back() != ':';
}

[[nodiscard]] bool meta_empty(const ItemMeta &meta) noexcept
{
    const auto &base = meta.base;
    return !base.has_display_name && !base.has_lore && base.enchants.empty() && !base.unbreakable && !base.has_damage &&
           !base.has_repair_cost && meta.pages.empty() && !meta.has_title && !meta.has_author && !meta.has_generation &&
           !meta.has_map_id && !meta.has_map_view;
}

[[nodiscard]] bool meta_common_equal(const ItemMeta &a, const ItemMeta &b) noexcept
{
    const auto &base_a = a.base;
    const auto &base_b = b.base;
    if (base_a.has_display_name != base_b.has_display_name) {
        return false;
    }
    if (base_a.has_display_name && std::string(base_a.display_name) != std::string(base_b.display_name)) {
        return false;
    }
    if (base_a.has_lore != base_b.has_lore || (base_a.has_lore && base_a.lore.size() != base_b.lore.size())) {
        return false;
    }
    if (base_a.has_lore) {
        for (std::size_t index = 0; index < base_a.lore.size(); ++index) {
            if (std::string(base_a.lore[index]) != std::string(base_b.lore[index])) {
                return false;
            }
        }
    }
    if (base_a.enchants.size() != base_b.enchants.size() || base_a.unbreakable != base_b.unbreakable ||
        base_a.has_damage != base_b.has_damage || base_a.has_repair_cost != base_b.has_repair_cost ||
        (base_a.has_damage && base_a.damage != base_b.damage) ||
        (base_a.has_repair_cost && base_a.repair_cost != base_b.repair_cost)) {
        return false;
    }
    std::vector<std::pair<std::string, std::int32_t>> entries_a;
    std::vector<std::pair<std::string, std::int32_t>> entries_b;
    entries_a.reserve(base_a.enchants.size());
    entries_b.reserve(base_b.enchants.size());
    for (const auto &enchant : base_a.enchants) {
        entries_a.emplace_back(std::string(enchant.type_id), enchant.level);
    }
    for (const auto &enchant : base_b.enchants) {
        entries_b.emplace_back(std::string(enchant.type_id), enchant.level);
    }
    std::sort(entries_a.begin(), entries_a.end());
    std::sort(entries_b.begin(), entries_b.end());
    return entries_a == entries_b;
}

[[nodiscard]] bool meta_equals_value(const ItemMeta &a, const ItemStackCollection &a_projectiles, const ItemMeta &b,
                                     const ItemStackCollection &b_projectiles, bool &out) noexcept
{
    if (a.kind != b.kind || !meta_common_equal(a, b)) {
        out = false;
        return true;
    }
    if (a.kind == kMetaKindBook || a.kind == kMetaKindWritableBook) {
        if (a.pages.size() != b.pages.size()) {
            out = false;
            return true;
        }
        for (std::size_t index = 0; index < a.pages.size(); ++index) {
            if (std::string(a.pages[index]) != std::string(b.pages[index])) {
                out = false;
                return true;
            }
        }
        if (a.kind == kMetaKindBook &&
            (a.has_title != b.has_title || (a.has_title && std::string(a.title) != std::string(b.title)) ||
             a.has_author != b.has_author || (a.has_author && std::string(a.author) != std::string(b.author)) ||
             a.has_generation != b.has_generation || (a.has_generation && a.generation != b.generation))) {
            out = false;
            return true;
        }
    }
    else if (a.kind == kMetaKindCrossbow) {
        if (a_projectiles.len() != b_projectiles.len()) {
            out = false;
            return true;
        }
        for (std::size_t index = 0; index < a_projectiles.len(); ++index) {
            const auto left = a_projectiles.get(index);
            const auto right = b_projectiles.get(index);
            if (left == nullptr || right == nullptr || !left->equals(*right)) {
                out = false;
                return true;
            }
        }
    }
    else if (a.kind == kMetaKindMap && (a.has_map_id != b.has_map_id || (a.has_map_id && a.map_id != b.map_id))) {
        out = false;
        return true;
    }
    out = true;
    return true;
}

} // namespace

bool inventory_item_meta_read(const endstone::ItemMeta &meta, ItemMeta &out) noexcept
{
    try {
        out = ItemMeta{};
        out.kind = static_cast<std::uint8_t>(meta.getType());
        out.base.has_display_name = meta.hasDisplayName();
        if (out.base.has_display_name) {
            out.base.display_name = rust::String(meta.getDisplayName());
        }
        out.base.has_lore = meta.hasLore();
        if (out.base.has_lore) {
            const auto lore = meta.getLore();
            out.base.lore.reserve(lore.size());
            for (const auto &line : lore) {
                out.base.lore.push_back(rust::String(line));
            }
        }
        std::vector<std::pair<std::string, std::int32_t>> enchants;
        for (const auto &[enchant, level] : meta.getEnchants()) {
            if (enchant != nullptr) {
                enchants.emplace_back(std::string(enchant->getId()), level);
            }
        }
        std::sort(enchants.begin(), enchants.end());
        out.base.enchants.reserve(enchants.size());
        for (const auto &[id, level] : enchants) {
            out.base.enchants.push_back(Enchantment{.type_id = rust::String(id), .level = level});
        }
        out.base.unbreakable = meta.isUnbreakable();
        out.base.has_damage = meta.hasDamage();
        out.base.damage = meta.getDamage();
        out.base.has_repair_cost = meta.hasRepairCost();
        out.base.repair_cost = meta.getRepairCost();

        switch (meta.getType()) {
        case endstone::ItemMeta::Type::WritableBook: {
            const auto *book = static_cast<const endstone::WritableBookMeta *>(&meta);
            if (book->hasPages()) {
                for (const auto &page : book->getPages()) {
                    out.pages.push_back(rust::String(page));
                }
            }
            break;
        }
        case endstone::ItemMeta::Type::Book: {
            const auto *book = static_cast<const endstone::BookMeta *>(&meta);
            if (book->hasPages()) {
                for (const auto &page : book->getPages()) {
                    out.pages.push_back(rust::String(page));
                }
            }
            out.has_title = book->hasTitle();
            if (out.has_title) {
                out.title = rust::String(book->getTitle());
            }
            out.has_author = book->hasAuthor();
            if (out.has_author) {
                out.author = rust::String(book->getAuthor());
            }
            out.has_generation = book->hasGeneration();
            if (out.has_generation) {
                out.generation = static_cast<std::uint8_t>(*book->getGeneration());
            }
            break;
        }
        case endstone::ItemMeta::Type::Map: {
            const auto *map_meta = static_cast<const endstone::MapMeta *>(&meta);
            out.has_map_id = map_meta->hasMapId();
            out.map_id = map_meta->getMapId();
            out.has_map_view = map_meta->hasMapView();
            break;
        }
        default:
            break;
        }
        return true;
    }
    catch (...) {
        out = ItemMeta{};
        return false;
    }
}

bool inventory_item_meta_read_projectiles(const endstone::ItemMeta &meta, ItemStackCollection &out) noexcept
{
    try {
        if (meta.getType() != endstone::ItemMeta::Type::CrossBow) {
            return true;
        }
        const auto *crossbow = static_cast<const endstone::CrossbowMeta *>(&meta);
        for (const auto &projectile : crossbow->getChargedProjectiles()) {
            ItemStack projectile_facade(projectile);
            out.push(projectile_facade);
        }
        return true;
    }
    catch (...) {
        return false;
    }
}

bool inventory_item_meta_build(endstone::Server &server, const ItemMeta &meta, const std::string &type_id,
                               const ItemStackCollection &projectiles, std::unique_ptr<endstone::ItemMeta> &out,
                               bool &out_kind_mismatch) noexcept
{
    out.reset();
    out_kind_mismatch = false;
    try {
        auto built = server.getItemFactory().getItemMeta(endstone::ItemTypeId{type_id});
        if (built == nullptr) {
            out_kind_mismatch = true;
            return true;
        }
        if (static_cast<std::uint8_t>(built->getType()) != meta.kind) {
            out_kind_mismatch = true;
        }
        built->setDisplayName(meta.base.has_display_name
                                  ? std::optional<std::string>{std::string(meta.base.display_name)}
                                  : std::nullopt);
        if (meta.base.has_lore) {
            std::vector<std::string> lore;
            lore.reserve(meta.base.lore.size());
            for (const auto &line : meta.base.lore) {
                lore.push_back(std::string(line));
            }
            built->setLore(std::move(lore));
        }
        else {
            built->setLore(std::nullopt);
        }
        for (const auto &enchant : meta.base.enchants) {
            const std::string id(enchant.type_id);
            if (!valid_type_id(id) || enchant.level < 1 || enchant.level > 65535 ||
                server.getRegistry<endstone::Enchantment>().get(endstone::EnchantmentId{id}) == nullptr) {
                return false;
            }
            (void)built->addEnchant(endstone::EnchantmentId{id}, enchant.level, true);
        }
        built->setUnbreakable(meta.base.unbreakable);
        if (meta.base.has_damage) {
            if (meta.base.damage < 0 || meta.base.damage > 65535) {
                return false;
            }
            built->setDamage(meta.base.damage);
        }
        if (meta.base.has_repair_cost) {
            if (meta.base.repair_cost < 0 || meta.base.repair_cost > 65535) {
                return false;
            }
            built->setRepairCost(meta.base.repair_cost);
        }
        if (!out_kind_mismatch) {
            switch (built->getType()) {
            case endstone::ItemMeta::Type::WritableBook: {
                auto *book = static_cast<endstone::WritableBookMeta *>(built.get());
                std::vector<std::string> pages;
                pages.reserve(meta.pages.size());
                for (const auto &page : meta.pages) {
                    pages.push_back(std::string(page));
                }
                book->setPages(std::move(pages));
                break;
            }
            case endstone::ItemMeta::Type::Book: {
                auto *book = static_cast<endstone::BookMeta *>(built.get());
                std::vector<std::string> pages;
                pages.reserve(meta.pages.size());
                for (const auto &page : meta.pages) {
                    pages.push_back(std::string(page));
                }
                book->setPages(std::move(pages));
                if (meta.has_title) {
                    book->setTitle(std::string(meta.title));
                }
                if (meta.has_author) {
                    book->setAuthor(std::string(meta.author));
                }
                if (meta.has_generation) {
                    if (meta.generation > 2) {
                        return false;
                    }
                    book->setGeneration(static_cast<endstone::BookMeta::Generation>(meta.generation));
                }
                break;
            }
            case endstone::ItemMeta::Type::CrossBow: {
                auto *crossbow = static_cast<endstone::CrossbowMeta *>(built.get());
                std::vector<endstone::ItemStack> native_projectiles;
                native_projectiles.reserve(projectiles.len());
                for (std::size_t index = 0; index < projectiles.len(); ++index) {
                    const auto projectile = projectiles.get(index);
                    if (projectile == nullptr || projectile->native() == nullptr) {
                        return false;
                    }
                    native_projectiles.push_back(*projectile->native());
                }
                crossbow->setChargedProjectiles(std::move(native_projectiles));
                break;
            }
            case endstone::ItemMeta::Type::Map: {
                auto *map_meta = static_cast<endstone::MapMeta *>(built.get());
                if (meta.has_map_id) {
                    map_meta->setMapId(meta.map_id);
                }
                break;
            }
            default:
                break;
            }
        }
        out = std::move(built);
        return true;
    }
    catch (...) {
        out.reset();
        return false;
    }
}

namespace detail {

bool create_item_meta_for_type(const ::aegilex::native::server::Server &server, const rust::Str type_id, ItemMeta &out,
                               ItemStackCollection &projectiles) noexcept
{
    try {
        const std::string identifier(type_id);
        if (server.native() == nullptr || !valid_type_id(identifier) ||
            server.native()->getRegistry<endstone::ItemType>().get(endstone::ItemTypeId{identifier}) == nullptr) {
            return false;
        }
        const auto meta = server.native()->getItemFactory().getItemMeta(endstone::ItemTypeId{identifier});
        if (meta == nullptr) {
            out = ItemMeta{};
            out.kind = kMetaKindItem;
            return true;
        }
        return inventory_item_meta_read(*meta, out) && inventory_item_meta_read_projectiles(*meta, projectiles);
    }
    catch (...) {
        return false;
    }
}

bool is_item_meta_applicable(const ::aegilex::native::server::Server &server, const rust::Str type_id,
                             const ItemMeta &meta, const ItemStackCollection &projectiles, bool &out) noexcept
{
    try {
        const std::string identifier(type_id);
        if (server.native() == nullptr || !valid_type_id(identifier) ||
            server.native()->getRegistry<endstone::ItemType>().get(endstone::ItemTypeId{identifier}) == nullptr) {
            return false;
        }
        std::unique_ptr<endstone::ItemMeta> built;
        bool mismatch = false;
        if (!inventory_item_meta_build(*server.native(), meta, identifier, projectiles, built, mismatch)) {
            return false;
        }
        out = built != nullptr && !mismatch &&
              server.native()->getItemFactory().isApplicable(built.get(), endstone::ItemTypeId{identifier});
        return true;
    }
    catch (...) {
        return false;
    }
}

bool are_item_metas_equal(const ::aegilex::native::server::Server &server, const bool has_a, const ItemMeta &a,
                          const ItemStackCollection &a_projectiles, const bool has_b, const ItemMeta &b,
                          const ItemStackCollection &b_projectiles, bool &out) noexcept
{
    try {
        if (server.native() == nullptr) {
            return false;
        }
        if (!has_a && !has_b) {
            out = true;
            return true;
        }
        if (!has_a || !has_b) {
            out = meta_empty(has_a ? a : b);
            return true;
        }
        return meta_equals_value(a, a_projectiles, b, b_projectiles, out);
    }
    catch (...) {
        return false;
    }
}

bool convert_item_meta_for_type(const ::aegilex::native::server::Server &server, const rust::Str type_id,
                                const ItemMeta &meta, const ItemStackCollection &projectiles, ItemMeta &out,
                                ItemStackCollection &converted_projectiles) noexcept
{
    try {
        const std::string identifier(type_id);
        if (server.native() == nullptr || !valid_type_id(identifier) ||
            server.native()->getRegistry<endstone::ItemType>().get(endstone::ItemTypeId{identifier}) == nullptr) {
            return false;
        }
        std::unique_ptr<endstone::ItemMeta> built;
        bool mismatch = false;
        if (!inventory_item_meta_build(*server.native(), meta, identifier, projectiles, built, mismatch)) {
            return false;
        }
        if (built == nullptr) {
            out = ItemMeta{};
            out.kind = kMetaKindItem;
            return true;
        }
        const auto converted =
            server.native()->getItemFactory().asMetaFor(built.get(), endstone::ItemTypeId{identifier});
        if (converted == nullptr) {
            out = ItemMeta{};
            out.kind = kMetaKindItem;
            return true;
        }
        return inventory_item_meta_read(*converted, out) &&
               inventory_item_meta_read_projectiles(*converted, converted_projectiles);
    }
    catch (...) {
        return false;
    }
}

} // namespace detail

} // namespace aegilex::native::inventory
