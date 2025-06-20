use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameAssetIndex {
    pub map_to_resources: Option<bool>,
    pub objects: HashMap<PathBuf, GameAsset>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameAsset {
    /// SHA-1 哈希值
    pub hash: String,
    /// 文件大小
    pub size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_asset_index() {
        let asset_index_files = [
            r"C:\Users\asank\AppData\Roaming\.minecraft\assets\indexes\26.json",
            r"C:\Users\asank\AppData\Roaming\.minecraft\assets\indexes\1.7.10.json",
            r"C:\Users\asank\AppData\Roaming\.minecraft\assets\indexes\1.19.json",
            r"C:\Users\asank\AppData\Roaming\.minecraft\assets\indexes\pre-1.6.json",
            r"C:\Users\asank\AppData\Roaming\.minecraft\assets\indexes\3.json",
        ];
        for i in asset_index_files.iter() {
            let content = std::fs::read_to_string(i).expect("Failed to read asset index file");
            let asset_index: GameAssetIndex =
                serde_json::from_str(&content).expect("Failed to parse asset index JSON");
            assert!(
                !asset_index.objects.is_empty(),
                "Asset index should not be empty"
            );
            println!("parse asset index file {} successfully", i);
        }
    }
}
