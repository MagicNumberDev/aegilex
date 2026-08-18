#include "block.h"
#include "../../../aegilex_types.h"

#include <aegilex-runtime/src/cxx_host_level.rs.h>

#include <endstone/block/block.h>
#include <endstone/block/block_data.h>
#include <endstone/block/block_state.h>
#include <endstone/level/dimension.h>
#include <endstone/level/location.h>
#include <endstone/server.h>

#include <cstdint>
#include <memory>
#include <string>
#include <string_view>
#include <type_traits>
#include <variant>

namespace aegilex::native::level {

class Block::impl {
  public:
    explicit impl(std::unique_ptr<endstone::Block> block, endstone::Server *server) noexcept
        : block(std::move(block)), server(server)
    {
    }

    std::unique_ptr<endstone::Block> block;
    endstone::Server *server;
};


namespace {

constexpr std::uint32_t kBlockStateBoolean = 0U;
constexpr std::uint32_t kBlockStateText = 1U;
constexpr std::uint32_t kBlockStateInteger = 2U;

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

Block::Block(std::unique_ptr<endstone::Block> block, endstone::Server *server) noexcept
    : impl(std::make_shared<class Block::impl>(std::move(block), server))
{
}

Block::Block(const endstone::Block &block, endstone::Server *server) noexcept
    : impl(std::make_shared<class Block::impl>(block.clone(), server))
{
}

Block::~Block() noexcept = default;

rust::String Block::getType() const
{
    try {
        return rust::String(impl->block->getType());
    }
    catch (...) {
        return rust::String();
    }
}

void Block::setType(const rust::Str type_id, const bool apply_physics) const
{
    try {
        impl->block->setType(std::string(type_id), apply_physics);
    }
    catch (...) {
    }
}

std::int32_t Block::getX() const
{
    try {
        return impl->block->getX();
    }
    catch (...) {
        return 0;
    }
}

std::int32_t Block::getY() const
{
    try {
        return impl->block->getY();
    }
    catch (...) {
        return 0;
    }
}

std::int32_t Block::getZ() const
{
    try {
        return impl->block->getZ();
    }
    catch (...) {
        return 0;
    }
}

Location Block::getLocation() const
{
    try {
        const auto &location = impl->block->getLocation();
        return Location{.x = location.getX(),
                        .y = location.getY(),
                        .z = location.getZ(),
                        .pitch = location.getPitch(),
                        .yaw = location.getYaw(),
                        .dimension = rust::String(location.getDimension().getName())};
    }
    catch (...) {
        return Location{};
    }
}

BlockData Block::getData() const
{
    BlockData data;
    try {
        const auto block_data = impl->block->getData();
        if (block_data == nullptr) {
            return data;
        }
        data.type_id = rust::String(block_data->getType());
        data.runtime_id = block_data->getRuntimeId();
        block_states_to_pairs(block_data->getBlockStates(), &data.states);
    }
    catch (...) {
    }
    return data;
}

BlockSnapshot Block::captureState() const
{
    BlockSnapshot snapshot;
    try {
        auto state = impl->block->captureState();
        if (state == nullptr) {
            return snapshot;
        }
        snapshot.dimension = rust::String(impl->block->getDimension().getName());
        snapshot.x = state->getX();
        snapshot.y = state->getY();
        snapshot.z = state->getZ();
        snapshot.type_id = rust::String(state->getType());
        const auto data = state->getData();
        if (data != nullptr) {
            snapshot.runtime_id = data->getRuntimeId();
            block_states_to_pairs(data->getBlockStates(), &snapshot.states);
        }
    }
    catch (...) {
    }
    return snapshot;
}

std::unique_ptr<Block> Block::getRelative(const std::int32_t dx, const std::int32_t dy, const std::int32_t dz) const
{
    try {
        if (impl == nullptr || impl->block == nullptr) {
            return std::unique_ptr<Block>();
        }
        auto block = impl->block->getRelative(dx, dy, dz);
        return block == nullptr ? std::unique_ptr<Block>() : std::make_unique<Block>(std::move(block), impl->server);
    }
    catch (...) {
        return std::unique_ptr<Block>();
    }
}

std::unique_ptr<Block> Block::clone() const
{
    try {
        if (impl == nullptr || impl->block == nullptr) {
            return std::unique_ptr<Block>();
        }
        auto block = impl->block->clone();
        return block == nullptr ? std::unique_ptr<Block>() : std::make_unique<Block>(std::move(block), impl->server);
    }
    catch (...) {
        return std::unique_ptr<Block>();
    }
}


endstone::Block *Block::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->block.get();
}

std::uint32_t Block::setData(const BlockData &data, const bool apply_physics) const
{
    try {
        if (impl == nullptr || impl->server == nullptr || impl->block == nullptr) {
            return aegilex::kInvalidArgument;
        }
        const std::string type_name(data.type_id);
        if (type_name.empty()) {
            return aegilex::kInvalidArgument;
        }
        endstone::BlockStates state_map;
        if (const auto status = block_states_from_pairs(data.states, &state_map); status != aegilex::kOk) {
            return status;
        }
        const auto block_data = state_map.empty() ? impl->server->createBlockData(type_name)
                                                  : impl->server->createBlockData(type_name, std::move(state_map));
        if (!block_data) {
            return aegilex::kHostError;
        }
        impl->block->setData(*block_data, apply_physics);
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}


} // namespace aegilex::native::level
