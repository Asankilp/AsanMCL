use crate::{
    config::model::DownloadSource, game::modloader::version::model::FabricSupportedGameVersions,
};

#[tauri::command]
pub async fn get_fabric_supported_game_versions(
    download_source: DownloadSource,
) -> Result<Vec<FabricSupportedGameVersions>, String> {
    return super::api::get_fabric_supported_game_versions(download_source)
        .await
        .map_err(|e| e.to_string());
}
