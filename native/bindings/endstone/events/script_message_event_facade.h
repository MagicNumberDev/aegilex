#pragma once

#include "bindings/endstone/command_sender.h"
#include "rust/cxx.h"

#include <optional>
#include <string>

namespace endstone {
class ScriptMessageEvent;
}

namespace aegilex::native::endstone_binding::events {

class ScriptMessageEventFacade final {
  public:
    explicit ScriptMessageEventFacade(endstone::ScriptMessageEvent *event) noexcept;
    ~ScriptMessageEventFacade() noexcept = default;

    ScriptMessageEventFacade(const ScriptMessageEventFacade &) = delete;
    ScriptMessageEventFacade &operator=(const ScriptMessageEventFacade &) = delete;

    [[nodiscard]] rust::String getMessageIdForRust() const noexcept;
    [[nodiscard]] rust::String getMessageForRust() const noexcept;
    [[nodiscard]] std::unique_ptr<::aegilex::native::host::CommandSender> getSender() const noexcept;
    [[nodiscard]] bool isCancelled() const noexcept;
    [[nodiscard]] bool setCancelled(bool cancelled) noexcept;

  private:
    endstone::ScriptMessageEvent *event_;
};

} // namespace aegilex::native::endstone_binding::events
