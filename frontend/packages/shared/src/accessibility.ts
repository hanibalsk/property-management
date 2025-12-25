/**
 * Accessibility types, constants, and hooks for Property Management apps.
 */

// ============================================
// Types
// ============================================

export type ColorScheme = 'light' | 'dark' | 'high-contrast';
export type TextSize = 'small' | 'medium' | 'large' | 'extra-large';

export interface AccessibilitySettings {
  colorScheme: ColorScheme;
  textSize: TextSize;
  reduceMotion: boolean;
  screenReaderEnabled: boolean;
  keyboardNavigationEnabled: boolean;
}

// ============================================
// Constants
// ============================================

export const COLOR_SCHEME_OPTIONS: { value: ColorScheme; label: string }[] = [
  { value: 'light', label: 'Light' },
  { value: 'dark', label: 'Dark' },
  { value: 'high-contrast', label: 'High Contrast' },
];

export const TEXT_SIZE_OPTIONS: { value: TextSize; label: string }[] = [
  { value: 'small', label: 'Small' },
  { value: 'medium', label: 'Medium' },
  { value: 'large', label: 'Large' },
  { value: 'extra-large', label: 'Extra Large' },
];

export const DEFAULT_ACCESSIBILITY_SETTINGS: AccessibilitySettings = {
  colorScheme: 'light',
  textSize: 'medium',
  reduceMotion: false,
  screenReaderEnabled: false,
  keyboardNavigationEnabled: true,
};

// ============================================
// Hooks
// ============================================

/**
 * Hook to access and update accessibility settings.
 *
 * @returns Accessibility context with settings and update function
 */
export function useAccessibility() {
  // This will be implemented by the AccessibilityProvider context
  // For now, return a basic implementation
  const settings: AccessibilitySettings = DEFAULT_ACCESSIBILITY_SETTINGS;

  const updateSettings = (updates: Partial<AccessibilitySettings>) => {
    console.warn(
      'useAccessibility: No AccessibilityProvider found. Settings not persisted.',
      updates
    );
  };

  return {
    settings,
    updateSettings,
  };
}

// ============================================
// Utilities
// ============================================

/**
 * Get CSS class name for text size.
 */
export function getTextSizeClass(size: TextSize): string {
  return `text-size-${size}`;
}

/**
 * Get CSS class name for color scheme.
 */
export function getColorSchemeClass(scheme: ColorScheme): string {
  return `color-scheme-${scheme}`;
}

/**
 * Check if user prefers reduced motion (from system settings).
 */
export function prefersReducedMotion(): boolean {
  if (typeof window === 'undefined') return false;
  return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
}

/**
 * Check if user prefers dark color scheme (from system settings).
 */
export function prefersDarkColorScheme(): boolean {
  if (typeof window === 'undefined') return false;
  return window.matchMedia('(prefers-color-scheme: dark)').matches;
}
