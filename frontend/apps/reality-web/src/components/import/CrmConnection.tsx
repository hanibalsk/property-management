/**
 * CrmConnection Component
 *
 * Connect and manage CRM systems (Epic 46, Story 46.2).
 */

'use client';

import type {
  CrmConnection as CrmConnectionType,
  CrmFieldMapping,
  CrmProvider,
} from '@ppt/reality-api-client';
import {
  useCreateCrmConnection,
  useCrmConnections,
  useDeleteCrmConnection,
  useMyAgency,
  useSyncCrmConnection,
  useTestCrmConnection,
} from '@ppt/reality-api-client';
import { useState } from 'react';

const CRM_PROVIDERS: { id: CrmProvider; name: string; logo: string }[] = [
  { id: 'salesforce', name: 'Salesforce', logo: '🔵' },
  { id: 'hubspot', name: 'HubSpot', logo: '🟠' },
  { id: 'pipedrive', name: 'Pipedrive', logo: '🟢' },
  { id: 'zoho', name: 'Zoho CRM', logo: '🔴' },
  { id: 'custom', name: 'Custom API', logo: '⚙️' },
];

const DEFAULT_FIELD_MAPPING: CrmFieldMapping = {
  title: 'name',
  description: 'description',
  price: 'amount',
  propertyType: 'property_type',
  address: 'address',
  city: 'city',
  rooms: 'bedrooms',
  size: 'square_feet',
};

export function CrmConnection() {
  const [showModal, setShowModal] = useState(false);
  const [selectedProvider, setSelectedProvider] = useState<CrmProvider | null>(null);

  const { data: agency } = useMyAgency();
  const { data: connections, isLoading } = useCrmConnections(agency?.id || '');

  return (
    <div className="crm-connection">
      <div className="header">
        <div>
          <h2>CRM Connections</h2>
          <p className="subtitle">Connect your CRM to automatically sync property listings</p>
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
          Add Connection
        </button>
      </div>

      {isLoading ? (
        <ConnectionsSkeleton />
      ) : connections?.length === 0 ? (
        <EmptyState onAdd={() => setShowModal(true)} />
      ) : (
        <div className="connections-grid">
          {connections?.map((conn) => (
            <ConnectionCard key={conn.id} connection={conn} agencyId={agency?.id || ''} />
          ))}
        </div>
      )}

      {showModal && (
        <AddConnectionModal
          agencyId={agency?.id || ''}
          selectedProvider={selectedProvider}
          onSelectProvider={setSelectedProvider}
          onClose={() => {
            setShowModal(false);
            setSelectedProvider(null);
          }}
        />
      )}

      <style jsx>{`
        .crm-connection {
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

        .connections-grid {
          display: grid;
          grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
          gap: 20px;
        }
      `}</style>
    </div>
  );
}

function ConnectionCard({
  connection,
  agencyId,
}: { connection: CrmConnectionType; agencyId: string }) {
  const deleteMutation = useDeleteCrmConnection(agencyId);
  const syncMutation = useSyncCrmConnection(agencyId, connection.id);

  const provider = CRM_PROVIDERS.find((p) => p.id === connection.provider);
  const statusConfig = getStatusConfig(connection.status);

  const handleDelete = async () => {
    if (confirm('Are you sure you want to remove this connection?')) {
      await deleteMutation.mutateAsync(connection.id);
    }
  };

  return (
    <div className="connection-card">
      <div className="card-header">
        <div className="provider-info">
          <span className="provider-logo">{provider?.logo}</span>
          <div>
            <h3>{connection.name}</h3>
            <span className="provider-name">{provider?.name}</span>
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
        {connection.lastSyncAt && (
          <div className="sync-info">
            <span className="label">Last sync:</span>
            <span className="value">{new Date(connection.lastSyncAt).toLocaleString()}</span>
          </div>
        )}
        {connection.nextSyncAt && (
          <div className="sync-info">
            <span className="label">Next sync:</span>
            <span className="value">{new Date(connection.nextSyncAt).toLocaleString()}</span>
          </div>
        )}
        <div className="sync-info">
          <span className="label">Frequency:</span>
          <span className="value">{formatFrequency(connection.syncFrequency)}</span>
        </div>
      </div>

      <div className="card-actions">
        <button
          type="button"
          className="action-button sync"
          onClick={() => syncMutation.mutate()}
          disabled={syncMutation.isPending || connection.status === 'syncing'}
        >
          {syncMutation.isPending ? 'Syncing...' : 'Sync Now'}
        </button>
        <button type="button" className="action-button settings">
          Settings
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

      <style jsx>{`
        .connection-card {
          background: #fff;
          border: 1px solid #e5e7eb;
          border-radius: 12px;
          overflow: hidden;
        }

        .card-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          padding: 20px;
          border-bottom: 1px solid #f3f4f6;
        }

        .provider-info {
          display: flex;
          align-items: center;
          gap: 12px;
        }

        .provider-logo {
          font-size: 24px;
        }

        h3 {
          font-size: 1rem;
          color: #111827;
          margin: 0;
        }

        .provider-name {
          font-size: 13px;
          color: #6b7280;
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

        .sync-info {
          display: flex;
          justify-content: space-between;
          margin-bottom: 8px;
          font-size: 13px;
        }

        .sync-info:last-child {
          margin-bottom: 0;
        }

        .sync-info .label {
          color: #6b7280;
        }

        .sync-info .value {
          color: #374151;
        }

        .card-actions {
          display: flex;
          gap: 8px;
          padding: 16px 20px;
          background: #f9fafb;
          border-top: 1px solid #f3f4f6;
        }

        .action-button {
          flex: 1;
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

        .action-button.settings {
          background: #fff;
          border: 1px solid #d1d5db;
          color: #374151;
        }

        .action-button.delete {
          background: #fff;
          border: 1px solid #fecaca;
          color: #dc2626;
        }

        .action-button.delete:hover {
          background: #fef2f2;
        }
      `}</style>
    </div>
  );
}

function AddConnectionModal({
  agencyId,
  selectedProvider,
  onSelectProvider,
  onClose,
}: {
  agencyId: string;
  selectedProvider: CrmProvider | null;
  onSelectProvider: (provider: CrmProvider) => void;
  onClose: () => void;
}) {
  const [step, setStep] = useState<'select' | 'configure' | 'mapping'>('select');
  const [name, setName] = useState('');
  const [apiKey, setApiKey] = useState('');
  const [instanceUrl, setInstanceUrl] = useState('');
  const [fieldMapping, setFieldMapping] = useState<CrmFieldMapping>(DEFAULT_FIELD_MAPPING);
  const [testResult, setTestResult] = useState<{ success: boolean; message: string } | null>(null);

  const testMutation = useTestCrmConnection(agencyId);
  const createMutation = useCreateCrmConnection(agencyId);

  const handleTest = async () => {
    if (!selectedProvider) return;
    try {
      const result = await testMutation.mutateAsync({
        provider: selectedProvider,
        apiKey,
        instanceUrl: selectedProvider === 'salesforce' ? instanceUrl : undefined,
      });
      setTestResult(result);
      if (result.success) {
        setStep('mapping');
      }
    } catch (error) {
      setTestResult({
        success: false,
        message: error instanceof Error ? error.message : 'Connection failed',
      });
    }
  };

  const handleCreate = async () => {
    if (!selectedProvider) return;
    await createMutation.mutateAsync({
      provider: selectedProvider,
      name: name || `${CRM_PROVIDERS.find((p) => p.id === selectedProvider)?.name} Connection`,
      credentials: {
        apiKey,
        instanceUrl: selectedProvider === 'salesforce' ? instanceUrl : undefined,
      },
      fieldMapping,
      syncFrequency: 'daily',
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
            {step === 'select' && 'Select CRM Provider'}
            {step === 'configure' && 'Configure Connection'}
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
          {step === 'select' && (
            <div className="provider-grid">
              {CRM_PROVIDERS.map((provider) => (
                <button
                  key={provider.id}
                  type="button"
                  className={`provider-card ${selectedProvider === provider.id ? 'selected' : ''}`}
                  onClick={() => {
                    onSelectProvider(provider.id);
                    setStep('configure');
                  }}
                >
                  <span className="provider-logo">{provider.logo}</span>
                  <span className="provider-name">{provider.name}</span>
                </button>
              ))}
            </div>
          )}

          {step === 'configure' && selectedProvider && (
            <div className="configure-form">
              <div className="form-group">
                <label htmlFor="conn-name">Connection Name (Optional)</label>
                <input
                  id="conn-name"
                  type="text"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder={`My ${CRM_PROVIDERS.find((p) => p.id === selectedProvider)?.name}`}
                />
              </div>

              <div className="form-group">
                <label htmlFor="api-key">API Key</label>
                <input
                  id="api-key"
                  type="password"
                  value={apiKey}
                  onChange={(e) => setApiKey(e.target.value)}
                  placeholder="Enter your API key"
                />
              </div>

              {selectedProvider === 'salesforce' && (
                <div className="form-group">
                  <label htmlFor="instance-url">Instance URL</label>
                  <input
                    id="instance-url"
                    type="url"
                    value={instanceUrl}
                    onChange={(e) => setInstanceUrl(e.target.value)}
                    placeholder="https://yourorg.salesforce.com"
                  />
                </div>
              )}

              {testResult && (
                <div className={`test-result ${testResult.success ? 'success' : 'error'}`}>
                  {testResult.success ? '✓' : '✕'} {testResult.message}
                </div>
              )}
            </div>
          )}

          {step === 'mapping' && (
            <div className="mapping-form">
              <p className="mapping-hint">Map your CRM fields to listing properties</p>
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
          {step === 'configure' && (
            <>
              <button type="button" className="secondary" onClick={() => setStep('select')}>
                Back
              </button>
              <button
                type="button"
                className="primary"
                onClick={handleTest}
                disabled={!apiKey || testMutation.isPending}
              >
                {testMutation.isPending ? 'Testing...' : 'Test Connection'}
              </button>
            </>
          )}

          {step === 'mapping' && (
            <>
              <button type="button" className="secondary" onClick={() => setStep('configure')}>
                Back
              </button>
              <button
                type="button"
                className="primary"
                onClick={handleCreate}
                disabled={createMutation.isPending}
              >
                {createMutation.isPending ? 'Creating...' : 'Create Connection'}
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
          max-width: 560px;
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

        .provider-grid {
          display: grid;
          grid-template-columns: repeat(2, 1fr);
          gap: 12px;
        }

        .provider-card {
          display: flex;
          flex-direction: column;
          align-items: center;
          gap: 8px;
          padding: 24px;
          border: 2px solid #e5e7eb;
          border-radius: 12px;
          background: #fff;
          cursor: pointer;
          transition: all 0.2s;
        }

        .provider-card:hover {
          border-color: #2563eb;
        }

        .provider-card.selected {
          border-color: #2563eb;
          background: #eff6ff;
        }

        .provider-card .provider-logo {
          font-size: 32px;
        }

        .provider-card .provider-name {
          font-size: 14px;
          font-weight: 500;
          color: #374151;
        }

        .configure-form,
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

        .form-group input {
          padding: 10px 12px;
          border: 1px solid #d1d5db;
          border-radius: 8px;
          font-size: 14px;
        }

        .form-group input:focus {
          outline: none;
          border-color: #2563eb;
          box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
        }

        .test-result {
          padding: 12px 16px;
          border-radius: 8px;
          font-size: 14px;
        }

        .test-result.success {
          background: #d1fae5;
          color: #047857;
        }

        .test-result.error {
          background: #fee2e2;
          color: #b91c1c;
        }

        .mapping-hint {
          color: #6b7280;
          margin: 0 0 8px;
          font-size: 14px;
        }

        .mapping-row {
          display: flex;
          align-items: center;
          gap: 12px;
        }

        .mapping-row label {
          width: 120px;
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

function ConnectionsSkeleton() {
  return (
    <div className="skeleton-grid">
      {[1, 2].map((i) => (
        <div key={`skel-${i}`} className="skeleton-card" />
      ))}
      <style jsx>{`
        .skeleton-grid {
          display: grid;
          grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
          gap: 20px;
        }
        .skeleton-card {
          height: 200px;
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
        <path d="M20 7h-4m0 0V3m0 4l4-4M4 17h4m0 0v4m0-4l-4 4" />
        <circle cx="12" cy="12" r="3" />
      </svg>
      <h3>No CRM connections</h3>
      <p>Connect your CRM to automatically sync listings</p>
      <button type="button" onClick={onAdd}>
        Add Your First Connection
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

function getStatusConfig(status: CrmConnectionType['status']) {
  const configs = {
    connected: { label: 'Connected', color: '#059669', bg: '#d1fae5' },
    disconnected: { label: 'Disconnected', color: '#6b7280', bg: '#e5e7eb' },
    error: { label: 'Error', color: '#dc2626', bg: '#fee2e2' },
    syncing: { label: 'Syncing', color: '#2563eb', bg: '#dbeafe' },
  };
  return configs[status];
}

function formatFrequency(frequency: string): string {
  const labels: Record<string, string> = {
    manual: 'Manual',
    hourly: 'Every hour',
    daily: 'Daily',
    weekly: 'Weekly',
  };
  return labels[frequency] || frequency;
}

function formatFieldLabel(field: string): string {
  return field.replace(/([A-Z])/g, ' $1').replace(/^./, (s) => s.toUpperCase());
}
