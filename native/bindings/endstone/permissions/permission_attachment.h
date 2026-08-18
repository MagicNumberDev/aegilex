#pragma once

#include "rust/cxx.h"

#include <cstddef>
#include <cstdint>
#include <memory>

namespace endstone {
class PermissionAttachment;
} // namespace endstone

namespace aegilex::native::admin {

struct PermissionValue;

// OOP/Pimpl facade over endstone::PermissionAttachment; non-owning, mirrors the
// layout of endstone/permissions/permission_attachment.h.
class PermissionAttachment {
  public:
    explicit PermissionAttachment(endstone::PermissionAttachment *attachment) noexcept;
    ~PermissionAttachment() noexcept = default;

    PermissionAttachment(const PermissionAttachment &) = delete;
    PermissionAttachment &operator=(const PermissionAttachment &) = delete;

    void setPermission(rust::Str permission, bool value) const;
    void unsetPermission(rust::Str permission) const;
    rust::Vec<rust::String> getPermissions() const;
    PermissionValue getPermissionValue(rust::Str permission) const;
    bool remove() const;
    [[nodiscard]] bool isSame(const PermissionAttachment &other) const noexcept;
    [[nodiscard]] endstone::PermissionAttachment *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

} // namespace aegilex::native::admin
