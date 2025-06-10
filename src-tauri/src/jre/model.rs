use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Architecture {
    X86,
    X86_64,
    Arm64,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct JreInfo {
    /// JRE 目录的路径
    pub path: PathBuf,
    /// Java 版本号
    pub version: String,
    /// 系统架构
    pub arch: Architecture,
    /// 提供者
    pub implementor: Option<String>,
    /// 是否为手动添加
    pub manual: Option<bool>,
}
