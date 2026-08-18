#pragma once

#include <cstdint>
#include <string>

#include <rust/cxx.h>

namespace endstone {
class ChunkEvent;
}

namespace aegilex::native::endstone_binding::events {

// Copies the only safe v0.11.6 ChunkEvent values while the callback is active.
class ChunkEventFacade final {
  public:
    explicit ChunkEventFacade(const endstone::ChunkEvent &event) noexcept;
    ChunkEventFacade(std::int32_t chunk_x, std::int32_t chunk_z, std::string dimension) noexcept;
    ~ChunkEventFacade() noexcept = default;

    ChunkEventFacade(const ChunkEventFacade &) = delete;
    ChunkEventFacade &operator=(const ChunkEventFacade &) = delete;
    ChunkEventFacade(ChunkEventFacade &&) = delete;
    ChunkEventFacade &operator=(ChunkEventFacade &&) = delete;

    [[nodiscard]] std::int32_t getChunkX() const noexcept;
    [[nodiscard]] std::int32_t getChunkZ() const noexcept;
    [[nodiscard]] rust::String getDimensionForRust() const noexcept;

  private:
    std::int32_t chunk_x_;
    std::int32_t chunk_z_;
    std::string dimension_;
};

} // namespace aegilex::native::endstone_binding::events
