#pragma once

#include "rust/cxx.h"

#include <cstdint>
#include <memory>
#include <string>

namespace endstone {
class ItemMeta;
class Server;
} // namespace endstone

namespace aegilex::native::server {
class Server;
}

namespace aegilex::native::inventory {

struct ItemMeta;
class ItemStack;
class ItemStackCollection;

// Internal meta conversion helpers shared by the item stack facade and the
// item meta bridge functions. ItemMeta is pure data (see the cxx bridge), so
// these helpers convert to and from endstone's ItemMeta object graph.
[[nodiscard]] bool inventory_item_meta_build(endstone::Server &server, const ItemMeta &meta, const std::string &type_id,
                                             const ItemStackCollection &projectiles,
                                             std::unique_ptr<endstone::ItemMeta> &out,
                                             bool &out_kind_mismatch) noexcept;
[[nodiscard]] bool inventory_item_meta_read(const endstone::ItemMeta &meta, ItemMeta &out) noexcept;
[[nodiscard]] bool inventory_item_meta_read_projectiles(const endstone::ItemMeta &meta,
                                                        ItemStackCollection &out) noexcept;

namespace detail {

[[nodiscard]] bool create_item_meta_for_type(const ::aegilex::native::server::Server &server, rust::Str type_id,
                                             ItemMeta &out, ItemStackCollection &projectiles) noexcept;
[[nodiscard]] bool is_item_meta_applicable(const ::aegilex::native::server::Server &server, rust::Str type_id,
                                           const ItemMeta &meta, const ItemStackCollection &projectiles,
                                           bool &out) noexcept;
[[nodiscard]] bool are_item_metas_equal(const ::aegilex::native::server::Server &server, bool has_a, const ItemMeta &a,
                                        const ItemStackCollection &a_projectiles, bool has_b, const ItemMeta &b,
                                        const ItemStackCollection &b_projectiles, bool &out) noexcept;
[[nodiscard]] bool convert_item_meta_for_type(const ::aegilex::native::server::Server &server, rust::Str type_id,
                                              const ItemMeta &meta, const ItemStackCollection &projectiles,
                                              ItemMeta &out, ItemStackCollection &converted_projectiles) noexcept;

} // namespace detail

} // namespace aegilex::native::inventory
