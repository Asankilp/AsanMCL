use crate::config::{
    model::{AccountConfig, JreConfig, LauncherConfig},
    saveload::{get_account_config, get_launcher_config, save_account_config, save_jre_config, save_launcher_config},
};

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

#[tauri::command]
pub async fn get_launcher_config_command() -> Result<LauncherConfig, String> {
    let launcher_config = get_launcher_config().await?;
    Ok(launcher_config)
}

#[tauri::command]
pub async fn save_launcher_config_command(config: LauncherConfig) -> Result<(), String> {
    let result = save_launcher_config(config).await;
    result.map_err(|e| format!("Failed to save launcher config: {}", e))
}

#[tauri::command]
pub async fn get_account_config_command() -> Result<AccountConfig, String> {
    let account_config = get_account_config().await?;
    Ok(account_config)
}
#[tauri::command]
pub async fn save_account_config_command(config: AccountConfig) -> Result<(), String> {
    let result = save_account_config(config).await;
    result.map_err(|e| format!("Failed to save account config: {}", e))
}