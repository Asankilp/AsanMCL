use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct GameAssetIndex {
    pub map_to_resources: Option<bool>,
    pub objects: HashMap<PathBuf, GameAsset>
} 

#[derive(Serialize, Deserialize, Debug)]
pub struct GameAsset {
    /// SHA-1 哈希值
    pub hash: String,
    /// 文件大小
    pub size: u64,
}