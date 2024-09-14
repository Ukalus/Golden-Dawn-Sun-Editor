// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_system;
mod header;
mod rom;
mod utils;

fn main() {
    tauri::Builder::default()
    .manage(rom::load_rom("/home/ukalus/Desktop/Golden Sun - Dark Dawn (USA, Australia) (En,Es).nds"))
    .invoke_handler(tauri::generate_handler![
      header::load_meta,
      file_system::load_fat_entries,
      file_system::load_directory,
      file_system::write_file_to_system,
      file_system::load_file_content,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
