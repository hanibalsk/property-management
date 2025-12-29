/**
 * Environment variables type declarations for Expo.
 *
 * Epic 85 - Story 85.1: Environment Variable Setup
 */

declare global {
  namespace NodeJS {
    interface ProcessEnv {
      /** API base URL for backend communication */
      EXPO_PUBLIC_API_BASE_URL?: string;
      /** WebSocket base URL for real-time communication */
      EXPO_PUBLIC_WS_BASE_URL?: string;
      /** Current environment: development, staging, or production */
      EXPO_PUBLIC_ENVIRONMENT?: 'development' | 'staging' | 'production';
      /** Enable debug mode features */
      EXPO_PUBLIC_DEBUG_MODE?: string;
      /** Build number for versioning */
      BUILD_NUMBER?: string;
    }
  }

  var process: {
    env: NodeJS.ProcessEnv;
  };
}

export {};
