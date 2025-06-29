use std::path::PathBuf;

use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct DownloadProgress {
    pub id: String,
    pub path: PathBuf,
    pub progress: f64,
    pub speed: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct DownloadError {
    pub error: String,
}