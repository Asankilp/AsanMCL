mod auth;
mod config;
mod game;
mod jre;
mod mojang;
mod util;

use auth::command::*;
use config::command::*;
use game::command::*;
use game::modloader::version::command::*;
use jre::command::*;
use mojang::command::*;
use util::command::*;

use crate::util::init::init_launcher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_launcher();
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_device_code,
            check_microsoft_login_availability,
            get_minecraft_profile,
            check_game_ownership,
            get_player_skins,
            get_player_capes,
            get_player_uuid,
            get_player_avatar_url,
            get_player_skin_preview_url,
            get_version_manifest,
            scan_all_jres,
            get_jre_info,
            get_jre_config_command,
            save_jre_config_command,
            get_all_jres,
            remove_jre,
            get_launcher_config_command,
            save_launcher_config_command,
            get_account_config_command,
            save_account_config_command,
            get_local_versions_command,
            init_game_path_command,
            init_launcher_command,
            download_files,
            cancel_download,
            get_fabric_loader_versions_by_game_version,
            update_reqwest_client,
            is_path_exists,
            read_local_json,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
