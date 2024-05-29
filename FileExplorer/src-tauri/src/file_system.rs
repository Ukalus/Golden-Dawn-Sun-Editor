use std::str::from_utf8;

use crate::rom::Romfs;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize, Clone, Debug)]

pub enum SubTableType {
    File(File),
    Directory(Directory),
}

#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct File {
    length: usize,
    name: String,
    id: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct Directory {
    length: usize,
    name: String,
    sub_directory_id: Option<u16>,
}
#[derive(Serialize, Deserialize)]
pub struct FatEntry {
    pub start_address: u32,
    pub end_address: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MainTableEntry {
    offset_to_subtable: usize,
    id_first_file: u16,
    // first entry is total number of directories
    id_parent_directory: u16,
    sub_table_entries: Vec<SubTableType>,
}

impl MainTableEntry {
    pub fn new(rom_fs: &State<Romfs>) -> Self{
        let fnt_base: usize = 4001792;
        let byte_entry: Vec<u8> = Romfs::load_bytes(&rom_fs, fnt_base, fnt_base + 8);
    
        let mut new = MainTableEntry{
            offset_to_subtable: u32::from_le_bytes(byte_entry[0..4].try_into().unwrap())
                .try_into()
                .unwrap(),
            id_first_file: u16::from_le_bytes(byte_entry[4..6].try_into().unwrap()),
            id_parent_directory: u16::from_le_bytes(byte_entry[6..8].try_into().unwrap()),
            sub_table_entries: vec![],
        };
        MainTableEntry::load_sub_entries(&rom_fs, &mut new);
        new 
    }
    pub fn load_sub_entries(rom_fs: &State<Romfs>, new: &mut MainTableEntry){
        let mut offset = 0;
        let fnt_base: usize = 4001792;
        let mut file_index: u16 = new.id_first_file - 1; 
        loop {
            
            let name_length: usize = Romfs::load_byte(
                rom_fs,
                fnt_base + new.offset_to_subtable + offset,
            );
        
            println!("name_length: {:?}", name_length);
        
            if name_length > 128 {
        
                let name: Vec<u8> = Romfs::load_bytes(
                    rom_fs,
                    fnt_base + new.offset_to_subtable + 1 + offset.clone(),
                    fnt_base + new.offset_to_subtable + name_length - 128 + 1 + offset,
                );

                let name: String = String::from_utf8(name).unwrap();
                offset = offset + name_length - 128 + 1 + 2;
                let entry = Directory {
                    length: name_length - 128,
                    name: name,
                    sub_directory_id: Some(Romfs::load_word(
                        rom_fs,
                        fnt_base + new.offset_to_subtable + name_length - 128,
                    )),
                };
                new.sub_table_entries.push(SubTableType::Directory(entry));
                
            } else if name_length != 0{
                let name: Vec<u8> = Romfs::load_bytes(
                    rom_fs,
                    fnt_base + new.offset_to_subtable + 1 + offset.clone(),
                    fnt_base + new.offset_to_subtable + name_length + 1 + offset.clone(),
                );

                let name: String = String::from_utf8(name).unwrap();
                offset = offset + name_length + 1;
                file_index += 1;
                let entry = File {
                    length: name_length,
                    name: name,
                    id: file_index,
                };
                new.sub_table_entries.push(SubTableType::File(entry));
            } else {
                break
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubTableEntry {
    name_length: usize,
    name: Vec<u8>,
    sub_directory_id: Option<u16>,
}



#[derive(Serialize, Deserialize)]
pub enum TableElements {
    Main(MainTableEntry),
    Sub(SubTableEntry),
}

#[tauri::command]
pub fn load_directory(rom_fs: State<Romfs>) -> MainTableEntry {
    MainTableEntry::new(&rom_fs)
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
