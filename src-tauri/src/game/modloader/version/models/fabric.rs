use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FabricSupportedGameVersion {
    pub version: String,
    pub stable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FabricLoaderVersion {
    pub separator: String,
    pub build: u64,
    pub maven: String,
    pub version: String,
    pub stable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FabricIntermediaryVersion {
    pub version: String,
    pub stable: bool,
    pub maven: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FabricLibraryInfo {
    pub name: Option<String>,
    pub url: Option<String>,
    pub md5: Option<String>,
    pub sha1: Option<String>,
    pub sha256: Option<String>,
    pub sha512: Option<String>,
    pub size: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FabricMainClasses {
    pub client: String,
    pub server: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FabricLibraries {
    pub client: Option<Vec<FabricLibraryInfo>>,
    pub common: Option<Vec<FabricLibraryInfo>>,
    pub server: Option<Vec<FabricLibraryInfo>>,
    pub development: Option<Vec<FabricLibraryInfo>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FabricLauncherMeta {
    pub version: u64,
    pub min_java_version: Option<u64>,
    pub libraries: FabricLibraries,
    // #[serde(rename = "mainClass")]
    // pub main_class: FabricMainClass,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FabricLoaderVersionJson {
    pub loader: FabricLoaderVersion,
    pub intermediary: Option<FabricIntermediaryVersion>,
    #[serde(rename = "launcherMeta")]
    pub launcher_meta: Option<FabricLauncherMeta>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_fabric_loader_versions() {
        let url = "https://bmclapi2.bangbang93.com/fabric-meta/v2/versions/loader/1.20";
        let response = reqwest::blocking::get(url).unwrap();
        let result: Vec<FabricLoaderVersionJson> = response
            .json()
            .expect("Failed to parse Fabric loader versions");
        println!("{:#?}", result);
    }
}
