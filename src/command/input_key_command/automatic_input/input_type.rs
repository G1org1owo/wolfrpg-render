use serde::Serialize;

#[derive(Serialize)]
pub enum InputType {
    Basic       = 0x00,
    KeyBoard    = 0x10,
    Mouse       = 0x20,
    Unknown
}

impl InputType {
    pub const fn new(options: u8) -> InputType {
        match options {
            0x00 => Self::Basic,
            0x10 => Self::KeyBoard,
            0x20 => Self::Mouse,
            _ => Self::Unknown
        }
    }
}