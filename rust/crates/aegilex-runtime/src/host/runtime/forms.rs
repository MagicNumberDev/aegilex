use crate::runtime::PluginStoreState;

pub(crate) const FORM_ACTION: u32 = 0;
pub(crate) const FORM_MESSAGE: u32 = 1;
pub(crate) const FORM_MODAL: u32 = 2;

pub(crate) struct GuestForm {
    pub(crate) title: String,
}

impl PluginStoreState {
    pub(crate) fn insert_guest_form(&mut self, form_id: u32, title: String) {
        self.forms
            .insert(form_id, super::forms::GuestForm { title });
    }

    pub(crate) fn remove_guest_form(&mut self, form_id: u32) -> bool {
        self.forms.remove(&form_id).is_some()
    }
}

/// Runtime form resources and Core ABI implementation.
use crate::host::endstone::support::*;
use crate::host::runtime::native;

// Flattened form control kind constants (see cxx_runtime::ffi::FormControlData).
const CONTROL_BUTTON: u32 = 0;
const CONTROL_LABEL: u32 = 1;
const CONTROL_HEADER: u32 = 2;
const CONTROL_DIVIDER: u32 = 3;
const CONTROL_DROPDOWN: u32 = 4;
const CONTROL_SLIDER: u32 = 5;
const CONTROL_TEXT_INPUT: u32 = 7;
const CONTROL_TOGGLE: u32 = 8;

// --- form spec conversion (mirrors host/forms.rs with core_host types) ---

fn message_text(message: &MessageMessage) -> Result<String, TypesHostError> {
    match message {
        MessageMessage::PlainText(text) => Ok(text.clone()),
        MessageMessage::Translatable(_) => Err(TypesHostError::NotFound),
    }
}

fn message_plain(message: &MessageMessage) -> String {
    message_text(message).unwrap_or_default()
}

fn form_defaults() -> cxx_event::FormControlData {
    cxx_event::FormControlData {
        kind: CONTROL_BUTTON,
        text: String::new(),
        has_icon: false,
        icon: String::new(),
        options: Vec::new(),
        placeholder: String::new(),
        has_min: false,
        min: 0.0,
        has_max: false,
        max: 0.0,
        has_step: false,
        step: 0.0,
        has_default_float: false,
        default_float: 0.0,
        has_default_index: false,
        default_index: 0,
        has_default_text: false,
        default_text: String::new(),
        has_default_bool: false,
        default_bool: false,
    }
}

fn control_button(control: &FormButtonButton) -> cxx_event::FormControlData {
    cxx_event::FormControlData {
        kind: CONTROL_BUTTON,
        text: message_plain(&control.text),
        has_icon: control.icon.is_some(),
        icon: control.icon.clone().unwrap_or_default(),
        ..form_defaults()
    }
}

fn control_label(control: &FormLabelLabel) -> cxx_event::FormControlData {
    cxx_event::FormControlData {
        kind: CONTROL_LABEL,
        text: message_plain(&control.text),
        ..form_defaults()
    }
}

fn control_header(control: &FormHeaderHeader) -> cxx_event::FormControlData {
    cxx_event::FormControlData {
        kind: CONTROL_HEADER,
        text: message_plain(&control.label),
        ..form_defaults()
    }
}

fn control_divider() -> cxx_event::FormControlData {
    cxx_event::FormControlData {
        kind: CONTROL_DIVIDER,
        ..form_defaults()
    }
}

fn control_dropdown(control: &FormDropdownDropdown) -> cxx_event::FormControlData {
    cxx_event::FormControlData {
        kind: CONTROL_DROPDOWN,
        text: message_plain(&control.label),
        options: control.options.clone(),
        has_default_index: control.default_index.is_some(),
        default_index: control.default_index.unwrap_or(0),
        ..form_defaults()
    }
}

fn control_slider(control: &FormSliderSlider) -> cxx_event::FormControlData {
    cxx_event::FormControlData {
        kind: CONTROL_SLIDER,
        text: message_plain(&control.label),
        has_min: true,
        min: control.min,
        has_max: true,
        max: control.max,
        has_step: true,
        step: control.step,
        has_default_float: control.default_value.is_some(),
        default_float: control.default_value.unwrap_or(0.0),
        ..form_defaults()
    }
}

fn control_text_input(control: &FormTextInputTextInput) -> cxx_event::FormControlData {
    cxx_event::FormControlData {
        kind: CONTROL_TEXT_INPUT,
        text: message_plain(&control.label),
        placeholder: message_plain(&control.placeholder),
        has_default_text: control.default_value.is_some(),
        default_text: control.default_value.clone().unwrap_or_default(),
        ..form_defaults()
    }
}

fn control_toggle(control: &FormToggleToggle) -> cxx_event::FormControlData {
    cxx_event::FormControlData {
        kind: CONTROL_TOGGLE,
        text: message_plain(&control.label),
        has_default_bool: true,
        default_bool: control.default_value,
        ..form_defaults()
    }
}

fn action_control(control: &ActionFormActionControl) -> cxx_event::FormControlData {
    match control {
        ActionFormActionControl::Button(value) => control_button(value),
        ActionFormActionControl::Label(value) => control_label(value),
        ActionFormActionControl::Header(value) => control_header(value),
        ActionFormActionControl::Divider(_) => control_divider(),
    }
}

fn modal_control(control: &ModalFormModalControl) -> cxx_event::FormControlData {
    match control {
        ModalFormModalControl::Dropdown(value) => control_dropdown(value),
        ModalFormModalControl::Slider(value) => control_slider(value),
        ModalFormModalControl::StepSlider(value) => control_dropdown(value),
        ModalFormModalControl::TextInput(value) => control_text_input(value),
        ModalFormModalControl::Toggle(value) => control_toggle(value),
        ModalFormModalControl::Label(value) => control_label(value),
        ModalFormModalControl::Header(value) => control_header(value),
        ModalFormModalControl::Divider(_) => control_divider(),
    }
}

fn spec_to_cxx(
    spec: &PlayerFormFormSpec,
) -> Result<(u32, String, cxx_event::FormSpecData), TypesHostError> {
    match spec {
        PlayerFormFormSpec::Action(spec) => {
            let title = message_text(&spec.title)?;
            let content = message_text(&spec.content)?;
            let controls = spec.controls.iter().map(action_control).collect();
            Ok((
                FORM_ACTION,
                title.clone(),
                cxx_event::FormSpecData {
                    kind: FORM_ACTION,
                    title,
                    has_content: true,
                    content,
                    has_button1: false,
                    button1: String::new(),
                    has_button2: false,
                    button2: String::new(),
                    controls,
                    has_submit_button: false,
                    submit_button: String::new(),
                    has_icon: false,
                    icon: String::new(),
                },
            ))
        }
        PlayerFormFormSpec::Message(spec) => {
            let title = message_text(&spec.title)?;
            let content = message_text(&spec.content)?;
            let button1 = message_text(&spec.button1)?;
            let button2 = message_text(&spec.button2)?;
            Ok((
                FORM_MESSAGE,
                title.clone(),
                cxx_event::FormSpecData {
                    kind: FORM_MESSAGE,
                    title,
                    has_content: true,
                    content,
                    has_button1: true,
                    button1,
                    has_button2: true,
                    button2,
                    controls: Vec::new(),
                    has_submit_button: false,
                    submit_button: String::new(),
                    has_icon: false,
                    icon: String::new(),
                },
            ))
        }
        PlayerFormFormSpec::Modal(spec) => {
            let title = message_text(&spec.title)?;
            let controls = spec.controls.iter().map(modal_control).collect();
            let submit_button = spec
                .submit_button
                .as_ref()
                .map(message_text)
                .transpose()?
                .unwrap_or_default();
            Ok((
                FORM_MODAL,
                title.clone(),
                cxx_event::FormSpecData {
                    kind: FORM_MODAL,
                    title,
                    has_content: false,
                    content: String::new(),
                    has_button1: false,
                    button1: String::new(),
                    has_button2: false,
                    button2: String::new(),
                    controls,
                    has_submit_button: spec.submit_button.is_some(),
                    submit_button,
                    has_icon: spec.icon.is_some(),
                    icon: spec.icon.clone().unwrap_or_default(),
                },
            ))
        }
    }
}

// --- NBT cxx conversion (mirrors host/inventory/nbt.rs on store_shared::NbtTag) ---

impl crate::core_host::imports::HostForm for PluginStoreState {
    fn form_get_title(
        &mut self,
        self_: u32,
    ) -> Result<Result<MessageMessage, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "form.form.get-title")?;
            self.forms
                .get(&self_)
                .map(|form| MessageMessage::PlainText(form.title.clone()))
                .ok_or_else(not_found)
                .map_err(map_core_host_error)
        })())
    }

    fn drop_form(&mut self, self_: u32) -> Result<(), String> {
        self.remove_guest_form(self_);
        Ok(())
    }
}

// --- player-form ---

impl crate::core_host::imports::HostPlayerForm for PluginStoreState {
    fn show(
        &mut self,
        player: u32,
        spec: PlayerFormFormSpec,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-form.show")?;
            let uuid = resolve_player(
                self,
                player_handle(self, player).map_err(map_core_host_error)?,
            )
            .map(|player| player.getUniqueId())
            .map_err(map_core_host_error)?;
            let (_kind, title, cxx_spec) = spec_to_cxx(&spec)?;
            let form_id = native::form_show(&self.host, &self.plugin_id, &uuid, &cxx_spec)
                .map_err(map_core_host_error)?;
            let form_id =
                u32::try_from(form_id).map_err(|_| map_core_host_error(limit_exceeded()))?;
            self.insert_guest_form(form_id, title);
            Ok(form_id)
        })())
    }

    fn close_form(&mut self, player: u32) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "player-form.close-form")?;
            let uuid = resolve_player(
                self,
                player_handle(self, player).map_err(map_core_host_error)?,
            )
            .map(|player| player.getUniqueId())
            .map_err(map_core_host_error)?;
            native::form_close(&self.host, &uuid).map_err(map_core_host_error)?;
            Ok(())
        })())
    }
}
