/**
 * Data Residency Configuration Card (Epic 146, Story 146.1).
 *
 * Displays current data residency configuration and compliance status.
 */

import type React from 'react';

export interface DataRegionInfo {
  region: string;
  display_name: string;
  location_code: string;
  compliance_frameworks: string[];
}

export interface ComplianceImplication {
  level: 'info' | 'warning' | 'requirement';
  title: string;
  description: string;
  regulation?: string;
}

export interface DataResidencyConfig {
  id: string;
  organization_id: string;
  primary_region: string;
  primary_region_display: string;
  backup_region?: string;
  backup_region_display?: string;
  status: 'active' | 'migrating' | 'pending' | 'suspended';
  allow_cross_region_access: boolean;
  compliance_frameworks: string[];
  compliance_implications: ComplianceImplication[];
  last_verified_at?: string;
  created_at: string;
  updated_at: string;
}

interface DataResidencyConfigCardProps {
  config: DataResidencyConfig;
  onEdit?: () => void;
  onVerify?: () => void;
}

const getStatusBadgeStyles = (status: DataResidencyConfig['status']): string => {
  const baseStyles = 'px-2 py-1 text-xs font-medium rounded-full';
  switch (status) {
    case 'active':
      return `${baseStyles} bg-green-100 text-green-800`;
    case 'migrating':
      return `${baseStyles} bg-yellow-100 text-yellow-800`;
    case 'pending':
      return `${baseStyles} bg-blue-100 text-blue-800`;
    case 'suspended':
      return `${baseStyles} bg-red-100 text-red-800`;
    default:
      return `${baseStyles} bg-gray-100 text-gray-800`;
  }
};

const getImplicationStyles = (level: ComplianceImplication['level']): string => {
  switch (level) {
    case 'info':
      return 'bg-blue-50 border-blue-200 text-blue-800';
    case 'warning':
      return 'bg-yellow-50 border-yellow-200 text-yellow-800';
    case 'requirement':
      return 'bg-red-50 border-red-200 text-red-800';
    default:
      return 'bg-gray-50 border-gray-200 text-gray-800';
  }
};

const getImplicationIcon = (level: ComplianceImplication['level']): string => {
  switch (level) {
    case 'info':
      return 'i';
    case 'warning':
      return '!';
    case 'requirement':
      return '*';
    default:
      return '-';
  }
};

export const DataResidencyConfigCard: React.FC<DataResidencyConfigCardProps> = ({
  config,
  onEdit,
  onVerify,
}) => {
  return (
    <div className="bg-white rounded-lg shadow-md border border-gray-200 overflow-hidden">
      {/* Header */}
      <div className="px-6 py-4 border-b border-gray-200 flex items-center justify-between">
        <div>
          <h3 className="text-lg font-semibold text-gray-900">Data Residency Configuration</h3>
          <p className="text-sm text-gray-500 mt-1">
            Configure where your organization's data is stored
          </p>
        </div>
        <span className={getStatusBadgeStyles(config.status)}>
          {config.status.charAt(0).toUpperCase() + config.status.slice(1)}
        </span>
      </div>

      {/* Region Configuration */}
      <div className="px-6 py-4">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {/* Primary Region */}
          <div>
            <label className="block text-sm font-medium text-gray-500 mb-2">Primary Region</label>
            <div className="flex items-center space-x-3">
              <div className="w-10 h-10 bg-indigo-100 rounded-lg flex items-center justify-center">
                <svg
                  className="w-6 h-6 text-indigo-600"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                  />
                </svg>
              </div>
              <div>
                <p className="text-sm font-medium text-gray-900">{config.primary_region_display}</p>
                <p className="text-xs text-gray-500">All data written to this region</p>
              </div>
            </div>
          </div>

          {/* Backup Region */}
          <div>
            <label className="block text-sm font-medium text-gray-500 mb-2">Backup Region</label>
            {config.backup_region_display ? (
              <div className="flex items-center space-x-3">
                <div className="w-10 h-10 bg-gray-100 rounded-lg flex items-center justify-center">
                  <svg
                    className="w-6 h-6 text-gray-600"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4"
                    />
                  </svg>
                </div>
                <div>
                  <p className="text-sm font-medium text-gray-900">
                    {config.backup_region_display}
                  </p>
                  <p className="text-xs text-gray-500">Disaster recovery failover</p>
                </div>
              </div>
            ) : (
              <p className="text-sm text-gray-500 italic">Not configured</p>
            )}
          </div>
        </div>

        {/* Cross-Region Access */}
        <div className="mt-6 p-4 bg-gray-50 rounded-lg">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-900">Cross-Region Access</p>
              <p className="text-xs text-gray-500 mt-1">
                Allow data access from regions other than primary
              </p>
            </div>
            <span
              className={`px-2 py-1 text-xs font-medium rounded ${
                config.allow_cross_region_access
                  ? 'bg-yellow-100 text-yellow-800'
                  : 'bg-green-100 text-green-800'
              }`}
            >
              {config.allow_cross_region_access ? 'Enabled' : 'Disabled'}
            </span>
          </div>
        </div>
      </div>

      {/* Compliance Frameworks */}
      <div className="px-6 py-4 border-t border-gray-200">
        <label className="block text-sm font-medium text-gray-500 mb-3">
          Compliance Frameworks
        </label>
        <div className="flex flex-wrap gap-2">
          {config.compliance_frameworks.map((framework, idx) => (
            <span key={idx} className="px-3 py-1 bg-indigo-50 text-indigo-700 text-sm rounded-full">
              {framework}
            </span>
          ))}
        </div>
      </div>

      {/* Compliance Implications */}
      {config.compliance_implications.length > 0 && (
        <div className="px-6 py-4 border-t border-gray-200">
          <label className="block text-sm font-medium text-gray-500 mb-3">
            Compliance Implications
          </label>
          <div className="space-y-2">
            {config.compliance_implications.map((implication, idx) => (
              <div
                key={idx}
                className={`p-3 rounded-lg border ${getImplicationStyles(implication.level)}`}
              >
                <div className="flex items-start space-x-2">
                  <span className="font-bold text-lg">{getImplicationIcon(implication.level)}</span>
                  <div>
                    <p className="font-medium">{implication.title}</p>
                    <p className="text-sm mt-1">{implication.description}</p>
                    {implication.regulation && (
                      <p className="text-xs mt-2 opacity-75">Reference: {implication.regulation}</p>
                    )}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Last Verified */}
      {config.last_verified_at && (
        <div className="px-6 py-3 bg-gray-50 border-t border-gray-200">
          <p className="text-xs text-gray-500">
            Last verified: {new Date(config.last_verified_at).toLocaleString()}
          </p>
        </div>
      )}

      {/* Actions */}
      <div className="px-6 py-4 border-t border-gray-200 flex justify-end space-x-3">
        {onVerify && (
          <button
            type="button"
            onClick={onVerify}
            className="px-4 py-2 text-sm font-medium text-indigo-600 bg-indigo-50 rounded-lg hover:bg-indigo-100 transition-colors"
          >
            Verify Compliance
          </button>
        )}
        {onEdit && (
          <button
            type="button"
            onClick={onEdit}
            className="px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 transition-colors"
          >
            Edit Configuration
          </button>
        )}
      </div>
    </div>
  );
};
