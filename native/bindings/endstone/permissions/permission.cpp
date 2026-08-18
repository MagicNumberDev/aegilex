#include "permission.h"
#include "../../../aegilex_types.h"

#include <endstone/permissions/permission.h>
#include <endstone/permissions/permission_default.h>

#include <string>
#include <utility>

namespace aegilex::native::admin {

class Permission::impl {
  public:
    explicit impl(endstone::Permission *definition) noexcept : definition(definition)
    {
    }

    endstone::Permission *definition;
};

namespace {

[[nodiscard]] std::uint8_t to_abi_default(const endstone::PermissionDefault value) noexcept
{
    switch (value) {
    case endstone::PermissionDefault::True:
        return aegilex::kPermissionDefaultTrue;
    case endstone::PermissionDefault::False:
        return aegilex::kPermissionDefaultFalse;
    case endstone::PermissionDefault::Operator:
        return aegilex::kPermissionDefaultOperator;
    case endstone::PermissionDefault::NotOperator:
        return aegilex::kPermissionDefaultNotOperator;
    case endstone::PermissionDefault::Console:
        return aegilex::kPermissionDefaultConsole;
    }
    return aegilex::kPermissionDefaultOperator;
}

[[nodiscard]] bool from_abi_default(const std::uint8_t value, endstone::PermissionDefault *out) noexcept
{
    switch (value) {
    case aegilex::kPermissionDefaultTrue:
        *out = endstone::PermissionDefault::True;
        return true;
    case aegilex::kPermissionDefaultFalse:
        *out = endstone::PermissionDefault::False;
        return true;
    case aegilex::kPermissionDefaultOperator:
        *out = endstone::PermissionDefault::Operator;
        return true;
    case aegilex::kPermissionDefaultNotOperator:
        *out = endstone::PermissionDefault::NotOperator;
        return true;
    case aegilex::kPermissionDefaultConsole:
        *out = endstone::PermissionDefault::Console;
        return true;
    }
    return false;
}

} // namespace

Permission::Permission(endstone::Permission *definition) noexcept
    : impl(std::make_shared<class Permission::impl>(definition))
{
}

endstone::Permission *Permission::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->definition;
}

std::string Permission::getName() const
{
    try {
        return impl->definition->getName();
    }
    catch (...) {
        return std::string();
    }
}

std::unordered_map<std::string, bool> &Permission::getChildren() const
{
    return impl->definition->getChildren();
}

std::uint8_t Permission::getDefault() const
{
    try {
        return to_abi_default(impl->definition->getDefault());
    }
    catch (...) {
        return aegilex::kPermissionDefaultOperator;
    }
}

void Permission::setDefault(const std::uint8_t default_value) const
{
    try {
        endstone::PermissionDefault value = endstone::PermissionDefault::Operator;
        if (from_abi_default(default_value, &value)) {
            impl->definition->setDefault(value);
        }
    }
    catch (...) {
    }
}

std::string Permission::getDescription() const
{
    try {
        return impl->definition->getDescription();
    }
    catch (...) {
        return std::string();
    }
}

void Permission::setDescription(std::string description) const
{
    try {
        impl->definition->setDescription(std::move(description));
    }
    catch (...) {
    }
}

void Permission::recalculatePermissibles() const
{
    try {
        impl->definition->recalculatePermissibles();
    }
    catch (...) {
    }
}

} // namespace aegilex::native::admin
