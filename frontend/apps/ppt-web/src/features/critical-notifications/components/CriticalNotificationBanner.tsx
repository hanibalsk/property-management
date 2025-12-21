/**
 * CriticalNotificationBanner Component (Epic 8A, Story 8A.2)
 *
 * A persistent banner that appears at the top of the page when there are
 * unacknowledged critical notifications.
 */

interface CriticalNotificationBannerProps {
  count: number;
  onClick: () => void;
}

export function CriticalNotificationBanner({ count, onClick }: CriticalNotificationBannerProps) {
  if (count === 0) return null;

  return (
    <div className="fixed top-0 left-0 right-0 z-40 bg-red-600 text-white">
      <div className="mx-auto max-w-7xl px-3 py-2 sm:px-6 lg:px-8">
        <div className="flex flex-wrap items-center justify-between">
          <div className="flex w-0 flex-1 items-center">
            <span className="flex rounded-lg bg-red-800 p-2">
              <svg
                className="h-5 w-5 text-white"
                fill="none"
                viewBox="0 0 24 24"
                strokeWidth="1.5"
                stroke="currentColor"
                aria-hidden="true"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  d="M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z"
                />
              </svg>
            </span>
            <p className="ml-3 truncate font-medium text-white">
              <span className="md:hidden">
                {count} critical {count === 1 ? 'notification' : 'notifications'}
              </span>
              <span className="hidden md:inline">
                You have {count} unacknowledged critical{' '}
                {count === 1 ? 'notification' : 'notifications'} that{' '}
                {count === 1 ? 'requires' : 'require'} your attention.
              </span>
            </p>
          </div>
          <div className="order-3 mt-2 w-full flex-shrink-0 sm:order-2 sm:mt-0 sm:w-auto">
            <button
              type="button"
              onClick={onClick}
              className="flex items-center justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-red-600 shadow-sm hover:bg-red-50"
            >
              View Now
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
