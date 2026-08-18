#pragma once

#include "rust/cxx.h"

#include <cstdint>
#include <memory>

namespace endstone {
class Chunk;
}

namespace aegilex::native::level {

class Chunk;

// OOP/Pimpl facade over endstone::Chunk; non-owning, mirrors the layout of
// endstone/level/chunk.h.
class Chunk {
  public:
    explicit Chunk(endstone::Chunk *chunk) noexcept;
    ~Chunk() noexcept = default;

    Chunk(const Chunk &) = delete;
    Chunk &operator=(const Chunk &) = delete;

    std::int32_t getX() const;
    std::int32_t getZ() const;
    rust::String getLevelName() const;
    rust::String getDimensionName() const;
    [[nodiscard]] endstone::Chunk *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

} // namespace aegilex::native::level
