use crate::util::reqwest_client::REQWEST_CLIENT;
use crate::{config::model::DownloadSource, game::version::model::VersionManifest};

use super::model::{
    CapeData, GameOwnershipResponse, MinecraftProfile, PlayerUuidResponse, SkinData,
};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
pub struct MinecraftClient;

const MOJANG_LAUNCHERMETA_ROOT: &str = "https://launchermeta.mojang.com";
const BMCLAPI_LAUNCHERMETA_ROOT: &str = "https://bmclapi2.bangbang93.com";

impl MinecraftClient {
    pub fn new() -> Self {
        Self {}
    }

    /// 获取 Minecraft 玩家信息
    pub async fn get_minecraft_profile(
        &self,
        access_token: &str,
    ) -> Result<MinecraftProfile, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", access_token))?,
        );
        let client = {
            let guard = REQWEST_CLIENT.lock().await;
            match &*guard {
                Some(c) => c.clone(),
                None => return Err("HTTP客户端未初始化".into()),
            }
        };
        let response = client
            .get("https://api.minecraftservices.com/minecraft/profile")
            .headers(headers)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!(
                "获取玩家信息失败: {} {}",
                response.status(),
                response.text().await?
            )
            .into());
        }

        Ok(response.json().await?)
    }

    /// 检查玩家是否拥有正版 Minecraft
    pub async fn check_game_ownership(
        &self,
        access_token: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", access_token))?,
        );
        let client = {
            let guard = REQWEST_CLIENT.lock().await;
            match &*guard {
                Some(c) => c.clone(),
                None => return Err("HTTP客户端未初始化".into()),
            }
        };
        let response = client
            .get("https://api.minecraftservices.com/entitlements/mcstore")
            .headers(headers)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!(
                "检查游戏所有权失败: {} {}",
                response.status(),
                response.text().await?
            )
            .into());
        }

        let data: GameOwnershipResponse = response.json().await?;
        Ok(!data.items.is_empty())
    }

    /// 获取玩家可用的皮肤列表
    pub async fn get_skins(
        &self,
        access_token: &str,
    ) -> Result<Vec<SkinData>, Box<dyn std::error::Error>> {
        let profile = self.get_minecraft_profile(access_token).await?;
        Ok(profile.skins)
    }

    /// 获取玩家可用的披风列表
    pub async fn get_capes(
        &self,
        access_token: &str,
    ) -> Result<Vec<CapeData>, Box<dyn std::error::Error>> {
        let profile = self.get_minecraft_profile(access_token).await?;
        Ok(profile.capes)
    }

    /// 从玩家名称获取其 UUID
    pub async fn get_player_uuid(
        &self,
        username: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let client = {
            let guard = REQWEST_CLIENT.lock().await;
            match &*guard {
                Some(c) => c.clone(),
                None => return Err("HTTP客户端未初始化".into()),
            }
        };
        let response = client
            .get(&format!(
                "https://api.mojang.com/users/profiles/minecraft/{}",
                username
            ))
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(format!("玩家 {} 不存在", username).into());
        }

        if !response.status().is_success() {
            return Err(format!(
                "获取玩家UUID失败: {} {}",
                response.status(),
                response.text().await?
            )
            .into());
        }

        let data: PlayerUuidResponse = response.json().await?;
        Ok(data.id)
    }
}

/// 生成玩家头像URL
pub fn get_player_avatar_url(uuid: Option<String>, size: Option<u32>) -> String {
    let clean_uuid = uuid.map_or(String::from("null"), |u| u.replace('-', ""));
    let size_param = size.map_or(String::from("64"), |s| s.to_string());
    format!(
        "https://crafatar.com/avatars/{}?size={}",
        clean_uuid, size_param
    )
}

/// 生成玩家皮肤预览URL
pub fn get_player_skin_preview_url(uuid: &str) -> String {
    let clean_uuid = uuid.replace('-', "");
    format!(
        "https://crafatar.com/renders/body/{}?overlay=true",
        clean_uuid
    )
}

pub async fn get_version_manifest(
    download_source: DownloadSource,
) -> Result<VersionManifest, String> {
    let url = match download_source {
        DownloadSource::Official => format!(
            "{}/mc/game/version_manifest_v2.json",
            MOJANG_LAUNCHERMETA_ROOT
        ),
        DownloadSource::BmclApi => format!(
            "{}/mc/game/version_manifest_v2.json",
            BMCLAPI_LAUNCHERMETA_ROOT
        ),
    };
    println!("Fetching version manifest from: {}", url);
    let client = {
        let guard = REQWEST_CLIENT.lock().await;
        match &*guard {
            Some(c) => c.clone(),
            None => return Err("HTTP客户端未初始化".to_string()),
        }
    };
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch version manifest: {}", e))?;
    if !response.status().is_success() {
        return Err(format!("Failed to fetch version manifest: {}", response.status()).into());
    }
    let manifest: VersionManifest = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse version manifest: {}", e))?;
    Ok(manifest)
}
