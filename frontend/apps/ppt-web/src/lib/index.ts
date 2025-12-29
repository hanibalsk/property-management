/**
 * Library Exports
 *
 * Centralized exports for ppt-web lib utilities (Story 79.1).
 */

// API Client
export {
  configureApiClient,
  getApiClient,
  getErrorMessage,
  isApiError,
  resetApiClient,
  type ApiClientConfig,
  type ApiError,
  type ApiErrorResponse,
  type TokenGetter,
} from './api';

// Query Keys
export {
  queryKeys,
  type AnnouncementFilters,
  type DocumentFilters,
  type DocumentSearchFilters,
  type FaultFilters,
  type FormFilters,
  type FormSubmissionFilters,
  type MessageFilters,
  type NeighborFilters,
  type PaginationParams,
  type QueryKeys,
  type VoteFilters,
} from './queryKeys';

// WebSocket (Story 79.4)
export {
  WebSocketService,
  createWebSocketService,
  type ConnectionState,
  type ConnectionStateHandler,
  type MessageHandler,
  type WebSocketEventType,
  type WebSocketMessage,
  type WebSocketServiceConfig,
} from './websocket';

// Error Handler (Story 79.3)
export {
  parseApiError,
  formatValidationErrors,
  getFieldError,
  type ErrorResponse,
  type ValidationDetail,
  type ParsedApiError,
} from './errorHandler';
