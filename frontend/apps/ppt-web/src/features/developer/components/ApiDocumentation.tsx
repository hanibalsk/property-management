/**
 * API Documentation Component (Epic 69 - Story 69.2)
 *
 * Interactive API documentation with endpoint explorer and code examples.
 */

import { useState } from 'react';
import type { ApiEndpointDoc, ApiChangelog } from '../types';

interface ApiDocumentationProps {
  endpoints: ApiEndpointDoc[];
  changelog: ApiChangelog[];
  onTestEndpoint?: (endpoint: ApiEndpointDoc) => void;
}

export function ApiDocumentation({ endpoints, changelog, onTestEndpoint }: ApiDocumentationProps) {
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedTag, setSelectedTag] = useState<string | null>(null);
  const [expandedEndpoint, setExpandedEndpoint] = useState<string | null>(null);

  // Group endpoints by tag
  const tags = [...new Set(endpoints.map((e) => e.tag))];

  const filteredEndpoints = endpoints.filter((endpoint) => {
    const matchesSearch =
      !searchQuery ||
      endpoint.path.toLowerCase().includes(searchQuery.toLowerCase()) ||
      endpoint.summary.toLowerCase().includes(searchQuery.toLowerCase());
    const matchesTag = !selectedTag || endpoint.tag === selectedTag;
    return matchesSearch && matchesTag;
  });

  const groupedEndpoints = filteredEndpoints.reduce(
    (acc, endpoint) => {
      if (!acc[endpoint.tag]) {
        acc[endpoint.tag] = [];
      }
      acc[endpoint.tag].push(endpoint);
      return acc;
    },
    {} as Record<string, ApiEndpointDoc[]>
  );

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold tracking-tight">API Documentation</h2>
        <p className="text-muted-foreground">
          Explore endpoints, view examples, and test API calls
        </p>
      </div>

      {/* Search and Filter */}
      <div className="flex gap-4">
        <div className="flex-1">
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search endpoints..."
            className="w-full px-4 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <select
          value={selectedTag || ''}
          onChange={(e) => setSelectedTag(e.target.value || null)}
          className="px-4 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="">All Tags</option>
          {tags.map((tag) => (
            <option key={tag} value={tag}>
              {tag}
            </option>
          ))}
        </select>
      </div>

      {/* Endpoints List */}
      <div className="space-y-6">
        {Object.entries(groupedEndpoints).map(([tag, tagEndpoints]) => (
          <div key={tag} className="border rounded-lg overflow-hidden">
            <div className="px-4 py-3 bg-gray-50 border-b">
              <h3 className="font-semibold">{tag}</h3>
              <p className="text-sm text-muted-foreground">
                {tagEndpoints.length} endpoint{tagEndpoints.length !== 1 ? 's' : ''}
              </p>
            </div>
            <div className="divide-y">
              {tagEndpoints.map((endpoint) => (
                <EndpointItem
                  key={endpoint.id}
                  endpoint={endpoint}
                  isExpanded={expandedEndpoint === endpoint.id}
                  onToggle={() =>
                    setExpandedEndpoint(expandedEndpoint === endpoint.id ? null : endpoint.id)
                  }
                  onTest={onTestEndpoint ? () => onTestEndpoint(endpoint) : undefined}
                />
              ))}
            </div>
          </div>
        ))}
      </div>

      {filteredEndpoints.length === 0 && (
        <div className="p-8 text-center bg-gray-50 rounded-lg">
          <p className="text-muted-foreground">No endpoints found matching your criteria.</p>
        </div>
      )}
    </div>
  );
}

interface EndpointItemProps {
  endpoint: ApiEndpointDoc;
  isExpanded: boolean;
  onToggle: () => void;
  onTest?: () => void;
}

function EndpointItem({ endpoint, isExpanded, onToggle, onTest }: EndpointItemProps) {
  const methodColors = {
    GET: 'bg-green-100 text-green-800',
    POST: 'bg-blue-100 text-blue-800',
    PUT: 'bg-yellow-100 text-yellow-800',
    PATCH: 'bg-orange-100 text-orange-800',
    DELETE: 'bg-red-100 text-red-800',
  };

  return (
    <div className="bg-white">
      <button
        type="button"
        onClick={onToggle}
        className="w-full px-4 py-3 flex items-center gap-4 text-left hover:bg-gray-50 transition-colors"
      >
        <span
          className={`px-2 py-1 text-xs font-bold rounded ${
            methodColors[endpoint.method] || 'bg-gray-100 text-gray-800'
          }`}
        >
          {endpoint.method}
        </span>
        <code className="text-sm font-mono flex-1">{endpoint.path}</code>
        {endpoint.isDeprecated && (
          <span className="px-2 py-0.5 text-xs bg-red-100 text-red-800 rounded">Deprecated</span>
        )}
        <span className="text-sm text-muted-foreground">{endpoint.summary}</span>
        <svg
          className={`w-5 h-5 text-gray-400 transition-transform ${isExpanded ? 'rotate-180' : ''}`}
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </button>

      {isExpanded && (
        <div className="px-4 pb-4 space-y-4">
          {endpoint.description && (
            <p className="text-sm text-muted-foreground">{endpoint.description}</p>
          )}

          {/* Authentication */}
          <div className="flex items-center gap-4 text-sm">
            <span className="font-medium">Authentication:</span>
            {endpoint.requiresAuth ? (
              <span className="text-yellow-600">Required</span>
            ) : (
              <span className="text-green-600">Not required</span>
            )}
            {endpoint.requiredScopes && endpoint.requiredScopes.length > 0 && (
              <div className="flex gap-1">
                {endpoint.requiredScopes.map((scope) => (
                  <span key={scope} className="px-2 py-0.5 text-xs bg-gray-100 rounded">
                    {scope}
                  </span>
                ))}
              </div>
            )}
          </div>

          {/* Request Example */}
          {endpoint.requestBody && (
            <div>
              <h4 className="text-sm font-medium mb-2">Request Body</h4>
              <pre className="p-3 bg-gray-900 text-gray-100 rounded-md text-sm overflow-x-auto">
                <code>{JSON.stringify(endpoint.requestBody, null, 2)}</code>
              </pre>
            </div>
          )}

          {/* Response Example */}
          {endpoint.responseBody && (
            <div>
              <h4 className="text-sm font-medium mb-2">Response</h4>
              <pre className="p-3 bg-gray-900 text-gray-100 rounded-md text-sm overflow-x-auto">
                <code>{JSON.stringify(endpoint.responseBody, null, 2)}</code>
              </pre>
            </div>
          )}

          {/* Actions */}
          <div className="flex gap-2 pt-2">
            {onTest && (
              <button
                type="button"
                onClick={onTest}
                className="px-4 py-2 bg-blue-600 text-white text-sm rounded-md hover:bg-blue-700 transition-colors"
              >
                Try it out
              </button>
            )}
          </div>
        </div>
      )}
    </div>
  );
}

// Changelog component
export function ApiChangelogList({ changelog }: { changelog: ApiChangelog[] }) {
  return (
    <div className="space-y-6">
      <h3 className="text-lg font-semibold">API Changelog</h3>

      <div className="space-y-4">
        {changelog.map((entry) => (
          <div key={entry.id} className="p-4 border rounded-lg">
            <div className="flex items-center gap-3 mb-2">
              <span className="px-2 py-1 bg-blue-100 text-blue-800 text-sm font-medium rounded">
                v{entry.version}
              </span>
              <span className="text-sm text-muted-foreground">{entry.releaseDate}</span>
            </div>
            <h4 className="font-medium mb-2">{entry.title}</h4>
            {entry.description && (
              <p className="text-sm text-muted-foreground mb-3">{entry.description}</p>
            )}
            <ul className="space-y-1 text-sm">
              {entry.changes.map((change, index) => (
                <li key={index} className="flex items-start gap-2">
                  <ChangeTypeBadge type={change.type} />
                  <span>{change.description}</span>
                </li>
              ))}
            </ul>
            {entry.breakingChanges && entry.breakingChanges.length > 0 && (
              <div className="mt-3 p-3 bg-red-50 rounded-md">
                <h5 className="text-sm font-medium text-red-800 mb-2">Breaking Changes</h5>
                <ul className="space-y-1 text-sm text-red-700">
                  {entry.breakingChanges.map((change, index) => (
                    <li key={index}>{change.description}</li>
                  ))}
                </ul>
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}

function ChangeTypeBadge({ type }: { type: string }) {
  const colors = {
    added: 'bg-green-100 text-green-800',
    changed: 'bg-blue-100 text-blue-800',
    deprecated: 'bg-yellow-100 text-yellow-800',
    removed: 'bg-red-100 text-red-800',
    fixed: 'bg-purple-100 text-purple-800',
    security: 'bg-orange-100 text-orange-800',
  };

  return (
    <span
      className={`px-1.5 py-0.5 text-xs font-medium rounded ${
        colors[type as keyof typeof colors] || 'bg-gray-100 text-gray-800'
      }`}
    >
      {type}
    </span>
  );
}

// Stub components for index exports
export function ApiEndpointExplorer({ endpoint }: { endpoint: ApiEndpointDoc }) {
  return <div>Endpoint Explorer: {endpoint.path}</div>;
}

export function SandboxTester() {
  return <div>Sandbox Tester Component</div>;
}
