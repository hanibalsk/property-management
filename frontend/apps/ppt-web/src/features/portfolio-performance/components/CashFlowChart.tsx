// Epic 144: Portfolio Performance Analytics - Cash Flow Chart Component
import type React from 'react';

interface CashFlowDataPoint {
  period: string;
  periodDate: string;
  grossIncome: number;
  operatingExpenses: number;
  noi: number;
  debtService?: number;
  netCashFlow: number;
}

interface CashFlowChartProps {
  data: CashFlowDataPoint[];
  currency?: string;
  showDebtService?: boolean;
}

export const CashFlowChart: React.FC<CashFlowChartProps> = ({
  data,
  currency = 'EUR',
  showDebtService: _showDebtService = true,
}) => {
  const formatCurrency = (value: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: currency,
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }).format(value);
  };

  // Calculate max value for scaling
  const maxValue = Math.max(...data.map((d) => Math.max(d.grossIncome, Math.abs(d.netCashFlow))));

  const getBarHeight = (value: number) => {
    return Math.abs(value / maxValue) * 100;
  };

  if (data.length === 0) {
    return (
      <div className="bg-white rounded-lg shadow-md p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Cash Flow Trend</h3>
        <div className="flex items-center justify-center h-48 text-gray-500">
          No cash flow data available
        </div>
      </div>
    );
  }

  // Calculate totals
  const totals = data.reduce(
    (acc, d) => ({
      grossIncome: acc.grossIncome + d.grossIncome,
      operatingExpenses: acc.operatingExpenses + d.operatingExpenses,
      noi: acc.noi + d.noi,
      debtService: acc.debtService + (d.debtService || 0),
      netCashFlow: acc.netCashFlow + d.netCashFlow,
    }),
    { grossIncome: 0, operatingExpenses: 0, noi: 0, debtService: 0, netCashFlow: 0 }
  );

  return (
    <div className="bg-white rounded-lg shadow-md p-6">
      <h3 className="text-lg font-semibold text-gray-900 mb-4">Cash Flow Trend</h3>

      {/* Summary stats */}
      <div className="grid grid-cols-4 gap-4 mb-6">
        <div>
          <p className="text-xs text-gray-500">Total Income</p>
          <p className="text-sm font-semibold text-green-600">
            {formatCurrency(totals.grossIncome)}
          </p>
        </div>
        <div>
          <p className="text-xs text-gray-500">Total Expenses</p>
          <p className="text-sm font-semibold text-red-600">
            {formatCurrency(totals.operatingExpenses)}
          </p>
        </div>
        <div>
          <p className="text-xs text-gray-500">Total NOI</p>
          <p className="text-sm font-semibold text-blue-600">{formatCurrency(totals.noi)}</p>
        </div>
        <div>
          <p className="text-xs text-gray-500">Net Cash Flow</p>
          <p
            className={`text-sm font-semibold ${totals.netCashFlow >= 0 ? 'text-green-600' : 'text-red-600'}`}
          >
            {formatCurrency(totals.netCashFlow)}
          </p>
        </div>
      </div>

      {/* Chart */}
      <div className="relative h-48">
        <div className="flex items-end justify-between h-full space-x-2">
          {data.slice(-12).map((point, index) => (
            <div key={index} className="flex-1 flex flex-col items-center">
              {/* Bars */}
              <div className="relative w-full h-40 flex items-end justify-center space-x-0.5">
                {/* Income bar */}
                <div
                  className="w-2 bg-green-400 rounded-t"
                  style={{ height: `${getBarHeight(point.grossIncome)}%` }}
                  title={`Income: ${formatCurrency(point.grossIncome)}`}
                />
                {/* Expense bar */}
                <div
                  className="w-2 bg-red-400 rounded-t"
                  style={{ height: `${getBarHeight(point.operatingExpenses)}%` }}
                  title={`Expenses: ${formatCurrency(point.operatingExpenses)}`}
                />
                {/* Net cash flow bar */}
                <div
                  className={`w-2 rounded-t ${point.netCashFlow >= 0 ? 'bg-blue-500' : 'bg-orange-500'}`}
                  style={{ height: `${getBarHeight(point.netCashFlow)}%` }}
                  title={`Net: ${formatCurrency(point.netCashFlow)}`}
                />
              </div>
              {/* Label */}
              <p className="text-xs text-gray-500 mt-1 truncate w-full text-center">
                {point.period.slice(-5)}
              </p>
            </div>
          ))}
        </div>
      </div>

      {/* Legend */}
      <div className="flex items-center justify-center space-x-4 mt-4">
        <div className="flex items-center">
          <div className="w-3 h-3 bg-green-400 rounded mr-1" />
          <span className="text-xs text-gray-500">Income</span>
        </div>
        <div className="flex items-center">
          <div className="w-3 h-3 bg-red-400 rounded mr-1" />
          <span className="text-xs text-gray-500">Expenses</span>
        </div>
        <div className="flex items-center">
          <div className="w-3 h-3 bg-blue-500 rounded mr-1" />
          <span className="text-xs text-gray-500">Net Cash Flow</span>
        </div>
      </div>
    </div>
  );
};
