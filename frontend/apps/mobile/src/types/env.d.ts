// Environment variables type declarations for Expo

declare global {
  namespace NodeJS {
    interface ProcessEnv {
      EXPO_PUBLIC_API_BASE_URL?: string;
      BUILD_NUMBER?: string;
    }
  }

  var process: {
    env: NodeJS.ProcessEnv;
  };
}

export {};
