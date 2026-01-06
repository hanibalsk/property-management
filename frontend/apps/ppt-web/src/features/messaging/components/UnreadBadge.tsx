/**
 * UnreadBadge Component
 *
 * Displays unread message count badge.
 */

interface UnreadBadgeProps {
  count: number;
  size?: 'sm' | 'md' | 'lg';
}

const sizeClasses = {
  sm: 'w-4 h-4 text-[10px]',
  md: 'w-5 h-5 text-xs',
  lg: 'w-6 h-6 text-sm',
};

export function UnreadBadge({ count, size = 'md' }: UnreadBadgeProps) {
  if (count === 0) return null;

  const displayCount = count > 99 ? '99+' : count.toString();

  return (
    <span
      className={`inline-flex items-center justify-center bg-blue-600 text-white font-medium rounded-full ${sizeClasses[size]}`}
      aria-label={`${count} unread messages`}
    >
      {displayCount}
    </span>
  );
}
