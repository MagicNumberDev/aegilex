#pragma once

#include "../aegilex_types.h"

#include <cstdint>
#include <memory>
#include <string>
#include <string_view>
#include <unordered_map>

namespace endstone {
class MapCanvas;
class MapView;
class Player;
} // namespace endstone

namespace aegilex::native {

class HostContext;
class Runtime;

// Routes guest map renderers into Endstone. Each registered renderer is an
// endstone::MapRenderer subclass whose initialize/render callbacks dispatch
// back into the owning plugin's exported map-renderer-callbacks on the primary
// thread; returned draw commands are applied to the native canvas afterwards.
class MapRendererBridge {
  public:
    MapRendererBridge(HostContext &context, Runtime *runtime);
    ~MapRendererBridge() noexcept = default;

    MapRendererBridge(const MapRendererBridge &) = delete;
    MapRendererBridge &operator=(const MapRendererBridge &) = delete;

    [[nodiscard]] aegilex::status register_renderer(const std::string_view plugin_id, const std::int64_t map_id,
                                                    const bool contextual, std::uint64_t *out_renderer_id) noexcept;
    [[nodiscard]] aegilex::status unregister_renderer(const std::string_view plugin_id,
                                                      const std::uint64_t renderer_id) noexcept;
    // Removes every renderer from its map; called before the bridge dies.
    void clear_all() noexcept;

  private:
    friend class AegilexMapRenderer;
    [[nodiscard]] bool can_dispatch_callback() const noexcept;
    [[nodiscard]] bool dispatch_initialize(std::string_view plugin_id, std::uint64_t renderer_id,
                                           std::int64_t map_id) noexcept;
    void dispatch_render(std::string_view plugin_id, std::uint64_t renderer_id, std::int64_t map_id,
                         endstone::Player *player, endstone::MapCanvas &canvas) noexcept;

    HostContext &context_;
    Runtime *runtime_;
    struct Renderer;
    std::unordered_map<std::uint64_t, std::shared_ptr<Renderer>> renderers_;
    std::uint64_t next_renderer_id_{1};
};

} // namespace aegilex::native
