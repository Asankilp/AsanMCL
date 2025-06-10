use std::path::PathBuf;


pub async fn get_global_config_path() -> Result<PathBuf, String> {
    let mut path = dirs::config_dir().ok_or("Unable to get global config dir")?;
    path.push("asanmcl");
    Ok(path)
}

pub fn get_workdir_config_path() -> Result<PathBuf, String> {
    let mut path = std::env::current_dir().expect("Unable to get workdir config dir");
    path.push(".asanmcl");
    Ok(path)
}