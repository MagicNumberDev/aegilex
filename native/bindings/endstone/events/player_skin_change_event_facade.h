#pragma once

#include "bindings/endstone/actor/player.h"

#include <memory>
#include <optional>
#include <string>

#include "rust/cxx.h"

namespace endstone {
class PlayerSkinChangeEvent;
}

namespace aegilex::native::endstone_binding::events {

// Non-owning, callback-scoped view of PlayerSkinChangeEvent. The skin value is
// copied by Endstone; message and cancellation mutate the live event.
class PlayerSkinChangeEventFacade final {
  public:
    explicit PlayerSkinChangeEventFacade(endstone::PlayerSkinChangeEvent *event) noexcept;
    ~PlayerSkinChangeEventFacade() noexcept = default;

    PlayerSkinChangeEventFacade(const PlayerSkinChangeEventFacade &) = delete;
    PlayerSkinChangeEventFacade &operator=(const PlayerSkinChangeEventFacade &) = delete;
    PlayerSkinChangeEventFacade(PlayerSkinChangeEventFacade &&) = delete;
    PlayerSkinChangeEventFacade &operator=(PlayerSkinChangeEventFacade &&) = delete;

    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] bool hasSkinChangeMessage() const noexcept;
    [[nodiscard]] rust::String getSkinChangeMessageForRust() const noexcept;
    [[nodiscard]] bool setSkinChangeMessageForRust(bool has_message, rust::Str message) noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::PlayerSkinChangeEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
