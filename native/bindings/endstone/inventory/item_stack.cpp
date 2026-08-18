#include "item_stack.h"

#include "item_meta.h"

#include "bindings/endstone/server.h"

#include <aegilex-runtime/src/cxx_host_inventory.rs.h>

#include <endstone/inventory/item_stack.h>

#include <cstdint>
#include <limits>
#include <memory>
#include <set>
#include <string>
#include <utility>
#include <vector>

namespace aegilex::native::inventory {

namespace {

constexpr std::uint8_t kNbtEnd = 0;
constexpr std::uint8_t kNbtByte = 1;
constexpr std::uint8_t kNbtShort = 2;
constexpr std::uint8_t kNbtInt = 3;
constexpr std::uint8_t kNbtLong = 4;
constexpr std::uint8_t kNbtFloat = 5;
constexpr std::uint8_t kNbtDouble = 6;
constexpr std::uint8_t kNbtByteArray = 7;
constexpr std::uint8_t kNbtString = 8;
constexpr std::uint8_t kNbtList = 9;
constexpr std::uint8_t kNbtCompound = 10;
constexpr std::uint8_t kNbtIntArray = 11;
[[nodiscard]] bool build_nbt_tag(const Nbt &tree, const std::uint32_t index, std::vector<std::uint8_t> &state,
                                 endstone::nbt::Tag &out)
{
    if (index >= tree.nodes.size() || state[index] != 0) {
        return false;
    }
    state[index] = 1;
    const auto &node = tree.nodes[index];
    endstone::nbt::Tag tag;
    switch (node.kind) {
    case kNbtEnd:
        break;
    case kNbtByte:
        tag = endstone::ByteTag(node.byte_value);
        break;
    case kNbtShort:
        tag = endstone::ShortTag(node.short_value);
        break;
    case kNbtInt:
        tag = endstone::IntTag(node.int_value);
        break;
    case kNbtLong:
        tag = endstone::LongTag(node.long_value);
        break;
    case kNbtFloat:
        tag = endstone::FloatTag(node.float_value);
        break;
    case kNbtDouble:
        tag = endstone::DoubleTag(node.double_value);
        break;
    case kNbtByteArray: {
        std::vector<std::uint8_t> values;
        values.reserve(node.byte_array.size());
        for (const auto value : node.byte_array) {
            values.push_back(value);
        }
        tag = endstone::ByteArrayTag(std::move(values));
        break;
    }
    case kNbtString:
        tag = endstone::StringTag(std::string(node.string_value));
        break;
    case kNbtList: {
        endstone::ListTag list;
        auto element_type = endstone::nbt::Type::End;
        for (const auto child_index : node.child_indices) {
            endstone::nbt::Tag child;
            if (!build_nbt_tag(tree, child_index, state, child) || child.type() == endstone::nbt::Type::End) {
                return false;
            }
            if (element_type == endstone::nbt::Type::End) {
                element_type = child.type();
            }
            else if (element_type != child.type()) {
                return false;
            }
            list.emplace_back(std::move(child));
        }
        tag = std::move(list);
        break;
    }
    case kNbtCompound: {
        endstone::CompoundTag compound;
        std::set<std::string> keys;
        for (const auto &entry : node.entries) {
            const std::string key(entry.key);
            if (!keys.insert(key).second) {
                return false;
            }
            endstone::nbt::Tag child;
            if (!build_nbt_tag(tree, entry.value_index, state, child)) {
                return false;
            }
            compound.insert_or_assign(key, std::move(child));
        }
        tag = std::move(compound);
        break;
    }
    case kNbtIntArray: {
        std::vector<std::int32_t> values;
        values.reserve(node.int_array.size());
        for (const auto value : node.int_array) {
            values.push_back(value);
        }
        tag = endstone::IntArrayTag(std::move(values));
        break;
    }
    default:
        return false;
    }
    state[index] = 2;
    out = std::move(tag);
    return true;
}

[[nodiscard]] bool nbt_to_compound(const Nbt &tree, endstone::CompoundTag &out)
{
    if (tree.nodes.empty() || tree.root_index >= tree.nodes.size()) {
        return false;
    }
    std::vector<std::uint8_t> state(tree.nodes.size());
    endstone::nbt::Tag root;
    if (!build_nbt_tag(tree, tree.root_index, state, root) || root.type() != endstone::nbt::Type::Compound) {
        return false;
    }
    for (const auto value : state) {
        if (value != 2) {
            return false;
        }
    }
    out = root.get<endstone::CompoundTag>();
    return true;
}

[[nodiscard]] bool append_nbt_tag(const endstone::nbt::Tag &tag, Nbt &out, std::uint32_t &out_index)
{
    if (out.nodes.size() >= std::numeric_limits<std::uint32_t>::max()) {
        return false;
    }
    out_index = static_cast<std::uint32_t>(out.nodes.size());
    out.nodes.push_back(NbtNode{});
    auto &node = out.nodes[out_index];
    switch (tag.type()) {
    case endstone::nbt::Type::End:
        node.kind = kNbtEnd;
        return true;
    case endstone::nbt::Type::Byte:
        node.kind = kNbtByte;
        node.byte_value = tag.get<endstone::ByteTag>().value();
        return true;
    case endstone::nbt::Type::Short:
        node.kind = kNbtShort;
        node.short_value = tag.get<endstone::ShortTag>().value();
        return true;
    case endstone::nbt::Type::Int:
        node.kind = kNbtInt;
        node.int_value = tag.get<endstone::IntTag>().value();
        return true;
    case endstone::nbt::Type::Long:
        node.kind = kNbtLong;
        node.long_value = tag.get<endstone::LongTag>().value();
        return true;
    case endstone::nbt::Type::Float:
        node.kind = kNbtFloat;
        node.float_value = tag.get<endstone::FloatTag>().value();
        return true;
    case endstone::nbt::Type::Double:
        node.kind = kNbtDouble;
        node.double_value = tag.get<endstone::DoubleTag>().value();
        return true;
    case endstone::nbt::Type::ByteArray:
        node.kind = kNbtByteArray;
        for (const auto value : tag.get<endstone::ByteArrayTag>()) {
            node.byte_array.push_back(value);
        }
        return true;
    case endstone::nbt::Type::String:
        node.kind = kNbtString;
        node.string_value = rust::String(tag.get<endstone::StringTag>().value());
        return true;
    case endstone::nbt::Type::List:
        node.kind = kNbtList;
        for (const auto &child : tag.get<endstone::ListTag>()) {
            std::uint32_t child_index = 0;
            if (!append_nbt_tag(child, out, child_index)) {
                return false;
            }
            out.nodes[out_index].child_indices.push_back(child_index);
        }
        return true;
    case endstone::nbt::Type::Compound:
        node.kind = kNbtCompound;
        for (const auto &[key, child] : tag.get<endstone::CompoundTag>()) {
            std::uint32_t child_index = 0;
            if (!append_nbt_tag(child, out, child_index)) {
                return false;
            }
            NbtEntry entry{};
            entry.key = rust::String(key);
            entry.value_index = child_index;
            out.nodes[out_index].entries.push_back(std::move(entry));
        }
        return true;
    case endstone::nbt::Type::IntArray:
        node.kind = kNbtIntArray;
        for (const auto value : tag.get<endstone::IntArrayTag>()) {
            node.int_array.push_back(value);
        }
        return true;
    }
    return false;
}

[[nodiscard]] bool compound_to_nbt(const endstone::CompoundTag &compound, Nbt &out)
{
    out.root_index = 0;
    out.nodes.clear();
    return append_nbt_tag(endstone::nbt::Tag(compound), out, out.root_index);
}

} // namespace

class ItemStack::impl {
  public:
    explicit impl(endstone::ItemStack *stack) noexcept : stack(stack)
    {
    }

    explicit impl(endstone::ItemStack stack)
        : owned(std::make_unique<endstone::ItemStack>(std::move(stack))), stack(owned.get())
    {
    }

    std::unique_ptr<endstone::ItemStack> owned;
    endstone::ItemStack *stack;
};

ItemStack::ItemStack(endstone::ItemStack *stack) noexcept : impl(std::make_shared<class ItemStack::impl>(stack))
{
}

ItemStack::ItemStack(endstone::ItemStack stack) noexcept
    : impl(std::make_shared<class ItemStack::impl>(std::move(stack)))
{
}

endstone::ItemStack *ItemStack::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->stack;
}

std::unique_ptr<endstone::ItemStack> ItemStack::takeNative() noexcept
{
    if (impl == nullptr || impl->owned == nullptr) {
        return {};
    }
    impl->stack = nullptr;
    return std::move(impl->owned);
}

rust::String ItemStack::getType() const
{
    try {
        return rust::String(std::string(impl->stack->getType().getId()));
    }
    catch (...) {
        return rust::String();
    }
}

bool ItemStack::setType(const rust::Str type_id) const
{
    try {
        if (impl == nullptr || impl->stack == nullptr || type_id.empty()) {
            return false;
        }
        impl->stack->setType(std::string(type_id.data(), type_id.size()));
        return true;
    }
    catch (...) {
        return false;
    }
}

std::int32_t ItemStack::getAmount() const
{
    try {
        return impl->stack->getAmount();
    }
    catch (...) {
        return 0;
    }
}

void ItemStack::setAmount(const std::int32_t amount) const
{
    try {
        impl->stack->setAmount(amount);
    }
    catch (...) {
    }
}

std::int32_t ItemStack::getData() const
{
    try {
        return impl->stack->getData();
    }
    catch (...) {
        return 0;
    }
}

void ItemStack::setData(const std::int32_t data) const
{
    try {
        impl->stack->setData(data);
    }
    catch (...) {
    }
}

rust::String ItemStack::getTranslationKey() const
{
    try {
        return impl == nullptr || impl->stack == nullptr ? rust::String()
                                                         : rust::String(impl->stack->getTranslationKey());
    }
    catch (...) {
        return rust::String();
    }
}

std::int32_t ItemStack::getMaxStackSize() const
{
    try {
        return impl == nullptr || impl->stack == nullptr ? 0 : impl->stack->getMaxStackSize();
    }
    catch (...) {
        return 0;
    }
}

std::unique_ptr<ItemStack> ItemStack::cloneItemStack() const
{
    try {
        return native() == nullptr ? std::unique_ptr<ItemStack>() : std::make_unique<ItemStack>(*native());
    }
    catch (...) {
        return {};
    }
}

bool ItemStack::hasItemMeta() const
{
    try {
        return impl->stack->hasItemMeta();
    }
    catch (...) {
        return false;
    }
}

bool ItemStack::equals(const ItemStack &other) const
{
    try {
        return *impl->stack == *other.impl->stack;
    }
    catch (...) {
        return false;
    }
}

bool ItemStack::isSimilar(const ItemStack &other) const
{
    try {
        return impl->stack->isSimilar(*other.impl->stack);
    }
    catch (...) {
        return false;
    }
}

bool ItemStack::getMeta(ItemMeta &out) const
{
    try {
        if (native() == nullptr) {
            return false;
        }
        const auto meta = native()->getItemMeta();
        if (meta == nullptr) {
            out = ItemMeta{};
            return true;
        }
        return inventory_item_meta_read(*meta, out);
    }
    catch (...) {
        return false;
    }
}

std::unique_ptr<ItemStackCollection> ItemStack::getChargedProjectiles() const
{
    try {
        auto projectiles = ItemStackCollection::create();
        if (projectiles == nullptr || native() == nullptr) {
            return {};
        }
        const auto meta = native()->getItemMeta();
        if (meta == nullptr || meta->getType() != endstone::ItemMeta::Type::CrossBow) {
            return projectiles;
        }
        return inventory_item_meta_read_projectiles(*meta, *projectiles) ? std::move(projectiles)
                                                                         : std::unique_ptr<ItemStackCollection>();
    }
    catch (...) {
        return {};
    }
}

bool ItemStack::setMeta(const ::aegilex::native::server::Server &server, const ItemMeta &meta,
                        const ItemStackCollection &projectiles, bool &out_success) const
{
    out_success = false;
    try {
        if (server.native() == nullptr || native() == nullptr) {
            return false;
        }
        std::unique_ptr<endstone::ItemMeta> built;
        bool mismatch = false;
        if (!inventory_item_meta_build(*server.native(), meta, std::string(getType()), projectiles, built, mismatch)) {
            return false;
        }
        out_success = !mismatch && native()->setItemMeta(built.get());
        return true;
    }
    catch (...) {
        return false;
    }
}

bool ItemStack::getNbt(Nbt &out) const
{
    try {
        return native() != nullptr && compound_to_nbt(native()->getNbt(), out);
    }
    catch (...) {
        return false;
    }
}

bool ItemStack::setNbt(const Nbt &value) const
{
    try {
        endstone::CompoundTag compound;
        if (native() == nullptr || !nbt_to_compound(value, compound)) {
            return false;
        }
        native()->setNbt(compound);
        return true;
    }
    catch (...) {
        return false;
    }
}

ItemStackRef::ItemStackRef(const endstone::ItemStack *stack) noexcept : stack_(stack)
{
}

rust::String ItemStackRef::getType() const
{
    try {
        return stack_ == nullptr ? rust::String() : rust::String(std::string(stack_->getType().getId()));
    }
    catch (...) {
        return rust::String();
    }
}

std::int32_t ItemStackRef::getAmount() const
{
    try {
        return stack_ == nullptr ? 0 : stack_->getAmount();
    }
    catch (...) {
        return 0;
    }
}

std::int32_t ItemStackRef::getData() const
{
    try {
        return stack_ == nullptr ? 0 : stack_->getData();
    }
    catch (...) {
        return 0;
    }
}

rust::String ItemStackRef::getTranslationKey() const
{
    try {
        return stack_ == nullptr ? rust::String() : rust::String(stack_->getTranslationKey());
    }
    catch (...) {
        return rust::String();
    }
}

std::int32_t ItemStackRef::getMaxStackSize() const
{
    try {
        return stack_ == nullptr ? 0 : stack_->getMaxStackSize();
    }
    catch (...) {
        return 0;
    }
}

std::unique_ptr<ItemStack> ItemStackRef::cloneItemStack() const
{
    try {
        return stack_ == nullptr ? std::unique_ptr<ItemStack>() : std::make_unique<ItemStack>(*stack_);
    }
    catch (...) {
        return {};
    }
}

bool ItemStackRef::hasItemMeta() const
{
    try {
        return stack_ != nullptr && stack_->hasItemMeta();
    }
    catch (...) {
        return false;
    }
}

bool ItemStackRef::isSimilar(const ItemStack &other) const
{
    try {
        return stack_ != nullptr && other.native() != nullptr && stack_->isSimilar(*other.native());
    }
    catch (...) {
        return false;
    }
}

bool ItemStackRef::getMeta(ItemMeta &out) const
{
    try {
        if (stack_ == nullptr) {
            return false;
        }
        const auto meta = stack_->getItemMeta();
        if (meta == nullptr) {
            out = ItemMeta{};
            return true;
        }
        return inventory_item_meta_read(*meta, out);
    }
    catch (...) {
        return false;
    }
}

std::unique_ptr<ItemStackCollection> ItemStackRef::getChargedProjectiles() const
{
    try {
        auto projectiles = ItemStackCollection::create();
        if (projectiles == nullptr || stack_ == nullptr) {
            return {};
        }
        const auto meta = stack_->getItemMeta();
        if (meta == nullptr || meta->getType() != endstone::ItemMeta::Type::CrossBow) {
            return projectiles;
        }
        return inventory_item_meta_read_projectiles(*meta, *projectiles) ? std::move(projectiles)
                                                                         : std::unique_ptr<ItemStackCollection>();
    }
    catch (...) {
        return {};
    }
}

bool ItemStackRef::getNbt(Nbt &out) const
{
    try {
        return stack_ != nullptr && compound_to_nbt(stack_->getNbt(), out);
    }
    catch (...) {
        return false;
    }
}

std::unique_ptr<ItemStackRef> borrow_item_stack(const ItemStack &stack) noexcept
{
    try {
        return std::make_unique<ItemStackRef>(stack.native());
    }
    catch (...) {
        return {};
    }
}

std::unique_ptr<ItemStackCollection> ItemStackCollection::create() noexcept
{
    try {
        return std::make_unique<ItemStackCollection>();
    }
    catch (...) {
        return {};
    }
}

void ItemStackCollection::push(const ItemStack &item)
{
    try {
        if (item.native() != nullptr) {
            items_.push_back(std::make_unique<ItemStack>(*item.native()));
        }
    }
    catch (...) {
    }
}

std::size_t ItemStackCollection::len() const noexcept
{
    return items_.size();
}

std::unique_ptr<ItemStack> ItemStackCollection::get(const std::size_t index) const
{
    try {
        if (index >= items_.size() || items_[index] == nullptr || items_[index]->native() == nullptr) {
            return std::unique_ptr<ItemStack>();
        }
        return std::make_unique<ItemStack>(*items_[index]->native());
    }
    catch (...) {
        return std::unique_ptr<ItemStack>();
    }
}

} // namespace aegilex::native::inventory
