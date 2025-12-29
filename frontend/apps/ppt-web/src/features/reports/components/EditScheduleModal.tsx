/**
 * EditScheduleModal - Modal for editing report schedules.
 *
 * Story 81.1 - Report Schedule Editing
 */

import type {
  CreateReportSchedule,
  ReportDefinition,
  ReportFormat,
  ReportSchedule,
  ScheduleFrequency,
} from '@ppt/api-client';
import { useCallback, useEffect, useState } from 'react';
import { RecipientManager } from './RecipientManager';

interface EditScheduleModalProps {
  schedule: ReportSchedule;
  reports: ReportDefinition[];
  isOpen: boolean;
  isSubmitting?: boolean;
  onClose: () => void;
  onSave: (id: string, data: Partial<CreateReportSchedule>) => Promise<void>;
  onPause?: (id: string) => Promise<void>;
  onResume?: (id: string) => Promise<void>;
}

const FREQUENCIES: { value: ScheduleFrequency; label: string }[] = [
  { value: 'daily', label: 'Daily' },
  { value: 'weekly', label: 'Weekly' },
  { value: 'monthly', label: 'Monthly' },
  { value: 'quarterly', label: 'Quarterly' },
  { value: 'yearly', label: 'Yearly' },
];

const FORMATS: { value: ReportFormat; label: string }[] = [
  { value: 'pdf', label: 'PDF' },
  { value: 'excel', label: 'Excel' },
  { value: 'csv', label: 'CSV' },
];

const DAYS_OF_WEEK = [
  { value: 0, label: 'Sunday' },
  { value: 1, label: 'Monday' },
  { value: 2, label: 'Tuesday' },
  { value: 3, label: 'Wednesday' },
  { value: 4, label: 'Thursday' },
  { value: 5, label: 'Friday' },
  { value: 6, label: 'Saturday' },
];

const TIMEZONES = [
  { value: 'Europe/Bratislava', label: 'Europe/Bratislava' },
  { value: 'Europe/Prague', label: 'Europe/Prague' },
  { value: 'Europe/Berlin', label: 'Europe/Berlin' },
  { value: 'Europe/London', label: 'Europe/London' },
  { value: 'America/New_York', label: 'America/New_York' },
  { value: 'UTC', label: 'UTC' },
];

interface FormErrors {
  name?: string;
  recipients?: string;
  time?: string;
}

export function EditScheduleModal({
  schedule,
  reports,
  isOpen,
  isSubmitting,
  onClose,
  onSave,
  onPause,
  onResume,
}: EditScheduleModalProps) {
  // Form state
  const [name, setName] = useState(schedule.name);
  const [reportId, setReportId] = useState(schedule.report_id);
  const [frequency, setFrequency] = useState<ScheduleFrequency>(schedule.frequency);
  const [dayOfWeek, setDayOfWeek] = useState(schedule.day_of_week ?? 1);
  const [dayOfMonth, setDayOfMonth] = useState(schedule.day_of_month ?? 1);
  const [time, setTime] = useState(schedule.time);
  const [timezone, setTimezone] = useState(schedule.timezone);
  const [format, setFormat] = useState<ReportFormat>(schedule.format);
  const [recipients, setRecipients] = useState<string[]>(schedule.recipients);
  const [errors, setErrors] = useState<FormErrors>({});
  const [isPausing, setIsPausing] = useState(false);

  // Reset form when schedule changes
  useEffect(() => {
    setName(schedule.name);
    setReportId(schedule.report_id);
    setFrequency(schedule.frequency);
    setDayOfWeek(schedule.day_of_week ?? 1);
    setDayOfMonth(schedule.day_of_month ?? 1);
    setTime(schedule.time);
    setTimezone(schedule.timezone);
    setFormat(schedule.format);
    setRecipients(schedule.recipients);
    setErrors({});
  }, [schedule]);

  const validate = useCallback((): boolean => {
    const newErrors: FormErrors = {};

    if (!name.trim()) {
      newErrors.name = 'Schedule name is required';
    }

    if (recipients.length === 0) {
      newErrors.recipients = 'At least one recipient is required';
    }

    if (!time) {
      newErrors.time = 'Time is required';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  }, [name, recipients, time]);

  const handleSubmit = useCallback(
    async (e: React.FormEvent) => {
      e.preventDefault();

      if (!validate()) return;

      const data: Partial<CreateReportSchedule> = {
        report_id: reportId,
        name,
        frequency,
        day_of_week: frequency === 'weekly' ? dayOfWeek : undefined,
        day_of_month: frequency === 'monthly' || frequency === 'quarterly' ? dayOfMonth : undefined,
        time,
        timezone,
        format,
        recipients,
      };

      await onSave(schedule.id, data);
    },
    [
      validate,
      reportId,
      name,
      frequency,
      dayOfWeek,
      dayOfMonth,
      time,
      timezone,
      format,
      recipients,
      onSave,
      schedule.id,
    ]
  );

  const handlePauseResume = useCallback(async () => {
    setIsPausing(true);
    try {
      if (schedule.is_active) {
        await onPause?.(schedule.id);
      } else {
        await onResume?.(schedule.id);
      }
    } finally {
      setIsPausing(false);
    }
  }, [schedule.id, schedule.is_active, onPause, onResume]);

  const handleRecipientsChange = useCallback((newRecipients: string[]) => {
    setRecipients(newRecipients);
    setErrors((prev) => ({ ...prev, recipients: undefined }));
  }, []);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 overflow-y-auto">
      {/* Backdrop */}
      <div className="fixed inset-0 bg-black bg-opacity-50" onClick={onClose} />

      {/* Modal */}
      <div className="flex min-h-full items-center justify-center p-4">
        <div className="relative bg-white rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
          {/* Header */}
          <div className="flex items-center justify-between px-6 py-4 border-b border-gray-200">
            <div>
              <h2 className="text-lg font-semibold text-gray-900">Edit Schedule</h2>
              {!schedule.is_active && (
                <span className="inline-flex items-center px-2 py-0.5 mt-1 text-xs font-medium bg-yellow-100 text-yellow-800 rounded-full">
                  Paused
                </span>
              )}
            </div>
            <button
              type="button"
              onClick={onClose}
              className="text-gray-400 hover:text-gray-500"
              aria-label="Close"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M6 18L18 6M6 6l12 12"
                />
              </svg>
            </button>
          </div>

          {/* Form */}
          <form onSubmit={handleSubmit} className="px-6 py-4 space-y-6">
            {/* Report Selection */}
            <div>
              <label htmlFor="edit-report" className="block text-sm font-medium text-gray-700">
                Report
              </label>
              <select
                id="edit-report"
                value={reportId}
                onChange={(e) => setReportId(e.target.value)}
                className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
              >
                {reports.map((report) => (
                  <option key={report.id} value={report.id}>
                    {report.name}
                  </option>
                ))}
              </select>
            </div>

            {/* Schedule Name */}
            <div>
              <label htmlFor="edit-name" className="block text-sm font-medium text-gray-700">
                Schedule Name *
              </label>
              <input
                type="text"
                id="edit-name"
                value={name}
                onChange={(e) => {
                  setName(e.target.value);
                  setErrors((prev) => ({ ...prev, name: undefined }));
                }}
                className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 ${
                  errors.name ? 'border-red-300' : 'border-gray-300'
                }`}
              />
              {errors.name && <p className="mt-1 text-sm text-red-600">{errors.name}</p>}
            </div>

            {/* Frequency */}
            <div>
              <label htmlFor="edit-frequency" className="block text-sm font-medium text-gray-700">
                Frequency
              </label>
              <select
                id="edit-frequency"
                value={frequency}
                onChange={(e) => setFrequency(e.target.value as ScheduleFrequency)}
                className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
              >
                {FREQUENCIES.map((freq) => (
                  <option key={freq.value} value={freq.value}>
                    {freq.label}
                  </option>
                ))}
              </select>
            </div>

            {/* Day Selection */}
            {frequency === 'weekly' && (
              <div>
                <label
                  htmlFor="edit-day-of-week"
                  className="block text-sm font-medium text-gray-700"
                >
                  Day of Week
                </label>
                <select
                  id="edit-day-of-week"
                  value={dayOfWeek}
                  onChange={(e) => setDayOfWeek(Number(e.target.value))}
                  className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                >
                  {DAYS_OF_WEEK.map((day) => (
                    <option key={day.value} value={day.value}>
                      {day.label}
                    </option>
                  ))}
                </select>
              </div>
            )}

            {(frequency === 'monthly' || frequency === 'quarterly') && (
              <div>
                <label
                  htmlFor="edit-day-of-month"
                  className="block text-sm font-medium text-gray-700"
                >
                  Day of Month
                </label>
                <input
                  type="number"
                  id="edit-day-of-month"
                  min="1"
                  max="31"
                  value={dayOfMonth}
                  onChange={(e) => {
                    const value = Number(e.target.value);
                    if (value >= 1 && value <= 31) {
                      setDayOfMonth(value);
                    }
                  }}
                  className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                />
                <p className="mt-1 text-xs text-gray-500">
                  For months with fewer days, the schedule will run on the last day of the month.
                </p>
              </div>
            )}

            {/* Time and Timezone */}
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label htmlFor="edit-time" className="block text-sm font-medium text-gray-700">
                  Time *
                </label>
                <input
                  type="time"
                  id="edit-time"
                  value={time}
                  onChange={(e) => {
                    setTime(e.target.value);
                    setErrors((prev) => ({ ...prev, time: undefined }));
                  }}
                  className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 ${
                    errors.time ? 'border-red-300' : 'border-gray-300'
                  }`}
                />
                {errors.time && <p className="mt-1 text-sm text-red-600">{errors.time}</p>}
              </div>
              <div>
                <label htmlFor="edit-timezone" className="block text-sm font-medium text-gray-700">
                  Timezone
                </label>
                <select
                  id="edit-timezone"
                  value={timezone}
                  onChange={(e) => setTimezone(e.target.value)}
                  className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                >
                  {TIMEZONES.map((tz) => (
                    <option key={tz.value} value={tz.value}>
                      {tz.label}
                    </option>
                  ))}
                </select>
              </div>
            </div>

            {/* Format */}
            <div>
              <label className="block text-sm font-medium text-gray-700">Export Format</label>
              <div className="mt-2 flex gap-4">
                {FORMATS.map((fmt) => (
                  <label key={fmt.value} className="flex items-center gap-2 cursor-pointer">
                    <input
                      type="radio"
                      name="format"
                      value={fmt.value}
                      checked={format === fmt.value}
                      onChange={() => setFormat(fmt.value)}
                      className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300"
                    />
                    <span className="text-sm text-gray-700">{fmt.label}</span>
                  </label>
                ))}
              </div>
            </div>

            {/* Recipients */}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Recipients *</label>
              <RecipientManager
                recipients={recipients}
                onChange={handleRecipientsChange}
                error={errors.recipients}
              />
            </div>
          </form>

          {/* Footer */}
          <div className="flex items-center justify-between px-6 py-4 border-t border-gray-200 bg-gray-50">
            <div>
              {(onPause || onResume) && (
                <button
                  type="button"
                  onClick={handlePauseResume}
                  disabled={isPausing || isSubmitting}
                  className={`px-4 py-2 text-sm font-medium rounded-md disabled:opacity-50 ${
                    schedule.is_active
                      ? 'text-yellow-700 bg-yellow-100 hover:bg-yellow-200'
                      : 'text-green-700 bg-green-100 hover:bg-green-200'
                  }`}
                >
                  {isPausing
                    ? 'Processing...'
                    : schedule.is_active
                      ? 'Pause Schedule'
                      : 'Resume Schedule'}
                </button>
              )}
            </div>
            <div className="flex gap-3">
              <button
                type="button"
                onClick={onClose}
                disabled={isSubmitting}
                className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50"
              >
                Cancel
              </button>
              <button
                type="submit"
                onClick={handleSubmit}
                disabled={isSubmitting}
                className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50"
              >
                {isSubmitting ? 'Saving...' : 'Save Changes'}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
