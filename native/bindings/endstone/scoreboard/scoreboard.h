#pragma once

#include "rust/cxx.h"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <variant>
#include <vector>

namespace endstone {
class Actor;
class Player;
class Scoreboard;
} // namespace endstone

namespace aegilex::native::actor {
class Actor;
}

namespace aegilex::native::player {
class Player;
}

namespace aegilex::native::server {
class Server;
}

namespace aegilex::native::ui {

class Objective; // defined in objective.h
class ScoreEntry;
class ScoreEntryCollection;

struct ScoreSummary;
struct ScoreSummaryList;

// OOP/Pimpl facade over endstone::Scoreboard. Created boards retain the shared
// Endstone owner; primary and player boards remain non-owning views.
class Scoreboard {
  public:
    explicit Scoreboard(endstone::Scoreboard *scoreboard) noexcept;
    explicit Scoreboard(std::shared_ptr<endstone::Scoreboard> scoreboard) noexcept;
    ~Scoreboard() noexcept = default;

    Scoreboard(const Scoreboard &) = delete;
    Scoreboard &operator=(const Scoreboard &) = delete;

    std::unique_ptr<Objective> getObjective(rust::Str name) const;
    std::unique_ptr<Objective> createObjective(rust::Str name, rust::Str display_name, bool has_render_type,
                                               std::uint8_t render_type) const;
    std::unique_ptr<Objective> getObjectiveBySlot(std::uint32_t slot) const;
    void removeObjective(rust::Str name) const;
    rust::Vec<rust::String> listObjectives() const;
    rust::Vec<rust::String> listObjectivesByCriteria(std::uint32_t criteria) const;
    void clearSlot(std::uint32_t slot) const;
    void resetScores(const ScoreEntry &entry) const;
    [[nodiscard]] ScoreSummaryList getScores(const ScoreEntry &entry) const;
    [[nodiscard]] std::unique_ptr<ScoreEntryCollection> listEntries() const;
    [[nodiscard]] endstone::Scoreboard *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

class ScoreEntry {
  public:
    explicit ScoreEntry(endstone::Player *player) noexcept;
    explicit ScoreEntry(endstone::Actor *actor) noexcept;
    explicit ScoreEntry(rust::Str text) noexcept;
    ~ScoreEntry() noexcept = default;

    ScoreEntry(const ScoreEntry &) = delete;
    ScoreEntry &operator=(const ScoreEntry &) = delete;

    [[nodiscard]] std::uint8_t kind() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> asPlayer() const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::actor::Actor> asActor() const;
    [[nodiscard]] rust::String getText() const;

    [[nodiscard]] static std::unique_ptr<ScoreEntry> fromText(rust::Str text) noexcept;
    [[nodiscard]] static std::unique_ptr<ScoreEntry>
    fromPlayer(const ::aegilex::native::player::Player &player) noexcept;
    [[nodiscard]] static std::unique_ptr<ScoreEntry> fromActor(const ::aegilex::native::actor::Actor &actor) noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;

    [[nodiscard]] const void *native() const noexcept;
    friend class Scoreboard;
    friend class Objective;
};

class ScoreEntryCollection {
  public:
    using Entry = std::unique_ptr<ScoreEntry>;

    explicit ScoreEntryCollection(std::vector<Entry> entries) noexcept;
    ~ScoreEntryCollection() noexcept = default;

    ScoreEntryCollection(const ScoreEntryCollection &) = delete;
    ScoreEntryCollection &operator=(const ScoreEntryCollection &) = delete;

    [[nodiscard]] std::size_t len() const noexcept;
    [[nodiscard]] std::unique_ptr<ScoreEntry> take(std::size_t index) noexcept;

  private:
    std::vector<Entry> entries_;
};

} // namespace aegilex::native::ui
