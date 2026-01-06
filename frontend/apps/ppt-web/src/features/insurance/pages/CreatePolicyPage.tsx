/**
 * CreatePolicyPage - Page for adding a new insurance policy.
 * Insurance Management Feature
 */

import { useTranslation } from 'react-i18next';
import { PolicyForm } from '../components/PolicyForm';
import type { PolicyFormData } from '../types';

interface CreatePolicyPageProps {
  buildings: Array<{ id: string; name: string }>;
  isSubmitting?: boolean;
  onSubmit: (data: PolicyFormData) => void;
  onCancel: () => void;
}

export function CreatePolicyPage({
  buildings,
  isSubmitting,
  onSubmit,
  onCancel,
}: CreatePolicyPageProps) {
  const { t } = useTranslation();

  return (
    <div className="max-w-2xl mx-auto px-4 py-8">
      <div className="mb-6">
        <button
          type="button"
          onClick={onCancel}
          className="text-sm text-blue-600 hover:text-blue-800 flex items-center gap-1 mb-4"
        >
          {t('insurance.backToPolicies')}
        </button>
        <h1 className="text-2xl font-bold text-gray-900">{t('insurance.createPolicy.title')}</h1>
        <p className="text-gray-600 mt-2">{t('insurance.createPolicy.description')}</p>
      </div>

      <div className="bg-white rounded-lg shadow p-6">
        <PolicyForm
          buildings={buildings}
          isSubmitting={isSubmitting}
          onSubmit={onSubmit}
          onCancel={onCancel}
        />
      </div>
    </div>
  );
}
