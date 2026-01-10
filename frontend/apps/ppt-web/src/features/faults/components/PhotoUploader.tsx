/**
 * PhotoUploader component (Epic 126, Story 126.1).
 *
 * Provides a photo-first experience for fault reporting.
 * Users can upload photos before filling out the form,
 * which are then analyzed for AI categorization.
 */

import { useCallback, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';

export interface UploadedPhoto {
  id: string;
  file: File;
  preview: string;
  status: 'pending' | 'uploading' | 'uploaded' | 'error';
  error?: string;
}

/** Generate a unique photo ID */
function generateId() {
  return `photo-${Date.now()}-${Math.random().toString(36).slice(2, 9)}`;
}

interface PhotoUploaderProps {
  /** Maximum number of photos allowed */
  maxPhotos?: number;
  /** Maximum file size in bytes (default 10MB) */
  maxFileSize?: number;
  /** Accepted file types */
  accept?: string;
  /** Photos already uploaded */
  photos: UploadedPhoto[];
  /** Callback when photos change */
  onPhotosChange: (photos: UploadedPhoto[]) => void;
  /** Whether the uploader is disabled */
  disabled?: boolean;
}

export function PhotoUploader({
  maxPhotos = 5,
  maxFileSize = 10 * 1024 * 1024, // 10MB
  accept = 'image/jpeg,image/png,image/webp,image/heic',
  photos,
  onPhotosChange,
  disabled = false,
}: PhotoUploaderProps) {
  const { t } = useTranslation();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [isDragging, setIsDragging] = useState(false);

  const validateFile = useCallback(
    (file: File): string | null => {
      if (!file.type.startsWith('image/')) {
        return t('faults.photo.invalidType');
      }
      if (file.size > maxFileSize) {
        return t('faults.photo.tooLarge', { maxSize: Math.round(maxFileSize / 1024 / 1024) });
      }
      return null;
    },
    [maxFileSize, t]
  );

  const handleFiles = useCallback(
    (files: FileList | File[]) => {
      const fileArray = Array.from(files);
      const remainingSlots = maxPhotos - photos.length;

      if (remainingSlots <= 0) {
        return;
      }

      const newPhotos: UploadedPhoto[] = [];

      for (const file of fileArray.slice(0, remainingSlots)) {
        const error = validateFile(file);
        const preview = URL.createObjectURL(file);

        newPhotos.push({
          id: generateId(),
          file,
          preview,
          status: error ? 'error' : 'pending',
          error: error || undefined,
        });
      }

      if (newPhotos.length > 0) {
        onPhotosChange([...photos, ...newPhotos]);
      }
    },
    [maxPhotos, photos, validateFile, onPhotosChange]
  );

  const handleDrop = useCallback(
    (e: React.DragEvent) => {
      e.preventDefault();
      setIsDragging(false);

      if (disabled) return;

      const { files } = e.dataTransfer;
      if (files.length > 0) {
        handleFiles(files);
      }
    },
    [disabled, handleFiles]
  );

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(true);
  }, []);

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);
  }, []);

  const handleInputChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const { files } = e.target;
      if (files && files.length > 0) {
        handleFiles(files);
      }
      // Reset input so same file can be selected again
      e.target.value = '';
    },
    [handleFiles]
  );

  const handleRemovePhoto = useCallback(
    (photoId: string) => {
      const photo = photos.find((p) => p.id === photoId);
      if (photo) {
        URL.revokeObjectURL(photo.preview);
      }
      onPhotosChange(photos.filter((p) => p.id !== photoId));
    },
    [photos, onPhotosChange]
  );

  const handleClick = useCallback(() => {
    if (!disabled && fileInputRef.current) {
      fileInputRef.current.click();
    }
  }, [disabled]);

  const canAddMore = photos.length < maxPhotos;

  return (
    <div className="photo-uploader">
      {/* Hidden file input */}
      <input
        ref={fileInputRef}
        type="file"
        accept={accept}
        multiple
        onChange={handleInputChange}
        className="sr-only"
        disabled={disabled}
        aria-label={t('faults.photo.selectPhotos')}
      />

      {/* Drop zone */}
      {canAddMore && (
        <div
          role="button"
          tabIndex={disabled ? -1 : 0}
          onClick={handleClick}
          onKeyDown={(e) => {
            if (e.key === 'Enter' || e.key === ' ') {
              e.preventDefault();
              handleClick();
            }
          }}
          onDrop={handleDrop}
          onDragOver={handleDragOver}
          onDragLeave={handleDragLeave}
          className={`
            relative border-2 border-dashed rounded-lg p-8 text-center cursor-pointer
            transition-colors duration-200
            ${isDragging ? 'border-blue-500 bg-blue-50' : 'border-gray-300 hover:border-gray-400'}
            ${disabled ? 'opacity-50 cursor-not-allowed' : ''}
          `}
          aria-label={t('faults.photo.dropZoneLabel')}
        >
          {/* Camera icon */}
          <div className="flex flex-col items-center gap-3">
            <div className="w-16 h-16 rounded-full bg-blue-100 flex items-center justify-center">
              <svg
                className="w-8 h-8 text-blue-600"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
                aria-hidden="true"
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
            </div>
            <div>
              <p className="text-lg font-medium text-gray-900">{t('faults.photo.addPhotos')}</p>
              <p className="text-sm text-gray-500">{t('faults.photo.dragOrClick')}</p>
            </div>
            <p className="text-xs text-gray-400">
              {t('faults.photo.maxPhotos', { max: maxPhotos - photos.length })}
            </p>
          </div>
        </div>
      )}

      {/* Photo preview grid */}
      {photos.length > 0 && (
        <div
          className="mt-4 grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4"
          role="list"
          aria-label={t('faults.photo.uploadedPhotos')}
        >
          {photos.map((photo) => (
            <div
              key={photo.id}
              className="relative group aspect-square rounded-lg overflow-hidden bg-gray-100"
              role="listitem"
            >
              <img
                src={photo.preview}
                alt={t('faults.photo.photoAlt', { name: photo.file.name })}
                className="w-full h-full object-cover"
              />

              {/* Status overlay */}
              {photo.status === 'uploading' && (
                <div className="absolute inset-0 bg-black/50 flex items-center justify-center">
                  <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-white" />
                </div>
              )}

              {photo.status === 'error' && (
                <div className="absolute inset-0 bg-red-500/50 flex items-center justify-center">
                  <div className="text-white text-center p-2">
                    <svg
                      className="w-6 h-6 mx-auto"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                      />
                    </svg>
                    <p className="text-xs mt-1">{photo.error}</p>
                  </div>
                </div>
              )}

              {/* Remove button */}
              <button
                type="button"
                onClick={() => handleRemovePhoto(photo.id)}
                className={`
                  absolute top-2 right-2 p-1.5 rounded-full
                  bg-black/50 text-white opacity-0 group-hover:opacity-100
                  hover:bg-black/70 transition-opacity
                  focus:opacity-100 focus:outline-none focus:ring-2 focus:ring-white
                `}
                aria-label={t('faults.photo.removePhoto', { name: photo.file.name })}
                disabled={disabled}
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

              {/* Photo filename */}
              <div className="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/60 to-transparent p-2">
                <p className="text-xs text-white truncate">{photo.file.name}</p>
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Helper text */}
      <p className="mt-2 text-xs text-gray-500">
        {t('faults.photo.acceptedFormats')} •{' '}
        {t('faults.photo.maxSize', { size: Math.round(maxFileSize / 1024 / 1024) })}
      </p>
    </div>
  );
}

PhotoUploader.displayName = 'PhotoUploader';
