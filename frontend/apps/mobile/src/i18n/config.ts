/**
 * i18n configuration for Property Management Mobile App
 */

export const locales = ['en', 'sk', 'cs', 'de'] as const;
export type Locale = (typeof locales)[number];

export const defaultLocale: Locale = 'en';

export const localeNames: Record<Locale, string> = {
  en: 'English',
  sk: 'Slovenčina',
  cs: 'Čeština',
  de: 'Deutsch',
};

export const localeFlags: Record<Locale, string> = {
  en: '🇬🇧',
  sk: '🇸🇰',
  cs: '🇨🇿',
  de: '🇩🇪',
};
