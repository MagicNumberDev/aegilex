// Test-only typed ui bridge stubs. Never linked into the plugin.

#include <aegilex-runtime/src/cxx_host_ui.rs.h>

#include "aegilex_types.h"

#include "bindings/endstone/boss/boss_bar.h"
#include "bindings/endstone/actor/actor.h"
#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/map/map.h"
#include "bindings/endstone/scoreboard/objective.h"
#include "bindings/endstone/scoreboard/scoreboard.h"

#include <cstdint>
#include <utility>
#include <string>

namespace aegilex::native::ui {

class BossBar::impl {
  public:
    impl() noexcept = default;
};

class Scoreboard::impl {
  public:
    impl() noexcept = default;
};

class Objective::impl {
  public:
    impl() noexcept = default;
};

class ScoreEntry::impl {
  public:
    impl() noexcept = default;
};

class Map::impl {
  public:
    impl() noexcept = default;
};

BossBar::BossBar(std::shared_ptr<endstone::BossBar>) noexcept : impl(std::make_shared<class BossBar::impl>())
{
}
BossBar::~BossBar() noexcept = default;

Scoreboard::Scoreboard(endstone::Scoreboard *) noexcept : impl(std::make_shared<class Scoreboard::impl>())
{
}

Scoreboard::Scoreboard(std::shared_ptr<endstone::Scoreboard>) noexcept
    : impl(std::make_shared<class Scoreboard::impl>())
{
}

Objective::Objective(std::shared_ptr<endstone::Objective>) noexcept : impl(std::make_shared<class Objective::impl>())
{
}

Map::Map(endstone::MapView *, endstone::Server *) noexcept : impl(std::make_shared<class Map::impl>())
{
}

std::unique_ptr<BossBar> BossBar::create(const ::aegilex::native::server::Server &, rust::Str, std::uint32_t,
                                         std::uint32_t, rust::Slice<const std::uint32_t>)
{
    return std::unique_ptr<BossBar>(new BossBar(nullptr));
}

rust::String BossBar::getTitle() const
{
    return rust::String("Aegilex");
}

void BossBar::setTitle(rust::Str) const
{
}

std::uint32_t BossBar::getColor() const
{
    return 0;
}

void BossBar::setColor(std::uint32_t) const
{
}

std::uint32_t BossBar::getStyle() const
{
    return 0;
}

void BossBar::setStyle(std::uint32_t) const
{
}

bool BossBar::hasFlag(std::uint32_t) const
{
    return true;
}

void BossBar::addFlag(std::uint32_t) const
{
}

void BossBar::removeFlag(std::uint32_t) const
{
}

float BossBar::getProgress() const
{
    return 0.5F;
}

void BossBar::setProgress(float) const
{
}

bool BossBar::isVisible() const
{
    return true;
}

void BossBar::setVisible(bool) const
{
}

void BossBar::addPlayer(const ::aegilex::native::player::Player &) const
{
}

void BossBar::removePlayer(const ::aegilex::native::player::Player &) const
{
}

void BossBar::removeAllPlayers() const
{
}

std::unique_ptr<PlayerCollection> BossBar::getPlayers() const
{
    return std::unique_ptr<PlayerCollection>(new PlayerCollection({}));
}

PlayerCollection::PlayerCollection(std::vector<std::unique_ptr<::aegilex::native::player::Player>> players) noexcept
    : players_(std::move(players))
{
}

std::size_t PlayerCollection::len() const noexcept
{
    return players_.size();
}

std::unique_ptr<::aegilex::native::player::Player> PlayerCollection::takePlayer(std::size_t index) noexcept
{
    return index < players_.size() ? std::move(players_[index]) : std::unique_ptr<::aegilex::native::player::Player>();
}

std::unique_ptr<Objective> Scoreboard::getObjective(rust::Str) const
{
    return std::unique_ptr<Objective>(new Objective(nullptr));
}

std::unique_ptr<Objective> Scoreboard::createObjective(rust::Str, rust::Str, bool, std::uint8_t) const
{
    return std::unique_ptr<Objective>(new Objective(nullptr));
}

std::unique_ptr<Objective> Scoreboard::getObjectiveBySlot(std::uint32_t) const
{
    return std::unique_ptr<Objective>(new Objective(nullptr));
}

void Scoreboard::removeObjective(rust::Str) const
{
}

rust::Vec<rust::String> Scoreboard::listObjectives() const
{
    rust::Vec<rust::String> names;
    names.push_back(rust::String("dummy"));
    return names;
}

rust::Vec<rust::String> Scoreboard::listObjectivesByCriteria(std::uint32_t) const
{
    rust::Vec<rust::String> names;
    names.push_back(rust::String("dummy"));
    return names;
}

void Scoreboard::clearSlot(std::uint32_t) const
{
}

void Scoreboard::resetScores(const ScoreEntry &) const
{
}

ScoreSummaryList Scoreboard::getScores(const ScoreEntry &) const
{
    rust::Vec<ScoreSummary> scores;
    scores.push_back(ScoreSummary{.objective_name = rust::String("dummy"), .value = 7, .score_set = true});
    return ScoreSummaryList{.status = aegilex::kOk, .scores = std::move(scores)};
}

std::unique_ptr<ScoreEntry> ScoreEntry::fromText(rust::Str text) noexcept
{
    return std::unique_ptr<ScoreEntry>(new ScoreEntry(text));
}

std::unique_ptr<ScoreEntry> ScoreEntry::fromPlayer(const ::aegilex::native::player::Player &) noexcept
{
    return std::unique_ptr<ScoreEntry>(new ScoreEntry(static_cast<endstone::Player *>(nullptr)));
}

std::unique_ptr<ScoreEntry> ScoreEntry::fromActor(const ::aegilex::native::actor::Actor &) noexcept
{
    return std::unique_ptr<ScoreEntry>(new ScoreEntry(static_cast<endstone::Actor *>(nullptr)));
}

ScoreEntry::ScoreEntry(endstone::Player *player) noexcept : impl(std::make_shared<class ScoreEntry::impl>())
{
}

ScoreEntryCollection::ScoreEntryCollection(std::vector<Entry> entries) noexcept : entries_(std::move(entries))
{
}

std::size_t ScoreEntryCollection::len() const noexcept
{
    return entries_.size();
}

ScoreEntry::ScoreEntry(endstone::Actor *actor) noexcept : impl(std::make_shared<class ScoreEntry::impl>())
{
}

ScoreEntry::ScoreEntry(rust::Str text) noexcept : impl(std::make_shared<class ScoreEntry::impl>())
{
}

std::uint8_t ScoreEntry::kind() const noexcept
{
    return 2;
}

std::unique_ptr<::aegilex::native::player::Player> ScoreEntry::asPlayer() const
{
    return std::unique_ptr<::aegilex::native::player::Player>();
}

std::unique_ptr<::aegilex::native::actor::Actor> ScoreEntry::asActor() const
{
    return std::unique_ptr<::aegilex::native::actor::Actor>();
}

rust::String ScoreEntry::getText() const
{
    return rust::String("first");
}

std::unique_ptr<ScoreEntry> ScoreEntryCollection::take(std::size_t index) noexcept
{
    return index < entries_.size() ? std::move(entries_[index]) : std::unique_ptr<ScoreEntry>();
}

std::unique_ptr<ScoreEntryCollection> Scoreboard::listEntries() const
{
    std::vector<ScoreEntryCollection::Entry> entries;
    entries.emplace_back(std::make_unique<ScoreEntry>(rust::Str("first")));
    return std::unique_ptr<ScoreEntryCollection>(new ScoreEntryCollection(std::move(entries)));
}

rust::String Objective::getName() const
{
    return rust::String("dummy");
}

rust::String Objective::getDisplayName() const
{
    return rust::String("Dummy");
}

void Objective::setDisplayName(rust::Str) const
{
}

std::uint32_t Objective::getCriteria() const
{
    return 0;
}

bool Objective::isModifiable() const
{
    return true;
}

bool Objective::isDisplayed() const
{
    return true;
}

bool Objective::getDisplaySlot(std::uint32_t &out_slot) const
{
    out_slot = 2;
    return true;
}

void Objective::setDisplaySlot(bool, std::uint32_t) const
{
}

bool Objective::getSortOrder(std::uint32_t &out_order) const
{
    out_order = 0;
    return true;
}

void Objective::setSortOrder(std::uint32_t) const
{
}

void Objective::setDisplay(bool, std::uint32_t, std::uint32_t) const
{
}

std::uint8_t Objective::getRenderType() const
{
    return 0;
}

void Objective::unregister() const
{
}

ScoreValue Objective::getScoreValue(const ScoreEntry &) const
{
    return ScoreValue{.status = aegilex::kOk, .value = 7};
}

bool Objective::setScoreValue(const ScoreEntry &, std::int32_t) const
{
    return true;
}

std::int64_t Map::getId() const
{
    return 7;
}

bool Map::isVirtual() const
{
    return false;
}

std::uint8_t Map::getScale() const
{
    return 2;
}

void Map::setScale(std::uint8_t) const
{
}

std::int32_t Map::getCenterX() const
{
    return 10;
}

void Map::setCenterX(std::int32_t) const
{
}

std::int32_t Map::getCenterZ() const
{
    return 20;
}

void Map::setCenterZ(std::int32_t) const
{
}

rust::String Map::getDimensionName() const
{
    return rust::String("overworld");
}

bool Map::isUnlimitedTracking() const
{
    return false;
}

void Map::setUnlimitedTracking(bool) const
{
}

bool Map::isLocked() const
{
    return false;
}

void Map::setLocked(bool) const
{
}

bool Map::setDimension(rust::Str) const
{
    return true;
}

} // namespace aegilex::native::ui
