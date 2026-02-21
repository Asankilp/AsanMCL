use crate::game::version::model::{ClientJson, LocalVersionInfo};
use std::fs;
use std::path::PathBuf;

pub async fn get_local_versions(game_path: PathBuf) -> Result<Vec<LocalVersionInfo>, String> {
    let versions_dir = game_path.join("versions");
    let mut versions = Vec::new();

    let entries =
        fs::read_dir(&versions_dir).map_err(|e| format!("无法读取 versions 目录: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
        let path = entry.path();

        // 只处理目录
        if path.is_dir() {
            let dir_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| "目录名无效".to_string())?;

            let json_path = path.join(format!("{}.json", dir_name));

            if json_path.exists() {
                let file_content = match fs::read_to_string(&json_path) {
                    Ok(content) => content,
                    Err(e) => {
                        eprintln!("读取文件失败 {}: {}", json_path.display(), e);
                        continue;
                    }
                };

                let info = match serde_json::from_str::<ClientJson>(&file_content) {
                    Ok(info) => info,
                    Err(e) => {
                        eprintln!("解析 JSON 失败 {}: {}", json_path.display(), e);
                        continue;
                    }
                };

                versions.push(LocalVersionInfo {
                    name: dir_name.to_string(),
                    info,
                });
            }
        }
    }

    Ok(versions)
}
