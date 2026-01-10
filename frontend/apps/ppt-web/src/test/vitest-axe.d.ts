/**
 * Type declarations for vitest-axe matchers
 *
 * Extends Vitest's Assertion interface with axe accessibility matchers.
 */

interface AxeMatchers<R = unknown> {
  /**
   * Assert that the axe-core results contain no accessibility violations.
   */
  toHaveNoViolations(): R;
}

declare module 'vitest' {
  interface Assertion<T = unknown> extends AxeMatchers<T> {}
  interface AsymmetricMatchersContaining extends AxeMatchers {}
}
