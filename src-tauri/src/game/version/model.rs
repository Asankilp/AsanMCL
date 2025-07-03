use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
    #[default]
    Release,
    Snapshot,
    OldAlpha,
    OldBeta,
    PreRelease,
    Experimental,
    Debug,
    Other(String),
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LatestVersion {
    /// 最新正式版
    pub release: String,
    /// 最新快照版
    pub snapshot: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocalVersionInfo {
    /// 版本名
    pub name: String,
    /// 版本信息
    pub info: ClientJson,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VersionInfo {
    /// 版本号
    pub id: String,
    /// 版本类型
    #[serde(rename = "type")]
    pub version_type: VersionType,
    #[serde(rename = "releaseTime")]
    /// 版本发布时间，格式为 ISO 8601
    pub release_time: String,
    /// 版本更新时间，格式为 ISO 8601
    pub time: String,
    /// 版本的 SHA1 哈希值
    pub sha1: Option<String>,
    /// 版本 JSON 文件的 URL
    pub url: String,
    #[serde(rename = "complianceLevel")]
    /// 版本的合规级别，1.16.4-pre2 之前为 0，之后的所有版本为 1
    pub compliance_level: Option<u32>,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VersionManifest {
    /// 最新版本
    pub latest: LatestVersion,
    /// 版本列表的版本信息
    pub versions: Vec<VersionInfo>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClientJson {
    /// 版本号
    pub id: Option<String>,
    #[serde(rename = "inheritsFrom")]
    /// 继承自的版本号
    pub inherits_from: Option<String>,
    #[serde(rename = "releaseTime")]
    /// 版本发布时间，格式为 ISO 8601
    pub release_time: Option<String>,
    #[serde(rename = "time")]
    /// 版本发布时间，格式为 ISO 8601
    pub time: Option<String>,
    #[serde(rename = "type")]
    /// 版本类型，可能的值包括 "release"、"snapshot"、"old_alpha"、"old_beta"、"pre_release"、"experimental"、"debug"，以及其他自定义类型
    pub version_type: Option<VersionType>,
    #[serde(rename = "minimumLauncherVersion")]
    /// 最低启动器版本
    pub minimum_launcher_version: Option<u32>,
    #[serde(rename = "mainClass")]
    /// 主类名，通常为 "net.minecraft.client.main.Main"
    pub main_class: Option<String>,
    #[serde(rename = "javaVersion")]
    /// Java 版本信息
    pub java_version: Option<JavaVersion>,
    #[serde(rename = "complianceLevel")]
    /// 合规级别，1.16.4-pre2 之前为 0，之后的所有版本为 1
    pub compliance_level: Option<u32>,
    /// 启动参数
    pub arguments: Option<Arguments>,
    #[serde(rename = "minecraftArguments")]
    /// 启动参数（1.13以前）
    pub minecraft_arguments: Option<String>,
    #[serde(rename = "assetIndex")]
    /// 资产索引信息
    pub asset_index: Option<AssetIndex>,
    /// 资产版本
    pub assets: Option<String>,
    /// 版本下载信息
    pub downloads: Option<Downloads>,
    /// 库列表
    pub libraries: Option<Vec<Library>>,
    /// Log4j 配置
    pub logging: Option<Logging>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JavaVersion {
    /// 官方启动器使用的 Java 运行环境名称，其值在 21w18a 之前为“jre-legacy”，在 1.18-pre1 之前为“java-runtime-alpha”，在 22w17a 之前为“java-runtime-beta”，在 24w13a 之前为“java-runtime-gamma”，自 24w14a 起为“java-runtime-delta”
    pub component: Option<String>,
    #[serde(rename = "majorVersion")]
    /// Java 版本号，通常为 8、16、17 或 21
    pub major_version: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Arguments {
    /// 游戏参数
    pub game: Option<Vec<Arg>>,
    /// JVM 参数
    pub jvm: Option<Vec<Arg>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Arg {
    Value(String),
    Conditional {
        rules: Vec<Rule>,
        value: ValueOrList,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueOrList {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Rule {
    /// 动作，可为 "allow" 或 "disallow"
    pub action: Option<String>,
    /// 操作系统条件
    pub os: Option<OsRule>,
    /// 功能
    pub features: Option<HashMap<String, bool>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OsRule {
    /// 操作系统名称，可为 "windows"、"osx"、"linux"
    pub name: Option<String>,
    /// 操作系统版本，通过System.getProperty("os.version")获取
    pub version: Option<String>,
    /// 架构类型
    pub arch: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssetIndex {
    /// 资产版本
    pub id: Option<String>,
    /// 资产 SHA1 哈希值
    pub sha1: Option<String>,
    /// 资产大小
    pub size: Option<u64>,
    #[serde(rename = "totalSize")]
    /// 资产总大小
    pub total_size: Option<u64>,
    /// 资产 URL
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Downloads {
    /// client.jar 下载信息
    pub client: Option<ArtifactFile>,
    #[serde(rename = "client_mappings")]
    /// 客户端混淆映射表下载信息
    pub client_mappings: Option<ArtifactFile>,
    /// server.jar 下载信息
    pub server: Option<ArtifactFile>,
    #[serde(rename = "server_mappings")]
    /// 服务端混淆映射表下载信息
    pub server_mappings: Option<ArtifactFile>,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Artifact {
//     /// 文件 SHA1 哈希值
//     pub sha1: String,
//     /// 文件大小
//     pub size: u64,
//     /// 文件 URL
//     pub url: String,
// }

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Library {
    /// 库的 Maven 名称，通常为 "groupId:artifactId:version"
    pub name: Option<String>,
    /// 库的 Maven 仓库 URL
    pub url: Option<String>,
    /// 库的下载信息
    pub downloads: Option<DownloadsInfo>,
    /// 库的原生文件信息
    pub natives: Option<HashMap<String, String>>,
    /// 库的提取规则
    pub extract: Option<ExtractRule>,
    /// 库的规则
    pub rules: Option<Vec<Rule>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DownloadsInfo {
    /// 文件信息
    pub artifact: Option<ArtifactFile>,
    /// 分类器文件信息
    pub classifiers: Option<HashMap<String, ArtifactFile>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExtractRule {
    /// 排除的文件路径，其值为 "META-INF/"
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Logging {
    pub client: Option<LoggingClient>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LoggingClient {
    /// Log4j JVM 参数，其值为 "-Dlog4j.configurationFile=${path}"
    pub argument: Option<String>,
    #[serde(rename = "file")]
    /// Log4j XML 配置文件信息
    pub file: Option<ArtifactFile>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ArtifactFile {
    #[serde(default)]
    /// 文件名
    pub id: Option<String>,
    /// 文件路径
    pub path: Option<PathBuf>,
    /// 文件 SHA-1 哈希值
    pub sha1: Option<String>,
    /// 文件大小
    pub size: Option<u64>,
    /// 文件 URL
    pub url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_client_json() {
        let test_files = [
            r"C:\Users\asank\AppData\Roaming\.minecraft\versions\fabric-loader-0.16.14-1.21.5\fabric-loader-0.16.14-1.21.5.json",
            r"C:\Users\asank\AppData\Roaming\.minecraft\versions\1.21.5\1.21.5.json",
            r"C:\Users\asank\AppData\Roaming\.minecraft\versions\1.7.10\1.7.10.json",
            r"C:\Users\asank\AppData\Roaming\.minecraft\versions\b1.8.1\b1.8.1.json",
            r"C:\Users\asank\AppData\Roaming\.minecraft\versions\24w14potato\24w14potato.json",
            r"C:\Users\asank\AppData\Roaming\.minecraft\versions\1.7.10-Forge10.13.4.1614-1.7.10\1.7.10-Forge10.13.4.1614-1.7.10.json",
            r"C:\Users\asank\AppData\Roaming\.minecraft\versions\1.20.1-OptiFine_HD_U_I6\1.20.1-OptiFine_HD_U_I6.json",
            r"C:\Users\asank\AppData\Roaming\.minecraft\versions\neoforge-21.1.172\neoforge-21.1.172.json",
            r"C:\Users\asank\AppData\Roaming\.minecraft\versions\1.21.6-rc1\1.21.6-rc1.json",
            r"C:\Users\asank\AppData\Roaming\.minecraft\versions\1.20.2-forge-48.1.0\1.20.2-forge-48.1.0.json",
            r"C:\Users\asank\AppData\Roaming\.minecraft\versions\25w21a\25w21a.json",
            r"C:\Users\asank\AppData\Roaming\.minecraft\versions\25w14craftmine\25w14craftmine.json",
            r"C:\Users\asank\AppData\Roaming\.minecraft\versions\rd-132211\rd-132211.json",
        ];

        for file in test_files.iter() {
            let content = std::fs::read_to_string(file).expect("Failed to read file");
            let client_json: ClientJson =
                serde_json::from_str(&content).expect("Failed to parse JSON");
            println!("{:#?}", client_json);
        }
    }

    #[test]
    fn test_parse_version_manifest() {
        let resp = reqwest::blocking::get(
            "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json",
        )
        .expect("Failed to get version manifest");
        let content = resp.text().expect("Failed to read response");
        let version_manifest: VersionManifest =
            serde_json::from_str(&content).expect("Failed to parse JSON");
        println!("{:#?}", version_manifest);
    }
}
