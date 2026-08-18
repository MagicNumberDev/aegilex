#pragma once

#include "../aegilex_types.h"

#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <string_view>
#include <unordered_map>

namespace endstone {
class Plugin;
class Player;
} // namespace endstone

namespace aegilex::runtime {
struct FormSpecData;
struct FormResponseData;
} // namespace aegilex::runtime

namespace aegilex::native {

class HostContext;
class Runtime;

// Owns guest-initiated Endstone forms. Each shown form records its logical id,
// owning plugin, and kind so that Endstone's submit/close callbacks can be
// routed back into the owning plugin's guest exports on the primary thread.
class FormBridge {
  public:
    FormBridge(HostContext &context, endstone::Plugin &plugin, Runtime *runtime);
    ~FormBridge() noexcept = default;

    FormBridge(const FormBridge &) = delete;
    FormBridge &operator=(const FormBridge &) = delete;

    // Builds the native form from the copied spec and shows it to the player
    // resolved by the 16-byte uuid. Returns the logical form id.
    [[nodiscard]] aegilex::status show(std::string_view plugin_id, const std::array<std::uint8_t, 16> &player_uuid,
                                       const aegilex::runtime::FormSpecData &spec, std::uint64_t *out_form_id) noexcept;
    // Closes every form currently open for the player resolved by uuid.
    [[nodiscard]] aegilex::status close_form(const std::array<std::uint8_t, 16> &player_uuid) noexcept;
    // Stops callbacks retained by Endstone from dereferencing this bridge.
    void clear_all() noexcept;
    // Stops and forgets forms owned by a disabled guest plugin.
    void clear_for_plugin(std::string_view plugin_id) noexcept;

  private:
    struct Record {
        std::string plugin_id;
        std::array<std::uint8_t, 16> player_uuid;
    };

    struct CallbackState {
        FormBridge *bridge{};
    };

    [[nodiscard]] std::uint64_t next_logical_id() noexcept;
    void dispatch_submit(std::uint64_t form_id, endstone::Player *player,
                         const aegilex::runtime::FormResponseData &response) noexcept;
    void dispatch_close(std::uint64_t form_id, endstone::Player *player) noexcept;

    HostContext &context_;
    endstone::Plugin &plugin_;
    Runtime *runtime_;
    std::shared_ptr<CallbackState> callbacks_;
    std::unordered_map<std::uint64_t, Record> forms_;
    std::uint64_t next_logical_id_{1};
};

} // namespace aegilex::native
