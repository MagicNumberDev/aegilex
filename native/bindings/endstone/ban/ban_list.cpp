#include "ban_list.h"
#include <aegilex-runtime/src/cxx_host_admin.rs.h>

#include <endstone/ban/ip_ban_entry.h>
#include <endstone/ban/ip_ban_list.h>
#include <endstone/ban/player_ban_entry.h>
#include <endstone/ban/player_ban_list.h>
#include <endstone/server.h>
#include <endstone/util/uuid.h>

#include <algorithm>
#include <chrono>
#include <cstdint>
#include <optional>
#include <string>

namespace aegilex::native::admin {

class BanList::impl {
  public:
    explicit impl(endstone::PlayerBanList *player_list) noexcept : player(player_list)
    {
    }

    explicit impl(endstone::IpBanList *ip_list) noexcept : ip(ip_list)
    {
    }

    endstone::PlayerBanList *player{};
    endstone::IpBanList *ip{};
};

namespace {

[[nodiscard]] std::optional<endstone::UUID> optional_uuid(const bool has_uuid,
                                                          const rust::Slice<const std::uint8_t> uuid) noexcept
{
    if (!has_uuid) {
        return std::nullopt;
    }
    endstone::UUID value{};
    std::copy(uuid.begin(), uuid.end(), value.begin());
    return value;
}

[[nodiscard]] std::optional<std::string> optional_text(const bool has, const rust::Str value) noexcept
{
    if (!has) {
        return std::nullopt;
    }
    return std::string(value);
}

[[nodiscard]] std::optional<endstone::BanEntry::Date> optional_expiration(const bool has,
                                                                          const std::int64_t expires) noexcept
{
    if (!has) {
        return std::nullopt;
    }
    return endstone::BanEntry::Date(std::chrono::milliseconds(expires));
}

[[nodiscard]] std::int64_t to_unix_milliseconds(const endstone::BanEntry::Date date) noexcept
{
    return std::chrono::duration_cast<std::chrono::milliseconds>(date.time_since_epoch()).count();
}

[[nodiscard]] bool fill_player_entry(const endstone::PlayerBanEntry &entry, PlayerBanEntry &out) noexcept
{
    try {
        out.name = rust::String(entry.getName());
        const auto uuid = entry.getUniqueId();
        out.has_uuid = uuid.has_value();
        if (uuid.has_value()) {
            std::copy(uuid->begin(), uuid->end(), out.uuid.begin());
        }
        const auto xuid = entry.getXuid();
        out.has_xuid = xuid.has_value();
        if (xuid.has_value()) {
            out.xuid = rust::String(*xuid);
        }
        out.has_reason = !entry.getReason().empty();
        out.reason = rust::String(entry.getReason());
        out.source = rust::String(entry.getSource());
        out.created = to_unix_milliseconds(entry.getCreated());
        const auto expiration = entry.getExpiration();
        out.has_expires = expiration.has_value();
        if (expiration.has_value()) {
            out.expires = to_unix_milliseconds(*expiration);
        }
        return true;
    }
    catch (...) {
        return false;
    }
}

[[nodiscard]] bool fill_entry(const endstone::IpBanEntry &entry, PlayerBanEntry &out) noexcept
{
    try {
        out.name = rust::String(entry.getAddress());
        out.has_uuid = false;
        out.has_xuid = false;
        out.has_reason = !entry.getReason().empty();
        out.reason = rust::String(entry.getReason());
        out.source = rust::String(entry.getSource());
        out.created = to_unix_milliseconds(entry.getCreated());
        const auto expiration = entry.getExpiration();
        out.has_expires = expiration.has_value();
        if (expiration.has_value()) {
            out.expires = to_unix_milliseconds(*expiration);
        }
        return true;
    }
    catch (...) {
        return false;
    }
}

[[nodiscard]] bool valid_identity(const bool has_uuid, const rust::Slice<const std::uint8_t> uuid) noexcept
{
    return !has_uuid || uuid.size() == endstone::UUID::size();
}

} // namespace

BanList::BanList(endstone::PlayerBanList *player_list) noexcept
    : impl(std::make_shared<class BanList::impl>(player_list))
{
}

BanList::BanList(endstone::IpBanList *ip_list) noexcept : impl(std::make_shared<class BanList::impl>(ip_list))
{
}

endstone::PlayerBanList *BanList::playerNative() const noexcept
{
    return impl == nullptr ? nullptr : impl->player;
}

endstone::IpBanList *BanList::ipNative() const noexcept
{
    return impl == nullptr ? nullptr : impl->ip;
}

bool BanList::isBanned(const rust::Str target) const
{
    try {
        if (impl->player != nullptr) {
            return impl->player->isBanned(std::string(target));
        }
        return impl->ip != nullptr && impl->ip->isBanned(std::string(target));
    }
    catch (...) {
        return false;
    }
}

bool BanList::isBannedByIdentity(const rust::Str target, const bool has_uuid,
                                 const rust::Slice<const std::uint8_t> uuid, const bool has_xuid,
                                 const rust::Str xuid) const
{
    try {
        if (impl->player == nullptr || !valid_identity(has_uuid, uuid)) {
            return false;
        }
        return impl->player->isBanned(std::string(target), optional_uuid(has_uuid, uuid),
                                      optional_text(has_xuid, xuid));
    }
    catch (...) {
        return false;
    }
}

bool BanList::getBanEntry(const rust::Str target, PlayerBanEntry &out) const
{
    try {
        if (impl->player != nullptr) {
            const auto entry = impl->player->getBanEntry(std::string(target));
            if (!entry) {
                return false;
            }
            return fill_player_entry(*entry, out);
        }
        if (impl->ip != nullptr) {
            const auto entry = impl->ip->getBanEntry(std::string(target));
            if (!entry) {
                return false;
            }
            return fill_entry(*entry, out);
        }
        return false;
    }
    catch (...) {
        return false;
    }
}

bool BanList::addBan(const rust::Str target, const bool has_reason, const rust::Str reason, const bool has_expires,
                     const std::int64_t expires, const bool has_source, const rust::Str source,
                     PlayerBanEntry &out) const
{
    try {
        const auto target_text = std::string(target);
        if (impl->player != nullptr) {
            const auto &entry =
                impl->player->addBan(target_text, optional_text(has_reason, reason),
                                     optional_expiration(has_expires, expires), optional_text(has_source, source));
            return fill_player_entry(*entry, out);
        }
        if (impl->ip != nullptr) {
            const auto &entry =
                impl->ip->addBan(target_text, optional_text(has_reason, reason),
                                 optional_expiration(has_expires, expires), optional_text(has_source, source));
            return fill_entry(*entry, out);
        }
        return false;
    }
    catch (...) {
        return false;
    }
}

bool BanList::addBanByIdentity(const rust::Str target, const bool has_uuid, const rust::Slice<const std::uint8_t> uuid,
                               const bool has_xuid, const rust::Str xuid, const bool has_reason, const rust::Str reason,
                               const bool has_expires, const std::int64_t expires, const bool has_source,
                               const rust::Str source, PlayerBanEntry &out) const
{
    try {
        if (impl->player == nullptr || !valid_identity(has_uuid, uuid)) {
            return false;
        }
        const auto &entry =
            impl->player->addBan(std::string(target), optional_uuid(has_uuid, uuid), optional_text(has_xuid, xuid),
                                 optional_text(has_reason, reason), optional_expiration(has_expires, expires),
                                 optional_text(has_source, source));
        return fill_player_entry(*entry, out);
    }
    catch (...) {
        return false;
    }
}

void BanList::removeBan(const rust::Str target) const
{
    try {
        const auto target_text = std::string(target);
        if (impl->player != nullptr) {
            impl->player->removeBan(target_text);
        }
        else if (impl->ip != nullptr) {
            impl->ip->removeBan(target_text);
        }
    }
    catch (...) {
    }
}

void BanList::removeBanByIdentity(const rust::Str target, const bool has_uuid,
                                  const rust::Slice<const std::uint8_t> uuid, const bool has_xuid,
                                  const rust::Str xuid) const
{
    try {
        if (impl->player == nullptr || !valid_identity(has_uuid, uuid)) {
            return;
        }
        impl->player->removeBan(std::string(target), optional_uuid(has_uuid, uuid), optional_text(has_xuid, xuid));
    }
    catch (...) {
    }
}

rust::Vec<rust::String> BanList::getTargets() const
{
    rust::Vec<rust::String> targets;
    try {
        if (impl->player != nullptr) {
            for (const auto &entry : impl->player->getEntries()) {
                targets.push_back(rust::String(entry->getName()));
            }
        }
        else if (impl->ip != nullptr) {
            for (const auto &entry : impl->ip->getEntries()) {
                targets.push_back(rust::String(entry->getAddress()));
            }
        }
    }
    catch (...) {
    }
    return targets;
}

} // namespace aegilex::native::admin
