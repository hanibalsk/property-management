/**
 * CurrencyExposureChart - Story 145.5
 *
 * Chart component displaying currency exposure analysis.
 */

interface CurrencyExposure {
  currency: string;
  receivablesAmount: number;
  payablesAmount: number;
  netExposure: number;
  assetValue: number;
  projectedRevenue: number;
  projectedExpenses: number;
}

interface CurrencyExposureChartProps {
  exposures: CurrencyExposure[];
  baseCurrency: string;
  isLoading?: boolean;
}

export function CurrencyExposureChart({
  exposures,
  baseCurrency,
  isLoading,
}: CurrencyExposureChartProps) {
  const formatCurrency = (amount: number, currency: string) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: currency,
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }).format(amount);
  };

  const getExposureColor = (netExposure: number) => {
    if (netExposure > 0) return 'text-green-600';
    if (netExposure < 0) return 'text-red-600';
    return 'text-gray-600';
  };

  const getBarWidth = (amount: number, maxAmount: number) => {
    if (maxAmount === 0) return 0;
    return Math.min((Math.abs(amount) / maxAmount) * 100, 100);
  };

  if (isLoading) {
    return (
      <div className="flex justify-center items-center py-8">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" />
      </div>
    );
  }

  if (exposures.length === 0) {
    return (
      <div className="text-center py-8 text-gray-500">No currency exposure data available.</div>
    );
  }

  const maxAmount = Math.max(
    ...exposures.flatMap((e) => [
      Math.abs(e.receivablesAmount),
      Math.abs(e.payablesAmount),
      Math.abs(e.netExposure),
    ])
  );

  return (
    <div className="space-y-6">
      {/* Summary Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div className="bg-green-50 rounded-lg p-4">
          <div className="text-sm text-green-600 font-medium">Total Receivables</div>
          <div className="text-2xl font-bold text-green-700">
            {formatCurrency(
              exposures.reduce((sum, e) => sum + e.receivablesAmount, 0),
              baseCurrency
            )}
          </div>
        </div>
        <div className="bg-red-50 rounded-lg p-4">
          <div className="text-sm text-red-600 font-medium">Total Payables</div>
          <div className="text-2xl font-bold text-red-700">
            {formatCurrency(
              exposures.reduce((sum, e) => sum + e.payablesAmount, 0),
              baseCurrency
            )}
          </div>
        </div>
        <div className="bg-blue-50 rounded-lg p-4">
          <div className="text-sm text-blue-600 font-medium">Net Exposure</div>
          <div className="text-2xl font-bold text-blue-700">
            {formatCurrency(
              exposures.reduce((sum, e) => sum + e.netExposure, 0),
              baseCurrency
            )}
          </div>
        </div>
      </div>

      {/* Currency Breakdown */}
      <div className="bg-white rounded-lg shadow border border-gray-200">
        <div className="px-4 py-3 border-b border-gray-200">
          <h3 className="text-lg font-medium text-gray-900">Exposure by Currency</h3>
        </div>
        <div className="p-4 space-y-4">
          {exposures.map((exposure) => (
            <div key={exposure.currency} className="space-y-2">
              <div className="flex items-center justify-between">
                <div className="flex items-center space-x-2">
                  <span className="font-semibold text-gray-900">{exposure.currency}</span>
                  <span className={`text-sm font-medium ${getExposureColor(exposure.netExposure)}`}>
                    Net: {formatCurrency(exposure.netExposure, exposure.currency)}
                  </span>
                </div>
              </div>

              {/* Receivables Bar */}
              <div className="flex items-center space-x-2">
                <span className="w-24 text-xs text-gray-500">Receivables</span>
                <div className="flex-1 bg-gray-100 rounded h-4 overflow-hidden">
                  <div
                    className="bg-green-500 h-full transition-all duration-300"
                    style={{ width: `${getBarWidth(exposure.receivablesAmount, maxAmount)}%` }}
                  />
                </div>
                <span className="w-32 text-xs text-right text-gray-600">
                  {formatCurrency(exposure.receivablesAmount, exposure.currency)}
                </span>
              </div>

              {/* Payables Bar */}
              <div className="flex items-center space-x-2">
                <span className="w-24 text-xs text-gray-500">Payables</span>
                <div className="flex-1 bg-gray-100 rounded h-4 overflow-hidden">
                  <div
                    className="bg-red-500 h-full transition-all duration-300"
                    style={{ width: `${getBarWidth(exposure.payablesAmount, maxAmount)}%` }}
                  />
                </div>
                <span className="w-32 text-xs text-right text-gray-600">
                  {formatCurrency(exposure.payablesAmount, exposure.currency)}
                </span>
              </div>

              {/* Projected Revenue/Expenses */}
              <div className="flex space-x-4 text-xs text-gray-500 mt-1">
                <span>
                  Projected Revenue: {formatCurrency(exposure.projectedRevenue, exposure.currency)}
                </span>
                <span>
                  Projected Expenses:{' '}
                  {formatCurrency(exposure.projectedExpenses, exposure.currency)}
                </span>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
