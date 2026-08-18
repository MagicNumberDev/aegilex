#include "map.h"

#include "bindings/endstone/server.h"

#include <aegilex-runtime/src/cxx_host_ui.rs.h>

#include <endstone/level/dimension.h>
#include <endstone/level/level.h>
#include <endstone/map/map_view.h>
#include <endstone/server.h>

#include <optional>
#include <string>
#include <string_view>

namespace aegilex::native::ui {

class Map::impl {
  public:
    explicit impl(endstone::MapView *map, endstone::Server *server) noexcept : map(map), server(server)
    {
    }

    endstone::MapView *map;
    endstone::Server *server;
};

namespace {

[[nodiscard]] std::optional<endstone::MapView::Scale> to_endstone_scale(const std::uint8_t scale) noexcept
{
    switch (scale) {
    case 0:
        return endstone::MapView::Scale::Closest;
    case 1:
        return endstone::MapView::Scale::Close;
    case 2:
        return endstone::MapView::Scale::Normal;
    case 3:
        return endstone::MapView::Scale::Far;
    case 4:
        return endstone::MapView::Scale::Farthest;
    }
    return std::nullopt;
}

} // namespace

Map::Map(endstone::MapView *map, endstone::Server *server) noexcept
    : impl(std::make_shared<class Map::impl>(map, server))
{
}

std::int64_t Map::getId() const
{
    try {
        return impl->map->getId();
    }
    catch (...) {
        return 0;
    }
}

bool Map::isVirtual() const
{
    try {
        return impl->map->isVirtual();
    }
    catch (...) {
        return false;
    }
}

std::uint8_t Map::getScale() const
{
    try {
        return static_cast<std::uint8_t>(impl->map->getScale());
    }
    catch (...) {
        return 0;
    }
}

void Map::setScale(const std::uint8_t scale) const
{
    try {
        const auto map_scale = to_endstone_scale(scale);
        if (!map_scale) {
            return;
        }
        impl->map->setScale(*map_scale);
    }
    catch (...) {
    }
}

std::int32_t Map::getCenterX() const
{
    try {
        return impl->map->getCenterX();
    }
    catch (...) {
        return 0;
    }
}

void Map::setCenterX(const std::int32_t x) const
{
    try {
        impl->map->setCenterX(x);
    }
    catch (...) {
    }
}

std::int32_t Map::getCenterZ() const
{
    try {
        return impl->map->getCenterZ();
    }
    catch (...) {
        return 0;
    }
}

void Map::setCenterZ(const std::int32_t z) const
{
    try {
        impl->map->setCenterZ(z);
    }
    catch (...) {
    }
}

rust::String Map::getDimensionName() const
{
    try {
        const auto *dimension = impl->map->getDimension();
        return rust::String(dimension != nullptr ? dimension->getName() : "");
    }
    catch (...) {
        return rust::String();
    }
}

bool Map::isUnlimitedTracking() const
{
    try {
        return impl->map->isUnlimitedTracking();
    }
    catch (...) {
        return false;
    }
}

void Map::setUnlimitedTracking(const bool unlimited) const
{
    try {
        impl->map->setUnlimitedTracking(unlimited);
    }
    catch (...) {
    }
}

bool Map::isLocked() const
{
    try {
        return impl->map->isLocked();
    }
    catch (...) {
        return false;
    }
}

void Map::setLocked(const bool locked) const
{
    try {
        impl->map->setLocked(locked);
    }
    catch (...) {
    }
}

bool Map::setDimension(const rust::Str dimension) const
{
    try {
        const std::string_view dimension_value(dimension.data(), dimension.size());
        if (impl == nullptr || impl->map == nullptr || impl->server == nullptr) {
            return false;
        }
        auto *level = impl->server->getLevel();
        auto *target = level == nullptr ? nullptr : level->getDimension(std::string(dimension_value));
        if (target == nullptr) {
            return false;
        }
        impl->map->setDimension(*target);
        return true;
    }
    catch (...) {
        return false;
    }
}

endstone::MapView *Map::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->map;
}

} // namespace aegilex::native::ui
