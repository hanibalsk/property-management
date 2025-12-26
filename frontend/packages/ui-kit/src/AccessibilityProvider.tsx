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
const EXPLICIT_SETTINGS_KEY = 'ppt-accessibility-explicit-settings';

/**
 * Tracks which settings have been explicitly set by the user (vs. system defaults).
 * Keys must match AccessibilitySettings interface properties.
 */
type ExplicitSettings = {
  [K in keyof AccessibilitySettings]?: boolean;
};

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

function loadExplicitSettings(): ExplicitSettings {
  if (typeof window === 'undefined') return {};

  try {
    const stored = localStorage.getItem(EXPLICIT_SETTINGS_KEY);
    if (stored) {
      return JSON.parse(stored);
    }
  } catch (error) {
    console.error('Failed to load explicit settings:', error);
  }

  return {};
}

function saveSettings(settings: AccessibilitySettings): void {
  if (typeof window === 'undefined') return;

  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
  } catch (error) {
    console.error('Failed to save accessibility settings:', error);
  }
}

function saveExplicitSettings(explicit: ExplicitSettings): void {
  if (typeof window === 'undefined') return;

  try {
    localStorage.setItem(EXPLICIT_SETTINGS_KEY, JSON.stringify(explicit));
  } catch (error) {
    console.error('Failed to save explicit settings:', error);
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

  const [explicitSettings, setExplicitSettings] = useState<ExplicitSettings>(() => {
    return loadExplicitSettings();
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

  // Save explicit settings when they change
  useEffect(() => {
    saveExplicitSettings(explicitSettings);
  }, [explicitSettings]);

  // Listen for system preference changes
  // Only apply system changes if user hasn't explicitly set the preference
  useEffect(() => {
    if (typeof window === 'undefined') return;

    const darkModeQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const reducedMotionQuery = window.matchMedia('(prefers-reduced-motion: reduce)');

    const handleDarkModeChange = (e: MediaQueryListEvent) => {
      // Only apply system preference if user hasn't explicitly set color scheme
      if (!explicitSettings.colorScheme) {
        setSettings((prev: AccessibilitySettings) => ({
          ...prev,
          colorScheme: e.matches ? 'dark' : 'light',
        }));
      }
    };

    const handleReducedMotionChange = (e: MediaQueryListEvent) => {
      // Only apply system preference if user hasn't explicitly set reduced motion
      if (!explicitSettings.reduceMotion) {
        setSettings((prev: AccessibilitySettings) => ({
          ...prev,
          reduceMotion: e.matches,
        }));
      }
    };

    darkModeQuery.addEventListener('change', handleDarkModeChange);
    reducedMotionQuery.addEventListener('change', handleReducedMotionChange);

    return () => {
      darkModeQuery.removeEventListener('change', handleDarkModeChange);
      reducedMotionQuery.removeEventListener('change', handleReducedMotionChange);
    };
  }, [explicitSettings]);

  const updateSettings = (updates: Partial<AccessibilitySettings>) => {
    setSettings((prev: AccessibilitySettings) => ({ ...prev, ...updates }));

    // Mark updated settings as explicitly set by the user
    // Use type-safe iteration over the keys that were actually provided
    const newExplicitSettings: ExplicitSettings = {};
    for (const key of Object.keys(updates)) {
      if (key in DEFAULT_ACCESSIBILITY_SETTINGS) {
        const validKey = key as keyof AccessibilitySettings;
        newExplicitSettings[validKey] = true;
      }
    }
    setExplicitSettings((prev) => ({ ...prev, ...newExplicitSettings }));
  };

  const value: AccessibilityContextValue = {
    settings,
    updateSettings,
  };

  return <AccessibilityContext.Provider value={value}>{children}</AccessibilityContext.Provider>;
};

AccessibilityProvider.displayName = 'AccessibilityProvider';
