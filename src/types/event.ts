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