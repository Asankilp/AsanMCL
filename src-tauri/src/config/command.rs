use crate::config::{configs::JreConfig, saveload::save_jre_config};

use super::saveload::get_jre_config;

#[tauri::command]
pub async fn get_jre_config_command() -> Result<JreConfig, String> {
    let jre_config = get_jre_config().await?;
    Ok(jre_config)
}

#[tauri::command]
pub async fn save_jre_config_command(config: JreConfig) -> Result<(), String> {
    let result = save_jre_config(config).await;
    result.map_err(|e| format!("Failed to save JRE config: {}", e))
}
