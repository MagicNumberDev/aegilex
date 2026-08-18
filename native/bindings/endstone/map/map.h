#pragma once

#include "rust/cxx.h"

#include <cstddef>
#include <cstdint>
#include <memory>

namespace endstone {
class MapView;
class Server;
} // namespace endstone

namespace aegilex::native::ui {

// OOP/Pimpl facade over endstone::MapView. The impl holds only a non-owning
// endstone::MapView*; the server owns the map. Mirrors the layout of
// endstone/map/map_view.h; Scale is passed as a raw u8.
class Map {
  public:
    explicit Map(endstone::MapView *map, endstone::Server *server = nullptr) noexcept;
    ~Map() noexcept = default;

    Map(const Map &) = delete;
    Map &operator=(const Map &) = delete;

    std::int64_t getId() const;
    bool isVirtual() const;
    std::uint8_t getScale() const;
    void setScale(std::uint8_t scale) const;
    std::int32_t getCenterX() const;
    void setCenterX(std::int32_t x) const;
    std::int32_t getCenterZ() const;
    void setCenterZ(std::int32_t z) const;
    rust::String getDimensionName() const;
    bool isUnlimitedTracking() const;
    void setUnlimitedTracking(bool unlimited) const;
    bool isLocked() const;
    void setLocked(bool locked) const;
    [[nodiscard]] bool setDimension(rust::Str dimension) const;
    [[nodiscard]] endstone::MapView *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

} // namespace aegilex::native::ui
