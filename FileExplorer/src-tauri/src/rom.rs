use std::fs;
use serde::{Deserialize, Serialize};
use tauri::State;

// Can this be made better by having fixed sizes instead of using Vec<u8>?
// it probably can with https://doc.rust-lang.org/std/primitive.array.html

#[derive(Serialize, Deserialize)]
pub struct Romfs {
  data: Vec<u8>
}
impl Romfs {
  pub fn load_bytes(rom_fs: &State<Romfs>, start: usize, end: usize) -> Vec<u8>{
    rom_fs.data[start..end].try_into().unwrap()
  }
  pub fn load_byte(rom_fs: &State<Romfs>, offset: usize) -> u8{
    rom_fs.data[offset].try_into().unwrap()
  }
  // loads a address as a u32, this assumes a address is always 4 bytes in size  
  pub fn load_address(rom_fs: &State<Romfs>, start: usize) -> u32{
    let val: [u8; 4] = rom_fs.data[start..start+4].try_into().unwrap();
    u32::from_le_bytes(val)
  }
  pub fn load_address_be(rom_fs: &State<Romfs>, start: usize) -> u32{
    let val: [u8; 4] = rom_fs.data[start..start+4].try_into().unwrap();
    u32::from_be_bytes(val)
  }
  // loads a u16 value used in a multitude places across the rom
  pub fn load_word(rom_fs: &State<Romfs>, start: usize) -> u16{
    let val: [u8; 2] = rom_fs.data[start..start+2].try_into().unwrap();
    u16::from_le_bytes(val)
  }
  pub fn load_word_be(rom_fs: &State<Romfs>, start: usize) -> u16{
    let val: [u8; 2] = rom_fs.data[start..start+2].try_into().unwrap();
    u16::from_be_bytes(val)
  }
   // loads a u32 value used in a multitude places across the rom
   pub fn load_double_word(rom_fs: &State<Romfs>, start: usize) -> u32{
    let val: [u8; 4] = rom_fs.data[start..start+4].try_into().unwrap();
    u32::from_le_bytes(val)
  }
  // loads the game title which is 12 bytes big and the only 12 byte value as far as i know
  pub fn load_string(rom_fs: &State<Romfs>, start: usize, end: usize) -> String {
    String::from_utf8(rom_fs.data[start..end].to_vec()).expect("couldn't parse string")
    
  }
}

#[tauri::command]
pub fn load_rom(path: &str) -> Romfs {
   Romfs{
    data: fs::read(path).expect("should work")
  }
}

