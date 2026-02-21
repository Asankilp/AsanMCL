use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureData {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<TextureMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureMetadata {
    #[serde(default)]
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinPreviewInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin_url: Option<String>,
    pub skin_model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cape_url: Option<String>,
}

impl Default for SkinPreviewInfo {
    fn default() -> Self {
        Self {
            skin_url: None,
            skin_model: String::from("classic"),
            cape_url: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinecraftProfile {
    pub id: String,           // UUID
    pub name: String,         // 玩家名称
    pub skins: Vec<SkinData>, // 皮肤列表
    pub capes: Vec<CapeData>, // 披风列表
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileProperty {
    pub name: String, // 属性名称，目前仅有 "textures"
    pub value: String, // Base64编码的属性值，通常包含皮肤和披风信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueriedMinecraftProfile {
    pub id: String,           // UUID
    pub name: String,         // 玩家名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy: Option<bool>,          // 是否为旧版账户
    pub properties: Vec<ProfileProperty>, // 账户属性列表

}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecodedTextureData {
    #[serde(rename = "SKIN")]
    pub skin: Option<TextureData>,
    #[serde(rename = "CAPE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cape: Option<TextureData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecodedTextureProperty {
    pub timestamp: u64,
    #[serde(rename = "profileId")]
    pub profile_id: String,
    #[serde(rename = "profileName")]
    pub profile_name: String,
    pub textures: DecodedTextureData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkinData {
    pub id: String,
    pub state: String,
    pub url: String,
    pub variant: String, // "classic" 或 "slim"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CapeData {
    pub id: String,
    pub state: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GameOwnershipResponse {
    pub items: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerUuidResponse {
    pub id: String,
}
