/**
 * CrossBorderLeaseCard - Story 145.4
 *
 * Card component displaying cross-border lease information.
 */

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

interface CrossBorderLeaseCardProps {
  lease: CrossBorderLease;
  onEdit?: () => void;
  onViewCompliance?: () => void;
}

const COUNTRY_NAMES: Record<string, string> = {
  SK: 'Slovakia',
  CZ: 'Czech Republic',
  AT: 'Austria',
  DE: 'Germany',
  PL: 'Poland',
  HU: 'Hungary',
  CH: 'Switzerland',
  GB: 'United Kingdom',
  FR: 'France',
  IT: 'Italy',
  ES: 'Spain',
  NL: 'Netherlands',
};

export function CrossBorderLeaseCard({
  lease,
  onEdit,
  onViewCompliance,
}: CrossBorderLeaseCardProps) {
  const getComplianceStatusBadge = (status: string) => {
    switch (status) {
      case 'compliant':
        return (
          <span className="px-2 py-1 text-xs font-medium bg-green-100 text-green-800 rounded">
            Compliant
          </span>
        );
      case 'pending_review':
        return (
          <span className="px-2 py-1 text-xs font-medium bg-yellow-100 text-yellow-800 rounded">
            Pending Review
          </span>
        );
      case 'non_compliant':
        return (
          <span className="px-2 py-1 text-xs font-medium bg-red-100 text-red-800 rounded">
            Non-Compliant
          </span>
        );
      case 'exempt':
        return (
          <span className="px-2 py-1 text-xs font-medium bg-gray-100 text-gray-800 rounded">
            Exempt
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

  const getCountryName = (code: string) => COUNTRY_NAMES[code] || code;

  return (
    <div className="bg-white rounded-lg shadow border border-gray-200 p-6">
      {/* Header */}
      <div className="flex items-start justify-between mb-4">
        <div>
          <h3 className="text-lg font-semibold text-gray-900">Cross-Border Lease</h3>
          <p className="text-sm text-gray-500">
            Property in {getCountryName(lease.propertyCountry)}
            {lease.tenantCountry && ` | Tenant from ${getCountryName(lease.tenantCountry)}`}
          </p>
        </div>
        <div className="flex items-center space-x-2">
          {getComplianceStatusBadge(lease.complianceStatus)}
        </div>
      </div>

      {/* Currency Information */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-4">
        <div>
          <div className="text-xs text-gray-500 uppercase">Property Currency</div>
          <div className="font-medium text-gray-900">{lease.propertyCurrency}</div>
        </div>
        <div>
          <div className="text-xs text-gray-500 uppercase">Lease Currency</div>
          <div className="font-medium text-gray-900">{lease.leaseCurrency}</div>
        </div>
        <div>
          <div className="text-xs text-gray-500 uppercase">Payment Currency</div>
          <div className="font-medium text-gray-900">{lease.paymentCurrency}</div>
        </div>
        {lease.fixedExchangeRate && (
          <div>
            <div className="text-xs text-gray-500 uppercase">Fixed Rate</div>
            <div className="font-medium text-gray-900">{lease.fixedExchangeRate.toFixed(4)}</div>
          </div>
        )}
      </div>

      {/* Conversion Settings */}
      <div className="bg-gray-50 rounded p-3 mb-4">
        <div className="text-xs text-gray-500 uppercase mb-2">Conversion Settings</div>
        <div className="flex flex-wrap gap-2">
          {lease.convertAtInvoiceDate && (
            <span className="px-2 py-1 text-xs bg-blue-100 text-blue-800 rounded">
              Convert at Invoice Date
            </span>
          )}
          {lease.convertAtPaymentDate && (
            <span className="px-2 py-1 text-xs bg-blue-100 text-blue-800 rounded">
              Convert at Payment Date
            </span>
          )}
          {lease.fixedExchangeRate && (
            <span className="px-2 py-1 text-xs bg-purple-100 text-purple-800 rounded">
              Fixed Rate Locked
            </span>
          )}
        </div>
      </div>

      {/* Tax Information */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-4">
        <div>
          <div className="text-xs text-gray-500 uppercase">VAT Applicable</div>
          <div className="font-medium text-gray-900">{lease.localVatApplicable ? 'Yes' : 'No'}</div>
        </div>
        {lease.vatRate !== undefined && (
          <div>
            <div className="text-xs text-gray-500 uppercase">VAT Rate</div>
            <div className="font-medium text-gray-900">{lease.vatRate}%</div>
          </div>
        )}
        <div>
          <div className="text-xs text-gray-500 uppercase">Reverse Charge</div>
          <div className="font-medium text-gray-900">{lease.reverseChargeVat ? 'Yes' : 'No'}</div>
        </div>
        {lease.withholdingTaxRate !== undefined && (
          <div>
            <div className="text-xs text-gray-500 uppercase">Withholding Tax</div>
            <div className="font-medium text-gray-900">{lease.withholdingTaxRate}%</div>
          </div>
        )}
      </div>

      {/* Legal Information */}
      {(lease.governingLaw || lease.jurisdiction) && (
        <div className="border-t pt-4 mt-4">
          <div className="grid grid-cols-2 gap-4">
            {lease.governingLaw && (
              <div>
                <div className="text-xs text-gray-500 uppercase">Governing Law</div>
                <div className="font-medium text-gray-900">
                  {getCountryName(lease.governingLaw)}
                </div>
              </div>
            )}
            {lease.jurisdiction && (
              <div>
                <div className="text-xs text-gray-500 uppercase">Jurisdiction</div>
                <div className="font-medium text-gray-900">
                  {getCountryName(lease.jurisdiction)}
                </div>
              </div>
            )}
          </div>
        </div>
      )}

      {/* Compliance Notes */}
      {lease.complianceNotes && (
        <div className="border-t pt-4 mt-4">
          <div className="text-xs text-gray-500 uppercase mb-1">Compliance Notes</div>
          <p className="text-sm text-gray-600">{lease.complianceNotes}</p>
        </div>
      )}

      {/* Actions */}
      <div className="border-t pt-4 mt-4 flex justify-end space-x-3">
        {onViewCompliance && (
          <button
            type="button"
            onClick={onViewCompliance}
            className="px-3 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded hover:bg-gray-200"
          >
            View Requirements
          </button>
        )}
        {onEdit && (
          <button
            type="button"
            onClick={onEdit}
            className="px-3 py-2 text-sm font-medium text-white bg-blue-600 rounded hover:bg-blue-700"
          >
            Edit
          </button>
        )}
      </div>
    </div>
  );
}
