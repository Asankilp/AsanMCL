// 游戏资源索引结构体
export interface GameAssetIndex {
  map_to_resources?: boolean;
  objects: Record<string, GameAsset>;
}

// 游戏资源对象结构体
export interface GameAsset {
  /** SHA-1 哈希值 */
  hash: string;
  /** 文件大小 */
  size: number;
}
