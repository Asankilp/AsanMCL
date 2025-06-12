use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{config::util::get_default_game_paths, jre::model::JreInfo};

#[derive(Serialize, Deserialize)]
pub struct JreConfig {
    pub jres: Vec<JreInfo>, // 存储 JRE 路径的字符串列表
}

impl Default for JreConfig {
    fn default() -> Self {
        JreConfig {
            jres: Vec::new(), // 初始化为空列表
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DownloadSources {
    Mojang,
    BmclApi,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProxyType {
    Http,
    Socks5,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProxyConfig {
    pub r#type: ProxyType,
    pub host: String,
    pub port: u16,
    pub enable_auth: bool,
    pub username: Option<String>,
    pub password: Option<String>,
}

// #[derive(Serialize, Deserialize)]
// pub struct GamePath {
//     pub path: PathBuf,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct LauncherConfig {
    #[serde(default = "default_last_game_path")]
    pub last_game_path: String,
    #[serde(default = "default_false")]
    pub close_after_launch: bool,
    #[serde(default = "default_download_source")]
    pub download_source: DownloadSources,
    #[serde(default = "default_false")]
    pub enable_proxy: bool,
    #[serde(default = "default_proxy_config")]
    pub proxy: ProxyConfig,
    #[serde(default = "default_game_paths")]
    pub game_path: HashMap<String, PathBuf>,
}

fn default_false() -> bool {
    false
}

fn default_last_game_path() -> String {
    "当前目录".to_string()
}

fn default_download_source() -> DownloadSources {
    DownloadSources::Mojang
}

fn default_proxy_config() -> ProxyConfig {
    ProxyConfig {
        r#type: ProxyType::Http,
        host: String::new(),
        port: 0,
        enable_auth: false,
        username: None,
        password: None,
    }
}

fn default_game_paths() -> HashMap<String, PathBuf> {
    get_default_game_paths()
}
impl Default for LauncherConfig {
    fn default() -> Self {
        LauncherConfig {
            last_game_path: default_last_game_path(),
            close_after_launch: false,
            download_source: default_download_source(),
            enable_proxy: false,
            proxy: default_proxy_config(),
            game_path: default_game_paths(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_launcher_config() {
        let config = LauncherConfig::default();
        println!("{:?}", config);
    }
}
