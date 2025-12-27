/**
 * Comparison Chart Component
 * Story 70.4: Comparable Sales/Rentals
 *
 * Visual comparison chart for property features.
 */

export interface ComparisonEntry {
  feature: string;
  sourceValue: string;
  comparableValues: string[];
}

export interface ComparisonChartProps {
  entries: ComparisonEntry[];
  sourceLabel?: string;
  comparableLabels?: string[];
  className?: string;
}

/**
 * Displays a side-by-side comparison of property features.
 */
export function ComparisonChart({
  entries,
  sourceLabel = 'Your Property',
  comparableLabels = [],
  className = '',
}: ComparisonChartProps) {
  // Determine number of comparables from first entry
  const numComparables = entries[0]?.comparableValues.length ?? 0;

  // Generate labels if not provided
  const labels = comparableLabels.length
    ? comparableLabels
    : Array.from({ length: numComparables }, (_, i) => `Comparable ${i + 1}`);

  return (
    <div className={`bg-white rounded-lg shadow-sm border ${className}`}>
      <div className="p-4 border-b">
        <h3 className="text-lg font-semibold text-gray-900">
          Feature Comparison
        </h3>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead className="bg-gray-50 border-b">
            <tr>
              <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Feature
              </th>
              <th className="px-4 py-3 text-center text-xs font-medium text-blue-600 uppercase bg-blue-50">
                {sourceLabel}
              </th>
              {labels.slice(0, numComparables).map((label, idx) => (
                <th
                  key={idx}
                  className="px-4 py-3 text-center text-xs font-medium text-gray-500 uppercase"
                >
                  {label}
                </th>
              ))}
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-100">
            {entries.map((entry, idx) => (
              <tr key={idx} className="hover:bg-gray-50">
                <td className="px-4 py-3 font-medium text-gray-900">
                  {entry.feature}
                </td>
                <td className="px-4 py-3 text-center font-semibold text-blue-600 bg-blue-50">
                  {entry.sourceValue}
                </td>
                {entry.comparableValues.map((value, vIdx) => (
                  <td key={vIdx} className="px-4 py-3 text-center text-gray-700">
                    {value}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {entries.length === 0 && (
        <div className="p-8 text-center text-gray-500">
          No comparison data available.
        </div>
      )}
    </div>
  );
}

/** Price Range Display Component */
export interface PriceRange {
  min: number;
  max: number;
  median: number;
  currency: string;
}

export interface PriceRangeDisplayProps {
  range: PriceRange;
  currentPrice?: number;
  className?: string;
}

export function PriceRangeDisplay({
  range,
  currentPrice,
  className = '',
}: PriceRangeDisplayProps) {
  const formatPrice = (price: number) => {
    return new Intl.NumberFormat('sk-SK', {
      style: 'currency',
      currency: range.currency,
      maximumFractionDigits: 0,
    }).format(price);
  };

  const getPosition = (price: number) => {
    const total = range.max - range.min;
    if (total === 0) return 50;
    return ((price - range.min) / total) * 100;
  };

  return (
    <div className={`bg-white rounded-lg shadow-sm border p-4 ${className}`}>
      <h4 className="text-sm font-medium text-gray-700 mb-4">
        Market Price Range
      </h4>

      {/* Price range bar */}
      <div className="relative pt-8 pb-4">
        {/* Background bar */}
        <div className="h-3 bg-gradient-to-r from-green-400 via-yellow-400 to-red-400 rounded-full" />

        {/* Median marker */}
        <div
          className="absolute top-0 transform -translate-x-1/2"
          style={{ left: `${getPosition(range.median)}%` }}
        >
          <div className="flex flex-col items-center">
            <span className="text-xs font-medium text-gray-600 whitespace-nowrap">
              Median: {formatPrice(range.median)}
            </span>
            <div className="w-0.5 h-3 bg-gray-600 mt-1" />
            <div className="w-3 h-3 bg-gray-600 rounded-full" />
          </div>
        </div>

        {/* Current price marker */}
        {currentPrice && (
          <div
            className="absolute top-0 transform -translate-x-1/2"
            style={{ left: `${getPosition(currentPrice)}%` }}
          >
            <div className="flex flex-col items-center">
              <span className="text-xs font-medium text-blue-600 whitespace-nowrap">
                Your Price: {formatPrice(currentPrice)}
              </span>
              <div className="w-0.5 h-3 bg-blue-600 mt-1" />
              <div className="w-3 h-3 bg-blue-600 rounded-full border-2 border-white shadow" />
            </div>
          </div>
        )}
      </div>

      {/* Min/Max labels */}
      <div className="flex justify-between text-sm text-gray-500">
        <span>{formatPrice(range.min)}</span>
        <span>{formatPrice(range.max)}</span>
      </div>

      {/* Position analysis */}
      {currentPrice && (
        <div className="mt-4 pt-4 border-t">
          <p className="text-sm text-gray-600">
            {currentPrice < range.median
              ? `Your price is ${formatPrice(range.median - currentPrice)} below the median.`
              : currentPrice > range.median
                ? `Your price is ${formatPrice(currentPrice - range.median)} above the median.`
                : 'Your price is exactly at the median.'}
          </p>
        </div>
      )}
    </div>
  );
}
