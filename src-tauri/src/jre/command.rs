use std::path::PathBuf;

use super::{
    finder::{scan_jres, verify_jre_path},
    util::sort_jres_by_version_desc,
};
use crate::{
    config::saveload::{get_jre_config, save_jre_config},
    jre::model::JreInfo,
};

#[tauri::command]
pub async fn scan_all_jres() -> Result<Vec<JreInfo>, String> {
    let mut jre_info = scan_jres();
    sort_jres_by_version_desc(&mut jre_info);
    Ok(jre_info)
}

#[tauri::command]
pub async fn get_jre_info(path: String) -> Result<Option<JreInfo>, String> {
    let jre_info = verify_jre_path(&PathBuf::from(path));
    if jre_info.is_none() {
        Err("JRE not found".into())
    } else {
        Ok(jre_info)
    }
}

#[tauri::command]
pub async fn get_all_jres() -> Result<Vec<JreInfo>, String> {
    // 自动扫描获取的JRE列表
    let jre_info = scan_jres();

    // 从配置文件读取的JRE列表
    let jre_config = get_jre_config().await.unwrap_or_default();

    // 使用HashSet去重，以路径作为唯一标识
    use std::collections::HashSet;
    let mut unique_paths: HashSet<PathBuf> = HashSet::new();
    let mut result = Vec::new();

    // 先添加扫描到的JRE
    for jre in jre_info {
        if unique_paths.insert(jre.path.clone()) {
            result.push(jre);
        }
    }

    // 再添加配置文件中的JRE（如果路径不重复）
    for jre in jre_config.jres {
        if unique_paths.insert(jre.path.clone()) {
            result.push(jre);
        }
    }

    // 对合并后的结果进行版本排序
    sort_jres_by_version_desc(&mut result);
    Ok(result)
}

#[tauri::command]
pub async fn remove_jre(jre: JreInfo) -> Result<(), String> {
    // 获取当前配置
    let mut jre_config = get_jre_config()
        .await
        .map_err(|e| format!("Failed to get JRE config: {}", e))?;

    // 找到并删除指定的JRE
    jre_config.jres.retain(|j| j.path != jre.path);

    // 保存更新后的配置
    save_jre_config(jre_config)
        .await
        .map_err(|e| format!("Failed to save JRE config: {}", e))?;

    Ok(())
}
