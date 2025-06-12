use std::fs;

use config::Config;

use super::util::get_workdir_config_file_path;
use crate::config::model::JreConfig;

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
