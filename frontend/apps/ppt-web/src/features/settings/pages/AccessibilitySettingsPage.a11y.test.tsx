/**
 * Accessibility tests for AccessibilitySettingsPage (Epic 125, Story 125.4).
 *
 * Tests high contrast theme, text size adjustments, and accessibility settings.
 */

/// <reference types="vitest/globals" />
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { axe } from 'vitest-axe';
import { AccessibilitySettingsPage } from './AccessibilitySettingsPage';

// Mock the accessibility context
const mockUpdateSettings = vi.fn();
const mockSettings = {
  colorScheme: 'light' as const,
  textSize: 'medium' as const,
  reduceMotion: false,
  screenReaderEnabled: false,
  keyboardNavigationEnabled: true,
};

vi.mock('@ppt/ui-kit', () => ({
  useAccessibilityContext: () => ({
    settings: mockSettings,
    updateSettings: mockUpdateSettings,
  }),
}));

describe('AccessibilitySettingsPage accessibility', () => {
  beforeEach(() => {
    mockUpdateSettings.mockClear();
  });

  it('should have no accessibility violations in default state', async () => {
    const { container } = render(<AccessibilitySettingsPage />);

    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it('should have proper heading hierarchy', () => {
    render(<AccessibilitySettingsPage />);

    // Main heading
    const h1 = screen.getByRole('heading', { level: 1, name: /accessibility settings/i });
    expect(h1).toBeInTheDocument();

    // Section headings
    const h2s = screen.getAllByRole('heading', { level: 2 });
    expect(h2s.length).toBeGreaterThanOrEqual(3);
    expect(h2s.some((h) => h.textContent?.includes('Visual'))).toBe(true);
    expect(h2s.some((h) => h.textContent?.includes('Motion'))).toBe(true);
    expect(h2s.some((h) => h.textContent?.includes('Navigation'))).toBe(true);
  });

  it('should have labeled form controls', () => {
    render(<AccessibilitySettingsPage />);

    // Color scheme select
    const colorSchemeSelect = screen.getByLabelText(/color scheme/i);
    expect(colorSchemeSelect).toBeInTheDocument();
    expect(colorSchemeSelect.tagName).toBe('SELECT');

    // Text size select
    const textSizeSelect = screen.getByLabelText(/text size/i);
    expect(textSizeSelect).toBeInTheDocument();
    expect(textSizeSelect.tagName).toBe('SELECT');

    // Checkboxes
    const reduceMotionCheckbox = screen.getByLabelText(/reduce motion/i);
    expect(reduceMotionCheckbox).toBeInTheDocument();

    const screenReaderCheckbox = screen.getByLabelText(/screen reader/i);
    expect(screenReaderCheckbox).toBeInTheDocument();

    const keyboardNavCheckbox = screen.getByLabelText(/keyboard navigation/i);
    expect(keyboardNavCheckbox).toBeInTheDocument();
  });

  it('should have aria-describedby for form controls', () => {
    render(<AccessibilitySettingsPage />);

    const colorSchemeSelect = screen.getByLabelText(/color scheme/i);
    expect(colorSchemeSelect).toHaveAttribute('aria-describedby');

    const textSizeSelect = screen.getByLabelText(/text size/i);
    expect(textSizeSelect).toHaveAttribute('aria-describedby');
  });

  it('should include high contrast option in color scheme', () => {
    render(<AccessibilitySettingsPage />);

    const colorSchemeSelect = screen.getByLabelText(/color scheme/i);
    const options = colorSchemeSelect.querySelectorAll('option');

    const optionTexts = Array.from(options).map((opt) => opt.textContent);
    expect(optionTexts).toContain('High Contrast');
  });

  it('should update settings when color scheme changed', async () => {
    const user = userEvent.setup();
    render(<AccessibilitySettingsPage />);

    const colorSchemeSelect = screen.getByLabelText(/color scheme/i);
    await user.selectOptions(colorSchemeSelect, 'high-contrast');

    expect(mockUpdateSettings).toHaveBeenCalledWith({ colorScheme: 'high-contrast' });
  });

  it('should update settings when text size changed', async () => {
    const user = userEvent.setup();
    render(<AccessibilitySettingsPage />);

    const textSizeSelect = screen.getByLabelText(/text size/i);
    await user.selectOptions(textSizeSelect, 'large');

    expect(mockUpdateSettings).toHaveBeenCalledWith({ textSize: 'large' });
  });

  it('should toggle reduce motion setting', async () => {
    const user = userEvent.setup();
    render(<AccessibilitySettingsPage />);

    const checkbox = screen.getByLabelText(/reduce motion/i);
    await user.click(checkbox);

    expect(mockUpdateSettings).toHaveBeenCalledWith({ reduceMotion: true });
  });

  it('should have keyboard accessible controls', async () => {
    const user = userEvent.setup();
    render(<AccessibilitySettingsPage />);

    // Tab to first control
    await user.tab();

    // Should be able to navigate with tab
    await waitFor(() => {
      const activeElement = document.activeElement;
      expect(
        activeElement?.tagName === 'SELECT' ||
          activeElement?.tagName === 'INPUT' ||
          activeElement?.tagName === 'BUTTON'
      ).toBe(true);
    });
  });

  it('should have proper sections with aria-label', () => {
    render(<AccessibilitySettingsPage />);

    // Preview section
    const previewSection = screen.getByRole('region', { name: /preview/i });
    expect(previewSection).toBeInTheDocument();

    // Additional info section
    const infoSection = screen.getByRole('region', { name: /additional information/i });
    expect(infoSection).toBeInTheDocument();
  });

  it('should have sample button in preview area', () => {
    render(<AccessibilitySettingsPage />);

    const sampleButton = screen.getByRole('button', { name: /sample button/i });
    expect(sampleButton).toBeInTheDocument();
  });

  it('should list keyboard shortcuts', () => {
    const { container } = render(<AccessibilitySettingsPage />);

    // Check for keyboard shortcut instructions using kbd elements
    const kbdElements = container.querySelectorAll('kbd');
    const kbdTexts = Array.from(kbdElements).map((el) => el.textContent);

    expect(kbdTexts).toContain('Tab');
    expect(kbdTexts).toContain('Shift + Tab');
    expect(kbdTexts.some((t) => t?.includes('Enter'))).toBe(true);
    expect(kbdTexts).toContain('Esc');
  });
});
