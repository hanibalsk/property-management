// Epic 144: Portfolio Performance Analytics - Portfolio Card Component
import type React from 'react';

interface PortfolioCardProps {
  id: string;
  name: string;
  description?: string;
  totalValue: number;
  totalEquity: number;
  totalDebt: number;
  propertyCount: number;
  targetReturnPct?: number;
  currency: string;
  isActive: boolean;
  onClick?: () => void;
}

export const PortfolioCard: React.FC<PortfolioCardProps> = ({
  name,
  description,
  totalValue,
  totalEquity,
  totalDebt,
  propertyCount,
  targetReturnPct,
  currency,
  isActive,
  onClick,
}) => {
  const formatCurrency = (value: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: currency,
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }).format(value);
  };

  const ltvRatio = totalValue > 0 ? (totalDebt / totalValue) * 100 : 0;

  return (
    <div
      className={`bg-white rounded-lg shadow-md p-6 cursor-pointer hover:shadow-lg transition-shadow ${
        !isActive ? 'opacity-60' : ''
      }`}
      onClick={onClick}
    >
      <div className="flex justify-between items-start mb-4">
        <div>
          <h3 className="text-lg font-semibold text-gray-900">{name}</h3>
          {description && <p className="text-sm text-gray-500 mt-1">{description}</p>}
        </div>
        <span
          className={`px-2 py-1 text-xs rounded-full ${
            isActive ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'
          }`}
        >
          {isActive ? 'Active' : 'Inactive'}
        </span>
      </div>

      <div className="grid grid-cols-2 gap-4 mb-4">
        <div>
          <p className="text-sm text-gray-500">Total Value</p>
          <p className="text-xl font-bold text-gray-900">{formatCurrency(totalValue)}</p>
        </div>
        <div>
          <p className="text-sm text-gray-500">Total Equity</p>
          <p className="text-xl font-bold text-green-600">{formatCurrency(totalEquity)}</p>
        </div>
      </div>

      <div className="grid grid-cols-3 gap-4 pt-4 border-t border-gray-100">
        <div>
          <p className="text-xs text-gray-500">Properties</p>
          <p className="text-sm font-semibold">{propertyCount}</p>
        </div>
        <div>
          <p className="text-xs text-gray-500">LTV Ratio</p>
          <p className="text-sm font-semibold">{ltvRatio.toFixed(1)}%</p>
        </div>
        {targetReturnPct && (
          <div>
            <p className="text-xs text-gray-500">Target Return</p>
            <p className="text-sm font-semibold">{targetReturnPct}%</p>
          </div>
        )}
      </div>
    </div>
  );
};
