#include "map_renderer_bridge.h"

#include "../host_context.h"
#include "../runtime_bridge.h"

#include <aegilex-runtime/src/cxx_runtime.rs.h>

#include <endstone/map/map_canvas.h>
#include <endstone/map/map_cursor.h>
#include <endstone/map/map_renderer.h>
#include <endstone/map/map_view.h>
#include <endstone/player.h>
#include <endstone/server.h>
#include <endstone/util/image.h>

#include <cstring>
#include <exception>
#include <utility>
#include <vector>

namespace aegilex::native {

namespace {

[[nodiscard]] endstone::MapView *resolve_map(endstone::Server &server, const std::int64_t map_id) noexcept
{
    try {
        return server.getMap(map_id);
    }
    catch (...) {
        return nullptr;
    }
}

void apply_commands(endstone::MapCanvas &canvas,
                    const rust::Vec<aegilex::runtime::MapDrawCommandData> &commands) noexcept
{
    for (const auto &command : commands) {
        switch (command.kind) {
        case 0: // set-pixel
            canvas.setPixel(command.x, command.y, command.argb);
            break;
        case 1: // fill-rect
            for (std::uint8_t y = command.y; y < command.y + command.height; ++y) {
                for (std::uint8_t x = command.x; x < command.x + command.width; ++x) {
                    canvas.setPixel(x, y, command.argb);
                }
            }
            break;
        case 2: { // draw-image: ARGB u32 pixels -> RGBA byte rows
            std::vector<std::uint8_t> rgba;
            rgba.reserve(static_cast<std::size_t>(command.width) * command.height * 4);
            for (const auto pixel : command.pixels) {
                rgba.push_back(static_cast<std::uint8_t>((pixel >> 16) & 0xFF));
                rgba.push_back(static_cast<std::uint8_t>((pixel >> 8) & 0xFF));
                rgba.push_back(static_cast<std::uint8_t>(pixel & 0xFF));
                rgba.push_back(static_cast<std::uint8_t>((pixel >> 24) & 0xFF));
            }
            canvas.drawImage(command.x, command.y,
                             endstone::Image{endstone::Image::Type::RGBA, command.width, command.height, rgba});
            break;
        }
        case 3: { // set-cursors
            std::vector<endstone::MapCursor> cursors;
            cursors.reserve(command.cursors.size());
            for (const auto &cursor : command.cursors) {
                cursors.emplace_back(cursor.x, cursor.y, static_cast<std::int8_t>(cursor.direction),
                                     static_cast<endstone::MapCursor::Type>(cursor.cursor_type), cursor.visible,
                                     std::string(cursor.caption.data(), cursor.caption.size()));
            }
            canvas.setCursors(cursors);
            break;
        }
        default:
            break;
        }
    }
}

} // namespace

struct MapRendererBridge::Renderer : endstone::MapRenderer {
    Renderer(const bool contextual, MapRendererBridge *bridge, std::string owner, const std::uint64_t renderer_id,
             const std::int64_t map_id)
        : endstone::MapRenderer(contextual), bridge(bridge), owner(std::move(owner)), renderer_id(renderer_id),
          map_id(map_id)
    {
    }

    void initialize(endstone::MapView &) override
    {
    }

    void render(endstone::MapView &, endstone::MapCanvas &canvas, endstone::Player &player) override
    {
        if (bridge != nullptr) {
            if (!initialized) {
                if (!bridge->dispatch_initialize(owner, renderer_id, map_id)) {
                    return;
                }
                initialized = true;
            }
            bridge->dispatch_render(owner, renderer_id, map_id, &player, canvas);
        }
    }

    MapRendererBridge *bridge;
    std::string owner;
    std::uint64_t renderer_id;
    std::int64_t map_id;
    bool initialized{};
};

MapRendererBridge::MapRendererBridge(HostContext &context, Runtime *runtime) : context_(context), runtime_(runtime)
{
}

aegilex::status MapRendererBridge::register_renderer(const std::string_view plugin_id, const std::int64_t map_id,
                                                     const bool contextual, std::uint64_t *out_renderer_id) noexcept
{
    try {
        if (plugin_id.empty() || out_renderer_id == nullptr || context_.server.native() == nullptr) {
            return aegilex::kInvalidArgument;
        }
        if (!context_.server.native()->isPrimaryThread()) {
            return aegilex::kWrongThread;
        }
        auto *map = resolve_map(*context_.server.native(), map_id);
        if (map == nullptr) {
            return aegilex::kNotFound;
        }
        const auto renderer_id = next_renderer_id_++;
        if (renderer_id == 0) {
            return aegilex::kLimitExceeded;
        }
        auto renderer = std::make_shared<Renderer>(contextual, this, std::string(plugin_id), renderer_id, map_id);
        map->addRenderer(renderer);
        renderers_.emplace(renderer_id, std::move(renderer));
        *out_renderer_id = renderer_id;
        return aegilex::kOk;
    }
    catch (const std::exception &error) {
        if (context_.logger != nullptr) {
            context_.logger->error("Aegilex could not register map renderer for map {}: {}", map_id, error.what());
        }
        return aegilex::kHostError;
    }
    catch (...) {
        if (context_.logger != nullptr) {
            context_.logger->error("Aegilex could not register map renderer for map {}: unknown exception", map_id);
        }
        return aegilex::kHostError;
    }
}

aegilex::status MapRendererBridge::unregister_renderer(const std::string_view plugin_id,
                                                       const std::uint64_t renderer_id) noexcept
{
    try {
        const auto it = renderers_.find(renderer_id);
        if (it == renderers_.end()) {
            return aegilex::kNotFound;
        }
        if (it->second->owner != plugin_id) {
            return aegilex::kDenied;
        }
        const auto map_id = it->second->map_id;
        auto renderer = it->second;
        renderers_.erase(it);
        renderer->bridge = nullptr;
        if (context_.server.native() != nullptr) {
            if (auto *map = resolve_map(*context_.server.native(), map_id); map != nullptr) {
                map->removeRenderer(renderer);
            }
        }
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

void MapRendererBridge::clear_all() noexcept
{
    try {
        auto renderers = std::move(renderers_);
        renderers_.clear();
        for (auto &[renderer_id, renderer] : renderers) {
            static_cast<void>(renderer_id);
            renderer->bridge = nullptr;
            if (context_.server.native() != nullptr) {
                if (auto *map = resolve_map(*context_.server.native(), renderer->map_id); map != nullptr) {
                    map->removeRenderer(renderer);
                }
            }
        }
    }
    catch (...) {
    }
}

bool MapRendererBridge::can_dispatch_callback() const noexcept
{
    return runtime_ != nullptr && context_.server.native() != nullptr && context_.accepting_calls &&
           context_.server.native()->isPrimaryThread();
}

bool MapRendererBridge::dispatch_initialize(const std::string_view plugin_id, const std::uint64_t renderer_id,
                                            const std::int64_t map_id) noexcept
{
    try {
        if (!can_dispatch_callback()) {
            return false;
        }
        static_cast<void>(
            aegilex::runtime::dispatch_map_initialize(*runtime_->handle, std::string(plugin_id), renderer_id, map_id));
        return true;
    }
    catch (...) {
        return false;
    }
}

void MapRendererBridge::dispatch_render(const std::string_view plugin_id, const std::uint64_t renderer_id,
                                        const std::int64_t map_id, endstone::Player *player,
                                        endstone::MapCanvas &canvas) noexcept
{
    try {
        if (!can_dispatch_callback()) {
            return;
        }
        std::array<std::uint8_t, 16> player_uuid{};
        const auto has_player = player != nullptr;
        if (has_player) {
            const auto uuid = player->getUniqueId();
            std::memcpy(player_uuid.data(), uuid.data, uuid.size());
        }
        const auto commands = aegilex::runtime::dispatch_map_render(
            *runtime_->handle, std::string(plugin_id), renderer_id, map_id, has_player,
            rust::Slice<const std::uint8_t>(player_uuid.data(), player_uuid.size()));
        apply_commands(canvas, commands);
    }
    catch (...) {
    }
}

} // namespace aegilex::native
