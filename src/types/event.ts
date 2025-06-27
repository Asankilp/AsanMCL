import { MinecraftAuthResponse } from "./account";

export type LoginEvent =
  | {
    event: 'started',
    data: {
      code: string;
    };
  }
  | {
    event: 'finished',
    data: {
      response: MinecraftAuthResponse;
    };
  }

export interface DownloadProgress {
  id: string;
  filename: string;
  progress: number;
}

export interface DownloadError {
  error: string;
}