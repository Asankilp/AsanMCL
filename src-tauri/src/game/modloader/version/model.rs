use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FabricSupportedGameVersions {
    pub version: String,
    pub latest: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FabricLoaderVersions {
    pub seperator: String,
    pub build: u64,
    pub maven: String,
    pub version: String,
    pub stable: bool,
}