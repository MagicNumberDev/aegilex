#include "objective.h"

#include "bindings/endstone/actor/actor.h"
#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/scoreboard/scoreboard.h"

#include <aegilex-runtime/src/cxx_host_ui.rs.h>

#include <endstone/scoreboard/criteria.h>
#include <endstone/scoreboard/display_slot.h>
#include <endstone/scoreboard/objective.h>
#include <endstone/scoreboard/score.h>
#include <endstone/scoreboard/score_entry.h>

#include <optional>
#include <string>
#include <string_view>
#include <utility>

namespace aegilex::native::ui {

class Objective::impl {
  public:
    explicit impl(std::shared_ptr<endstone::Objective> objective) noexcept : objective(std::move(objective))
    {
    }

    std::shared_ptr<endstone::Objective> objective;
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

[[nodiscard]] ScoreValue score_value(const endstone::Objective &objective, const endstone::ScoreEntry &entry) noexcept
{
    try {
        const auto score = objective.getScore(entry);
        return score == nullptr ? ScoreValue{.status = 1, .value = 0}
                                : ScoreValue{.status = 0, .value = score->getValue()};
    }
    catch (...) {
        return ScoreValue{.status = 4, .value = 0};
    }
}

[[nodiscard]] bool set_score_value(const endstone::Objective &objective, const endstone::ScoreEntry &entry,
                                   const std::int32_t value) noexcept
{
    try {
        const auto score = objective.getScore(entry);
        if (score == nullptr) {
            return false;
        }
        score->setValue(value);
        return true;
    }
    catch (...) {
        return false;
    }
}

} // namespace

Objective::Objective(std::shared_ptr<endstone::Objective> objective) noexcept
    : impl(std::make_shared<class Objective::impl>(std::move(objective)))
{
}

endstone::Objective *Objective::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->objective.get();
}

rust::String Objective::getName() const
{
    try {
        return native() == nullptr ? rust::String() : rust::String(native()->getName());
    }
    catch (...) {
        return rust::String();
    }
}

rust::String Objective::getDisplayName() const
{
    try {
        return native() == nullptr ? rust::String() : rust::String(native()->getDisplayName());
    }
    catch (...) {
        return rust::String();
    }
}

void Objective::setDisplayName(const rust::Str display_name) const
{
    try {
        const std::string_view value(display_name.data(), display_name.size());
        if (native() != nullptr) {
            native()->setDisplayName(std::string(value));
        }
    }
    catch (...) {
    }
}

std::uint32_t Objective::getCriteria() const
{
    try {
        return native() == nullptr || native()->getCriteria().getName() != "dummy" ? 1 : 0;
    }
    catch (...) {
        return 1;
    }
}

bool Objective::isModifiable() const
{
    try {
        return native() != nullptr && native()->isModifiable();
    }
    catch (...) {
        return false;
    }
}

bool Objective::isDisplayed() const
{
    try {
        return native() != nullptr && native()->isDisplayed();
    }
    catch (...) {
        return false;
    }
}

bool Objective::getDisplaySlot(std::uint32_t &out_slot) const
{
    try {
        if (native() == nullptr) {
            return false;
        }
        const auto slot = native()->getDisplaySlot();
        if (!slot) {
            return false;
        }
        out_slot = static_cast<std::uint32_t>(*slot);
        return true;
    }
    catch (...) {
        return false;
    }
}

void Objective::setDisplaySlot(const bool has_slot, const std::uint32_t slot) const
{
    try {
        if (native() == nullptr) {
            return;
        }
        if (!has_slot) {
            native()->setDisplaySlot(std::nullopt);
            return;
        }
        const auto display_slot = to_endstone_slot(slot);
        if (display_slot) {
            native()->setDisplaySlot(*display_slot);
        }
    }
    catch (...) {
    }
}

void Objective::setDisplay(const bool has_slot, const std::uint32_t slot, const std::uint32_t order) const
{
    try {
        if (native() == nullptr) {
            return;
        }
        if (!has_slot) {
            native()->setDisplaySlot(std::nullopt);
            return;
        }
        const auto display_slot = to_endstone_slot(slot);
        if (!display_slot) {
            return;
        }
        native()->setDisplay(*display_slot, static_cast<endstone::ObjectiveSortOrder>(order));
    }
    catch (...) {
    }
}

bool Objective::getSortOrder(std::uint32_t &out_order) const
{
    try {
        if (native() == nullptr) {
            return false;
        }
        const auto order = native()->getSortOrder();
        if (!order) {
            return false;
        }
        out_order = static_cast<std::uint32_t>(*order);
        return true;
    }
    catch (...) {
        return false;
    }
}

void Objective::setSortOrder(const std::uint32_t order) const
{
    try {
        if (native() == nullptr) {
            return;
        }
        if (order == 0) {
            native()->setSortOrder(endstone::ObjectiveSortOrder::Ascending);
        }
        else if (order == 1) {
            native()->setSortOrder(endstone::ObjectiveSortOrder::Descending);
        }
    }
    catch (...) {
    }
}

std::uint8_t Objective::getRenderType() const
{
    try {
        return native() == nullptr ? 0 : static_cast<std::uint8_t>(native()->getRenderType());
    }
    catch (...) {
        return 0;
    }
}

void Objective::unregister() const
{
    try {
        if (native() != nullptr) {
            native()->unregister();
        }
    }
    catch (...) {
    }
}

ScoreValue Objective::getScoreValue(const ScoreEntry &entry) const
{
    return native() == nullptr || entry.native() == nullptr
               ? ScoreValue{.status = 1, .value = 0}
               : score_value(*native(), *static_cast<const endstone::ScoreEntry *>(entry.native()));
}

bool Objective::setScoreValue(const ScoreEntry &entry, const std::int32_t value) const
{
    return native() != nullptr && entry.native() != nullptr &&
           set_score_value(*native(), *static_cast<const endstone::ScoreEntry *>(entry.native()), value);
}

} // namespace aegilex::native::ui
