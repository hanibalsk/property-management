/**
 * OCR Meter Reading Hook
 * Epic 128: OCR Meter Preview
 *
 * Handles OCR processing and correction feedback.
 */

import { useMutation, useQueryClient } from '@tanstack/react-query';
import type { OcrCorrection, OcrResult } from '../components/OcrPreviewCard';

const API_BASE = '/api/v1/ai';

/** Request to process meter image with OCR */
interface OcrProcessRequest {
  image: File;
  meterId: string;
}

/** Response from OCR processing */
interface OcrProcessResponse {
  result: OcrResult;
  imageUrl: string;
}

/** API helper for OCR requests */
async function ocrFetch<T>(url: string, options?: RequestInit): Promise<T> {
  const response = await fetch(url, options);

  if (!response.ok) {
    const error = await response.json().catch(() => ({ message: 'OCR request failed' }));
    throw new Error(error.message || `HTTP error ${response.status}`);
  }

  return response.json();
}

/** Process meter image with OCR */
export function useOcrProcessImage() {
  return useMutation({
    mutationFn: async ({ image, meterId }: OcrProcessRequest): Promise<OcrProcessResponse> => {
      const formData = new FormData();
      formData.append('image', image);
      formData.append('meter_id', meterId);

      return ocrFetch<OcrProcessResponse>(`${API_BASE}/ocr/meter-reading`, {
        method: 'POST',
        body: formData,
      });
    },
  });
}

/** Submit OCR correction for model training */
export function useOcrCorrection() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (correction: OcrCorrection): Promise<void> => {
      await ocrFetch<void>(`${API_BASE}/ocr/correction`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          original_value: correction.originalValue,
          corrected_value: correction.correctedValue,
          image_url: correction.imageUrl,
          bounding_box: correction.boundingBox,
          timestamp: correction.timestamp,
        }),
      });
    },
    onSuccess: () => {
      // Optionally invalidate any OCR-related caches
      queryClient.invalidateQueries({ queryKey: ['ocr'] });
    },
  });
}

/** Hook to handle the complete OCR flow */
export function useOcrMeterReading(meterId: string) {
  const processImage = useOcrProcessImage();
  const submitCorrection = useOcrCorrection();

  const process = async (image: File) => {
    return processImage.mutateAsync({ image, meterId });
  };

  const correctAndSubmit = async (correction: OcrCorrection) => {
    await submitCorrection.mutateAsync(correction);
  };

  return {
    process,
    correctAndSubmit,
    isProcessing: processImage.isPending,
    isSubmittingCorrection: submitCorrection.isPending,
    processingError: processImage.error,
    correctionError: submitCorrection.error,
    reset: () => {
      processImage.reset();
      submitCorrection.reset();
    },
  };
}
