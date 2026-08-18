#pragma once

#include "rust/cxx.h"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

namespace endstone {
class CommandSender;
class Server;
} // namespace endstone

namespace aegilex::native::level {
class Block;
}

namespace aegilex::native::host {

using Block = ::aegilex::native::level::Block;

// OOP/Pimpl facade over endstone::CommandSender. The impl holds only a
// non-owning endstone::CommandSender* (BDS owns the sender); no VM semantics
// live here. Mirrors the layout of endstone/command/command_sender.h.
class CommandSender {
  public:
    explicit CommandSender(endstone::CommandSender *sender, endstone::Server *server = nullptr) noexcept;
    ~CommandSender() noexcept = default;

    CommandSender(const CommandSender &) = delete;
    CommandSender &operator=(const CommandSender &) = delete;

    rust::String getName() const;
    void sendMessage(rust::Str message) const;
    void sendTranslatableMessage(rust::Str text, rust::Vec<rust::String> parameters) const;
    void sendErrorMessage(rust::Str message) const;
    void sendTranslatableErrorMessage(rust::Str text, rust::Vec<rust::String> parameters) const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::level::Block> getBlock() const;
    [[nodiscard]] endstone::CommandSender *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

} // namespace aegilex::native::host

// cxx_runtime owns the command dispatch signature under aegilex::runtime.
// Keep its opaque type spelling aligned with the typed host facade.
namespace aegilex::runtime {
using CommandSender = ::aegilex::native::host::CommandSender;
}
