use crate::rom::Romfs;
use serde::{ Deserialize, Serialize};
use serde_json::map::Entry;
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct FatEntry {
    pub start_address: u32,
    pub end_address: u32,
}

#[derive(Serialize, Deserialize)]
pub enum SubTableType {
    File,
    SubDirectory,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MainTableEntry {
    offset_to_subtable: usize,
    id_first_file: u16,
    // first entry is total number of directories
    id_parent_directory: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SubTableEntry {
    name_length: usize,
    name: Vec<u8>,
    sub_directory_id: Option<u16>,
}

#[derive(Serialize, Deserialize)]
pub enum PathElements {
    Main(MainTableEntry),
    Sub(SubTableEntry),
}

#[tauri::command]
pub fn load_main_table(rom_fs: State<Romfs>) -> Vec<MainTableEntry> {
    let fnt_base: usize = 4001792;
    let number_of_directories: usize = Romfs::load_word(&rom_fs, fnt_base + 6).try_into().unwrap();
    let main_directory: Vec<u8> =
        Romfs::load_bytes(&rom_fs, fnt_base, fnt_base + (number_of_directories * 8));

    let mut entries: Vec<MainTableEntry> = vec![];
    for byte_entry in main_directory.chunks_exact(8) {
        let entry = MainTableEntry {
            offset_to_subtable: usize::from_le_bytes(byte_entry[0..4].try_into().unwrap()),
            id_first_file: u16::from_le_bytes(byte_entry[4..6].try_into().unwrap()),
            id_parent_directory: u16::from_le_bytes(byte_entry[6..8].try_into().unwrap()),
        };
        entries.push(entry);
    }
    return entries;
}


pub fn load_main_table_entry(rom_fs: &State<Romfs>) -> MainTableEntry {
    let fnt_base: usize = 4001792;
    let byte_entry: Vec<u8> = Romfs::load_bytes(&rom_fs, fnt_base, fnt_base + 8);
    

    MainTableEntry {
        offset_to_subtable: u32::from_le_bytes(byte_entry[0..4].try_into().unwrap()).try_into().unwrap(),
        id_first_file: u16::from_le_bytes(byte_entry[4..6].try_into().unwrap()),
        id_parent_directory: u16::from_le_bytes(byte_entry[6..8].try_into().unwrap()),
    }
}

pub fn load_sub_table_entry(
    rom_fs: &State<Romfs>,
    main_directory_entry: MainTableEntry,
    offset: &mut usize,
) -> SubTableEntry {
    let fnt_base: usize = 4001792;
    let mut list: Vec<SubTableEntry> = vec![];
    
    
    
    let name_length: usize = Romfs::load_byte(rom_fs,fnt_base+main_directory_entry.offset_to_subtable+offset.clone());
    let name: Vec<u8> = Romfs::load_bytes(rom_fs,fnt_base+main_directory_entry.offset_to_subtable+1+offset.clone(),fnt_base+main_directory_entry.offset_to_subtable+name_length+1+offset.clone());
    
    
    println!("name_length: {:?}", name_length);

    if name_length > 128 {
        *offset = *offset + name_length + 2;
        SubTableEntry {
            name_length: name_length,
            name: name,
            sub_directory_id: Some(
                Romfs::load_word(
                    rom_fs,
                    fnt_base + main_directory_entry.offset_to_subtable + name_length + name_length,
                ),
            )
        }
        
    }
    else if name_length == 0 {
        println!("name_length: {:?}", name_length);
        SubTableEntry {
            name_length: 0,
            name: vec![],
            sub_directory_id: None
        }
    }
    else{
        *offset = *offset + name_length + 1;
        SubTableEntry {
                name_length: name_length,
                name: name,
                sub_directory_id: None,
         }
    }

}

#[tauri::command]
pub fn load_directory(rom_fs: State<Romfs>) -> Vec<SubTableEntry> {
    let main_entry = load_main_table_entry(&rom_fs);
    let mut directory: Vec<SubTableEntry> = vec![];
    let mut offset = 0;
    // 400 is just an arbitrary number right here, it should be working recursively until entry.name_length = 0 
    // 
    for i in 0..400 {
        let entry = load_sub_table_entry(&rom_fs, main_entry.clone(),&mut offset);
        if entry.name_length < 128 {
            directory.push(entry);
        }
        else {
            directory.push(entry);
            return directory;
        }
        
    }
    directory
}

#[tauri::command]
pub fn load_fat_entries(rom_fs: State<Romfs>) -> Vec<FatEntry> {
    // can this be not hardcoded but instead gotten from the file header?
    let fat_offset: usize = Romfs::load_address(&rom_fs, 72).try_into().unwrap();
    let fat_size: usize = Romfs::load_address(&rom_fs, 76).try_into().unwrap();
    let fat_list: Vec<u8> = Romfs::load_bytes(&rom_fs, fat_offset, fat_offset + fat_size);
    let mut file_addresses_list: Vec<FatEntry> = vec![];
    for chunk in fat_list.chunks_exact(8) {
        if let (start, end) = chunk.split_at(4) {
            let file_addresses = FatEntry {
                start_address: u32::from_le_bytes(start.try_into().unwrap()),
                end_address: u32::from_le_bytes(end.try_into().unwrap()),
            };
            file_addresses_list.push(file_addresses);
        }
    }

    file_addresses_list
}

#[tauri::command]
pub fn load_file_content(rom_fs: State<Romfs>, start: usize, end: usize) -> Vec<u8> {
    Romfs::load_bytes(&rom_fs, start, end).try_into().unwrap()
}
