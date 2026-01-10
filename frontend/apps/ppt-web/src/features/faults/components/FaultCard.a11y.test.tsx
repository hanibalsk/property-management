/// <reference types="vitest/globals" />
/**
 * FaultCard Accessibility Tests (Epic 125, Story 125.2)
 *
 * Tests WCAG 2.1 AA compliance for the FaultCard component including:
 * - Semantic structure
 * - Color contrast (via axe-core)
 * - Button accessibility
 * - Icon accessibility (aria-label, title)
 */

import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { axe } from 'vitest-axe';
import { FaultCard, type FaultSummary } from './FaultCard';

const mockFault: FaultSummary = {
  id: 'fault-1',
  buildingId: 'building-1',
  title: 'Water leak in bathroom',
  category: 'plumbing',
  priority: 'high',
  status: 'new',
  createdAt: '2024-01-15T10:00:00Z',
};

describe('FaultCard Accessibility', () => {
  it('should have no accessibility violations with standard fault', async () => {
    const { container } = render(<FaultCard fault={mockFault} />);

    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it('should have no accessibility violations with urgent priority', async () => {
    const urgentFault: FaultSummary = {
      ...mockFault,
      priority: 'urgent',
    };
    const { container } = render(<FaultCard fault={urgentFault} />);

    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it('should have accessible urgent icon with title', () => {
    const urgentFault: FaultSummary = {
      ...mockFault,
      priority: 'urgent',
    };
    render(<FaultCard fault={urgentFault} />);

    // SVG should have accessible title
    const svg = document.querySelector('svg');
    expect(svg).toHaveAttribute('aria-label', 'Urgent');
    expect(svg?.querySelector('title')?.textContent).toBe('Urgent');
  });

  it('should have accessible title as heading', () => {
    render(<FaultCard fault={mockFault} />);

    expect(screen.getByRole('heading', { level: 3 })).toHaveTextContent('Water leak in bathroom');
  });

  it('action buttons should be keyboard accessible', async () => {
    const user = userEvent.setup();
    const onView = vi.fn();
    const onEdit = vi.fn();
    const onTriage = vi.fn();

    render(<FaultCard fault={mockFault} onView={onView} onEdit={onEdit} onTriage={onTriage} />);

    // Tab to each button and press Enter
    await user.tab();
    expect(screen.getByText('View')).toHaveFocus();
    await user.keyboard('{Enter}');
    expect(onView).toHaveBeenCalledWith('fault-1');

    await user.tab();
    expect(screen.getByText('Edit')).toHaveFocus();
    await user.keyboard('{Enter}');
    expect(onEdit).toHaveBeenCalledWith('fault-1');

    await user.tab();
    expect(screen.getByText('Triage')).toHaveFocus();
    await user.keyboard('{Enter}');
    expect(onTriage).toHaveBeenCalledWith('fault-1');
  });

  it('should display status with correct label', () => {
    render(<FaultCard fault={mockFault} />);

    expect(screen.getByText('New')).toBeInTheDocument();
  });

  it('should display priority with correct label', () => {
    render(<FaultCard fault={mockFault} />);

    expect(screen.getByText('High')).toBeInTheDocument();
  });

  it('should display category with correct label', () => {
    render(<FaultCard fault={mockFault} />);

    expect(screen.getByText('Plumbing')).toBeInTheDocument();
  });

  it('should have no violations for resolved fault (fewer buttons)', async () => {
    const resolvedFault: FaultSummary = {
      ...mockFault,
      status: 'resolved',
    };
    const { container } = render(<FaultCard fault={resolvedFault} />);

    const results = await axe(container);
    expect(results).toHaveNoViolations();
  });

  it('should have no violations for all status types', async () => {
    const statuses: FaultSummary['status'][] = [
      'new',
      'triaged',
      'in_progress',
      'waiting_parts',
      'scheduled',
      'resolved',
      'closed',
      'reopened',
    ];

    for (const status of statuses) {
      const fault: FaultSummary = { ...mockFault, status };
      const { container, unmount } = render(<FaultCard fault={fault} />);

      const results = await axe(container);
      expect(results).toHaveNoViolations();

      unmount();
    }
  });
});
