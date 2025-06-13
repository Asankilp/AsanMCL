// 下载源
export enum DownloadSources {
    Mojang = 'mojang',
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
    host: string;
    port: number;
    enable_auth: boolean;
    username?: string;
    password?: string;
}

// 启动器配置
export interface LauncherConfig {
    last_game_path: string;
    close_after_launch: boolean;
    color_theme: ColorTheme;
    download_source: DownloadSources;
    enable_proxy: boolean;
    proxy: ProxyConfig;
    game_path: Record<string, string>;
}

// 默认值
export const defaultProxyConfig = (): ProxyConfig => ({
    type: ProxyType.Http,
    host: '',
    port: 0,
    enable_auth: false,
    username: undefined,
    password: undefined
});

export const defaultLauncherConfig = (): LauncherConfig => ({
    last_game_path: '当前目录',
    close_after_launch: false,
    color_theme: ColorTheme.FollowSystem,
    download_source: DownloadSources.Mojang,
    enable_proxy: false,
    proxy: defaultProxyConfig(),
    game_path: {} // 在实际使用时会从 Tauri 后端获取默认值
});
