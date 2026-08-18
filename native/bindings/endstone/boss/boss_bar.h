#pragma once

#include "rust/cxx.h"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <vector>

namespace endstone {
class BossBar;
} // namespace endstone

namespace aegilex::native::player {
class Player;
}

namespace aegilex::native::server {
class Server;
}

namespace aegilex::native::ui {

class PlayerCollection;

// OOP/Pimpl facade over endstone::BossBar. The facade owns the bar so Rust can
// retain it behind an invocation-scoped guest handle.
class BossBar {
  public:
    explicit BossBar(std::shared_ptr<endstone::BossBar> bar) noexcept;
    ~BossBar() noexcept;

    BossBar(const BossBar &) = delete;
    BossBar &operator=(const BossBar &) = delete;

    rust::String getTitle() const;
    void setTitle(rust::Str title) const;
    std::uint32_t getColor() const;
    void setColor(std::uint32_t color) const;
    std::uint32_t getStyle() const;
    void setStyle(std::uint32_t style) const;
    bool hasFlag(std::uint32_t flag) const;
    void addFlag(std::uint32_t flag) const;
    void removeFlag(std::uint32_t flag) const;
    float getProgress() const;
    void setProgress(float progress) const;
    bool isVisible() const;
    void setVisible(bool visible) const;
    void addPlayer(const ::aegilex::native::player::Player &player) const;
    void removePlayer(const ::aegilex::native::player::Player &player) const;
    void removeAllPlayers() const;
    [[nodiscard]] std::unique_ptr<PlayerCollection> getPlayers() const;
    [[nodiscard]] static std::unique_ptr<BossBar> create(const ::aegilex::native::server::Server &server,
                                                         rust::Str title, std::uint32_t color, std::uint32_t style,
                                                         rust::Slice<const std::uint32_t> flags);
    [[nodiscard]] endstone::BossBar *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

class PlayerCollection {
  public:
    explicit PlayerCollection(std::vector<std::unique_ptr<::aegilex::native::player::Player>> players) noexcept;
    ~PlayerCollection() noexcept = default;

    PlayerCollection(const PlayerCollection &) = delete;
    PlayerCollection &operator=(const PlayerCollection &) = delete;

    [[nodiscard]] std::size_t len() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> takePlayer(std::size_t index) noexcept;

  private:
    std::vector<std::unique_ptr<::aegilex::native::player::Player>> players_;
};

} // namespace aegilex::native::ui
