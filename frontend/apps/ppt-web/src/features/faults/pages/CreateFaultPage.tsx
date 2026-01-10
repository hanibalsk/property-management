/**
 * CreateFaultPage - page for creating a new fault report.
 * Epic 4: Fault Reporting & Resolution (UC-03.1)
 * Epic 126: AI-Enhanced Fault Reporting (Story 126.1 - Photo-First)
 */

import { useCallback, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { type AiSuggestion, FaultForm, type FaultFormData } from '../components/FaultForm';
import type { UploadedPhoto } from '../components/PhotoUploader';

interface CreateFaultPageProps {
  buildings: Array<{ id: string; name: string }>;
  units: Array<{ id: string; designation: string }>;
  isSubmitting?: boolean;
  onSubmit: (data: FaultFormData) => void;
  onCancel: () => void;
  /** Enable photo-first mode with AI suggestions (Epic 126) */
  enablePhotoFirst?: boolean;
}

export function CreateFaultPage({
  buildings,
  units,
  isSubmitting,
  onSubmit,
  onCancel,
  enablePhotoFirst = true,
}: CreateFaultPageProps) {
  const { t } = useTranslation();
  const [aiSuggestion, setAiSuggestion] = useState<AiSuggestion | null>(null);
  const [aiSuggestionLoading, setAiSuggestionLoading] = useState(false);

  // Simulate AI suggestion when photos are uploaded
  // In production, this would call the backend API
  const handlePhotosChange = useCallback((photos: UploadedPhoto[]) => {
    if (
      photos.length > 0 &&
      photos.some((p) => p.status === 'pending' || p.status === 'uploaded')
    ) {
      setAiSuggestionLoading(true);
      // Simulate API call delay
      setTimeout(() => {
        // Mock AI suggestion - in production, call useRequestAiSuggestion
        setAiSuggestion({
          category: 'plumbing',
          confidence: 0.85,
          priority: 'medium',
        });
        setAiSuggestionLoading(false);
      }, 1500);
    } else {
      setAiSuggestion(null);
    }
  }, []);

  return (
    <div className="max-w-2xl mx-auto px-4 py-8">
      <div className="mb-6">
        <button
          type="button"
          onClick={onCancel}
          className="text-sm text-blue-600 hover:text-blue-800 flex items-center gap-1 mb-4"
        >
          ← {t('common.backToFaults')}
        </button>
        <h1 className="text-2xl font-bold text-gray-900">{t('faults.reportFault')}</h1>
        <p className="text-gray-600 mt-2">
          {enablePhotoFirst
            ? t('faults.photo.sectionDescription')
            : "Describe the issue you're experiencing. Include as much detail as possible to help us resolve it quickly."}
        </p>
      </div>

      <div className="bg-white rounded-lg shadow p-6">
        <FaultForm
          buildings={buildings}
          units={units}
          isSubmitting={isSubmitting}
          onSubmit={onSubmit}
          onCancel={onCancel}
          enablePhotoFirst={enablePhotoFirst}
          aiSuggestion={aiSuggestion}
          aiSuggestionLoading={aiSuggestionLoading}
          onPhotosChange={handlePhotosChange}
        />
      </div>
    </div>
  );
}
