#pragma once

#include "rust/cxx.h"

#include <cstddef>
#include <cstdint>
#include <memory>

namespace endstone {
class IpBanList;
class PlayerBanList;
} // namespace endstone

namespace aegilex::native::admin {

struct PlayerBanEntry;

// OOP/Pimpl facade over endstone::PlayerBanList / endstone::IpBanList. The impl
// holds only non-owning pointers (the server owns the ban lists); ban entries
// are returned as value data (shared structs in the bridge).
class BanList {
  public:
    explicit BanList(endstone::PlayerBanList *player_list) noexcept;
    explicit BanList(endstone::IpBanList *ip_list) noexcept;
    ~BanList() noexcept = default;

    BanList(const BanList &) = delete;
    BanList &operator=(const BanList &) = delete;

    bool isBanned(rust::Str target) const;
    bool isBannedByIdentity(rust::Str target, bool has_uuid, rust::Slice<const std::uint8_t> uuid, bool has_xuid,
                            rust::Str xuid) const;
    bool getBanEntry(rust::Str target, PlayerBanEntry &out) const;
    bool addBan(rust::Str target, bool has_reason, rust::Str reason, bool has_expires, std::int64_t expires,
                bool has_source, rust::Str source, PlayerBanEntry &out) const;
    bool addBanByIdentity(rust::Str target, bool has_uuid, rust::Slice<const std::uint8_t> uuid, bool has_xuid,
                          rust::Str xuid, bool has_reason, rust::Str reason, bool has_expires, std::int64_t expires,
                          bool has_source, rust::Str source, PlayerBanEntry &out) const;
    void removeBan(rust::Str target) const;
    void removeBanByIdentity(rust::Str target, bool has_uuid, rust::Slice<const std::uint8_t> uuid, bool has_xuid,
                             rust::Str xuid) const;
    rust::Vec<rust::String> getTargets() const;
    [[nodiscard]] endstone::PlayerBanList *playerNative() const noexcept;
    [[nodiscard]] endstone::IpBanList *ipNative() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

} // namespace aegilex::native::admin
