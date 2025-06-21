use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProfileIcon {
    Furnace,
    GrassBlock,
    Chest,
    Dirt,
    CobbleStone,
    Anvil,
    Forge,
    Fabric,
    LiteLoader,
    OptiFine,
    NeoForge,
    Quilt,
    Custom(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GameDir {
    Default,
    Isolated,
    Custom(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum JvmMemory {
    Auto,
    Custom(u32),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdvancedProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jvm_args: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_args: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_game_integrity_check: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    #[serde(default = "default_icon")]
    pub icon: ProfileIcon,
    pub name: String,
    pub version_name: String,
    #[serde(default = "default_game_dir")]
    pub game_dir: GameDir,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jre_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jvm_memory: Option<JvmMemory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Resolution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fullscreen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_logs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced: Option<AdvancedProfile>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSettings {
    pub game_dir: GameDir,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProfileJson {
    pub last_profile: String,
    pub profiles: HashMap<String, Profile>,
}

fn default_icon() -> ProfileIcon {
    ProfileIcon::Furnace
}

fn default_game_dir() -> GameDir {
    GameDir::Default
}