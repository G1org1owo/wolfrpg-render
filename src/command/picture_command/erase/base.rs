use serde::Serialize;
use crate::byte_utils::as_u32_le;

#[derive(Serialize)]
pub struct Base {
    process_time: u32
}

impl Base {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let process_time: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        (offset, Self {
            process_time
        })
    }
}