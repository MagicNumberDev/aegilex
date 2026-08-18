#include "permissible.h"
#include <aegilex-runtime/src/cxx_host_common.rs.h>

#include "bindings/endstone/actor/actor.h"
#include "bindings/endstone/actor/mob.h"
#include "bindings/endstone/actor/player.h"
#include "bindings/endstone/command_sender.h"
#include "bindings/endstone/permissions/permission_attachment.h"
#include "bindings/endstone/server.h"

#include <endstone/command/command_sender.h>
#include <endstone/actor/actor.h>
#include <endstone/actor/mob.h>
#include <endstone/player.h>
#include <endstone/permissions/permissible.h>
#include <endstone/permissions/permission_attachment_info.h>
#include <endstone/plugin/plugin_manager.h>
#include <endstone/server.h>

#include <algorithm>
#include <string>
#include <string_view>

namespace aegilex::native::host {

namespace {

[[nodiscard]] bool valid_permission_name(const std::string_view name) noexcept
{
    if (name.empty()) {
        return false;
    }
    return std::all_of(name.begin(), name.end(), [](const char value) {
        return (value >= 'a' && value <= 'z') || (value >= '0' && value <= '9') || value == '.' || value == '_' ||
               value == '-';
    });
}

} // namespace

class Permissible::impl {
  public:
    explicit impl(endstone::Permissible *permissible) noexcept : permissible(permissible)
    {
    }

    endstone::Permissible *permissible;
};

Permissible::Permissible(endstone::Permissible *permissible) noexcept
    : impl(std::make_shared<class Permissible::impl>(permissible))
{
}

endstone::Permissible *Permissible::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->permissible;
}

PermissionLevel Permissible::getPermissionLevel() const
{
    try {
        switch (impl->permissible->getPermissionLevel()) {
        case endstone::PermissionLevel::Operator:
            return PermissionLevel::Operator;
        case endstone::PermissionLevel::Console:
            return PermissionLevel::Console;
        case endstone::PermissionLevel::Default:
            return PermissionLevel::Default;
        }
        return PermissionLevel::Default;
    }
    catch (...) {
        return PermissionLevel::Default;
    }
}

bool Permissible::isPermissionSet(const rust::Str name) const
{
    try {
        return impl->permissible->isPermissionSet(std::string(name.data(), name.size()));
    }
    catch (...) {
        return false;
    }
}

bool Permissible::hasPermission(const rust::Str name) const
{
    try {
        return impl->permissible->hasPermission(std::string(name.data(), name.size()));
    }
    catch (...) {
        return false;
    }
}

EffectivePermission Permissible::getEffectivePermission(const rust::Str name) const
{
    try {
        const std::string permission(name.data(), name.size());
        for (const auto *entry : impl->permissible->getEffectivePermissions()) {
            if (entry != nullptr && entry->getPermission() == permission) {
                return EffectivePermission{.has = true, .value = entry->getValue()};
            }
        }
    }
    catch (...) {
    }
    return EffectivePermission{.has = false, .value = false};
}

rust::Vec<rust::String> Permissible::listEffectivePermissionNames() const
{
    rust::Vec<rust::String> names;
    try {
        for (const auto *entry : impl->permissible->getEffectivePermissions()) {
            if (entry != nullptr) {
                names.push_back(rust::String(entry->getPermission()));
            }
        }
    }
    catch (...) {
    }
    return names;
}

std::unique_ptr<::aegilex::native::admin::PermissionAttachment>
Permissible::attach(const ::aegilex::native::server::Server &server, const rust::Str name, const bool value) const
{
    try {
        if (native() == nullptr || server.native() == nullptr ||
            !valid_permission_name(std::string_view(name.data(), name.size()))) {
            return {};
        }
        auto *plugin = server.native()->getPluginManager().getPlugin("aegilex");
        auto *attachment = plugin == nullptr ? nullptr : native()->addAttachment(*plugin, std::string(name), value);
        return attachment == nullptr ? std::unique_ptr<::aegilex::native::admin::PermissionAttachment>()
                                     : std::make_unique<::aegilex::native::admin::PermissionAttachment>(attachment);
    }
    catch (...) {
        return {};
    }
}

std::unique_ptr<::aegilex::native::admin::PermissionAttachment>
Permissible::attachEmpty(const ::aegilex::native::server::Server &server) const
{
    try {
        if (native() == nullptr || server.native() == nullptr) {
            return {};
        }
        auto *plugin = server.native()->getPluginManager().getPlugin("aegilex");
        auto *attachment = plugin == nullptr ? nullptr : native()->addAttachment(*plugin);
        return attachment == nullptr ? std::unique_ptr<::aegilex::native::admin::PermissionAttachment>()
                                     : std::make_unique<::aegilex::native::admin::PermissionAttachment>(attachment);
    }
    catch (...) {
        return {};
    }
}

std::unique_ptr<::aegilex::native::admin::PermissionAttachment>
Permissible::getEffectiveAttachment(const rust::Str permission) const
{
    try {
        if (native() == nullptr) {
            return {};
        }
        const std::string name(permission.data(), permission.size());
        for (const auto *entry : native()->getEffectivePermissions()) {
            if (entry != nullptr && entry->getPermission() == name && entry->getAttachment() != nullptr) {
                return std::make_unique<::aegilex::native::admin::PermissionAttachment>(entry->getAttachment());
            }
        }
    }
    catch (...) {
    }
    return {};
}

void Permissible::recalculatePermissions() const
{
    try {
        impl->permissible->recalculatePermissions();
    }
    catch (...) {
    }
}

std::unique_ptr<Permissible> permissible_from_command_sender(const CommandSender &sender) noexcept
{
    try {
        return sender.native() == nullptr
                   ? std::unique_ptr<Permissible>()
                   : std::make_unique<Permissible>(static_cast<endstone::Permissible *>(sender.native()));
    }
    catch (...) {
        return std::unique_ptr<Permissible>();
    }
}

std::unique_ptr<CommandSender> asCommandSender(const Permissible &permissible,
                                               const ::aegilex::native::server::Server &server) noexcept
{
    try {
        auto *sender = permissible.native() == nullptr ? nullptr : permissible.native()->asCommandSender();
        return sender == nullptr ? std::unique_ptr<CommandSender>()
                                 : std::make_unique<CommandSender>(sender, server.native());
    }
    catch (...) {
        return std::unique_ptr<CommandSender>();
    }
}

} // namespace aegilex::native::host
