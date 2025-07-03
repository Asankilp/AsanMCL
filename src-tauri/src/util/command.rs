use futures::FutureExt;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Mutex;
use std::{collections::HashMap, path::PathBuf};
use tauri::ipc::Channel;
use tauri::{AppHandle, Emitter};
use tokio::sync::watch;

use crate::config::model::LauncherConfig;
use crate::util::downloader::download_with_progress;
use crate::util::model::{DownloadError, DownloadProgress};
use crate::util::{game::init_game_path, init::init_launcher};
// 全局存储下载任务的取消句柄
static DOWNLOAD_CANCEL_MAP: Lazy<Mutex<HashMap<String, watch::Sender<bool>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[tauri::command]
pub async fn init_game_path_command(path: PathBuf) -> Result<(), String> {
    init_game_path(path)
}

#[tauri::command]
pub fn init_launcher_command() -> () {
    init_launcher()
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum DownloadEvent {
    Progress {
        id: String,
        path: PathBuf,
        progress: f64,
        speed: f64,
    },
    Error {
        error: String,
    },
    Finished,
}

#[tauri::command]
pub async fn download_files(
    on_event: Channel<DownloadEvent>,
    files: HashMap<String, PathBuf>,
) -> Result<(), String> {
    use futures::future;
    let mut handles = Vec::new();
    for (url, save_path) in files {
        let on_event = on_event.clone();
        let (cancel_tx, cancel_rx) = match watch::channel(false) {
            (tx, rx) => (tx, rx),
        };
        let save_path_clone = save_path.clone();
        let cancel_tx_clone = cancel_tx.clone();
        handles.push(tokio::spawn({
            let event_clone_for_error = on_event.clone();
            async move {
                let result = std::panic::AssertUnwindSafe(download_with_progress(
                    &url,
                    save_path,
                    {
                        let on_event = on_event.clone();
                        move |id, percent, speed| {
                            if percent >= 0.0 {
                                let mut map = DOWNLOAD_CANCEL_MAP.lock().unwrap();
                                map.entry(id.clone())
                                    .or_insert_with(|| cancel_tx_clone.clone());
                            }
                            let payload = DownloadProgress {
                                id: id.clone(),
                                path: save_path_clone.clone(),
                                progress: percent,
                                speed: speed,
                            };
                            println!("下载进度: {:?}", payload);
                            let _ = on_event.send(DownloadEvent::Progress {
                                id: id.clone(),
                                path: save_path_clone.clone(),
                                progress: percent,
                                speed: speed,
                            });
                        }
                    },
                    &cancel_rx,
                ))
                .catch_unwind()
                .await;
                match result {
                    Ok(Ok(id)) => {
                        let mut map = DOWNLOAD_CANCEL_MAP.lock().unwrap();
                        map.remove(&id);
                        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
                    }
                    Ok(Err(_e)) => {
                        let _ = event_clone_for_error.send(DownloadEvent::Error {
                            error: format!("{:?}", _e),
                        });
                        Err(_e)
                    }
                    Err(_e) => {
                        let _ = event_clone_for_error.send(DownloadEvent::Error {
                            error: format!("{:?}", _e),
                        });
                        Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("{:?}", _e),
                        ))
                            as Box<dyn std::error::Error + Send + Sync>)
                    }
                }
            }
        }));
    }
    let results = future::join_all(handles).await;
    let mut errors = Vec::new();
    for res in results {
        match res {
            Ok(Ok(())) => {}                          // 成功
            Ok(Err(e)) => errors.push(e.to_string()), // 任务返回错误
            Err(e) => errors.push(e.to_string()),     // 任务本身 panic 或 join 失败
        }
    }
    if errors.is_empty() {
        println!("所有下载任务已完成");
        let _ = on_event.send(DownloadEvent::Finished);
        Ok(())
    } else {
        Err(format!("部分下载任务失败: {}", errors.join("; ")))
    }
}

#[tauri::command]
pub fn cancel_download(id: String) -> Result<(), String> {
    let mut map = DOWNLOAD_CANCEL_MAP.lock().unwrap();
    // println!("{:?}", map);
    if let Some(tx) = map.remove(&id) {
        let _ = tx.send(true);
        Ok(())
    } else {
        Err("未找到对应下载任务".to_string())
    }
}

#[tauri::command]
pub async fn update_reqwest_client(config: LauncherConfig) -> () {
    super::reqwest_client::update_reqwest_client(&config).await;
}

#[tauri::command]
pub fn is_path_exists(path: PathBuf) -> bool {
    path.exists()
}

#[tauri::command]
pub async fn read_local_json(path: PathBuf) -> Result<Value, String> {
    let content = tokio::fs::read_to_string(&path)
        .await
        .map_err(|e| format!("读取文件失败: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("解析 JSON 失败: {}", e))
}
