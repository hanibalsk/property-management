/**
 * TransactionsList - Story 145.3
 *
 * List component displaying multi-currency transactions.
 */

interface Transaction {
  id: string;
  sourceType: string;
  sourceId: string;
  originalCurrency: string;
  originalAmount: number;
  baseCurrency: string;
  convertedAmount: number;
  exchangeRate: number;
  rateDate: string;
  conversionStatus: string;
  isRateOverride: boolean;
  realizedGainLoss: number;
  createdAt: string;
}

interface TransactionsListProps {
  transactions: Transaction[];
  isLoading?: boolean;
  onViewDetails?: (transaction: Transaction) => void;
  onUpdateRate?: (transaction: Transaction) => void;
}

export function TransactionsList({
  transactions,
  isLoading,
  onViewDetails,
  onUpdateRate,
}: TransactionsListProps) {
  const formatCurrency = (amount: number, currency: string) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: currency,
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    }).format(amount);
  };

  const getStatusBadge = (status: string) => {
    switch (status) {
      case 'converted':
        return (
          <span className="px-2 py-1 text-xs font-medium bg-green-100 text-green-800 rounded">
            Converted
          </span>
        );
      case 'pending':
        return (
          <span className="px-2 py-1 text-xs font-medium bg-yellow-100 text-yellow-800 rounded">
            Pending
          </span>
        );
      case 'failed':
        return (
          <span className="px-2 py-1 text-xs font-medium bg-red-100 text-red-800 rounded">
            Failed
          </span>
        );
      case 'manual':
        return (
          <span className="px-2 py-1 text-xs font-medium bg-blue-100 text-blue-800 rounded">
            Manual
          </span>
        );
      default:
        return (
          <span className="px-2 py-1 text-xs font-medium bg-gray-100 text-gray-800 rounded">
            {status}
          </span>
        );
    }
  };

  const getSourceTypeLabel = (type: string) => {
    switch (type) {
      case 'invoice':
        return 'Invoice';
      case 'payment':
        return 'Payment';
      case 'lease_payment':
        return 'Lease Payment';
      case 'expense':
        return 'Expense';
      case 'rental_income':
        return 'Rental Income';
      default:
        return type;
    }
  };

  if (isLoading) {
    return (
      <div className="flex justify-center items-center py-8">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" />
      </div>
    );
  }

  if (transactions.length === 0) {
    return <div className="text-center py-8 text-gray-500">No transactions found.</div>;
  }

  return (
    <div className="space-y-4">
      {transactions.map((transaction) => (
        <div key={transaction.id} className="bg-white rounded-lg shadow border border-gray-200 p-4">
          <div className="flex items-start justify-between">
            <div>
              <div className="flex items-center space-x-2 mb-1">
                <span className="text-sm font-medium text-gray-500">
                  {getSourceTypeLabel(transaction.sourceType)}
                </span>
                {getStatusBadge(transaction.conversionStatus)}
                {transaction.isRateOverride && (
                  <span className="px-2 py-1 text-xs font-medium bg-yellow-100 text-yellow-800 rounded">
                    Rate Override
                  </span>
                )}
              </div>
              <div className="text-xs text-gray-400">
                {new Date(transaction.createdAt).toLocaleString()}
              </div>
            </div>
            <div className="flex space-x-2">
              {onViewDetails && (
                <button
                  type="button"
                  onClick={() => onViewDetails(transaction)}
                  className="text-blue-600 hover:text-blue-800 text-sm font-medium"
                >
                  View
                </button>
              )}
              {onUpdateRate && (
                <button
                  type="button"
                  onClick={() => onUpdateRate(transaction)}
                  className="text-gray-600 hover:text-gray-800 text-sm font-medium"
                >
                  Update Rate
                </button>
              )}
            </div>
          </div>

          <div className="mt-4 grid grid-cols-1 md:grid-cols-3 gap-4">
            {/* Original Amount */}
            <div>
              <div className="text-xs text-gray-500 uppercase">Original</div>
              <div className="text-lg font-semibold text-gray-900">
                {formatCurrency(transaction.originalAmount, transaction.originalCurrency)}
              </div>
            </div>

            {/* Exchange Rate */}
            <div className="flex items-center">
              <div>
                <div className="text-xs text-gray-500 uppercase">Rate</div>
                <div className="text-sm text-gray-600">
                  1 {transaction.originalCurrency} = {transaction.exchangeRate.toFixed(4)}{' '}
                  {transaction.baseCurrency}
                </div>
                <div className="text-xs text-gray-400">
                  {new Date(transaction.rateDate).toLocaleDateString()}
                </div>
              </div>
            </div>

            {/* Converted Amount */}
            <div>
              <div className="text-xs text-gray-500 uppercase">Converted</div>
              <div className="text-lg font-semibold text-gray-900">
                {formatCurrency(transaction.convertedAmount, transaction.baseCurrency)}
              </div>
              {transaction.realizedGainLoss !== 0 && (
                <div
                  className={`text-xs ${
                    transaction.realizedGainLoss > 0 ? 'text-green-600' : 'text-red-600'
                  }`}
                >
                  FX {transaction.realizedGainLoss > 0 ? 'Gain' : 'Loss'}:{' '}
                  {formatCurrency(Math.abs(transaction.realizedGainLoss), transaction.baseCurrency)}
                </div>
              )}
            </div>
          </div>
        </div>
      ))}
    </div>
  );
}
