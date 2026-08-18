#include "permission_definition.h"
#include "../../../aegilex_types.h"

#include <aegilex-runtime/src/cxx_host_admin.rs.h>

#include <endstone/permissions/permission.h>
#include <endstone/permissions/permission_default.h>

#include <algorithm>
#include <cstdint>
#include <memory>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

namespace aegilex::native::admin {

class PermissionDefinition::impl {
  public:
    explicit impl(endstone::Permission *definition) noexcept : definition(definition)
    {
    }

    endstone::Permission *definition;
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

[[nodiscard]] bool permission_default(const std::uint8_t raw, endstone::PermissionDefault &out) noexcept
{
    switch (raw) {
    case aegilex::kPermissionDefaultTrue:
        out = endstone::PermissionDefault::True;
        return true;
    case aegilex::kPermissionDefaultFalse:
        out = endstone::PermissionDefault::False;
        return true;
    case aegilex::kPermissionDefaultOperator:
        out = endstone::PermissionDefault::Operator;
        return true;
    case aegilex::kPermissionDefaultNotOperator:
        out = endstone::PermissionDefault::NotOperator;
        return true;
    case aegilex::kPermissionDefaultConsole:
        out = endstone::PermissionDefault::Console;
        return true;
    default:
        return false;
    }
}

} // namespace

PermissionDefinition::PermissionDefinition(endstone::Permission *definition) noexcept
    : impl(std::make_shared<class PermissionDefinition::impl>(definition))
{
}

endstone::Permission *PermissionDefinition::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->definition;
}

rust::String PermissionDefinition::getName() const
{
    try {
        return native() == nullptr ? rust::String() : rust::String(native()->getName());
    }
    catch (...) {
        return rust::String();
    }
}

rust::String PermissionDefinition::getDescription() const
{
    try {
        return native() == nullptr ? rust::String() : rust::String(native()->getDescription());
    }
    catch (...) {
        return rust::String();
    }
}

void PermissionDefinition::setDescription(const rust::Str description) const
{
    try {
        if (native() != nullptr) {
            native()->setDescription(std::string(description));
        }
    }
    catch (...) {
    }
}

std::uint8_t PermissionDefinition::getDefault() const
{
    try {
        if (native() == nullptr) {
            return aegilex::kPermissionDefaultOperator;
        }
        switch (native()->getDefault()) {
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
    }
    catch (...) {
    }
    return aegilex::kPermissionDefaultOperator;
}

void PermissionDefinition::setDefault(const std::uint8_t default_value) const
{
    try {
        endstone::PermissionDefault value = endstone::PermissionDefault::Operator;
        if (native() != nullptr && permission_default(default_value, value)) {
            native()->setDefault(value);
        }
    }
    catch (...) {
    }
}

rust::Vec<PermissionChild> PermissionDefinition::getChildren() const
{
    rust::Vec<PermissionChild> children;
    try {
        if (native() != nullptr) {
            for (const auto &[name, value] : native()->getChildren()) {
                children.push_back(PermissionChild{.name = rust::String(name), .value = value});
            }
        }
    }
    catch (...) {
    }
    return children;
}

void PermissionDefinition::addChild(const rust::Str name, const bool value) const
{
    try {
        if (native() != nullptr && valid_permission_name(std::string_view(name.data(), name.size()))) {
            native()->getChildren()[std::string(name)] = value;
            native()->recalculatePermissibles();
        }
    }
    catch (...) {
    }
}

void PermissionDefinition::removeChild(const rust::Str name) const
{
    try {
        if (native() != nullptr) {
            static_cast<void>(native()->getChildren().erase(std::string(name)));
            native()->recalculatePermissibles();
        }
    }
    catch (...) {
    }
}

void PermissionDefinition::recalculatePermissibles() const
{
    try {
        if (native() != nullptr) {
            native()->recalculatePermissibles();
        }
    }
    catch (...) {
    }
}

std::unique_ptr<PermissionDefinition> PermissionDefinition::addParentByName(const rust::Str name,
                                                                            const bool value) const
{
    try {
        if (native() == nullptr || !valid_permission_name(std::string_view(name.data(), name.size()))) {
            return {};
        }
        auto *parent = native()->addParent(std::string(name), value);
        return parent == nullptr ? std::unique_ptr<PermissionDefinition>()
                                 : std::make_unique<PermissionDefinition>(parent);
    }
    catch (...) {
        return {};
    }
}

PermissionDefinitionCollection::PermissionDefinitionCollection(
    std::vector<std::unique_ptr<PermissionDefinition>> definitions) noexcept
    : definitions_(std::move(definitions))
{
}

std::size_t PermissionDefinitionCollection::len() const noexcept
{
    return definitions_.size();
}

std::unique_ptr<PermissionDefinition>
PermissionDefinitionCollection::takePermissionDefinition(const std::size_t index) noexcept
{
    return index < definitions_.size() ? std::move(definitions_[index]) : std::unique_ptr<PermissionDefinition>();
}

} // namespace aegilex::native::admin
