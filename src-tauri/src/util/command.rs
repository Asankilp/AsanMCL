use std::path::PathBuf;

use crate::util::{game::init_game_path, init::init_launcher};

#[tauri::command]
pub async fn init_game_path_command(path: PathBuf) -> Result<(), String> {
    init_game_path(path)
}

#[tauri::command]
pub fn init_launcher_command() -> () {
    init_launcher()
}
