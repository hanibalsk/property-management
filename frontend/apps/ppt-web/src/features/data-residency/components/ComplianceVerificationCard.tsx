/**
 * Compliance Verification Card (Epic 146, Story 146.3).
 *
 * Displays compliance verification results with data location breakdown.
 */

import type React from 'react';

export interface DataLocationSummary {
  data_type: string;
  region: string;
  configured_region: string;
  is_correct_location: boolean;
  record_count: number;
  last_updated?: string;
}

export interface ComplianceIssue {
  severity: 'low' | 'medium' | 'high' | 'critical';
  title: string;
  description: string;
  data_type?: string;
  region?: string;
  resolution: string;
}

export interface RegionAccessSummary {
  region: string;
  read_count: number;
  write_count: number;
  cross_region_count: number;
  period: string;
}

export interface ComplianceVerificationResult {
  id: string;
  organization_id: string;
  compliance_status: 'compliant' | 'partially_compliant' | 'non_compliant' | 'verifying';
  is_compliant: boolean;
  verified_at: string;
  data_locations: DataLocationSummary[];
  access_by_region: RegionAccessSummary[];
  issues: ComplianceIssue[];
  report_available: boolean;
}

interface ComplianceVerificationCardProps {
  result: ComplianceVerificationResult;
  onExportReport?: () => void;
  onRunVerification?: () => void;
}

const getStatusStyles = (status: ComplianceVerificationResult['compliance_status']) => {
  switch (status) {
    case 'compliant':
      return {
        bg: 'bg-green-100',
        text: 'text-green-800',
        icon: 'M5 13l4 4L19 7',
        label: 'Compliant',
      };
    case 'partially_compliant':
      return {
        bg: 'bg-yellow-100',
        text: 'text-yellow-800',
        icon: 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z',
        label: 'Partially Compliant',
      };
    case 'non_compliant':
      return {
        bg: 'bg-red-100',
        text: 'text-red-800',
        icon: 'M6 18L18 6M6 6l12 12',
        label: 'Non-Compliant',
      };
    case 'verifying':
      return {
        bg: 'bg-blue-100',
        text: 'text-blue-800',
        icon: 'M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15',
        label: 'Verifying',
      };
    default:
      return {
        bg: 'bg-gray-100',
        text: 'text-gray-800',
        icon: '',
        label: 'Unknown',
      };
  }
};

const getSeverityStyles = (severity: ComplianceIssue['severity']) => {
  switch (severity) {
    case 'low':
      return 'bg-blue-50 border-blue-200 text-blue-800';
    case 'medium':
      return 'bg-yellow-50 border-yellow-200 text-yellow-800';
    case 'high':
      return 'bg-orange-50 border-orange-200 text-orange-800';
    case 'critical':
      return 'bg-red-50 border-red-200 text-red-800';
    default:
      return 'bg-gray-50 border-gray-200 text-gray-800';
  }
};

const formatDataType = (dataType: string): string => {
  return dataType
    .split('_')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
};

const formatNumber = (num: number): string => {
  return num.toLocaleString();
};

export const ComplianceVerificationCard: React.FC<ComplianceVerificationCardProps> = ({
  result,
  onExportReport,
  onRunVerification,
}) => {
  const statusStyles = getStatusStyles(result.compliance_status);

  return (
    <div className="bg-white rounded-lg shadow-md border border-gray-200 overflow-hidden">
      {/* Header with Status */}
      <div className="px-6 py-4 border-b border-gray-200">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <div
              className={`w-12 h-12 rounded-full ${statusStyles.bg} flex items-center justify-center`}
            >
              <svg
                className={`w-6 h-6 ${statusStyles.text}`}
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d={statusStyles.icon}
                />
              </svg>
            </div>
            <div>
              <h3 className="text-lg font-semibold text-gray-900">Compliance Verification</h3>
              <p className={`text-sm font-medium ${statusStyles.text}`}>{statusStyles.label}</p>
            </div>
          </div>
          <p className="text-sm text-gray-500">
            Verified: {new Date(result.verified_at).toLocaleString()}
          </p>
        </div>
      </div>

      {/* Data Locations */}
      <div className="px-6 py-4">
        <h4 className="text-sm font-medium text-gray-700 mb-3">Data Locations</h4>
        <div className="overflow-x-auto">
          <table className="min-w-full">
            <thead>
              <tr className="text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                <th className="pb-2">Data Type</th>
                <th className="pb-2">Current Region</th>
                <th className="pb-2">Expected</th>
                <th className="pb-2">Records</th>
                <th className="pb-2">Status</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-100">
              {result.data_locations.map((location, idx) => (
                <tr key={idx} className="text-sm">
                  <td className="py-2 font-medium text-gray-900">
                    {formatDataType(location.data_type)}
                  </td>
                  <td className="py-2 text-gray-600">{location.region}</td>
                  <td className="py-2 text-gray-600">{location.configured_region}</td>
                  <td className="py-2 text-gray-600">{formatNumber(location.record_count)}</td>
                  <td className="py-2">
                    {location.is_correct_location ? (
                      <span className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-green-100 text-green-800">
                        <svg
                          className="w-3 h-3 mr-1"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={2}
                            d="M5 13l4 4L19 7"
                          />
                        </svg>
                        Correct
                      </span>
                    ) : (
                      <span className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-red-100 text-red-800">
                        <svg
                          className="w-3 h-3 mr-1"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={2}
                            d="M12 9v2m0 4h.01"
                          />
                        </svg>
                        Misplaced
                      </span>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Access by Region */}
      {result.access_by_region.length > 0 && (
        <div className="px-6 py-4 border-t border-gray-200">
          <h4 className="text-sm font-medium text-gray-700 mb-3">Recent Access by Region</h4>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {result.access_by_region.map((access, idx) => (
              <div key={idx} className="p-3 bg-gray-50 rounded-lg">
                <p className="text-sm font-medium text-gray-900">{access.region}</p>
                <div className="mt-2 grid grid-cols-3 gap-2 text-xs">
                  <div>
                    <p className="text-gray-500">Reads</p>
                    <p className="font-medium text-gray-900">{formatNumber(access.read_count)}</p>
                  </div>
                  <div>
                    <p className="text-gray-500">Writes</p>
                    <p className="font-medium text-gray-900">{formatNumber(access.write_count)}</p>
                  </div>
                  <div>
                    <p className="text-gray-500">Cross-Region</p>
                    <p
                      className={`font-medium ${access.cross_region_count > 0 ? 'text-yellow-600' : 'text-gray-900'}`}
                    >
                      {formatNumber(access.cross_region_count)}
                    </p>
                  </div>
                </div>
                <p className="text-xs text-gray-400 mt-2">Period: {access.period}</p>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Issues */}
      {result.issues.length > 0 && (
        <div className="px-6 py-4 border-t border-gray-200">
          <h4 className="text-sm font-medium text-gray-700 mb-3">
            Issues Found ({result.issues.length})
          </h4>
          <div className="space-y-3">
            {result.issues.map((issue, idx) => (
              <div
                key={idx}
                className={`p-4 rounded-lg border ${getSeverityStyles(issue.severity)}`}
              >
                <div className="flex items-start justify-between">
                  <div>
                    <span className="inline-block px-2 py-0.5 text-xs font-medium rounded uppercase mb-2">
                      {issue.severity}
                    </span>
                    <h5 className="font-medium">{issue.title}</h5>
                    <p className="text-sm mt-1">{issue.description}</p>
                    {(issue.data_type || issue.region) && (
                      <p className="text-xs mt-2 opacity-75">
                        {issue.data_type && `Data Type: ${formatDataType(issue.data_type)}`}
                        {issue.data_type && issue.region && ' | '}
                        {issue.region && `Region: ${issue.region}`}
                      </p>
                    )}
                  </div>
                </div>
                <div className="mt-3 p-2 bg-white bg-opacity-50 rounded">
                  <p className="text-xs font-medium">Resolution:</p>
                  <p className="text-sm">{issue.resolution}</p>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Actions */}
      <div className="px-6 py-4 border-t border-gray-200 flex justify-between items-center">
        <div>
          {result.report_available && (
            <span className="text-xs text-gray-500">Detailed report available for download</span>
          )}
        </div>
        <div className="flex space-x-3">
          {result.report_available && onExportReport && (
            <button
              type="button"
              onClick={onExportReport}
              className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
            >
              Export Report
            </button>
          )}
          {onRunVerification && (
            <button
              type="button"
              onClick={onRunVerification}
              className="px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 transition-colors"
            >
              Run Verification
            </button>
          )}
        </div>
      </div>
    </div>
  );
};
