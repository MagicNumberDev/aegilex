#include "scoreboard.h"
#include "objective.h"

#include "bindings/endstone/actor/actor.h"
#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/server.h"

#include <aegilex-runtime/src/cxx_host_ui.rs.h>

#include <endstone/scoreboard/criteria.h>
#include <endstone/scoreboard/display_slot.h>
#include <endstone/scoreboard/render_type.h>
#include <endstone/scoreboard/objective.h>
#include <endstone/scoreboard/score.h>
#include <endstone/scoreboard/score_entry.h>
#include <endstone/scoreboard/scoreboard.h>
#include <endstone/server.h>

#include <optional>
#include <string>
#include <string_view>
#include <vector>

namespace aegilex::native::ui {

class Scoreboard::impl {
  public:
    explicit impl(endstone::Scoreboard *scoreboard) noexcept : scoreboard(scoreboard)
    {
    }

    explicit impl(std::shared_ptr<endstone::Scoreboard> scoreboard) noexcept
        : owner(std::move(scoreboard)), scoreboard(owner.get())
    {
    }

    std::shared_ptr<endstone::Scoreboard> owner;
    endstone::Scoreboard *scoreboard;
};

namespace {

[[nodiscard]] std::optional<endstone::DisplaySlot> to_endstone_slot(const std::uint32_t slot) noexcept
{
    switch (slot) {
    case 0:
        return endstone::DisplaySlot::BelowName;
    case 1:
        return endstone::DisplaySlot::PlayerList;
    case 2:
        return endstone::DisplaySlot::SideBar;
    }
    return std::nullopt;
}

} // namespace

Scoreboard::Scoreboard(endstone::Scoreboard *scoreboard) noexcept
    : impl(std::make_shared<class Scoreboard::impl>(scoreboard))
{
}

Scoreboard::Scoreboard(std::shared_ptr<endstone::Scoreboard> scoreboard) noexcept
    : impl(std::make_shared<class Scoreboard::impl>(std::move(scoreboard)))
{
}

std::unique_ptr<Objective> Scoreboard::getObjective(const rust::Str name) const
{
    try {
        const std::string_view value(name.data(), name.size());
        auto objective = impl->scoreboard->getObjective(std::string(value));
        return objective == nullptr ? std::unique_ptr<Objective>() : std::make_unique<Objective>(std::move(objective));
    }
    catch (...) {
        return std::unique_ptr<Objective>();
    }
}

std::unique_ptr<Objective> Scoreboard::createObjective(const rust::Str name, const rust::Str display_name,
                                                       const bool has_render_type, const std::uint8_t render_type) const
{
    try {
        const std::string_view name_value(name.data(), name.size());
        const std::string_view display_value(display_name.data(), display_name.size());
        if (has_render_type) {
            auto objective = impl->scoreboard->addObjective(std::string(name_value), endstone::Criteria::Type::Dummy,
                                                            std::string(display_value),
                                                            static_cast<endstone::RenderType>(render_type));
            return objective == nullptr ? std::unique_ptr<Objective>()
                                        : std::make_unique<Objective>(std::move(objective));
        }
        auto objective = impl->scoreboard->addObjective(std::string(name_value), endstone::Criteria::Type::Dummy,
                                                        std::string(display_value));
        return objective == nullptr ? std::unique_ptr<Objective>() : std::make_unique<Objective>(std::move(objective));
    }
    catch (...) {
        return std::unique_ptr<Objective>();
    }
}

std::unique_ptr<Objective> Scoreboard::getObjectiveBySlot(const std::uint32_t slot) const
{
    try {
        const auto display_slot = to_endstone_slot(slot);
        if (!display_slot) {
            return std::unique_ptr<Objective>();
        }
        auto objective = impl->scoreboard->getObjective(*display_slot);
        return objective == nullptr ? std::unique_ptr<Objective>() : std::make_unique<Objective>(std::move(objective));
    }
    catch (...) {
        return std::unique_ptr<Objective>();
    }
}

void Scoreboard::removeObjective(const rust::Str name) const
{
    try {
        const auto objective = getObjective(name);
        if (objective != nullptr) {
            objective->unregister();
        }
    }
    catch (...) {
    }
}

rust::Vec<rust::String> Scoreboard::listObjectives() const
{
    rust::Vec<rust::String> names;
    try {
        for (const auto &objective : impl->scoreboard->getObjectives()) {
            if (objective != nullptr) {
                names.push_back(rust::String(objective->getName()));
            }
        }
    }
    catch (...) {
    }
    return names;
}

rust::Vec<rust::String> Scoreboard::listObjectivesByCriteria(const std::uint32_t criteria) const
{
    rust::Vec<rust::String> names;
    try {
        if (criteria != 0) {
            return names;
        }
        for (const auto &objective : impl->scoreboard->getObjectivesByCriteria(endstone::Criteria::Type::Dummy)) {
            if (objective != nullptr) {
                names.push_back(rust::String(objective->getName()));
            }
        }
    }
    catch (...) {
    }
    return names;
}

void Scoreboard::clearSlot(const std::uint32_t slot) const
{
    try {
        const auto display_slot = to_endstone_slot(slot);
        if (!display_slot) {
            return;
        }
        impl->scoreboard->clearSlot(*display_slot);
    }
    catch (...) {
    }
}

endstone::Scoreboard *Scoreboard::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->scoreboard;
}

void Scoreboard::resetScores(const ScoreEntry &entry) const
{
    try {
        if (native() != nullptr && entry.native() != nullptr) {
            native()->resetScores(*static_cast<const endstone::ScoreEntry *>(entry.native()));
        }
    }
    catch (...) {
    }
}

namespace {

[[nodiscard]] ScoreSummaryList summaries_for(const endstone::Scoreboard &scoreboard, const endstone::ScoreEntry &entry)
{
    rust::Vec<ScoreSummary> summaries;
    for (const auto &score : scoreboard.getScores(entry)) {
        if (score != nullptr) {
            summaries.push_back(ScoreSummary{.objective_name = rust::String(score->getObjective().getName()),
                                             .value = score->getValue(),
                                             .score_set = score->isScoreSet()});
        }
    }
    return ScoreSummaryList{.status = 0, .scores = std::move(summaries)};
}

} // namespace

ScoreSummaryList Scoreboard::getScores(const ScoreEntry &entry) const
{
    try {
        if (native() == nullptr || entry.native() == nullptr) {
            return ScoreSummaryList{.status = 1, .scores = {}};
        }
        return summaries_for(*native(), *static_cast<const endstone::ScoreEntry *>(entry.native()));
    }
    catch (...) {
        return ScoreSummaryList{.status = 4, .scores = {}};
    }
}

class ScoreEntry::impl {
  public:
    explicit impl(endstone::ScoreEntry entry) noexcept : entry(std::move(entry))
    {
    }

    endstone::ScoreEntry entry;
};

ScoreEntry::ScoreEntry(endstone::Player *player) noexcept
    : impl(std::make_shared<class ScoreEntry::impl>(endstone::ScoreEntry{player}))
{
}

ScoreEntry::ScoreEntry(endstone::Actor *actor) noexcept
    : impl(std::make_shared<class ScoreEntry::impl>(endstone::ScoreEntry{actor}))
{
}

ScoreEntry::ScoreEntry(const rust::Str text) noexcept
    : impl(std::make_shared<class ScoreEntry::impl>(endstone::ScoreEntry{std::string(text)}))
{
}

std::unique_ptr<ScoreEntry> ScoreEntry::fromText(const rust::Str text) noexcept
{
    try {
        return std::make_unique<ScoreEntry>(text);
    }
    catch (...) {
        return std::unique_ptr<ScoreEntry>();
    }
}

std::unique_ptr<ScoreEntry> ScoreEntry::fromPlayer(const ::aegilex::native::player::Player &player) noexcept
{
    try {
        return player.native() == nullptr ? std::unique_ptr<ScoreEntry>()
                                          : std::make_unique<ScoreEntry>(player.native());
    }
    catch (...) {
        return std::unique_ptr<ScoreEntry>();
    }
}

std::unique_ptr<ScoreEntry> ScoreEntry::fromActor(const ::aegilex::native::actor::Actor &actor) noexcept
{
    try {
        return actor.native() == nullptr ? std::unique_ptr<ScoreEntry>() : std::make_unique<ScoreEntry>(actor.native());
    }
    catch (...) {
        return std::unique_ptr<ScoreEntry>();
    }
}

const void *ScoreEntry::native() const noexcept
{
    return impl == nullptr ? nullptr : &impl->entry;
}

std::uint8_t ScoreEntry::kind() const noexcept
{
    const auto *entry = static_cast<const endstone::ScoreEntry *>(native());
    return entry == nullptr ? 0 : static_cast<std::uint8_t>(entry->index() + 1);
}

std::unique_ptr<::aegilex::native::player::Player> ScoreEntry::asPlayer() const
{
    try {
        const auto *entry = static_cast<const endstone::ScoreEntry *>(native());
        const auto *player = entry == nullptr ? nullptr : std::get_if<endstone::Player *>(entry);
        return player == nullptr || *player == nullptr ? std::unique_ptr<::aegilex::native::player::Player>()
                                                       : std::make_unique<::aegilex::native::player::Player>(*player);
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::player::Player>();
    }
}

std::unique_ptr<::aegilex::native::actor::Actor> ScoreEntry::asActor() const
{
    try {
        const auto *entry = static_cast<const endstone::ScoreEntry *>(native());
        const auto *actor = entry == nullptr ? nullptr : std::get_if<endstone::Actor *>(entry);
        return actor == nullptr || *actor == nullptr ? std::unique_ptr<::aegilex::native::actor::Actor>()
                                                     : std::make_unique<::aegilex::native::actor::Actor>(*actor);
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::actor::Actor>();
    }
}

rust::String ScoreEntry::getText() const
{
    try {
        const auto *entry = static_cast<const endstone::ScoreEntry *>(native());
        const auto *text = entry == nullptr ? nullptr : std::get_if<std::string>(entry);
        return text == nullptr ? rust::String() : rust::String(*text);
    }
    catch (...) {
        return rust::String();
    }
}

ScoreEntryCollection::ScoreEntryCollection(std::vector<Entry> entries) noexcept : entries_(std::move(entries))
{
}

std::size_t ScoreEntryCollection::len() const noexcept
{
    return entries_.size();
}

std::unique_ptr<ScoreEntry> ScoreEntryCollection::take(const std::size_t index) noexcept
{
    return index < entries_.size() ? std::move(entries_[index]) : std::unique_ptr<ScoreEntry>();
}

std::unique_ptr<ScoreEntryCollection> Scoreboard::listEntries() const
{
    try {
        if (native() == nullptr) {
            return std::unique_ptr<ScoreEntryCollection>();
        }
        std::vector<ScoreEntryCollection::Entry> rows;
        for (const auto &entry : native()->getEntries()) {
            if (const auto *player = std::get_if<endstone::Player *>(&entry); player != nullptr && *player != nullptr) {
                rows.emplace_back(std::make_unique<ScoreEntry>(*player));
            }
            else if (const auto *actor = std::get_if<endstone::Actor *>(&entry);
                     actor != nullptr && *actor != nullptr) {
                rows.emplace_back(std::make_unique<ScoreEntry>(*actor));
            }
            else if (const auto *text = std::get_if<std::string>(&entry)) {
                rows.emplace_back(std::make_unique<ScoreEntry>(rust::Str(*text)));
            }
        }
        return std::make_unique<ScoreEntryCollection>(std::move(rows));
    }
    catch (...) {
        return std::unique_ptr<ScoreEntryCollection>();
    }
}

} // namespace aegilex::native::ui
