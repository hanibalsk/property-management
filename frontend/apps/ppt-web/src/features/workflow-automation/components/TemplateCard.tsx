/**
 * TemplateCard Component
 *
 * Display automation template in a card format.
 * Part of Story 43.2: Template Library.
 */

import type { AutomationTemplate } from '@ppt/api-client';

interface TemplateCardProps {
  template: AutomationTemplate;
  onUse: (template: AutomationTemplate) => void;
  onPreview: (template: AutomationTemplate) => void;
}

const categoryColors: Record<string, string> = {
  faults: 'bg-red-100 text-red-800',
  payments: 'bg-green-100 text-green-800',
  communications: 'bg-blue-100 text-blue-800',
  documents: 'bg-purple-100 text-purple-800',
  maintenance: 'bg-orange-100 text-orange-800',
  general: 'bg-gray-100 text-gray-800',
};

const categoryIcons: Record<string, string> = {
  faults: '🔧',
  payments: '💰',
  communications: '📢',
  documents: '📄',
  maintenance: '🛠️',
  general: '⚙️',
};

export function TemplateCard({ template, onUse, onPreview }: TemplateCardProps) {
  const categoryColor = categoryColors[template.category] ?? categoryColors.general;
  const categoryIcon = categoryIcons[template.category] ?? categoryIcons.general;

  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-5 hover:shadow-md transition-shadow flex flex-col h-full">
      <div className="flex items-start gap-3 mb-4">
        <span className="text-3xl">{categoryIcon}</span>
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-1 flex-wrap">
            <h3 className="text-base font-semibold text-gray-900">{template.name}</h3>
            <span
              className={`px-2 py-0.5 text-xs font-medium rounded-full capitalize ${categoryColor}`}
            >
              {template.category}
            </span>
            {template.isPopular && (
              <span className="px-2 py-0.5 text-xs font-medium rounded-full bg-yellow-100 text-yellow-800">
                Popular
              </span>
            )}
          </div>
          <p className="text-sm text-gray-500 line-clamp-2">{template.description}</p>
        </div>
      </div>

      {/* Template Details */}
      <div className="flex-1">
        <div className="flex flex-wrap gap-2 text-xs text-gray-400 mb-4">
          <span className="flex items-center gap-1">
            <svg
              className="w-3 h-3"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              aria-hidden="true"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M13 10V3L4 14h7v7l9-11h-7z"
              />
            </svg>
            {template.triggerType === 'time_based'
              ? 'Time Based'
              : template.triggerType === 'event_based'
                ? 'Event Based'
                : template.triggerType === 'condition_based'
                  ? 'Condition Based'
                  : 'Manual'}
          </span>
          <span>•</span>
          <span>
            {template.actionsCount ?? 0} action{(template.actionsCount ?? 0) !== 1 ? 's' : ''}
          </span>
          {template.usageCount !== undefined && (
            <>
              <span>•</span>
              <span>{template.usageCount} uses</span>
            </>
          )}
        </div>

        {/* Tags */}
        {template.tags && template.tags.length > 0 && (
          <div className="flex flex-wrap gap-1 mb-4">
            {template.tags.slice(0, 3).map((tag) => (
              <span key={tag} className="px-2 py-0.5 text-xs bg-gray-100 text-gray-600 rounded">
                {tag}
              </span>
            ))}
            {template.tags.length > 3 && (
              <span className="px-2 py-0.5 text-xs text-gray-400">
                +{template.tags.length - 3} more
              </span>
            )}
          </div>
        )}
      </div>

      {/* Action Buttons */}
      <div className="flex gap-2 pt-4 border-t border-gray-100 mt-auto">
        <button
          type="button"
          onClick={() => onPreview(template)}
          className="flex-1 px-3 py-2 text-sm font-medium text-gray-700 bg-gray-50 rounded-md hover:bg-gray-100 transition-colors"
        >
          Preview
        </button>
        <button
          type="button"
          onClick={() => onUse(template)}
          className="flex-1 px-3 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 transition-colors"
        >
          Use Template
        </button>
      </div>
    </div>
  );
}
