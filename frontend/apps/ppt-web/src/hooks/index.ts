/**
 * Hooks index for ppt-web.
 */

export { useOrganization, OrganizationContext } from './useOrganization';
export type { OrganizationContextValue } from './useOrganization';

export {
  useWebSocket,
  useWebSocketSubscriptions,
  useWebSocketState,
  useWebSocketSend,
} from './useWebSocket';
export type { UseWebSocketResult } from './useWebSocket';

export { useNetworkStatus, useNetworkStatusEffect, isOnline } from './useNetworkStatus';
export type { NetworkStatus, NetworkStatusCallback } from './useNetworkStatus';
