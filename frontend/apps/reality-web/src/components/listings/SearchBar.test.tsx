/**
 * SearchBar Component Tests
 *
 * Tests for search bar with autocomplete (Epic 44, Story 44.2).
 */

import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { SearchBar } from './SearchBar';

// Mock the useSearchSuggestions hook
vi.mock('@ppt/reality-api-client', () => ({
  useSearchSuggestions: vi.fn(() => ({
    data: [
      { type: 'city', value: 'bratislava', label: 'Bratislava', count: 150 },
      { type: 'district', value: 'old-town', label: 'Old Town', count: 45 },
    ],
    isLoading: false,
  })),
}));

describe('SearchBar', () => {
  it('renders search input with placeholder', () => {
    render(<SearchBar onSearch={vi.fn()} />);
    // The mock returns the key name, so placeholder is just "placeholder"
    expect(screen.getByPlaceholderText('placeholder')).toBeInTheDocument();
  });

  it('renders search button', () => {
    render(<SearchBar onSearch={vi.fn()} />);
    // Button name from mock is just "button"
    expect(screen.getByRole('button', { name: 'button' })).toBeInTheDocument();
  });

  it('displays initial query value', () => {
    render(<SearchBar initialQuery="apartments" onSearch={vi.fn()} />);
    expect(screen.getByDisplayValue('apartments')).toBeInTheDocument();
  });

  it('calls onSearch when form is submitted', () => {
    const onSearch = vi.fn();
    render(<SearchBar onSearch={onSearch} />);

    const input = screen.getByPlaceholderText('placeholder');
    fireEvent.change(input, { target: { value: 'bratislava' } });

    const form = input.closest('form');
    fireEvent.submit(form!);

    expect(onSearch).toHaveBeenCalledWith('bratislava');
  });

  it('shows clear button when there is text in input', () => {
    render(<SearchBar initialQuery="test" onSearch={vi.fn()} />);
    expect(screen.getByRole('button', { name: /clear search/i })).toBeInTheDocument();
  });

  it('does not show clear button when input is empty', () => {
    render(<SearchBar onSearch={vi.fn()} />);
    expect(screen.queryByRole('button', { name: /clear search/i })).not.toBeInTheDocument();
  });

  it('clears input and calls onSearch when clear button is clicked', () => {
    const onSearch = vi.fn();
    render(<SearchBar initialQuery="test" onSearch={onSearch} />);

    const clearButton = screen.getByRole('button', { name: /clear search/i });
    fireEvent.click(clearButton);

    expect(screen.getByPlaceholderText('placeholder')).toHaveValue('');
    expect(onSearch).toHaveBeenCalledWith('');
  });

  it('updates query when typing in input', () => {
    render(<SearchBar onSearch={vi.fn()} />);

    const input = screen.getByPlaceholderText('placeholder');
    fireEvent.change(input, { target: { value: 'new search' } });

    expect(input).toHaveValue('new search');
  });

  it('shows suggestions dropdown on focus', () => {
    render(<SearchBar onSearch={vi.fn()} />);

    const input = screen.getByPlaceholderText('placeholder');
    fireEvent.focus(input);

    // Suggestions should appear after focus
    expect(screen.getByText('Bratislava')).toBeInTheDocument();
    expect(screen.getByText('Old Town')).toBeInTheDocument();
  });

  it('calls onSearch when city suggestion is clicked', () => {
    const onSearch = vi.fn();
    render(<SearchBar onSearch={onSearch} />);

    const input = screen.getByPlaceholderText('placeholder');
    fireEvent.focus(input);

    // Click on city suggestion
    const suggestionButton = screen.getByRole('button', { name: /Bratislava/i });
    fireEvent.click(suggestionButton);

    expect(onSearch).toHaveBeenCalledWith('bratislava');
  });
});
