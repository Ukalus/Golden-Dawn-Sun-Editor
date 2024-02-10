// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod load;

fn main() {
  tauri::Builder::default()
    .manage(load::load_rom("/home/ukalus/projects/golden_sun_reverse/ROM/Golden (En)SunDarkDawn.nds"))
    .invoke_handler(tauri::generate_handler![load::load_meta,load::load_fnt])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
