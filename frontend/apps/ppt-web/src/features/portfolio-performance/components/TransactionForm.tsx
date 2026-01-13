// Epic 144: Portfolio Performance Analytics - Transaction Form Component
import type React from 'react';
import { useState } from 'react';

interface TransactionFormData {
  propertyId: string;
  transactionType: string;
  category?: string;
  amount: number;
  currency: string;
  transactionDate: string;
  periodStart?: string;
  periodEnd?: string;
  description?: string;
  vendorName?: string;
  referenceNumber?: string;
  isRecurring: boolean;
  recurrenceFrequency?: string;
}

interface TransactionFormProps {
  initialData?: Partial<TransactionFormData>;
  properties: Array<{ id: string; name: string }>;
  onSubmit: (data: TransactionFormData) => void;
  onCancel: () => void;
  isLoading?: boolean;
}

export const TransactionForm: React.FC<TransactionFormProps> = ({
  initialData,
  properties,
  onSubmit,
  onCancel,
  isLoading = false,
}) => {
  const [formData, setFormData] = useState<TransactionFormData>({
    propertyId: initialData?.propertyId || '',
    transactionType: initialData?.transactionType || 'rental_income',
    category: initialData?.category || '',
    amount: initialData?.amount || 0,
    currency: initialData?.currency || 'EUR',
    transactionDate: initialData?.transactionDate || new Date().toISOString().split('T')[0],
    periodStart: initialData?.periodStart,
    periodEnd: initialData?.periodEnd,
    description: initialData?.description || '',
    vendorName: initialData?.vendorName || '',
    referenceNumber: initialData?.referenceNumber || '',
    isRecurring: initialData?.isRecurring || false,
    recurrenceFrequency: initialData?.recurrenceFrequency || '',
  });

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>
  ) => {
    const { name, value, type } = e.target;
    const checked = (e.target as HTMLInputElement).checked;

    setFormData((prev) => ({
      ...prev,
      [name]:
        type === 'checkbox' ? checked : type === 'number' ? Number.parseFloat(value) || 0 : value,
    }));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit(formData);
  };

  const transactionTypes = [
    { value: 'rental_income', label: 'Rental Income', isIncome: true },
    { value: 'other_income', label: 'Other Income', isIncome: true },
    { value: 'operating_expense', label: 'Operating Expense', isIncome: false },
    { value: 'mortgage_payment', label: 'Mortgage Payment', isIncome: false },
    { value: 'capital_expenditure', label: 'Capital Expenditure', isIncome: false },
    { value: 'tax_payment', label: 'Tax Payment', isIncome: false },
    { value: 'insurance', label: 'Insurance', isIncome: false },
    { value: 'property_management', label: 'Property Management', isIncome: false },
    { value: 'maintenance', label: 'Maintenance', isIncome: false },
    { value: 'utilities', label: 'Utilities', isIncome: false },
    { value: 'vacancy_cost', label: 'Vacancy Cost', isIncome: false },
    { value: 'leasing_cost', label: 'Leasing Cost', isIncome: false },
    { value: 'legal_professional', label: 'Legal/Professional', isIncome: false },
    { value: 'other', label: 'Other', isIncome: false },
  ];

  const recurrenceOptions = [
    { value: 'monthly', label: 'Monthly' },
    { value: 'quarterly', label: 'Quarterly' },
    { value: 'annually', label: 'Annually' },
  ];

  const selectedType = transactionTypes.find((t) => t.value === formData.transactionType);
  const isIncome = selectedType?.isIncome || false;

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Transaction Type */}
      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Property *</label>
          <select
            name="propertyId"
            value={formData.propertyId}
            onChange={handleChange}
            required
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
          >
            <option value="">Select a property</option>
            {properties.map((property) => (
              <option key={property.id} value={property.id}>
                {property.name}
              </option>
            ))}
          </select>
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Transaction Type *</label>
          <select
            name="transactionType"
            value={formData.transactionType}
            onChange={handleChange}
            required
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
          >
            <optgroup label="Income">
              {transactionTypes
                .filter((t) => t.isIncome)
                .map((type) => (
                  <option key={type.value} value={type.value}>
                    {type.label}
                  </option>
                ))}
            </optgroup>
            <optgroup label="Expenses">
              {transactionTypes
                .filter((t) => !t.isIncome)
                .map((type) => (
                  <option key={type.value} value={type.value}>
                    {type.label}
                  </option>
                ))}
            </optgroup>
          </select>
        </div>
      </div>

      {/* Amount and Date */}
      <div className="grid grid-cols-3 gap-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Amount *</label>
          <div className="relative">
            <span
              className={`absolute left-3 top-2 text-sm ${isIncome ? 'text-green-600' : 'text-red-600'}`}
            >
              {isIncome ? '+' : '-'}
            </span>
            <input
              type="number"
              name="amount"
              value={formData.amount}
              onChange={handleChange}
              required
              min="0"
              step="0.01"
              className="w-full pl-8 pr-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Currency</label>
          <select
            name="currency"
            value={formData.currency}
            onChange={handleChange}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
          >
            <option value="EUR">EUR</option>
            <option value="USD">USD</option>
            <option value="GBP">GBP</option>
            <option value="CZK">CZK</option>
          </select>
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Transaction Date *</label>
          <input
            type="date"
            name="transactionDate"
            value={formData.transactionDate}
            onChange={handleChange}
            required
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
          />
        </div>
      </div>

      {/* Period */}
      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Period Start</label>
          <input
            type="date"
            name="periodStart"
            value={formData.periodStart || ''}
            onChange={handleChange}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Period End</label>
          <input
            type="date"
            name="periodEnd"
            value={formData.periodEnd || ''}
            onChange={handleChange}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
          />
        </div>
      </div>

      {/* Category and Vendor */}
      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Category</label>
          <input
            type="text"
            name="category"
            value={formData.category}
            onChange={handleChange}
            placeholder="e.g., Repairs, Marketing"
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Vendor/Source</label>
          <input
            type="text"
            name="vendorName"
            value={formData.vendorName}
            onChange={handleChange}
            placeholder="e.g., Tenant name, Vendor name"
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
          />
        </div>
      </div>

      {/* Reference Number */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">Reference Number</label>
        <input
          type="text"
          name="referenceNumber"
          value={formData.referenceNumber}
          onChange={handleChange}
          placeholder="Invoice or receipt number"
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
        />
      </div>

      {/* Description */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">Description</label>
        <textarea
          name="description"
          value={formData.description}
          onChange={handleChange}
          rows={2}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
          placeholder="Additional details..."
        />
      </div>

      {/* Recurring */}
      <div className="bg-gray-50 p-4 rounded-lg">
        <div className="flex items-center mb-3">
          <input
            type="checkbox"
            id="isRecurring"
            name="isRecurring"
            checked={formData.isRecurring}
            onChange={handleChange}
            className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
          />
          <label htmlFor="isRecurring" className="ml-2 text-sm font-medium text-gray-700">
            This is a recurring transaction
          </label>
        </div>
        {formData.isRecurring && (
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Recurrence Frequency
            </label>
            <select
              name="recurrenceFrequency"
              value={formData.recurrenceFrequency}
              onChange={handleChange}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="">Select frequency</option>
              {recurrenceOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </div>
        )}
      </div>

      {/* Actions */}
      <div className="flex justify-end space-x-3 pt-4 border-t">
        <button
          type="button"
          onClick={onCancel}
          className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
        >
          Cancel
        </button>
        <button
          type="submit"
          disabled={isLoading}
          className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50"
        >
          {isLoading ? 'Saving...' : 'Save Transaction'}
        </button>
      </div>
    </form>
  );
};
