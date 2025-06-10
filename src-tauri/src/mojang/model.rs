use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MinecraftProfile {
    pub id: String,           // UUID
    pub name: String,         // 玩家名称
    pub skins: Vec<SkinData>, // 皮肤列表
    pub capes: Vec<CapeData>, // 披风列表
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
