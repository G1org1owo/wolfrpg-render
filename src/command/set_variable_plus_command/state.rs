use serde::Serialize;
use crate::command::set_variable_plus_command::character::Character;
use crate::command::set_variable_plus_command::other::Other;
use crate::command::set_variable_plus_command::picture::Picture;
use crate::command::set_variable_plus_command::position::Position;

#[derive(Serialize)]
pub enum State {
    Character(Character),
    Position(Position),
    Picture(Picture),
    Other(Other)
}

impl State {
    pub fn parse_character(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, character): (usize, Character) = Character::parse(bytes);

        (bytes_read, Self::Character(character))
    }

    pub fn parse_position(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, position): (usize, Position) = Position::parse(bytes);

        (bytes_read, Self::Position(position))
    }

    pub fn parse_picture(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, picture): (usize, Picture) = Picture::parse(bytes);

        (bytes_read, Self::Picture(picture))
    }

    pub fn parse_other(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, other): (usize, Other) = Other::parse(bytes);

        (bytes_read, Self::Other(other))
    }
}