// Test-only typed host-common (command sender / permissible / offline player)
// bridge stubs. Never linked into the plugin.

#include <aegilex-runtime/src/cxx_host_common.rs.h>

#include "bindings/endstone/command_sender.h"
#include "bindings/endstone/actor/actor.h"
#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/level/block.h"
#include "bindings/endstone/permissions/permissible.h"
#include "bindings/endstone/permissions/permission_attachment.h"

#include <string>

namespace aegilex::native::host {

class CommandSender::impl {
  public:
    impl() noexcept = default;
};

class Permissible::impl {
  public:
    impl() noexcept = default;
};

CommandSender::CommandSender(endstone::CommandSender *, endstone::Server *) noexcept
    : impl(std::make_shared<class CommandSender::impl>())
{
}

rust::String CommandSender::getName() const
{
    return rust::String("Aegilex");
}

void CommandSender::sendMessage(rust::Str) const
{
}

void CommandSender::sendTranslatableMessage(rust::Str, rust::Vec<rust::String>) const
{
}

void CommandSender::sendErrorMessage(rust::Str) const
{
}

void CommandSender::sendTranslatableErrorMessage(rust::Str, rust::Vec<rust::String>) const
{
}

std::unique_ptr<::aegilex::native::level::Block> CommandSender::getBlock() const
{
    return std::unique_ptr<::aegilex::native::level::Block>();
}

Permissible::Permissible(endstone::Permissible *) noexcept : impl(std::make_shared<class Permissible::impl>())
{
}

PermissionLevel Permissible::getPermissionLevel() const
{
    return PermissionLevel::Default;
}

bool Permissible::isPermissionSet(rust::Str) const
{
    return false;
}

bool Permissible::hasPermission(rust::Str) const
{
    return true;
}

EffectivePermission Permissible::getEffectivePermission(rust::Str) const
{
    return EffectivePermission{.has = true, .value = true};
}

rust::Vec<rust::String> Permissible::listEffectivePermissionNames() const
{
    rust::Vec<rust::String> names;
    names.push_back(rust::String("aegilex.probe"));
    return names;
}

std::unique_ptr<::aegilex::native::admin::PermissionAttachment>
Permissible::attach(const ::aegilex::native::server::Server &, rust::Str, bool) const
{
    return std::make_unique<::aegilex::native::admin::PermissionAttachment>(nullptr);
}

std::unique_ptr<::aegilex::native::admin::PermissionAttachment>
Permissible::attachEmpty(const ::aegilex::native::server::Server &) const
{
    return std::make_unique<::aegilex::native::admin::PermissionAttachment>(nullptr);
}

std::unique_ptr<::aegilex::native::admin::PermissionAttachment> Permissible::getEffectiveAttachment(rust::Str) const
{
    return std::make_unique<::aegilex::native::admin::PermissionAttachment>(nullptr);
}

void Permissible::recalculatePermissions() const
{
}

std::unique_ptr<Permissible> permissible_from_command_sender(const CommandSender &) noexcept
{
    return std::make_unique<Permissible>(nullptr);
}

std::unique_ptr<CommandSender> asCommandSender(const Permissible &, const ::aegilex::native::server::Server &) noexcept
{
    return std::make_unique<CommandSender>(nullptr);
}

} // namespace aegilex::native::host
