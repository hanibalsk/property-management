/**
 * CreateDelegationPage - page for creating a new delegation.
 * Epic 3: Ownership Management (Story 3.4.1)
 */

import { useTranslation } from 'react-i18next';
import {
  CreateDelegationForm,
  type CreateDelegationFormData,
} from '../components/CreateDelegationForm';

interface CreateDelegationPageProps {
  users: Array<{ id: string; name: string; email: string }>;
  units: Array<{ id: string; designation: string }>;
  isSubmitting?: boolean;
  onSubmit: (data: CreateDelegationFormData) => void;
  onCancel: () => void;
}

export function CreateDelegationPage({
  users,
  units,
  isSubmitting,
  onSubmit,
  onCancel,
}: CreateDelegationPageProps) {
  const { t } = useTranslation();

  return (
    <div className="max-w-2xl mx-auto px-4 py-8">
      <div className="mb-6">
        <button
          type="button"
          onClick={onCancel}
          className="text-sm text-blue-600 hover:text-blue-800 flex items-center gap-1 mb-4"
        >
          {t('delegation.backToDelegations')}
        </button>
        <h1 className="text-2xl font-bold text-gray-900">{t('delegation.createDelegation')}</h1>
        <p className="text-gray-600 mt-2">{t('delegation.createDescription')}</p>
      </div>

      <div className="bg-white rounded-lg shadow p-6">
        <CreateDelegationForm
          users={users}
          units={units}
          isSubmitting={isSubmitting}
          onSubmit={onSubmit}
          onCancel={onCancel}
        />
      </div>
    </div>
  );
}
