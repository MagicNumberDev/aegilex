#pragma once

#include "rust/cxx.h"

#include <cstdint>
#include <memory>

namespace endstone {
class Dimension;
}

namespace aegilex::native::level {

class Dimension;

// OOP/Pimpl facade over endstone::Dimension; non-owning, mirrors the layout
// of endstone/level/dimension.h.
class Dimension {
  public:
    explicit Dimension(endstone::Dimension *dimension) noexcept;
    ~Dimension() noexcept = default;

    Dimension(const Dimension &) = delete;
    Dimension &operator=(const Dimension &) = delete;

    rust::String getName() const;
    std::uint32_t getType() const;
    rust::String getLevelName() const;
    [[nodiscard]] endstone::Dimension *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

} // namespace aegilex::native::level
