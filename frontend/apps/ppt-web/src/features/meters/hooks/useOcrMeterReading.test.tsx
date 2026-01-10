/**
 * OCR Meter Reading Hook Tests
 * Epic 128: OCR Meter Preview
 */

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { renderHook, waitFor } from '@testing-library/react';
import type { ReactNode } from 'react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { useOcrCorrection, useOcrMeterReading, useOcrProcessImage } from './useOcrMeterReading';

// Mock fetch
const mockFetch = vi.fn();
global.fetch = mockFetch;

// Create wrapper with QueryClient
function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: { retry: false },
      mutations: { retry: false },
    },
  });

  return ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  );
}

describe('useOcrProcessImage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  it('successfully processes an image', async () => {
    const mockResult = {
      result: {
        extractedValue: 12345,
        confidence: 0.95,
        boundingBox: { x: 10, y: 20, width: 30, height: 15 },
        rawText: '12345',
        processingTimeMs: 250,
      },
      imageUrl: 'https://example.com/processed.jpg',
    };

    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: () => Promise.resolve(mockResult),
    });

    const { result } = renderHook(() => useOcrProcessImage(), {
      wrapper: createWrapper(),
    });

    const file = new File(['test'], 'meter.jpg', { type: 'image/jpeg' });
    result.current.mutate({ image: file, meterId: 'meter-123' });

    await waitFor(() => expect(result.current.isSuccess).toBe(true));

    expect(result.current.data).toEqual(mockResult);
    expect(mockFetch).toHaveBeenCalledWith(
      '/api/v1/ai/ocr/meter-reading',
      expect.objectContaining({
        method: 'POST',
        body: expect.any(FormData),
      })
    );
  });

  it('handles processing errors', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 500,
      json: () => Promise.resolve({ message: 'Processing failed' }),
    });

    const { result } = renderHook(() => useOcrProcessImage(), {
      wrapper: createWrapper(),
    });

    const file = new File(['test'], 'meter.jpg', { type: 'image/jpeg' });
    result.current.mutate({ image: file, meterId: 'meter-123' });

    await waitFor(() => expect(result.current.isError).toBe(true));

    expect(result.current.error?.message).toBe('Processing failed');
  });

  it('handles network errors', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 500,
      json: () => Promise.reject(new Error('Network error')),
    });

    const { result } = renderHook(() => useOcrProcessImage(), {
      wrapper: createWrapper(),
    });

    const file = new File(['test'], 'meter.jpg', { type: 'image/jpeg' });
    result.current.mutate({ image: file, meterId: 'meter-123' });

    await waitFor(() => expect(result.current.isError).toBe(true));

    expect(result.current.error?.message).toBe('OCR request failed');
  });
});

describe('useOcrCorrection', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  it('successfully submits a correction', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: () => Promise.resolve({}),
    });

    const { result } = renderHook(() => useOcrCorrection(), {
      wrapper: createWrapper(),
    });

    result.current.mutate({
      originalValue: 12345,
      correctedValue: 12400,
      imageUrl: 'https://example.com/meter.jpg',
      boundingBox: { x: 10, y: 20, width: 30, height: 15 },
      timestamp: '2024-01-15T10:30:00Z',
    });

    await waitFor(() => expect(result.current.isSuccess).toBe(true));

    expect(mockFetch).toHaveBeenCalledWith('/api/v1/ai/ocr/correction', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        original_value: 12345,
        corrected_value: 12400,
        image_url: 'https://example.com/meter.jpg',
        bounding_box: { x: 10, y: 20, width: 30, height: 15 },
        timestamp: '2024-01-15T10:30:00Z',
      }),
    });
  });

  it('handles correction submission errors', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 400,
      json: () => Promise.resolve({ message: 'Invalid correction data' }),
    });

    const { result } = renderHook(() => useOcrCorrection(), {
      wrapper: createWrapper(),
    });

    result.current.mutate({
      originalValue: 12345,
      correctedValue: 12400,
      imageUrl: 'https://example.com/meter.jpg',
      timestamp: '2024-01-15T10:30:00Z',
    });

    await waitFor(() => expect(result.current.isError).toBe(true));

    expect(result.current.error?.message).toBe('Invalid correction data');
  });
});

describe('useOcrMeterReading', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  it('provides process and correctAndSubmit functions', () => {
    const { result } = renderHook(() => useOcrMeterReading('meter-123'), {
      wrapper: createWrapper(),
    });

    expect(result.current.process).toBeDefined();
    expect(typeof result.current.process).toBe('function');
    expect(result.current.correctAndSubmit).toBeDefined();
    expect(typeof result.current.correctAndSubmit).toBe('function');
  });

  it('provides loading states', () => {
    const { result } = renderHook(() => useOcrMeterReading('meter-123'), {
      wrapper: createWrapper(),
    });

    expect(result.current.isProcessing).toBe(false);
    expect(result.current.isSubmittingCorrection).toBe(false);
  });

  it('provides error states', () => {
    const { result } = renderHook(() => useOcrMeterReading('meter-123'), {
      wrapper: createWrapper(),
    });

    expect(result.current.processingError).toBeNull();
    expect(result.current.correctionError).toBeNull();
  });

  it('provides reset function', () => {
    const { result } = renderHook(() => useOcrMeterReading('meter-123'), {
      wrapper: createWrapper(),
    });

    expect(result.current.reset).toBeDefined();
    expect(typeof result.current.reset).toBe('function');
  });

  it('processes an image through the hook', async () => {
    const mockResult = {
      result: {
        extractedValue: 12345,
        confidence: 0.95,
        boundingBox: { x: 10, y: 20, width: 30, height: 15 },
        rawText: '12345',
        processingTimeMs: 250,
      },
      imageUrl: 'https://example.com/processed.jpg',
    };

    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: () => Promise.resolve(mockResult),
    });

    const { result } = renderHook(() => useOcrMeterReading('meter-123'), {
      wrapper: createWrapper(),
    });

    const file = new File(['test'], 'meter.jpg', { type: 'image/jpeg' });
    const processResult = await result.current.process(file);

    expect(processResult).toEqual(mockResult);
  });

  it('submits a correction through the hook', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: () => Promise.resolve({}),
    });

    const { result } = renderHook(() => useOcrMeterReading('meter-123'), {
      wrapper: createWrapper(),
    });

    await result.current.correctAndSubmit({
      originalValue: 12345,
      correctedValue: 12400,
      imageUrl: 'https://example.com/meter.jpg',
      timestamp: '2024-01-15T10:30:00Z',
    });

    expect(mockFetch).toHaveBeenCalledWith('/api/v1/ai/ocr/correction', expect.any(Object));
  });
});
