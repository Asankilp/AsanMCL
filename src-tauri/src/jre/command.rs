use super::{finder::scan_jres};
use crate::jre::finder::JreInfo;
use serde::{Serialize, Deserialize};

#[tauri::command]
pub async fn scan_all_jres() -> Result<Vec<JreInfo>, String> {
    let jre_info = scan_jres();
    Ok(jre_info)
}