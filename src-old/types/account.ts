export interface CustomAccountData {
  username: string
  email: string
  password: string
}

export interface OfflineAccountData {
  playerName: string
  uuid?: string
}

export interface UserCodeResult {
    verificationUri: string,
    userCode: string,
}

export interface MinecraftAuthResponse {
    userId: string,
    accessToken: string,
    tokenType: string,
    expiresIn: number,
}