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
#[serde(rename_all = "lowercase")]
pub enum DownloadSources {
    Official,
    BmclApi,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProxyType {
    Http,
    Socks5,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ColorTheme {
    FollowSystem,
    Light,
    Dark,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct LauncherConfig {
    #[serde(default = "default_last_game_path")]
    pub last_game_path: String,
    pub selected_account: Option<String>,
    #[serde(default = "default_false")]
    pub close_after_launch: bool,
    #[serde(default = "default_color_theme")]
    pub color_theme: ColorTheme,
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
    DownloadSources::Official
}

fn default_color_theme() -> ColorTheme {
    ColorTheme::FollowSystem
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
            selected_account: None,
            close_after_launch: false,
            color_theme: default_color_theme(),
            download_source: default_download_source(),
            enable_proxy: false,
            proxy: default_proxy_config(),
            game_path: default_game_paths(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    Microsoft,
    Offline,
    External
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub account_type: AccountType,
    pub name: String, // 玩家名
    pub uuid: String,     // 存储玩家的 UUID
    pub access_token: Option<String>, // 访问令牌
    pub refresh_token: Option<String>, // 刷新令牌
    pub user_id: Option<String>, // Microsoft 账号的用户 ID
    pub expires_in: Option<u64>, // 访问令牌的过期时间
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountConfig {
    pub accounts: Vec<AccountInfo>, // 存储多个账号信息的列表
}
impl Default for AccountConfig {
    fn default() -> Self {
        AccountConfig {
            accounts: Vec::new(), // 初始化为空列表
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
