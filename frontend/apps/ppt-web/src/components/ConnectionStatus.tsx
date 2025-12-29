/**
 * Connection Status Indicator for ppt-web (Story 79.4)
 *
 * Displays WebSocket connection status with:
 * - Green dot for connected
 * - Yellow dot for connecting
 * - Red dot for disconnected/error
 * - Tooltip with details
 */

import type React from 'react';
import { useCallback, useState } from 'react';
import { useWebSocketState } from '../hooks/useWebSocket';
import './ConnectionStatus.css';

/**
 * Props for the ConnectionStatus component.
 */
export interface ConnectionStatusProps {
  /**
   * Whether to show the status label text.
   * @default false
   */
  showLabel?: boolean;

  /**
   * Custom class name for styling.
   */
  className?: string;

  /**
   * Aria label for the status indicator.
   * @default 'Connection status'
   */
  ariaLabel?: string;
}

/**
 * Connection status indicator component.
 *
 * Shows a colored dot indicating WebSocket connection state:
 * - Green: Connected
 * - Yellow: Connecting
 * - Red: Disconnected or Error
 *
 * Includes a tooltip with more details on hover.
 */
export function ConnectionStatus({
  showLabel = false,
  className = '',
  ariaLabel = 'Connection status',
}: ConnectionStatusProps): React.ReactElement {
  const { isConnected, isConnecting, error, reconnect } = useWebSocketState();
  const [showTooltip, setShowTooltip] = useState(false);

  const status = isConnected ? 'connected' : isConnecting ? 'connecting' : 'disconnected';

  const statusLabel = isConnected
    ? 'Connected'
    : isConnecting
      ? 'Connecting...'
      : error
        ? 'Connection Error'
        : 'Disconnected';

  const statusDescription = isConnected
    ? 'Real-time updates are active'
    : isConnecting
      ? 'Establishing connection...'
      : error
        ? `Error: ${error.message}`
        : 'Real-time updates are not available';

  const handleMouseEnter = useCallback(() => {
    setShowTooltip(true);
  }, []);

  const handleMouseLeave = useCallback(() => {
    setShowTooltip(false);
  }, []);

  const handleKeyDown = useCallback(
    (event: React.KeyboardEvent) => {
      if (event.key === 'Enter' || event.key === ' ') {
        event.preventDefault();
        if (!isConnected && !isConnecting) {
          reconnect();
        }
      }
    },
    [isConnected, isConnecting, reconnect]
  );

  const handleClick = useCallback(() => {
    if (!isConnected && !isConnecting) {
      reconnect();
    }
  }, [isConnected, isConnecting, reconnect]);

  return (
    <div
      className={`connection-status ${className}`.trim()}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onClick={handleClick}
      onKeyDown={handleKeyDown}
      role="status"
      aria-label={`${ariaLabel}: ${statusLabel}`}
      tabIndex={!isConnected && !isConnecting ? 0 : -1}
    >
      <span
        className={`connection-status__dot connection-status__dot--${status}`}
        aria-hidden="true"
      />

      {showLabel && <span className="connection-status__label">{statusLabel}</span>}

      {showTooltip && (
        <div className="connection-status__tooltip" role="tooltip">
          <div className="connection-status__tooltip-header">{statusLabel}</div>
          <div className="connection-status__tooltip-description">{statusDescription}</div>
          {!isConnected && !isConnecting && (
            <div className="connection-status__tooltip-action">Click to reconnect</div>
          )}
        </div>
      )}
    </div>
  );
}

ConnectionStatus.displayName = 'ConnectionStatus';
