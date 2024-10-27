use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::effect_command::base_options::BaseOptions;

#[derive(Serialize)]
pub struct Base {
    options: BaseOptions,
    duration: u32,
    target: u32,
    range: u32,
    value1: u32,
    value2: u32,
    value3: u32
}

impl Base {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let options: u32 = as_u32_le(&bytes[offset..offset + 4]);
        let options: BaseOptions = BaseOptions::new(options);
        offset += 4;

        let duration: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let target: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let range: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let value1: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let value2: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let value3: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        offset += 3; // Command end signature

        (offset, Self {
            options,
            duration,
            target,
            range,
            value1,
            value2,
            value3
        })
    }
}