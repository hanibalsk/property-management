import type { AuthRequirement } from './auth';
import type { QueryParam, RouteParam } from './route';

/**
 * HTTP methods for API endpoints
 */
export type HttpMethod = 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE';

/**
 * Request body schema reference
 */
export interface RequestBodySchema {
  /** Reference to OpenAPI schema (e.g., 'Auth.LoginRequest') */
  ref?: string;
  /** Inline schema definition */
  inline?: Record<string, unknown>;
  /** Content type */
  contentType?: string;
  required: boolean;
}

/**
 * Response schema by status code
 */
export interface ResponseSchema {
  statusCode: number;
  description: string;
  /** Reference to OpenAPI schema */
  ref?: string;
  /** Inline schema definition */
  inline?: Record<string, unknown>;
}

/**
 * API endpoint definition
 */
export interface ApiEndpoint {
  /** Operation ID (e.g., 'AuthApi_login') */
  operationId: string;
  /** Server this endpoint belongs to */
  server: 'api-server' | 'reality-server';
  /** HTTP method */
  method: HttpMethod;
  /** Path pattern (e.g., '/api/v1/auth/login') */
  path: string;
  /** Brief description */
  description?: string;
  /** OpenAPI tags */
  tags?: string[];
  /** Path parameters */
  pathParams?: RouteParam[];
  /** Query parameters */
  queryParams?: QueryParam[];
  /** Request body schema */
  requestBody?: RequestBodySchema;
  /** Response schemas by status code */
  responses: ResponseSchema[];
  /** Authentication requirements */
  auth: AuthRequirement;
  /** Rate limiting info */
  rateLimit?: {
    requests: number;
    window: string;
  };
  /** Feature/Epic this endpoint belongs to */
  feature?: string;
}
