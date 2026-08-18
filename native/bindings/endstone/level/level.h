#pragma once

#include "rust/cxx.h"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <vector>

namespace endstone {
class Dimension;
class Level;
class Server;
} // namespace endstone

namespace aegilex::native::actor {
class Actor;
}

namespace aegilex::native::inventory {
class ItemStack;
}

namespace aegilex::native::level {

struct Location;
struct DimensionSummary;
struct ChunkSummary;
struct BlockData;
struct BlockSnapshot;
struct BlockStatePair;

class Dimension; // defined in dimension.h
class Chunk;     // defined in chunk.h
class Block;     // defined in block.h

class ActorCollection {
  public:
    ActorCollection() = default;
    ~ActorCollection() noexcept = default;

    ActorCollection(const ActorCollection &) = delete;
    ActorCollection &operator=(const ActorCollection &) = delete;

    void push(std::unique_ptr<::aegilex::native::actor::Actor> actor);
    [[nodiscard]] std::size_t len() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::actor::Actor> get(std::size_t index) const;

  private:
    std::vector<std::unique_ptr<::aegilex::native::actor::Actor>> actors_;
};

// OOP/Pimpl facade over endstone::Level. The impl holds only a non-owning
// endstone::Level* (BDS owns the level); no VM semantics live here.
class Level {
  public:
    explicit Level(endstone::Level *level, endstone::Server *server = nullptr) noexcept;
    ~Level() noexcept = default;

    Level(const Level &) = delete;
    Level &operator=(const Level &) = delete;

    rust::String getName() const;
    std::int32_t getTime() const;
    void setTime(std::int32_t time) const;
    std::int64_t getSeed() const;
    rust::Vec<DimensionSummary> getDimensions() const;
    std::unique_ptr<Dimension> getDimension(rust::Str name) const;
    [[nodiscard]] std::unique_ptr<Block> getBlock(rust::Str dimension, std::int32_t x, std::int32_t y,
                                                  std::int32_t z) const;
    [[nodiscard]] std::unique_ptr<Block> getHighestBlock(rust::Str dimension, std::int32_t x, std::int32_t z) const;
    [[nodiscard]] std::unique_ptr<Block> getRelativeBlock(rust::Str dimension, std::int32_t x, std::int32_t y,
                                                          std::int32_t z, std::int32_t dx, std::int32_t dy,
                                                          std::int32_t dz) const;
    [[nodiscard]] std::unique_ptr<ActorCollection> getActors(rust::Str dimension) const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::actor::Actor>
    spawnActor(rust::Str dimension, const Location &location, rust::Str actor_type) const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::actor::Actor>
    dropItem(rust::Str dimension, const Location &location, const ::aegilex::native::inventory::ItemStack &item) const;
    [[nodiscard]] std::uint32_t setBlock(rust::Str dimension, std::int32_t x, std::int32_t y, std::int32_t z,
                                         rust::Str type_id, const rust::Vec<BlockStatePair> &states,
                                         bool has_apply_physics, bool apply_physics) const;
    [[nodiscard]] std::uint32_t getHighestBlockY(rust::Str dimension, std::int32_t x, std::int32_t z,
                                                 std::int32_t &out_y) const;
    [[nodiscard]] std::uint32_t listLoadedChunks(rust::Str dimension, rust::Vec<ChunkSummary> &out) const;
    [[nodiscard]] std::uint32_t updateBlockState(const BlockSnapshot &state, bool has_force, bool force,
                                                 bool has_apply_physics, bool apply_physics, bool &out_applied) const;
    [[nodiscard]] std::uint32_t createBlockData(rust::Str type_id, const rust::Vec<BlockStatePair> &states,
                                                BlockData &out) const;
    [[nodiscard]] std::uint32_t blockTypeHasItem(rust::Str type_id, bool &out_has_item) const;
    [[nodiscard]] endstone::Level *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

} // namespace aegilex::native::level
