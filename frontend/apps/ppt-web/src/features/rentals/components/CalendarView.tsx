/**
 * CalendarView Component
 *
 * Displays a calendar showing bookings and blocks.
 * Epic 18: Short-Term Rental Integration (Story 18.2)
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { CalendarEvent, CalendarEventType } from '../types';

interface CalendarViewProps {
  events: CalendarEvent[];
  currentMonth?: Date;
  isLoading?: boolean;
  onDateSelect?: (date: Date) => void;
  onEventClick?: (event: CalendarEvent) => void;
  onMonthChange?: (date: Date) => void;
  onAddBlock?: (startDate: Date, endDate: Date) => void;
}

const eventTypeColors: Record<CalendarEventType, { bg: string; border: string; text: string }> = {
  booking: { bg: 'bg-blue-100', border: 'border-blue-400', text: 'text-blue-800' },
  block: { bg: 'bg-gray-200', border: 'border-gray-400', text: 'text-gray-700' },
  maintenance: { bg: 'bg-orange-100', border: 'border-orange-400', text: 'text-orange-800' },
};

function getDaysInMonth(year: number, month: number): Date[] {
  const days: Date[] = [];
  const firstDay = new Date(year, month, 1);
  const lastDay = new Date(year, month + 1, 0);

  // Add days from previous month to fill the first week
  const startDayOfWeek = firstDay.getDay();
  const prevMonthLastDay = new Date(year, month, 0).getDate();
  for (let i = startDayOfWeek - 1; i >= 0; i--) {
    days.push(new Date(year, month - 1, prevMonthLastDay - i));
  }

  // Add days of current month
  for (let i = 1; i <= lastDay.getDate(); i++) {
    days.push(new Date(year, month, i));
  }

  // Add days from next month to fill the last week
  const remainingDays = 7 - (days.length % 7);
  if (remainingDays < 7) {
    for (let i = 1; i <= remainingDays; i++) {
      days.push(new Date(year, month + 1, i));
    }
  }

  return days;
}

function isSameDay(d1: Date, d2: Date): boolean {
  return (
    d1.getFullYear() === d2.getFullYear() &&
    d1.getMonth() === d2.getMonth() &&
    d1.getDate() === d2.getDate()
  );
}

function isDateInRange(date: Date, start: Date, end: Date): boolean {
  const dateTime = date.getTime();
  const startTime = new Date(start.getFullYear(), start.getMonth(), start.getDate()).getTime();
  const endTime = new Date(end.getFullYear(), end.getMonth(), end.getDate()).getTime();
  return dateTime >= startTime && dateTime <= endTime;
}

export function CalendarView({
  events,
  currentMonth = new Date(),
  isLoading,
  onDateSelect,
  onEventClick,
  onMonthChange,
  onAddBlock,
}: CalendarViewProps) {
  const { t } = useTranslation();
  const [viewDate, setViewDate] = useState(currentMonth);
  const [selectionStart, setSelectionStart] = useState<Date | null>(null);
  const [selectionEnd, setSelectionEnd] = useState<Date | null>(null);
  const [isSelecting, setIsSelecting] = useState(false);

  const year = viewDate.getFullYear();
  const month = viewDate.getMonth();
  const days = getDaysInMonth(year, month);
  const today = new Date();

  const monthNames = [
    t('rentals.calendar.january'),
    t('rentals.calendar.february'),
    t('rentals.calendar.march'),
    t('rentals.calendar.april'),
    t('rentals.calendar.may'),
    t('rentals.calendar.june'),
    t('rentals.calendar.july'),
    t('rentals.calendar.august'),
    t('rentals.calendar.september'),
    t('rentals.calendar.october'),
    t('rentals.calendar.november'),
    t('rentals.calendar.december'),
  ];

  const weekDays = [
    t('rentals.calendar.sun'),
    t('rentals.calendar.mon'),
    t('rentals.calendar.tue'),
    t('rentals.calendar.wed'),
    t('rentals.calendar.thu'),
    t('rentals.calendar.fri'),
    t('rentals.calendar.sat'),
  ];

  const goToPrevMonth = () => {
    const newDate = new Date(year, month - 1, 1);
    setViewDate(newDate);
    onMonthChange?.(newDate);
  };

  const goToNextMonth = () => {
    const newDate = new Date(year, month + 1, 1);
    setViewDate(newDate);
    onMonthChange?.(newDate);
  };

  const goToToday = () => {
    const newDate = new Date();
    setViewDate(newDate);
    onMonthChange?.(newDate);
  };

  const getEventsForDate = (date: Date): CalendarEvent[] => {
    return events.filter((event) => {
      const start = new Date(event.startDate);
      const end = new Date(event.endDate);
      return isDateInRange(date, start, end);
    });
  };

  const handleDateMouseDown = (date: Date) => {
    if (onAddBlock) {
      setIsSelecting(true);
      setSelectionStart(date);
      setSelectionEnd(date);
    }
    onDateSelect?.(date);
  };

  const handleDateMouseEnter = (date: Date) => {
    if (isSelecting && selectionStart) {
      setSelectionEnd(date);
    }
  };

  const handleMouseUp = () => {
    if (isSelecting && selectionStart && selectionEnd && onAddBlock) {
      const start = selectionStart < selectionEnd ? selectionStart : selectionEnd;
      const end = selectionStart < selectionEnd ? selectionEnd : selectionStart;
      onAddBlock(start, end);
    }
    setIsSelecting(false);
    setSelectionStart(null);
    setSelectionEnd(null);
  };

  const isDateSelected = (date: Date): boolean => {
    if (!selectionStart || !selectionEnd) return false;
    const start = selectionStart < selectionEnd ? selectionStart : selectionEnd;
    const end = selectionStart < selectionEnd ? selectionEnd : selectionStart;
    return isDateInRange(date, start, end);
  };

  return (
    <div
      className="bg-white rounded-lg shadow"
      onMouseUp={handleMouseUp}
      onMouseLeave={handleMouseUp}
    >
      {/* Header */}
      <div className="flex items-center justify-between px-6 py-4 border-b">
        <div className="flex items-center gap-4">
          <h2 className="text-lg font-semibold text-gray-900">
            {monthNames[month]} {year}
          </h2>
          <button
            type="button"
            onClick={goToToday}
            className="text-sm text-blue-600 hover:text-blue-800"
          >
            {t('rentals.calendar.today')}
          </button>
        </div>
        <div className="flex items-center gap-2">
          <button
            type="button"
            onClick={goToPrevMonth}
            className="p-2 hover:bg-gray-100 rounded-lg"
            aria-label={t('rentals.calendar.previousMonth')}
          >
            <svg
              className="w-5 h-5 text-gray-600"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              aria-hidden="true"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M15 19l-7-7 7-7"
              />
            </svg>
          </button>
          <button
            type="button"
            onClick={goToNextMonth}
            className="p-2 hover:bg-gray-100 rounded-lg"
            aria-label={t('rentals.calendar.nextMonth')}
          >
            <svg
              className="w-5 h-5 text-gray-600"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              aria-hidden="true"
            >
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
            </svg>
          </button>
        </div>
      </div>

      {/* Legend */}
      <div className="px-6 py-2 border-b flex items-center gap-4 text-xs">
        <div className="flex items-center gap-1">
          <span className="w-3 h-3 rounded bg-blue-100 border border-blue-400" />
          <span className="text-gray-600">{t('rentals.calendar.booking')}</span>
        </div>
        <div className="flex items-center gap-1">
          <span className="w-3 h-3 rounded bg-gray-200 border border-gray-400" />
          <span className="text-gray-600">{t('rentals.calendar.blocked')}</span>
        </div>
        <div className="flex items-center gap-1">
          <span className="w-3 h-3 rounded bg-orange-100 border border-orange-400" />
          <span className="text-gray-600">{t('rentals.calendar.maintenance')}</span>
        </div>
      </div>

      {/* Calendar Grid */}
      <div className="p-4">
        {isLoading ? (
          <div className="h-96 flex items-center justify-center">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" />
          </div>
        ) : (
          <>
            {/* Week day headers */}
            <div className="grid grid-cols-7 mb-2">
              {weekDays.map((day) => (
                <div key={day} className="text-center text-xs font-medium text-gray-500 py-2">
                  {day}
                </div>
              ))}
            </div>

            {/* Days grid */}
            <div className="grid grid-cols-7 gap-1">
              {days.map((date, index) => {
                const isCurrentMonth = date.getMonth() === month;
                const isToday = isSameDay(date, today);
                const dateEvents = getEventsForDate(date);
                const isSelected = isDateSelected(date);

                return (
                  <div
                    key={index}
                    className={`
                      min-h-[100px] p-1 border rounded cursor-pointer transition-colors
                      ${isCurrentMonth ? 'bg-white' : 'bg-gray-50'}
                      ${isToday ? 'border-blue-500 border-2' : 'border-gray-200'}
                      ${isSelected ? 'bg-blue-50' : ''}
                      hover:bg-gray-50
                    `}
                    onMouseDown={() => handleDateMouseDown(date)}
                    onMouseEnter={() => handleDateMouseEnter(date)}
                  >
                    <div
                      className={`
                        text-sm font-medium mb-1
                        ${isCurrentMonth ? 'text-gray-900' : 'text-gray-400'}
                        ${isToday ? 'text-blue-600' : ''}
                      `}
                    >
                      {date.getDate()}
                    </div>
                    <div className="space-y-1">
                      {dateEvents.slice(0, 3).map((event) => {
                        const colors = eventTypeColors[event.type];
                        return (
                          <button
                            key={event.id}
                            type="button"
                            onClick={(e) => {
                              e.stopPropagation();
                              onEventClick?.(event);
                            }}
                            className={`
                              w-full text-left text-xs px-1 py-0.5 rounded truncate
                              ${colors.bg} ${colors.border} ${colors.text}
                              border-l-2 hover:opacity-80
                            `}
                            title={event.title}
                          >
                            {event.title}
                          </button>
                        );
                      })}
                      {dateEvents.length > 3 && (
                        <div className="text-xs text-gray-500 px-1">
                          +{dateEvents.length - 3} {t('rentals.calendar.more')}
                        </div>
                      )}
                    </div>
                  </div>
                );
              })}
            </div>
          </>
        )}
      </div>
    </div>
  );
}
