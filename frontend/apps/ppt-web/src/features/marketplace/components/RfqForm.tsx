/**
 * RfqForm component - form for creating Request for Quote.
 * Epic 68: Service Provider Marketplace (Story 68.3)
 */

import { useState } from 'react';
import type { ServiceCategory } from './ProviderCard';

export interface RfqFormData {
  title: string;
  description: string;
  serviceCategory: ServiceCategory;
  scopeOfWork?: string;
  preferredStartDate?: string;
  preferredEndDate?: string;
  isUrgent?: boolean;
  budgetMin?: number;
  budgetMax?: number;
  currency?: string;
  quoteDeadline?: string;
  contactPreference?: 'email' | 'phone' | 'any';
  siteVisitRequired?: boolean;
  providerIds: string[];
}

interface RfqFormProps {
  initialData?: Partial<RfqFormData>;
  buildingId?: string;
  buildingName?: string;
  selectedProviders?: Array<{ id: string; companyName: string }>;
  onSubmit: (data: RfqFormData) => void;
  onCancel: () => void;
  onSelectProviders: () => void;
  isLoading?: boolean;
}

const categories: { value: ServiceCategory; label: string }[] = [
  { value: 'plumbing', label: 'Plumbing' },
  { value: 'electrical', label: 'Electrical' },
  { value: 'hvac', label: 'HVAC' },
  { value: 'cleaning', label: 'Cleaning' },
  { value: 'landscaping', label: 'Landscaping' },
  { value: 'security', label: 'Security' },
  { value: 'painting', label: 'Painting' },
  { value: 'roofing', label: 'Roofing' },
  { value: 'carpentry', label: 'Carpentry' },
  { value: 'locksmith', label: 'Locksmith' },
  { value: 'pest_control', label: 'Pest Control' },
  { value: 'general_maintenance', label: 'General Maintenance' },
  { value: 'elevator_maintenance', label: 'Elevator Maintenance' },
  { value: 'fire_safety', label: 'Fire Safety' },
  { value: 'waste_management', label: 'Waste Management' },
  { value: 'other', label: 'Other' },
];

const defaultFormData: RfqFormData = {
  title: '',
  description: '',
  serviceCategory: 'general_maintenance',
  currency: 'EUR',
  contactPreference: 'any',
  providerIds: [],
};

export function RfqForm({
  initialData,
  buildingName,
  selectedProviders = [],
  onSubmit,
  onCancel,
  onSelectProviders,
  isLoading,
}: RfqFormProps) {
  const [formData, setFormData] = useState<RfqFormData>({
    ...defaultFormData,
    ...initialData,
    providerIds: selectedProviders.map((p) => p.id),
  });
  const [errors, setErrors] = useState<Record<string, string>>({});

  const updateField = <K extends keyof RfqFormData>(key: K, value: RfqFormData[K]) => {
    setFormData((prev) => ({ ...prev, [key]: value }));
    if (errors[key]) {
      setErrors((prev) => {
        const next = { ...prev };
        delete next[key];
        return next;
      });
    }
  };

  const validate = (): boolean => {
    const newErrors: Record<string, string> = {};

    if (!formData.title.trim()) {
      newErrors.title = 'Title is required';
    }
    if (!formData.description.trim()) {
      newErrors.description = 'Description is required';
    }
    if (selectedProviders.length === 0) {
      newErrors.providers = 'Select at least one provider to send the RFQ to';
    }
    if (formData.budgetMin && formData.budgetMax && formData.budgetMin > formData.budgetMax) {
      newErrors.budgetMax = 'Maximum budget must be greater than minimum';
    }
    if (formData.preferredStartDate && formData.preferredEndDate) {
      if (new Date(formData.preferredStartDate) > new Date(formData.preferredEndDate)) {
        newErrors.preferredEndDate = 'End date must be after start date';
      }
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (validate()) {
      onSubmit({
        ...formData,
        providerIds: selectedProviders.map((p) => p.id),
      });
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-gray-900 mb-6">Create Request for Quote</h2>

        {buildingName && (
          <div className="mb-6 p-4 bg-gray-50 rounded-lg">
            <p className="text-sm text-gray-600">
              Building: <span className="font-medium text-gray-900">{buildingName}</span>
            </p>
          </div>
        )}

        {/* Basic Info */}
        <div className="space-y-4">
          <div>
            <label htmlFor="title" className="block text-sm font-medium text-gray-700 mb-1">
              Title *
            </label>
            <input
              id="title"
              type="text"
              value={formData.title}
              onChange={(e) => updateField('title', e.target.value)}
              placeholder="e.g., Annual HVAC Maintenance"
              className={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 ${
                errors.title ? 'border-red-500' : 'border-gray-300'
              }`}
            />
            {errors.title && <p className="mt-1 text-sm text-red-600">{errors.title}</p>}
          </div>

          <div>
            <label htmlFor="serviceCategory" className="block text-sm font-medium text-gray-700 mb-1">
              Service Category *
            </label>
            <select
              id="serviceCategory"
              value={formData.serviceCategory}
              onChange={(e) => updateField('serviceCategory', e.target.value as ServiceCategory)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
            >
              {categories.map((cat) => (
                <option key={cat.value} value={cat.value}>
                  {cat.label}
                </option>
              ))}
            </select>
          </div>

          <div>
            <label htmlFor="description" className="block text-sm font-medium text-gray-700 mb-1">
              Description *
            </label>
            <textarea
              id="description"
              rows={4}
              value={formData.description}
              onChange={(e) => updateField('description', e.target.value)}
              placeholder="Describe what you need in detail..."
              className={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 ${
                errors.description ? 'border-red-500' : 'border-gray-300'
              }`}
            />
            {errors.description && <p className="mt-1 text-sm text-red-600">{errors.description}</p>}
          </div>

          <div>
            <label htmlFor="scopeOfWork" className="block text-sm font-medium text-gray-700 mb-1">
              Scope of Work
            </label>
            <textarea
              id="scopeOfWork"
              rows={3}
              value={formData.scopeOfWork || ''}
              onChange={(e) => updateField('scopeOfWork', e.target.value)}
              placeholder="Detailed scope, specifications, requirements..."
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
            />
          </div>
        </div>

        {/* Timeline */}
        <div className="mt-6 pt-6 border-t">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Timeline</h3>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label htmlFor="preferredStartDate" className="block text-sm font-medium text-gray-700 mb-1">
                Preferred Start Date
              </label>
              <input
                id="preferredStartDate"
                type="date"
                value={formData.preferredStartDate || ''}
                onChange={(e) => updateField('preferredStartDate', e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div>
              <label htmlFor="preferredEndDate" className="block text-sm font-medium text-gray-700 mb-1">
                Preferred End Date
              </label>
              <input
                id="preferredEndDate"
                type="date"
                value={formData.preferredEndDate || ''}
                onChange={(e) => updateField('preferredEndDate', e.target.value)}
                className={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 ${
                  errors.preferredEndDate ? 'border-red-500' : 'border-gray-300'
                }`}
              />
              {errors.preferredEndDate && (
                <p className="mt-1 text-sm text-red-600">{errors.preferredEndDate}</p>
              )}
            </div>

            <div>
              <label htmlFor="quoteDeadline" className="block text-sm font-medium text-gray-700 mb-1">
                Quote Deadline
              </label>
              <input
                id="quoteDeadline"
                type="datetime-local"
                value={formData.quoteDeadline || ''}
                onChange={(e) => updateField('quoteDeadline', e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
              />
              <p className="mt-1 text-xs text-gray-500">When should providers submit their quotes by?</p>
            </div>

            <div className="flex items-end">
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="checkbox"
                  checked={formData.isUrgent || false}
                  onChange={(e) => updateField('isUrgent', e.target.checked)}
                  className="w-4 h-4 text-red-600 border-gray-300 rounded focus:ring-red-500"
                />
                <span className="text-sm text-gray-700">Mark as Urgent</span>
              </label>
            </div>
          </div>
        </div>

        {/* Budget */}
        <div className="mt-6 pt-6 border-t">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Budget (Optional)</h3>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div>
              <label htmlFor="budgetMin" className="block text-sm font-medium text-gray-700 mb-1">
                Minimum Budget
              </label>
              <input
                id="budgetMin"
                type="number"
                min="0"
                step="0.01"
                value={formData.budgetMin || ''}
                onChange={(e) =>
                  updateField('budgetMin', e.target.value ? Number(e.target.value) : undefined)
                }
                placeholder="0.00"
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div>
              <label htmlFor="budgetMax" className="block text-sm font-medium text-gray-700 mb-1">
                Maximum Budget
              </label>
              <input
                id="budgetMax"
                type="number"
                min="0"
                step="0.01"
                value={formData.budgetMax || ''}
                onChange={(e) =>
                  updateField('budgetMax', e.target.value ? Number(e.target.value) : undefined)
                }
                placeholder="0.00"
                className={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 ${
                  errors.budgetMax ? 'border-red-500' : 'border-gray-300'
                }`}
              />
              {errors.budgetMax && <p className="mt-1 text-sm text-red-600">{errors.budgetMax}</p>}
            </div>

            <div>
              <label htmlFor="currency" className="block text-sm font-medium text-gray-700 mb-1">
                Currency
              </label>
              <select
                id="currency"
                value={formData.currency || 'EUR'}
                onChange={(e) => updateField('currency', e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
              >
                <option value="EUR">EUR</option>
                <option value="CZK">CZK</option>
                <option value="USD">USD</option>
                <option value="GBP">GBP</option>
              </select>
            </div>
          </div>
        </div>

        {/* Preferences */}
        <div className="mt-6 pt-6 border-t">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Preferences</h3>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label htmlFor="contactPreference" className="block text-sm font-medium text-gray-700 mb-1">
                Contact Preference
              </label>
              <select
                id="contactPreference"
                value={formData.contactPreference || 'any'}
                onChange={(e) =>
                  updateField('contactPreference', e.target.value as RfqFormData['contactPreference'])
                }
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
              >
                <option value="any">Any</option>
                <option value="email">Email Only</option>
                <option value="phone">Phone Only</option>
              </select>
            </div>

            <div className="flex items-end">
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="checkbox"
                  checked={formData.siteVisitRequired || false}
                  onChange={(e) => updateField('siteVisitRequired', e.target.checked)}
                  className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                />
                <span className="text-sm text-gray-700">Site Visit Required Before Quote</span>
              </label>
            </div>
          </div>
        </div>

        {/* Selected Providers */}
        <div className="mt-6 pt-6 border-t">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-medium text-gray-900">Send RFQ To</h3>
            <button
              type="button"
              onClick={onSelectProviders}
              className="text-sm font-medium text-blue-600 hover:text-blue-800"
            >
              {selectedProviders.length > 0 ? 'Modify Selection' : 'Select Providers'}
            </button>
          </div>

          {errors.providers && <p className="mb-2 text-sm text-red-600">{errors.providers}</p>}

          {selectedProviders.length === 0 ? (
            <div className="text-center py-8 bg-gray-50 rounded-lg border-2 border-dashed border-gray-300">
              <p className="text-gray-500">No providers selected</p>
              <button
                type="button"
                onClick={onSelectProviders}
                className="mt-2 text-sm font-medium text-blue-600 hover:text-blue-800"
              >
                Browse and select providers
              </button>
            </div>
          ) : (
            <div className="space-y-2">
              {selectedProviders.map((provider) => (
                <div
                  key={provider.id}
                  className="flex items-center justify-between p-3 bg-gray-50 rounded-lg"
                >
                  <span className="font-medium text-gray-900">{provider.companyName}</span>
                  <button
                    type="button"
                    onClick={() => {
                      // Remove provider from selection
                      const newProviders = selectedProviders.filter((p) => p.id !== provider.id);
                      updateField(
                        'providerIds',
                        newProviders.map((p) => p.id)
                      );
                    }}
                    className="text-sm text-red-600 hover:text-red-800"
                  >
                    Remove
                  </button>
                </div>
              ))}
              <p className="text-sm text-gray-500">
                {selectedProviders.length} provider{selectedProviders.length !== 1 ? 's' : ''} will
                receive this RFQ
              </p>
            </div>
          )}
        </div>
      </div>

      {/* Actions */}
      <div className="flex items-center justify-end gap-3">
        <button
          type="button"
          onClick={onCancel}
          className="px-4 py-2 text-gray-700 font-medium hover:text-gray-900"
        >
          Cancel
        </button>
        <button
          type="submit"
          disabled={isLoading}
          className="px-6 py-2 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
        >
          {isLoading ? 'Sending...' : 'Send RFQ'}
        </button>
      </div>
    </form>
  );
}
