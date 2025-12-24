/**
 * ConditionBuilder Component
 *
 * Build and configure automation conditions.
 * Part of Story 43.1: Automation Rule Builder.
 */

import type { ConditionOperator, TriggerCondition } from '@ppt/api-client';

interface ConditionBuilderProps {
  conditions: TriggerCondition[];
  onChange: (conditions: TriggerCondition[]) => void;
  disabled?: boolean;
}

const fieldOptions = [
  { value: 'fault.category', label: 'Fault Category', type: 'select' },
  { value: 'fault.priority', label: 'Fault Priority', type: 'select' },
  { value: 'fault.status', label: 'Fault Status', type: 'select' },
  { value: 'fault.location', label: 'Fault Location', type: 'text' },
  { value: 'payment.amount', label: 'Payment Amount', type: 'number' },
  { value: 'payment.daysOverdue', label: 'Days Overdue', type: 'number' },
  { value: 'document.type', label: 'Document Type', type: 'select' },
  { value: 'document.size', label: 'Document Size (MB)', type: 'number' },
  { value: 'user.role', label: 'User Role', type: 'select' },
  { value: 'building.id', label: 'Building', type: 'select' },
  { value: 'unit.id', label: 'Unit', type: 'select' },
];

const operatorOptions: { value: ConditionOperator; label: string; types: string[] }[] = [
  { value: 'equals', label: 'equals', types: ['text', 'number', 'select'] },
  { value: 'not_equals', label: 'does not equal', types: ['text', 'number', 'select'] },
  { value: 'contains', label: 'contains', types: ['text'] },
  { value: 'not_contains', label: 'does not contain', types: ['text'] },
  { value: 'greater_than', label: 'greater than', types: ['number'] },
  { value: 'less_than', label: 'less than', types: ['number'] },
  { value: 'greater_than_or_equals', label: 'greater than or equals', types: ['number'] },
  { value: 'less_than_or_equals', label: 'less than or equals', types: ['number'] },
  { value: 'is_empty', label: 'is empty', types: ['text', 'select'] },
  { value: 'is_not_empty', label: 'is not empty', types: ['text', 'select'] },
  { value: 'in_list', label: 'is one of', types: ['text', 'select'] },
  { value: 'not_in_list', label: 'is not one of', types: ['text', 'select'] },
];

const fieldValueOptions: Record<string, { value: string; label: string }[]> = {
  'fault.category': [
    { value: 'electrical', label: 'Electrical' },
    { value: 'plumbing', label: 'Plumbing' },
    { value: 'hvac', label: 'HVAC' },
    { value: 'structural', label: 'Structural' },
    { value: 'other', label: 'Other' },
  ],
  'fault.priority': [
    { value: 'low', label: 'Low' },
    { value: 'medium', label: 'Medium' },
    { value: 'high', label: 'High' },
    { value: 'critical', label: 'Critical' },
  ],
  'fault.status': [
    { value: 'reported', label: 'Reported' },
    { value: 'acknowledged', label: 'Acknowledged' },
    { value: 'in_progress', label: 'In Progress' },
    { value: 'resolved', label: 'Resolved' },
    { value: 'closed', label: 'Closed' },
  ],
  'document.type': [
    { value: 'contract', label: 'Contract' },
    { value: 'invoice', label: 'Invoice' },
    { value: 'report', label: 'Report' },
    { value: 'minutes', label: 'Meeting Minutes' },
    { value: 'other', label: 'Other' },
  ],
  'user.role': [
    { value: 'owner', label: 'Owner' },
    { value: 'tenant', label: 'Tenant' },
    { value: 'manager', label: 'Manager' },
    { value: 'admin', label: 'Admin' },
  ],
};

export function ConditionBuilder({ conditions, onChange, disabled }: ConditionBuilderProps) {
  const addCondition = () => {
    onChange([...conditions, { field: 'fault.category', operator: 'equals', value: '' }]);
  };

  const updateCondition = (index: number, updates: Partial<TriggerCondition>) => {
    const newConditions = [...conditions];
    newConditions[index] = { ...newConditions[index], ...updates };
    onChange(newConditions);
  };

  const removeCondition = (index: number) => {
    onChange(conditions.filter((_, i) => i !== index));
  };

  const getFieldType = (field: string) => {
    return fieldOptions.find((f) => f.value === field)?.type ?? 'text';
  };

  const getAvailableOperators = (field: string) => {
    const fieldType = getFieldType(field);
    return operatorOptions.filter((op) => op.types.includes(fieldType));
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <span className="block text-sm font-medium text-gray-700">Conditions</span>
        <button
          type="button"
          onClick={addCondition}
          disabled={disabled}
          className="inline-flex items-center px-3 py-1 text-sm bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 disabled:opacity-50"
        >
          <svg
            className="w-4 h-4 mr-1"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            aria-hidden="true"
          >
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
          </svg>
          Add Condition
        </button>
      </div>

      {conditions.length === 0 ? (
        <div className="bg-gray-50 rounded-lg p-6 text-center">
          <svg
            className="mx-auto h-12 w-12 text-gray-400"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
            />
          </svg>
          <p className="mt-2 text-sm text-gray-500">No conditions added</p>
          <p className="text-xs text-gray-400">
            Conditions filter when the automation runs. Without conditions, it runs for all matching
            triggers.
          </p>
        </div>
      ) : (
        <div className="space-y-3">
          {conditions.map((condition, index) => (
            <div
              key={`condition-${index}`}
              className="flex items-start gap-2 p-3 bg-gray-50 rounded-lg"
            >
              {index > 0 && (
                <span className="self-center px-2 py-1 text-xs font-medium text-gray-500 bg-gray-200 rounded">
                  AND
                </span>
              )}

              <div className="flex-1 grid grid-cols-3 gap-2">
                {/* Field Selector */}
                <select
                  value={condition.field}
                  onChange={(e) => updateCondition(index, { field: e.target.value, value: '' })}
                  disabled={disabled}
                  className="px-2 py-1.5 text-sm border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                >
                  {fieldOptions.map((field) => (
                    <option key={field.value} value={field.value}>
                      {field.label}
                    </option>
                  ))}
                </select>

                {/* Operator Selector */}
                <select
                  value={condition.operator}
                  onChange={(e) =>
                    updateCondition(index, { operator: e.target.value as ConditionOperator })
                  }
                  disabled={disabled}
                  className="px-2 py-1.5 text-sm border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                >
                  {getAvailableOperators(condition.field).map((op) => (
                    <option key={op.value} value={op.value}>
                      {op.label}
                    </option>
                  ))}
                </select>

                {/* Value Input */}
                {!['is_empty', 'is_not_empty'].includes(condition.operator) &&
                  (fieldValueOptions[condition.field] ? (
                    <select
                      value={condition.value as string}
                      onChange={(e) => updateCondition(index, { value: e.target.value })}
                      disabled={disabled}
                      className="px-2 py-1.5 text-sm border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                    >
                      <option value="">Select...</option>
                      {fieldValueOptions[condition.field].map((opt) => (
                        <option key={opt.value} value={opt.value}>
                          {opt.label}
                        </option>
                      ))}
                    </select>
                  ) : getFieldType(condition.field) === 'number' ? (
                    <input
                      type="number"
                      value={condition.value as number}
                      onChange={(e) => updateCondition(index, { value: Number(e.target.value) })}
                      disabled={disabled}
                      placeholder="Value"
                      className="px-2 py-1.5 text-sm border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                    />
                  ) : (
                    <input
                      type="text"
                      value={condition.value as string}
                      onChange={(e) => updateCondition(index, { value: e.target.value })}
                      disabled={disabled}
                      placeholder="Value"
                      className="px-2 py-1.5 text-sm border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                    />
                  ))}
              </div>

              <button
                type="button"
                onClick={() => removeCondition(index)}
                disabled={disabled}
                className="p-1 text-gray-400 hover:text-red-500 disabled:opacity-50"
                title="Remove condition"
              >
                <svg
                  className="w-5 h-5"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                  aria-hidden="true"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                  />
                </svg>
              </button>
            </div>
          ))}
        </div>
      )}

      {conditions.length > 0 && (
        <p className="text-xs text-gray-500 mt-2">
          All conditions must be true for the automation to run.
        </p>
      )}
    </div>
  );
}
