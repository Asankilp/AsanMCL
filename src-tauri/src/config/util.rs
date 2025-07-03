use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn get_global_config_path() -> Result<PathBuf, String> {
    let mut path = dirs::config_dir().expect("Unable to get workdir config dir");
    path.push(".asanmcl");

    if path.exists() && !path.is_dir() {
        fs::remove_file(&path).map_err(|e| format!("Failed to remove .asanmcl file: {}", e))?;
    }

    if !path.exists() {
        fs::create_dir(&path).map_err(|e| format!("Failed to create .asanmcl directory: {}", e))?;
    }

    Ok(path)
}

pub fn get_global_config_file_path(filename: &str) -> Result<PathBuf, String> {
    let mut path = get_global_config_path()?;
    path.push(filename);
    Ok(path)
}

/// 获取工作目录下配置文件存放目录
pub fn get_workdir_config_path() -> Result<PathBuf, String> {
    let mut path = std::env::current_dir().expect("Unable to get workdir config dir");
    path.push(".asanmcl");

    if path.exists() && !path.is_dir() {
        fs::remove_file(&path).map_err(|e| format!("Failed to remove .asanmcl file: {}", e))?;
    }

    if !path.exists() {
        fs::create_dir(&path).map_err(|e| format!("Failed to create .asanmcl directory: {}", e))?;
    }

    Ok(path)
}

/// 获取工作目录下指定文件的目录位置
pub fn get_workdir_config_file_path(filename: &str) -> Result<PathBuf, String> {
    let mut path = get_workdir_config_path()?;
    path.push(filename);
    Ok(path)
}

pub fn get_default_game_paths() -> HashMap<String, PathBuf> {
    let mut paths = HashMap::new();
    let minecraft_path: PathBuf;
    #[cfg(target_os = "windows")]
    {
        minecraft_path = dirs::config_dir().unwrap_or_default().join(".minecraft");
    }
    #[cfg(target_os = "macos")]
    {
        minecraft_path = dirs::config_dir().unwrap_or_default().join("minecraft");
    }
    #[cfg(target_os = "linux")]
    {
        minecraft_path = dirs::home_dir().unwrap_or_default().join(".minecraft");
    }
    paths.insert(
        "game.dir.current_dir".to_string(),
        PathBuf::from(".minecraft"),
    );
    paths.insert("game.dir.official_launcher_dir".to_string(), minecraft_path);
    paths
}
