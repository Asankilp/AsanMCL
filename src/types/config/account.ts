export enum AccountType {
  Microsoft = 'microsoft',
  Offline = 'offline',
  External = 'external',
}

export interface AccountInfo {
    accountType: AccountType;
    name: string; // 玩家名
    uuid: string; // UUID
    accessToken?: string; // 访问令牌
    refreshToken?: string; // 刷新令牌
    userId?: string; // 用户 ID
    expiresIn?: number; // 令牌过期时间
}

export interface AccountConfig {
    accounts: AccountInfo[];
}