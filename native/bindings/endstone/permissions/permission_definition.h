#pragma once

#include "rust/cxx.h"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <vector>

namespace endstone {
class Permission;
} // namespace endstone

namespace aegilex::native::admin {

class Permission;
struct PermissionChild;

// Guest-facing OOP/Pimpl facade over endstone::Permission; non-owning. The
// property surface mirrors the definition WIT interface; children map to value
// data (PermissionChild shared structs in the bridge).
class PermissionDefinition {
  public:
    explicit PermissionDefinition(endstone::Permission *definition) noexcept;
    ~PermissionDefinition() noexcept = default;

    PermissionDefinition(const PermissionDefinition &) = delete;
    PermissionDefinition &operator=(const PermissionDefinition &) = delete;

    rust::String getName() const;
    rust::String getDescription() const;
    void setDescription(rust::Str description) const;
    std::uint8_t getDefault() const;
    void setDefault(std::uint8_t default_value) const;
    rust::Vec<PermissionChild> getChildren() const;
    void addChild(rust::Str name, bool value) const;
    void removeChild(rust::Str name) const;
    void recalculatePermissibles() const;
    [[nodiscard]] std::unique_ptr<PermissionDefinition> addParentByName(rust::Str name, bool value) const;
    [[nodiscard]] endstone::Permission *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

class PermissionDefinitionCollection {
  public:
    explicit PermissionDefinitionCollection(std::vector<std::unique_ptr<PermissionDefinition>> definitions) noexcept;
    ~PermissionDefinitionCollection() noexcept = default;

    PermissionDefinitionCollection(const PermissionDefinitionCollection &) = delete;
    PermissionDefinitionCollection &operator=(const PermissionDefinitionCollection &) = delete;

    [[nodiscard]] std::size_t len() const noexcept;
    [[nodiscard]] std::unique_ptr<PermissionDefinition> takePermissionDefinition(std::size_t index) noexcept;

  private:
    std::vector<std::unique_ptr<PermissionDefinition>> definitions_;
};

} // namespace aegilex::native::admin
