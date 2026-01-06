/**
 * AttachmentPreview Component
 *
 * Displays attached files with icon/thumbnail, file name, size,
 * and actions for removing or downloading attachments.
 */

import { useTranslation } from 'react-i18next';
import type { MessageAttachment } from '../types';

interface AttachmentPreviewProps {
  attachments: MessageAttachment[];
  onRemove?: (attachmentId: string) => void;
  isEditable?: boolean;
}

function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${Number.parseFloat((bytes / k ** i).toFixed(1))} ${sizes[i]}`;
}

function getFileIcon(type: string): React.ReactNode {
  if (type.startsWith('image/')) {
    return (
      <svg
        className="w-5 h-5 text-blue-500"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
        aria-hidden="true"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={2}
          d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
        />
      </svg>
    );
  }
  if (type === 'application/pdf') {
    return (
      <svg
        className="w-5 h-5 text-red-500"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
        aria-hidden="true"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={2}
          d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"
        />
      </svg>
    );
  }
  return (
    <svg
      className="w-5 h-5 text-gray-500"
      fill="none"
      stroke="currentColor"
      viewBox="0 0 24 24"
      aria-hidden="true"
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        strokeWidth={2}
        d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
      />
    </svg>
  );
}

function isImageType(type: string): boolean {
  return type.startsWith('image/');
}

export function AttachmentPreview({
  attachments,
  onRemove,
  isEditable = false,
}: AttachmentPreviewProps) {
  const { t } = useTranslation();

  if (attachments.length === 0) {
    return null;
  }

  return (
    <div className="flex flex-wrap gap-2">
      {attachments.map((attachment) => (
        <div
          key={attachment.id}
          className="relative group flex items-center gap-2 p-2 bg-gray-100 rounded-lg border border-gray-200 max-w-xs"
        >
          {/* Thumbnail or Icon */}
          {isImageType(attachment.type) ? (
            <div className="w-10 h-10 flex-shrink-0 rounded overflow-hidden bg-gray-200">
              <img
                src={attachment.url}
                alt={attachment.name}
                className="w-full h-full object-cover"
              />
            </div>
          ) : (
            <div className="w-10 h-10 flex-shrink-0 flex items-center justify-center bg-gray-200 rounded">
              {getFileIcon(attachment.type)}
            </div>
          )}

          {/* File Info */}
          <div className="flex-1 min-w-0">
            <p className="text-sm font-medium text-gray-900 truncate" title={attachment.name}>
              {attachment.name}
            </p>
            <p className="text-xs text-gray-500">{formatFileSize(attachment.size)}</p>
          </div>

          {/* Actions */}
          <div className="flex items-center gap-1">
            {/* Download button (always shown for received attachments) */}
            {!isEditable && (
              <a
                href={attachment.url}
                download={attachment.name}
                className="p-1 text-gray-400 hover:text-blue-600 rounded transition-colors"
                title={t('documents.download')}
              >
                <span className="sr-only">{t('documents.download')}</span>
                <svg
                  className="w-4 h-4"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                  aria-hidden="true"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                  />
                </svg>
              </a>
            )}

            {/* Remove button (only shown when editable) */}
            {isEditable && onRemove && (
              <button
                type="button"
                onClick={() => onRemove(attachment.id)}
                className="p-1 text-gray-400 hover:text-red-600 rounded transition-colors"
                title={t('common.delete')}
              >
                <svg
                  className="w-4 h-4"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                  aria-hidden="true"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M6 18L18 6M6 6l12 12"
                  />
                </svg>
              </button>
            )}
          </div>
        </div>
      ))}
    </div>
  );
}

interface AttachmentPreviewInlineProps {
  attachments: MessageAttachment[];
}

/**
 * Inline attachment preview for message bubbles.
 * Shows attachments within the message context.
 */
export function AttachmentPreviewInline({ attachments }: AttachmentPreviewInlineProps) {
  if (attachments.length === 0) {
    return null;
  }

  // Separate images and other files
  const images = attachments.filter((a) => isImageType(a.type));
  const files = attachments.filter((a) => !isImageType(a.type));

  return (
    <div className="mt-2 space-y-2">
      {/* Image attachments as thumbnails */}
      {images.length > 0 && (
        <div className="flex flex-wrap gap-1">
          {images.map((image) => (
            <a
              key={image.id}
              href={image.url}
              target="_blank"
              rel="noopener noreferrer"
              className="block rounded overflow-hidden hover:opacity-90 transition-opacity"
            >
              <img
                src={image.url}
                alt={image.name}
                className="max-w-[200px] max-h-[150px] object-cover rounded"
              />
            </a>
          ))}
        </div>
      )}

      {/* File attachments as list */}
      {files.length > 0 && (
        <div className="space-y-1">
          {files.map((file) => (
            <a
              key={file.id}
              href={file.url}
              download={file.name}
              className="flex items-center gap-2 p-2 bg-white/10 rounded hover:bg-white/20 transition-colors text-sm"
            >
              {getFileIcon(file.type)}
              <span className="flex-1 truncate">{file.name}</span>
              <span className="text-xs opacity-70">{formatFileSize(file.size)}</span>
              <svg
                className="w-4 h-4 flex-shrink-0"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
                aria-hidden="true"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                />
              </svg>
            </a>
          ))}
        </div>
      )}
    </div>
  );
}
