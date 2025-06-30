use crate::config::model::DownloadSource;
use crate::game::modloader::version::models::fabric::{
    FabricLoaderVersionJson, FabricSupportedGameVersion,
};
use crate::util::reqwest_client::REQWEST_CLIENT;

// #[derive(serde::Deserialize)]
// pub struct BmclApiError {
//     #[serde(rename = "httpCode")]
//     pub http_code: Option<u16>,
//     pub message: Option<String>,
// }
pub async fn get_fabric_supported_game_versions(
    download_source: DownloadSource,
) -> Result<Vec<FabricSupportedGameVersion>, String> {
    let url = match download_source {
        DownloadSource::Official => "https://meta.fabricmc.net/v2/versions/game",
        DownloadSource::BmclApi => "https://bmclapi2.bangbang93.com/fabric-meta/v2/versions/game",
    };
    let client = {
        let guard = REQWEST_CLIENT.lock().await;
        match &*guard {
            Some(c) => c.clone(),
            None => return Err("HTTP客户端未初始化".to_string()),
        }
    };
    let response = client.get(url).send().await.map_err(|e| e.to_string())?;
    let versions: Vec<FabricSupportedGameVersion> =
        response.json().await.map_err(|e| e.to_string())?;

    Ok(versions)
}

pub async fn get_fabric_loader_versions_by_game_version(
    game_version: String,
    download_source: DownloadSource,
) -> Result<Vec<String>, String> {
    // 先判断是否支持该游戏版本
    let supported_versions = get_fabric_supported_game_versions(download_source.clone()).await?;
    let is_supported = supported_versions.iter().any(|v| v.version == game_version);
    if !is_supported {
        return Err("不支持该游戏版本".to_string());
    }
    let url = match download_source {
        DownloadSource::Official => format!(
            "https://meta.fabricmc.net/v1/versions/loader/{}",
            game_version
        ),
        DownloadSource::BmclApi => format!(
            "https://bmclapi2.bangbang93.com/fabric-meta/v2/versions/loader/{}",
            game_version
        ), // BMCLAPI 不支持 v1 格式
    };
    let client = {
        let guard = REQWEST_CLIENT.lock().await;
        match &*guard {
            Some(c) => c.clone(),
            None => return Err("HTTP客户端未初始化".to_string()),
        }
    };
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    let versions: Vec<FabricLoaderVersionJson> =
        response.json().await.map_err(|e| e.to_string())?;
    let versions_list = versions
        .into_iter()
        .map(|v| v.loader.version)
        .collect::<Vec<String>>();

    Ok(versions_list)
}

pub async fn get_forge_supported_game_versions() -> Result<Vec<String>, String> {
    let url = "https://bmclapi2.bangbang93.com/forge/minecraft";
    let client = {
        let guard = REQWEST_CLIENT.lock().await;
        match &*guard {
            Some(c) => c.clone(),
            None => return Err("HTTP客户端未初始化".to_string()),
        }
    };
    let response = client.get(url).send().await.map_err(|e| e.to_string())?;
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
