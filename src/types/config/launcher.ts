// 下载源
export enum DownloadSources {
    Mojang = 'Mojang',
    BmclApi = 'BmclApi'
}

// 代理类型
export enum ProxyType {
    Http = 'Http',
    Socks5 = 'Socks5'
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
    download_source: DownloadSources.Mojang,
    enable_proxy: false,
    proxy: defaultProxyConfig(),
    game_path: {} // 在实际使用时会从 Tauri 后端获取默认值
});
