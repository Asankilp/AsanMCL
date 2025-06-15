use std::fs;

use config::Config;

use super::util::get_workdir_config_file_path;
use crate::config::{model::{AccountConfig, JreConfig, LauncherConfig}, util::get_global_config_file_path};

pub async fn get_jre_config() -> Result<JreConfig, String> {
    let config_path = get_workdir_config_file_path("jres.json")?;
    if !config_path.exists() {
        let default_config = serde_json::to_string_pretty(&JreConfig::default())
            .map_err(|e| format!("Failed to serialize default JRE config: {}", e))?;
        fs::write(config_path.clone(), default_config)
            .map_err(|e| format!("Failed to create JRE config file: {}", e))?;
    }
    let config_ = Config::builder()
        .add_source(config::File::with_name(config_path.to_str().unwrap()))
        .build()
        .map_err(|e| format!("Failed to load JRE config: {}", e))?;
    let jre_config: JreConfig = config_
        .try_deserialize()
        .map_err(|e| format!("Failed to deserialize JRE config: {}", e))?;
    Ok(jre_config)
}

pub async fn save_jre_config(config: JreConfig) -> Result<(), String> {
    let config_path = get_workdir_config_file_path("jres.json")?;

    let file_content = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize JRE config: {}", e))?;
    fs::write(config_path, file_content)
        .map_err(|e| format!("Failed to save JRE config: {}", e))?;

    Ok(())
}

pub async fn get_launcher_config() -> Result<LauncherConfig, String> {
    let config_path = get_workdir_config_file_path("launcher.json")?;
    if !config_path.exists() {
        let default_config = serde_json::to_string_pretty(&LauncherConfig::default())
            .map_err(|e| format!("Failed to serialize default launcher config: {}", e))?;
        fs::write(config_path.clone(), default_config)
            .map_err(|e| format!("Failed to create launcher config file: {}", e))?;
    }
    let config_ = Config::builder()
        .add_source(config::File::with_name(config_path.to_str().unwrap()))
        .build()
        .map_err(|e| format!("Failed to load launcher config: {}", e))?;
    let launcher_config: LauncherConfig = config_
        .try_deserialize()
        .map_err(|e| format!("Failed to deserialize launcher config: {}", e))?;
    Ok(launcher_config)
}

pub async fn save_launcher_config(config: LauncherConfig) -> Result<(), String> {
    let config_path = get_workdir_config_file_path("launcher.json")?;

    let file_content = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize launcher config: {}", e))?;
    fs::write(config_path, file_content)
        .map_err(|e| format!("Failed to save launcher config: {}", e))?;

    Ok(())
}

pub async fn get_account_config() -> Result<AccountConfig, String> {
    let config_path = get_global_config_file_path("accounts.json")?;
    if !config_path.exists() {
        let default_config = serde_json::to_string_pretty(&AccountConfig::default())
            .map_err(|e| format!("Failed to serialize default account config: {}", e))?;
        fs::write(config_path.clone(), default_config)
            .map_err(|e| format!("Failed to create account config file: {}", e))?;
    }
    let config_ = Config::builder()
        .add_source(config::File::with_name(config_path.to_str().unwrap()))
        .build()
        .map_err(|e| format!("Failed to load account config: {}", e))?;
    let account_config: AccountConfig = config_
        .try_deserialize()
        .map_err(|e| format!("Failed to deserialize account config: {}", e))?;
    Ok(account_config)
}
pub async fn save_account_config(config: AccountConfig) -> Result<(), String> {
    let config_path = get_global_config_file_path("accounts.json")?;

    let file_content = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize account config: {}", e))?;
    fs::write(config_path, file_content)
        .map_err(|e| format!("Failed to save account config: {}", e))?;

    Ok(())
}