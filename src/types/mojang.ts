export interface MinecraftProfile {
  id: string;              // UUID
  name: string;            // 玩家名称
  skins: SkinData[];       // 皮肤列表
  capes: CapeData[];       // 披风列表
}

export interface SkinData {
  id: string;
  state: string;
  url: string;
  variant: "classic" | "slim";
  alias?: string;
}

export interface CapeData {
  id: string;
  state: string;
  url: string;
  alias?: string;
}
