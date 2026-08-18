#include "permission_attachment.h"
#include "../../../aegilex_types.h"

#include <aegilex-runtime/src/cxx_host_admin.rs.h>

#include <endstone/permissions/permission_attachment.h>
#include <endstone/permissions/permission.h>

#include <algorithm>
#include <cstdint>
#include <string>
#include <string_view>
#include <utility>

namespace aegilex::native::admin {

class PermissionAttachment::impl {
  public:
    explicit impl(endstone::PermissionAttachment *attachment) noexcept : attachment(attachment)
    {
    }

    endstone::PermissionAttachment *attachment;
};

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

PermissionAttachment::PermissionAttachment(endstone::PermissionAttachment *attachment) noexcept
    : impl(std::make_shared<class PermissionAttachment::impl>(attachment))
{
}

endstone::PermissionAttachment *PermissionAttachment::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->attachment;
}

void PermissionAttachment::setPermission(const rust::Str permission, const bool value) const
{
    try {
        if (!valid_permission_name(std::string_view(permission.data(), permission.size()))) {
            return;
        }
        impl->attachment->setPermission(std::string(permission), value);
    }
    catch (...) {
    }
}

void PermissionAttachment::unsetPermission(const rust::Str permission) const
{
    try {
        if (!valid_permission_name(std::string_view(permission.data(), permission.size()))) {
            return;
        }
        impl->attachment->unsetPermission(std::string(permission));
    }
    catch (...) {
    }
}

rust::Vec<rust::String> PermissionAttachment::getPermissions() const
{
    rust::Vec<rust::String> names;
    try {
        for (const auto &[name, value] : impl->attachment->getPermissions()) {
            static_cast<void>(value);
            names.push_back(rust::String(name));
        }
    }
    catch (...) {
    }
    return names;
}

PermissionValue PermissionAttachment::getPermissionValue(const rust::Str permission) const
{
    try {
        const auto &permissions = impl->attachment->getPermissions();
        const auto found = permissions.find(std::string(permission));
        if (found == permissions.end()) {
            return PermissionValue{.has = false, .value = false};
        }
        return PermissionValue{.has = true, .value = found->second};
    }
    catch (...) {
        return PermissionValue{.has = false, .value = false};
    }
}

bool PermissionAttachment::remove() const
{
    try {
        return impl->attachment->remove();
    }
    catch (...) {
        return false;
    }
}

bool PermissionAttachment::isSame(const PermissionAttachment &other) const noexcept
{
    return native() != nullptr && native() == other.native();
}

} // namespace aegilex::native::admin
