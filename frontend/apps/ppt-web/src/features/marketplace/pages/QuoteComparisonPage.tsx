/**
 * QuoteComparisonPage - side-by-side quote comparison for an RFQ.
 * Epic 68: Service Provider Marketplace (Story 68.3)
 */

import { useState } from 'react';
import type { RfqSummary } from '../components/RfqCard';
import { QuoteCard, type QuoteSummary } from '../components/QuoteCard';
import { QuoteComparisonTable } from '../components/QuoteComparisonTable';

interface QuoteComparisonPageProps {
  rfq: RfqSummary;
  quotes: QuoteSummary[];
  isLoading?: boolean;
  onAcceptQuote: (quoteId: string) => void;
  onRejectQuote: (quoteId: string) => void;
  onViewQuote: (quoteId: string) => void;
  onViewProvider: (providerId: string) => void;
  onBack: () => void;
}

type ViewMode = 'table' | 'cards';

export function QuoteComparisonPage({
  rfq,
  quotes,
  isLoading,
  onAcceptQuote,
  onRejectQuote,
  onViewQuote,
  onViewProvider,
  onBack,
}: QuoteComparisonPageProps) {
  const [viewMode, setViewMode] = useState<ViewMode>('table');
  const [selectedQuotes, setSelectedQuotes] = useState<string[]>([]);

  const handleSelectQuote = (quoteId: string) => {
    if (selectedQuotes.includes(quoteId)) {
      setSelectedQuotes((prev) => prev.filter((id) => id !== quoteId));
    } else {
      setSelectedQuotes((prev) => [...prev, quoteId]);
    }
  };

  const quotesToCompare =
    selectedQuotes.length > 0
      ? quotes.filter((q) => selectedQuotes.includes(q.id))
      : quotes;

  if (isLoading) {
    return (
      <div className="max-w-6xl mx-auto px-4 py-8">
        <div className="animate-pulse">
          <div className="h-8 bg-gray-200 rounded w-1/4 mb-4" />
          <div className="h-64 bg-gray-200 rounded" />
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-6xl mx-auto px-4 py-8">
      {/* Back Button */}
      <button
        type="button"
        onClick={onBack}
        className="flex items-center gap-2 text-gray-600 hover:text-gray-900 mb-6"
      >
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <title>Back</title>
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
        </svg>
        Back to RFQs
      </button>

      {/* Header */}
      <div className="mb-6">
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold text-gray-900">Compare Quotes</h1>
            <p className="mt-1 text-gray-600">{rfq.title}</p>
          </div>
          <div className="flex items-center gap-2 bg-gray-100 rounded-lg p-1">
            <button
              type="button"
              onClick={() => setViewMode('table')}
              className={`px-3 py-1 text-sm font-medium rounded ${
                viewMode === 'table'
                  ? 'bg-white text-gray-900 shadow'
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <title>Table view</title>
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M3 10h18M3 14h18m-9-4v8m-7 0h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"
                />
              </svg>
            </button>
            <button
              type="button"
              onClick={() => setViewMode('cards')}
              className={`px-3 py-1 text-sm font-medium rounded ${
                viewMode === 'cards'
                  ? 'bg-white text-gray-900 shadow'
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <title>Card view</title>
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"
                />
              </svg>
            </button>
          </div>
        </div>
      </div>

      {/* RFQ Summary */}
      <div className="bg-white rounded-lg shadow p-4 mb-6">
        <div className="flex items-center justify-between text-sm">
          <div className="flex items-center gap-6">
            <div>
              <span className="text-gray-500">Category:</span>{' '}
              <span className="font-medium text-gray-900">
                {rfq.serviceCategory.replace('_', ' ').charAt(0).toUpperCase() +
                  rfq.serviceCategory.slice(1).replace('_', ' ')}
              </span>
            </div>
            {rfq.budgetMin || rfq.budgetMax ? (
              <div>
                <span className="text-gray-500">Budget:</span>{' '}
                <span className="font-medium text-gray-900">
                  {new Intl.NumberFormat('en-EU', {
                    style: 'currency',
                    currency: rfq.currency || 'EUR',
                    minimumFractionDigits: 0,
                  }).format(rfq.budgetMin || 0)}
                  {' - '}
                  {new Intl.NumberFormat('en-EU', {
                    style: 'currency',
                    currency: rfq.currency || 'EUR',
                    minimumFractionDigits: 0,
                  }).format(rfq.budgetMax || 0)}
                </span>
              </div>
            ) : null}
            <div>
              <span className="text-gray-500">Quotes:</span>{' '}
              <span className="font-medium text-gray-900">{quotes.length}</span>
            </div>
          </div>
          {rfq.isUrgent && (
            <span className="inline-flex items-center px-2 py-1 rounded text-xs font-medium bg-red-100 text-red-800">
              Urgent
            </span>
          )}
        </div>
      </div>

      {/* Quote Selection (for card view) */}
      {viewMode === 'cards' && quotes.length > 2 && (
        <div className="mb-4 flex items-center gap-4">
          <p className="text-sm text-gray-600">
            {selectedQuotes.length > 0
              ? `Comparing ${selectedQuotes.length} selected quotes`
              : 'Select quotes to compare, or view all'}
          </p>
          {selectedQuotes.length > 0 && (
            <button
              type="button"
              onClick={() => setSelectedQuotes([])}
              className="text-sm text-blue-600 hover:text-blue-800"
            >
              Show all
            </button>
          )}
        </div>
      )}

      {/* No Quotes */}
      {quotes.length === 0 ? (
        <div className="text-center py-12 bg-gray-50 rounded-lg">
          <svg
            className="mx-auto w-12 h-12 text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <title>No quotes</title>
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
            />
          </svg>
          <h3 className="mt-4 text-lg font-medium text-gray-900">No quotes yet</h3>
          <p className="mt-2 text-gray-500">
            Providers have not submitted quotes for this RFQ yet
          </p>
        </div>
      ) : viewMode === 'table' ? (
        /* Table View */
        <QuoteComparisonTable
          quotes={quotesToCompare}
          onAccept={onAcceptQuote}
          onViewProvider={onViewProvider}
          onViewQuote={onViewQuote}
        />
      ) : (
        /* Card View */
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {quotes.map((quote) => (
            <QuoteCard
              key={quote.id}
              quote={quote}
              isSelected={selectedQuotes.includes(quote.id)}
              onView={onViewQuote}
              onAccept={onAcceptQuote}
              onReject={onRejectQuote}
              onSelect={handleSelectQuote}
              onViewProvider={onViewProvider}
            />
          ))}
        </div>
      )}

      {/* Summary Stats */}
      {quotes.length > 0 && (
        <div className="mt-8 grid grid-cols-2 md:grid-cols-4 gap-4">
          <div className="bg-white rounded-lg shadow p-4 text-center">
            <p className="text-2xl font-bold text-gray-900">{quotes.length}</p>
            <p className="text-sm text-gray-500">Total Quotes</p>
          </div>
          <div className="bg-white rounded-lg shadow p-4 text-center">
            <p className="text-2xl font-bold text-green-600">
              {new Intl.NumberFormat('en-EU', {
                style: 'currency',
                currency: quotes[0]?.currency || 'EUR',
                minimumFractionDigits: 0,
              }).format(Math.min(...quotes.map((q) => q.price)))}
            </p>
            <p className="text-sm text-gray-500">Lowest Quote</p>
          </div>
          <div className="bg-white rounded-lg shadow p-4 text-center">
            <p className="text-2xl font-bold text-gray-900">
              {new Intl.NumberFormat('en-EU', {
                style: 'currency',
                currency: quotes[0]?.currency || 'EUR',
                minimumFractionDigits: 0,
              }).format(Math.max(...quotes.map((q) => q.price)))}
            </p>
            <p className="text-sm text-gray-500">Highest Quote</p>
          </div>
          <div className="bg-white rounded-lg shadow p-4 text-center">
            <p className="text-2xl font-bold text-blue-600">
              {new Intl.NumberFormat('en-EU', {
                style: 'currency',
                currency: quotes[0]?.currency || 'EUR',
                minimumFractionDigits: 0,
              }).format(quotes.reduce((sum, q) => sum + q.price, 0) / quotes.length)}
            </p>
            <p className="text-sm text-gray-500">Average Quote</p>
          </div>
        </div>
      )}
    </div>
  );
}
