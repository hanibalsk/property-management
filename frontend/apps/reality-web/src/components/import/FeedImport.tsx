/**
 * FeedImport Component
 *
 * Import from XML/RSS feeds (Epic 46, Story 46.4).
 */

'use client';

import type {
  FeedFieldMapping,
  FeedFormat,
  FeedPreview,
  FeedSource,
  SyncFrequency,
} from '@ppt/reality-api-client';
import {
  useCreateFeedSource,
  useDeleteFeedSource,
  useFeedPreview,
  useFeedSources,
  useFeedSyncHistory,
  useMyAgency,
  useSyncFeedSource,
  useUpdateFeedSource,
} from '@ppt/reality-api-client';
import { useState } from 'react';

const DEFAULT_FIELD_MAPPING: FeedFieldMapping = {
  title: 'title',
  description: 'description',
  price: 'price',
  propertyType: 'type',
  address: 'address',
  city: 'city',
  rooms: 'bedrooms',
  size: 'area',
  photos: 'images',
};

const FREQUENCY_OPTIONS: { value: SyncFrequency; label: string }[] = [
  { value: 'manual', label: 'Manual' },
  { value: 'hourly', label: 'Hourly' },
  { value: 'daily', label: 'Daily' },
  { value: 'weekly', label: 'Weekly' },
];

export function FeedImport() {
  const [showModal, setShowModal] = useState(false);

  const { data: agency } = useMyAgency();
  const { data: feeds, isLoading } = useFeedSources(agency?.id || '');

  return (
    <div className="feed-import">
      <div className="header">
        <div>
          <h2>Feed Sources</h2>
          <p className="subtitle">Import listings from XML, RSS, or JSON feeds</p>
        </div>
        <button type="button" className="add-button" onClick={() => setShowModal(true)}>
          <svg
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            aria-hidden="true"
          >
            <line x1="12" y1="5" x2="12" y2="19" />
            <line x1="5" y1="12" x2="19" y2="12" />
          </svg>
          Add Feed
        </button>
      </div>

      {isLoading ? (
        <FeedsSkeleton />
      ) : feeds?.length === 0 ? (
        <EmptyState onAdd={() => setShowModal(true)} />
      ) : (
        <div className="feeds-list">
          {feeds?.map((feed) => (
            <FeedCard key={feed.id} feed={feed} agencyId={agency?.id || ''} />
          ))}
        </div>
      )}

      {showModal && (
        <AddFeedModal agencyId={agency?.id || ''} onClose={() => setShowModal(false)} />
      )}

      <style jsx>{`
        .feed-import {
          padding: 24px;
        }

        .header {
          display: flex;
          justify-content: space-between;
          align-items: flex-start;
          margin-bottom: 24px;
          flex-wrap: wrap;
          gap: 16px;
        }

        h2 {
          font-size: 1.5rem;
          color: #111827;
          margin: 0 0 4px;
        }

        .subtitle {
          color: #6b7280;
          margin: 0;
        }

        .add-button {
          display: flex;
          align-items: center;
          gap: 8px;
          padding: 12px 20px;
          background: #2563eb;
          color: #fff;
          border: none;
          border-radius: 8px;
          font-size: 14px;
          font-weight: 500;
          cursor: pointer;
        }

        .add-button:hover {
          background: #1d4ed8;
        }

        .feeds-list {
          display: flex;
          flex-direction: column;
          gap: 16px;
        }
      `}</style>
    </div>
  );
}

function FeedCard({ feed, agencyId }: { feed: FeedSource; agencyId: string }) {
  const [showHistory, setShowHistory] = useState(false);
  const deleteMutation = useDeleteFeedSource(agencyId);
  const syncMutation = useSyncFeedSource(agencyId, feed.id);
  const updateMutation = useUpdateFeedSource(agencyId, feed.id);
  const { data: history } = useFeedSyncHistory(agencyId, feed.id, showHistory ? 5 : 0);

  const statusConfig = getFeedStatusConfig(feed.status);

  const handleTogglePause = async () => {
    await updateMutation.mutateAsync({
      status: feed.status === 'active' ? 'paused' : 'active',
    });
  };

  const handleDelete = async () => {
    if (confirm('Are you sure you want to remove this feed?')) {
      await deleteMutation.mutateAsync(feed.id);
    }
  };

  return (
    <div className="feed-card">
      <div className="card-header">
        <div className="feed-info">
          <div className="format-badge">{getFormatLabel(feed.format)}</div>
          <div>
            <h3>{feed.name}</h3>
            <p className="feed-url">{feed.url}</p>
          </div>
        </div>
        <span
          className="status-badge"
          style={{ background: statusConfig.bg, color: statusConfig.color }}
        >
          {statusConfig.label}
        </span>
      </div>

      <div className="card-body">
        <div className="stats-row">
          <div className="stat">
            <span className="stat-value">{feed.totalListings}</span>
            <span className="stat-label">Listings</span>
          </div>
          <div className="stat">
            <span className="stat-value">{formatFrequency(feed.syncFrequency)}</span>
            <span className="stat-label">Sync Frequency</span>
          </div>
          {feed.lastFetchAt && (
            <div className="stat">
              <span className="stat-value">{new Date(feed.lastFetchAt).toLocaleDateString()}</span>
              <span className="stat-label">Last Sync</span>
            </div>
          )}
        </div>
      </div>

      <div className="card-actions">
        <button
          type="button"
          className="action-button sync"
          onClick={() => syncMutation.mutate()}
          disabled={syncMutation.isPending || feed.status === 'error'}
        >
          {syncMutation.isPending ? 'Syncing...' : 'Sync Now'}
        </button>
        <button
          type="button"
          className="action-button pause"
          onClick={handleTogglePause}
          disabled={updateMutation.isPending}
        >
          {feed.status === 'active' ? 'Pause' : 'Resume'}
        </button>
        <button
          type="button"
          className="action-button history"
          onClick={() => setShowHistory(!showHistory)}
        >
          History
        </button>
        <button
          type="button"
          className="action-button delete"
          onClick={handleDelete}
          disabled={deleteMutation.isPending}
        >
          Remove
        </button>
      </div>

      {showHistory && history && (
        <div className="history-section">
          <h4>Sync History</h4>
          {history.length === 0 ? (
            <p className="no-history">No sync history yet</p>
          ) : (
            <div className="history-list">
              {history.map((item) => (
                <div key={item.id} className="history-item">
                  <span className="history-date">{new Date(item.startedAt).toLocaleString()}</span>
                  <span className={`history-status ${item.status}`}>{item.status}</span>
                  <span className="history-stats">
                    {item.recordsCreated} created, {item.recordsUpdated} updated
                  </span>
                </div>
              ))}
            </div>
          )}
        </div>
      )}

      <style jsx>{`
        .feed-card {
          background: #fff;
          border: 1px solid #e5e7eb;
          border-radius: 12px;
          overflow: hidden;
        }

        .card-header {
          display: flex;
          justify-content: space-between;
          align-items: flex-start;
          padding: 20px;
          border-bottom: 1px solid #f3f4f6;
        }

        .feed-info {
          display: flex;
          gap: 12px;
        }

        .format-badge {
          padding: 6px 10px;
          background: #e5e7eb;
          border-radius: 6px;
          font-size: 11px;
          font-weight: 600;
          color: #374151;
          text-transform: uppercase;
        }

        h3 {
          font-size: 1rem;
          color: #111827;
          margin: 0 0 4px;
        }

        .feed-url {
          font-size: 13px;
          color: #6b7280;
          margin: 0;
          max-width: 400px;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
        }

        .status-badge {
          padding: 4px 10px;
          border-radius: 12px;
          font-size: 12px;
          font-weight: 500;
        }

        .card-body {
          padding: 16px 20px;
        }

        .stats-row {
          display: flex;
          gap: 32px;
        }

        .stat {
          display: flex;
          flex-direction: column;
        }

        .stat-value {
          font-size: 1.25rem;
          font-weight: 600;
          color: #111827;
        }

        .stat-label {
          font-size: 12px;
          color: #6b7280;
        }

        .card-actions {
          display: flex;
          gap: 8px;
          padding: 16px 20px;
          background: #f9fafb;
          border-top: 1px solid #f3f4f6;
        }

        .action-button {
          padding: 8px 12px;
          border-radius: 6px;
          font-size: 13px;
          font-weight: 500;
          cursor: pointer;
        }

        .action-button.sync {
          background: #2563eb;
          color: #fff;
          border: none;
        }

        .action-button.sync:disabled {
          opacity: 0.5;
        }

        .action-button.pause,
        .action-button.history {
          background: #fff;
          border: 1px solid #d1d5db;
          color: #374151;
        }

        .action-button.delete {
          background: #fff;
          border: 1px solid #fecaca;
          color: #dc2626;
          margin-left: auto;
        }

        .action-button.delete:hover {
          background: #fef2f2;
        }

        .history-section {
          padding: 16px 20px;
          border-top: 1px solid #e5e7eb;
          background: #fafafa;
        }

        .history-section h4 {
          font-size: 14px;
          color: #374151;
          margin: 0 0 12px;
        }

        .history-list {
          display: flex;
          flex-direction: column;
          gap: 8px;
        }

        .history-item {
          display: flex;
          align-items: center;
          gap: 12px;
          padding: 10px 12px;
          background: #fff;
          border-radius: 6px;
          font-size: 13px;
        }

        .history-date {
          color: #374151;
        }

        .history-status {
          padding: 2px 8px;
          border-radius: 4px;
          font-size: 11px;
          font-weight: 500;
        }

        .history-status.completed {
          background: #d1fae5;
          color: #059669;
        }

        .history-status.failed {
          background: #fee2e2;
          color: #dc2626;
        }

        .history-status.running {
          background: #dbeafe;
          color: #2563eb;
        }

        .history-stats {
          color: #6b7280;
          margin-left: auto;
        }

        .no-history {
          color: #6b7280;
          font-size: 13px;
          text-align: center;
          padding: 12px;
          margin: 0;
        }
      `}</style>
    </div>
  );
}

function AddFeedModal({ agencyId, onClose }: { agencyId: string; onClose: () => void }) {
  const [step, setStep] = useState<'url' | 'preview' | 'mapping'>('url');
  const [url, setUrl] = useState('');
  const [name, setName] = useState('');
  const [frequency, setFrequency] = useState<SyncFrequency>('daily');
  const [fieldMapping, setFieldMapping] = useState<FeedFieldMapping>(DEFAULT_FIELD_MAPPING);
  const [previewData, setPreviewData] = useState<FeedPreview | null>(null);

  const previewMutation = useFeedPreview(agencyId);
  const createMutation = useCreateFeedSource(agencyId);

  const handlePreview = async () => {
    try {
      const result = await previewMutation.mutateAsync(url);
      setPreviewData(result);
      if (result.success) {
        setStep('preview');
      }
    } catch {
      // Error handled by mutation
    }
  };

  const handleCreate = async () => {
    await createMutation.mutateAsync({
      name: name || 'Property Feed',
      url,
      format: previewData?.format,
      fieldMapping,
      syncFrequency: frequency,
    });
    onClose();
  };

  return (
    <div
      className="modal-overlay"
      onClick={onClose}
      onKeyDown={(e) => e.key === 'Escape' && onClose()}
      role="dialog"
      aria-modal="true"
      aria-labelledby="modal-title"
    >
      <div className="modal-content" onClick={(e) => e.stopPropagation()} onKeyDown={() => {}}>
        <div className="modal-header">
          <h2 id="modal-title">
            {step === 'url' && 'Add Feed Source'}
            {step === 'preview' && 'Feed Preview'}
            {step === 'mapping' && 'Field Mapping'}
          </h2>
          <button type="button" className="close-button" onClick={onClose} aria-label="Close modal">
            <svg
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              aria-hidden="true"
            >
              <line x1="18" y1="6" x2="6" y2="18" />
              <line x1="6" y1="6" x2="18" y2="18" />
            </svg>
          </button>
        </div>

        <div className="modal-body">
          {step === 'url' && (
            <div className="url-form">
              <div className="form-group">
                <label htmlFor="feed-url">Feed URL</label>
                <input
                  id="feed-url"
                  type="url"
                  value={url}
                  onChange={(e) => setUrl(e.target.value)}
                  placeholder="https://example.com/feed.xml"
                />
                <p className="hint">Supports XML, RSS, Atom, and JSON feeds</p>
              </div>

              <div className="form-group">
                <label htmlFor="feed-name">Feed Name (Optional)</label>
                <input
                  id="feed-name"
                  type="text"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder="My Property Feed"
                />
              </div>

              {previewMutation.error && (
                <div className="error-message">
                  Failed to fetch feed. Please check the URL and try again.
                </div>
              )}
            </div>
          )}

          {step === 'preview' && previewData && (
            <div className="preview-section">
              <div className="preview-info">
                <div className="info-card">
                  <span className="info-label">Format</span>
                  <span className="info-value">{getFormatLabel(previewData.format)}</span>
                </div>
                <div className="info-card">
                  <span className="info-label">Items Found</span>
                  <span className="info-value">{previewData.totalItems}</span>
                </div>
              </div>

              <div className="available-fields">
                <h4>Available Fields</h4>
                <div className="fields-list">
                  {previewData.availableFields.map((field) => (
                    <span key={field} className="field-tag">
                      {field}
                    </span>
                  ))}
                </div>
              </div>

              {previewData.sampleItems.length > 0 && (
                <div className="sample-items">
                  <h4>Sample Data</h4>
                  <pre>{JSON.stringify(previewData.sampleItems[0], null, 2)}</pre>
                </div>
              )}

              <div className="form-group">
                <label htmlFor="sync-freq">Sync Frequency</label>
                <select
                  id="sync-freq"
                  value={frequency}
                  onChange={(e) => setFrequency(e.target.value as SyncFrequency)}
                >
                  {FREQUENCY_OPTIONS.map((opt) => (
                    <option key={opt.value} value={opt.value}>
                      {opt.label}
                    </option>
                  ))}
                </select>
              </div>
            </div>
          )}

          {step === 'mapping' && (
            <div className="mapping-form">
              <p className="mapping-hint">Map feed fields to listing properties</p>
              {Object.entries(DEFAULT_FIELD_MAPPING).map(([localField, defaultRemote]) => (
                <div key={localField} className="mapping-row">
                  <label htmlFor={`map-${localField}`}>{formatFieldLabel(localField)}</label>
                  <input
                    id={`map-${localField}`}
                    type="text"
                    value={fieldMapping[localField] || ''}
                    onChange={(e) =>
                      setFieldMapping((prev) => ({ ...prev, [localField]: e.target.value }))
                    }
                    placeholder={defaultRemote}
                  />
                </div>
              ))}
            </div>
          )}
        </div>

        <div className="modal-footer">
          {step === 'url' && (
            <button
              type="button"
              className="primary"
              onClick={handlePreview}
              disabled={!url || previewMutation.isPending}
            >
              {previewMutation.isPending ? 'Fetching...' : 'Fetch Feed'}
            </button>
          )}

          {step === 'preview' && (
            <>
              <button type="button" className="secondary" onClick={() => setStep('url')}>
                Back
              </button>
              <button type="button" className="primary" onClick={() => setStep('mapping')}>
                Configure Mapping
              </button>
            </>
          )}

          {step === 'mapping' && (
            <>
              <button type="button" className="secondary" onClick={() => setStep('preview')}>
                Back
              </button>
              <button
                type="button"
                className="primary"
                onClick={handleCreate}
                disabled={createMutation.isPending}
              >
                {createMutation.isPending ? 'Creating...' : 'Create Feed'}
              </button>
            </>
          )}
        </div>
      </div>

      <style jsx>{`
        .modal-overlay {
          position: fixed;
          inset: 0;
          background: rgba(0, 0, 0, 0.5);
          display: flex;
          align-items: center;
          justify-content: center;
          z-index: 100;
        }

        .modal-content {
          background: #fff;
          border-radius: 16px;
          width: 90%;
          max-width: 600px;
          max-height: 90vh;
          overflow-y: auto;
        }

        .modal-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          padding: 20px 24px;
          border-bottom: 1px solid #e5e7eb;
        }

        .modal-header h2 {
          font-size: 1.25rem;
          color: #111827;
          margin: 0;
        }

        .close-button {
          background: none;
          border: none;
          padding: 4px;
          cursor: pointer;
          color: #6b7280;
        }

        .modal-body {
          padding: 24px;
        }

        .url-form,
        .preview-section,
        .mapping-form {
          display: flex;
          flex-direction: column;
          gap: 16px;
        }

        .form-group {
          display: flex;
          flex-direction: column;
          gap: 6px;
        }

        .form-group label {
          font-size: 13px;
          font-weight: 500;
          color: #374151;
        }

        .form-group input,
        .form-group select {
          padding: 10px 12px;
          border: 1px solid #d1d5db;
          border-radius: 8px;
          font-size: 14px;
        }

        .form-group input:focus,
        .form-group select:focus {
          outline: none;
          border-color: #2563eb;
          box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
        }

        .hint {
          font-size: 12px;
          color: #6b7280;
          margin: 0;
        }

        .error-message {
          padding: 12px 16px;
          background: #fee2e2;
          color: #b91c1c;
          border-radius: 8px;
          font-size: 14px;
        }

        .preview-info {
          display: flex;
          gap: 16px;
        }

        .info-card {
          flex: 1;
          padding: 16px;
          background: #f9fafb;
          border-radius: 8px;
          text-align: center;
        }

        .info-label {
          display: block;
          font-size: 12px;
          color: #6b7280;
          margin-bottom: 4px;
        }

        .info-value {
          font-size: 1.25rem;
          font-weight: 600;
          color: #111827;
        }

        .available-fields h4,
        .sample-items h4 {
          font-size: 14px;
          color: #374151;
          margin: 0 0 12px;
        }

        .fields-list {
          display: flex;
          flex-wrap: wrap;
          gap: 8px;
        }

        .field-tag {
          padding: 4px 10px;
          background: #e5e7eb;
          border-radius: 4px;
          font-size: 12px;
          color: #374151;
        }

        .sample-items pre {
          padding: 12px;
          background: #f9fafb;
          border-radius: 8px;
          font-size: 11px;
          overflow-x: auto;
          max-height: 150px;
        }

        .mapping-hint {
          color: #6b7280;
          font-size: 14px;
          margin: 0 0 8px;
        }

        .mapping-row {
          display: flex;
          align-items: center;
          gap: 12px;
        }

        .mapping-row label {
          width: 100px;
          font-size: 13px;
          font-weight: 500;
          color: #374151;
        }

        .mapping-row input {
          flex: 1;
          padding: 8px 12px;
          border: 1px solid #d1d5db;
          border-radius: 6px;
          font-size: 13px;
        }

        .modal-footer {
          display: flex;
          justify-content: flex-end;
          gap: 12px;
          padding: 16px 24px;
          border-top: 1px solid #e5e7eb;
          background: #f9fafb;
        }

        .modal-footer button {
          padding: 10px 20px;
          border-radius: 8px;
          font-size: 14px;
          font-weight: 500;
          cursor: pointer;
        }

        .secondary {
          background: #fff;
          border: 1px solid #d1d5db;
          color: #374151;
        }

        .primary {
          background: #2563eb;
          border: none;
          color: #fff;
        }

        .primary:disabled {
          opacity: 0.5;
          cursor: not-allowed;
        }
      `}</style>
    </div>
  );
}

function FeedsSkeleton() {
  return (
    <div className="skeleton-list">
      {[1, 2].map((i) => (
        <div key={`skel-${i}`} className="skeleton-card" />
      ))}
      <style jsx>{`
        .skeleton-list {
          display: flex;
          flex-direction: column;
          gap: 16px;
        }
        .skeleton-card {
          height: 180px;
          background: #e5e7eb;
          border-radius: 12px;
        }
      `}</style>
    </div>
  );
}

function EmptyState({ onAdd }: { onAdd: () => void }) {
  return (
    <div className="empty-state">
      <svg
        width="64"
        height="64"
        viewBox="0 0 24 24"
        fill="none"
        stroke="#9ca3af"
        strokeWidth="1.5"
        aria-hidden="true"
      >
        <path d="M4 11a9 9 0 0 1 9 9" />
        <path d="M4 4a16 16 0 0 1 16 16" />
        <circle cx="5" cy="19" r="2" />
      </svg>
      <h3>No feed sources</h3>
      <p>Add an XML, RSS, or JSON feed to import listings</p>
      <button type="button" onClick={onAdd}>
        Add Your First Feed
      </button>
      <style jsx>{`
        .empty-state {
          display: flex;
          flex-direction: column;
          align-items: center;
          padding: 64px 24px;
          text-align: center;
          background: #fff;
          border: 1px solid #e5e7eb;
          border-radius: 12px;
        }
        h3 {
          font-size: 1.25rem;
          color: #111827;
          margin: 24px 0 8px;
        }
        p {
          color: #6b7280;
          margin: 0 0 24px;
        }
        button {
          padding: 12px 24px;
          background: #2563eb;
          color: #fff;
          border: none;
          border-radius: 8px;
          font-weight: 500;
          cursor: pointer;
        }
      `}</style>
    </div>
  );
}

function getFeedStatusConfig(status: FeedSource['status']) {
  const configs = {
    active: { label: 'Active', color: '#059669', bg: '#d1fae5' },
    paused: { label: 'Paused', color: '#6b7280', bg: '#e5e7eb' },
    error: { label: 'Error', color: '#dc2626', bg: '#fee2e2' },
  };
  return configs[status];
}

function getFormatLabel(format: FeedFormat): string {
  const labels: Record<FeedFormat, string> = {
    xml: 'XML',
    rss: 'RSS',
    atom: 'Atom',
    json: 'JSON',
  };
  return labels[format] || format.toUpperCase();
}

function formatFrequency(frequency: string): string {
  const labels: Record<string, string> = {
    manual: 'Manual',
    hourly: 'Hourly',
    daily: 'Daily',
    weekly: 'Weekly',
  };
  return labels[frequency] || frequency;
}

function formatFieldLabel(field: string): string {
  return field.replace(/([A-Z])/g, ' $1').replace(/^./, (s) => s.toUpperCase());
}
