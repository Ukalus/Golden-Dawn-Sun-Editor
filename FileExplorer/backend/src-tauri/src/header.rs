use serde::{Deserialize, Serialize};
use tauri::State;
use crate::rom::Romfs;

#[derive(Serialize, Deserialize)]
pub struct RomMetadata {
    game_title: String,
    game_code: u32,
    maker_code: u16,
    // unit_code: u8,
    // encryption_seed_select: u8,
    // device_capability: u8,
    // // 8 reserved bytes
    // nds_region: u8,
    // rom_version: u8,
    // auto_start: u8,
    arm9_offset: u32,
    // arm9_entry_address: [u8;4],
    // arm9_ram_address: [u8;4],
    // arm9_size: [u8;4],
    arm7_offset: u32,
    // arm7_entry_address: [u8;4],
    // arm7_ram_address: [u8;4],
    // arm7_size: [u8;4],
    file_name_table_offset: u32,
    file_name_table_size: u32,
    file_allocation_table_offset: u32,
    file_allocation_table_size: u32,
}

#[tauri::command]
pub fn load_meta(rom_fs: State<Romfs>) -> RomMetadata {
  RomMetadata {
    game_title: Romfs::load_game_title(&rom_fs,0), 
    game_code: Romfs::load_double_word(&rom_fs,12),
    maker_code: Romfs::load_word(&rom_fs,14),

    arm9_offset: Romfs::load_double_word(&rom_fs,32),
    arm7_offset: Romfs::load_double_word(&rom_fs,48),

    file_name_table_offset: Romfs::load_double_word(&rom_fs,64),
    file_name_table_size: Romfs::load_double_word(&rom_fs,68),
    file_allocation_table_offset: Romfs::load_double_word(&rom_fs,72),
    file_allocation_table_size: Romfs::load_double_word(&rom_fs,76),
  }
}