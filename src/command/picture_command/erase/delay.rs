use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::picture_command::erase::base::Base;

#[derive(Serialize)]
pub struct Delay {
    base_fields: Base,
    delay: u32,
}

impl Delay {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let (bytes_read, base_fields): (usize, Base)
            = Base::parse(&bytes[offset..]);
        offset += bytes_read;

        let delay: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        (offset, Self {
            base_fields,
            delay
        })
    }
}