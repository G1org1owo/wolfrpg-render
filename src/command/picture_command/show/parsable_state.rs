use crate::byte_utils::as_u32_le;
use crate::command::common::u32_or_string::U32OrString;
use crate::command::picture_command::options::Options;
use crate::command::picture_command::show::parsable_fields::ParsableFields;
use crate::command::picture_command::show::parser::{make_filename_and_string, parse_string_value};

pub trait ParsableState<T: ParsableFields<T>> {
    fn parse_fields(bytes: &[u8], options: &Options)
        -> (usize, (u32, u32, [u8; 3], T, u8, Option<U32OrString>, Option<String>)) {
        let mut offset: usize = 0;

        let position_x: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let position_y: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        offset += 4; // zoom
        offset += 4; // angle

        let filename_variable: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let unknown1: [u8; 3] = bytes[offset..offset+3].try_into().unwrap();
        offset += 3;

        let (bytes_read, fields): (usize, T)
            = T::parse(&bytes[offset..]);
        offset += bytes_read;

        let unknown2: u8 = bytes[offset];
        offset += 1;

        let (bytes_read, string_value): (usize, Option<String>)
            = parse_string_value(&bytes[offset..]);
        offset += bytes_read;

        let (filename, string): (Option<U32OrString>, Option<String>)
            = make_filename_and_string(string_value, Some(filename_variable), options);

        (offset, (
            position_x,
            position_y,
            unknown1,
            fields,
            unknown2,
            filename,
            string
        ))
    }
}