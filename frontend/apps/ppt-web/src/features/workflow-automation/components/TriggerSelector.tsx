/**
 * TriggerSelector Component
 *
 * Select and configure automation triggers.
 * Part of Story 43.1: Automation Rule Builder.
 */

import type {
  AutomationTrigger,
  EventTriggerType,
  TimeTriggerConfig,
  TriggerType,
} from '@ppt/api-client';
import { useState } from 'react';

interface TriggerSelectorProps {
  value?: Partial<AutomationTrigger>;
  onChange: (trigger: Partial<AutomationTrigger>) => void;
  disabled?: boolean;
}

const triggerTypes: { value: TriggerType; label: string; description: string; icon: string }[] = [
  {
    value: 'time_based',
    label: 'Time Based',
    description: 'Run on a schedule (daily, weekly, etc.)',
    icon: '🕐',
  },
  {
    value: 'event_based',
    label: 'Event Based',
    description: 'Run when something happens',
    icon: '⚡',
  },
  {
    value: 'condition_based',
    label: 'Condition Based',
    description: 'Run when conditions are met',
    icon: '🔀',
  },
  {
    value: 'manual',
    label: 'Manual',
    description: 'Run only when triggered manually',
    icon: '👆',
  },
];

const eventTypes: { value: EventTriggerType; label: string; category: string }[] = [
  { value: 'fault_created', label: 'Fault Created', category: 'Faults' },
  { value: 'fault_status_changed', label: 'Fault Status Changed', category: 'Faults' },
  { value: 'payment_received', label: 'Payment Received', category: 'Payments' },
  { value: 'payment_overdue', label: 'Payment Overdue', category: 'Payments' },
  { value: 'document_uploaded', label: 'Document Uploaded', category: 'Documents' },
  { value: 'announcement_published', label: 'Announcement Published', category: 'Announcements' },
  { value: 'vote_started', label: 'Vote Started', category: 'Voting' },
  { value: 'vote_ended', label: 'Vote Ended', category: 'Voting' },
  { value: 'guest_registered', label: 'Guest Registered', category: 'Guests' },
  { value: 'maintenance_scheduled', label: 'Maintenance Scheduled', category: 'Maintenance' },
  { value: 'meter_reading_due', label: 'Meter Reading Due', category: 'Meters' },
  { value: 'lease_expiring', label: 'Lease Expiring', category: 'Leases' },
];

const schedulePresets = [
  { label: 'Every hour', value: '0 * * * *' },
  { label: 'Every day at 9 AM', value: '0 9 * * *' },
  { label: 'Every Monday at 9 AM', value: '0 9 * * 1' },
  { label: 'First of every month', value: '0 9 1 * *' },
  { label: 'Custom...', value: 'custom' },
];

export function TriggerSelector({ value, onChange, disabled }: TriggerSelectorProps) {
  const [showScheduleCustom, setShowScheduleCustom] = useState(false);
  const [customCron, setCustomCron] = useState('');

  const handleTypeSelect = (type: TriggerType) => {
    onChange({
      ...value,
      type,
      name: triggerTypes.find((t) => t.value === type)?.label ?? '',
      timeConfig:
        type === 'time_based'
          ? { schedule: '0 9 * * *', timezone: 'Europe/Bratislava' }
          : undefined,
      eventConfig: type === 'event_based' ? { eventType: 'fault_created' } : undefined,
    });
  };

  const handleEventTypeChange = (eventType: EventTriggerType) => {
    onChange({
      ...value,
      eventConfig: { ...value?.eventConfig, eventType },
    });
  };

  const handleScheduleChange = (schedule: string) => {
    if (schedule === 'custom') {
      setShowScheduleCustom(true);
      return;
    }
    setShowScheduleCustom(false);
    onChange({
      ...value,
      timeConfig: {
        ...value?.timeConfig,
        schedule,
        timezone: value?.timeConfig?.timezone ?? 'Europe/Bratislava',
      } as TimeTriggerConfig,
    });
  };

  const handleCustomCronSave = () => {
    if (customCron) {
      onChange({
        ...value,
        timeConfig: {
          schedule: customCron,
          timezone: value?.timeConfig?.timezone ?? 'Europe/Bratislava',
        },
      });
      setShowScheduleCustom(false);
    }
  };

  return (
    <div className="space-y-6">
      {/* Trigger Type Selection */}
      <div>
        <span className="block text-sm font-medium text-gray-700 mb-3">Trigger Type</span>
        <div className="grid grid-cols-2 gap-3">
          {triggerTypes.map((trigger) => (
            <button
              key={trigger.value}
              type="button"
              disabled={disabled}
              onClick={() => handleTypeSelect(trigger.value)}
              className={`flex flex-col items-start p-4 border rounded-lg transition-colors text-left ${
                value?.type === trigger.value
                  ? 'border-blue-500 bg-blue-50 ring-1 ring-blue-500'
                  : 'border-gray-200 hover:border-gray-300'
              } ${disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}`}
            >
              <span className="text-2xl mb-2">{trigger.icon}</span>
              <span className="font-medium text-gray-900">{trigger.label}</span>
              <span className="text-xs text-gray-500 mt-1">{trigger.description}</span>
            </button>
          ))}
        </div>
      </div>

      {/* Time-based Configuration */}
      {value?.type === 'time_based' && (
        <div className="border-t pt-6">
          <span className="block text-sm font-medium text-gray-700 mb-3">Schedule</span>
          <div className="space-y-3">
            {schedulePresets.map((preset) => (
              <label
                key={preset.value}
                className={`flex items-center p-3 border rounded-lg cursor-pointer transition-colors ${
                  value?.timeConfig?.schedule === preset.value ||
                  (preset.value === 'custom' && showScheduleCustom)
                    ? 'border-blue-500 bg-blue-50'
                    : 'border-gray-200 hover:border-gray-300'
                }`}
              >
                <input
                  type="radio"
                  name="schedule"
                  value={preset.value}
                  checked={value?.timeConfig?.schedule === preset.value}
                  onChange={() => handleScheduleChange(preset.value)}
                  disabled={disabled}
                  className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300"
                />
                <span className="ml-3 text-sm text-gray-900">{preset.label}</span>
              </label>
            ))}

            {showScheduleCustom && (
              <div className="flex gap-2 mt-3">
                <input
                  type="text"
                  value={customCron}
                  onChange={(e) => setCustomCron(e.target.value)}
                  placeholder="0 9 * * *"
                  disabled={disabled}
                  className="flex-1 px-3 py-2 border border-gray-300 rounded-md text-sm focus:ring-blue-500 focus:border-blue-500"
                />
                <button
                  type="button"
                  onClick={handleCustomCronSave}
                  disabled={disabled || !customCron}
                  className="px-4 py-2 bg-blue-600 text-white rounded-md text-sm hover:bg-blue-700 disabled:opacity-50"
                >
                  Apply
                </button>
              </div>
            )}

            <div className="mt-4">
              <label htmlFor="timezone" className="block text-sm font-medium text-gray-700 mb-1">
                Timezone
              </label>
              <select
                id="timezone"
                value={value?.timeConfig?.timezone ?? 'Europe/Bratislava'}
                onChange={(e) =>
                  onChange({
                    ...value,
                    timeConfig: {
                      ...value?.timeConfig,
                      timezone: e.target.value,
                    } as TimeTriggerConfig,
                  })
                }
                disabled={disabled}
                className="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:ring-blue-500 focus:border-blue-500"
              >
                <option value="Europe/Bratislava">Europe/Bratislava (CET)</option>
                <option value="Europe/Prague">Europe/Prague (CET)</option>
                <option value="Europe/Vienna">Europe/Vienna (CET)</option>
                <option value="Europe/Berlin">Europe/Berlin (CET)</option>
                <option value="UTC">UTC</option>
              </select>
            </div>
          </div>
        </div>
      )}

      {/* Event-based Configuration */}
      {value?.type === 'event_based' && (
        <div className="border-t pt-6">
          <span className="block text-sm font-medium text-gray-700 mb-3">Event Type</span>
          <div className="space-y-4">
            {[
              'Faults',
              'Payments',
              'Documents',
              'Announcements',
              'Voting',
              'Guests',
              'Maintenance',
              'Meters',
              'Leases',
            ].map((category) => {
              const categoryEvents = eventTypes.filter((e) => e.category === category);
              if (categoryEvents.length === 0) return null;

              return (
                <div key={category}>
                  <h4 className="text-xs font-medium text-gray-500 uppercase mb-2">{category}</h4>
                  <div className="grid grid-cols-2 gap-2">
                    {categoryEvents.map((event) => (
                      <label
                        key={event.value}
                        className={`flex items-center p-2 border rounded cursor-pointer transition-colors ${
                          value?.eventConfig?.eventType === event.value
                            ? 'border-blue-500 bg-blue-50'
                            : 'border-gray-200 hover:border-gray-300'
                        }`}
                      >
                        <input
                          type="radio"
                          name="eventType"
                          value={event.value}
                          checked={value?.eventConfig?.eventType === event.value}
                          onChange={() => handleEventTypeChange(event.value)}
                          disabled={disabled}
                          className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300"
                        />
                        <span className="ml-2 text-sm text-gray-900">{event.label}</span>
                      </label>
                    ))}
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      )}

      {/* Manual Trigger Info */}
      {value?.type === 'manual' && (
        <div className="border-t pt-6">
          <div className="bg-gray-50 rounded-lg p-4">
            <div className="flex items-start gap-3">
              <span className="text-2xl">👆</span>
              <div>
                <p className="text-sm text-gray-700">
                  This automation will only run when you manually trigger it from the automations
                  dashboard.
                </p>
                <p className="text-xs text-gray-500 mt-2">
                  Useful for one-time tasks or actions you want to control.
                </p>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Condition-based Info */}
      {value?.type === 'condition_based' && (
        <div className="border-t pt-6">
          <div className="bg-amber-50 rounded-lg p-4">
            <div className="flex items-start gap-3">
              <span className="text-2xl">🔀</span>
              <div>
                <p className="text-sm text-gray-700">
                  This automation will check conditions periodically and run when all conditions are
                  met.
                </p>
                <p className="text-xs text-gray-500 mt-2">Configure conditions in the next step.</p>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
