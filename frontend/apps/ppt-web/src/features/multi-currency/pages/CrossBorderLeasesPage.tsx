/**
 * CrossBorderLeasesPage - Story 145.4
 *
 * Page for managing cross-border leases with currency and compliance handling.
 */

import { useState } from 'react';
import { CrossBorderLeaseCard } from '../components/CrossBorderLeaseCard';

interface CrossBorderLease {
  id: string;
  leaseId: string;
  propertyCountry: string;
  propertyCurrency: string;
  tenantCountry?: string;
  leaseCurrency: string;
  paymentCurrency: string;
  convertAtInvoiceDate: boolean;
  convertAtPaymentDate: boolean;
  fixedExchangeRate?: number;
  localVatApplicable: boolean;
  vatRate?: number;
  reverseChargeVat: boolean;
  withholdingTaxRate?: number;
  complianceStatus: string;
  complianceNotes?: string;
  governingLaw?: string;
  jurisdiction?: string;
}

interface ComplianceRequirement {
  id: string;
  country: string;
  requirementType: string;
  requirementName: string;
  description?: string;
  thresholdAmount?: number;
  thresholdCurrency?: string;
  reportingFrequency?: string;
  reportingDeadlineDays?: number;
}

interface CrossBorderLeasesPageProps {
  leases: CrossBorderLease[];
  complianceRequirements?: ComplianceRequirement[];
  isLoading?: boolean;
  onCreateLease?: () => void;
  onEditLease?: (lease: CrossBorderLease) => void;
  onViewCompliance?: (country: string) => void;
  onFilterByCountry?: (country?: string) => void;
  onFilterByStatus?: (status?: string) => void;
}

const COUNTRY_OPTIONS = [
  { code: 'SK', name: 'Slovakia' },
  { code: 'CZ', name: 'Czech Republic' },
  { code: 'AT', name: 'Austria' },
  { code: 'DE', name: 'Germany' },
  { code: 'PL', name: 'Poland' },
  { code: 'HU', name: 'Hungary' },
  { code: 'CH', name: 'Switzerland' },
  { code: 'GB', name: 'United Kingdom' },
];

const STATUS_OPTIONS = [
  { value: 'compliant', label: 'Compliant' },
  { value: 'pending_review', label: 'Pending Review' },
  { value: 'non_compliant', label: 'Non-Compliant' },
  { value: 'exempt', label: 'Exempt' },
];

export function CrossBorderLeasesPage({
  leases,
  complianceRequirements,
  isLoading,
  onCreateLease,
  onEditLease,
  onViewCompliance,
  onFilterByCountry,
  onFilterByStatus,
}: CrossBorderLeasesPageProps) {
  const [selectedCountry, setSelectedCountry] = useState<string>('');
  const [selectedStatus, setSelectedStatus] = useState<string>('');
  const [showRequirements, setShowRequirements] = useState(false);
  const [requirementsCountry, setRequirementsCountry] = useState<string>('');

  const handleCountryFilter = (country: string) => {
    setSelectedCountry(country);
    onFilterByCountry?.(country || undefined);
  };

  const handleStatusFilter = (status: string) => {
    setSelectedStatus(status);
    onFilterByStatus?.(status || undefined);
  };

  const handleViewRequirements = (country: string) => {
    setRequirementsCountry(country);
    setShowRequirements(true);
    onViewCompliance?.(country);
  };

  const filteredRequirements = complianceRequirements?.filter(
    (req) => req.country === requirementsCountry
  );

  const getStatusCounts = () => {
    const counts: Record<string, number> = {};
    for (const lease of leases) {
      counts[lease.complianceStatus] = (counts[lease.complianceStatus] || 0) + 1;
    }
    return counts;
  };

  const statusCounts = getStatusCounts();

  return (
    <div className="min-h-screen bg-gray-100">
      {/* Header */}
      <div className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-2xl font-bold text-gray-900">Cross-Border Leases</h1>
              <p className="text-sm text-gray-500">
                Manage leases across countries with currency and compliance handling
              </p>
            </div>
            {onCreateLease && (
              <button
                type="button"
                onClick={onCreateLease}
                className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700"
              >
                Add Cross-Border Lease
              </button>
            )}
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6 space-y-6">
        {/* Status Summary */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          {STATUS_OPTIONS.map((status) => (
            <button
              type="button"
              key={status.value}
              onClick={() =>
                handleStatusFilter(status.value === selectedStatus ? '' : status.value)
              }
              className={`p-4 rounded-lg border-2 transition-all ${
                selectedStatus === status.value
                  ? 'border-blue-500 bg-blue-50'
                  : 'border-transparent bg-white shadow hover:border-gray-300'
              }`}
            >
              <div className="text-2xl font-bold text-gray-900">
                {statusCounts[status.value] || 0}
              </div>
              <div className="text-sm text-gray-500">{status.label}</div>
            </button>
          ))}
        </div>

        {/* Filters */}
        <div className="bg-white rounded-lg shadow p-4">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Property Country
              </label>
              <select
                value={selectedCountry}
                onChange={(e) => handleCountryFilter(e.target.value)}
                className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
              >
                <option value="">All Countries</option>
                {COUNTRY_OPTIONS.map((country) => (
                  <option key={country.code} value={country.code}>
                    {country.name}
                  </option>
                ))}
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Compliance Status
              </label>
              <select
                value={selectedStatus}
                onChange={(e) => handleStatusFilter(e.target.value)}
                className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
              >
                <option value="">All Statuses</option>
                {STATUS_OPTIONS.map((status) => (
                  <option key={status.value} value={status.value}>
                    {status.label}
                  </option>
                ))}
              </select>
            </div>
            <div className="flex items-end">
              <button
                type="button"
                onClick={() => {
                  setSelectedCountry('');
                  setSelectedStatus('');
                  onFilterByCountry?.();
                  onFilterByStatus?.();
                }}
                className="w-full px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200"
              >
                Clear Filters
              </button>
            </div>
          </div>
        </div>

        {/* Leases List */}
        {isLoading ? (
          <div className="flex justify-center items-center py-8">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" />
          </div>
        ) : leases.length === 0 ? (
          <div className="bg-white rounded-lg shadow p-8 text-center">
            <p className="text-gray-500">No cross-border leases found.</p>
          </div>
        ) : (
          <div className="space-y-4">
            {leases.map((lease) => (
              <CrossBorderLeaseCard
                key={lease.id}
                lease={lease}
                onEdit={onEditLease ? () => onEditLease(lease) : undefined}
                onViewCompliance={() => handleViewRequirements(lease.propertyCountry)}
              />
            ))}
          </div>
        )}
      </div>

      {/* Compliance Requirements Modal */}
      {showRequirements && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg shadow-xl p-6 w-full max-w-2xl max-h-[80vh] overflow-auto">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-lg font-medium text-gray-900">
                Compliance Requirements - {requirementsCountry}
              </h3>
              <button
                type="button"
                onClick={() => setShowRequirements(false)}
                className="text-gray-400 hover:text-gray-500"
              >
                <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M6 18L18 6M6 6l12 12"
                  />
                </svg>
              </button>
            </div>

            {filteredRequirements && filteredRequirements.length > 0 ? (
              <div className="space-y-4">
                {filteredRequirements.map((req) => (
                  <div key={req.id} className="border rounded-lg p-4">
                    <div className="flex items-start justify-between">
                      <div>
                        <span className="px-2 py-1 text-xs font-medium bg-gray-100 text-gray-800 rounded">
                          {req.requirementType}
                        </span>
                        <h4 className="mt-2 font-medium text-gray-900">{req.requirementName}</h4>
                        {req.description && (
                          <p className="mt-1 text-sm text-gray-500">{req.description}</p>
                        )}
                      </div>
                    </div>
                    <div className="mt-3 grid grid-cols-2 gap-2 text-sm">
                      {req.thresholdAmount && (
                        <div>
                          <span className="text-gray-500">Threshold:</span>{' '}
                          {req.thresholdAmount.toLocaleString()} {req.thresholdCurrency}
                        </div>
                      )}
                      {req.reportingFrequency && (
                        <div>
                          <span className="text-gray-500">Frequency:</span> {req.reportingFrequency}
                        </div>
                      )}
                      {req.reportingDeadlineDays && (
                        <div>
                          <span className="text-gray-500">Deadline:</span>{' '}
                          {req.reportingDeadlineDays} days
                        </div>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <p className="text-gray-500 text-center py-4">
                No compliance requirements found for this country.
              </p>
            )}

            <div className="mt-6 flex justify-end">
              <button
                type="button"
                onClick={() => setShowRequirements(false)}
                className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200"
              >
                Close
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
