use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct DownloadProgress {
    pub id: String,
    pub filename: String,
    pub progress: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct DownloadError {
    pub error: String,
}