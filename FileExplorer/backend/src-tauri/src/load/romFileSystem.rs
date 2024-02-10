use std::fs;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct MainTable {
   pub offset_to_subtable: [u8; 4],
   pub id_first_subtable: [u8; 2],
   pub sub_table: SubTable,
}

#[derive(Serialize, Deserialize)]
pub struct SubTable {
   pub type_or_length: u8,
   pub file_name: Vec<u8>,
}


