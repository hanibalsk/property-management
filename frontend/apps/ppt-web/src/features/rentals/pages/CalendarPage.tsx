/**
 * CalendarPage - Full calendar view for a unit.
 * Epic 18: Short-Term Rental Integration (Story 18.2)
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { CalendarView } from '../components/CalendarView';
import type { CalendarEvent } from '../types';

interface Unit {
  id: string;
  name: string;
  buildingId: string;
  buildingName: string;
}

interface CalendarPageProps {
  units: Unit[];
  events: CalendarEvent[];
  selectedUnitId?: string;
  isLoading?: boolean;
  onUnitChange: (unitId: string) => void;
  onMonthChange: (date: Date) => void;
  onEventClick: (event: CalendarEvent) => void;
  onAddBlock: (unitId: string, startDate: Date, endDate: Date) => void;
  onBack: () => void;
}

export function CalendarPage({
  units,
  events,
  selectedUnitId,
  isLoading,
  onUnitChange,
  onMonthChange,
  onEventClick,
  onAddBlock,
  onBack,
}: CalendarPageProps) {
  const { t } = useTranslation();
  const [showBlockModal, setShowBlockModal] = useState(false);
  const [blockDates, setBlockDates] = useState<{ start: Date; end: Date } | null>(null);
  const [blockReason, setBlockReason] = useState('');

  const selectedUnit = units.find((u) => u.id === selectedUnitId);

  const handleAddBlock = (startDate: Date, endDate: Date) => {
    if (!selectedUnitId) return;
    setBlockDates({ start: startDate, end: endDate });
    setShowBlockModal(true);
  };

  const handleConfirmBlock = () => {
    if (!selectedUnitId || !blockDates) return;
    onAddBlock(selectedUnitId, blockDates.start, blockDates.end);
    setShowBlockModal(false);
    setBlockDates(null);
    setBlockReason('');
  };

  const formatDateRange = (start: Date, end: Date): string => {
    const options: Intl.DateTimeFormatOptions = {
      weekday: 'short',
      month: 'short',
      day: 'numeric',
    };
    return `${start.toLocaleDateString(undefined, options)} - ${end.toLocaleDateString(undefined, options)}`;
  };

  return (
    <div className="min-h-screen bg-gray-100">
      {/* Header */}
      <div className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <button type="button" onClick={onBack} className="text-gray-500 hover:text-gray-700">
                <svg
                  className="w-6 h-6"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                  aria-hidden="true"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M10 19l-7-7m0 0l7-7m-7 7h18"
                  />
                </svg>
              </button>
              <div>
                <h1 className="text-2xl font-bold text-gray-900">{t('rentals.calendar.title')}</h1>
                <p className="mt-1 text-sm text-gray-500">{t('rentals.calendar.subtitle')}</p>
              </div>
            </div>
            <div className="flex items-center gap-4">
              <div>
                <label htmlFor="unit-select" className="sr-only">
                  {t('rentals.calendar.selectUnit')}
                </label>
                <select
                  id="unit-select"
                  value={selectedUnitId || ''}
                  onChange={(e) => onUnitChange(e.target.value)}
                  className="px-4 py-2 border border-gray-300 rounded-md text-sm focus:ring-blue-500 focus:border-blue-500"
                >
                  <option value="">{t('rentals.calendar.selectUnit')}</option>
                  {units.map((unit) => (
                    <option key={unit.id} value={unit.id}>
                      {unit.name} - {unit.buildingName}
                    </option>
                  ))}
                </select>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Content */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {!selectedUnitId ? (
          <div className="bg-white rounded-lg shadow p-12 text-center">
            <svg
              className="w-16 h-16 mx-auto text-gray-400"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              aria-hidden="true"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
              />
            </svg>
            <h2 className="mt-4 text-lg font-medium text-gray-900">
              {t('rentals.calendar.selectUnitPrompt')}
            </h2>
            <p className="mt-2 text-sm text-gray-500">{t('rentals.calendar.selectUnitDesc')}</p>
          </div>
        ) : (
          <>
            {/* Unit Info */}
            {selectedUnit && (
              <div className="mb-6 flex items-center justify-between">
                <div>
                  <h2 className="text-lg font-semibold text-gray-900">{selectedUnit.name}</h2>
                  <p className="text-sm text-gray-500">{selectedUnit.buildingName}</p>
                </div>
                <div className="flex items-center gap-4 text-sm">
                  <div className="flex items-center gap-2">
                    <span className="w-3 h-3 rounded bg-blue-100 border border-blue-400" />
                    <span className="text-gray-600">
                      {events.filter((e) => e.type === 'booking').length}{' '}
                      {t('rentals.calendar.bookings')}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="w-3 h-3 rounded bg-gray-200 border border-gray-400" />
                    <span className="text-gray-600">
                      {events.filter((e) => e.type === 'block').length}{' '}
                      {t('rentals.calendar.blocks')}
                    </span>
                  </div>
                </div>
              </div>
            )}

            {/* Calendar */}
            <CalendarView
              events={events}
              isLoading={isLoading}
              onEventClick={onEventClick}
              onMonthChange={onMonthChange}
              onAddBlock={handleAddBlock}
            />

            {/* Instructions */}
            <div className="mt-6 bg-blue-50 border border-blue-200 rounded-lg p-4">
              <div className="flex items-start gap-3">
                <svg
                  className="w-5 h-5 text-blue-500 flex-shrink-0 mt-0.5"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                  aria-hidden="true"
                >
                  <path
                    fillRule="evenodd"
                    d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
                    clipRule="evenodd"
                  />
                </svg>
                <div>
                  <p className="text-sm font-medium text-blue-800">{t('rentals.calendar.tip')}</p>
                  <p className="mt-1 text-sm text-blue-700">{t('rentals.calendar.tipDesc')}</p>
                </div>
              </div>
            </div>
          </>
        )}
      </div>

      {/* Block Modal */}
      {showBlockModal && blockDates && (
        <div className="fixed inset-0 z-50 overflow-y-auto">
          <div className="flex min-h-full items-center justify-center p-4">
            <div
              className="fixed inset-0 bg-black bg-opacity-25"
              onClick={() => setShowBlockModal(false)}
            />
            <div className="relative bg-white rounded-lg shadow-xl max-w-md w-full p-6">
              <h3 className="text-lg font-semibold text-gray-900">
                {t('rentals.calendar.blockDates')}
              </h3>
              <p className="mt-2 text-sm text-gray-500">{t('rentals.calendar.blockDatesDesc')}</p>

              <div className="mt-4 p-3 bg-gray-50 rounded-lg">
                <p className="text-sm font-medium text-gray-900">
                  {formatDateRange(blockDates.start, blockDates.end)}
                </p>
                <p className="text-xs text-gray-500 mt-1">
                  {Math.ceil(
                    (blockDates.end.getTime() - blockDates.start.getTime()) / (1000 * 60 * 60 * 24)
                  ) + 1}{' '}
                  {t('rentals.calendar.days')}
                </p>
              </div>

              <div className="mt-4">
                <label htmlFor="block-reason" className="block text-sm font-medium text-gray-700">
                  {t('rentals.calendar.reason')} ({t('common.optional')})
                </label>
                <textarea
                  id="block-reason"
                  value={blockReason}
                  onChange={(e) => setBlockReason(e.target.value)}
                  rows={3}
                  placeholder={t('rentals.calendar.reasonPlaceholder')}
                  className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                />
              </div>

              <div className="mt-6 flex items-center justify-end gap-3">
                <button
                  type="button"
                  onClick={() => setShowBlockModal(false)}
                  className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
                >
                  {t('common.cancel')}
                </button>
                <button
                  type="button"
                  onClick={handleConfirmBlock}
                  className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700"
                >
                  {t('rentals.calendar.confirmBlock')}
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
