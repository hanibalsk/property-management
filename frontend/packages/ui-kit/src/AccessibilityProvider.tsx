/**
 * AccessibilityProvider component for managing accessibility settings.
 *
 * Provides context for accessibility settings and applies them to the DOM.
 */

import type { AccessibilitySettings } from '@ppt/shared';
import {
  DEFAULT_ACCESSIBILITY_SETTINGS,
  getColorSchemeClass,
  getTextSizeClass,
  prefersDarkColorScheme,
  prefersReducedMotion,
} from '@ppt/shared';
import type React from 'react';
import { createContext, useContext, useEffect, useState } from 'react';

// ============================================
// Context
// ============================================

interface AccessibilityContextValue {
  settings: AccessibilitySettings;
  updateSettings: (updates: Partial<AccessibilitySettings>) => void;
}

const AccessibilityContext = createContext<AccessibilityContextValue | undefined>(undefined);

// ============================================
// Hook
// ============================================

/**
 * Hook to access accessibility context.
 * Must be used within an AccessibilityProvider.
 */
export function useAccessibilityContext(): AccessibilityContextValue {
  const context = useContext(AccessibilityContext);
  if (!context) {
    throw new Error('useAccessibilityContext must be used within AccessibilityProvider');
  }
  return context;
}

// ============================================
// Storage
// ============================================

const STORAGE_KEY = 'ppt-accessibility-settings';

function loadSettings(): AccessibilitySettings {
  if (typeof window === 'undefined') return DEFAULT_ACCESSIBILITY_SETTINGS;

  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      const parsed = JSON.parse(stored);
      return { ...DEFAULT_ACCESSIBILITY_SETTINGS, ...parsed };
    }
  } catch (error) {
    console.error('Failed to load accessibility settings:', error);
  }

  // Use system preferences as defaults
  return {
    ...DEFAULT_ACCESSIBILITY_SETTINGS,
    colorScheme: prefersDarkColorScheme() ? 'dark' : 'light',
    reduceMotion: prefersReducedMotion(),
  };
}

function saveSettings(settings: AccessibilitySettings): void {
  if (typeof window === 'undefined') return;

  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
  } catch (error) {
    console.error('Failed to save accessibility settings:', error);
  }
}

// ============================================
// Provider Component
// ============================================

export interface AccessibilityProviderProps {
  children: React.ReactNode;
  initialSettings?: Partial<AccessibilitySettings>;
}

export const AccessibilityProvider: React.FC<AccessibilityProviderProps> = ({
  children,
  initialSettings,
}) => {
  const [settings, setSettings] = useState<AccessibilitySettings>(() => {
    const loaded = loadSettings();
    return initialSettings ? { ...loaded, ...initialSettings } : loaded;
  });

  // Update DOM when settings change
  useEffect(() => {
    const root = document.documentElement;

    // Apply color scheme
    root.setAttribute('data-color-scheme', settings.colorScheme);
    root.className = root.className
      .split(' ')
      .filter((c) => !c.startsWith('color-scheme-'))
      .concat(getColorSchemeClass(settings.colorScheme))
      .join(' ');

    // Apply text size
    root.setAttribute('data-text-size', settings.textSize);
    root.className = root.className
      .split(' ')
      .filter((c) => !c.startsWith('text-size-'))
      .concat(getTextSizeClass(settings.textSize))
      .join(' ');

    // Apply reduced motion
    if (settings.reduceMotion) {
      root.setAttribute('data-reduce-motion', 'true');
    } else {
      root.removeAttribute('data-reduce-motion');
    }

    // Apply screen reader mode
    if (settings.screenReaderEnabled) {
      root.setAttribute('data-screen-reader', 'true');
    } else {
      root.removeAttribute('data-screen-reader');
    }

    // Apply keyboard navigation mode
    if (settings.keyboardNavigationEnabled) {
      root.setAttribute('data-keyboard-nav', 'true');
    } else {
      root.removeAttribute('data-keyboard-nav');
    }

    // Save to localStorage
    saveSettings(settings);
  }, [settings]);

  // Listen for system preference changes
  useEffect(() => {
    if (typeof window === 'undefined') return;

    const darkModeQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const reducedMotionQuery = window.matchMedia('(prefers-reduced-motion: reduce)');

    const handleDarkModeChange = (e: MediaQueryListEvent) => {
      setSettings((prev: AccessibilitySettings) => ({
        ...prev,
        colorScheme: e.matches ? 'dark' : 'light',
      }));
    };

    const handleReducedMotionChange = (e: MediaQueryListEvent) => {
      setSettings((prev: AccessibilitySettings) => ({
        ...prev,
        reduceMotion: e.matches,
      }));
    };

    darkModeQuery.addEventListener('change', handleDarkModeChange);
    reducedMotionQuery.addEventListener('change', handleReducedMotionChange);

    return () => {
      darkModeQuery.removeEventListener('change', handleDarkModeChange);
      reducedMotionQuery.removeEventListener('change', handleReducedMotionChange);
    };
  }, []);

  const updateSettings = (updates: Partial<AccessibilitySettings>) => {
    setSettings((prev: AccessibilitySettings) => ({ ...prev, ...updates }));
  };

  const value: AccessibilityContextValue = {
    settings,
    updateSettings,
  };

  return <AccessibilityContext.Provider value={value}>{children}</AccessibilityContext.Provider>;
};

AccessibilityProvider.displayName = 'AccessibilityProvider';
