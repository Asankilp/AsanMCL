// ProfileIcon 枚举
export type ProfileIcon =
  | 'furnace'
  | 'grassblock'
  | 'chest'
  | 'dirt'
  | 'cobblestone'
  | 'anvil'
  | 'forge'
  | 'fabric'
  | 'liteloader'
  | 'optifine'
  | 'neoforge'
  | 'quilt'
  | { custom: string };

// GameDir 枚举
export type GameDir = 'default' | 'isolated' | { custom: string };

// 分辨率结构体
export interface Resolution {
  width: number;
  height: number;
}

// JvmMemory 枚举
export type JvmMemory = 'auto' | { custom: number };

// 高级配置结构体
export interface AdvancedProfile {
  jvmArgs?: string;
  gameArgs?: string;
  wrapCommand?: string;
  skipGameIntegrityCheck?: boolean;
}

// Profile 结构体
export interface Profile {
  icon: ProfileIcon;
  name: string;
  versionName: string;
  gameDir: GameDir;
  jrePath?: string;
  jvmMemory?: JvmMemory;
  resolution?: Resolution;
  fullscreen?: boolean;
  showLogs?: boolean;
  serverAddress?: string;
  advanced?: AdvancedProfile;
}

// ProfileSettings 结构体
export interface ProfileSettings {
  gameDir: GameDir;
}

// ProfileJson 结构体
export interface ProfileJson {
  lastProfile: string;
  profiles: Record<string, Profile>;
}
