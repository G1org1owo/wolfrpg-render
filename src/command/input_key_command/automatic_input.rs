use serde::Serialize;
use crate::command::input_key_command::automatic_input::input_type::InputType;
use crate::command::input_key_command::automatic_input::state::State;

mod basic;
mod state;
mod basic_options;
mod input_type;
mod keyboard;
mod mouse;
mod mouse_options;
mod mouse_type;

#[derive(Serialize)]
pub struct AutomaticInput {
    input_type: InputType,
    state: State,
}

impl AutomaticInput {
    fn parse(bytes: &[u8], parse_state: fn(&[u8], &InputType) -> (usize, State)) -> (usize, Self) {
        let mut offset: usize = 0;

        let input_type: u8 = bytes[offset + 3];
        let input_type: InputType = InputType::new(input_type);

        let (bytes_read, state): (usize, State) = parse_state(&bytes[offset..], &input_type);
        offset += bytes_read;

        offset += 3; // Command end signature

        (offset, Self {
            input_type,
            state
        })
    }

    pub fn parse_base(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, State::parse_base)
    }

    pub fn parse_keyboard(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, State::parse_keyboard)
    }

    pub fn input_type(&self) -> &InputType {
        &self.input_type
    }

    pub fn input_type_mut(&mut self) -> &mut InputType {
        &mut self.input_type
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }
}