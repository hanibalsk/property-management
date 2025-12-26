/**
 * E-Signature Workflows List Component
 *
 * Displays and manages e-signature workflows (Story 61.3).
 */

import type { ESignatureWorkflow } from '@ppt/api-client';
import {
  useESignatureWorkflows,
  useSendESignatureReminder,
  useSendESignatureWorkflow,
  useVoidESignatureWorkflow,
} from '@ppt/api-client';
import { useState } from 'react';

interface ESignatureWorkflowsListProps {
  organizationId: string;
  onCreate?: () => void;
}

const statusColors: Record<string, string> = {
  draft: 'bg-gray-100 text-gray-800',
  sent: 'bg-blue-100 text-blue-800',
  viewed: 'bg-yellow-100 text-yellow-800',
  signed: 'bg-green-100 text-green-800',
  completed: 'bg-green-100 text-green-800',
  declined: 'bg-red-100 text-red-800',
  voided: 'bg-gray-100 text-gray-800',
  expired: 'bg-orange-100 text-orange-800',
};

const providerLabels: Record<string, string> = {
  docusign: 'DocuSign',
  adobe_sign: 'Adobe Sign',
  hellosign: 'HelloSign',
  internal: 'Internal',
};

export function ESignatureWorkflowsList({
  organizationId,
  onCreate,
}: ESignatureWorkflowsListProps) {
  const { data: workflows, isLoading } = useESignatureWorkflows(organizationId);
  const sendWorkflow = useSendESignatureWorkflow(organizationId);
  const voidWorkflow = useVoidESignatureWorkflow(organizationId);
  const sendReminder = useSendESignatureReminder();
  const [selectedWorkflow, setSelectedWorkflow] = useState<string | null>(null);

  const handleSend = async (id: string) => {
    if (confirm('Are you sure you want to send this document for signing?')) {
      await sendWorkflow.mutateAsync(id);
    }
  };

  const handleVoid = async (id: string) => {
    if (confirm('Are you sure you want to void this signature request?')) {
      await voidWorkflow.mutateAsync(id);
    }
  };

  const handleReminder = async (id: string) => {
    await sendReminder.mutateAsync(id);
    alert('Reminder sent successfully');
  };

  const formatDate = (dateStr: string) => {
    return new Date(dateStr).toLocaleDateString();
  };

  if (isLoading) {
    return (
      <div className="rounded-lg border bg-card p-6">
        <h3 className="text-lg font-semibold">E-Signature Workflows</h3>
        <p className="text-muted-foreground">Loading...</p>
      </div>
    );
  }

  return (
    <div className="rounded-lg border bg-card p-6">
      <div className="flex items-center justify-between mb-4">
        <div>
          <h3 className="text-lg font-semibold">E-Signature Workflows</h3>
          <p className="text-sm text-muted-foreground">
            Request and track electronic signatures on documents
          </p>
        </div>
        <button
          type="button"
          onClick={onCreate}
          className="inline-flex items-center px-4 py-2 text-sm font-medium bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
        >
          + Request Signature
        </button>
      </div>

      {workflows?.length === 0 ? (
        <div className="flex flex-col items-center justify-center py-8 text-center">
          <div className="text-4xl mb-4">sig</div>
          <p className="text-muted-foreground">No signature workflows</p>
          <p className="text-sm text-muted-foreground">
            Create a signature request to get documents signed electronically
          </p>
        </div>
      ) : (
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="py-3 px-4 text-left text-sm font-medium">Document</th>
                <th className="py-3 px-4 text-left text-sm font-medium">Provider</th>
                <th className="py-3 px-4 text-left text-sm font-medium">Status</th>
                <th className="py-3 px-4 text-left text-sm font-medium">Created</th>
                <th className="py-3 px-4 text-left text-sm font-medium">Expires</th>
                <th className="py-3 px-4 text-left text-sm font-medium w-[200px]">Actions</th>
              </tr>
            </thead>
            <tbody>
              {workflows?.map((workflow: ESignatureWorkflow) => (
                <tr key={workflow.id} className="border-b">
                  <td className="py-3 px-4">
                    <div className="font-medium">{workflow.title}</div>
                    {workflow.message && (
                      <div className="text-sm text-muted-foreground truncate max-w-[200px]">
                        {workflow.message}
                      </div>
                    )}
                  </td>
                  <td className="py-3 px-4 text-sm">
                    {providerLabels[workflow.provider] || workflow.provider}
                  </td>
                  <td className="py-3 px-4">
                    <span
                      className={`inline-flex items-center px-2 py-0.5 rounded text-xs font-medium ${statusColors[workflow.status]}`}
                    >
                      {workflow.status}
                    </span>
                  </td>
                  <td className="py-3 px-4 text-sm text-muted-foreground">
                    {formatDate(workflow.createdAt)}
                  </td>
                  <td className="py-3 px-4 text-sm text-muted-foreground">
                    {workflow.expiresAt ? formatDate(workflow.expiresAt) : '-'}
                  </td>
                  <td className="py-3 px-4">
                    <div className="flex items-center gap-2">
                      {workflow.status === 'draft' && (
                        <button
                          type="button"
                          onClick={() => handleSend(workflow.id)}
                          className="px-3 py-1 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
                        >
                          Send
                        </button>
                      )}
                      {(workflow.status === 'sent' || workflow.status === 'viewed') && (
                        <>
                          <button
                            type="button"
                            onClick={() => handleReminder(workflow.id)}
                            className="px-3 py-1 text-sm border rounded-md hover:bg-muted"
                          >
                            Remind
                          </button>
                          <button
                            type="button"
                            onClick={() => handleVoid(workflow.id)}
                            className="px-3 py-1 text-sm text-red-600 border border-red-200 rounded-md hover:bg-red-50"
                          >
                            Void
                          </button>
                        </>
                      )}
                      <button
                        type="button"
                        onClick={() => setSelectedWorkflow(workflow.id)}
                        className="px-3 py-1 text-sm border rounded-md hover:bg-muted"
                      >
                        Details
                      </button>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {selectedWorkflow && (
        <WorkflowDetailsDialog
          workflowId={selectedWorkflow}
          onClose={() => setSelectedWorkflow(null)}
        />
      )}
    </div>
  );
}

function WorkflowDetailsDialog({
  workflowId,
  onClose,
}: {
  workflowId: string;
  onClose: () => void;
}) {
  // In a full implementation, this would fetch workflow details with recipients
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="bg-background rounded-lg border shadow-lg w-full max-w-lg p-6">
        <div className="mb-4">
          <h3 className="text-lg font-semibold">Signature Workflow Details</h3>
          <p className="text-sm text-muted-foreground">View recipients and signature status</p>
        </div>
        <div className="space-y-4">
          <div className="rounded-lg border p-4">
            <div className="text-sm text-muted-foreground mb-2">Recipients</div>
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <div className="h-8 w-8 rounded-full bg-muted flex items-center justify-center text-xs">
                    1
                  </div>
                  <div>
                    <div className="font-medium text-sm">Loading...</div>
                    <div className="text-xs text-muted-foreground">Recipient details</div>
                  </div>
                </div>
                <span className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-gray-100 text-gray-800">
                  pending
                </span>
              </div>
            </div>
          </div>
          <div className="text-sm text-muted-foreground">Workflow ID: {workflowId}</div>
        </div>
        <div className="mt-4 flex justify-end">
          <button
            type="button"
            onClick={onClose}
            className="px-4 py-2 text-sm font-medium bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
}
