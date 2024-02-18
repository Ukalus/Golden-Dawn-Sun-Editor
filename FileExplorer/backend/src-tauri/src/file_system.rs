// how to interpret the directory tree:
// 1. Main table entry points to Subtable entry with the offset_to_subtable
// 2. If its a file you get a ID which probably corresponses to the fat table entry
// 3. If its a directory: you got 2 additional bytes for a Sub_directory_id 

use serde::{Deserialize, Serialize};
use tauri::State;
use tauri::Manager;
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
   pub directory_table_entries: Vec<DirectoryTableEntry>
}

#[derive(Serialize,Deserialize)]
pub struct DirectoryTableEntry {
  pub offset_to_subtable: u32,
  pub id_first_file_subtable: u16,
   // id_parent_directory is actually the total number of directories for the first entry 
  pub id_parent_directory: u16,
}

#[derive(Serialize,Deserialize)]
pub struct SubTableDirectory {
  pub entries: Vec<SubTableEntry>,
  
}
#[derive(Serialize,Deserialize)]
pub struct SubTableEntry {
  pub length: u8,
  pub name: String, 
}


#[tauri::command]
pub fn load_fnt(rom_fs: State<Romfs>) -> DirectoryTable{
  let fnt_base = 4001792; // fnt offset from header
  
  let num_of_entries: usize = 500;
  let mut entries: Vec<DirectoryTableEntry> = vec![];
  for i in (0..num_of_entries*8).step_by(8){
    let tab_entry = DirectoryTableEntry{
      offset_to_subtable : Romfs::load_address(&rom_fs,fnt_base+i),
      id_first_file_subtable : Romfs::load_word(&rom_fs,fnt_base+i+4),
      id_parent_directory : Romfs::load_word(&rom_fs,fnt_base+i+6),
    };
    entries.push(tab_entry);
  }
  DirectoryTable{
    directory_table_entries: entries,
  }
}

#[tauri::command]
pub fn load_fat(rom_fs: State<Romfs>) -> FatTable {
  // can this be not hardcoded but instead gotten from the file header?
  let fat_offset: usize = Romfs::load_address(&rom_fs,64).try_into().unwrap();
  // where do i find out how many files there are exactly?
  let num_of_files: usize = 100;
  let mut file_addresses_list: Vec<FileAddresses> = vec![];
  for i in (0..num_of_files*8).step_by(8) {
    let file_addresses = FileAddresses{
      start_address : Romfs::load_address(&rom_fs,fat_offset+i),
      end_address : Romfs::load_address(&rom_fs,fat_offset+i+4),
    };
    file_addresses_list.push(file_addresses)
    
  };
   
  FatTable{
    file_addresses_list: file_addresses_list,
  }
}

#[tauri::command]
pub fn load_sub_tables(app: tauri::AppHandle) -> SubTableDirectory {
  
  let rom_fs: State<Romfs> = app.state::<Romfs>();
  let file_name_table_offset: usize = Romfs::load_address(&rom_fs,64).try_into().unwrap();
  // HELP, this line is cloning the entire rom filestream -> this should not happen
  let directory_table: DirectoryTable = load_fnt(rom_fs.clone());
  let mut entries: Vec<SubTableEntry> = vec![];
  for entry in directory_table.directory_table_entries {
    let offset_to_subtable: usize = entry.offset_to_subtable.try_into().unwrap();
    let length_and_type: u8 = Romfs::load_byte(&rom_fs.clone(), file_name_table_offset + offset_to_subtable);

    let entry = SubTableEntry {
      length: length_and_type,
      name: "test.bin".to_string(),
    };
    entries.push(entry);
  };

  SubTableDirectory{
    entries: entries,

  }
}

