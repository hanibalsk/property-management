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
 * NOTE: This is a stub implementation that does not persist settings.
 * For full functionality, use the `useAccessibilityContext` hook from
 * `@ppt/ui-kit` which requires an AccessibilityProvider wrapper.
 *
 * @deprecated Use useAccessibilityContext from @ppt/ui-kit instead
 * @returns Accessibility context with settings and update function (non-persisting)
 */
export function useAccessibility() {
  // IMPORTANT: This is a fallback stub. For actual usage, consumers should:
  // 1. Wrap their app with AccessibilityProvider from @ppt/ui-kit
  // 2. Use useAccessibilityContext from @ppt/ui-kit
  //
  // This stub exists only for type compatibility and testing scenarios.
  const settings: AccessibilitySettings = DEFAULT_ACCESSIBILITY_SETTINGS;

  const updateSettings = (_updates: Partial<AccessibilitySettings>) => {
    // Warn developers in development mode that this stub doesn't persist settings
    if (process.env.NODE_ENV === 'development') {
      console.warn(
        '[useAccessibility] This is a stub implementation that does not persist settings. ' +
          'Use useAccessibilityContext from @ppt/ui-kit with AccessibilityProvider instead.'
      );
    }
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
