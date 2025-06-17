use std::path::PathBuf;

use crate::game::version::{model::LocalVersionInfo, util::get_local_versions};

#[tauri::command]
pub async fn get_local_versions_command(game_path: PathBuf) -> Result<Vec<LocalVersionInfo>, String> {
    get_local_versions(game_path).await.map_err(|e| e.to_string())
}