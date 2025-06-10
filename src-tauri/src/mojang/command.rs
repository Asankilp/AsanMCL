use super::api::*;
use super::model::{CapeData, MinecraftProfile, SkinData};

#[tauri::command]
pub async fn get_minecraft_profile(access_token: String) -> Result<MinecraftProfile, String> {
    let client = MinecraftClient::new();
    client
        .get_minecraft_profile(&access_token)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn check_game_ownership(access_token: String) -> Result<bool, String> {
    let client = MinecraftClient::new();
    client
        .check_game_ownership(&access_token)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_player_skins(access_token: String) -> Result<Vec<SkinData>, String> {
    let client = MinecraftClient::new();
    client
        .get_skins(&access_token)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_player_capes(access_token: String) -> Result<Vec<CapeData>, String> {
    let client = MinecraftClient::new();
    client
        .get_capes(&access_token)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_player_uuid(username: String) -> Result<String, String> {
    let client = MinecraftClient::new();
    client
        .get_player_uuid(&username)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_player_avatar_url(uuid: String, size: Option<u32>) -> String {
    super::api::get_player_avatar_url(&uuid, size)
}

#[tauri::command]
pub fn get_player_skin_preview_url(uuid: String) -> String {
    super::api::get_player_skin_preview_url(&uuid)
}
