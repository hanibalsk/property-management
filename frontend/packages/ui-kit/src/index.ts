/**
 * UI Kit - Design system for Property Management apps.
 *
 * Shared components between web and mobile.
 */

// Placeholder - components will be added during implementation
export const VERSION = '0.1.0';

// Accessibility components
export { SkipNavigation } from './SkipNavigation';
export type { SkipNavigationProps } from './SkipNavigation';
export { AccessibilityProvider, useAccessibilityContext } from './AccessibilityProvider';
export type { AccessibilityProviderProps } from './AccessibilityProvider';

// Example component types (to be implemented)
export interface ButtonProps {
  variant: 'primary' | 'secondary' | 'danger';
  size: 'sm' | 'md' | 'lg';
  children: React.ReactNode;
  onClick?: () => void;
  disabled?: boolean;
}

export interface InputProps {
  type: 'text' | 'email' | 'password' | 'number';
  label: string;
  value: string;
  onChange: (value: string) => void;
  error?: string;
  disabled?: boolean;
}

// Design tokens
export const colors = {
  primary: '#3B82F6',
  secondary: '#6B7280',
  success: '#10B981',
  warning: '#F59E0B',
  danger: '#EF4444',
  background: '#FFFFFF',
  text: '#1F2937',
};

export const spacing = {
  xs: 4,
  sm: 8,
  md: 16,
  lg: 24,
  xl: 32,
};
