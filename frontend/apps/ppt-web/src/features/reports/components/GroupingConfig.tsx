/**
 * GroupingConfig component for configuring report groupings.
 */

import type { ReportField, ReportGrouping } from '@ppt/api-client';

interface GroupingConfigProps {
  fields: ReportField[];
  groupings: ReportGrouping[];
  onGroupingsChange: (groupings: ReportGrouping[]) => void;
}

export function GroupingConfig({ fields, groupings, onGroupingsChange }: GroupingConfigProps) {
  const addGrouping = () => {
    const availableFields = fields.filter((f) => !groupings.find((g) => g.field_id === f.id));
    if (availableFields.length === 0) return;

    const newGrouping: ReportGrouping = {
      field_id: availableFields[0].id,
      order: 'asc',
    };
    onGroupingsChange([...groupings, newGrouping]);
  };

  const updateGrouping = (index: number, updates: Partial<ReportGrouping>) => {
    const newGroupings = [...groupings];
    newGroupings[index] = { ...newGroupings[index], ...updates };
    onGroupingsChange(newGroupings);
  };

  const removeGrouping = (index: number) => {
    onGroupingsChange(groupings.filter((_, i) => i !== index));
  };

  const moveGrouping = (index: number, direction: 'up' | 'down') => {
    const newGroupings = [...groupings];
    const newIndex = direction === 'up' ? index - 1 : index + 1;
    if (newIndex < 0 || newIndex >= groupings.length) return;
    [newGroupings[index], newGroupings[newIndex]] = [newGroupings[newIndex], newGroupings[index]];
    onGroupingsChange(newGroupings);
  };

  const getFieldById = (id: string) => fields.find((f) => f.id === id);

  const availableFields = fields.filter((f) => !groupings.find((g) => g.field_id === f.id));

  return (
    <div className="space-y-3">
      <div className="flex items-center justify-between">
        <h4 className="text-sm font-medium text-gray-700">Group By</h4>
        <button
          type="button"
          onClick={addGrouping}
          disabled={availableFields.length === 0}
          className="text-sm text-blue-600 hover:text-blue-800 disabled:text-gray-400"
        >
          + Add Grouping
        </button>
      </div>

      {groupings.length === 0 ? (
        <p className="text-sm text-gray-500 text-center py-4 border border-dashed rounded-lg">
          No groupings. Data will be displayed as individual rows.
        </p>
      ) : (
        <div className="space-y-2">
          {groupings.map((grouping, index) => {
            const field = getFieldById(grouping.field_id);

            return (
              <div
                key={`grouping-${grouping.field_id}-${index}`}
                className="flex items-center gap-2 p-2 bg-gray-50 rounded-lg"
              >
                <span className="text-xs text-gray-500 w-6">{index + 1}.</span>

                {/* Field Select */}
                <select
                  value={grouping.field_id}
                  onChange={(e) => updateGrouping(index, { field_id: e.target.value })}
                  className="flex-1 px-2 py-1 text-sm border border-gray-300 rounded bg-white"
                >
                  {field && (
                    <option key={field.id} value={field.id}>
                      {field.name}
                    </option>
                  )}
                  {availableFields.map((f) => (
                    <option key={f.id} value={f.id}>
                      {f.name}
                    </option>
                  ))}
                </select>

                {/* Sort Order */}
                <select
                  value={grouping.order}
                  onChange={(e) =>
                    updateGrouping(index, { order: e.target.value as 'asc' | 'desc' })
                  }
                  className="px-2 py-1 text-sm border border-gray-300 rounded bg-white"
                >
                  <option value="asc">Ascending</option>
                  <option value="desc">Descending</option>
                </select>

                {/* Reorder Buttons */}
                <div className="flex items-center gap-1">
                  <button
                    type="button"
                    onClick={() => moveGrouping(index, 'up')}
                    disabled={index === 0}
                    className="text-gray-400 hover:text-gray-600 disabled:opacity-30"
                    title="Move up"
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
                        d="M5 15l7-7 7 7"
                      />
                    </svg>
                  </button>
                  <button
                    type="button"
                    onClick={() => moveGrouping(index, 'down')}
                    disabled={index === groupings.length - 1}
                    className="text-gray-400 hover:text-gray-600 disabled:opacity-30"
                    title="Move down"
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
                        d="M19 9l-7 7-7-7"
                      />
                    </svg>
                  </button>
                </div>

                {/* Remove Button */}
                <button
                  type="button"
                  onClick={() => removeGrouping(index)}
                  className="text-red-500 hover:text-red-700 p-1"
                  title="Remove grouping"
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

      {groupings.length > 0 && (
        <p className="text-xs text-gray-500">
          Data will be grouped by these fields in the order shown.
        </p>
      )}
    </div>
  );
}
