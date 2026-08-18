#pragma once

#include "rust/cxx.h"

#include <cstddef>
#include <cstdint>
#include <memory>

namespace endstone {
class Objective;
} // namespace endstone

namespace aegilex::native::actor {
class Actor;
}

namespace aegilex::native::player {
class Player;
}

namespace aegilex::native::ui {

class ScoreEntry;
struct ScoreValue;

// OOP/Pimpl facade over endstone::Objective. The impl owns the endstone
// wrapper returned by the scoreboard; the underlying objective stays
// registered on the scoreboard until unregister(). Mirrors the layout of
// endstone/scoreboard/objective.h; enum values are passed as raw u32.
class Objective {
  public:
    explicit Objective(std::shared_ptr<endstone::Objective> objective) noexcept;
    ~Objective() noexcept = default;

    Objective(const Objective &) = delete;
    Objective &operator=(const Objective &) = delete;

    rust::String getName() const;
    rust::String getDisplayName() const;
    void setDisplayName(rust::Str display_name) const;
    std::uint32_t getCriteria() const;
    bool isModifiable() const;
    bool isDisplayed() const;
    // Returns false when the objective has no display slot.
    [[nodiscard]] bool getDisplaySlot(std::uint32_t &out_slot) const;
    void setDisplaySlot(bool has_slot, std::uint32_t slot) const;
    // Returns false when the objective has no sort order.
    [[nodiscard]] bool getSortOrder(std::uint32_t &out_order) const;
    void setSortOrder(std::uint32_t order) const;
    void setDisplay(bool has_slot, std::uint32_t slot, std::uint32_t order) const;
    std::uint8_t getRenderType() const;
    void unregister() const;
    [[nodiscard]] ScoreValue getScoreValue(const ScoreEntry &entry) const;
    [[nodiscard]] bool setScoreValue(const ScoreEntry &entry, std::int32_t value) const;
    [[nodiscard]] endstone::Objective *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

} // namespace aegilex::native::ui
