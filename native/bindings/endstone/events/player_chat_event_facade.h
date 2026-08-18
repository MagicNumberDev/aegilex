#pragma once

#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/server.h"
#include "rust/cxx.h"

#include <optional>
#include <string>
#include <string_view>
#include <vector>

namespace endstone {
class PlayerChatEvent;
}

namespace aegilex::native::endstone_binding::events {

class PlayerChatEventFacade final {
  public:
    explicit PlayerChatEventFacade(endstone::PlayerChatEvent *event) noexcept;
    ~PlayerChatEventFacade() noexcept = default;

    PlayerChatEventFacade(const PlayerChatEventFacade &) = delete;
    PlayerChatEventFacade &operator=(const PlayerChatEventFacade &) = delete;

    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;
    [[nodiscard]] std::string getMessage() const noexcept;
    [[nodiscard]] bool setMessage(std::string_view message) noexcept;
    [[nodiscard]] std::string getFormat() const noexcept;
    [[nodiscard]] bool setFormat(std::string_view format) noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::player::Player> getPlayer() const noexcept;
    [[nodiscard]] bool setPlayer(const ::aegilex::native::player::Player &player) noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::server::PlayerCollection> getRecipients() const noexcept;

    [[nodiscard]] rust::String getMessageForRust() const noexcept;
    [[nodiscard]] bool setMessageForRust(rust::Str message) noexcept;
    [[nodiscard]] rust::String getFormatForRust() const noexcept;
    [[nodiscard]] bool setFormatForRust(rust::Str format) noexcept;

  private:
    endstone::PlayerChatEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
