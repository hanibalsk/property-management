/**
 * ReadingForm component - form to submit a new meter reading.
 * Meters feature: Self-readings and consumption tracking.
 */

import { useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { Meter, ReadingFormData } from '../types';

interface ReadingFormProps {
  meter: Meter;
  isSubmitting?: boolean;
  onSubmit: (data: ReadingFormData) => void;
  onCancel: () => void;
}

export function ReadingForm({ meter, isSubmitting, onSubmit, onCancel }: ReadingFormProps) {
  const { t } = useTranslation();
  const fileInputRef = useRef<HTMLInputElement>(null);

  const [formData, setFormData] = useState<ReadingFormData>({
    meterId: meter.id,
    value: meter.lastReadingValue || 0,
    readingDate: new Date().toISOString().split('T')[0],
    notes: '',
  });
  const [photoPreview, setPhotoPreview] = useState<string | null>(null);
  const [errors, setErrors] = useState<Partial<Record<keyof ReadingFormData, string>>>({});

  const validate = (): boolean => {
    const newErrors: Partial<Record<keyof ReadingFormData, string>> = {};

    if (formData.value < 0) {
      newErrors.value = t('meters.form.errors.valueNegative');
    }

    if (meter.lastReadingValue !== undefined && formData.value < meter.lastReadingValue) {
      newErrors.value = t('meters.form.errors.valueLowerThanLast');
    }

    if (!formData.readingDate) {
      newErrors.readingDate = t('meters.form.errors.dateRequired');
    }

    const readingDate = new Date(formData.readingDate);
    const today = new Date();
    if (readingDate > today) {
      newErrors.readingDate = t('meters.form.errors.futureDate');
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (validate()) {
      onSubmit(formData);
    }
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value, type } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: type === 'number' ? Number.parseFloat(value) || 0 : value,
    }));
    if (errors[name as keyof ReadingFormData]) {
      setErrors((prev) => ({ ...prev, [name]: undefined }));
    }
  };

  const handlePhotoChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      setFormData((prev) => ({ ...prev, photo: file }));
      const reader = new FileReader();
      reader.onloadend = () => {
        setPhotoPreview(reader.result as string);
      };
      reader.readAsDataURL(file);
    }
  };

  const handleRemovePhoto = () => {
    setFormData((prev) => ({ ...prev, photo: undefined }));
    setPhotoPreview(null);
    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }
  };

  const consumption =
    meter.lastReadingValue !== undefined ? formData.value - meter.lastReadingValue : null;

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Meter Info */}
      <div className="bg-gray-50 rounded-lg p-4">
        <h3 className="text-sm font-medium text-gray-700 mb-2">{t('meters.meterInfo')}</h3>
        <dl className="grid grid-cols-2 gap-2 text-sm">
          <dt className="text-gray-500">{t('meters.type')}</dt>
          <dd className="font-medium">{t(`meters.types.${meter.meterType}`)}</dd>
          <dt className="text-gray-500">{t('meters.serialNumber')}</dt>
          <dd className="font-medium">{meter.serialNumber}</dd>
          {meter.lastReadingValue !== undefined && (
            <>
              <dt className="text-gray-500">{t('meters.lastReading')}</dt>
              <dd className="font-medium">
                {meter.lastReadingValue.toLocaleString()} {meter.unit}
              </dd>
            </>
          )}
        </dl>
      </div>

      {/* Reading Value */}
      <div>
        <label htmlFor="value" className="block text-sm font-medium text-gray-700">
          {t('meters.form.currentReading')} ({meter.unit}) *
        </label>
        <input
          type="number"
          id="value"
          name="value"
          value={formData.value}
          onChange={handleChange}
          step="0.01"
          min="0"
          className={`mt-1 block w-full rounded-md border ${
            errors.value ? 'border-red-500' : 'border-gray-300'
          } px-3 py-2 text-lg font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500`}
        />
        {errors.value && <p className="mt-1 text-sm text-red-500">{errors.value}</p>}
        {consumption !== null && consumption >= 0 && (
          <p className="mt-1 text-sm text-gray-600">
            {t('meters.form.consumption')}: {consumption.toLocaleString()} {meter.unit}
          </p>
        )}
      </div>

      {/* Reading Date */}
      <div>
        <label htmlFor="readingDate" className="block text-sm font-medium text-gray-700">
          {t('meters.form.readingDate')} *
        </label>
        <input
          type="date"
          id="readingDate"
          name="readingDate"
          value={formData.readingDate}
          onChange={handleChange}
          max={new Date().toISOString().split('T')[0]}
          className={`mt-1 block w-full rounded-md border ${
            errors.readingDate ? 'border-red-500' : 'border-gray-300'
          } px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500`}
        />
        {errors.readingDate && <p className="mt-1 text-sm text-red-500">{errors.readingDate}</p>}
      </div>

      {/* Photo Upload */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          {t('meters.form.photo')} ({t('common.optional')})
        </label>
        {photoPreview ? (
          <div className="relative inline-block">
            <img
              src={photoPreview}
              alt={t('meters.form.photoPreview')}
              className="w-48 h-48 object-cover rounded-lg border"
            />
            <button
              type="button"
              onClick={handleRemovePhoto}
              className="absolute top-1 right-1 p-1 bg-red-500 text-white rounded-full hover:bg-red-600"
              aria-label={t('common.delete')}
            >
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M6 18L18 6M6 6l12 12"
                />
              </svg>
            </button>
          </div>
        ) : (
          <label className="flex flex-col items-center justify-center w-full h-32 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer hover:bg-gray-50">
            <div className="flex flex-col items-center justify-center pt-5 pb-6">
              <svg
                className="w-8 h-8 mb-2 text-gray-400"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z"
                />
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M15 13a3 3 0 11-6 0 3 3 0 016 0z"
                />
              </svg>
              <p className="text-sm text-gray-500">{t('meters.form.uploadPhoto')}</p>
            </div>
            <input
              ref={fileInputRef}
              type="file"
              accept="image/*"
              className="hidden"
              onChange={handlePhotoChange}
            />
          </label>
        )}
        <p className="mt-1 text-xs text-gray-500">{t('meters.form.photoHelp')}</p>
      </div>

      {/* Notes */}
      <div>
        <label htmlFor="notes" className="block text-sm font-medium text-gray-700">
          {t('meters.form.notes')} ({t('common.optional')})
        </label>
        <textarea
          id="notes"
          name="notes"
          value={formData.notes || ''}
          onChange={handleChange}
          rows={3}
          placeholder={t('meters.form.notesPlaceholder')}
          className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      {/* Actions */}
      <div className="flex items-center justify-end gap-3 pt-4 border-t">
        <button
          type="button"
          onClick={onCancel}
          disabled={isSubmitting}
          className="px-4 py-2 text-gray-700 border border-gray-300 rounded-lg hover:bg-gray-50 disabled:opacity-50"
        >
          {t('common.cancel')}
        </button>
        <button
          type="submit"
          disabled={isSubmitting}
          className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 flex items-center gap-2"
        >
          {isSubmitting && (
            <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white" />
          )}
          {isSubmitting ? t('common.saving') : t('meters.form.submitReading')}
        </button>
      </div>
    </form>
  );
}
