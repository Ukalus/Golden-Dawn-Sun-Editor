use std::fs;
use serde::{Deserialize, Serialize};
use tauri::State;
mod romFileSystem;

// Can this be made better by having fixed sizes instead of using Vec<u8>?
// it probably can with https://doc.rust-lang.org/std/primitive.array.html
#[derive(Serialize, Deserialize)]
pub struct RomMetadata {
    game_title: [u8;12],
    game_code: [u8;4],
    // maker_code: [u8;2],
    // unit_code: u8,
    // encryption_seed_select: u8,
    // device_capability: u8,
    // // 8 reserved bytes
    // nds_region: u8,
    // rom_version: u8,
    // auto_start: u8,
    arm9_offset: [u8;4],
    // arm9_entry_address: [u8;4],
    // arm9_ram_address: [u8;4],
    // arm9_size: [u8;4],
    arm7_offset: [u8;4],
    // arm7_entry_address: [u8;4],
    // arm7_ram_address: [u8;4],
    // arm7_size: [u8;4],
    // // this is where the spicy stuff begins: FNT (File name Table) Offset
    file_name_table_offset: [u8;4],
    file_name_table_size: [u8;4],
    file_allocation_table_offset: [u8;4],
    file_allocation_table_size: [u8;4],
}

#[derive(Serialize, Deserialize)]
pub struct Romfs {
  data: Vec<u8>
}

#[tauri::command]
pub fn load_rom(path: &str) -> Romfs {
   Romfs{
    data: fs::read(path).expect("should work")
  }
}

#[tauri::command]
pub fn load_fnt(rom_fs: State<Romfs>) -> romFileSystem::DirectoryTable{
  let fnt_base = 4001792; // fnt offset from header
  let off_subtable = load_bytes(&rom_fs,fnt_base,fnt_base+4);
  let id_first_file = load_bytes(&rom_fs,fnt_base+4,fnt_base+6);
  let id_parent_dict = load_bytes(&rom_fs,fnt_base+6,fnt_base+8);
  romFileSystem::DirectoryTable{
    offset_to_subtable: off_subtable,
    id_first_file_subtable: id_first_file,
    id_parent_directory: id_parent_dict,
  }
}


fn load_bytes<const N: usize>(rom_fs: &State<Romfs>, start: usize, end: usize) -> [u8; N ]{
  rom_fs.data[start..end].try_into().unwrap()
  
}

#[tauri::command]
pub fn load_fat(rom_fs: State<Romfs>) -> romFileSystem::FatTable {
  let fat_offset = 4268544;
  let file_addresses = romFileSystem::FileAdresses{
    start_adress : load_bytes(&rom_fs,fat_offset,fat_offset+4),
    end_adress : load_bytes(&rom_fs,fat_offset+4,fat_offset+8),
  }; 
  let file_addresses2 = romFileSystem::FileAdresses{
    start_adress : load_bytes(&rom_fs,fat_offset+8,fat_offset+12),
    end_adress : load_bytes(&rom_fs,fat_offset+12,fat_offset+16),
  };   
  romFileSystem::FatTable{
    file_adresses: vec![file_addresses,file_addresses2]
  }
}

#[tauri::command]
pub fn load_meta(rom_fs: State<Romfs>) -> RomMetadata {
  RomMetadata {
    game_title: load_bytes(&rom_fs,0,12), 
    game_code: load_bytes(&rom_fs,12,16),
    // maker_code: rom_fs[16..18].try_into().unwrap(),
    // unit_code: rom_fs[18],
    // encryption_seed_select: rom_fs[19],
    // device_capability: rom_fs[20],
    // nds_region: rom_fs[29],
    // rom_version: rom_fs[30],
    // auto_start: rom_fs[31],
    arm9_offset: load_bytes(&rom_fs,32,36),
    // arm9_entry_address: rom_fs[36..40].try_into().unwrap(),
    // arm9_ram_address: rom_fs[40..44].try_into().unwrap(),
    // arm9_size: rom_fs[44..48].try_into().unwrap(),
    arm7_offset: load_bytes(&rom_fs,48,52),
    // arm7_entry_address: rom_fs[52..56].try_into().unwrap(),
    // arm7_ram_address: rom_fs[56..60].try_into().unwrap(),
    // arm7_size: rom_fs[60..64].try_into().unwrap(),
    // // this is where the spicy stuff begins: FNT (File name Table) Offset
    file_name_table_offset: load_bytes(&rom_fs,64,68),
    file_name_table_size: load_bytes(&rom_fs,68,72),
    file_allocation_table_offset: load_bytes(&rom_fs,72,76),
    file_allocation_table_size: load_bytes(&rom_fs,76,80),
    // fnt: rom_fs.data[4001792..4268342].try_into().unwrap(),
  }
}