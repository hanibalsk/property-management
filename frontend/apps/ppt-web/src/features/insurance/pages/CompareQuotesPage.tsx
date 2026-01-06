/**
 * CompareQuotesPage - Compare insurance quotes from multiple providers.
 * Insurance Management Feature (UC-35)
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { QuoteComparisonCard } from '../components/QuoteComparisonCard';
import { QuoteRequestForm } from '../components/QuoteRequestForm';
import type { InsuranceQuote, QuoteRequest } from '../types';

interface CompareQuotesPageProps {
  buildings?: Array<{ id: string; name: string }>;
  quotes: InsuranceQuote[];
  isLoading?: boolean;
  isSaving?: boolean;
  comparisonId?: string;
  onRequestQuotes: (request: QuoteRequest) => void;
  onSelectQuote: (quoteId: string) => void;
  onProceedToAddPolicy: (quoteId: string) => void;
  onSaveComparison: () => void;
  onNavigateToDashboard: () => void;
}

export function CompareQuotesPage({
  buildings,
  quotes,
  isLoading,
  isSaving,
  comparisonId,
  onRequestQuotes,
  onSelectQuote,
  onProceedToAddPolicy,
  onSaveComparison,
  onNavigateToDashboard,
}: CompareQuotesPageProps) {
  const { t } = useTranslation();
  const [showAnnual, setShowAnnual] = useState(false);
  const [selectedQuoteId, setSelectedQuoteId] = useState<string | null>(null);
  const [showForm, setShowForm] = useState(quotes.length === 0);

  const handleSelectQuote = (quoteId: string) => {
    setSelectedQuoteId(quoteId);
    onSelectQuote(quoteId);
  };

  const handleProceed = () => {
    if (selectedQuoteId) {
      onProceedToAddPolicy(selectedQuoteId);
    }
  };

  const handleRequestQuotes = (request: QuoteRequest) => {
    onRequestQuotes(request);
    setShowForm(false);
    setSelectedQuoteId(null);
  };

  const sortedQuotes = [...quotes].sort((a, b) => {
    // Sort by best value first, then lowest price, then by monthly premium
    if (a.isBestValue && !b.isBestValue) return -1;
    if (!a.isBestValue && b.isBestValue) return 1;
    if (a.isLowestPrice && !b.isLowestPrice) return -1;
    if (!a.isLowestPrice && b.isLowestPrice) return 1;
    return a.premiumMonthly - b.premiumMonthly;
  });

  return (
    <div className="min-h-screen bg-gray-100">
      {/* Header */}
      <div className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
            <div>
              <button
                type="button"
                onClick={onNavigateToDashboard}
                className="text-sm text-blue-600 hover:text-blue-800 mb-2"
              >
                {t('insurance.backToDashboard')}
              </button>
              <h1 className="text-2xl font-bold text-gray-900">{t('insurance.quotes.title')}</h1>
              <p className="text-gray-600 mt-1">{t('insurance.quotes.subtitle')}</p>
            </div>
            {quotes.length > 0 && !showForm && (
              <button
                type="button"
                onClick={() => setShowForm(true)}
                className="px-4 py-2 border border-blue-600 text-blue-600 rounded-lg hover:bg-blue-50"
              >
                {t('insurance.quotes.newRequest')}
              </button>
            )}
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Quote Request Form */}
        {showForm && (
          <div className="bg-white rounded-lg shadow p-6 mb-8">
            <h2 className="text-lg font-semibold text-gray-900 mb-4">
              {t('insurance.quotes.requestTitle')}
            </h2>
            <p className="text-sm text-gray-600 mb-6">{t('insurance.quotes.requestDescription')}</p>
            <QuoteRequestForm
              buildings={buildings}
              isLoading={isLoading}
              onSubmit={handleRequestQuotes}
            />
          </div>
        )}

        {/* Loading State */}
        {isLoading && (
          <div className="flex flex-col items-center justify-center py-12">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mb-4" />
            <p className="text-gray-600">{t('insurance.quotes.fetchingQuotes')}</p>
          </div>
        )}

        {/* Quotes Results */}
        {!isLoading && quotes.length > 0 && !showForm && (
          <>
            {/* Price Toggle & Actions */}
            <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
              <div className="flex items-center gap-2">
                <span className="text-sm text-gray-600">
                  {t('insurance.quotes.showing')} {quotes.length} {t('insurance.quotes.quotes')}
                </span>
              </div>
              <div className="flex items-center gap-4">
                {/* Monthly/Annual Toggle */}
                <div className="flex items-center bg-gray-100 rounded-lg p-1">
                  <button
                    type="button"
                    onClick={() => setShowAnnual(false)}
                    className={`px-3 py-1 text-sm rounded-md transition-colors ${
                      !showAnnual
                        ? 'bg-white text-gray-900 shadow'
                        : 'text-gray-600 hover:text-gray-900'
                    }`}
                  >
                    {t('insurance.quotes.monthly')}
                  </button>
                  <button
                    type="button"
                    onClick={() => setShowAnnual(true)}
                    className={`px-3 py-1 text-sm rounded-md transition-colors ${
                      showAnnual
                        ? 'bg-white text-gray-900 shadow'
                        : 'text-gray-600 hover:text-gray-900'
                    }`}
                  >
                    {t('insurance.quotes.annual')}
                  </button>
                </div>

                {/* Save Comparison */}
                <button
                  type="button"
                  onClick={onSaveComparison}
                  disabled={isSaving || !!comparisonId}
                  className="px-4 py-2 text-sm border border-gray-300 rounded-lg hover:bg-gray-50 disabled:opacity-50 flex items-center gap-2"
                >
                  {isSaving ? (
                    <svg className="animate-spin h-4 w-4" fill="none" viewBox="0 0 24 24">
                      <circle
                        className="opacity-25"
                        cx="12"
                        cy="12"
                        r="10"
                        stroke="currentColor"
                        strokeWidth="4"
                      />
                      <path
                        className="opacity-75"
                        fill="currentColor"
                        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                      />
                    </svg>
                  ) : (
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z"
                      />
                    </svg>
                  )}
                  {comparisonId
                    ? t('insurance.quotes.saved')
                    : t('insurance.quotes.saveComparison')}
                </button>
              </div>
            </div>

            {/* Comparison Table (Desktop) */}
            <div className="hidden lg:block mb-8 overflow-x-auto">
              <table className="w-full bg-white rounded-lg shadow">
                <thead>
                  <tr className="border-b">
                    <th className="text-left p-4 font-medium text-gray-700">
                      {t('insurance.quotes.feature')}
                    </th>
                    {sortedQuotes.map((quote) => (
                      <th key={quote.id} className="p-4 text-center min-w-[180px]">
                        <div className="flex flex-col items-center gap-1">
                          {(quote.isBestValue || quote.isLowestPrice) && (
                            <span
                              className={`px-2 py-0.5 text-xs font-semibold rounded-full ${
                                quote.isBestValue
                                  ? 'bg-green-100 text-green-800'
                                  : 'bg-blue-100 text-blue-800'
                              }`}
                            >
                              {quote.isBestValue
                                ? t('insurance.quotes.bestValue')
                                : t('insurance.quotes.lowestPrice')}
                            </span>
                          )}
                          <span className="font-semibold">{quote.provider.name}</span>
                        </div>
                      </th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  {/* Premium Row */}
                  <tr className="border-b bg-gray-50">
                    <td className="p-4 font-medium text-gray-700">{t('insurance.premium')}</td>
                    {sortedQuotes.map((quote) => (
                      <td key={quote.id} className="p-4 text-center">
                        <span className="text-lg font-bold text-gray-900">
                          {new Intl.NumberFormat('en-US', {
                            style: 'currency',
                            currency: quote.currency,
                          }).format(showAnnual ? quote.premiumAnnual : quote.premiumMonthly)}
                        </span>
                        <span className="text-xs text-gray-500">
                          /{showAnnual ? t('insurance.quotes.year') : t('insurance.quotes.month')}
                        </span>
                      </td>
                    ))}
                  </tr>
                  {/* Coverage Row */}
                  <tr className="border-b">
                    <td className="p-4 font-medium text-gray-700">{t('insurance.coverage')}</td>
                    {sortedQuotes.map((quote) => (
                      <td key={quote.id} className="p-4 text-center">
                        {new Intl.NumberFormat('en-US', {
                          style: 'currency',
                          currency: quote.currency,
                        }).format(quote.coverageAmount)}
                      </td>
                    ))}
                  </tr>
                  {/* Deductible Row */}
                  <tr className="border-b">
                    <td className="p-4 font-medium text-gray-700">{t('insurance.deductible')}</td>
                    {sortedQuotes.map((quote) => (
                      <td key={quote.id} className="p-4 text-center">
                        {new Intl.NumberFormat('en-US', {
                          style: 'currency',
                          currency: quote.currency,
                        }).format(quote.deductible)}
                      </td>
                    ))}
                  </tr>
                  {/* Key Features Row */}
                  <tr className="border-b">
                    <td className="p-4 font-medium text-gray-700 align-top">
                      {t('insurance.quotes.keyFeatures')}
                    </td>
                    {sortedQuotes.map((quote) => (
                      <td key={quote.id} className="p-4 text-sm">
                        <ul className="space-y-1">
                          {quote.features.slice(0, 4).map((feature, idx) => (
                            <li key={idx} className="flex items-start gap-1">
                              <svg
                                className="w-4 h-4 text-green-500 flex-shrink-0 mt-0.5"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                              >
                                <path
                                  strokeLinecap="round"
                                  strokeLinejoin="round"
                                  strokeWidth={2}
                                  d="M5 13l4 4L19 7"
                                />
                              </svg>
                              <span className="text-gray-600">{feature}</span>
                            </li>
                          ))}
                        </ul>
                      </td>
                    ))}
                  </tr>
                  {/* Exclusions Row */}
                  <tr className="border-b">
                    <td className="p-4 font-medium text-gray-700 align-top">
                      {t('insurance.quotes.exclusions')}
                    </td>
                    {sortedQuotes.map((quote) => (
                      <td key={quote.id} className="p-4 text-sm">
                        <ul className="space-y-1">
                          {quote.exclusions.slice(0, 3).map((exclusion, idx) => (
                            <li key={idx} className="flex items-start gap-1">
                              <svg
                                className="w-4 h-4 text-red-400 flex-shrink-0 mt-0.5"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                              >
                                <path
                                  strokeLinecap="round"
                                  strokeLinejoin="round"
                                  strokeWidth={2}
                                  d="M6 18L18 6M6 6l12 12"
                                />
                              </svg>
                              <span className="text-gray-500">{exclusion}</span>
                            </li>
                          ))}
                        </ul>
                      </td>
                    ))}
                  </tr>
                  {/* Select Row */}
                  <tr>
                    <td className="p-4" />
                    {sortedQuotes.map((quote) => (
                      <td key={quote.id} className="p-4 text-center">
                        <button
                          type="button"
                          onClick={() => handleSelectQuote(quote.id)}
                          className={`px-4 py-2 rounded-lg font-medium transition-colors ${
                            selectedQuoteId === quote.id
                              ? 'bg-blue-600 text-white'
                              : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                          }`}
                        >
                          {selectedQuoteId === quote.id
                            ? t('insurance.quotes.selected')
                            : t('insurance.quotes.selectQuote')}
                        </button>
                      </td>
                    ))}
                  </tr>
                </tbody>
              </table>
            </div>

            {/* Quote Cards (Mobile/Tablet) */}
            <div className="lg:hidden grid grid-cols-1 md:grid-cols-2 gap-6">
              {sortedQuotes.map((quote) => (
                <QuoteComparisonCard
                  key={quote.id}
                  quote={quote}
                  showAnnual={showAnnual}
                  onSelect={handleSelectQuote}
                  isSelected={selectedQuoteId === quote.id}
                />
              ))}
            </div>

            {/* Proceed Button */}
            {selectedQuoteId && (
              <div className="fixed bottom-0 left-0 right-0 bg-white border-t shadow-lg p-4 lg:relative lg:border-0 lg:shadow-none lg:mt-8 lg:p-0">
                <div className="max-w-7xl mx-auto flex flex-col sm:flex-row items-center justify-between gap-4">
                  <p className="text-sm text-gray-600">
                    {t('insurance.quotes.quoteSelected')}:{' '}
                    <span className="font-medium">
                      {quotes.find((q) => q.id === selectedQuoteId)?.provider.name}
                    </span>
                  </p>
                  <button
                    type="button"
                    onClick={handleProceed}
                    className="w-full sm:w-auto px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 font-medium"
                  >
                    {t('insurance.quotes.proceedToAddPolicy')}
                  </button>
                </div>
              </div>
            )}
          </>
        )}

        {/* Empty State */}
        {!isLoading && quotes.length === 0 && !showForm && (
          <div className="bg-white rounded-lg shadow p-12 text-center">
            <svg
              className="mx-auto h-12 w-12 text-gray-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"
              />
            </svg>
            <h3 className="mt-4 text-lg font-medium text-gray-900">
              {t('insurance.quotes.noQuotes')}
            </h3>
            <p className="mt-2 text-gray-500">{t('insurance.quotes.noQuotesDescription')}</p>
            <button
              type="button"
              onClick={() => setShowForm(true)}
              className="mt-4 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
            >
              {t('insurance.quotes.getStarted')}
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
