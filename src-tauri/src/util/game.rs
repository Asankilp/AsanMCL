use std::fs;
use std::path::PathBuf;

pub fn init_game_path(path: PathBuf) -> Result<(), String> {
    // 如果 path 不存在，则创建
    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| format!("创建主目录失败: {}", e))?;
    }

    // 需要创建的子文件夹
    let folders = [
        "assets",
        "versions",
        "mods",
        "libraries",
        "natives",
        "saves",
        "resourcepacks",
        "logs",
        "config",
    ];
    for folder in folders.iter() {
        let mut subdir = path.clone();
        subdir.push(folder);
        if !subdir.exists() {
            fs::create_dir_all(&subdir)
                .map_err(|e| format!("创建子目录 {:?} 失败: {}", folder, e))?;
        }
    }

    Ok(())
}
