/**
 * PersonMonthForm component - form to enter/edit person count.
 * Used for recording the number of persons living in a unit for a given month.
 */

import { type FormEvent, useState } from 'react';
import { useTranslation } from 'react-i18next';

export interface PersonMonthFormData {
  year: number;
  month: number;
  personCount: number;
}

interface PersonMonthFormProps {
  initialData?: Partial<PersonMonthFormData>;
  isLoading?: boolean;
  onSubmit: (data: PersonMonthFormData) => void;
  onCancel: () => void;
}

const currentYear = new Date().getFullYear();
const currentMonth = new Date().getMonth() + 1;

const monthOptions = [
  { value: 1, key: 'january' },
  { value: 2, key: 'february' },
  { value: 3, key: 'march' },
  { value: 4, key: 'april' },
  { value: 5, key: 'may' },
  { value: 6, key: 'june' },
  { value: 7, key: 'july' },
  { value: 8, key: 'august' },
  { value: 9, key: 'september' },
  { value: 10, key: 'october' },
  { value: 11, key: 'november' },
  { value: 12, key: 'december' },
];

const yearOptions = Array.from({ length: 5 }, (_, i) => currentYear - 2 + i);

export function PersonMonthForm({
  initialData,
  isLoading,
  onSubmit,
  onCancel,
}: PersonMonthFormProps) {
  const { t } = useTranslation();
  const [formData, setFormData] = useState<PersonMonthFormData>({
    year: initialData?.year ?? currentYear,
    month: initialData?.month ?? currentMonth,
    personCount: initialData?.personCount ?? 1,
  });
  const [errors, setErrors] = useState<Partial<Record<keyof PersonMonthFormData, string>>>({});

  const validate = (): boolean => {
    const newErrors: Partial<Record<keyof PersonMonthFormData, string>> = {};

    if (formData.personCount < 0) {
      newErrors.personCount = t('personMonths.errors.personCountInvalid');
    }
    if (formData.personCount > 100) {
      newErrors.personCount = t('personMonths.errors.personCountTooHigh');
    }
    if (!formData.year || formData.year < 2000 || formData.year > 2100) {
      newErrors.year = t('personMonths.errors.yearInvalid');
    }
    if (!formData.month || formData.month < 1 || formData.month > 12) {
      newErrors.month = t('personMonths.errors.monthInvalid');
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    if (validate()) {
      onSubmit(formData);
    }
  };

  const handlePersonCountChange = (value: string) => {
    const count = Number.parseInt(value, 10);
    if (!Number.isNaN(count)) {
      setFormData((prev) => ({ ...prev, personCount: count }));
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Year and Month */}
      <div className="grid grid-cols-2 gap-4">
        <div>
          <label htmlFor="year" className="block text-sm font-medium text-gray-700">
            {t('personMonths.form.year')} *
          </label>
          <select
            id="year"
            value={formData.year}
            onChange={(e) =>
              setFormData((prev) => ({ ...prev, year: Number.parseInt(e.target.value, 10) }))
            }
            className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
              errors.year ? 'border-red-300' : 'border-gray-300'
            }`}
          >
            {yearOptions.map((year) => (
              <option key={year} value={year}>
                {year}
              </option>
            ))}
          </select>
          {errors.year && <p className="mt-1 text-sm text-red-600">{errors.year}</p>}
        </div>

        <div>
          <label htmlFor="month" className="block text-sm font-medium text-gray-700">
            {t('personMonths.form.month')} *
          </label>
          <select
            id="month"
            value={formData.month}
            onChange={(e) =>
              setFormData((prev) => ({ ...prev, month: Number.parseInt(e.target.value, 10) }))
            }
            className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
              errors.month ? 'border-red-300' : 'border-gray-300'
            }`}
          >
            {monthOptions.map((option) => (
              <option key={option.value} value={option.value}>
                {t(`personMonths.months.${option.key}`)}
              </option>
            ))}
          </select>
          {errors.month && <p className="mt-1 text-sm text-red-600">{errors.month}</p>}
        </div>
      </div>

      {/* Person Count */}
      <div>
        <label htmlFor="personCount" className="block text-sm font-medium text-gray-700">
          {t('personMonths.form.personCount')} *
        </label>
        <div className="mt-1 flex items-center gap-3">
          <button
            type="button"
            onClick={() =>
              formData.personCount > 0 &&
              setFormData((prev) => ({ ...prev, personCount: prev.personCount - 1 }))
            }
            disabled={formData.personCount <= 0}
            className="w-10 h-10 flex items-center justify-center rounded-full border border-gray-300 bg-white hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span className="text-lg font-medium">-</span>
          </button>
          <input
            type="number"
            id="personCount"
            min={0}
            max={100}
            value={formData.personCount}
            onChange={(e) => handlePersonCountChange(e.target.value)}
            className={`block w-24 px-3 py-2 text-center text-lg font-medium border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
              errors.personCount ? 'border-red-300' : 'border-gray-300'
            }`}
          />
          <button
            type="button"
            onClick={() =>
              formData.personCount < 100 &&
              setFormData((prev) => ({ ...prev, personCount: prev.personCount + 1 }))
            }
            disabled={formData.personCount >= 100}
            className="w-10 h-10 flex items-center justify-center rounded-full border border-gray-300 bg-white hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span className="text-lg font-medium">+</span>
          </button>
        </div>
        {errors.personCount && <p className="mt-1 text-sm text-red-600">{errors.personCount}</p>}
        <p className="mt-2 text-xs text-gray-500">{t('personMonths.form.personCountHelp')}</p>
      </div>

      {/* Actions */}
      <div className="flex items-center justify-end gap-3 pt-4 border-t">
        <button
          type="button"
          onClick={onCancel}
          disabled={isLoading}
          className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50"
        >
          {t('common.cancel')}
        </button>
        <button
          type="submit"
          disabled={isLoading}
          className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50 flex items-center gap-2"
        >
          {isLoading && (
            <span className="animate-spin rounded-full h-4 w-4 border-b-2 border-white" />
          )}
          {t('common.save')}
        </button>
      </div>
    </form>
  );
}
