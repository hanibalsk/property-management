/**
 * Accounting Exports List Component
 *
 * Displays and manages accounting exports (Story 61.2).
 */

import type {
  AccountingExport,
  AccountingSystem,
  CreateAccountingExport,
  ExportType,
} from '@ppt/api-client';
import {
  getAccountingExportDownloadUrl,
  useAccountingExports,
  useAccountingSettings,
  useCreateAccountingExport,
} from '@ppt/api-client';
import { useState } from 'react';

interface AccountingExportsListProps {
  organizationId: string;
}

const systemInfo: Record<AccountingSystem, { name: string; format: string; description: string }> =
  {
    pohoda: {
      name: 'POHODA',
      format: 'XML',
      description: 'Czech accounting software by Stormware',
    },
    money_s3: {
      name: 'Money S3',
      format: 'CSV',
      description: 'Czech and Slovak accounting software',
    },
    quickbooks: {
      name: 'QuickBooks',
      format: 'IIF/QBO',
      description: 'Intuit accounting solution',
    },
    xero: {
      name: 'Xero',
      format: 'CSV',
      description: 'Cloud-based accounting platform',
    },
  };

const statusColors: Record<string, string> = {
  pending: 'bg-yellow-100 text-yellow-800',
  processing: 'bg-blue-100 text-blue-800',
  completed: 'bg-green-100 text-green-800',
  failed: 'bg-red-100 text-red-800',
};

export function AccountingExportsList({ organizationId }: AccountingExportsListProps) {
  const [selectedSystem, setSelectedSystem] = useState<AccountingSystem | null>(null);
  const [showCreateForm, setShowCreateForm] = useState(false);

  const { data: exports, isLoading } = useAccountingExports(organizationId, {
    systemType: selectedSystem ?? undefined,
    limit: 20,
  });

  if (isLoading && !showCreateForm) {
    return (
      <div className="rounded-lg border bg-card p-6">
        <h3 className="text-lg font-semibold">Accounting Exports</h3>
        <p className="text-muted-foreground">Loading...</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* System Selection */}
      <div className="rounded-lg border bg-card p-6">
        <div className="flex items-center gap-4 mb-6">
          <div className="flex h-12 w-12 items-center justify-center rounded-full bg-green-100">
            <span className="text-green-600 font-bold text-xl">$</span>
          </div>
          <div>
            <h3 className="text-lg font-medium">Accounting Exports</h3>
            <p className="text-sm text-muted-foreground">
              Export financial data to POHODA, Money S3, QuickBooks, and more
            </p>
          </div>
        </div>

        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4 mb-6">
          {(
            Object.entries(systemInfo) as [
              AccountingSystem,
              (typeof systemInfo)[AccountingSystem],
            ][]
          ).map(([system, info]) => (
            <button
              type="button"
              key={system}
              onClick={() => {
                setSelectedSystem(system);
                setShowCreateForm(true);
              }}
              className={`rounded-lg border p-4 text-left hover:border-primary cursor-pointer transition-colors ${
                selectedSystem === system ? 'border-primary bg-primary/5' : ''
              }`}
            >
              <div className="flex items-center gap-3 mb-2">
                <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-muted">
                  <span className="font-bold text-sm">$</span>
                </div>
                <div>
                  <div className="font-medium">{info.name}</div>
                  <div className="text-xs text-muted-foreground">{info.format}</div>
                </div>
              </div>
              <p className="text-sm text-muted-foreground">{info.description}</p>
            </button>
          ))}
        </div>
      </div>

      {/* Create Export Form */}
      {showCreateForm && selectedSystem && (
        <CreateExportForm
          organizationId={organizationId}
          systemType={selectedSystem}
          onClose={() => {
            setShowCreateForm(false);
            setSelectedSystem(null);
          }}
        />
      )}

      {/* Export History */}
      <div className="rounded-lg border bg-card p-6">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-semibold">Export History</h3>
          <select
            value={selectedSystem ?? ''}
            onChange={(e) => setSelectedSystem((e.target.value as AccountingSystem) || null)}
            className="px-3 py-1 text-sm border rounded-md bg-background"
          >
            <option value="">All Systems</option>
            {Object.entries(systemInfo).map(([system, info]) => (
              <option key={system} value={system}>
                {info.name}
              </option>
            ))}
          </select>
        </div>

        {exports?.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-8 text-center">
            <p className="text-muted-foreground">No exports yet</p>
            <p className="text-sm text-muted-foreground">
              Select an accounting system above to create your first export
            </p>
          </div>
        ) : (
          <div className="space-y-3">
            {exports?.map((exportItem: AccountingExport) => (
              <ExportListItem key={exportItem.id} exportItem={exportItem} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

interface CreateExportFormProps {
  organizationId: string;
  systemType: AccountingSystem;
  onClose: () => void;
}

function CreateExportForm({ organizationId, systemType, onClose }: CreateExportFormProps) {
  const [exportType, setExportType] = useState<ExportType>('invoices');
  const [periodStart, setPeriodStart] = useState(
    new Date(new Date().getFullYear(), new Date().getMonth(), 1).toISOString().split('T')[0]
  );
  const [periodEnd, setPeriodEnd] = useState(new Date().toISOString().split('T')[0]);

  const { data: settings } = useAccountingSettings(organizationId, systemType);
  const createExport = useCreateAccountingExport(organizationId);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const data: CreateAccountingExport = {
      systemType,
      exportType,
      periodStart,
      periodEnd,
    };

    try {
      await createExport.mutateAsync(data);
      onClose();
    } catch (error) {
      console.error('Failed to create export:', error);
    }
  };

  const info = systemInfo[systemType];

  return (
    <div className="rounded-lg border bg-card p-6">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold">Create {info.name} Export</h3>
        <button
          type="button"
          onClick={onClose}
          className="text-muted-foreground hover:text-foreground"
        >
          X
        </button>
      </div>

      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label htmlFor="export-type" className="block text-sm font-medium mb-1">
            Export Type
          </label>
          <select
            id="export-type"
            value={exportType}
            onChange={(e) => setExportType(e.target.value as ExportType)}
            className="w-full px-3 py-2 border rounded-md bg-background"
          >
            <option value="invoices">Invoices</option>
            <option value="payments">Payments</option>
            <option value="full">Full Export (Invoices + Payments)</option>
          </select>
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div>
            <label htmlFor="period-start" className="block text-sm font-medium mb-1">
              Period Start
            </label>
            <input
              id="period-start"
              type="date"
              value={periodStart}
              onChange={(e) => setPeriodStart(e.target.value)}
              className="w-full px-3 py-2 border rounded-md bg-background"
            />
          </div>
          <div>
            <label htmlFor="period-end" className="block text-sm font-medium mb-1">
              Period End
            </label>
            <input
              id="period-end"
              type="date"
              value={periodEnd}
              onChange={(e) => setPeriodEnd(e.target.value)}
              className="w-full px-3 py-2 border rounded-md bg-background"
            />
          </div>
        </div>

        {settings && (
          <div className="rounded-md bg-muted/50 p-3 text-sm">
            <p className="font-medium mb-1">Export Settings</p>
            {settings.defaultCostCenter && (
              <p className="text-muted-foreground">
                Default Cost Center: {settings.defaultCostCenter}
              </p>
            )}
            {settings.autoExportEnabled && (
              <p className="text-muted-foreground">
                Auto-export: {settings.autoExportSchedule ?? 'Enabled'}
              </p>
            )}
          </div>
        )}

        <div className="flex justify-end gap-2">
          <button
            type="button"
            onClick={onClose}
            className="px-4 py-2 text-sm border rounded-md hover:bg-muted"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={createExport.isPending}
            className="px-4 py-2 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50"
          >
            {createExport.isPending ? 'Creating...' : `Export to ${info.format}`}
          </button>
        </div>
      </form>
    </div>
  );
}

interface ExportListItemProps {
  exportItem: AccountingExport;
}

function ExportListItem({ exportItem }: ExportListItemProps) {
  const info = systemInfo[exportItem.systemType as AccountingSystem];

  return (
    <div className="flex items-center justify-between rounded-lg border p-4">
      <div className="flex items-center gap-4">
        <div className="flex h-10 w-10 items-center justify-center rounded-full bg-muted">
          <span className="font-bold text-sm">$</span>
        </div>
        <div>
          <div className="flex items-center gap-2">
            <span className="font-medium">{info?.name ?? exportItem.systemType}</span>
            <span
              className={`inline-flex items-center px-2 py-0.5 rounded text-xs font-medium ${
                statusColors[exportItem.status]
              }`}
            >
              {exportItem.status}
            </span>
          </div>
          <div className="text-sm text-muted-foreground">
            {exportItem.exportType} | {exportItem.periodStart} to {exportItem.periodEnd}
          </div>
          {exportItem.recordCount && (
            <div className="text-xs text-muted-foreground">
              {exportItem.recordCount} records
              {exportItem.fileSize && ` | ${(exportItem.fileSize / 1024).toFixed(1)} KB`}
            </div>
          )}
        </div>
      </div>
      <div className="flex items-center gap-2">
        {exportItem.status === 'completed' && exportItem.filePath && (
          <a
            href={getAccountingExportDownloadUrl(exportItem.id)}
            className="px-3 py-1 text-sm border rounded-md hover:bg-muted"
            download
          >
            Download
          </a>
        )}
        {exportItem.status === 'failed' && exportItem.errorMessage && (
          <span className="text-xs text-red-600" title={exportItem.errorMessage}>
            Error
          </span>
        )}
        <span className="text-xs text-muted-foreground">
          {new Date(exportItem.createdAt).toLocaleDateString()}
        </span>
      </div>
    </div>
  );
}
