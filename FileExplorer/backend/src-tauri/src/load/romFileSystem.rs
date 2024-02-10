use std::fs;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct FatTable {
   pub file_adresses: Vec<FileAdresses>,
}

#[derive(Serialize, Deserialize)]
pub struct FileAdresses {
   pub start_adress: [u8; 4],
   pub end_adress: [u8; 4],
}

#[derive(Serialize, Deserialize)]
pub struct DirectoryTable {
   pub offset_to_subtable: [u8; 4],
   pub id_first_file_subtable: [u8; 2],
   // id_parent_directory is actually the total number of directories for the first entry 
   pub id_parent_directory: [u8; 2],
}

#[derive(Serialize, Deserialize)]
pub struct SubTable {
   pub type_or_length: u8,
   pub file_name: Vec<u8>,
}


