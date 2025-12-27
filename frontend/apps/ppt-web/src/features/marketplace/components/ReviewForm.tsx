/**
 * ReviewForm component - form for submitting provider reviews.
 * Epic 68: Service Provider Marketplace (Story 68.5)
 */

import { useState } from 'react';
import { RatingInput } from './RatingStars';

export interface ReviewFormData {
  qualityRating: number;
  timelinessRating: number;
  communicationRating: number;
  valueRating: number;
  reviewTitle?: string;
  reviewText?: string;
  jobId?: string;
  rfqId?: string;
}

interface ReviewFormProps {
  providerName: string;
  jobReference?: string;
  onSubmit: (data: ReviewFormData) => void;
  onCancel: () => void;
  isLoading?: boolean;
}

const defaultFormData: ReviewFormData = {
  qualityRating: 0,
  timelinessRating: 0,
  communicationRating: 0,
  valueRating: 0,
};

export function ReviewForm({
  providerName,
  jobReference,
  onSubmit,
  onCancel,
  isLoading,
}: ReviewFormProps) {
  const [formData, setFormData] = useState<ReviewFormData>(defaultFormData);
  const [errors, setErrors] = useState<Record<string, string>>({});

  const updateField = <K extends keyof ReviewFormData>(key: K, value: ReviewFormData[K]) => {
    setFormData((prev) => ({ ...prev, [key]: value }));
    if (errors[key]) {
      setErrors((prev) => {
        const next = { ...prev };
        delete next[key];
        return next;
      });
    }
  };

  const validate = (): boolean => {
    const newErrors: Record<string, string> = {};

    if (formData.qualityRating === 0) {
      newErrors.qualityRating = 'Please rate quality';
    }
    if (formData.timelinessRating === 0) {
      newErrors.timelinessRating = 'Please rate timeliness';
    }
    if (formData.communicationRating === 0) {
      newErrors.communicationRating = 'Please rate communication';
    }
    if (formData.valueRating === 0) {
      newErrors.valueRating = 'Please rate value';
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

  const overallRating =
    (formData.qualityRating +
      formData.timelinessRating +
      formData.communicationRating +
      formData.valueRating) /
    4;

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-gray-900 mb-2">
          Review {providerName}
        </h2>
        {jobReference && (
          <p className="text-sm text-gray-500 mb-6">For job: {jobReference}</p>
        )}

        {/* Rating Inputs */}
        <div className="space-y-6">
          <div>
            <div className="flex items-center justify-between mb-2">
              <label className="text-sm font-medium text-gray-700">Quality of Work *</label>
              {errors.qualityRating && (
                <span className="text-sm text-red-600">{errors.qualityRating}</span>
              )}
            </div>
            <RatingInput
              value={formData.qualityRating}
              onChange={(val) => updateField('qualityRating', val)}
            />
            <p className="mt-1 text-xs text-gray-500">
              How would you rate the quality of the work performed?
            </p>
          </div>

          <div>
            <div className="flex items-center justify-between mb-2">
              <label className="text-sm font-medium text-gray-700">Timeliness *</label>
              {errors.timelinessRating && (
                <span className="text-sm text-red-600">{errors.timelinessRating}</span>
              )}
            </div>
            <RatingInput
              value={formData.timelinessRating}
              onChange={(val) => updateField('timelinessRating', val)}
            />
            <p className="mt-1 text-xs text-gray-500">
              Did the provider complete the work on time?
            </p>
          </div>

          <div>
            <div className="flex items-center justify-between mb-2">
              <label className="text-sm font-medium text-gray-700">Communication *</label>
              {errors.communicationRating && (
                <span className="text-sm text-red-600">{errors.communicationRating}</span>
              )}
            </div>
            <RatingInput
              value={formData.communicationRating}
              onChange={(val) => updateField('communicationRating', val)}
            />
            <p className="mt-1 text-xs text-gray-500">
              How responsive and professional was their communication?
            </p>
          </div>

          <div>
            <div className="flex items-center justify-between mb-2">
              <label className="text-sm font-medium text-gray-700">Value for Money *</label>
              {errors.valueRating && (
                <span className="text-sm text-red-600">{errors.valueRating}</span>
              )}
            </div>
            <RatingInput
              value={formData.valueRating}
              onChange={(val) => updateField('valueRating', val)}
            />
            <p className="mt-1 text-xs text-gray-500">
              Was the pricing fair for the work delivered?
            </p>
          </div>

          {/* Overall Rating Preview */}
          {overallRating > 0 && (
            <div className="p-4 bg-gray-50 rounded-lg text-center">
              <p className="text-sm text-gray-500">Overall Rating</p>
              <p className="text-3xl font-bold text-gray-900">{overallRating.toFixed(1)}</p>
              <div className="mt-1 flex justify-center">
                {[...Array(5)].map((_, i) => (
                  <svg
                    key={`overall-star-${i}`}
                    className={`w-5 h-5 ${
                      i < Math.round(overallRating) ? 'text-yellow-400' : 'text-gray-300'
                    }`}
                    fill="currentColor"
                    viewBox="0 0 20 20"
                  >
                    <title>{i < Math.round(overallRating) ? 'Filled star' : 'Empty star'}</title>
                    <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
                  </svg>
                ))}
              </div>
            </div>
          )}
        </div>

        {/* Written Review */}
        <div className="mt-6 pt-6 border-t space-y-4">
          <div>
            <label htmlFor="reviewTitle" className="block text-sm font-medium text-gray-700 mb-1">
              Review Title
            </label>
            <input
              id="reviewTitle"
              type="text"
              value={formData.reviewTitle || ''}
              onChange={(e) => updateField('reviewTitle', e.target.value)}
              placeholder="Summarize your experience"
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div>
            <label htmlFor="reviewText" className="block text-sm font-medium text-gray-700 mb-1">
              Your Review
            </label>
            <textarea
              id="reviewText"
              rows={4}
              value={formData.reviewText || ''}
              onChange={(e) => updateField('reviewText', e.target.value)}
              placeholder="Share details about your experience with this provider..."
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
            />
            <p className="mt-1 text-xs text-gray-500">
              Your review helps other property managers make informed decisions.
            </p>
          </div>
        </div>

        {/* Guidelines */}
        <div className="mt-6 p-4 bg-blue-50 rounded-lg">
          <h3 className="text-sm font-medium text-blue-800">Review Guidelines</h3>
          <ul className="mt-2 text-sm text-blue-700 list-disc list-inside space-y-1">
            <li>Be honest and specific about your experience</li>
            <li>Focus on the work performed and professionalism</li>
            <li>Avoid personal information or inappropriate content</li>
            <li>The provider can respond to your review</li>
          </ul>
        </div>
      </div>

      {/* Actions */}
      <div className="flex items-center justify-end gap-3">
        <button
          type="button"
          onClick={onCancel}
          className="px-4 py-2 text-gray-700 font-medium hover:text-gray-900"
        >
          Cancel
        </button>
        <button
          type="submit"
          disabled={isLoading}
          className="px-6 py-2 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
        >
          {isLoading ? 'Submitting...' : 'Submit Review'}
        </button>
      </div>
    </form>
  );
}

interface ResponseFormProps {
  reviewId: string;
  onSubmit: (reviewId: string, response: string) => void;
  onCancel: () => void;
  isLoading?: boolean;
}

export function ResponseForm({ reviewId, onSubmit, onCancel, isLoading }: ResponseFormProps) {
  const [response, setResponse] = useState('');
  const [error, setError] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!response.trim()) {
      setError('Please enter a response');
      return;
    }
    onSubmit(reviewId, response);
  };

  return (
    <form onSubmit={handleSubmit} className="bg-white rounded-lg shadow p-6">
      <h3 className="text-lg font-medium text-gray-900 mb-4">Respond to Review</h3>

      <div>
        <textarea
          rows={4}
          value={response}
          onChange={(e) => {
            setResponse(e.target.value);
            if (error) setError('');
          }}
          placeholder="Write your response to this review..."
          className={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 ${
            error ? 'border-red-500' : 'border-gray-300'
          }`}
        />
        {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
        <p className="mt-1 text-xs text-gray-500">
          Your response will be visible to everyone who views this review.
        </p>
      </div>

      <div className="mt-4 flex items-center justify-end gap-3">
        <button
          type="button"
          onClick={onCancel}
          className="px-4 py-2 text-gray-700 font-medium hover:text-gray-900"
        >
          Cancel
        </button>
        <button
          type="submit"
          disabled={isLoading}
          className="px-6 py-2 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
        >
          {isLoading ? 'Submitting...' : 'Post Response'}
        </button>
      </div>
    </form>
  );
}
