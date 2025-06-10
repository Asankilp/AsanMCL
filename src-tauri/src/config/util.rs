use std::fs;
use std::path::PathBuf;

// pub async fn get_global_config_path() -> Result<PathBuf, String> {
//     let mut path = dirs::config_dir().ok_or("Unable to get global config dir")?;
//     path.push("asanmcl");
//     Ok(path)
// }

// 获取工作目录下配置文件存放目录
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

// 获取工作目录下指定文件的目录位置
pub fn get_workdir_config_file_path(filename: &str) -> Result<PathBuf, String> {
    let mut path = get_workdir_config_path()?;
    path.push(filename);
    Ok(path)
}
