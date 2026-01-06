/**
 * FileClaimPage - Page for filing a new insurance claim.
 * Insurance Management Feature
 */

import { useTranslation } from 'react-i18next';
import { ClaimForm } from '../components/ClaimForm';
import type { ClaimFormData, InsurancePolicy } from '../types';

interface FileClaimPageProps {
  policies: InsurancePolicy[];
  buildings: Array<{ id: string; name: string }>;
  units: Array<{ id: string; designation: string }>;
  preselectedPolicyId?: string;
  isSubmitting?: boolean;
  onSubmit: (data: ClaimFormData) => void;
  onCancel: () => void;
}

export function FileClaimPage({
  policies,
  buildings,
  units,
  preselectedPolicyId,
  isSubmitting,
  onSubmit,
  onCancel,
}: FileClaimPageProps) {
  const { t } = useTranslation();

  return (
    <div className="max-w-2xl mx-auto px-4 py-8">
      <div className="mb-6">
        <button
          type="button"
          onClick={onCancel}
          className="text-sm text-blue-600 hover:text-blue-800 flex items-center gap-1 mb-4"
        >
          {t('insurance.backToClaims')}
        </button>
        <h1 className="text-2xl font-bold text-gray-900">{t('insurance.fileClaimPage.title')}</h1>
        <p className="text-gray-600 mt-2">{t('insurance.fileClaimPage.description')}</p>
      </div>

      <div className="bg-white rounded-lg shadow p-6">
        <ClaimForm
          initialData={preselectedPolicyId ? { policyId: preselectedPolicyId } : undefined}
          policies={policies}
          buildings={buildings}
          units={units}
          isSubmitting={isSubmitting}
          onSubmit={onSubmit}
          onCancel={onCancel}
        />
      </div>
    </div>
  );
}
