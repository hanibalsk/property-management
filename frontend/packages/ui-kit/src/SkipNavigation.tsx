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
  navigationId = 'main-navigation',
}) => {
  return (
    <div className="skip-navigation">
      <a href={`#${mainContentId}`} className="skip-link">
        Skip to main content
      </a>
      <a href={`#${navigationId}`} className="skip-link">
        Skip to navigation
      </a>
    </div>
  );
};

SkipNavigation.displayName = 'SkipNavigation';
