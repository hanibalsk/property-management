/**
 * Vitest test setup file (Epic 80, Story 80.3)
 *
 * Configures testing environment:
 * - Jest DOM matchers for DOM assertions
 * - Cleanup after each test
 * - Mock implementations for browser APIs
 * - i18n mock for translation testing
 */

import '@testing-library/jest-dom';
import { cleanup } from '@testing-library/react';
import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import { afterEach } from 'vitest';

// Initialize i18n for tests with English translations
i18n.use(initReactI18next).init({
  lng: 'en',
  fallbackLng: 'en',
  resources: {
    en: {
      translation: {
        common: {
          notifications: 'Notifications',
          dismissNotification: 'Dismiss notification',
          copied: 'Copied',
          copyErrorMessage: 'Copy error message',
        },
      },
    },
  },
  interpolation: {
    escapeValue: false,
  },
});

// Cleanup after each test to prevent memory leaks
afterEach(() => {
  cleanup();
});

// Mock window.matchMedia for components using media queries
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: (query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: () => {},
    removeListener: () => {},
    addEventListener: () => {},
    removeEventListener: () => {},
    dispatchEvent: () => false,
  }),
});

// Mock ResizeObserver for components using it
(globalThis as Record<string, unknown>).ResizeObserver = class ResizeObserver {
  observe() {}
  unobserve() {}
  disconnect() {}
};

// Mock IntersectionObserver for lazy loading components
(globalThis as Record<string, unknown>).IntersectionObserver = class IntersectionObserver {
  root = null;
  rootMargin = '';
  thresholds: number[] = [];

  observe() {}
  unobserve() {}
  disconnect() {}
  takeRecords(): IntersectionObserverEntry[] {
    return [];
  }
};
