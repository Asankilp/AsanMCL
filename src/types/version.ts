// 定义版本类型枚举
export enum VersionType {
    Release = 'release',
    Snapshot = 'snapshot',
    OldAlpha = 'old_alpha',
    OldBeta = 'old_beta',
    PreRelease = 'pre_release',
    Experimental = 'experimental',
    Debug = 'debug',
    Other = 'other'
}

// 最新版本接口
export interface LatestVersion {
    /** 最新正式版 */
    release: string;
    /** 最新快照版 */
    snapshot: string;
}

// 版本信息接口
export interface VersionInfo {
    /** 版本号 */
    id: string;
    /** 版本类型 */
    type: VersionType;
    /** 版本发布时间，格式为 ISO 8601 */
    releaseTime: string;
    /** 版本更新时间，格式为 ISO 8601 */
    time: string;
    /** 版本的 SHA1 哈希值 */
    sha1?: string;
    /** 版本 JSON 文件的 URL */
    url: string;
    /** 版本的合规级别，1.16.4-pre2 之前为 0，之后的所有版本为 1 */
    complianceLevel?: number;
}

export interface LocalVersionInfo {
    name: string;
    info: ClientJson;
}

// 版本清单接口
export interface VersionManifest {
    /** 最新版本 */
    latest: LatestVersion;
    /** 版本列表的版本信息 */
    versions: VersionInfo[];
}

// 值或列表类型
export type ValueOrList = string | string[];

// 参数类型
export type Arg = string | {
    rules: Rule[];
    value: ValueOrList;
};

// 规则接口
export interface Rule {
    /** 动作，可为 "allow" 或 "disallow" */
    action?: string;
    /** 操作系统条件 */
    os?: OsRule;
    /** 功能 */
    features?: Record<string, boolean>;
}

// 操作系统规则接口
export interface OsRule {
    /** 操作系统名称，可为 "windows"、"osx"、"linux" */
    name?: string;
    /** 操作系统版本 */
    version?: string;
    /** 架构类型 */
    arch?: string;
}

// Java版本接口
export interface JavaVersion {
    /** 官方启动器使用的 Java 运行环境名称 */
    component?: string;
    /** Java 版本号，通常为 8、16、17 或 21 */
    majorVersion?: number;
}

// 启动参数接口
export interface Arguments {
    /** 游戏参数 */
    game?: Arg[];
    /** JVM 参数 */
    jvm?: Arg[];
}

// 资产索引接口
export interface AssetIndex {
    /** 资产版本 */
    id?: string;
    /** 资产 SHA1 哈希值 */
    sha1?: string;
    /** 资产大小 */
    size?: number;
    /** 资产总大小 */
    totalSize?: number;
    /** 资产 URL */
    url?: string;
}

// 下载文件接口
export interface ArtifactFile {
    /** 文件名 */
    id?: string;
    /** 文件路径 */
    path?: string;
    /** 文件 SHA-1 哈希值 */
    sha1?: string;
    /** 文件大小 */
    size?: number;
    /** 文件 URL */
    url?: string;
}

// 下载信息接口
export interface Downloads {
    /** client.jar 下载信息 */
    client?: ArtifactFile;
    /** 客户端混淆映射表下载信息 */
    client_mappings?: ArtifactFile;
    /** server.jar 下载信息 */
    server?: ArtifactFile;
    /** 服务端混淆映射表下载信息 */
    server_mappings?: ArtifactFile;
}

// 库下载信息接口
export interface DownloadsInfo {
    /** 文件信息 */
    artifact?: ArtifactFile;
    /** 分类器文件信息 */
    classifiers?: Record<string, ArtifactFile>;
}

// 提取规则接口
export interface ExtractRule {
    /** 排除的文件路径 */
    exclude?: string[];
}

// 库接口
export interface Library {
    /** 库的 Maven 名称，通常为 "groupId:artifactId:version" */
    name?: string;
    /** 库的 Maven 仓库 URL */
    url?: string;
    /** 库的下载信息 */
    downloads?: DownloadsInfo;
    /** 库的原生文件信息 */
    natives?: Record<string, string>;
    /** 库的提取规则 */
    extract?: ExtractRule;
    /** 库的规则 */
    rules?: Rule[];
}

// 日志客户端接口
export interface LoggingClient {
    /** Log4j JVM 参数 */
    argument?: string;
    /** Log4j XML 配置文件信息 */
    file?: ArtifactFile;
}

// 日志接口
export interface Logging {
    client?: LoggingClient;
}

// 客户端JSON配置接口
export interface ClientJson {
    /** 版本号 */
    id?: string;
    /** 继承自的版本号 */
    inheritsFrom?: string;
    /** 版本发布时间，格式为 ISO 8601 */
    releaseTime?: string;
    /** 版本发布时间，格式为 ISO 8601 */
    time?: string;
    /** 版本类型 */
    type?: VersionType;
    /** 最低启动器版本 */
    minimumLauncherVersion?: number;
    /** 主类名 */
    mainClass?: string;
    /** Java 版本信息 */
    javaVersion?: JavaVersion;
    /** 合规级别 */
    complianceLevel?: number;
    /** 启动参数 */
    arguments?: Arguments;
    /** 启动参数（1.13以前） */
    minecraftArguments?: string;
    /** 资产索引信息 */
    assetIndex?: AssetIndex;
    /** 资产版本 */
    assets?: string;
    /** 版本下载信息 */
    downloads?: Downloads;
    /** 库列表 */
    libraries?: Library[];
    /** Log4j 配置 */
    logging?: Logging;
}