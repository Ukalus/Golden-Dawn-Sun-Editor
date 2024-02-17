// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod rom;
mod file_system;
mod header;
mod utils;

fn main() {
  tauri::Builder::default()
    .manage(rom::load_rom("/home/ukalus/projects/golden_sun_reverse/ROM/Golden (En)SunDarkDawn.nds"))
    .invoke_handler(tauri::generate_handler![header::load_meta,file_system::load_fnt,file_system::load_fat])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
