/**
 * Pricing Suggestion Card Component
 * Story 70.2: Dynamic Pricing Suggestions
 *
 * Displays suggested price range with confidence level and market trend.
 */

export interface PricingSuggestion {
  id: string;
  listingId: string;
  suggestedPriceLow: number;
  suggestedPriceMid: number;
  suggestedPriceHigh: number;
  currency: string;
  confidenceLevel: 'high' | 'medium' | 'low';
  confidenceScore?: number;
  comparablesCount: number;
  marketTrend: 'rising' | 'stable' | 'falling';
  seasonalAdjustment?: number;
  calculatedAt: string;
  validUntil: string;
}

export interface PricingSuggestionCardProps {
  suggestion: PricingSuggestion;
  currentPrice?: number;
  onAnalyze?: () => void;
  className?: string;
}

/**
 * Displays pricing suggestion with visual indicators.
 */
export function PricingSuggestionCard({
  suggestion,
  currentPrice,
  onAnalyze,
  className = '',
}: PricingSuggestionCardProps) {
  const formatPrice = (price: number) => {
    return new Intl.NumberFormat('sk-SK', {
      style: 'currency',
      currency: suggestion.currency,
      maximumFractionDigits: 0,
    }).format(price);
  };

  const getConfidenceColor = () => {
    switch (suggestion.confidenceLevel) {
      case 'high':
        return 'text-green-600 bg-green-100';
      case 'medium':
        return 'text-yellow-600 bg-yellow-100';
      case 'low':
        return 'text-red-600 bg-red-100';
      default:
        return 'text-gray-600 bg-gray-100';
    }
  };

  const getTrendIcon = () => {
    switch (suggestion.marketTrend) {
      case 'rising':
        return (
          <svg
            className="w-4 h-4 text-green-500"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"
            />
          </svg>
        );
      case 'falling':
        return (
          <svg
            className="w-4 h-4 text-red-500"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M13 17h8m0 0v-8m0 8l-8-8-4 4-6-6"
            />
          </svg>
        );
      default:
        return (
          <svg
            className="w-4 h-4 text-gray-500"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M5 12h14"
            />
          </svg>
        );
    }
  };

  const getPricePosition = () => {
    if (!currentPrice) return null;
    const range = suggestion.suggestedPriceHigh - suggestion.suggestedPriceLow;
    const position =
      ((currentPrice - suggestion.suggestedPriceLow) / range) * 100;
    return Math.max(0, Math.min(100, position));
  };

  const pricePosition = getPricePosition();

  return (
    <div className={`bg-white rounded-lg shadow-sm border p-6 ${className}`}>
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold text-gray-900">
          Pricing Suggestion
        </h3>
        <span
          className={`px-2 py-1 rounded-full text-xs font-medium capitalize ${getConfidenceColor()}`}
        >
          {suggestion.confidenceLevel} confidence
        </span>
      </div>

      {/* Suggested price range */}
      <div className="mb-6">
        <div className="text-center mb-4">
          <div className="text-3xl font-bold text-gray-900">
            {formatPrice(suggestion.suggestedPriceMid)}
          </div>
          <div className="text-sm text-gray-500">Suggested Price</div>
        </div>

        {/* Price range visualization */}
        <div className="relative pt-6 pb-2">
          <div className="h-2 bg-gray-200 rounded-full">
            <div
              className="h-full bg-gradient-to-r from-green-400 via-blue-400 to-green-400 rounded-full"
              style={{ width: '100%' }}
            />
          </div>

          {/* Current price marker */}
          {currentPrice && pricePosition !== null && (
            <div
              className="absolute top-0 w-4 h-4 -ml-2 transform -translate-y-1"
              style={{ left: `${pricePosition}%` }}
            >
              <div className="w-4 h-4 bg-blue-600 rounded-full border-2 border-white shadow" />
              <div className="absolute -top-6 left-1/2 transform -translate-x-1/2 whitespace-nowrap">
                <span className="text-xs font-medium text-blue-600">
                  Current: {formatPrice(currentPrice)}
                </span>
              </div>
            </div>
          )}

          {/* Range labels */}
          <div className="flex justify-between mt-2 text-sm text-gray-500">
            <span>{formatPrice(suggestion.suggestedPriceLow)}</span>
            <span>{formatPrice(suggestion.suggestedPriceHigh)}</span>
          </div>
        </div>
      </div>

      {/* Stats grid */}
      <div className="grid grid-cols-3 gap-4 mb-4">
        <div className="text-center p-3 bg-gray-50 rounded-lg">
          <div className="flex items-center justify-center gap-1 mb-1">
            {getTrendIcon()}
            <span className="text-sm font-medium capitalize">
              {suggestion.marketTrend}
            </span>
          </div>
          <div className="text-xs text-gray-500">Market Trend</div>
        </div>

        <div className="text-center p-3 bg-gray-50 rounded-lg">
          <div className="text-sm font-medium">{suggestion.comparablesCount}</div>
          <div className="text-xs text-gray-500">Comparables</div>
        </div>

        <div className="text-center p-3 bg-gray-50 rounded-lg">
          <div className="text-sm font-medium">
            {suggestion.confidenceScore ? `${suggestion.confidenceScore}%` : '-'}
          </div>
          <div className="text-xs text-gray-500">Score</div>
        </div>
      </div>

      {/* Seasonal adjustment */}
      {suggestion.seasonalAdjustment && suggestion.seasonalAdjustment !== 1 && (
        <div className="flex items-center gap-2 text-sm text-gray-600 mb-4">
          <svg
            className="w-4 h-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <span>
            Includes {((suggestion.seasonalAdjustment - 1) * 100).toFixed(0)}%
            seasonal adjustment
          </span>
        </div>
      )}

      {/* Valid until */}
      <div className="text-xs text-gray-400 mb-4">
        Valid until: {new Date(suggestion.validUntil).toLocaleDateString()}
      </div>

      {/* Action */}
      {onAnalyze && (
        <button
          type="button"
          onClick={onAnalyze}
          className="w-full px-4 py-2 text-blue-600 bg-blue-50 hover:bg-blue-100 rounded-lg transition-colors text-sm font-medium"
        >
          View Full Analysis
        </button>
      )}
    </div>
  );
}
