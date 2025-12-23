/**
 * DeadlineCountdown - Countdown timer card for upcoming submission deadlines.
 * Epic 41: Government Portal UI (Story 41.3)
 */

interface DeadlineCountdownProps {
  dueDate: string;
  templateName: string;
  daysUntilDue: number;
}

export function DeadlineCountdown({ dueDate, templateName, daysUntilDue }: DeadlineCountdownProps) {
  const getUrgencyStyles = () => {
    if (daysUntilDue <= 1) {
      return {
        border: 'border-red-300',
        background: 'bg-red-50',
        badge: 'bg-red-100 text-red-800',
        icon: 'text-red-500',
      };
    }
    if (daysUntilDue <= 3) {
      return {
        border: 'border-amber-300',
        background: 'bg-amber-50',
        badge: 'bg-amber-100 text-amber-800',
        icon: 'text-amber-500',
      };
    }
    if (daysUntilDue <= 7) {
      return {
        border: 'border-yellow-300',
        background: 'bg-yellow-50',
        badge: 'bg-yellow-100 text-yellow-800',
        icon: 'text-yellow-500',
      };
    }
    return {
      border: 'border-gray-200',
      background: 'bg-white',
      badge: 'bg-gray-100 text-gray-700',
      icon: 'text-gray-400',
    };
  };

  const getUrgencyLabel = () => {
    if (daysUntilDue <= 0) return 'Overdue!';
    if (daysUntilDue === 1) return 'Due tomorrow';
    if (daysUntilDue <= 3) return 'Urgent';
    if (daysUntilDue <= 7) return 'Soon';
    return `${daysUntilDue} days`;
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('en-GB', {
      weekday: 'short',
      day: '2-digit',
      month: 'short',
      year: 'numeric',
    });
  };

  const styles = getUrgencyStyles();

  return (
    <div className={`rounded-lg border ${styles.border} ${styles.background} p-4`}>
      <div className="flex items-start justify-between gap-3">
        <div className="flex-1 min-w-0">
          <h3 className="font-medium text-gray-900 truncate">{templateName}</h3>
          <p className="mt-1 text-sm text-gray-500">{formatDate(dueDate)}</p>
        </div>
        <div className={styles.icon}>
          {daysUntilDue <= 1 ? (
            <svg className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
              />
            </svg>
          ) : (
            <svg className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
          )}
        </div>
      </div>
      <div className="mt-3">
        <span
          className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${styles.badge}`}
        >
          {getUrgencyLabel()}
        </span>
      </div>
    </div>
  );
}
