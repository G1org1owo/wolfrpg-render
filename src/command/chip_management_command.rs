use serde::Serialize;
use crate::command::chip_management_command::map_chip_settings::MapChipSettings;
use crate::command::chip_management_command::overwrite_map_chips::OverwriteMapChips;
use crate::command::chip_management_command::switch_chipset::SwitchChipset;

mod map_chip_settings;
mod options;
mod switch_chipset;
mod overwrite_map_chips;

#[derive(Serialize)]
pub enum ChipManagementCommand {
    MapChipSettings(MapChipSettings),
    SwitchChipset(SwitchChipset),
    OverwriteMapChips(OverwriteMapChips),
}

impl ChipManagementCommand {
    pub fn parse_map_chip_settings(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, MapChipSettings) = MapChipSettings::parse(bytes);

        (bytes_read, Self::MapChipSettings(command))
    }

    pub fn parse_switch_chipset(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, SwitchChipset) = SwitchChipset::parse(bytes);

        (bytes_read, Self::SwitchChipset(command))
    }

    pub fn parse_overwrite_map_chips(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, OverwriteMapChips) = OverwriteMapChips::parse(bytes);

        (bytes_read, Self::OverwriteMapChips(command))
    }
}