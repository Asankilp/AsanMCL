use std::path::PathBuf;

use super::{finder::{scan_jres, verify_jre_path}, util::sort_jres_by_version_desc};
use crate::jre::model::JreInfo;


#[tauri::command]
pub async fn scan_all_jres() -> Result<Vec<JreInfo>, String> {
    let mut jre_info = scan_jres();
    sort_jres_by_version_desc(&mut jre_info);
    Ok(jre_info)
}

#[tauri::command]
pub async fn get_jre_info(path: String) -> Result<Option<JreInfo>, ()> {
    let jre_info = verify_jre_path(&PathBuf::from(path));
    if jre_info.is_none() {
        Err(())
    } else {
        Ok(jre_info)
    }
}