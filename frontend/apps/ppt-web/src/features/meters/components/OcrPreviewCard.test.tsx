/**
 * OcrPreviewCard component tests
 * Epic 128: OCR Meter Preview
 */

/// <reference types="vitest/globals" />
import { fireEvent, render, screen } from '@testing-library/react';
import { OcrPreviewCard, type OcrResult } from './OcrPreviewCard';

// Mock useTranslation
vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string, params?: Record<string, unknown>) => {
      const translations: Record<string, string> = {
        'meters.ocr.previewTitle': 'OCR Reading Preview',
        'meters.ocr.confidenceHigh': 'High confidence',
        'meters.ocr.confidenceMedium': 'Medium confidence',
        'meters.ocr.confidenceLow': 'Low confidence',
        'meters.ocr.extractedValue': 'Extracted Value',
        'meters.ocr.detectedArea': 'Detected',
        'meters.ocr.meterPhoto': 'Meter photo',
        'meters.ocr.acceptValue': 'Accept Value',
        'meters.ocr.acceptCorrected': 'Accept Corrected Value',
        'meters.ocr.correctValue': 'Correct Value',
        'meters.ocr.correctedValue': 'Corrected value',
        'meters.ocr.retakePhoto': 'Retake Photo',
        'meters.ocr.unusualChange': 'Unusual Reading',
        'meters.ocr.valueLowerThanLast': 'Value is lower than last reading',
        'meters.ocr.unusuallyHighChange': 'Consumption seems unusually high',
        'meters.ocr.lastReading': 'Last reading',
        'meters.ocr.consumption': 'Consumption',
        'meters.ocr.processingTime': `Processed in ${params?.ms}ms`,
        'common.cancel': 'Cancel',
      };
      return translations[key] || key;
    },
  }),
}));

const mockOcrResult: OcrResult = {
  extractedValue: 12345,
  confidence: 0.95,
  boundingBox: {
    x: 10,
    y: 20,
    width: 30,
    height: 15,
  },
  rawText: '12345',
  processingTimeMs: 250,
};

const defaultProps = {
  imageUrl: 'https://example.com/meter.jpg',
  ocrResult: mockOcrResult,
  meterUnit: 'kWh',
  onAccept: vi.fn(),
  onCorrect: vi.fn(),
  onRetake: vi.fn(),
};

describe('OcrPreviewCard', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('rendering', () => {
    it('renders the preview title', () => {
      render(<OcrPreviewCard {...defaultProps} />);
      expect(screen.getByText('OCR Reading Preview')).toBeInTheDocument();
    });

    it('renders the extracted value with unit', () => {
      render(<OcrPreviewCard {...defaultProps} />);
      expect(screen.getByText(/12,345/)).toBeInTheDocument();
      expect(screen.getByText(/kWh/)).toBeInTheDocument();
    });

    it('renders the meter image', () => {
      render(<OcrPreviewCard {...defaultProps} />);
      const img = screen.getByAltText('Meter photo');
      expect(img).toHaveAttribute('src', 'https://example.com/meter.jpg');
    });

    it('renders processing time', () => {
      render(<OcrPreviewCard {...defaultProps} />);
      expect(screen.getByText('Processed in 250ms')).toBeInTheDocument();
    });

    it('renders action buttons', () => {
      render(<OcrPreviewCard {...defaultProps} />);
      expect(screen.getByText('Accept Value')).toBeInTheDocument();
      expect(screen.getByText('Correct Value')).toBeInTheDocument();
      expect(screen.getByText('Retake Photo')).toBeInTheDocument();
    });
  });

  describe('confidence indicators', () => {
    it('shows high confidence for >= 90%', () => {
      render(
        <OcrPreviewCard {...defaultProps} ocrResult={{ ...mockOcrResult, confidence: 0.95 }} />
      );
      expect(screen.getByText(/High confidence/)).toBeInTheDocument();
      expect(screen.getByText(/95%/)).toBeInTheDocument();
    });

    it('shows medium confidence for >= 70% and < 90%', () => {
      render(
        <OcrPreviewCard {...defaultProps} ocrResult={{ ...mockOcrResult, confidence: 0.75 }} />
      );
      expect(screen.getByText(/Medium confidence/)).toBeInTheDocument();
      expect(screen.getByText(/75%/)).toBeInTheDocument();
    });

    it('shows low confidence for < 70%', () => {
      render(
        <OcrPreviewCard {...defaultProps} ocrResult={{ ...mockOcrResult, confidence: 0.5 }} />
      );
      expect(screen.getByText(/Low confidence/)).toBeInTheDocument();
      expect(screen.getByText(/50%/)).toBeInTheDocument();
    });
  });

  describe('last reading comparison', () => {
    it('shows consumption when value is higher than last reading', () => {
      render(<OcrPreviewCard {...defaultProps} lastReadingValue={12000} />);
      expect(screen.getByText(/Last reading/)).toBeInTheDocument();
      expect(screen.getByText(/12,000/)).toBeInTheDocument();
      expect(screen.getByText(/\+345/)).toBeInTheDocument();
    });

    it('shows warning when value is lower than last reading', () => {
      render(<OcrPreviewCard {...defaultProps} lastReadingValue={13000} />);
      expect(screen.getByText('Unusual Reading')).toBeInTheDocument();
      expect(screen.getByText('Value is lower than last reading')).toBeInTheDocument();
    });

    it('shows warning when consumption is unusually high', () => {
      render(<OcrPreviewCard {...defaultProps} lastReadingValue={5000} />);
      expect(screen.getByText('Unusual Reading')).toBeInTheDocument();
      expect(screen.getByText('Consumption seems unusually high')).toBeInTheDocument();
    });

    it('does not show comparison when no last reading provided', () => {
      render(<OcrPreviewCard {...defaultProps} />);
      expect(screen.queryByText(/Last reading/)).not.toBeInTheDocument();
    });
  });

  describe('accept functionality', () => {
    it('calls onAccept with extracted value when clicking accept', () => {
      const onAccept = vi.fn();
      render(<OcrPreviewCard {...defaultProps} onAccept={onAccept} />);

      fireEvent.click(screen.getByText('Accept Value'));
      expect(onAccept).toHaveBeenCalledWith(12345);
    });

    it('disables accept button when loading', () => {
      render(<OcrPreviewCard {...defaultProps} isLoading />);
      expect(screen.getByText('Accept Value').closest('button')).toBeDisabled();
    });
  });

  describe('correction functionality', () => {
    it('enters edit mode when clicking correct', () => {
      render(<OcrPreviewCard {...defaultProps} />);

      fireEvent.click(screen.getByText('Correct Value'));

      expect(screen.getByRole('spinbutton')).toBeInTheDocument();
      expect(screen.getByText('Cancel')).toBeInTheDocument();
    });

    it('shows Accept Corrected Value button when editing', () => {
      render(<OcrPreviewCard {...defaultProps} />);

      fireEvent.click(screen.getByText('Correct Value'));

      expect(screen.getByText('Accept Corrected Value')).toBeInTheDocument();
    });

    it('allows editing the value', () => {
      render(<OcrPreviewCard {...defaultProps} />);

      fireEvent.click(screen.getByText('Correct Value'));
      const input = screen.getByRole('spinbutton');

      fireEvent.change(input, { target: { value: '12400' } });

      expect(input).toHaveValue(12400);
    });

    it('calls onCorrect and onAccept with corrected value', () => {
      const onCorrect = vi.fn();
      const onAccept = vi.fn();
      render(<OcrPreviewCard {...defaultProps} onCorrect={onCorrect} onAccept={onAccept} />);

      fireEvent.click(screen.getByText('Correct Value'));
      const input = screen.getByRole('spinbutton');
      fireEvent.change(input, { target: { value: '12400' } });
      fireEvent.click(screen.getByText('Accept Corrected Value'));

      expect(onCorrect).toHaveBeenCalledWith(
        expect.objectContaining({
          originalValue: 12345,
          correctedValue: 12400,
          imageUrl: 'https://example.com/meter.jpg',
          boundingBox: mockOcrResult.boundingBox,
        })
      );
      expect(onAccept).toHaveBeenCalledWith(12400);
    });

    it('calls only onAccept if value unchanged in edit mode', () => {
      const onCorrect = vi.fn();
      const onAccept = vi.fn();
      render(<OcrPreviewCard {...defaultProps} onCorrect={onCorrect} onAccept={onAccept} />);

      fireEvent.click(screen.getByText('Correct Value'));
      // Value unchanged (still 12345)
      fireEvent.click(screen.getByText('Accept Corrected Value'));

      expect(onCorrect).not.toHaveBeenCalled();
      expect(onAccept).toHaveBeenCalledWith(12345);
    });

    it('exits edit mode when clicking cancel', () => {
      render(<OcrPreviewCard {...defaultProps} />);

      fireEvent.click(screen.getByText('Correct Value'));
      expect(screen.getByRole('spinbutton')).toBeInTheDocument();

      fireEvent.click(screen.getByText('Cancel'));
      expect(screen.queryByRole('spinbutton')).not.toBeInTheDocument();
    });
  });

  describe('retake functionality', () => {
    it('calls onRetake when clicking retake', () => {
      const onRetake = vi.fn();
      render(<OcrPreviewCard {...defaultProps} onRetake={onRetake} />);

      fireEvent.click(screen.getByText('Retake Photo'));
      expect(onRetake).toHaveBeenCalled();
    });

    it('disables retake button when loading', () => {
      render(<OcrPreviewCard {...defaultProps} isLoading />);
      expect(screen.getByText('Retake Photo').closest('button')).toBeDisabled();
    });
  });

  describe('bounding box', () => {
    it('renders bounding box overlay', () => {
      render(<OcrPreviewCard {...defaultProps} />);

      // Check for the "Detected" label inside bounding box
      expect(screen.getByText('Detected')).toBeInTheDocument();
    });
  });

  describe('loading state', () => {
    it('shows spinner when loading', () => {
      render(<OcrPreviewCard {...defaultProps} isLoading />);

      // Check for spinner animation class
      const spinner = document.querySelector('.animate-spin');
      expect(spinner).toBeInTheDocument();
    });

    it('disables all action buttons when loading', () => {
      render(<OcrPreviewCard {...defaultProps} isLoading />);

      const buttons = screen.getAllByRole('button');
      for (const button of buttons) {
        expect(button).toBeDisabled();
      }
    });
  });
});
