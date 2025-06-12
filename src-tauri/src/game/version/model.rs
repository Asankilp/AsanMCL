use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

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
pub struct ClientJson {
    /// 版本名称
    pub id: String,
    #[serde(rename = "inheritsFrom")]
    /// 继承自的版本名称
    #[serde(default)]
    pub inherits_from: Option<String>,
    #[serde(rename = "releaseTime")]
    /// 版本发布时间，格式为 ISO 8601
    #[serde(default)]
    pub release_time: Option<String>,
    #[serde(rename = "time")]
    /// 版本发布时间，格式为 ISO 8601
    #[serde(default)]
    pub time: Option<String>,
    #[serde(rename = "type")]
    /// 版本类型，可能的值包括 "release"、"snapshot"、"old_alpha"、"old_beta"、"pre_release"、"experimental"、"debug"，以及其他自定义类型
    pub version_type: VersionType,
    #[serde(rename = "minimumLauncherVersion")]
    /// 最低启动器版本
    #[serde(default)]
    pub minimum_launcher_version: Option<u32>,
    #[serde(rename = "mainClass")]
    /// 主类名，通常为 "net.minecraft.client.main.Main"
    pub main_class: String,
    #[serde(rename = "javaVersion")]
    /// Java 版本信息
    #[serde(default)]
    pub java_version: Option<JavaVersion>,
    #[serde(rename = "complianceLevel")]
    /// 合规级别，1.16.4-pre2 之前为 0，之后的所有版本为 1
    #[serde(default)]
    pub compliance_level: Option<u32>,
    /// 启动参数
    #[serde(default)]
    pub arguments: Option<Arguments>,
    #[serde(rename = "assetIndex")]
    /// 资产索引信息
    #[serde(default)]
    pub asset_index: Option<AssetIndex>,
    /// 资产版本
    #[serde(default)]
    pub assets: Option<String>,
    /// 版本下载信息
    #[serde(default)]
    pub downloads: Option<Downloads>,
    /// 库列表
    #[serde(default)]
    pub libraries: Option<Vec<Library>>,
    /// Log4j 配置
    #[serde(default)]
    pub logging: Option<Logging>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JavaVersion {
    #[serde(default)]
    /// 官方启动器使用的 Java 运行环境名称，其值在 21w18a 之前为“jre-legacy”，在 1.18-pre1 之前为“java-runtime-alpha”，在 22w17a 之前为“java-runtime-beta”，在 24w13a 之前为“java-runtime-gamma”，自 24w14a 起为“java-runtime-delta”
    pub component: Option<String>,
    #[serde(rename = "majorVersion")]
    /// Java 版本号，通常为 8、16、17 或 21
    pub major_version: u32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Arguments {
    #[serde(default)]
    /// 游戏参数
    pub game: Option<Vec<Arg>>,
    #[serde(default)]
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
    pub action: String,
    #[serde(default)]
    /// 操作系统条件
    pub os: Option<OsRule>,
    #[serde(default)]
    /// 功能
    pub features: Option<HashMap<String, bool>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OsRule {
    #[serde(default)]
    /// 操作系统名称，可为 "windows"、"osx"、"linux"
    pub name: Option<String>,
    #[serde(default)]
    /// 操作系统版本
    pub version: Option<String>,
    #[serde(default)]
    /// 架构类型
    pub arch: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssetIndex {
    /// 资产版本
    pub id: String,
    /// 资产 SHA1 哈希值
    pub sha1: String,
    /// 资产大小
    pub size: u64,
    #[serde(rename = "totalSize")]
    #[serde(default)]
    /// 资产总大小
    pub total_size: Option<u64>,
    /// 资产 URL
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Downloads {
    /// client.jar 下载信息
    pub client: ArtifactFile,
    #[serde(rename = "client_mappings")]
    #[serde(default)]
    /// 客户端混淆映射表下载信息
    pub client_mappings: Option<ArtifactFile>,
    #[serde(default)]
    /// server.jar 下载信息
    pub server: Option<ArtifactFile>,
    #[serde(rename = "server_mappings")]
    #[serde(default)]
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
    pub name: String,
    #[serde(default)]
    /// 库的 Maven 仓库 URL
    pub url: Option<String>,
    #[serde(default)]
    /// 库的下载信息
    pub downloads: Option<DownloadsInfo>,
    #[serde(default)]
    /// 库的原生文件信息
    pub natives: Option<HashMap<String, String>>,
    #[serde(default)]
    /// 库的提取规则
    pub extract: Option<ExtractRule>,
    #[serde(default)]
    /// 库的规则
    pub rules: Option<Vec<Rule>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DownloadsInfo {
    /// 文件信息
    pub artifact: Option<ArtifactFile>,
    #[serde(default)]
    /// 分类器文件信息
    pub classifiers: Option<HashMap<String, ArtifactFile>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExtractRule {
    /// 排除的文件路径，其值为 "META-INF/"
    pub exclude: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Logging {
    pub client: LoggingClient,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LoggingClient {
    /// Log4j JVM 参数，其值为 "-Dlog4j.configurationFile=${path}"
    pub argument: String,
    #[serde(rename = "file")]
    /// Log4j XML 配置文件信息
    pub file: ArtifactFile,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ArtifactFile {
    #[serde(default)]
    /// 文件名
    pub id: Option<String>,
    /// 文件 SHA-1 哈希值
    pub sha1: String,
    /// 文件大小
    pub size: u64,
    /// 文件 URL
    pub url: String,
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
            r"C:\Users\asank\AppData\Roaming\.minecraft\versions\b1.8.1\b1.8.1.json"
        ];

        for file in test_files.iter() {
            let content = fs::read_to_string(file).expect("Failed to read file");
            let client_json: ClientJson = serde_json::from_str(&content).expect("Failed to parse JSON");
            println!("{:#?}", client_json);
        }
    }
}