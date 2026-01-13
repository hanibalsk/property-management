// Epic 144: Portfolio Performance Analytics - Property Form Component
import type React from 'react';
import { useState } from 'react';

interface PropertyFormData {
  buildingId: string;
  propertyName?: string;
  acquisitionDate: string;
  acquisitionPrice: number;
  acquisitionCosts?: number;
  financingType: string;
  downPayment?: number;
  loanAmount?: number;
  interestRate?: number;
  loanTermYears?: number;
  monthlyPayment?: number;
  loanStartDate?: string;
  ownershipPercentage: number;
  currentValue?: number;
  currency: string;
  notes?: string;
}

interface PropertyFormProps {
  initialData?: Partial<PropertyFormData>;
  buildings: Array<{ id: string; name: string; address?: string }>;
  onSubmit: (data: PropertyFormData) => void;
  onCancel: () => void;
  isLoading?: boolean;
}

export const PropertyForm: React.FC<PropertyFormProps> = ({
  initialData,
  buildings,
  onSubmit,
  onCancel,
  isLoading = false,
}) => {
  const [formData, setFormData] = useState<PropertyFormData>({
    buildingId: initialData?.buildingId || '',
    propertyName: initialData?.propertyName || '',
    acquisitionDate: initialData?.acquisitionDate || new Date().toISOString().split('T')[0],
    acquisitionPrice: initialData?.acquisitionPrice || 0,
    acquisitionCosts: initialData?.acquisitionCosts,
    financingType: initialData?.financingType || 'mortgage',
    downPayment: initialData?.downPayment,
    loanAmount: initialData?.loanAmount,
    interestRate: initialData?.interestRate,
    loanTermYears: initialData?.loanTermYears || 30,
    monthlyPayment: initialData?.monthlyPayment,
    loanStartDate: initialData?.loanStartDate,
    ownershipPercentage: initialData?.ownershipPercentage || 100,
    currentValue: initialData?.currentValue,
    currency: initialData?.currency || 'EUR',
    notes: initialData?.notes || '',
  });

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>
  ) => {
    const { name, value, type } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: type === 'number' ? Number.parseFloat(value) || 0 : value,
    }));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit(formData);
  };

  const financingTypes = [
    { value: 'cash', label: 'Cash' },
    { value: 'mortgage', label: 'Mortgage' },
    { value: 'commercial', label: 'Commercial Loan' },
    { value: 'private_lending', label: 'Private Lending' },
    { value: 'partnership', label: 'Partnership' },
    { value: 'syndication', label: 'Syndication' },
    { value: 'mixed', label: 'Mixed' },
  ];

  const showLoanFields = formData.financingType !== 'cash';

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Property Selection */}
      <div className="bg-gray-50 p-4 rounded-lg">
        <h3 className="text-sm font-semibold text-gray-700 mb-3">Property Details</h3>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">Building *</label>
            <select
              name="buildingId"
              value={formData.buildingId}
              onChange={handleChange}
              required
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="">Select a building</option>
              {buildings.map((building) => (
                <option key={building.id} value={building.id}>
                  {building.name} {building.address ? `- ${building.address}` : ''}
                </option>
              ))}
            </select>
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Property Name (Optional)
            </label>
            <input
              type="text"
              name="propertyName"
              value={formData.propertyName}
              onChange={handleChange}
              placeholder="Custom name for this property"
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
        </div>
      </div>

      {/* Acquisition Details */}
      <div className="bg-gray-50 p-4 rounded-lg">
        <h3 className="text-sm font-semibold text-gray-700 mb-3">Acquisition Details</h3>
        <div className="grid grid-cols-3 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Acquisition Date *
            </label>
            <input
              type="date"
              name="acquisitionDate"
              value={formData.acquisitionDate}
              onChange={handleChange}
              required
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">Purchase Price *</label>
            <input
              type="number"
              name="acquisitionPrice"
              value={formData.acquisitionPrice}
              onChange={handleChange}
              required
              min="0"
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">Closing Costs</label>
            <input
              type="number"
              name="acquisitionCosts"
              value={formData.acquisitionCosts || ''}
              onChange={handleChange}
              min="0"
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
        </div>
      </div>

      {/* Financing Details */}
      <div className="bg-gray-50 p-4 rounded-lg">
        <h3 className="text-sm font-semibold text-gray-700 mb-3">Financing Details</h3>
        <div className="grid grid-cols-3 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">Financing Type *</label>
            <select
              name="financingType"
              value={formData.financingType}
              onChange={handleChange}
              required
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            >
              {financingTypes.map((type) => (
                <option key={type.value} value={type.value}>
                  {type.label}
                </option>
              ))}
            </select>
          </div>
          {showLoanFields && (
            <>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Down Payment</label>
                <input
                  type="number"
                  name="downPayment"
                  value={formData.downPayment || ''}
                  onChange={handleChange}
                  min="0"
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Loan Amount</label>
                <input
                  type="number"
                  name="loanAmount"
                  value={formData.loanAmount || ''}
                  onChange={handleChange}
                  min="0"
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                />
              </div>
            </>
          )}
        </div>
        {showLoanFields && (
          <div className="grid grid-cols-4 gap-4 mt-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Interest Rate (%)
              </label>
              <input
                type="number"
                name="interestRate"
                value={formData.interestRate || ''}
                onChange={handleChange}
                step="0.01"
                min="0"
                max="100"
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Loan Term (Years)
              </label>
              <input
                type="number"
                name="loanTermYears"
                value={formData.loanTermYears || ''}
                onChange={handleChange}
                min="1"
                max="40"
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Monthly Payment
              </label>
              <input
                type="number"
                name="monthlyPayment"
                value={formData.monthlyPayment || ''}
                onChange={handleChange}
                min="0"
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Loan Start Date
              </label>
              <input
                type="date"
                name="loanStartDate"
                value={formData.loanStartDate || ''}
                onChange={handleChange}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
              />
            </div>
          </div>
        )}
      </div>

      {/* Ownership & Current Value */}
      <div className="bg-gray-50 p-4 rounded-lg">
        <h3 className="text-sm font-semibold text-gray-700 mb-3">Ownership & Valuation</h3>
        <div className="grid grid-cols-3 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">Ownership (%) *</label>
            <input
              type="number"
              name="ownershipPercentage"
              value={formData.ownershipPercentage}
              onChange={handleChange}
              required
              min="0"
              max="100"
              step="0.01"
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">Current Value</label>
            <input
              type="number"
              name="currentValue"
              value={formData.currentValue || ''}
              onChange={handleChange}
              min="0"
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            />
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
        </div>
      </div>

      {/* Notes */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">Notes</label>
        <textarea
          name="notes"
          value={formData.notes}
          onChange={handleChange}
          rows={3}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
          placeholder="Additional notes about this property..."
        />
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
          {isLoading ? 'Saving...' : 'Save Property'}
        </button>
      </div>
    </form>
  );
};
