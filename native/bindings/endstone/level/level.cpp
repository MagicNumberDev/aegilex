#include "level.h"
#include "../../../aegilex_types.h"
#include "block.h"
#include "dimension.h"
#include "../actor/actor.h"
#include "../inventory/item_stack.h"

#include <aegilex-runtime/src/cxx_host_level.rs.h>

#include <endstone/actor/item.h>
#include <endstone/block/block.h>
#include <endstone/block/block_data.h>
#include <endstone/block/block_state.h>
#include <endstone/block/block_type.h>
#include <endstone/level/dimension.h>
#include <endstone/level/level.h>
#include <endstone/server.h>

#include <cstdint>
#include <memory>
#include <string>
#include <string_view>
#include <type_traits>
#include <unordered_map>
#include <variant>
#include <vector>

namespace aegilex::native::level {

class Level::impl {
  public:
    explicit impl(endstone::Level *level, endstone::Server *server) noexcept : level(level), server(server)
    {
    }

    endstone::Level *level;
    endstone::Server *server;
};

namespace {

constexpr std::uint32_t kBlockStateBoolean = 0U;
constexpr std::uint32_t kBlockStateText = 1U;
constexpr std::uint32_t kBlockStateInteger = 2U;

[[nodiscard]] endstone::Dimension *resolve_dimension(const Level &level, const std::string_view dimension) noexcept
{
    if (dimension.empty() || level.native() == nullptr) {
        return nullptr;
    }
    return level.native()->getDimension(std::string(dimension));
}

[[nodiscard]] aegilex::status block_states_from_pairs(const rust::Vec<BlockStatePair> &pairs,
                                                      endstone::BlockStates *out) noexcept
{
    for (const auto &pair : pairs) {
        switch (pair.value_kind) {
        case kBlockStateBoolean:
            out->emplace(std::string(pair.key), pair.boolean);
            break;
        case kBlockStateText:
            out->emplace(std::string(pair.key), std::string(pair.text));
            break;
        case kBlockStateInteger:
            out->emplace(std::string(pair.key), pair.integer);
            break;
        default:
            return aegilex::kInvalidArgument;
        }
    }
    return aegilex::kOk;
}

void block_states_to_pairs(const endstone::BlockStates &states, rust::Vec<BlockStatePair> *out) noexcept
{
    for (const auto &[key, value] : states) {
        BlockStatePair pair;
        pair.key = rust::String(key);
        std::visit(
            [&pair](const auto &stored) {
                using Stored = std::decay_t<decltype(stored)>;
                if constexpr (std::is_same_v<Stored, bool>) {
                    pair.value_kind = kBlockStateBoolean;
                    pair.boolean = stored;
                }
                else if constexpr (std::is_same_v<Stored, std::string>) {
                    pair.value_kind = kBlockStateText;
                    pair.text = rust::String(stored);
                }
                else {
                    pair.value_kind = kBlockStateInteger;
                    pair.integer = stored;
                }
            },
            value);
        out->push_back(std::move(pair));
    }
}

} // namespace

Level::Level(endstone::Level *level, endstone::Server *server) noexcept
    : impl(std::make_shared<class Level::impl>(level, server))
{
}

endstone::Level *Level::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->level;
}

rust::String Level::getName() const
{
    try {
        return rust::String(impl->level->getName());
    }
    catch (...) {
        return rust::String();
    }
}

std::int32_t Level::getTime() const
{
    try {
        return impl->level->getTime();
    }
    catch (...) {
        return 0;
    }
}

void Level::setTime(const std::int32_t time) const
{
    try {
        impl->level->setTime(time);
    }
    catch (...) {
    }
}

std::int64_t Level::getSeed() const
{
    try {
        return impl->level->getSeed();
    }
    catch (...) {
        return 0;
    }
}

rust::Vec<DimensionSummary> Level::getDimensions() const
{
    rust::Vec<DimensionSummary> dimensions;
    try {
        for (auto *dimension : impl->level->getDimensions()) {
            DimensionSummary summary;
            summary.name = rust::String(dimension->getName());
            summary.kind = static_cast<std::uint32_t>(dimension->getType());
            summary.level = rust::String(dimension->getLevel().getName());
            dimensions.push_back(std::move(summary));
        }
    }
    catch (...) {
    }
    return dimensions;
}

std::unique_ptr<Dimension> Level::getDimension(const rust::Str name) const
{
    try {
        auto *dimension = impl->level->getDimension(std::string(name));
        if (dimension == nullptr) {
            return std::unique_ptr<Dimension>();
        }
        return std::unique_ptr<Dimension>(new Dimension(dimension));
    }
    catch (...) {
        return std::unique_ptr<Dimension>();
    }
}

std::unique_ptr<Block> Level::getBlock(const rust::Str dimension, const std::int32_t x, const std::int32_t y,
                                       const std::int32_t z) const
{
    try {
        if (impl == nullptr || impl->level == nullptr || dimension.empty()) {
            return std::unique_ptr<Block>();
        }
        auto *dimension_ptr = impl->level->getDimension(std::string(dimension));
        if (dimension_ptr == nullptr) {
            return std::unique_ptr<Block>();
        }
        auto block = dimension_ptr->getBlockAt(x, y, z);
        return block == nullptr ? std::unique_ptr<Block>() : std::make_unique<Block>(std::move(block), impl->server);
    }
    catch (...) {
        return std::unique_ptr<Block>();
    }
}

std::unique_ptr<Block> Level::getHighestBlock(const rust::Str dimension, const std::int32_t x,
                                              const std::int32_t z) const
{
    try {
        if (impl == nullptr || impl->level == nullptr || dimension.empty()) {
            return std::unique_ptr<Block>();
        }
        auto *dimension_ptr = impl->level->getDimension(std::string(dimension));
        if (dimension_ptr == nullptr) {
            return std::unique_ptr<Block>();
        }
        auto block = dimension_ptr->getHighestBlockAt(x, z);
        return block == nullptr ? std::unique_ptr<Block>() : std::make_unique<Block>(std::move(block), impl->server);
    }
    catch (...) {
        return std::unique_ptr<Block>();
    }
}

std::unique_ptr<Block> Level::getRelativeBlock(const rust::Str dimension, const std::int32_t x, const std::int32_t y,
                                               const std::int32_t z, const std::int32_t dx, const std::int32_t dy,
                                               const std::int32_t dz) const
{
    try {
        auto block = getBlock(dimension, x, y, z);
        return block == nullptr ? std::unique_ptr<Block>() : block->getRelative(dx, dy, dz);
    }
    catch (...) {
        return std::unique_ptr<Block>();
    }
}

void ActorCollection::push(std::unique_ptr<::aegilex::native::actor::Actor> actor)
{
    if (actor != nullptr) {
        actors_.push_back(std::move(actor));
    }
}

std::size_t ActorCollection::len() const noexcept
{
    return actors_.size();
}

std::unique_ptr<::aegilex::native::actor::Actor> ActorCollection::get(const std::size_t index) const
{
    try {
        if (index >= actors_.size() || actors_[index] == nullptr) {
            return std::unique_ptr<::aegilex::native::actor::Actor>();
        }
        return actors_[index]->clone();
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::actor::Actor>();
    }
}

std::unique_ptr<ActorCollection> Level::getActors(const rust::Str dimension) const
{
    try {
        std::vector<endstone::Actor *> actors;
        if (dimension.empty()) {
            actors = impl->level->getActors();
        }
        else {
            auto *dimension_ptr = impl->level->getDimension(std::string(dimension));
            if (dimension_ptr == nullptr) {
                return std::unique_ptr<ActorCollection>();
            }
            actors = dimension_ptr->getActors();
        }
        auto collection = std::make_unique<ActorCollection>();
        for (auto *actor : actors) {
            if (actor != nullptr) {
                collection->push(std::make_unique<::aegilex::native::actor::Actor>(actor));
            }
        }
        return collection;
    }
    catch (...) {
        return std::unique_ptr<ActorCollection>();
    }
}

std::unique_ptr<::aegilex::native::actor::Actor> Level::spawnActor(const rust::Str dimension, const Location &location,
                                                                   const rust::Str actor_type) const
{
    try {
        auto *dimension_ptr = impl->level->getDimension(std::string(dimension));
        if (dimension_ptr == nullptr || actor_type.empty() ||
            std::string(location.dimension) != std::string(dimension)) {
            return std::unique_ptr<::aegilex::native::actor::Actor>();
        }
        const endstone::Location target{*dimension_ptr, location.x,     location.y,
                                        location.z,     location.pitch, location.yaw};
        auto *actor = dimension_ptr->spawnActor(target, std::string(actor_type));
        return actor == nullptr ? std::unique_ptr<::aegilex::native::actor::Actor>()
                                : std::make_unique<::aegilex::native::actor::Actor>(actor);
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::actor::Actor>();
    }
}

std::unique_ptr<::aegilex::native::actor::Actor>
Level::dropItem(const rust::Str dimension, const Location &location,
                const ::aegilex::native::inventory::ItemStack &stack) const
{
    try {
        auto *dimension_ptr = impl->level->getDimension(std::string(dimension));
        if (dimension_ptr == nullptr || stack.native() == nullptr ||
            std::string(location.dimension) != std::string(dimension)) {
            return std::unique_ptr<::aegilex::native::actor::Actor>();
        }
        const endstone::Location target{*dimension_ptr, location.x,     location.y,
                                        location.z,     location.pitch, location.yaw};
        auto &item = dimension_ptr->dropItem(target, *stack.native());
        return std::make_unique<::aegilex::native::actor::Actor>(static_cast<endstone::Actor *>(&item));
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::actor::Actor>();
    }
}

std::uint32_t Level::setBlock(const rust::Str dimension, const std::int32_t x, const std::int32_t y,
                              const std::int32_t z, const rust::Str type_id, const rust::Vec<BlockStatePair> &states,
                              const bool has_apply_physics, const bool apply_physics) const
{
    try {
        if (type_id.empty()) {
            return aegilex::kInvalidArgument;
        }
        if (impl == nullptr || impl->server == nullptr) {
            return aegilex::kHostError;
        }
        auto *dimension_ptr = resolve_dimension(*this, std::string_view(dimension.data(), dimension.size()));
        if (dimension_ptr == nullptr) {
            return aegilex::kNotFound;
        }
        const bool physics = has_apply_physics ? apply_physics : true;
        auto block = dimension_ptr->getBlockAt(x, y, z);
        if (!block) {
            return aegilex::kHostError;
        }
        const std::string type_name(type_id);
        block->setType(type_name, physics);
        endstone::BlockStates state_map;
        if (const auto status = block_states_from_pairs(states, &state_map); status != aegilex::kOk) {
            return status;
        }
        if (!state_map.empty()) {
            const auto data = impl->server->createBlockData(type_name, std::move(state_map));
            if (!data) {
                return aegilex::kHostError;
            }
            block->setData(*data, physics);
        }
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

std::uint32_t Level::getHighestBlockY(const rust::Str dimension, const std::int32_t x, const std::int32_t z,
                                      std::int32_t &out_y) const
{
    try {
        auto *dimension_ptr = resolve_dimension(*this, std::string_view(dimension.data(), dimension.size()));
        if (dimension_ptr == nullptr) {
            return aegilex::kNotFound;
        }
        out_y = dimension_ptr->getHighestBlockYAt(x, z);
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

std::uint32_t Level::listLoadedChunks(const rust::Str dimension, rust::Vec<ChunkSummary> &out) const
{
    try {
        auto *dimension_ptr = resolve_dimension(*this, std::string_view(dimension.data(), dimension.size()));
        if (dimension_ptr == nullptr) {
            return aegilex::kNotFound;
        }
        for (const auto &chunk : dimension_ptr->getLoadedChunks()) {
            ChunkSummary summary;
            summary.dimension = rust::String(chunk->getDimension().getName());
            summary.x = chunk->getX();
            summary.z = chunk->getZ();
            summary.level_name = rust::String(chunk->getLevel().getName());
            out.push_back(std::move(summary));
        }
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

std::uint32_t Level::updateBlockState(const BlockSnapshot &state, const bool has_force, const bool force,
                                      const bool has_apply_physics, const bool apply_physics, bool &out_applied) const
{
    try {
        if (impl == nullptr || impl->server == nullptr) {
            return aegilex::kHostError;
        }
        const std::string dimension_name(state.dimension);
        auto *dimension_ptr = resolve_dimension(*this, dimension_name);
        if (dimension_ptr == nullptr) {
            return aegilex::kNotFound;
        }
        const std::string type_name(state.type_id);
        if (type_name.empty()) {
            return aegilex::kInvalidArgument;
        }
        auto block = dimension_ptr->getBlockAt(state.x, state.y, state.z);
        if (!block) {
            return aegilex::kHostError;
        }
        auto captured = block->captureState();
        if (!captured) {
            return aegilex::kHostError;
        }
        captured->setType(type_name);
        endstone::BlockStates state_map;
        if (const auto status = block_states_from_pairs(state.states, &state_map); status != aegilex::kOk) {
            return status;
        }
        if (!state_map.empty()) {
            const auto data = impl->server->createBlockData(type_name, std::move(state_map));
            if (!data) {
                return aegilex::kHostError;
            }
            captured->setData(*data);
        }
        out_applied = captured->update(has_force ? force : false, has_apply_physics ? apply_physics : true);
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

std::uint32_t Level::createBlockData(const rust::Str type_id, const rust::Vec<BlockStatePair> &states,
                                     BlockData &out) const
{
    try {
        if (type_id.empty()) {
            return aegilex::kInvalidArgument;
        }
        if (impl == nullptr || impl->server == nullptr) {
            return aegilex::kHostError;
        }
        endstone::BlockStates state_map;
        if (const auto status = block_states_from_pairs(states, &state_map); status != aegilex::kOk) {
            return status;
        }
        const std::string type_name(type_id);
        const auto *block_type = impl->server->getRegistry<endstone::BlockType>().get(endstone::BlockTypeId{type_name});
        if (block_type == nullptr) {
            return aegilex::kNotFound;
        }
        const auto data = state_map.empty() ? impl->server->createBlockData(type_name)
                                            : impl->server->createBlockData(type_name, std::move(state_map));
        if (!data) {
            return aegilex::kHostError;
        }
        out.type_id = rust::String(data->getType());
        out.runtime_id = data->getRuntimeId();
        block_states_to_pairs(data->getBlockStates(), &out.states);
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

std::uint32_t Level::blockTypeHasItem(const rust::Str type_id, bool &out_has_item) const
{
    try {
        if (impl == nullptr || impl->server == nullptr) {
            return aegilex::kHostError;
        }
        const std::string type_name(type_id);
        if (type_name.empty()) {
            return aegilex::kInvalidArgument;
        }
        const auto *block_type = impl->server->getRegistry<endstone::BlockType>().get(endstone::BlockTypeId{type_name});
        if (block_type == nullptr) {
            return aegilex::kNotFound;
        }
        out_has_item = block_type->hasItemType();
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

} // namespace aegilex::native::level
