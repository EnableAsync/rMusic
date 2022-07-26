#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod music;

fn main() {
  env_logger::init();
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![
    music::search_music
  ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


