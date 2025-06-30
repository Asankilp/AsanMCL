use crate::{
    config::model::DownloadSource,
    game::modloader::version::models::fabric::FabricSupportedGameVersion,
};

#[tauri::command]
pub async fn get_fabric_supported_game_versions(
    download_source: DownloadSource,
) -> Result<Vec<FabricSupportedGameVersion>, String> {
    return super::api::get_fabric_supported_game_versions(download_source)
        .await
        .map_err(|e| e.to_string());
}

#[tauri::command]
pub async fn get_fabric_loader_versions_by_game_version(
    game_version: String,
    download_source: DownloadSource,
) -> Result<Vec<String>, String> {
    return super::api::get_fabric_loader_versions_by_game_version(game_version, download_source)
        .await
        .map_err(|e| e.to_string());
}
