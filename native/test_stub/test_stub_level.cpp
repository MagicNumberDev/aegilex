// Test-only typed level/dimension/chunk/block bridge stubs. Never linked into the plugin.

#include <aegilex-runtime/src/cxx_host_level.rs.h>

#include "bindings/endstone/level/block.h"
#include "bindings/endstone/level/chunk.h"
#include "bindings/endstone/level/dimension.h"
#include "bindings/endstone/level/level.h"
#include "bindings/endstone/actor/actor.h"
#include "bindings/endstone/inventory/item_stack.h"

#include <cstdint>
#include <memory>
#include <string>

// The test-only facades never dereference a native block, but constructing the
// owning facade requires a complete pointee type for std::unique_ptr.
namespace endstone {
class Block {};
} // namespace endstone

namespace aegilex::native::level {

class Level::impl {
  public:
    impl() noexcept = default;
};

class Dimension::impl {
  public:
    impl() noexcept = default;
};

class Chunk::impl {
  public:
    impl() noexcept = default;
};

class Block::impl {
  public:
    std::string type = "minecraft:stone";
};


Level::Level(endstone::Level *, endstone::Server *) noexcept : impl(std::make_shared<class Level::impl>())
{
}

Dimension::Dimension(endstone::Dimension *) noexcept : impl(std::make_shared<class Dimension::impl>())
{
}

Chunk::Chunk(endstone::Chunk *) noexcept : impl(std::make_shared<class Chunk::impl>())
{
}

Block::Block(std::unique_ptr<endstone::Block>, endstone::Server *) noexcept
    : impl(std::make_shared<class Block::impl>())
{
}

Block::Block(const endstone::Block &, endstone::Server *) noexcept
    : impl(std::make_shared<class Block::impl>())
{
}

Block::~Block() noexcept = default;


rust::String Level::getName() const
{
    return rust::String("Aegilex");
}

std::int32_t Level::getTime() const
{
    return 7;
}

void Level::setTime(std::int32_t) const
{
}

std::int64_t Level::getSeed() const
{
    return 7;
}

rust::Vec<DimensionSummary> Level::getDimensions() const
{
    rust::Vec<DimensionSummary> dimensions;
    dimensions.push_back(
        DimensionSummary{.name = rust::String("overworld"), .kind = 0, .level = rust::String("Aegilex")});
    return dimensions;
}

std::unique_ptr<Dimension> Level::getDimension(rust::Str) const
{
    return std::unique_ptr<Dimension>(new Dimension(nullptr));
}

std::unique_ptr<Block> Level::getBlock(rust::Str, std::int32_t, std::int32_t, std::int32_t) const
{
    return std::unique_ptr<Block>(new Block(std::unique_ptr<endstone::Block>()));
}

std::unique_ptr<Block> Level::getHighestBlock(rust::Str, std::int32_t, std::int32_t) const
{
    return std::unique_ptr<Block>(new Block(std::unique_ptr<endstone::Block>()));
}

std::unique_ptr<Block> Level::getRelativeBlock(rust::Str, std::int32_t, std::int32_t, std::int32_t, std::int32_t,
                                               std::int32_t, std::int32_t) const
{
    return std::unique_ptr<Block>(new Block(std::unique_ptr<endstone::Block>()));
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

std::unique_ptr<::aegilex::native::actor::Actor> ActorCollection::get(std::size_t index) const
{
    return index < actors_.size() ? actors_[index]->clone() : std::unique_ptr<::aegilex::native::actor::Actor>();
}

std::unique_ptr<ActorCollection> Level::getActors(rust::Str) const
{
    auto actors = std::make_unique<ActorCollection>();
    actors->push(std::make_unique<::aegilex::native::actor::Actor>(reinterpret_cast<endstone::Actor *>(1)));
    return actors;
}

std::unique_ptr<::aegilex::native::actor::Actor> Level::spawnActor(rust::Str, const Location &, rust::Str) const
{
    return std::make_unique<::aegilex::native::actor::Actor>(nullptr);
}

std::unique_ptr<::aegilex::native::actor::Actor> Level::dropItem(rust::Str, const Location &,
                                                                 const ::aegilex::native::inventory::ItemStack &) const
{
    return std::make_unique<::aegilex::native::actor::Actor>(nullptr);
}

rust::String Dimension::getName() const
{
    return rust::String("overworld");
}

std::uint32_t Dimension::getType() const
{
    return 0;
}

rust::String Dimension::getLevelName() const
{
    return rust::String("Aegilex");
}

std::int32_t Chunk::getX() const
{
    return 1;
}

std::int32_t Chunk::getZ() const
{
    return 2;
}

rust::String Chunk::getLevelName() const
{
    return rust::String("Aegilex");
}

rust::String Chunk::getDimensionName() const
{
    return rust::String("overworld");
}

std::unique_ptr<Block> Block::getRelative(std::int32_t, std::int32_t, std::int32_t) const
{
    return std::unique_ptr<Block>(new Block(std::unique_ptr<endstone::Block>()));
}

std::unique_ptr<Block> Block::clone() const
{
    return std::unique_ptr<Block>(new Block(std::unique_ptr<endstone::Block>()));
}


rust::String Block::getType() const
{
    return rust::String(impl->type);
}

void Block::setType(const rust::Str type, bool) const
{
    impl->type = std::string(type);
}

std::int32_t Block::getX() const
{
    return 1;
}

std::int32_t Block::getY() const
{
    return 2;
}

std::int32_t Block::getZ() const
{
    return 3;
}

Location Block::getLocation() const
{
    return Location{
        .x = 1.0F, .y = 2.0F, .z = 3.0F, .pitch = 0.0F, .yaw = 0.0F, .dimension = rust::String("overworld")};
}

BlockData Block::getData() const
{
    return BlockData{
        .type_id = rust::String("minecraft:stone"), .states = rust::Vec<BlockStatePair>(), .runtime_id = 7};
}

BlockSnapshot Block::captureState() const
{
    return BlockSnapshot{.dimension = rust::String("overworld"),
                         .x = 1,
                         .y = 2,
                         .z = 3,
                         .type_id = rust::String("minecraft:stone"),
                         .states = rust::Vec<BlockStatePair>(),
                         .runtime_id = 7};
}


std::uint32_t Level::setBlock(rust::Str, std::int32_t, std::int32_t, std::int32_t, rust::Str,
                              const rust::Vec<BlockStatePair> &, bool, bool) const
{
    return 0;
}

std::uint32_t Level::getHighestBlockY(rust::Str, std::int32_t, std::int32_t, std::int32_t &out_y) const
{
    out_y = 7;
    return 0;
}

std::uint32_t Level::listLoadedChunks(rust::Str, rust::Vec<ChunkSummary> &out) const
{
    out.push_back(
        ChunkSummary{.dimension = rust::String("overworld"), .x = 1, .z = 2, .level_name = rust::String("Aegilex")});
    return 0;
}

std::uint32_t Level::updateBlockState(const BlockSnapshot &, bool, bool, bool, bool, bool &out_applied) const
{
    out_applied = true;
    return 0;
}

std::uint32_t Level::createBlockData(rust::Str, const rust::Vec<BlockStatePair> &, BlockData &out) const
{
    out = BlockData{.type_id = rust::String("minecraft:stone"), .states = rust::Vec<BlockStatePair>(), .runtime_id = 7};
    return 0;
}

std::uint32_t Level::blockTypeHasItem(rust::Str, bool &out_has_item) const
{
    out_has_item = true;
    return 0;
}

std::uint32_t Block::setData(const BlockData &, bool) const
{
    return 0;
}

} // namespace aegilex::native::level
