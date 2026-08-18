#pragma once

#include "rust/cxx.h"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>

namespace endstone {
class Permissible;
} // namespace endstone

namespace aegilex::native::admin {
class PermissionAttachment;
}

namespace aegilex::native::host {

struct EffectivePermission;
enum class PermissionLevel : std::uint8_t;

class CommandSender; // defined in command_sender.h

} // namespace aegilex::native::host

namespace aegilex::native::server {
class Server;
}

namespace aegilex::native::host {

// OOP/Pimpl facade over endstone::Permissible. The impl holds only a
// non-owning endstone::Permissible* (BDS owns the object); no VM semantics
// live here. Mirrors the layout of endstone/permissions/permissible.h.
class Permissible {
  public:
    explicit Permissible(endstone::Permissible *permissible) noexcept;
    ~Permissible() noexcept = default;

    Permissible(const Permissible &) = delete;
    Permissible &operator=(const Permissible &) = delete;

    PermissionLevel getPermissionLevel() const;
    bool isPermissionSet(rust::Str name) const;
    bool hasPermission(rust::Str name) const;
    [[nodiscard]] EffectivePermission getEffectivePermission(rust::Str name) const;
    [[nodiscard]] rust::Vec<rust::String> listEffectivePermissionNames() const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::admin::PermissionAttachment>
    attach(const ::aegilex::native::server::Server &server, rust::Str name, bool value) const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::admin::PermissionAttachment>
    attachEmpty(const ::aegilex::native::server::Server &server) const;
    [[nodiscard]] std::unique_ptr<::aegilex::native::admin::PermissionAttachment>
    getEffectiveAttachment(rust::Str permission) const;
    void recalculatePermissions() const;
    [[nodiscard]] endstone::Permissible *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

[[nodiscard]] std::unique_ptr<Permissible> permissible_from_command_sender(const CommandSender &sender) noexcept;
[[nodiscard]] std::unique_ptr<CommandSender> asCommandSender(const Permissible &permissible,
                                                             const ::aegilex::native::server::Server &server) noexcept;

} // namespace aegilex::native::host
