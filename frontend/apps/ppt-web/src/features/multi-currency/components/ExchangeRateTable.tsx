/**
 * ExchangeRateTable - Story 145.2
 *
 * Table component displaying exchange rate history.
 */

interface ExchangeRate {
  id: string;
  fromCurrency: string;
  toCurrency: string;
  rate: number;
  inverseRate: number;
  rateDate: string;
  source: string;
  isOverride: boolean;
  overrideReason?: string;
}

interface ExchangeRateTableProps {
  rates: ExchangeRate[];
  isLoading?: boolean;
  onOverride?: (rate: ExchangeRate) => void;
}

export function ExchangeRateTable({ rates, isLoading, onOverride }: ExchangeRateTableProps) {
  const getSourceBadge = (source: string, isOverride: boolean) => {
    if (isOverride) {
      return (
        <span className="px-2 py-1 text-xs font-medium bg-yellow-100 text-yellow-800 rounded">
          Manual Override
        </span>
      );
    }
    switch (source) {
      case 'ecb':
        return (
          <span className="px-2 py-1 text-xs font-medium bg-blue-100 text-blue-800 rounded">
            ECB
          </span>
        );
      case 'xe':
        return (
          <span className="px-2 py-1 text-xs font-medium bg-green-100 text-green-800 rounded">
            XE
          </span>
        );
      default:
        return (
          <span className="px-2 py-1 text-xs font-medium bg-gray-100 text-gray-800 rounded">
            {source.toUpperCase()}
          </span>
        );
    }
  };

  if (isLoading) {
    return (
      <div className="flex justify-center items-center py-8">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" />
      </div>
    );
  }

  if (rates.length === 0) {
    return <div className="text-center py-8 text-gray-500">No exchange rates found.</div>;
  }

  return (
    <div className="overflow-x-auto">
      <table className="min-w-full divide-y divide-gray-200">
        <thead className="bg-gray-50">
          <tr>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Currency Pair
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Rate
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Inverse
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Date
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Source
            </th>
            {onOverride && (
              <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                Actions
              </th>
            )}
          </tr>
        </thead>
        <tbody className="bg-white divide-y divide-gray-200">
          {rates.map((rate) => (
            <tr key={rate.id} className="hover:bg-gray-50">
              <td className="px-6 py-4 whitespace-nowrap">
                <div className="flex items-center space-x-2">
                  <span className="font-medium text-gray-900">{rate.fromCurrency}</span>
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
                  <span className="font-medium text-gray-900">{rate.toCurrency}</span>
                </div>
              </td>
              <td className="px-6 py-4 whitespace-nowrap text-gray-900 font-mono">
                {rate.rate.toFixed(6)}
              </td>
              <td className="px-6 py-4 whitespace-nowrap text-gray-500 font-mono">
                {rate.inverseRate.toFixed(6)}
              </td>
              <td className="px-6 py-4 whitespace-nowrap text-gray-500">
                {new Date(rate.rateDate).toLocaleDateString()}
              </td>
              <td className="px-6 py-4 whitespace-nowrap">
                {getSourceBadge(rate.source, rate.isOverride)}
                {rate.isOverride && rate.overrideReason && (
                  <p className="text-xs text-gray-400 mt-1">{rate.overrideReason}</p>
                )}
              </td>
              {onOverride && (
                <td className="px-6 py-4 whitespace-nowrap text-right">
                  <button
                    type="button"
                    onClick={() => onOverride(rate)}
                    className="text-blue-600 hover:text-blue-900 text-sm font-medium"
                  >
                    Override
                  </button>
                </td>
              )}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
