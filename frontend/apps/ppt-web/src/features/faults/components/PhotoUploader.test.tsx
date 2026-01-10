/**
 * PhotoUploader component tests (Epic 126, Story 126.1).
 */

import { fireEvent, render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PhotoUploader, type UploadedPhoto } from './PhotoUploader';

// Mock URL.createObjectURL and revokeObjectURL for jsdom
const mockCreateObjectURL = vi.fn(() => 'blob:mock-url');
const mockRevokeObjectURL = vi.fn();
URL.createObjectURL = mockCreateObjectURL;
URL.revokeObjectURL = mockRevokeObjectURL;

// Mock translations
vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string, params?: Record<string, unknown>) => {
      const translations: Record<string, string> = {
        'faults.photo.addPhotos': 'Add Photos',
        'faults.photo.dragOrClick': 'Drag photos here or click to browse',
        'faults.photo.maxPhotos': `${params?.max} more photo(s) allowed`,
        'faults.photo.selectPhotos': 'Select photos to upload',
        'faults.photo.dropZoneLabel': 'Photo upload area',
        'faults.photo.uploadedPhotos': 'Uploaded photos',
        'faults.photo.photoAlt': `Uploaded photo: ${params?.name}`,
        'faults.photo.removePhoto': `Remove photo ${params?.name}`,
        'faults.photo.invalidType': 'Only image files are allowed',
        'faults.photo.tooLarge': `File size must be less than ${params?.maxSize}MB`,
        'faults.photo.acceptedFormats': 'JPEG, PNG, WebP, HEIC',
        'faults.photo.maxSize': `Max ${params?.size}MB per file`,
      };
      return translations[key] || key;
    },
  }),
}));

describe('PhotoUploader', () => {
  const mockOnPhotosChange = vi.fn();

  beforeEach(() => {
    mockOnPhotosChange.mockClear();
  });

  it('renders drop zone when no photos are uploaded', () => {
    render(<PhotoUploader photos={[]} onPhotosChange={mockOnPhotosChange} />);

    expect(screen.getByText('Add Photos')).toBeInTheDocument();
    expect(screen.getByText('Drag photos here or click to browse')).toBeInTheDocument();
  });

  it('shows remaining photo count', () => {
    render(<PhotoUploader photos={[]} onPhotosChange={mockOnPhotosChange} maxPhotos={5} />);

    expect(screen.getByText('5 more photo(s) allowed')).toBeInTheDocument();
  });

  it('hides drop zone when max photos reached', () => {
    const photos: UploadedPhoto[] = Array.from({ length: 5 }, (_, i) => ({
      id: `photo-${i}`,
      file: new File([''], `test${i}.jpg`, { type: 'image/jpeg' }),
      preview: `blob:test${i}`,
      status: 'uploaded',
    }));

    render(<PhotoUploader photos={photos} onPhotosChange={mockOnPhotosChange} maxPhotos={5} />);

    expect(screen.queryByText('Add Photos')).not.toBeInTheDocument();
  });

  it('displays uploaded photos in grid', () => {
    const photos: UploadedPhoto[] = [
      {
        id: 'photo-1',
        file: new File([''], 'test.jpg', { type: 'image/jpeg' }),
        preview: 'blob:test',
        status: 'uploaded',
      },
    ];

    render(<PhotoUploader photos={photos} onPhotosChange={mockOnPhotosChange} />);

    expect(screen.getByRole('list', { name: 'Uploaded photos' })).toBeInTheDocument();
    expect(screen.getByAltText('Uploaded photo: test.jpg')).toBeInTheDocument();
  });

  it('calls onPhotosChange when remove button is clicked', () => {
    const photos: UploadedPhoto[] = [
      {
        id: 'photo-1',
        file: new File([''], 'test.jpg', { type: 'image/jpeg' }),
        preview: 'blob:test',
        status: 'uploaded',
      },
    ];

    render(<PhotoUploader photos={photos} onPhotosChange={mockOnPhotosChange} />);

    const removeButton = screen.getByLabelText('Remove photo test.jpg');
    fireEvent.click(removeButton);

    expect(mockOnPhotosChange).toHaveBeenCalledWith([]);
  });

  it('shows loading state for uploading photos', () => {
    const photos: UploadedPhoto[] = [
      {
        id: 'photo-1',
        file: new File([''], 'test.jpg', { type: 'image/jpeg' }),
        preview: 'blob:test',
        status: 'uploading',
      },
    ];

    render(<PhotoUploader photos={photos} onPhotosChange={mockOnPhotosChange} />);

    // Check for spinner (animate-spin class)
    const spinner = document.querySelector('.animate-spin');
    expect(spinner).toBeInTheDocument();
  });

  it('shows error state for failed photos', () => {
    const photos: UploadedPhoto[] = [
      {
        id: 'photo-1',
        file: new File([''], 'test.jpg', { type: 'image/jpeg' }),
        preview: 'blob:test',
        status: 'error',
        error: 'Upload failed',
      },
    ];

    render(<PhotoUploader photos={photos} onPhotosChange={mockOnPhotosChange} />);

    expect(screen.getByText('Upload failed')).toBeInTheDocument();
  });

  it('disables interactions when disabled prop is true', () => {
    render(<PhotoUploader photos={[]} onPhotosChange={mockOnPhotosChange} disabled />);

    const dropZone = screen.getByRole('button', { name: 'Photo upload area' });
    expect(dropZone).toHaveAttribute('tabIndex', '-1');
  });

  it('accepts files via file input', () => {
    render(<PhotoUploader photos={[]} onPhotosChange={mockOnPhotosChange} />);

    const input = document.querySelector('input[type="file"]') as HTMLInputElement;
    expect(input).toBeInTheDocument();
    expect(input).toHaveAttribute('accept', 'image/jpeg,image/png,image/webp,image/heic');
    expect(input).toHaveAttribute('multiple');
  });

  it('validates file size', () => {
    render(
      <PhotoUploader photos={[]} onPhotosChange={mockOnPhotosChange} maxFileSize={1024 * 1024} />
    );

    const input = document.querySelector('input[type="file"]') as HTMLInputElement;

    // Create a file larger than 1MB
    const largeFile = new File(['x'.repeat(2 * 1024 * 1024)], 'large.jpg', { type: 'image/jpeg' });

    // Simulate file selection
    Object.defineProperty(input, 'files', { value: [largeFile] });
    fireEvent.change(input);

    // Should be called with error status
    expect(mockOnPhotosChange).toHaveBeenCalled();
    const addedPhotos = mockOnPhotosChange.mock.calls[0][0];
    expect(addedPhotos[0].status).toBe('error');
  });
});
