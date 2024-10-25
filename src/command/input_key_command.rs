use serde::Serialize;
use crate::command::input_key_command::automatic_input::AutomaticInput;
use crate::command::input_key_command::input_key::InputKey;
use crate::command::input_key_command::input_toggle::InputToggle;

mod input_key;
mod automatic_input;
mod input_toggle;

#[derive(Serialize)]
pub enum InputKeyCommand {
    InputKey(InputKey),
    AutomaticInput(AutomaticInput),
    InputToggle(InputToggle)
}

impl InputKeyCommand {
    pub fn parse_input_key_base(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, InputKey) = InputKey::parse_base(bytes);

        (bytes_read, Self::InputKey(command))
    }

    pub fn parse_input_key_keyboard_or_pad(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, InputKey) = InputKey::parse_keyboard_or_pad(bytes);

        (bytes_read, Self::InputKey(command))
    }

    pub fn parse_automatic_input_base(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, AutomaticInput) = AutomaticInput::parse_base(bytes);

        (bytes_read, Self::AutomaticInput(command))
    }

    pub fn parse_automatic_input_keyboard(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, AutomaticInput) = AutomaticInput::parse_keyboard(bytes);

        (bytes_read, Self::AutomaticInput(command))
    }

    pub fn parse_input_toggle(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, InputToggle) = InputToggle::parse(bytes);

        (bytes_read, Self::InputToggle(command))
    }
}