use crate::command::input_key_command::input_toggle::basic_inputs::BasicInputs;
use crate::command::input_key_command::input_toggle::enabled_inputs::EnabledInputs;
use serde::Serialize;

#[derive(Serialize)]
pub struct Basic {
    inputs: BasicInputs,
    enabled_inputs: EnabledInputs
}

impl Basic {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let inputs: u8 = bytes[offset];
        let inputs: BasicInputs = BasicInputs::new(inputs);
        offset += 1;

        let enabled_inputs: u8 = bytes[offset];
        let enabled_inputs: EnabledInputs = EnabledInputs::new(enabled_inputs);
        offset += 1;

        offset += 1; // padding

        offset += 1; // input_type

        (offset, Self {
            inputs,
            enabled_inputs
        })
    }

    pub fn inputs(&self) -> &BasicInputs {
        &self.inputs
    }

    pub fn inputs_mut(&mut self) -> &mut BasicInputs {
        &mut self.inputs
    }

    pub fn enabled_inputs(&self) -> &EnabledInputs {
        &self.enabled_inputs
    }

    pub fn enabled_inputs_mut(&mut self) -> &mut EnabledInputs {
        &mut self.enabled_inputs
    }
}