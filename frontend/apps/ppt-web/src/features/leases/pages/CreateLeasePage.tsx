/**
 * CreateLeasePage - Create a new lease from template.
 * Epic 19: Lease Management & Tenant Screening
 */

import { useTranslation } from 'react-i18next';
import { LeaseForm } from '../components/LeaseForm';
import type { CreateLeaseData, LeaseTemplate } from '../types';

interface Unit {
  id: string;
  number: string;
  buildingName: string;
}

interface Tenant {
  id: string;
  name: string;
  email: string;
}

interface CreateLeasePageProps {
  units: Unit[];
  tenants: Tenant[];
  templates: LeaseTemplate[];
  isLoading?: boolean;
  isSubmitting?: boolean;
  onSubmit: (data: CreateLeaseData) => void;
  onCancel: () => void;
}

export function CreateLeasePage({
  units,
  tenants,
  templates,
  isLoading,
  isSubmitting,
  onSubmit,
  onCancel,
}: CreateLeasePageProps) {
  const { t } = useTranslation();

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" />
      </div>
    );
  }

  return (
    <div className="max-w-2xl mx-auto px-4 py-8">
      {/* Back Button */}
      <button
        type="button"
        onClick={onCancel}
        className="flex items-center gap-2 text-gray-600 hover:text-gray-800 mb-6"
      >
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
        </svg>
        {t('leases.create.back')}
      </button>

      {/* Header */}
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-gray-900">{t('leases.create.title')}</h1>
        <p className="mt-1 text-sm text-gray-500">{t('leases.create.subtitle')}</p>
      </div>

      {/* Form */}
      <div className="bg-white rounded-lg shadow p-6">
        <LeaseForm
          units={units}
          tenants={tenants}
          templates={templates}
          isSubmitting={isSubmitting}
          onSubmit={onSubmit}
          onCancel={onCancel}
        />
      </div>
    </div>
  );
}
