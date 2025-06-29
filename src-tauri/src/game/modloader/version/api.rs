use crate::config::model::DownloadSource;
use crate::game::modloader::version::models::fabric::{FabricLoaderVersionJson, FabricSupportedGameVersions};

#[derive(serde::Deserialize)]
pub struct BmclApiError {
    #[serde(rename = "httpCode")]
    pub http_code: Option<u16>,
    pub message: Option<String>,
}
pub async fn get_fabric_supported_game_versions(
    download_source: DownloadSource,
) -> Result<Vec<FabricSupportedGameVersions>, String> {
    let url = match download_source {
        DownloadSource::Official => "https://meta.fabricmc.net/v2/versions/game",
        DownloadSource::BmclApi => "https://bmclapi2.bangbang93.com/fabric-meta/v2/versions/game",
    };
    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;

    let versions: Vec<FabricSupportedGameVersions> =
        response.json().await.map_err(|e| e.to_string())?;

    Ok(versions)
}

pub async fn get_fabric_loader_versions_by_game_version(game_version: String, download_source: DownloadSource) -> Result<Vec<String>, String> {
    let url = match download_source {
        DownloadSource::Official => format!("https://meta.fabricmc.net/v2/versions/loader/{}", game_version),
        DownloadSource::BmclApi => format!("https://bmclapi2.bangbang93.com/fabric-meta/v2/versions/loader/{}", game_version),
    };
    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;

    let text = response.text().await.map_err(|e| e.to_string())?;
    match download_source {
        DownloadSource::Official => {
            if text.trim() == "[]" {
                return Err("不支持该游戏版本".to_string());
            }
        }
        DownloadSource::BmclApi => {
            if let Ok(err_obj) = serde_json::from_str::<BmclApiError>(&text) {
                if err_obj.http_code == Some(404) {
                    return Err("不支持该游戏版本".to_string());
                }
            }
        }
    }
    let versions: Vec<FabricLoaderVersionJson> = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let versions_list = versions
        .into_iter()
        .map(|v| v.loader.version)
        .collect::<Vec<String>>();

    Ok(versions_list)
}

pub async fn get_forge_supported_game_versions() -> Result<Vec<String>, String> {
    let url = "https://bmclapi2.bangbang93.com/forge/minecraft";
    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
    let versions: Vec<String> = response.json().await.map_err(|e| e.to_string())?;
    Ok(versions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_fabric_supported_game_versions() {
        let versions = get_fabric_supported_game_versions(DownloadSource::Official).await;
        println!("{:?}", versions);
    }
}
