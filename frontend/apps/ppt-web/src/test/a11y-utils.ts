/**
 * Accessibility testing utilities for vitest-axe
 * Provides reusable test helpers for component accessibility validation.
 *
 * @module test/a11y-utils
 */

import type { AxeResults, RunOptions } from 'axe-core';
import { axe } from 'vitest-axe';

/**
 * Default axe-core rules configuration optimized for WCAG 2.1 AA compliance
 */
export const defaultA11yRules: RunOptions = {
  rules: {
    // WCAG 2.1 Level AA rules
    'color-contrast': { enabled: true },
    'valid-lang': { enabled: true },
    'landmark-one-main': { enabled: true },
    'page-has-heading-one': { enabled: true },
    bypass: { enabled: true },
    // Disable rules that may cause false positives in component tests
    region: { enabled: false }, // Components may not have landmark regions
  },
};

/**
 * Run axe accessibility check on a container with custom options
 * @param container - The DOM container to test
 * @param options - Optional axe-core run options
 * @returns AxeResults from the accessibility scan
 */
export async function runA11yCheck(
  container: HTMLElement,
  options: RunOptions = {}
): Promise<AxeResults> {
  return axe(container, {
    ...defaultA11yRules,
    ...options,
  });
}

/**
 * Format axe violations for readable error output
 * @param violations - Array of axe violations
 * @returns Formatted string of violations
 */
export function formatViolations(violations: AxeResults['violations']): string {
  if (violations.length === 0) {
    return 'No accessibility violations found';
  }

  return violations
    .map((violation: AxeResults['violations'][number]) => {
      const nodeInfo = violation.nodes
        .map(
          (node: AxeResults['violations'][number]['nodes'][number]) =>
            `  - ${node.html}\n    Fix: ${node.failureSummary}`
        )
        .join('\n');

      return `
[${violation.impact?.toUpperCase()}] ${violation.id}: ${violation.description}
Help: ${violation.helpUrl}
Affected elements:
${nodeInfo}`;
    })
    .join('\n\n');
}

/**
 * Check if results have critical or serious violations
 * @param results - AxeResults from accessibility scan
 * @returns true if there are critical or serious violations
 */
export function hasCriticalViolations(results: AxeResults): boolean {
  return results.violations.some(
    (v: AxeResults['violations'][number]) => v.impact === 'critical' || v.impact === 'serious'
  );
}

/**
 * Filter violations by impact level
 * @param results - AxeResults from accessibility scan
 * @param impacts - Array of impact levels to include
 * @returns Filtered violations
 */
export function filterViolationsByImpact(
  results: AxeResults,
  impacts: Array<'critical' | 'serious' | 'moderate' | 'minor'>
): AxeResults['violations'] {
  return results.violations.filter(
    (v: AxeResults['violations'][number]) =>
      v.impact && impacts.includes(v.impact as 'critical' | 'serious' | 'moderate' | 'minor')
  );
}
