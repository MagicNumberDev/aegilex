#include "form_bridge.h"

#include "../bindings/endstone/actor/player.h"
#include "../host_context.h"
#include "../runtime_bridge.h"

#include <aegilex-runtime/src/cxx_runtime.rs.h>

#include <endstone/form/action_form.h>
#include <endstone/form/controls/button.h>
#include <endstone/form/controls/divider.h>
#include <endstone/form/controls/dropdown.h>
#include <endstone/form/controls/header.h>
#include <endstone/form/controls/label.h>
#include <endstone/form/controls/slider.h>
#include <endstone/form/controls/step_slider.h>
#include <endstone/form/controls/text_input.h>
#include <endstone/form/controls/toggle.h>
#include <endstone/form/message_form.h>
#include <endstone/form/modal_form.h>
#include <endstone/player.h>
#include <endstone/server.h>

#include <cstring>
#include <limits>
#include <memory>
#include <string>
#include <utility>

namespace aegilex::native {

namespace {

// Control kinds must match the FormControlData.kind encoding in cxx_runtime.rs.
constexpr std::uint32_t kControlButton = 0;
constexpr std::uint32_t kControlLabel = 1;
constexpr std::uint32_t kControlHeader = 2;
constexpr std::uint32_t kControlDivider = 3;
constexpr std::uint32_t kControlDropdown = 4;
constexpr std::uint32_t kControlSlider = 5;
constexpr std::uint32_t kControlStepSlider = 6;
constexpr std::uint32_t kControlTextInput = 7;
constexpr std::uint32_t kControlToggle = 8;

// Form kinds must match the FormSpecData.kind encoding in cxx_runtime.rs.
constexpr std::uint32_t kFormAction = 0;
constexpr std::uint32_t kFormMessage = 1;
constexpr std::uint32_t kFormModal = 2;

[[nodiscard]] endstone::Player *resolve_player(endstone::Server &server,
                                               const std::array<std::uint8_t, 16> &uuid) noexcept
{
    try {
        endstone::UUID native_uuid;
        std::memcpy(native_uuid.data, uuid.data(), uuid.size());
        return server.getPlayer(native_uuid);
    }
    catch (...) {
        return nullptr;
    }
}

[[nodiscard]] std::string_view message_text(const rust::String &text) noexcept
{
    return std::string_view(text.data(), text.size());
}

void apply_controls(endstone::ActionForm &form, const rust::Vec<aegilex::runtime::FormControlData> &controls) noexcept
{
    for (const auto &control : controls) {
        const auto text = message_text(control.text);
        switch (control.kind) {
        case kControlButton: {
            std::optional<std::string> icon;
            if (control.has_icon) {
                icon = std::string(message_text(control.icon));
            }
            form.addButton(endstone::Message{std::string(text)}, icon);
            break;
        }
        case kControlLabel:
            form.addLabel(endstone::Message{std::string(text)});
            break;
        case kControlHeader:
            form.addHeader(endstone::Message{std::string(text)});
            break;
        case kControlDivider:
            form.addDivider();
            break;
        default:
            break;
        }
    }
}

[[nodiscard]] std::vector<std::string> copy_options(const rust::Vec<rust::String> &options) noexcept
{
    std::vector<std::string> copied;
    copied.reserve(options.size());
    for (const auto &option : options) {
        copied.emplace_back(message_text(option));
    }
    return copied;
}

void apply_controls(endstone::ModalForm &form, const rust::Vec<aegilex::runtime::FormControlData> &controls) noexcept
{
    for (const auto &control : controls) {
        const auto text = message_text(control.text);
        switch (control.kind) {
        case kControlDropdown: {
            std::optional<int> default_index;
            if (control.has_default_index) {
                default_index = static_cast<int>(control.default_index);
            }
            endstone::Dropdown dropdown(endstone::Message{std::string(text)}, copy_options(control.options),
                                        default_index);
            form.addControl(dropdown);
            break;
        }
        case kControlSlider: {
            std::optional<float> default_value;
            if (control.has_default_float) {
                default_value = control.default_float;
            }
            endstone::Slider slider(endstone::Message{std::string(text)}, control.min, control.max, control.step,
                                    default_value);
            form.addControl(slider);
            break;
        }
        case kControlStepSlider: {
            std::optional<int> default_index;
            if (control.has_default_index) {
                default_index = static_cast<int>(control.default_index);
            }
            endstone::StepSlider step_slider(endstone::Message{std::string(text)}, copy_options(control.options),
                                             default_index);
            form.addControl(step_slider);
            break;
        }
        case kControlTextInput: {
            std::optional<std::string> default_text;
            if (control.has_default_text) {
                default_text = std::string(message_text(control.default_text));
            }
            endstone::TextInput text_input(endstone::Message{std::string(text)},
                                           endstone::Message{std::string(message_text(control.placeholder))},
                                           default_text);
            form.addControl(text_input);
            break;
        }
        case kControlToggle: {
            endstone::Toggle toggle(endstone::Message{std::string(text)}, control.default_bool);
            form.addControl(toggle);
            break;
        }
        case kControlLabel:
            form.addControl(endstone::Label{endstone::Message{std::string(text)}});
            break;
        case kControlHeader:
            form.addControl(endstone::Header{endstone::Message{std::string(text)}});
            break;
        case kControlDivider:
            form.addControl(endstone::Divider{});
            break;
        default:
            break;
        }
    }
}

} // namespace

FormBridge::FormBridge(HostContext &context, endstone::Plugin &plugin, Runtime *runtime)
    : context_(context), plugin_(plugin), runtime_(runtime), callbacks_(std::make_shared<CallbackState>())
{
    callbacks_->bridge = this;
}

void FormBridge::clear_all() noexcept
{
    callbacks_->bridge = nullptr;
    forms_.clear();
}

void FormBridge::clear_for_plugin(const std::string_view plugin_id) noexcept
{
    try {
        std::erase_if(forms_, [plugin_id](const auto &entry) { return entry.second.plugin_id == plugin_id; });
    }
    catch (...) {
    }
}

std::uint64_t FormBridge::next_logical_id() noexcept
{
    if (next_logical_id_ == 0 || next_logical_id_ > std::numeric_limits<std::uint32_t>::max()) {
        return 0;
    }
    return next_logical_id_++;
}

aegilex::status FormBridge::show(const std::string_view plugin_id, const std::array<std::uint8_t, 16> &player_uuid,
                                 const aegilex::runtime::FormSpecData &spec, std::uint64_t *out_form_id) noexcept
{
    try {
        if (plugin_id.empty() || out_form_id == nullptr || context_.server.native() == nullptr) {
            return aegilex::kInvalidArgument;
        }
        if (!context_.server.native()->isPrimaryThread()) {
            return aegilex::kWrongThread;
        }
        auto *player = resolve_player(*context_.server.native(), player_uuid);
        if (player == nullptr) {
            return aegilex::kNotFound;
        }
        const auto logical_id = next_logical_id();
        if (logical_id == 0) {
            return aegilex::kLimitExceeded;
        }

        const std::string owner(plugin_id);
        const auto callbacks = callbacks_;
        const auto title = message_text(spec.title);
        switch (spec.kind) {
        case kFormAction: {
            endstone::ActionForm form;
            form.setTitle(endstone::Message{std::string(title)});
            if (spec.has_content) {
                form.setContent(endstone::Message{std::string(message_text(spec.content))});
            }
            apply_controls(form, spec.controls);
            form.setOnClose([callbacks, logical_id](endstone::Player *event_player) {
                if (callbacks->bridge != nullptr) {
                    callbacks->bridge->dispatch_close(logical_id, event_player);
                }
            });
            form.setOnSubmit([callbacks, logical_id](endstone::Player *event_player, const int selected) {
                if (selected < 0 || callbacks->bridge == nullptr) {
                    return;
                }
                aegilex::runtime::FormResponseData response{.kind = kFormAction,
                                                            .has_selected_index = true,
                                                            .selected_index = static_cast<std::uint32_t>(selected)};
                callbacks->bridge->dispatch_submit(logical_id, event_player, response);
            });
            forms_.emplace(logical_id, Record{owner, player_uuid});
            player->sendForm(std::move(form));
            break;
        }
        case kFormMessage: {
            endstone::MessageForm form;
            form.setTitle(endstone::Message{std::string(title)});
            if (spec.has_content) {
                form.setContent(endstone::Message{std::string(message_text(spec.content))});
            }
            if (spec.has_button1) {
                form.setButton1(endstone::Message{std::string(message_text(spec.button1))});
            }
            if (spec.has_button2) {
                form.setButton2(endstone::Message{std::string(message_text(spec.button2))});
            }
            form.setOnClose([callbacks, logical_id](endstone::Player *event_player) {
                if (callbacks->bridge != nullptr) {
                    callbacks->bridge->dispatch_close(logical_id, event_player);
                }
            });
            form.setOnSubmit([callbacks, logical_id](endstone::Player *event_player, const int button) {
                if (button < 0 || button > std::numeric_limits<std::uint8_t>::max() || callbacks->bridge == nullptr) {
                    return;
                }
                aegilex::runtime::FormResponseData response{.kind = kFormMessage,
                                                            .message_button = static_cast<std::uint8_t>(button)};
                callbacks->bridge->dispatch_submit(logical_id, event_player, response);
            });
            forms_.emplace(logical_id, Record{owner, player_uuid});
            player->sendForm(std::move(form));
            break;
        }
        case kFormModal: {
            endstone::ModalForm form;
            form.setTitle(endstone::Message{std::string(title)});
            apply_controls(form, spec.controls);
            if (spec.has_submit_button) {
                form.setSubmitButton(endstone::Message{std::string(message_text(spec.submit_button))});
            }
            form.setOnClose([callbacks, logical_id](endstone::Player *event_player) {
                if (callbacks->bridge != nullptr) {
                    callbacks->bridge->dispatch_close(logical_id, event_player);
                }
            });
            form.setOnSubmit([callbacks, logical_id](endstone::Player *event_player, const std::string &json) {
                if (callbacks->bridge == nullptr) {
                    return;
                }
                aegilex::runtime::FormResponseData response{.kind = kFormModal, .modal_json = rust::String(json)};
                callbacks->bridge->dispatch_submit(logical_id, event_player, response);
            });
            forms_.emplace(logical_id, Record{owner, player_uuid});
            player->sendForm(std::move(form));
            break;
        }
        default:
            return aegilex::kInvalidArgument;
        }

        *out_form_id = logical_id;
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

aegilex::status FormBridge::close_form(const std::array<std::uint8_t, 16> &player_uuid) noexcept
{
    try {
        if (context_.server.native() == nullptr) {
            return aegilex::kInvalidArgument;
        }
        if (!context_.server.native()->isPrimaryThread()) {
            return aegilex::kWrongThread;
        }
        auto *player = resolve_player(*context_.server.native(), player_uuid);
        if (player == nullptr) {
            return aegilex::kNotFound;
        }
        player->closeForm();
        for (auto it = forms_.begin(); it != forms_.end();) {
            if (it->second.player_uuid == player_uuid) {
                it = forms_.erase(it);
            }
            else {
                ++it;
            }
        }
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

void FormBridge::dispatch_submit(const std::uint64_t form_id, endstone::Player *player,
                                 const aegilex::runtime::FormResponseData &response) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        const auto it = forms_.find(form_id);
        if (it == forms_.end()) {
            return;
        }
        const auto record = it->second;
        forms_.erase(it);
        std::array<std::uint8_t, 16> player_uuid = record.player_uuid;
        const auto has_player = player != nullptr;
        if (has_player) {
            const auto uuid = player->getUniqueId();
            std::memcpy(player_uuid.data(), uuid.data, uuid.size());
        }
        static_cast<void>(aegilex::runtime::dispatch_form_submit(
            *runtime_->handle, record.plugin_id, form_id, has_player,
            rust::Slice<const std::uint8_t>(player_uuid.data(), player_uuid.size()), response));
    }
    catch (...) {
    }
}

void FormBridge::dispatch_close(const std::uint64_t form_id, endstone::Player *player) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return;
        }
        const auto it = forms_.find(form_id);
        if (it == forms_.end()) {
            return;
        }
        const auto record = it->second;
        std::array<std::uint8_t, 16> player_uuid = record.player_uuid;
        const auto has_player = player != nullptr;
        if (has_player) {
            const auto uuid = player->getUniqueId();
            std::memcpy(player_uuid.data(), uuid.data, uuid.size());
        }
        static_cast<void>(aegilex::runtime::dispatch_form_close(
            *runtime_->handle, record.plugin_id, form_id, has_player,
            rust::Slice<const std::uint8_t>(player_uuid.data(), player_uuid.size())));
    }
    catch (...) {
    }
}

} // namespace aegilex::native
