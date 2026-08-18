#include "boss_bar.h"

#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/server.h"

#include <endstone/boss/boss_bar.h>
#include <endstone/player.h>
#include <endstone/server.h>

#include <cmath>
#include <optional>
#include <string>
#include <string_view>
#include <utility>

namespace aegilex::native::ui {

class BossBar::impl {
  public:
    explicit impl(std::shared_ptr<endstone::BossBar> bar) noexcept : bar(std::move(bar))
    {
    }

    std::shared_ptr<endstone::BossBar> bar;
};

namespace {

[[nodiscard]] std::optional<endstone::BarColor> to_endstone_color(const std::uint32_t color) noexcept
{
    switch (color) {
    case 0:
        return endstone::BarColor::Pink;
    case 1:
        return endstone::BarColor::Blue;
    case 2:
        return endstone::BarColor::Red;
    case 3:
        return endstone::BarColor::Green;
    case 4:
        return endstone::BarColor::Yellow;
    case 5:
        return endstone::BarColor::Purple;
    case 6:
        return endstone::BarColor::RebeccaPurple;
    case 7:
        return endstone::BarColor::White;
    }
    return std::nullopt;
}

[[nodiscard]] std::optional<endstone::BarStyle> to_endstone_style(const std::uint32_t style) noexcept
{
    switch (style) {
    case 0:
        return endstone::BarStyle::Solid;
    case 1:
        return endstone::BarStyle::Segmented6;
    case 2:
        return endstone::BarStyle::Segmented10;
    case 3:
        return endstone::BarStyle::Segmented12;
    case 4:
        return endstone::BarStyle::Segmented20;
    }
    return std::nullopt;
}

[[nodiscard]] std::optional<endstone::BarFlag> to_endstone_flag(const std::uint32_t flag) noexcept
{
    switch (flag) {
    case 0:
        return endstone::BarFlag::DarkenSky;
    case 1:
        return endstone::BarFlag::CreateFog;
    }
    return std::nullopt;
}

} // namespace

BossBar::BossBar(std::shared_ptr<endstone::BossBar> bar) noexcept
    : impl(std::make_shared<class BossBar::impl>(std::move(bar)))
{
}

BossBar::~BossBar() noexcept
{
    removeAllPlayers();
}

endstone::BossBar *BossBar::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->bar.get();
}

rust::String BossBar::getTitle() const
{
    try {
        return native() == nullptr ? rust::String() : rust::String(native()->getTitle());
    }
    catch (...) {
        return rust::String();
    }
}

void BossBar::setTitle(const rust::Str title) const
{
    try {
        const std::string_view value(title.data(), title.size());
        if (native() == nullptr) {
            return;
        }
        native()->setTitle(std::string(value));
    }
    catch (...) {
    }
}

std::uint32_t BossBar::getColor() const
{
    try {
        return native() == nullptr ? 0 : static_cast<std::uint32_t>(native()->getColor());
    }
    catch (...) {
        return 0;
    }
}

void BossBar::setColor(const std::uint32_t color) const
{
    try {
        const auto bar_color = to_endstone_color(color);
        if (native() != nullptr && bar_color) {
            native()->setColor(*bar_color);
        }
    }
    catch (...) {
    }
}

std::uint32_t BossBar::getStyle() const
{
    try {
        return native() == nullptr ? 0 : static_cast<std::uint32_t>(native()->getStyle());
    }
    catch (...) {
        return 0;
    }
}

void BossBar::setStyle(const std::uint32_t style) const
{
    try {
        const auto bar_style = to_endstone_style(style);
        if (native() != nullptr && bar_style) {
            native()->setStyle(*bar_style);
        }
    }
    catch (...) {
    }
}

bool BossBar::hasFlag(const std::uint32_t flag) const
{
    try {
        const auto bar_flag = to_endstone_flag(flag);
        return native() != nullptr && bar_flag && native()->hasFlag(*bar_flag);
    }
    catch (...) {
        return false;
    }
}

void BossBar::addFlag(const std::uint32_t flag) const
{
    try {
        const auto bar_flag = to_endstone_flag(flag);
        if (native() != nullptr && bar_flag) {
            native()->addFlag(*bar_flag);
        }
    }
    catch (...) {
    }
}

void BossBar::removeFlag(const std::uint32_t flag) const
{
    try {
        const auto bar_flag = to_endstone_flag(flag);
        if (native() != nullptr && bar_flag) {
            native()->removeFlag(*bar_flag);
        }
    }
    catch (...) {
    }
}

float BossBar::getProgress() const
{
    try {
        return native() == nullptr ? 0.0F : native()->getProgress();
    }
    catch (...) {
        return 0.0F;
    }
}

void BossBar::setProgress(const float progress) const
{
    try {
        if (native() != nullptr && std::isfinite(progress) && progress >= 0.0F && progress <= 1.0F) {
            native()->setProgress(progress);
        }
    }
    catch (...) {
    }
}

bool BossBar::isVisible() const
{
    try {
        return native() != nullptr && native()->isVisible();
    }
    catch (...) {
        return false;
    }
}

void BossBar::setVisible(const bool visible) const
{
    try {
        if (native() != nullptr) {
            native()->setVisible(visible);
        }
    }
    catch (...) {
    }
}

void BossBar::addPlayer(const ::aegilex::native::player::Player &player) const
{
    try {
        if (native() != nullptr && player.native() != nullptr) {
            native()->addPlayer(*player.native());
        }
    }
    catch (...) {
    }
}

void BossBar::removePlayer(const ::aegilex::native::player::Player &player) const
{
    try {
        if (native() != nullptr && player.native() != nullptr) {
            native()->removePlayer(*player.native());
        }
    }
    catch (...) {
    }
}

void BossBar::removeAllPlayers() const
{
    try {
        if (native() != nullptr) {
            native()->removeAll();
        }
    }
    catch (...) {
    }
}

std::unique_ptr<PlayerCollection> BossBar::getPlayers() const
{
    try {
        if (native() == nullptr) {
            return std::unique_ptr<PlayerCollection>();
        }
        std::vector<std::unique_ptr<::aegilex::native::player::Player>> players;
        for (auto *player : native()->getPlayers()) {
            if (player != nullptr) {
                players.push_back(std::make_unique<::aegilex::native::player::Player>(player));
            }
        }
        return std::make_unique<PlayerCollection>(std::move(players));
    }
    catch (...) {
        return std::unique_ptr<PlayerCollection>();
    }
}

PlayerCollection::PlayerCollection(std::vector<std::unique_ptr<::aegilex::native::player::Player>> players) noexcept
    : players_(std::move(players))
{
}

std::size_t PlayerCollection::len() const noexcept
{
    return players_.size();
}

std::unique_ptr<::aegilex::native::player::Player> PlayerCollection::takePlayer(const std::size_t index) noexcept
{
    if (index >= players_.size()) {
        return std::unique_ptr<::aegilex::native::player::Player>();
    }
    return std::move(players_[index]);
}

std::unique_ptr<BossBar> BossBar::create(const ::aegilex::native::server::Server &server, const rust::Str title,
                                         const std::uint32_t color, const std::uint32_t style,
                                         const rust::Slice<const std::uint32_t> flags)
{
    try {
        if (server.native() == nullptr) {
            return std::unique_ptr<BossBar>();
        }
        const std::string_view title_value(title.data(), title.size());
        const auto bar_color = to_endstone_color(color);
        const auto bar_style = to_endstone_style(style);
        if (!bar_color || !bar_style) {
            return std::unique_ptr<BossBar>();
        }
        std::vector<endstone::BarFlag> bar_flags;
        bar_flags.reserve(flags.size());
        for (const auto flag : flags) {
            const auto bar_flag = to_endstone_flag(flag);
            if (!bar_flag) {
                return std::unique_ptr<BossBar>();
            }
            bar_flags.push_back(*bar_flag);
        }
        auto bar =
            server.native()->createBossBar(std::string(title_value), *bar_color, *bar_style, std::move(bar_flags));
        return bar == nullptr ? std::unique_ptr<BossBar>()
                              : std::make_unique<BossBar>(std::shared_ptr<endstone::BossBar>(std::move(bar)));
    }
    catch (...) {
        return std::unique_ptr<BossBar>();
    }
}

} // namespace aegilex::native::ui
