/**
 * FeedbackPage - user feedback submission page.
 * UC-42: Onboarding/Help Feature
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { FeedbackFormData, FeedbackType } from '../types';

export interface FeedbackPageProps {
  isLoading?: boolean;
  isSubmitting?: boolean;
  onSubmit: (data: FeedbackFormData) => void;
  onNavigateBack: () => void;
}

export function FeedbackPage({
  isLoading,
  isSubmitting,
  onSubmit,
  onNavigateBack,
}: FeedbackPageProps) {
  const { t } = useTranslation();
  const [formData, setFormData] = useState<FeedbackFormData>({
    type: 'suggestion',
    subject: '',
    description: '',
    rating: undefined,
    attachments: undefined,
  });
  const [errors, setErrors] = useState<Partial<Record<keyof FeedbackFormData, string>>>({});
  const [hoveredRating, setHoveredRating] = useState<number | null>(null);
  const [isSubmitted, setIsSubmitted] = useState(false);

  const feedbackTypes: { value: FeedbackType; label: string; icon: React.ReactNode }[] = [
    {
      value: 'suggestion',
      label: t('help.feedback.typeSuggestion'),
      icon: (
        <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <title>{t('help.feedback.typeSuggestion')}</title>
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"
          />
        </svg>
      ),
    },
    {
      value: 'complaint',
      label: t('help.feedback.typeComplaint'),
      icon: (
        <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <title>{t('help.feedback.typeComplaint')}</title>
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
      ),
    },
    {
      value: 'compliment',
      label: t('help.feedback.typeCompliment'),
      icon: (
        <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <title>{t('help.feedback.typeCompliment')}</title>
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
      ),
    },
  ];

  const validate = (): boolean => {
    const newErrors: Partial<Record<keyof FeedbackFormData, string>> = {};

    if (!formData.subject.trim()) {
      newErrors.subject = t('help.feedback.errors.subjectRequired');
    }

    if (!formData.description.trim()) {
      newErrors.description = t('help.feedback.errors.descriptionRequired');
    } else if (formData.description.length < 10) {
      newErrors.description = t('help.feedback.errors.descriptionTooShort');
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (validate()) {
      onSubmit(formData);
      setIsSubmitted(true);
    }
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (files) {
      setFormData((prev) => ({
        ...prev,
        attachments: Array.from(files),
      }));
    }
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" />
      </div>
    );
  }

  if (isSubmitted) {
    return (
      <div className="max-w-2xl mx-auto px-4 py-8">
        <div className="text-center py-12 bg-white rounded-lg shadow">
          <svg
            className="mx-auto h-16 w-16 text-green-500 mb-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <title>{t('help.feedback.submitted')}</title>
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <h2 className="text-2xl font-bold text-gray-900 mb-2">{t('help.feedback.thankYou')}</h2>
          <p className="text-gray-600 mb-6">{t('help.feedback.submittedDescription')}</p>
          <button
            type="button"
            onClick={onNavigateBack}
            className="px-6 py-2 bg-blue-600 text-white font-medium rounded-md hover:bg-blue-700"
          >
            {t('help.backToHelpCenter')}
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-2xl mx-auto px-4 py-8">
      {/* Header */}
      <div className="mb-6">
        <button
          type="button"
          onClick={onNavigateBack}
          className="flex items-center text-sm text-gray-600 hover:text-gray-800 mb-4"
        >
          <svg className="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
            <title>{t('common.back')}</title>
            <path
              fillRule="evenodd"
              d="M9.707 16.707a1 1 0 01-1.414 0l-6-6a1 1 0 010-1.414l6-6a1 1 0 011.414 1.414L5.414 9H17a1 1 0 110 2H5.414l4.293 4.293a1 1 0 010 1.414z"
              clipRule="evenodd"
            />
          </svg>
          {t('help.backToHelpCenter')}
        </button>

        <h1 className="text-2xl font-bold text-gray-900">{t('help.feedback.title')}</h1>
        <p className="mt-1 text-sm text-gray-500">{t('help.feedback.subtitle')}</p>
      </div>

      <form onSubmit={handleSubmit} className="space-y-6">
        {/* Feedback type selector */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-3">
            {t('help.feedback.selectType')}
          </label>
          <div className="grid grid-cols-3 gap-3">
            {feedbackTypes.map((type) => (
              <button
                key={type.value}
                type="button"
                onClick={() => setFormData((prev) => ({ ...prev, type: type.value }))}
                className={`flex flex-col items-center p-4 rounded-lg border-2 transition-all ${
                  formData.type === type.value
                    ? 'border-blue-500 bg-blue-50 text-blue-700'
                    : 'border-gray-200 hover:border-gray-300 text-gray-600'
                }`}
              >
                <div
                  className={`p-2 rounded-full mb-2 ${formData.type === type.value ? 'bg-blue-100' : 'bg-gray-100'}`}
                >
                  {type.icon}
                </div>
                <span className="text-sm font-medium">{type.label}</span>
              </button>
            ))}
          </div>
        </div>

        {/* Subject */}
        <div>
          <label htmlFor="subject" className="block text-sm font-medium text-gray-700 mb-1">
            {t('help.feedback.subject')}
          </label>
          <input
            type="text"
            id="subject"
            value={formData.subject}
            onChange={(e) => setFormData((prev) => ({ ...prev, subject: e.target.value }))}
            placeholder={t('help.feedback.subjectPlaceholder')}
            className={`w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 ${
              errors.subject ? 'border-red-300' : 'border-gray-300'
            }`}
          />
          {errors.subject && <p className="mt-1 text-sm text-red-600">{errors.subject}</p>}
        </div>

        {/* Description */}
        <div>
          <label htmlFor="description" className="block text-sm font-medium text-gray-700 mb-1">
            {t('help.feedback.description')}
          </label>
          <textarea
            id="description"
            value={formData.description}
            onChange={(e) => setFormData((prev) => ({ ...prev, description: e.target.value }))}
            placeholder={t('help.feedback.descriptionPlaceholder')}
            rows={5}
            className={`w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 ${
              errors.description ? 'border-red-300' : 'border-gray-300'
            }`}
          />
          {errors.description && <p className="mt-1 text-sm text-red-600">{errors.description}</p>}
        </div>

        {/* Rating */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            {t('help.feedback.rating')} ({t('common.optional')})
          </label>
          <div className="flex items-center gap-1">
            {[1, 2, 3, 4, 5].map((star) => (
              <button
                key={star}
                type="button"
                onClick={() => setFormData((prev) => ({ ...prev, rating: star }))}
                onMouseEnter={() => setHoveredRating(star)}
                onMouseLeave={() => setHoveredRating(null)}
                className="p-1 focus:outline-none"
              >
                <svg
                  className={`w-8 h-8 transition-colors ${
                    star <= (hoveredRating ?? formData.rating ?? 0)
                      ? 'text-yellow-400'
                      : 'text-gray-300'
                  }`}
                  fill="currentColor"
                  viewBox="0 0 20 20"
                >
                  <title>{`${star} ${t('help.feedback.stars')}`}</title>
                  <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
                </svg>
              </button>
            ))}
            {formData.rating && (
              <button
                type="button"
                onClick={() => setFormData((prev) => ({ ...prev, rating: undefined }))}
                className="ml-2 text-sm text-gray-500 hover:text-gray-700"
              >
                {t('common.clearAll')}
              </button>
            )}
          </div>
        </div>

        {/* Screenshot attachment */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            {t('help.feedback.attachment')} ({t('common.optional')})
          </label>
          <div className="border-2 border-dashed border-gray-300 rounded-lg p-6 text-center hover:border-gray-400 transition-colors">
            <input
              type="file"
              id="attachment"
              accept="image/*"
              multiple
              onChange={handleFileChange}
              className="hidden"
            />
            <label htmlFor="attachment" className="cursor-pointer">
              <svg
                className="mx-auto h-12 w-12 text-gray-400 mb-3"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <title>{t('help.feedback.uploadScreenshot')}</title>
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={1.5}
                  d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                />
              </svg>
              <p className="text-sm text-gray-600">{t('help.feedback.uploadScreenshot')}</p>
              <p className="text-xs text-gray-400 mt-1">{t('help.feedback.uploadHint')}</p>
            </label>
          </div>
          {formData.attachments && formData.attachments.length > 0 && (
            <div className="mt-2 text-sm text-gray-600">
              {formData.attachments.length} {t('help.feedback.filesSelected')}
            </div>
          )}
        </div>

        {/* Submit button */}
        <div className="flex justify-end gap-3 pt-4 border-t">
          <button
            type="button"
            onClick={onNavigateBack}
            className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
          >
            {t('common.cancel')}
          </button>
          <button
            type="submit"
            disabled={isSubmitting}
            className="px-6 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isSubmitting ? t('common.loading') : t('common.submit')}
          </button>
        </div>
      </form>
    </div>
  );
}
