/**
 * ExchangeRateCard - Story 145.2
 *
 * Card component displaying exchange rate information for a currency pair.
 */

interface ExchangeRateCardProps {
  fromCurrency: string;
  toCurrency: string;
  rate: number;
  rateDate: string;
  source: string;
  change24h?: number;
  change7d?: number;
  isOverride?: boolean;
  onOverride?: () => void;
}

export function ExchangeRateCard({
  fromCurrency,
  toCurrency,
  rate,
  rateDate,
  source,
  change24h,
  change7d,
  isOverride,
  onOverride,
}: ExchangeRateCardProps) {
  const formatChange = (change: number | undefined) => {
    if (change === undefined) return null;
    const isPositive = change >= 0;
    return (
      <span className={isPositive ? 'text-green-600' : 'text-red-600'}>
        {isPositive ? '+' : ''}
        {change.toFixed(2)}%
      </span>
    );
  };

  const getSourceLabel = (src: string) => {
    switch (src) {
      case 'ecb':
        return 'ECB';
      case 'xe':
        return 'XE';
      case 'manual':
        return 'Manual';
      default:
        return src.toUpperCase();
    }
  };

  return (
    <div className="bg-white rounded-lg shadow p-4 border border-gray-200">
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center space-x-2">
          <span className="text-lg font-semibold text-gray-900">{fromCurrency}</span>
          <svg
            className="w-4 h-4 text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M14 5l7 7m0 0l-7 7m7-7H3"
            />
          </svg>
          <span className="text-lg font-semibold text-gray-900">{toCurrency}</span>
        </div>
        {isOverride && (
          <span className="px-2 py-1 text-xs font-medium bg-yellow-100 text-yellow-800 rounded">
            Override
          </span>
        )}
      </div>

      <div className="text-3xl font-bold text-gray-900 mb-2">{rate.toFixed(4)}</div>

      <div className="flex items-center space-x-4 text-sm text-gray-500 mb-3">
        {change24h !== undefined && <div>24h: {formatChange(change24h)}</div>}
        {change7d !== undefined && <div>7d: {formatChange(change7d)}</div>}
      </div>

      <div className="flex items-center justify-between text-xs text-gray-400">
        <span>
          {new Date(rateDate).toLocaleDateString()} - {getSourceLabel(source)}
        </span>
        {onOverride && (
          <button
            type="button"
            onClick={onOverride}
            className="text-blue-600 hover:text-blue-800 font-medium"
          >
            Override
          </button>
        )}
      </div>
    </div>
  );
}
