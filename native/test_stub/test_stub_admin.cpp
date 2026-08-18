// Test-only typed admin bridge stubs. Never linked into the plugin.

#include <aegilex-runtime/src/cxx_host_admin.rs.h>

#include "bindings/endstone/ban/ban_list.h"
#include "bindings/endstone/permissions/permission_attachment.h"
#include "bindings/endstone/permissions/permission_definition.h"

#include <memory>
#include <string>
#include <utility>
#include <vector>

namespace aegilex::native::admin {

class BanList::impl {
  public:
    impl() noexcept = default;
};

class PermissionAttachment::impl {
  public:
    explicit impl(endstone::PermissionAttachment *attachment) noexcept : attachment(attachment)
    {
    }

    endstone::PermissionAttachment *attachment;
};

class PermissionDefinition::impl {
  public:
    explicit impl(endstone::Permission *definition) noexcept : definition(definition)
    {
    }

    endstone::Permission *definition;
};

BanList::BanList(endstone::PlayerBanList *) noexcept : impl(std::make_shared<class BanList::impl>())
{
}

BanList::BanList(endstone::IpBanList *) noexcept : impl(std::make_shared<class BanList::impl>())
{
}

bool BanList::isBanned(rust::Str) const
{
    return true;
}

bool BanList::isBannedByIdentity(rust::Str, bool, rust::Slice<const std::uint8_t>, bool, rust::Str) const
{
    return true;
}

bool BanList::getBanEntry(rust::Str, PlayerBanEntry &out) const
{
    out = PlayerBanEntry{.name = rust::String("Aegilex"),
                         .has_uuid = false,
                         .uuid = {},
                         .has_xuid = false,
                         .xuid = rust::String(),
                         .has_reason = true,
                         .reason = rust::String("probe"),
                         .source = rust::String("stub"),
                         .created = 7000,
                         .has_expires = false,
                         .expires = 0};
    return true;
}

bool BanList::addBan(rust::Str target, bool, rust::Str, bool, std::int64_t, bool, rust::Str, PlayerBanEntry &out) const
{
    out = PlayerBanEntry{.name = rust::String(std::string(target)),
                         .has_uuid = false,
                         .uuid = {},
                         .has_xuid = false,
                         .xuid = rust::String(),
                         .has_reason = false,
                         .reason = rust::String(),
                         .source = rust::String("stub"),
                         .created = 7000,
                         .has_expires = false,
                         .expires = 0};
    return true;
}

bool BanList::addBanByIdentity(rust::Str target, bool, rust::Slice<const std::uint8_t>, bool, rust::Str, bool,
                               rust::Str, bool, std::int64_t, bool, rust::Str, PlayerBanEntry &out) const
{
    return addBan(target, false, rust::Str(), false, 0, false, rust::Str(), out);
}

void BanList::removeBan(rust::Str) const
{
}

void BanList::removeBanByIdentity(rust::Str, bool, rust::Slice<const std::uint8_t>, bool, rust::Str) const
{
}

rust::Vec<rust::String> BanList::getTargets() const
{
    rust::Vec<rust::String> targets;
    targets.push_back(rust::String("Aegilex"));
    return targets;
}

PermissionAttachment::PermissionAttachment(endstone::PermissionAttachment *attachment) noexcept
    : impl(std::make_shared<class PermissionAttachment::impl>(attachment))
{
}

endstone::PermissionAttachment *PermissionAttachment::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->attachment;
}

void PermissionAttachment::setPermission(rust::Str, bool) const
{
}

void PermissionAttachment::unsetPermission(rust::Str) const
{
}

rust::Vec<rust::String> PermissionAttachment::getPermissions() const
{
    rust::Vec<rust::String> names;
    names.push_back(rust::String("aegilex.probe"));
    return names;
}

PermissionValue PermissionAttachment::getPermissionValue(rust::Str) const
{
    return PermissionValue{.has = true, .value = true};
}

bool PermissionAttachment::remove() const
{
    return true;
}

bool PermissionAttachment::isSame(const PermissionAttachment &other) const noexcept
{
    return native() != nullptr && native() == other.native();
}

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
    return rust::String("aegilex.probe");
}

rust::String PermissionDefinition::getDescription() const
{
    return rust::String("probe permission");
}

void PermissionDefinition::setDescription(rust::Str) const
{
}

std::uint8_t PermissionDefinition::getDefault() const
{
    return 2;
}

void PermissionDefinition::setDefault(std::uint8_t) const
{
}

rust::Vec<PermissionChild> PermissionDefinition::getChildren() const
{
    rust::Vec<PermissionChild> children;
    children.push_back(PermissionChild{.name = rust::String("aegilex.child"), .value = true});
    return children;
}

void PermissionDefinition::addChild(rust::Str, bool) const
{
}

void PermissionDefinition::removeChild(rust::Str) const
{
}

void PermissionDefinition::recalculatePermissibles() const
{
}

std::unique_ptr<PermissionDefinition> PermissionDefinition::addParentByName(rust::Str, bool) const
{
    return std::make_unique<PermissionDefinition>(nullptr);
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
