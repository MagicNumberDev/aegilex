#pragma once

#include "rust/cxx.h"

#include <cstdint>
#include <memory>
#include <vector>

namespace endstone {
class ItemStack;
class ItemMeta;
} // namespace endstone

namespace aegilex::native::server {
class Server;
}

namespace aegilex::native::inventory {

struct ItemMeta;
struct Nbt;

// OOP/Pimpl facade over endstone::ItemStack. Pointer construction borrows a
// native stack; value construction retains an owning snapshot. No VM semantics
// live here. Mirrors the layout of endstone/inventory/item_stack.h.
class ItemStack {
  public:
    explicit ItemStack(endstone::ItemStack *stack) noexcept;
    explicit ItemStack(endstone::ItemStack stack) noexcept;
    ~ItemStack() noexcept = default;

    ItemStack(const ItemStack &) = delete;
    ItemStack &operator=(const ItemStack &) = delete;

    rust::String getType() const;
    [[nodiscard]] bool setType(rust::Str type_id) const;
    std::int32_t getAmount() const;
    void setAmount(std::int32_t amount) const;
    std::int32_t getData() const;
    void setData(std::int32_t data) const;
    rust::String getTranslationKey() const;
    std::int32_t getMaxStackSize() const;
    [[nodiscard]] std::unique_ptr<ItemStack> cloneItemStack() const;
    bool hasItemMeta() const;
    bool equals(const ItemStack &other) const;
    bool isSimilar(const ItemStack &other) const;
    [[nodiscard]] bool getMeta(ItemMeta &out) const;
    [[nodiscard]] std::unique_ptr<class ItemStackCollection> getChargedProjectiles() const;
    [[nodiscard]] bool setMeta(const ::aegilex::native::server::Server &server, const ItemMeta &meta,
                               const class ItemStackCollection &projectiles, bool &out_success) const;
    [[nodiscard]] bool getNbt(Nbt &out) const;
    [[nodiscard]] bool setNbt(const Nbt &value) const;
    [[nodiscard]] endstone::ItemStack *native() const noexcept;
    [[nodiscard]] std::unique_ptr<endstone::ItemStack> takeNative() noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

// Read-only, non-owning facade for an externally owned ItemStack. This is
// restricted to the native callback that supplied the stack.
class ItemStackRef {
  public:
    explicit ItemStackRef(const endstone::ItemStack *stack) noexcept;
    ~ItemStackRef() noexcept = default;

    ItemStackRef(const ItemStackRef &) = delete;
    ItemStackRef &operator=(const ItemStackRef &) = delete;

    rust::String getType() const;
    std::int32_t getAmount() const;
    std::int32_t getData() const;
    rust::String getTranslationKey() const;
    std::int32_t getMaxStackSize() const;
    [[nodiscard]] std::unique_ptr<ItemStack> cloneItemStack() const;
    bool hasItemMeta() const;
    bool isSimilar(const ItemStack &other) const;
    [[nodiscard]] bool getMeta(ItemMeta &out) const;
    [[nodiscard]] std::unique_ptr<class ItemStackCollection> getChargedProjectiles() const;
    [[nodiscard]] bool getNbt(Nbt &out) const;

  private:
    const endstone::ItemStack *stack_;
};

class ItemStackCollection {
  public:
    [[nodiscard]] static std::unique_ptr<ItemStackCollection> create() noexcept;
    ItemStackCollection() = default;
    ~ItemStackCollection() noexcept = default;

    ItemStackCollection(const ItemStackCollection &) = delete;
    ItemStackCollection &operator=(const ItemStackCollection &) = delete;

    void push(const ItemStack &item);
    [[nodiscard]] std::size_t len() const noexcept;
    [[nodiscard]] std::unique_ptr<ItemStack> get(std::size_t index) const;

  private:
    std::vector<std::unique_ptr<ItemStack>> items_;
};

// Creates a read-only view over an owned facade's native stack. Runtime event
// adapters use the ItemStackRef constructor directly for external stacks.
[[nodiscard]] std::unique_ptr<ItemStackRef> borrow_item_stack(const ItemStack &stack) noexcept;

} // namespace aegilex::native::inventory
