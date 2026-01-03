import { useTranslation } from 'react-i18next';
import { type Locale, localeFlags, localeNames, locales } from '../i18n';

export function LanguageSwitcher() {
  const { i18n } = useTranslation();
  const currentLocale = i18n.language as Locale;

  const handleChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const newLocale = event.target.value as Locale;
    i18n.changeLanguage(newLocale);
  };

  return (
    <select
      value={currentLocale}
      onChange={handleChange}
      className="bg-transparent border border-gray-300 rounded-md px-2 py-1 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
      aria-label="Select language"
    >
      {locales.map((locale) => (
        <option key={locale} value={locale}>
          {localeFlags[locale]} {localeNames[locale]}
        </option>
      ))}
    </select>
  );
}
