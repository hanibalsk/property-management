/**
 * CountrySelector - Country selector for government portal submissions.
 * Epic 41: Government Portal UI (Story 41.1)
 */

interface CountrySelectorProps {
  value: string;
  onChange: (country: string) => void;
  disabled?: boolean;
}

const countries = [
  { code: 'SK', name: 'Slovakia', flag: '🇸🇰' },
  { code: 'CZ', name: 'Czech Republic', flag: '🇨🇿' },
  { code: 'DE', name: 'Germany', flag: '🇩🇪' },
  { code: 'AT', name: 'Austria', flag: '🇦🇹' },
  { code: 'PL', name: 'Poland', flag: '🇵🇱' },
  { code: 'HU', name: 'Hungary', flag: '🇭🇺' },
];

export function CountrySelector({ value, onChange, disabled = false }: CountrySelectorProps) {
  return (
    <div className="space-y-2">
      <label htmlFor="country-selector" className="block text-sm font-medium text-gray-700">
        Country
      </label>
      <select
        id="country-selector"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        disabled={disabled}
        className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm disabled:bg-gray-100 disabled:cursor-not-allowed"
      >
        {countries.map((country) => (
          <option key={country.code} value={country.code}>
            {country.flag} {country.name} ({country.code})
          </option>
        ))}
      </select>
    </div>
  );
}

export function CountryFlag({ countryCode }: { countryCode: string }) {
  const country = countries.find((c) => c.code === countryCode);
  return <span title={country?.name}>{country?.flag || '🌐'}</span>;
}

export { countries };
