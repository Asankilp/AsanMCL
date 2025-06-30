// 下载源
export enum DownloadSource {
    Official = 'official',
    BmclApi = 'bmclapi'
}

// 代理类型
export enum ProxyType {
    Http = 'http',
    Socks5 = 'socks5'
}

// 颜色主题
export enum ColorTheme {
    Light = 'light',
    Dark = 'dark',
    FollowSystem = 'follow_system'
}

// 代理配置
export interface ProxyConfig {
    type: ProxyType;
    host?: string;
    enableAuth: boolean;
    username?: string;
    password?: string;
}

// 启动器配置
export interface LauncherConfig {
    lastGamePath: string;
    selectedAccount?: string; // 选中的账号
    closeAfterLaunch: boolean;
    colorTheme: ColorTheme;
    downloadSource: DownloadSource;
    enableProxy: boolean;
    proxy: ProxyConfig;
    gamePath: Record<string, string>;
}

// 默认值
export const defaultProxyConfig = (): ProxyConfig => ({
    type: ProxyType.Http,
    host: undefined,
    enableAuth: false,
    username: undefined,
    password: undefined
});

export const defaultLauncherConfig = (): LauncherConfig => ({
    lastGamePath: '当前目录',
    closeAfterLaunch: false,
    colorTheme: ColorTheme.FollowSystem,
    downloadSource: DownloadSource.Official,
    enableProxy: false,
    proxy: defaultProxyConfig(),
    gamePath: {} // 在实际使用时会从 Tauri 后端获取默认值
});
