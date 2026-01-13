/**
 * CurrencySelector - Story 145.1
 *
 * Dropdown component for selecting a currency.
 */

interface CurrencyOption {
  code: string;
  name: string;
  symbol?: string;
}

interface CurrencySelectorProps {
  value: string;
  onChange: (currency: string) => void;
  currencies?: CurrencyOption[];
  label?: string;
  disabled?: boolean;
  showSymbol?: boolean;
  className?: string;
}

const DEFAULT_CURRENCIES: CurrencyOption[] = [
  { code: 'EUR', name: 'Euro', symbol: '\u20AC' },
  { code: 'CZK', name: 'Czech Koruna', symbol: 'Kc' },
  { code: 'CHF', name: 'Swiss Franc', symbol: 'CHF' },
  { code: 'GBP', name: 'British Pound', symbol: '\u00A3' },
  { code: 'PLN', name: 'Polish Zloty', symbol: 'zl' },
  { code: 'USD', name: 'US Dollar', symbol: '$' },
  { code: 'HUF', name: 'Hungarian Forint', symbol: 'Ft' },
  { code: 'RON', name: 'Romanian Leu', symbol: 'lei' },
  { code: 'SEK', name: 'Swedish Krona', symbol: 'kr' },
  { code: 'DKK', name: 'Danish Krone', symbol: 'kr' },
  { code: 'NOK', name: 'Norwegian Krone', symbol: 'kr' },
];

export function CurrencySelector({
  value,
  onChange,
  currencies = DEFAULT_CURRENCIES,
  label,
  disabled = false,
  showSymbol = true,
  className = '',
}: CurrencySelectorProps) {
  return (
    <div className={className}>
      {label && <label className="block text-sm font-medium text-gray-700 mb-1">{label}</label>}
      <select
        value={value}
        onChange={(e) => onChange(e.target.value)}
        disabled={disabled}
        className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 disabled:bg-gray-100 disabled:cursor-not-allowed"
      >
        {currencies.map((currency) => (
          <option key={currency.code} value={currency.code}>
            {currency.code}
            {showSymbol && currency.symbol ? ` (${currency.symbol})` : ''} - {currency.name}
          </option>
        ))}
      </select>
    </div>
  );
}
