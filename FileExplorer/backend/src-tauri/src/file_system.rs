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
pub struct DirectoryTable {
   pub offset_to_subtable: u32,
   pub id_first_file_subtable: u16,
   // id_parent_directory is actually the total number of directories for the first entry 
   pub id_parent_directory: u16,
}


#[tauri::command]
pub fn load_fnt(rom_fs: State<Romfs>) -> DirectoryTable{
  let fnt_base = 4001792; // fnt offset from header
  let off_subtable = Romfs::load_address(&rom_fs,fnt_base);
  let id_first_file = Romfs::load_word(&rom_fs,fnt_base+4);
  let id_parent_dict = Romfs::load_word(&rom_fs,fnt_base+6);
  DirectoryTable{
    offset_to_subtable: off_subtable,
    id_first_file_subtable: id_first_file,
    id_parent_directory: id_parent_dict,
  }
}

#[tauri::command]
pub fn load_fat(rom_fs: State<Romfs>) -> FatTable {
  // can this be not hardcoded but instead gotten from the file header?
  let fat_offset: usize = Romfs::load_address(&rom_fs,64).try_into().unwrap();
  // where do i find out how many files there are exactly?
  let num_of_files: usize = 20;
  let mut file_addresses_list: Vec<FileAddresses> = vec![];
  for mut i in 0..num_of_files {
    let file_addresses = FileAddresses{
      start_address : Romfs::load_address(&rom_fs,fat_offset+i),
      end_address : Romfs::load_address(&rom_fs,fat_offset+i+4),
    };
    i = i + 4;
    file_addresses_list.push(file_addresses)

  };
   
  FatTable{
    file_addresses_list: file_addresses_list,
  }
}

