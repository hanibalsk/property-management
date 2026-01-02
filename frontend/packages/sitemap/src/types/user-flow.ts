import type { Application } from './route';
import type { UserRole } from './auth';

/**
 * Categories for user flows
 */
export type FlowCategory =
  | 'authentication'
  | 'onboarding'
  | 'fault_management'
  | 'voting'
  | 'documents'
  | 'announcements'
  | 'disputes'
  | 'listings'
  | 'rentals'
  | 'settings'
  | 'admin';

/**
 * Action to perform in a flow step
 */
export interface FlowAction {
  type: 'navigate' | 'click' | 'fill' | 'submit' | 'wait' | 'assert' | 'api_call';
  /** Target selector or element */
  target?: string;
  /** Value to use (for fill, can be object for multiple fields) */
  value?: string | Record<string, unknown>;
  /** Wait time in milliseconds */
  waitMs?: number;
}

/**
 * Assertion to make in a flow step
 */
export interface FlowAssertion {
  type: 'visible' | 'hidden' | 'text' | 'url' | 'api_response' | 'state';
  /** Target selector or property */
  target?: string;
  /** Expected value */
  expected: unknown;
}

/**
 * Single step in a user flow
 */
export interface FlowStep {
  /** Step order (1-based) */
  order: number;
  /** Step name */
  name: string;
  /** Step description */
  description?: string;
  /** Route or screen ID to navigate to */
  route?: string;
  /** API endpoint to call (operation ID) */
  apiCall?: string;
  /** Expected state after this step */
  expectedState?: Record<string, unknown>;
  /** Assertions to make */
  assertions?: FlowAssertion[];
  /** User action to perform */
  action?: FlowAction;
}

/**
 * Complete user flow definition
 */
export interface UserFlow {
  /** Unique identifier */
  id: string;
  /** Human-readable name */
  name: string;
  /** Application(s) this flow spans */
  apps: Application[];
  /** Brief description */
  description?: string;
  /** Category for organization */
  category: FlowCategory;
  /** User role required to execute this flow */
  requiredRole: UserRole;
  /** Prerequisites (other flow IDs that must succeed first) */
  prerequisites?: string[];
  /** Flow steps */
  steps: FlowStep[];
  /** Expected final state */
  expectedOutcome?: Record<string, unknown>;
  /** Related use case IDs */
  useCases?: string[];
  /** Tags for categorization */
  tags?: string[];
}
