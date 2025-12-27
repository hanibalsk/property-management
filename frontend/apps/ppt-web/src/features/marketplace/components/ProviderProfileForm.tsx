/**
 * ProviderProfileForm component - form for creating/editing provider profiles.
 * Epic 68: Service Provider Marketplace (Story 68.1)
 */

import { useState } from 'react';
import type { ServiceCategory } from './ProviderCard';

export interface ProviderProfileFormData {
  companyName: string;
  businessRegistrationNumber?: string;
  taxId?: string;
  description?: string;
  logoUrl?: string;
  website?: string;
  contactName: string;
  contactEmail: string;
  contactPhone?: string;
  address?: string;
  city?: string;
  postalCode?: string;
  country?: string;
  serviceCategories: ServiceCategory[];
  serviceDescription?: string;
  specializations?: string[];
  coveragePostalCodes?: string[];
  coverageRadiusKm?: number;
  coverageRegions?: string[];
  pricingType: 'hourly' | 'project' | 'fixed' | 'quote_required';
  hourlyRateMin?: number;
  hourlyRateMax?: number;
  currency?: string;
  responseTimeHours?: number;
  emergencyAvailable?: boolean;
  portfolioImages?: string[];
  portfolioDescription?: string;
}

interface ProviderProfileFormProps {
  initialData?: Partial<ProviderProfileFormData>;
  onSubmit: (data: ProviderProfileFormData) => void;
  onCancel: () => void;
  isLoading?: boolean;
  isEdit?: boolean;
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

const defaultFormData: ProviderProfileFormData = {
  companyName: '',
  contactName: '',
  contactEmail: '',
  serviceCategories: [],
  pricingType: 'hourly',
  currency: 'EUR',
};

export function ProviderProfileForm({
  initialData,
  onSubmit,
  onCancel,
  isLoading,
  isEdit,
}: ProviderProfileFormProps) {
  const [formData, setFormData] = useState<ProviderProfileFormData>({
    ...defaultFormData,
    ...initialData,
  });
  const [errors, setErrors] = useState<Record<string, string>>({});
  const [currentStep, setCurrentStep] = useState(1);

  const totalSteps = 4;

  const updateField = <K extends keyof ProviderProfileFormData>(
    key: K,
    value: ProviderProfileFormData[K]
  ) => {
    setFormData((prev) => ({ ...prev, [key]: value }));
    if (errors[key]) {
      setErrors((prev) => {
        const next = { ...prev };
        delete next[key];
        return next;
      });
    }
  };

  const toggleCategory = (cat: ServiceCategory) => {
    const current = formData.serviceCategories;
    if (current.includes(cat)) {
      updateField(
        'serviceCategories',
        current.filter((c) => c !== cat)
      );
    } else {
      updateField('serviceCategories', [...current, cat]);
    }
  };

  const validateStep = (step: number): boolean => {
    const newErrors: Record<string, string> = {};

    if (step === 1) {
      if (!formData.companyName.trim()) {
        newErrors.companyName = 'Company name is required';
      }
      if (!formData.contactName.trim()) {
        newErrors.contactName = 'Contact name is required';
      }
      if (!formData.contactEmail.trim()) {
        newErrors.contactEmail = 'Contact email is required';
      } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.contactEmail)) {
        newErrors.contactEmail = 'Invalid email format';
      }
    }

    if (step === 2) {
      if (formData.serviceCategories.length === 0) {
        newErrors.serviceCategories = 'Select at least one service category';
      }
    }

    if (step === 3) {
      if (formData.pricingType === 'hourly') {
        if (formData.hourlyRateMin && formData.hourlyRateMax) {
          if (formData.hourlyRateMin > formData.hourlyRateMax) {
            newErrors.hourlyRateMax = 'Maximum rate must be greater than minimum';
          }
        }
      }
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleNext = () => {
    if (validateStep(currentStep)) {
      setCurrentStep((prev) => Math.min(prev + 1, totalSteps));
    }
  };

  const handlePrev = () => {
    setCurrentStep((prev) => Math.max(prev - 1, 1));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (validateStep(currentStep)) {
      onSubmit(formData);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="max-w-2xl mx-auto">
      {/* Progress Indicator */}
      <div className="mb-8">
        <div className="flex items-center justify-between">
          {['Company Info', 'Services', 'Pricing', 'Portfolio'].map((label, index) => (
            <div key={label} className="flex items-center">
              <div
                className={`w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium ${
                  index + 1 < currentStep
                    ? 'bg-green-500 text-white'
                    : index + 1 === currentStep
                      ? 'bg-blue-600 text-white'
                      : 'bg-gray-200 text-gray-600'
                }`}
              >
                {index + 1 < currentStep ? (
                  <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                    <title>Completed</title>
                    <path
                      fillRule="evenodd"
                      d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                      clipRule="evenodd"
                    />
                  </svg>
                ) : (
                  index + 1
                )}
              </div>
              <span
                className={`ml-2 text-sm ${index + 1 === currentStep ? 'font-medium' : 'text-gray-500'}`}
              >
                {label}
              </span>
              {index < 3 && <div className="w-12 h-0.5 bg-gray-200 mx-4" />}
            </div>
          ))}
        </div>
      </div>

      {/* Step 1: Company Information */}
      {currentStep === 1 && (
        <div className="space-y-6">
          <h2 className="text-xl font-semibold text-gray-900">Company Information</h2>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="md:col-span-2">
              <label htmlFor="companyName" className="block text-sm font-medium text-gray-700 mb-1">
                Company Name *
              </label>
              <input
                id="companyName"
                type="text"
                value={formData.companyName}
                onChange={(e) => updateField('companyName', e.target.value)}
                className={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 ${
                  errors.companyName ? 'border-red-500' : 'border-gray-300'
                }`}
              />
              {errors.companyName && (
                <p className="mt-1 text-sm text-red-600">{errors.companyName}</p>
              )}
            </div>

            <div>
              <label
                htmlFor="businessRegistrationNumber"
                className="block text-sm font-medium text-gray-700 mb-1"
              >
                Business Registration Number
              </label>
              <input
                id="businessRegistrationNumber"
                type="text"
                value={formData.businessRegistrationNumber || ''}
                onChange={(e) => updateField('businessRegistrationNumber', e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div>
              <label htmlFor="taxId" className="block text-sm font-medium text-gray-700 mb-1">
                Tax ID
              </label>
              <input
                id="taxId"
                type="text"
                value={formData.taxId || ''}
                onChange={(e) => updateField('taxId', e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div className="md:col-span-2">
              <label htmlFor="description" className="block text-sm font-medium text-gray-700 mb-1">
                Company Description
              </label>
              <textarea
                id="description"
                rows={4}
                value={formData.description || ''}
                onChange={(e) => updateField('description', e.target.value)}
                placeholder="Tell potential clients about your company, experience, and what sets you apart..."
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div>
              <label htmlFor="website" className="block text-sm font-medium text-gray-700 mb-1">
                Website
              </label>
              <input
                id="website"
                type="url"
                value={formData.website || ''}
                onChange={(e) => updateField('website', e.target.value)}
                placeholder="https://www.example.com"
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div>
              <label htmlFor="logoUrl" className="block text-sm font-medium text-gray-700 mb-1">
                Logo URL
              </label>
              <input
                id="logoUrl"
                type="url"
                value={formData.logoUrl || ''}
                onChange={(e) => updateField('logoUrl', e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
              />
            </div>
          </div>

          <h3 className="text-lg font-medium text-gray-900 pt-4">Contact Information</h3>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label htmlFor="contactName" className="block text-sm font-medium text-gray-700 mb-1">
                Contact Name *
              </label>
              <input
                id="contactName"
                type="text"
                value={formData.contactName}
                onChange={(e) => updateField('contactName', e.target.value)}
                className={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 ${
                  errors.contactName ? 'border-red-500' : 'border-gray-300'
                }`}
              />
              {errors.contactName && (
                <p className="mt-1 text-sm text-red-600">{errors.contactName}</p>
              )}
            </div>

            <div>
              <label
                htmlFor="contactEmail"
                className="block text-sm font-medium text-gray-700 mb-1"
              >
                Contact Email *
              </label>
              <input
                id="contactEmail"
                type="email"
                value={formData.contactEmail}
                onChange={(e) => updateField('contactEmail', e.target.value)}
                className={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 ${
                  errors.contactEmail ? 'border-red-500' : 'border-gray-300'
                }`}
              />
              {errors.contactEmail && (
                <p className="mt-1 text-sm text-red-600">{errors.contactEmail}</p>
              )}
            </div>

            <div>
              <label
                htmlFor="contactPhone"
                className="block text-sm font-medium text-gray-700 mb-1"
              >
                Contact Phone
              </label>
              <input
                id="contactPhone"
                type="tel"
                value={formData.contactPhone || ''}
                onChange={(e) => updateField('contactPhone', e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div>
              <label htmlFor="city" className="block text-sm font-medium text-gray-700 mb-1">
                City
              </label>
              <input
                id="city"
                type="text"
                value={formData.city || ''}
                onChange={(e) => updateField('city', e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div className="md:col-span-2">
              <label htmlFor="address" className="block text-sm font-medium text-gray-700 mb-1">
                Address
              </label>
              <input
                id="address"
                type="text"
                value={formData.address || ''}
                onChange={(e) => updateField('address', e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
              />
            </div>
          </div>
        </div>
      )}

      {/* Step 2: Services */}
      {currentStep === 2 && (
        <div className="space-y-6">
          <h2 className="text-xl font-semibold text-gray-900">Services Offered</h2>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-3">
              Select Service Categories *
            </label>
            {errors.serviceCategories && (
              <p className="mb-2 text-sm text-red-600">{errors.serviceCategories}</p>
            )}
            <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
              {categories.map((cat) => (
                <label
                  key={cat.value}
                  className={`flex items-center p-3 border rounded-lg cursor-pointer transition-colors ${
                    formData.serviceCategories.includes(cat.value)
                      ? 'border-blue-500 bg-blue-50'
                      : 'border-gray-300 hover:bg-gray-50'
                  }`}
                >
                  <input
                    type="checkbox"
                    checked={formData.serviceCategories.includes(cat.value)}
                    onChange={() => toggleCategory(cat.value)}
                    className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                  />
                  <span className="ml-2 text-sm text-gray-700">{cat.label}</span>
                </label>
              ))}
            </div>
          </div>

          <div>
            <label
              htmlFor="serviceDescription"
              className="block text-sm font-medium text-gray-700 mb-1"
            >
              Service Description
            </label>
            <textarea
              id="serviceDescription"
              rows={4}
              value={formData.serviceDescription || ''}
              onChange={(e) => updateField('serviceDescription', e.target.value)}
              placeholder="Describe your services in more detail..."
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div>
            <label
              htmlFor="coverageRadiusKm"
              className="block text-sm font-medium text-gray-700 mb-1"
            >
              Coverage Radius (km)
            </label>
            <input
              id="coverageRadiusKm"
              type="number"
              min="1"
              value={formData.coverageRadiusKm || ''}
              onChange={(e) =>
                updateField('coverageRadiusKm', e.target.value ? Number(e.target.value) : undefined)
              }
              placeholder="e.g., 50"
              className="w-32 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div className="flex items-center gap-4">
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={formData.emergencyAvailable || false}
                onChange={(e) => updateField('emergencyAvailable', e.target.checked)}
                className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
              />
              <span className="text-sm text-gray-700">Available for 24/7 Emergency Calls</span>
            </label>
          </div>
        </div>
      )}

      {/* Step 3: Pricing */}
      {currentStep === 3 && (
        <div className="space-y-6">
          <h2 className="text-xl font-semibold text-gray-900">Pricing Structure</h2>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-3">Pricing Type</label>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
              {[
                { value: 'hourly', label: 'Hourly Rate' },
                { value: 'project', label: 'Per Project' },
                { value: 'fixed', label: 'Fixed Price' },
                { value: 'quote_required', label: 'Quote Required' },
              ].map((opt) => (
                <label
                  key={opt.value}
                  className={`flex items-center justify-center p-3 border rounded-lg cursor-pointer transition-colors ${
                    formData.pricingType === opt.value
                      ? 'border-blue-500 bg-blue-50'
                      : 'border-gray-300 hover:bg-gray-50'
                  }`}
                >
                  <input
                    type="radio"
                    name="pricingType"
                    value={opt.value}
                    checked={formData.pricingType === opt.value}
                    onChange={(e) =>
                      updateField(
                        'pricingType',
                        e.target.value as ProviderProfileFormData['pricingType']
                      )
                    }
                    className="sr-only"
                  />
                  <span className="text-sm text-gray-700">{opt.label}</span>
                </label>
              ))}
            </div>
          </div>

          {formData.pricingType === 'hourly' && (
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
              <div>
                <label
                  htmlFor="hourlyRateMin"
                  className="block text-sm font-medium text-gray-700 mb-1"
                >
                  Minimum Hourly Rate
                </label>
                <div className="relative">
                  <input
                    id="hourlyRateMin"
                    type="number"
                    min="0"
                    step="0.01"
                    value={formData.hourlyRateMin || ''}
                    onChange={(e) =>
                      updateField(
                        'hourlyRateMin',
                        e.target.value ? Number(e.target.value) : undefined
                      )
                    }
                    className="w-full pl-8 pr-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
                  />
                  <span className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500">
                    {formData.currency === 'EUR' ? 'EUR' : formData.currency || 'EUR'}
                  </span>
                </div>
              </div>

              <div>
                <label
                  htmlFor="hourlyRateMax"
                  className="block text-sm font-medium text-gray-700 mb-1"
                >
                  Maximum Hourly Rate
                </label>
                <div className="relative">
                  <input
                    id="hourlyRateMax"
                    type="number"
                    min="0"
                    step="0.01"
                    value={formData.hourlyRateMax || ''}
                    onChange={(e) =>
                      updateField(
                        'hourlyRateMax',
                        e.target.value ? Number(e.target.value) : undefined
                      )
                    }
                    className={`w-full pl-8 pr-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 ${
                      errors.hourlyRateMax ? 'border-red-500' : 'border-gray-300'
                    }`}
                  />
                  <span className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500">
                    {formData.currency === 'EUR' ? 'EUR' : formData.currency || 'EUR'}
                  </span>
                </div>
                {errors.hourlyRateMax && (
                  <p className="mt-1 text-sm text-red-600">{errors.hourlyRateMax}</p>
                )}
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
          )}

          <div>
            <label
              htmlFor="responseTimeHours"
              className="block text-sm font-medium text-gray-700 mb-1"
            >
              Typical Response Time (hours)
            </label>
            <input
              id="responseTimeHours"
              type="number"
              min="1"
              value={formData.responseTimeHours || ''}
              onChange={(e) =>
                updateField(
                  'responseTimeHours',
                  e.target.value ? Number(e.target.value) : undefined
                )
              }
              placeholder="e.g., 24"
              className="w-32 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
            />
            <p className="mt-1 text-sm text-gray-500">
              How quickly do you typically respond to inquiries?
            </p>
          </div>
        </div>
      )}

      {/* Step 4: Portfolio */}
      {currentStep === 4 && (
        <div className="space-y-6">
          <h2 className="text-xl font-semibold text-gray-900">Portfolio</h2>

          <div>
            <label
              htmlFor="portfolioDescription"
              className="block text-sm font-medium text-gray-700 mb-1"
            >
              Portfolio Description
            </label>
            <textarea
              id="portfolioDescription"
              rows={4}
              value={formData.portfolioDescription || ''}
              onChange={(e) => updateField('portfolioDescription', e.target.value)}
              placeholder="Describe your past work, notable projects, and achievements..."
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-3">Portfolio Images</label>
            <p className="text-sm text-gray-500 mb-4">
              Add URLs to images showcasing your work. You can add more images after creating your
              profile.
            </p>
            <div className="border-2 border-dashed border-gray-300 rounded-lg p-8 text-center">
              <svg className="mx-auto w-12 h-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <title>Upload images</title>
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
              <p className="mt-2 text-sm text-gray-600">Image upload will be available after profile creation</p>
            </div>
          </div>

          <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
            <h3 className="text-sm font-medium text-blue-800">Ready to Submit?</h3>
            <p className="mt-1 text-sm text-blue-700">
              Your profile will be reviewed by our team. Once approved, it will be visible in the
              marketplace.
            </p>
          </div>
        </div>
      )}

      {/* Navigation Buttons */}
      <div className="mt-8 flex items-center justify-between pt-6 border-t">
        <button
          type="button"
          onClick={currentStep === 1 ? onCancel : handlePrev}
          className="px-4 py-2 text-gray-700 font-medium hover:text-gray-900"
        >
          {currentStep === 1 ? 'Cancel' : 'Previous'}
        </button>

        <div className="flex items-center gap-3">
          {currentStep < totalSteps ? (
            <button
              type="button"
              onClick={handleNext}
              className="px-6 py-2 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
            >
              Next
            </button>
          ) : (
            <button
              type="submit"
              disabled={isLoading}
              className="px-6 py-2 bg-green-600 text-white font-medium rounded-lg hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 disabled:opacity-50"
            >
              {isLoading ? 'Saving...' : isEdit ? 'Update Profile' : 'Create Profile'}
            </button>
          )}
        </div>
      </div>
    </form>
  );
}
