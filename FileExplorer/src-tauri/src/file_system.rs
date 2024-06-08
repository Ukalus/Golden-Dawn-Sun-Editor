
use crate::rom::Romfs;
use serde::{Deserialize, Serialize};
use tauri::State;
use std::fs::File;
use std::io::Write; // bring trait into scope
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]

pub enum SubTableType {
    NDSFile(NDSFile),
    NDSDirectory(NDSDirectory),
}

#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct NDSFile {
    length: usize,
    name: String,
    id: u16,
    file_data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct NDSDirectory {
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
    pub fn new(rom_fs: &State<Romfs>, offset: usize) -> Self{
        let fnt_base: usize = 4001792;
        let byte_entry: Vec<u8> = Romfs::load_bytes(&rom_fs, fnt_base + 8*offset, fnt_base + 8*offset + 8);
    
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
                let entry = NDSDirectory {
                    length: name_length - 128,
                    name: name,
                    sub_directory_id: Some(Romfs::load_word(
                        rom_fs,
                        fnt_base + new.offset_to_subtable + name_length - 128,
                    )),
                };
                new.sub_table_entries.push(SubTableType::NDSDirectory(entry));
                
            } else if name_length != 0{
                let name: Vec<u8> = Romfs::load_bytes(
                    rom_fs,
                    fnt_base + new.offset_to_subtable + 1 + offset.clone(),
                    fnt_base + new.offset_to_subtable + name_length + 1 + offset.clone(),
                );

                let name: String = String::from_utf8(name).unwrap();
                offset = offset + name_length + 1;

                

                // 
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
            
                file_index += 1;
                let file_data = rom_fs.data[file_addresses_list[file_index as usize].start_address as usize..file_addresses_list[file_index as usize].end_address as usize].try_into().unwrap();
                let entry = NDSFile {
                    length: name_length,
                    name: name,
                    id: file_index,
                    file_data: file_data,
                };
                new.sub_table_entries.push(SubTableType::NDSFile(entry));
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
pub fn load_directory(rom_fs: State<Romfs>, offset: usize) -> MainTableEntry {
    MainTableEntry::new(&rom_fs, offset)
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
pub fn write_file_to_system(nds_file: NDSFile){
 
    if !Path::new("../exported_files").exists(){
        fs::create_dir("../exported_files").unwrap();
    }
    let file_path: String = "../exported_files/".to_owned() + &nds_file.name;
    let mut file = File::create(file_path).unwrap();

    file.write_all(&nds_file.file_data).unwrap();
}

#[tauri::command]
pub fn load_file_content(rom_fs: State<Romfs>, start: usize, end: usize) -> Vec<u8> {
    Romfs::load_bytes(&rom_fs, start, end).try_into().unwrap()
}
