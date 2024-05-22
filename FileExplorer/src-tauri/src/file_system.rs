// how to interpret the directory tree:
// 1. Main table entry points to Subtable entry with the offset_to_subtable
// 2. If its a file you get a ID which probably corresponses to the fat table entry
// 3. If its a directory: you got 2 additional bytes for a Sub_directory_id 

use serde::{de, Deserialize, Serialize};
use serde_json::value::Index;
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
pub enum SubTableType {
  File,
  Root,
  SubDirectory,
}

#[derive(Serialize, Deserialize)]
pub struct MainTableEntry {
  offset_to_subtable: u32,
  id_first_file: u16,
  // first entry is total number of directories
  id_parent_directory: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SubTableEntry {
  name_length: usize,
  name: Vec<u8>,
  sub_directory_id: u16,
  offset: usize,
}

#[derive(Serialize, Deserialize)]
pub struct FNT {
  PathList: Vec<Vec<PathElements>>
}

#[derive(Serialize, Deserialize)]
pub enum PathElements{
  Main(MainTableEntry),
  Sub(SubTableEntry),
}

#[derive(Serialize, Deserialize)]
pub struct Directory{
  entries: Vec<SubTableEntry>,
}

#[tauri::command]
pub fn load_main_table(rom_fs: State<Romfs>) -> Vec<MainTableEntry>{
  // Read first entry because its a bit different than the others
  let fnt_base: usize = 4001792;
  let number_of_directories: usize = Romfs::load_word(&rom_fs,fnt_base+6).try_into().unwrap();
  let main_directory: Vec<u8> = Romfs::load_bytes(&rom_fs,fnt_base,fnt_base+(number_of_directories*8));

  let mut entries: Vec<MainTableEntry> = vec![];
  for byte_entry in main_directory.chunks_exact(8){

    let offset_to_subtable = u32::from_le_bytes(byte_entry[0..4].try_into().unwrap());
   
    let entry = MainTableEntry {
      offset_to_subtable: offset_to_subtable,
      id_first_file: u16::from_le_bytes(byte_entry[4..6].try_into().unwrap()),
      id_parent_directory:  u16::from_le_bytes(byte_entry[6..8].try_into().unwrap()),
    };

    entries.push(entry);

  }
  return entries
}

#[tauri::command]
pub fn load_main_table_entry(rom_fs: &State<Romfs>, offset: usize) -> MainTableEntry{
  // Read first entry because its a bit different than the others
  let fnt_base: usize = 4001792;
  let byte_entry: Vec<u8> = Romfs::load_bytes(&rom_fs,fnt_base,fnt_base+offset+8);
  
  MainTableEntry {
    offset_to_subtable: u32::from_le_bytes(byte_entry[0..4].try_into().unwrap()),
    id_first_file: u16::from_le_bytes(byte_entry[4..6].try_into().unwrap()),
    id_parent_directory:  u16::from_le_bytes(byte_entry[6..8].try_into().unwrap()),
  }

}

pub fn load_sub_table_entry(rom_fs: &State<Romfs>, main_directory_entry: MainTableEntry) -> Vec<SubTableEntry> {
  let fnt_base: usize = 4001792;
  let subtable_length: usize = usize::try_from(Romfs::load_byte(&rom_fs,fnt_base+usize::try_from(main_directory_entry.offset_to_subtable).unwrap())).unwrap();
  println!(" subtable_{:?}",subtable_length);
  let subtable_data_offset: usize = fnt_base + usize::try_from(main_directory_entry.offset_to_subtable).unwrap();
  let subtable_id_offset: usize = fnt_base + subtable_length + usize::try_from(main_directory_entry.offset_to_subtable).unwrap();

  let subtable_name: Vec<u8> = Romfs::load_bytes(&rom_fs,subtable_data_offset, subtable_data_offset + subtable_length);
  let subtable_id: u16 = Romfs::load_word(&rom_fs,subtable_id_offset);

  

  let list: Vec<SubTableEntry> = vec![];
  let mut id: usize = 0;
  let mut offset = 1 + subtable_length ;

  let first_entry = SubTableEntry {
    name_length: subtable_length,
    name: subtable_name,
    sub_directory_id:subtable_id,
    offset: offset.clone(),
  };


  if subtable_length < 128 {
    offset += 2;
  }

  walk_sub_table(&rom_fs, list, id, first_entry, offset.clone())
  

}

#[tauri::command]
pub fn load_directory(rom_fs: State<Romfs>) -> Vec<SubTableEntry> {
  let main_entry = load_main_table_entry(&rom_fs, 0);
  load_sub_table_entry(&rom_fs, main_entry)
  
}
fn walk_sub_table(rom_fs: &State<Romfs>,mut list: Vec<SubTableEntry>,mut id: usize,previous_sub_entry: SubTableEntry,  offset: usize) -> Vec<SubTableEntry>{
  println!("Offset in the Walk table{:?}",offset.clone());
  
  let fnt_base: usize = 4001792;
  let subtable_length: usize = usize::try_from(Romfs::load_byte(&rom_fs,fnt_base+offset+1)).unwrap();
  let subtable_data_offset: usize = fnt_base + offset;
  let subtable_id_offset: usize = fnt_base + subtable_length + offset;

  let subtable_name: Vec<u8> = Romfs::load_bytes(&rom_fs,subtable_data_offset, subtable_data_offset + subtable_length + 1);
  let subtable_id: u16 = Romfs::load_word(&rom_fs,subtable_id_offset);
  println!("offset: {:?}", offset);
  println!("offset: {:?}", id);
  if  previous_sub_entry.name_length == 0 {
    return list
  }
  else if previous_sub_entry.name_length > 128{
   
    let entry = SubTableEntry {
      name_length: subtable_length,
      name: subtable_name,
      sub_directory_id:subtable_id,
      offset: offset.clone(),
    };
    list.push(entry.clone());
    let offset = offset + 2 + subtable_length;
    walk_sub_table(&rom_fs, list, id, entry.clone(), offset.clone())
  }
  else{
    
    let entry = SubTableEntry {
      name_length: subtable_length,
      name: subtable_name,
      sub_directory_id:subtable_id,
      offset: offset.clone(),
    };
    list.push(entry.clone());
    id += 1;
    let offset = offset + subtable_length;
    walk_sub_table(&rom_fs, list, id, entry.clone(), offset.clone())
  }

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

