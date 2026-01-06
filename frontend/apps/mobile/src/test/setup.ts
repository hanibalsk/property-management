/**
 * Jest test setup file for Mobile App
 *
 * Configures testing environment:
 * - Mock implementations for React Native APIs
 * - Mock implementations for Expo modules
 * - i18next mock for translation testing
 */

import '@testing-library/react-native/extend-expect';

// Mock react-i18next
jest.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => key,
    i18n: {
      language: 'en',
      changeLanguage: jest.fn(),
    },
  }),
  initReactI18next: {
    type: '3rdParty',
    init: jest.fn(),
  },
}));

// Mock i18n config
jest.mock('../i18n', () => ({
  locales: ['en', 'sk', 'cs', 'de'],
  localeNames: {
    en: 'English',
    sk: 'Slovenčina',
    cs: 'Čeština',
    de: 'Deutsch',
  },
  localeFlags: {
    en: '🇬🇧',
    sk: '🇸🇰',
    cs: '🇨🇿',
    de: '🇩🇪',
  },
}));

// Mock Expo modules
jest.mock('expo-secure-store', () => ({
  getItemAsync: jest.fn(),
  setItemAsync: jest.fn(),
  deleteItemAsync: jest.fn(),
}));

jest.mock('expo-constants', () => ({
  expoConfig: {
    extra: {
      apiUrl: 'http://localhost:8080',
    },
  },
}));

jest.mock('expo-localization', () => ({
  locale: 'en-US',
  locales: ['en-US'],
  timezone: 'America/New_York',
  isRTL: false,
  getLocales: () => [{ languageCode: 'en', countryCode: 'US' }],
}));

jest.mock('@react-native-async-storage/async-storage', () => ({
  getItem: jest.fn(),
  setItem: jest.fn(),
  removeItem: jest.fn(),
  clear: jest.fn(),
}));

// Suppress console warnings in tests
const originalConsoleError = console.error;
console.error = (...args) => {
  if (
    typeof args[0] === 'string' &&
    (args[0].includes('Warning: ReactDOM.render is no longer supported') ||
      args[0].includes('Warning: An update to') ||
      args[0].includes('Warning: `ReactDOMTestUtils.act`'))
  ) {
    return;
  }
  originalConsoleError(...args);
};
