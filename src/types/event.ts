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
  path: string;
  progress: number;
  speed: number;
}

export interface DownloadError {
  error: string;
}

export type DownloadEvent =
  | {
    event: 'progress';
    data: {
      id: string;
      path: string;
      progress: number;
      speed: number;
    };
  }
  | {
    event: 'error';
    data: {
      error: string;
    };
  }
  | {
    event: 'finished';
  }