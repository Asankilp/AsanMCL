use crate::{
    config::model::DownloadSource, game::modloader::version::model::FabricSupportedGameVersions,
};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_fabric_supported_game_versions() {
        let versions = get_fabric_supported_game_versions(DownloadSource::Official).await;
        println!("{:?}", versions);
    }
}
