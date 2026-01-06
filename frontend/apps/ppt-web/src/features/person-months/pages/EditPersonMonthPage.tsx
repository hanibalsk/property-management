/**
 * EditPersonMonthPage - edit person count for a month.
 * Allows users to set or update the number of persons for a unit in a given month.
 */

import { useTranslation } from 'react-i18next';
import { PersonMonthForm, type PersonMonthFormData } from '../components/PersonMonthForm';

interface EditPersonMonthPageProps {
  buildingName: string;
  unitDesignation: string;
  initialData?: Partial<PersonMonthFormData>;
  isSubmitting?: boolean;
  onSubmit: (data: PersonMonthFormData) => void;
  onCancel: () => void;
}

export function EditPersonMonthPage({
  buildingName,
  unitDesignation,
  initialData,
  isSubmitting,
  onSubmit,
  onCancel,
}: EditPersonMonthPageProps) {
  const { t } = useTranslation();

  const isEditing = initialData?.personCount !== undefined;

  return (
    <div className="max-w-2xl mx-auto px-4 py-8">
      {/* Header */}
      <div className="mb-6">
        <button
          type="button"
          onClick={onCancel}
          className="text-sm text-blue-600 hover:text-blue-800 flex items-center gap-1 mb-4"
        >
          &larr; {t('personMonths.backToUnit')}
        </button>
        <h1 className="text-2xl font-bold text-gray-900">
          {isEditing ? t('personMonths.editEntry') : t('personMonths.addEntry')}
        </h1>
        <p className="text-gray-600 mt-1">
          {buildingName} - {unitDesignation}
        </p>
      </div>

      {/* Description */}
      <div className="bg-blue-50 border border-blue-200 rounded-lg p-4 mb-6">
        <p className="text-sm text-blue-800">{t('personMonths.formDescription')}</p>
      </div>

      {/* Form */}
      <div className="bg-white rounded-lg shadow p-6">
        <PersonMonthForm
          initialData={initialData}
          isLoading={isSubmitting}
          onSubmit={onSubmit}
          onCancel={onCancel}
        />
      </div>
    </div>
  );
}
