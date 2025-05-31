mod auth;
mod jre;
mod mojang;

use auth::command::*;
use jre::command::*;
use mojang::command::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_device_code,
            get_minecraft_profile,
            check_game_ownership,
            get_player_skins,
            get_player_capes,
            get_player_uuid,
            get_player_avatar_url,
            get_player_skin_preview_url,
            scan_all_jres,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
