/**
 * API configuration for the mobile app.
 *
 * Epic 49 - Story 49.1: Home Screen Widgets
 */

/**
 * Get the API base URL based on the environment.
 * In development, uses local server; in production, uses the configured endpoint.
 */
export function getApiBaseUrl(): string {
  // Check for environment variable first (react-native-config or similar)
  // @ts-expect-error - process.env is provided by metro bundler
  const envApiUrl = process.env.API_BASE_URL;
  if (envApiUrl) {
    return envApiUrl;
  }

  // Check for environment-specific configuration
  if (__DEV__) {
    // Android emulator uses 10.0.2.2 to reach host localhost
    // iOS simulator can use localhost directly
    const Platform = require('react-native').Platform;
    return Platform.OS === 'android' ? 'http://10.0.2.2:8080' : 'http://localhost:8080';
  }

  // Production API endpoint fallback
  return 'https://api.ppt.example.com';
}

/**
 * API configuration object.
 */
export const apiConfig = {
  get baseUrl(): string {
    return getApiBaseUrl();
  },
  timeout: 30000,
  retryAttempts: 3,
};
