#pragma once

#include "rust/cxx.h"

#include <cstdint>
#include <memory>
#include <string>
#include <unordered_map>

namespace endstone {
class Permission;
} // namespace endstone

namespace aegilex::native::admin {

struct PermissionChild;

// Raw OOP/Pimpl facade over endstone::Permission; non-owning, mirrors the API
// of endstone/permissions/permission.h with standard library types. The guest
// view (PermissionDefinition) delegates to this class.
class Permission {
  public:
    explicit Permission(endstone::Permission *definition) noexcept;
    ~Permission() noexcept = default;

    Permission(const Permission &) = delete;
    Permission &operator=(const Permission &) = delete;

    std::string getName() const;
    std::unordered_map<std::string, bool> &getChildren() const;
    std::uint8_t getDefault() const;
    void setDefault(std::uint8_t default_value) const;
    std::string getDescription() const;
    void setDescription(std::string description) const;
    void recalculatePermissibles() const;
    [[nodiscard]] endstone::Permission *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

} // namespace aegilex::native::admin
