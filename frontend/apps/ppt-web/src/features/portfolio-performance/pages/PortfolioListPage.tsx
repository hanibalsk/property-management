// Epic 144: Portfolio Performance Analytics - Portfolio List Page
import type React from 'react';
import { useState } from 'react';
import { PortfolioCard } from '../components';

interface PortfolioListPageProps {
  onSelectPortfolio: (portfolioId: string) => void;
  onCreatePortfolio: () => void;
}

// Mock data - would be fetched from API
const mockPortfolios = [
  {
    id: '1',
    name: 'Core Investment Portfolio',
    description: 'Primary long-term investment properties focused on cash flow',
    totalValue: 2500000,
    totalEquity: 1200000,
    totalDebt: 1300000,
    propertyCount: 4,
    targetReturnPct: 12,
    currency: 'EUR',
    isActive: true,
  },
  {
    id: '2',
    name: 'Value-Add Portfolio',
    description: 'Properties acquired for renovation and improvement',
    totalValue: 800000,
    totalEquity: 450000,
    totalDebt: 350000,
    propertyCount: 2,
    targetReturnPct: 18,
    currency: 'EUR',
    isActive: true,
  },
  {
    id: '3',
    name: 'Legacy Holdings',
    description: 'Older properties scheduled for disposition',
    totalValue: 350000,
    totalEquity: 350000,
    totalDebt: 0,
    propertyCount: 1,
    currency: 'EUR',
    isActive: false,
  },
];

export const PortfolioListPage: React.FC<PortfolioListPageProps> = ({
  onSelectPortfolio,
  onCreatePortfolio,
}) => {
  const [showInactive, setShowInactive] = useState(false);

  const filteredPortfolios = showInactive
    ? mockPortfolios
    : mockPortfolios.filter((p) => p.isActive);

  const totalStats = mockPortfolios
    .filter((p) => p.isActive)
    .reduce(
      (acc, p) => ({
        totalValue: acc.totalValue + p.totalValue,
        totalEquity: acc.totalEquity + p.totalEquity,
        propertyCount: acc.propertyCount + p.propertyCount,
      }),
      { totalValue: 0, totalEquity: 0, propertyCount: 0 }
    );

  const formatCurrency = (value: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'EUR',
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }).format(value);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Investment Portfolios</h1>
          <p className="text-sm text-gray-500">
            Manage and analyze your property investment portfolios
          </p>
        </div>
        <button
          type="button"
          onClick={onCreatePortfolio}
          className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700"
        >
          Create Portfolio
        </button>
      </div>

      {/* Summary Stats */}
      <div className="bg-white rounded-lg shadow-md p-6">
        <h2 className="text-sm font-medium text-gray-500 mb-4">
          Total Holdings (Active Portfolios)
        </h2>
        <div className="grid grid-cols-4 gap-6">
          <div>
            <p className="text-2xl font-bold text-gray-900">
              {filteredPortfolios.filter((p) => p.isActive).length}
            </p>
            <p className="text-sm text-gray-500">Active Portfolios</p>
          </div>
          <div>
            <p className="text-2xl font-bold text-gray-900">
              {formatCurrency(totalStats.totalValue)}
            </p>
            <p className="text-sm text-gray-500">Total Value</p>
          </div>
          <div>
            <p className="text-2xl font-bold text-green-600">
              {formatCurrency(totalStats.totalEquity)}
            </p>
            <p className="text-sm text-gray-500">Total Equity</p>
          </div>
          <div>
            <p className="text-2xl font-bold text-gray-900">{totalStats.propertyCount}</p>
            <p className="text-sm text-gray-500">Total Properties</p>
          </div>
        </div>
      </div>

      {/* Filter */}
      <div className="flex items-center">
        <label className="flex items-center cursor-pointer">
          <input
            type="checkbox"
            checked={showInactive}
            onChange={(e) => setShowInactive(e.target.checked)}
            className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
          />
          <span className="ml-2 text-sm text-gray-600">Show inactive portfolios</span>
        </label>
      </div>

      {/* Portfolio Grid */}
      <div className="grid grid-cols-3 gap-6">
        {filteredPortfolios.map((portfolio) => (
          <PortfolioCard
            key={portfolio.id}
            {...portfolio}
            onClick={() => onSelectPortfolio(portfolio.id)}
          />
        ))}
      </div>

      {filteredPortfolios.length === 0 && (
        <div className="text-center py-12 bg-gray-50 rounded-lg">
          <svg
            className="mx-auto h-12 w-12 text-gray-400"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"
            />
          </svg>
          <h3 className="mt-2 text-sm font-medium text-gray-900">No portfolios</h3>
          <p className="mt-1 text-sm text-gray-500">
            Get started by creating your first investment portfolio.
          </p>
          <div className="mt-6">
            <button
              type="button"
              onClick={onCreatePortfolio}
              className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700"
            >
              Create Portfolio
            </button>
          </div>
        </div>
      )}
    </div>
  );
};
