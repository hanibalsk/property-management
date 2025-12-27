/**
 * Pricing Analysis Panel Component
 * Story 70.2: Dynamic Pricing Suggestions
 *
 * Detailed pricing analysis with factors, history, and comparables.
 */

import type { PricingSuggestion } from './PricingSuggestionCard';

export interface PricingFactor {
  id: string;
  factorType: string;
  factorName: string;
  impact: number;
  explanation: string;
}

export interface PriceHistoryPoint {
  id: string;
  price: number;
  pricePerSqm?: number;
  currency: string;
  recordedAt: string;
}

export interface PricingAnalysis {
  suggestion: PricingSuggestion;
  factors: PricingFactor[];
  priceHistory: PriceHistoryPoint[];
  comparablesUsed: Array<{
    id: string;
    propertyType: string;
    city: string;
    sizeSqm: number;
    price: number;
    pricePerSqm: number;
    similarityScore: number;
  }>;
}

export interface PricingAnalysisPanelProps {
  analysis: PricingAnalysis;
  onRefresh?: () => void;
  isRefreshing?: boolean;
  className?: string;
}

/**
 * Displays detailed pricing analysis with all contributing factors.
 */
export function PricingAnalysisPanel({
  analysis,
  onRefresh,
  isRefreshing = false,
  className = '',
}: PricingAnalysisPanelProps) {
  const formatPrice = (price: number, currency: string) => {
    return new Intl.NumberFormat('sk-SK', {
      style: 'currency',
      currency,
      maximumFractionDigits: 0,
    }).format(price);
  };

  const formatDate = (dateStr: string) => {
    return new Date(dateStr).toLocaleDateString('sk-SK', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  };

  return (
    <div className={`bg-white rounded-lg shadow-sm border ${className}`}>
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b">
        <h3 className="text-lg font-semibold text-gray-900">
          Pricing Analysis
        </h3>
        {onRefresh && (
          <button
            type="button"
            onClick={onRefresh}
            disabled={isRefreshing}
            className="flex items-center gap-2 px-3 py-1.5 text-sm text-blue-600 hover:bg-blue-50 rounded-lg transition-colors disabled:opacity-50"
          >
            <svg
              className={`w-4 h-4 ${isRefreshing ? 'animate-spin' : ''}`}
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
              />
            </svg>
            {isRefreshing ? 'Refreshing...' : 'Refresh'}
          </button>
        )}
      </div>

      {/* Price Factors */}
      <div className="p-4 border-b">
        <h4 className="text-sm font-medium text-gray-700 mb-3">
          Contributing Factors
        </h4>
        <div className="space-y-3">
          {analysis.factors.map((factor) => (
            <div
              key={factor.id}
              className="flex items-center justify-between p-3 bg-gray-50 rounded-lg"
            >
              <div className="flex-1">
                <div className="flex items-center gap-2">
                  <span className="text-xs px-2 py-0.5 bg-gray-200 rounded capitalize">
                    {factor.factorType}
                  </span>
                  <span className="font-medium text-gray-900">
                    {factor.factorName}
                  </span>
                </div>
                <p className="text-sm text-gray-600 mt-1">
                  {factor.explanation}
                </p>
              </div>
              <div
                className={`text-lg font-semibold ml-4 ${
                  factor.impact >= 0 ? 'text-green-600' : 'text-red-600'
                }`}
              >
                {factor.impact >= 0 ? '+' : ''}
                {factor.impact.toFixed(1)}%
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Price History */}
      <div className="p-4 border-b">
        <h4 className="text-sm font-medium text-gray-700 mb-3">
          Price History (Area)
        </h4>
        {analysis.priceHistory.length > 0 ? (
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="text-left text-gray-500">
                  <th className="pb-2">Date</th>
                  <th className="pb-2 text-right">Price</th>
                  <th className="pb-2 text-right">Per sqm</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-100">
                {analysis.priceHistory.map((point) => (
                  <tr key={point.id}>
                    <td className="py-2">{formatDate(point.recordedAt)}</td>
                    <td className="py-2 text-right font-medium">
                      {formatPrice(point.price, point.currency)}
                    </td>
                    <td className="py-2 text-right text-gray-500">
                      {point.pricePerSqm
                        ? `${formatPrice(point.pricePerSqm, point.currency)}/m2`
                        : '-'}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        ) : (
          <p className="text-sm text-gray-500">
            No price history available for this area.
          </p>
        )}
      </div>

      {/* Comparables Used */}
      <div className="p-4">
        <h4 className="text-sm font-medium text-gray-700 mb-3">
          Comparables Used ({analysis.comparablesUsed.length})
        </h4>
        {analysis.comparablesUsed.length > 0 ? (
          <div className="space-y-2">
            {analysis.comparablesUsed.map((comp) => (
              <div
                key={comp.id}
                className="flex items-center justify-between p-3 bg-gray-50 rounded-lg"
              >
                <div>
                  <div className="font-medium text-gray-900 capitalize">
                    {comp.propertyType} - {comp.city}
                  </div>
                  <div className="text-sm text-gray-500">
                    {comp.sizeSqm} sqm
                  </div>
                </div>
                <div className="text-right">
                  <div className="font-medium">
                    {formatPrice(comp.price, analysis.suggestion.currency)}
                  </div>
                  <div className="text-xs text-gray-500">
                    {comp.similarityScore}% match
                  </div>
                </div>
              </div>
            ))}
          </div>
        ) : (
          <p className="text-sm text-gray-500">
            No comparable properties found.
          </p>
        )}
      </div>

      {/* Footer info */}
      <div className="px-4 py-3 bg-gray-50 rounded-b-lg text-xs text-gray-500">
        Analysis generated on {formatDate(analysis.suggestion.calculatedAt)}
      </div>
    </div>
  );
}
