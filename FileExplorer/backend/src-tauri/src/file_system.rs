// how to interpret the directory tree:
// 1. Main table entry points to Subtable entry with the offset_to_subtable
// 2. If its a file you get a ID which probably corresponses to the fat table entry
// 3. If its a directory: you got 2 additional bytes for a Sub_directory_id 

use serde::{Deserialize, Serialize};
use tauri::State;
use crate::rom::Romfs;



#[derive(Serialize, Deserialize)]
pub struct FatTable {
   pub file_addresses_list: Vec<FileAddresses>,
}

#[derive(Serialize, Deserialize)]
pub struct FileAddresses {
   pub start_address: u32,
   pub end_address: u32,
}

#[derive(Serialize, Deserialize)]
pub enum FntEntryType {
  File,
  Root,
  SubDirectory,
}

#[derive(Serialize, Deserialize)]
pub struct FntEntry {
  entry_type: FntEntryType,
  offset_to_subtable: u32,
  id_first_file: u16,
  // first entry is total number of directories
  id_parent_directory: u16,
  next_entry: SubTable,
  
}

#[derive(Serialize, Deserialize)]
pub struct SubTable {
  length: usize,
  name: Vec<u8>,
  sub_directory_id: u8,
}

#[tauri::command]
pub fn load_fnt(rom_fs: State<Romfs>) -> Vec<FntEntry>{
  // Read first entry because its a bit different than the others
  let fnt_base: usize = 4001792;
  let number_of_directories: usize = Romfs::load_word(&rom_fs,fnt_base+6).try_into().unwrap();
  let main_directory: Vec<u8> = Romfs::load_bytes(&rom_fs,fnt_base,fnt_base+(number_of_directories*6));

  let mut entries: Vec<FntEntry> = vec![];
  for byte_entry in main_directory.chunks_exact(8){

    
    let offset_to_subtable = u32::from_le_bytes(byte_entry[0..4].try_into().unwrap());
    
    let subtable_length: usize = usize::try_from(Romfs::load_byte(&rom_fs,fnt_base+usize::try_from(offset_to_subtable).unwrap())).unwrap();
    let subtable_data_offset: usize = fnt_base + usize::try_from(offset_to_subtable).unwrap();
    let subtable_id_offset: usize = fnt_base + subtable_length + usize::try_from(offset_to_subtable).unwrap();
    

    let subtable_name: Vec<u8> = Romfs::load_bytes(&rom_fs,subtable_data_offset, subtable_data_offset + subtable_length + 1);
    let subtable_id: u8 = Romfs::load_byte(&rom_fs,subtable_id_offset);
    let subTable = SubTable {
      length: subtable_length,
      name: subtable_name,
      sub_directory_id:subtable_id,
    };
    let entry = FntEntry {
      entry_type: FntEntryType::Root,
      offset_to_subtable: offset_to_subtable,
      id_first_file: u16::from_le_bytes(byte_entry[4..6].try_into().unwrap()),
      id_parent_directory:  u16::from_le_bytes(byte_entry[6..8].try_into().unwrap()),
      next_entry: subTable,
    };

    entries.push(entry);

  }

  return entries
  

  
  // Read the rest of the entries, do this recursively until you get to an actual file
}

#[tauri::command]
pub fn load_fat(rom_fs: State<Romfs>) -> FatTable {
  // can this be not hardcoded but instead gotten from the file header?
  let fat_offset: usize = Romfs::load_address(&rom_fs,72).try_into().unwrap();
  let fat_size: usize = Romfs::load_address(&rom_fs,76).try_into().unwrap();
  let fat_list: Vec<u8> = Romfs::load_bytes(&rom_fs,fat_offset,fat_offset+fat_size);
  let mut file_addresses_list: Vec<FileAddresses> = vec![];
  for chunk in fat_list.chunks_exact(8) {
    if let (start, end) = chunk.split_at(4) {
      let file_addresses = FileAddresses{
        start_address :  u32::from_le_bytes(start.try_into().unwrap()),
        end_address : u32::from_le_bytes(end.try_into().unwrap()),
      };
      file_addresses_list.push(file_addresses);
    }
  };
   
  FatTable{
    file_addresses_list: file_addresses_list,
  }
}

#[tauri::command]
pub fn load_file(rom_fs: State<Romfs>,start: usize, end: usize) -> Vec<u8> {
  Romfs::load_bytes(&rom_fs,start,end).try_into().unwrap()
}

