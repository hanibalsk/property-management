/**
 * App configuration constants.
 *
 * This file provides a centralized location for app-wide constants including
 * version info and API endpoints.
 */

/**
 * App version from package.json
 * Note: In production, this should be synchronized with package.json via build process.
 * For now, updated manually or via version bump script.
 */
export const APP_VERSION = '0.2.96';

/**
 * Build number (can be overridden by CI/CD via environment variable)
 */
export const BUILD_NUMBER = process.env.BUILD_NUMBER ?? '1';

/**
 * API base URL from environment variable or default
 * Set EXPO_PUBLIC_API_BASE_URL in .env file for custom endpoints
 */
export const API_BASE_URL = process.env.EXPO_PUBLIC_API_BASE_URL ?? 'https://api.ppt.example.com';

/**
 * Default constants
 */
export const CONSTANTS = {
  APP_VERSION,
  BUILD_NUMBER,
  API_BASE_URL,
} as const;
