/**
 * Shared formatting utilities for financial components.
 */

/**
 * Detects and returns the user's locale for formatting.
 * Falls back to 'en-US' if locale cannot be determined.
 */
export function getUserLocale(): string {
  if (typeof navigator !== 'undefined' && navigator.language) {
    return navigator.language;
  }

  if (typeof Intl !== 'undefined' && typeof Intl.DateTimeFormat === 'function') {
    try {
      const resolved = new Intl.DateTimeFormat().resolvedOptions().locale;
      if (resolved) {
        return resolved;
      }
    } catch {
      // Ignore and fall through to default
    }
  }

  // Fallback to previous behavior
  return 'en-US';
}

/**
 * Formats a number as currency using the user's locale.
 */
export function formatCurrency(amount: number, currency = 'EUR'): string {
  return new Intl.NumberFormat(getUserLocale(), {
    style: 'currency',
    currency,
  }).format(amount);
}

/**
 * Pre-configured date formatter for invoice dates.
 */
const invoiceDateFormatter = new Intl.DateTimeFormat(getUserLocale(), {
  month: 'short',
  day: 'numeric',
  year: 'numeric',
});

/**
 * Formats a date string using the invoice date format (short month, day, year).
 */
export function formatDate(dateString: string): string {
  return invoiceDateFormatter.format(new Date(dateString));
}

/**
 * Pre-configured date formatter for detailed dates.
 */
const detailedDateFormatter = new Intl.DateTimeFormat(getUserLocale(), {
  month: 'long',
  day: 'numeric',
  year: 'numeric',
});

/**
 * Formats a date string using the detailed date format (long month, day, year).
 */
export function formatDetailedDate(dateString: string): string {
  return detailedDateFormatter.format(new Date(dateString));
}
