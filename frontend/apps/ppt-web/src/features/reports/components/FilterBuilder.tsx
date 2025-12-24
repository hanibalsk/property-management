/**
 * FilterBuilder component for building report filters.
 */

import type { ReportField, ReportFilter } from '@ppt/api-client';

interface FilterBuilderProps {
  fields: ReportField[];
  filters: ReportFilter[];
  onFiltersChange: (filters: ReportFilter[]) => void;
}

const OPERATORS = {
  eq: 'Equals',
  ne: 'Not Equals',
  gt: 'Greater Than',
  gte: 'Greater Than or Equal',
  lt: 'Less Than',
  lte: 'Less Than or Equal',
  contains: 'Contains',
  in: 'In List',
  between: 'Between',
};

type Operator = keyof typeof OPERATORS;

const getOperatorsForType = (type: ReportField['type']): Operator[] => {
  switch (type) {
    case 'number':
    case 'currency':
    case 'percentage':
    case 'date':
      return ['eq', 'ne', 'gt', 'gte', 'lt', 'lte', 'between'];
    default:
      return ['eq', 'ne', 'contains', 'in'];
  }
};

export function FilterBuilder({ fields, filters, onFiltersChange }: FilterBuilderProps) {
  const addFilter = () => {
    if (fields.length === 0) return;
    const newFilter: ReportFilter = {
      field_id: fields[0].id,
      operator: 'eq',
      value: '',
    };
    onFiltersChange([...filters, newFilter]);
  };

  const updateFilter = (index: number, updates: Partial<ReportFilter>) => {
    const newFilters = [...filters];
    newFilters[index] = { ...newFilters[index], ...updates };
    onFiltersChange(newFilters);
  };

  const removeFilter = (index: number) => {
    onFiltersChange(filters.filter((_, i) => i !== index));
  };

  const getFieldById = (id: string) => fields.find((f) => f.id === id);

  const renderValueInput = (filter: ReportFilter, index: number) => {
    const field = getFieldById(filter.field_id);
    if (!field) return null;

    if (filter.operator === 'between') {
      const isDateField = field.type === 'date';
      const [min, max] = isDateField
        ? (filter.value as [string, string]) || ['', '']
        : (filter.value as [number, number]) || [0, 0];
      return (
        <div className="flex items-center gap-2">
          <input
            type={isDateField ? 'date' : 'number'}
            value={min}
            onChange={(e) =>
              updateFilter(index, {
                value: isDateField
                  ? [e.target.value, max as string]
                  : [Number(e.target.value), max as number],
              })
            }
            className="w-28 px-2 py-1 text-sm border border-gray-300 rounded"
            placeholder="Min"
          />
          <span className="text-gray-500">and</span>
          <input
            type={isDateField ? 'date' : 'number'}
            value={max}
            onChange={(e) =>
              updateFilter(index, {
                value: isDateField
                  ? [min as string, e.target.value]
                  : [min as number, Number(e.target.value)],
              })
            }
            className="w-28 px-2 py-1 text-sm border border-gray-300 rounded"
            placeholder="Max"
          />
        </div>
      );
    }

    if (filter.operator === 'in') {
      return (
        <input
          type="text"
          value={(filter.value as string[])?.join(', ') || ''}
          onChange={(e) =>
            updateFilter(index, {
              value: e.target.value.split(',').map((v) => v.trim()),
            })
          }
          className="flex-1 px-2 py-1 text-sm border border-gray-300 rounded"
          placeholder="Value1, Value2, Value3"
        />
      );
    }

    switch (field.type) {
      case 'number':
      case 'currency':
      case 'percentage':
        return (
          <input
            type="number"
            value={filter.value as number}
            onChange={(e) => updateFilter(index, { value: Number(e.target.value) })}
            className="w-32 px-2 py-1 text-sm border border-gray-300 rounded"
            placeholder="Value"
          />
        );
      case 'date':
        return (
          <input
            type="date"
            value={filter.value as string}
            onChange={(e) => updateFilter(index, { value: e.target.value })}
            className="w-40 px-2 py-1 text-sm border border-gray-300 rounded"
          />
        );
      default:
        return (
          <input
            type="text"
            value={filter.value as string}
            onChange={(e) => updateFilter(index, { value: e.target.value })}
            className="flex-1 px-2 py-1 text-sm border border-gray-300 rounded"
            placeholder="Value"
          />
        );
    }
  };

  return (
    <div className="space-y-3">
      <div className="flex items-center justify-between">
        <h4 className="text-sm font-medium text-gray-700">Filters</h4>
        <button
          type="button"
          onClick={addFilter}
          disabled={fields.length === 0}
          className="text-sm text-blue-600 hover:text-blue-800 disabled:text-gray-400"
        >
          + Add Filter
        </button>
      </div>

      {filters.length === 0 ? (
        <p className="text-sm text-gray-500 text-center py-4 border border-dashed rounded-lg">
          No filters applied. Click "Add Filter" to filter results.
        </p>
      ) : (
        <div className="space-y-2">
          {filters.map((filter, index) => {
            const field = getFieldById(filter.field_id);
            const availableOperators = field ? getOperatorsForType(field.type) : ['eq'];

            return (
              <div
                key={`filter-${filter.field_id}-${index}`}
                className="flex items-center gap-2 p-2 bg-gray-50 rounded-lg"
              >
                {index > 0 && <span className="text-xs text-gray-500 w-8">AND</span>}

                {/* Field Select */}
                <select
                  value={filter.field_id}
                  onChange={(e) => {
                    const newFieldId = e.target.value;
                    const newField = getFieldById(newFieldId);
                    if (!newField) return;

                    const newAvailableOperators = getOperatorsForType(newField.type);
                    const newOperator = newAvailableOperators.includes(filter.operator as Operator)
                      ? filter.operator
                      : newAvailableOperators[0];

                    updateFilter(index, { field_id: newFieldId, operator: newOperator, value: '' });
                  }}
                  className="px-2 py-1 text-sm border border-gray-300 rounded bg-white"
                >
                  {fields.map((f) => (
                    <option key={f.id} value={f.id}>
                      {f.name}
                    </option>
                  ))}
                </select>

                {/* Operator Select */}
                <select
                  value={filter.operator}
                  onChange={(e) => {
                    const newOperator = e.target.value as ReportFilter['operator'];
                    let newValue: string | number | string[] | [number, number] = filter.value;

                    if (newOperator === 'between') {
                      if (!Array.isArray(newValue) || newValue.length !== 2) {
                        newValue = ['', ''];
                      }
                    } else if (newOperator === 'in') {
                      if (!Array.isArray(newValue)) {
                        if (newValue !== undefined && newValue !== null && newValue !== '') {
                          newValue = [String(newValue)];
                        } else {
                          newValue = [];
                        }
                      }
                    } else {
                      if (Array.isArray(newValue)) {
                        newValue = String(newValue[0] ?? '');
                      }
                    }

                    updateFilter(index, { operator: newOperator, value: newValue });
                  }}
                  className="px-2 py-1 text-sm border border-gray-300 rounded bg-white"
                >
                  {availableOperators.map((op) => (
                    <option key={op} value={op}>
                      {OPERATORS[op as Operator]}
                    </option>
                  ))}
                </select>

                {/* Value Input */}
                {renderValueInput(filter, index)}

                {/* Remove Button */}
                <button
                  type="button"
                  onClick={() => removeFilter(index)}
                  className="text-red-500 hover:text-red-700 p-1"
                  title="Remove filter"
                >
                  <svg
                    className="w-4 h-4"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                    aria-hidden="true"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M6 18L18 6M6 6l12 12"
                    />
                  </svg>
                </button>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
