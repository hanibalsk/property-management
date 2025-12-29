/**
 * Moderation Queue Statistics Component (Epic 67, Story 67.4).
 *
 * Displays overview statistics for the content moderation queue.
 */

import type React from 'react';

export interface PriorityCount {
  priority: number;
  count: number;
}

export interface ViolationTypeCount {
  violation_type: string;
  count: number;
}

export interface ModerationQueueStatsData {
  pending_count: number;
  under_review_count: number;
  by_priority: PriorityCount[];
  by_violation_type: ViolationTypeCount[];
  avg_resolution_time_hours: number;
  overdue_count: number;
}

export interface ModerationQueueStatsProps {
  stats: ModerationQueueStatsData;
  onFilterByPriority?: (priority: number) => void;
  onFilterByViolationType?: (type: string) => void;
  onShowOverdue?: () => void;
}

const getPriorityLabel = (priority: number): string => {
  switch (priority) {
    case 1:
      return 'Critical';
    case 2:
      return 'High';
    case 3:
      return 'Medium';
    case 4:
      return 'Low';
    case 5:
      return 'Lowest';
    default:
      return `P${priority}`;
  }
};

const formatViolationType = (type: string): string => {
  return type
    .split('_')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
};

const formatHours = (hours: number): string => {
  if (hours < 1) {
    return `${Math.round(hours * 60)}m`;
  }
  if (hours < 24) {
    return `${hours.toFixed(1)}h`;
  }
  const days = Math.floor(hours / 24);
  const remainingHours = hours % 24;
  return `${days}d ${remainingHours.toFixed(0)}h`;
};

export const ModerationQueueStats: React.FC<ModerationQueueStatsProps> = ({
  stats,
  onFilterByPriority,
  onFilterByViolationType,
  onShowOverdue,
}) => {
  const totalPending = stats.pending_count + stats.under_review_count;

  return (
    <div className="moderation-queue-stats">
      {/* Overview Cards */}
      <div className="moderation-stats-overview">
        <div className="moderation-stat-card pending">
          <div className="moderation-stat-value">{stats.pending_count}</div>
          <div className="moderation-stat-label">Pending Cases</div>
        </div>
        <div className="moderation-stat-card review">
          <div className="moderation-stat-value">{stats.under_review_count}</div>
          <div className="moderation-stat-label">Under Review</div>
        </div>
        <div className="moderation-stat-card total">
          <div className="moderation-stat-value">{totalPending}</div>
          <div className="moderation-stat-label">Total Queue</div>
        </div>
        <div className="moderation-stat-card time">
          <div className="moderation-stat-value">
            {formatHours(stats.avg_resolution_time_hours)}
          </div>
          <div className="moderation-stat-label">Avg Resolution</div>
        </div>
      </div>

      {/* Overdue Alert */}
      {stats.overdue_count > 0 && (
        <div
          className="moderation-overdue-alert"
          onClick={onShowOverdue}
          onKeyDown={(e) => e.key === 'Enter' && onShowOverdue?.()}
          role="button"
          tabIndex={0}
        >
          <span className="moderation-overdue-icon">!</span>
          <span className="moderation-overdue-text">
            {stats.overdue_count} case{stats.overdue_count > 1 ? 's' : ''} overdue (24h+ SLA)
          </span>
        </div>
      )}

      {/* Priority Breakdown */}
      <div className="moderation-priority-breakdown">
        <h4>By Priority</h4>
        <div className="moderation-priority-bars">
          {stats.by_priority.map((item) => (
            <div
              key={item.priority}
              className={`moderation-priority-bar priority-${item.priority}`}
              onClick={() => onFilterByPriority?.(item.priority)}
              onKeyDown={(e) => e.key === 'Enter' && onFilterByPriority?.(item.priority)}
              role="button"
              tabIndex={0}
            >
              <span className="moderation-priority-label">{getPriorityLabel(item.priority)}</span>
              <div className="moderation-priority-bar-container">
                <div
                  className="moderation-priority-bar-fill"
                  style={{
                    width: `${totalPending > 0 ? (item.count / totalPending) * 100 : 0}%`,
                  }}
                />
              </div>
              <span className="moderation-priority-count">{item.count}</span>
            </div>
          ))}
        </div>
      </div>

      {/* Violation Type Breakdown */}
      {stats.by_violation_type.length > 0 && (
        <div className="moderation-violation-breakdown">
          <h4>By Violation Type</h4>
          <ul className="moderation-violation-list">
            {stats.by_violation_type.map((item, index) => (
              <li
                key={index}
                className="moderation-violation-item"
                onClick={() => onFilterByViolationType?.(item.violation_type)}
                onKeyDown={(e) =>
                  e.key === 'Enter' && onFilterByViolationType?.(item.violation_type)
                }
              >
                <span className="moderation-violation-type">
                  {formatViolationType(item.violation_type)}
                </span>
                <span className="moderation-violation-count">{item.count}</span>
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
};

ModerationQueueStats.displayName = 'ModerationQueueStats';
