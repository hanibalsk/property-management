/**
 * i18n configuration for Reality Portal
 */

export const locales = ['en', 'sk', 'cs', 'de', 'pl', 'hu'] as const;
export type Locale = (typeof locales)[number];

export const defaultLocale: Locale = 'en';

export const localeNames: Record<Locale, string> = {
  en: 'English',
  sk: 'Slovenčina',
  cs: 'Čeština',
  de: 'Deutsch',
  pl: 'Polski',
  hu: 'Magyar',
};

export const localeFlags: Record<Locale, string> = {
  en: '🇬🇧',
  sk: '🇸🇰',
  cs: '🇨🇿',
  de: '🇩🇪',
  pl: '🇵🇱',
  hu: '🇭🇺',
};
