/**
 * SkipNavigation component for keyboard accessibility.
 *
 * Provides skip links to jump to main content, bypassing navigation.
 */

import type React from 'react';

export interface SkipNavigationProps {
  mainContentId?: string;
  navigationId?: string;
}

export const SkipNavigation: React.FC<SkipNavigationProps> = ({
  mainContentId = 'main-content',
  navigationId: _navigationId = 'main-navigation',
}) => {
  // Skip links allow keyboard users to bypass repetitive navigation content
  // The primary purpose is to jump directly to main content
  return (
    <div className="skip-navigation">
      <a href={`#${mainContentId}`} className="skip-link">
        Skip to main content
      </a>
    </div>
  );
};

SkipNavigation.displayName = 'SkipNavigation';
