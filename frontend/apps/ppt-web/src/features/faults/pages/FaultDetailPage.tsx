/**
 * FaultDetailPage - displays full fault details with timeline.
 * Epic 4: Fault Reporting & Resolution (UC-03.5, UC-03.6, UC-03.7, UC-03.10, UC-03.11)
 */

import { useState } from 'react';
import type { FaultCategory, FaultPriority, FaultStatus } from '../components/FaultCard';
import { FaultTimeline, type TimelineEntry } from '../components/FaultTimeline';
import { type TriageData, TriageFaultDialog } from '../components/TriageFaultDialog';

export interface FaultDetail {
  id: string;
  organizationId: string;
  buildingId: string;
  unitId?: string;
  reporterId: string;
  title: string;
  description: string;
  locationDescription?: string;
  category: FaultCategory;
  priority: FaultPriority;
  status: FaultStatus;
  aiCategory?: string;
  aiPriority?: string;
  aiConfidence?: number;
  assignedTo?: string;
  assignedAt?: string;
  triagedBy?: string;
  triagedAt?: string;
  resolvedAt?: string;
  resolvedBy?: string;
  resolutionNotes?: string;
  confirmedAt?: string;
  confirmedBy?: string;
  rating?: number;
  feedback?: string;
  scheduledDate?: string;
  estimatedCompletion?: string;
  createdAt: string;
  updatedAt: string;
  // Joined fields
  reporterName: string;
  reporterEmail: string;
  buildingName: string;
  buildingAddress: string;
  unitDesignation?: string;
  assignedToName?: string;
  attachmentCount: number;
  commentCount: number;
}

export interface FaultAttachment {
  id: string;
  faultId: string;
  filename: string;
  originalFilename: string;
  contentType: string;
  sizeBytes: number;
  storageUrl: string;
  thumbnailUrl?: string;
  description?: string;
  createdAt: string;
}

interface FaultDetailPageProps {
  fault: FaultDetail;
  timeline: TimelineEntry[];
  attachments: FaultAttachment[];
  technicians?: Array<{ id: string; name: string }>;
  isManager?: boolean;
  isLoading?: boolean;
  onBack: () => void;
  onEdit: () => void;
  onTriage: (data: TriageData) => void;
  onResolve: (notes: string) => void;
  onConfirm: (rating?: number, feedback?: string) => void;
  onReopen: (reason: string) => void;
  onAddComment: (note: string, isInternal: boolean) => void;
  onAddAttachment: (file: File) => void;
  onDeleteAttachment: (id: string) => void;
}

const statusColors: Record<FaultStatus, string> = {
  new: 'bg-red-100 text-red-800',
  triaged: 'bg-blue-100 text-blue-800',
  in_progress: 'bg-yellow-100 text-yellow-800',
  waiting_parts: 'bg-orange-100 text-orange-800',
  scheduled: 'bg-purple-100 text-purple-800',
  resolved: 'bg-green-100 text-green-800',
  closed: 'bg-gray-100 text-gray-800',
  reopened: 'bg-red-100 text-red-800',
};

const priorityColors: Record<FaultPriority, string> = {
  low: 'text-gray-500',
  medium: 'text-blue-500',
  high: 'text-orange-500',
  urgent: 'text-red-600 font-bold',
};

const statusLabels: Record<FaultStatus, string> = {
  new: 'New',
  triaged: 'Triaged',
  in_progress: 'In Progress',
  waiting_parts: 'Waiting for Parts',
  scheduled: 'Scheduled',
  resolved: 'Resolved',
  closed: 'Closed',
  reopened: 'Reopened',
};

const priorityLabels: Record<FaultPriority, string> = {
  low: 'Low',
  medium: 'Medium',
  high: 'High',
  urgent: 'Urgent',
};

const categoryLabels: Record<FaultCategory, string> = {
  plumbing: 'Plumbing',
  electrical: 'Electrical',
  heating: 'Heating',
  structural: 'Structural',
  exterior: 'Exterior',
  elevator: 'Elevator',
  common_area: 'Common Area',
  security: 'Security',
  cleaning: 'Cleaning',
  other: 'Other',
};

export function FaultDetailPage({
  fault,
  timeline,
  attachments,
  technicians,
  isManager = false,
  isLoading,
  onBack,
  onEdit,
  onTriage,
  onResolve,
  onConfirm,
  onReopen,
  onAddComment,
  onAddAttachment,
  onDeleteAttachment,
}: FaultDetailPageProps) {
  const [showTriageDialog, setShowTriageDialog] = useState(false);
  const [showResolveDialog, setShowResolveDialog] = useState(false);
  const [showConfirmDialog, setShowConfirmDialog] = useState(false);
  const [showReopenDialog, setShowReopenDialog] = useState(false);
  const [comment, setComment] = useState('');
  const [isInternalComment, setIsInternalComment] = useState(false);
  const [resolutionNotes, setResolutionNotes] = useState('');
  const [confirmRating, setConfirmRating] = useState<number | undefined>();
  const [confirmFeedback, setConfirmFeedback] = useState('');
  const [reopenReason, setReopenReason] = useState('');

  const canEdit = fault.status === 'new';
  const canTriage = fault.status === 'new' && isManager;
  const canResolve =
    isManager &&
    ['triaged', 'in_progress', 'waiting_parts', 'scheduled', 'reopened'].includes(fault.status);
  const canConfirm = fault.status === 'resolved';
  const canReopen = fault.status === 'closed';

  const handleAddComment = (e: React.FormEvent) => {
    e.preventDefault();
    if (comment.trim()) {
      onAddComment(comment, isInternalComment);
      setComment('');
      setIsInternalComment(false);
    }
  };

  const handleResolve = () => {
    onResolve(resolutionNotes);
    setShowResolveDialog(false);
    setResolutionNotes('');
  };

  const handleConfirm = () => {
    onConfirm(confirmRating, confirmFeedback || undefined);
    setShowConfirmDialog(false);
    setConfirmRating(undefined);
    setConfirmFeedback('');
  };

  const handleReopen = () => {
    onReopen(reopenReason);
    setShowReopenDialog(false);
    setReopenReason('');
  };

  const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      onAddAttachment(file);
    }
  };

  if (isLoading) {
    return (
      <div className="flex justify-center py-12">
        <div className="animate-spin rounded-full h-10 w-10 border-b-2 border-blue-600" />
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto px-4 py-8">
      {/* Header */}
      <div className="mb-6">
        <button
          type="button"
          onClick={onBack}
          className="text-sm text-blue-600 hover:text-blue-800 flex items-center gap-1 mb-4"
        >
          ← Back to Faults
        </button>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold text-gray-900">{fault.title}</h1>
            <div className="mt-2 flex items-center gap-3">
              <span
                className={`px-2 py-1 text-sm font-medium rounded ${statusColors[fault.status]}`}
              >
                {statusLabels[fault.status]}
              </span>
              <span className={`text-sm font-medium ${priorityColors[fault.priority]}`}>
                {priorityLabels[fault.priority]} Priority
              </span>
              <span className="text-sm text-gray-500">{categoryLabels[fault.category]}</span>
            </div>
          </div>
          <div className="flex gap-2">
            {canEdit && (
              <button
                type="button"
                onClick={onEdit}
                className="px-3 py-1.5 text-sm border border-gray-300 rounded-lg hover:bg-gray-50"
              >
                Edit
              </button>
            )}
            {canTriage && (
              <button
                type="button"
                onClick={() => setShowTriageDialog(true)}
                className="px-3 py-1.5 text-sm bg-blue-600 text-white rounded-lg hover:bg-blue-700"
              >
                Triage
              </button>
            )}
            {canResolve && (
              <button
                type="button"
                onClick={() => setShowResolveDialog(true)}
                className="px-3 py-1.5 text-sm bg-green-600 text-white rounded-lg hover:bg-green-700"
              >
                Resolve
              </button>
            )}
            {canConfirm && (
              <button
                type="button"
                onClick={() => setShowConfirmDialog(true)}
                className="px-3 py-1.5 text-sm bg-green-600 text-white rounded-lg hover:bg-green-700"
              >
                Confirm
              </button>
            )}
            {canReopen && (
              <button
                type="button"
                onClick={() => setShowReopenDialog(true)}
                className="px-3 py-1.5 text-sm bg-orange-600 text-white rounded-lg hover:bg-orange-700"
              >
                Reopen
              </button>
            )}
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Main Content */}
        <div className="lg:col-span-2 space-y-6">
          {/* Description */}
          <div className="bg-white rounded-lg shadow p-6">
            <h2 className="text-lg font-semibold text-gray-900 mb-3">Description</h2>
            <p className="text-gray-700 whitespace-pre-wrap">{fault.description}</p>
            {fault.locationDescription && (
              <p className="mt-3 text-sm text-gray-500">
                <strong>Location:</strong> {fault.locationDescription}
              </p>
            )}
          </div>

          {/* Attachments */}
          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-lg font-semibold text-gray-900">
                Attachments ({attachments.length})
              </h2>
              <label className="px-3 py-1.5 text-sm border border-gray-300 rounded-lg hover:bg-gray-50 cursor-pointer">
                Add Photo
                <input
                  type="file"
                  accept="image/*"
                  className="hidden"
                  onChange={handleFileUpload}
                />
              </label>
            </div>
            {attachments.length === 0 ? (
              <p className="text-gray-500">No attachments.</p>
            ) : (
              <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
                {attachments.map((att) => (
                  <div key={att.id} className="relative group">
                    {att.contentType.startsWith('image/') ? (
                      <img
                        src={att.thumbnailUrl || att.storageUrl}
                        alt={att.originalFilename}
                        className="w-full h-32 object-cover rounded-lg"
                      />
                    ) : (
                      <div className="w-full h-32 bg-gray-100 rounded-lg flex items-center justify-center">
                        📎 {att.originalFilename}
                      </div>
                    )}
                    <button
                      type="button"
                      onClick={() => onDeleteAttachment(att.id)}
                      className="absolute top-1 right-1 p-1 bg-red-500 text-white rounded-full opacity-0 group-hover:opacity-100 transition-opacity"
                      title="Delete"
                    >
                      ✕
                    </button>
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* Timeline */}
          <div className="bg-white rounded-lg shadow p-6">
            <FaultTimeline entries={timeline} />
          </div>

          {/* Add Comment */}
          <div className="bg-white rounded-lg shadow p-6">
            <h3 className="text-lg font-semibold text-gray-900 mb-4">Add Comment</h3>
            <form onSubmit={handleAddComment}>
              <textarea
                value={comment}
                onChange={(e) => setComment(e.target.value)}
                rows={3}
                placeholder="Write a comment..."
                className="w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
              {isManager && (
                <label className="flex items-center gap-2 mt-2">
                  <input
                    type="checkbox"
                    checked={isInternalComment}
                    onChange={(e) => setIsInternalComment(e.target.checked)}
                  />
                  <span className="text-sm text-gray-600">
                    Internal note (not visible to reporter)
                  </span>
                </label>
              )}
              <button
                type="submit"
                disabled={!comment.trim()}
                className="mt-3 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50"
              >
                Add Comment
              </button>
            </form>
          </div>
        </div>

        {/* Sidebar */}
        <div className="space-y-6">
          {/* Details */}
          <div className="bg-white rounded-lg shadow p-6">
            <h3 className="text-lg font-semibold text-gray-900 mb-4">Details</h3>
            <dl className="space-y-3 text-sm">
              <div>
                <dt className="text-gray-500">Building</dt>
                <dd className="font-medium">{fault.buildingName}</dd>
                <dd className="text-gray-500">{fault.buildingAddress}</dd>
              </div>
              {fault.unitDesignation && (
                <div>
                  <dt className="text-gray-500">Unit</dt>
                  <dd className="font-medium">{fault.unitDesignation}</dd>
                </div>
              )}
              <div>
                <dt className="text-gray-500">Reported by</dt>
                <dd className="font-medium">{fault.reporterName}</dd>
                <dd className="text-gray-500">{fault.reporterEmail}</dd>
              </div>
              <div>
                <dt className="text-gray-500">Reported on</dt>
                <dd className="font-medium">{new Date(fault.createdAt).toLocaleString()}</dd>
              </div>
              {fault.assignedToName && (
                <div>
                  <dt className="text-gray-500">Assigned to</dt>
                  <dd className="font-medium">{fault.assignedToName}</dd>
                </div>
              )}
              {fault.scheduledDate && (
                <div>
                  <dt className="text-gray-500">Scheduled</dt>
                  <dd className="font-medium">
                    {new Date(fault.scheduledDate).toLocaleDateString()}
                  </dd>
                </div>
              )}
              {fault.resolvedAt && (
                <div>
                  <dt className="text-gray-500">Resolved on</dt>
                  <dd className="font-medium">{new Date(fault.resolvedAt).toLocaleString()}</dd>
                </div>
              )}
              {fault.rating && (
                <div>
                  <dt className="text-gray-500">Rating</dt>
                  <dd className="font-medium">{'⭐'.repeat(fault.rating)}</dd>
                </div>
              )}
            </dl>
          </div>

          {/* AI Suggestion (if available) */}
          {fault.aiCategory && fault.aiConfidence && (
            <div className="bg-blue-50 rounded-lg p-6 border border-blue-100">
              <h3 className="text-lg font-semibold text-blue-900 mb-2">🤖 AI Analysis</h3>
              <p className="text-sm text-blue-800">
                Suggested category: <strong>{fault.aiCategory}</strong>
              </p>
              {fault.aiPriority && (
                <p className="text-sm text-blue-800">
                  Suggested priority: <strong>{fault.aiPriority}</strong>
                </p>
              )}
              <p className="text-sm text-blue-600 mt-1">
                {Math.round(fault.aiConfidence * 100)}% confidence
              </p>
            </div>
          )}
        </div>
      </div>

      {/* Triage Dialog */}
      <TriageFaultDialog
        isOpen={showTriageDialog}
        faultTitle={fault.title}
        currentCategory={fault.category}
        aiSuggestion={
          fault.aiCategory && fault.aiConfidence
            ? {
                category: fault.aiCategory,
                priority: fault.aiPriority,
                confidence: fault.aiConfidence,
              }
            : undefined
        }
        technicians={technicians}
        onSubmit={(data) => {
          onTriage(data);
          setShowTriageDialog(false);
        }}
        onClose={() => setShowTriageDialog(false)}
      />

      {/* Resolve Dialog */}
      {showResolveDialog && (
        <div className="fixed inset-0 z-50 overflow-y-auto">
          <button
            type="button"
            className="fixed inset-0 bg-black bg-opacity-50 cursor-default"
            onClick={() => setShowResolveDialog(false)}
            onKeyDown={(e) => e.key === 'Escape' && setShowResolveDialog(false)}
            aria-label="Close dialog"
          />
          <div className="flex min-h-full items-center justify-center p-4">
            <div className="relative w-full max-w-md bg-white rounded-lg shadow-xl p-6">
              <h2 className="text-lg font-semibold mb-4">Resolve Fault</h2>
              <textarea
                value={resolutionNotes}
                onChange={(e) => setResolutionNotes(e.target.value)}
                rows={4}
                placeholder="Describe how the issue was resolved..."
                className="w-full rounded-md border border-gray-300 px-3 py-2"
              />
              <div className="flex justify-end gap-3 mt-4">
                <button
                  type="button"
                  onClick={() => setShowResolveDialog(false)}
                  className="px-4 py-2 border rounded-lg"
                >
                  Cancel
                </button>
                <button
                  type="button"
                  onClick={handleResolve}
                  className="px-4 py-2 bg-green-600 text-white rounded-lg"
                >
                  Resolve
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Confirm Dialog */}
      {showConfirmDialog && (
        <div className="fixed inset-0 z-50 overflow-y-auto">
          <button
            type="button"
            className="fixed inset-0 bg-black bg-opacity-50 cursor-default"
            onClick={() => setShowConfirmDialog(false)}
            onKeyDown={(e) => e.key === 'Escape' && setShowConfirmDialog(false)}
            aria-label="Close dialog"
          />
          <div className="flex min-h-full items-center justify-center p-4">
            <div className="relative w-full max-w-md bg-white rounded-lg shadow-xl p-6">
              <h2 className="text-lg font-semibold mb-4">Confirm Resolution</h2>
              <fieldset className="mb-4">
                <legend className="block text-sm font-medium text-gray-700 mb-2">
                  Rate the resolution (optional)
                </legend>
                <div className="flex gap-2">
                  {[1, 2, 3, 4, 5].map((star) => (
                    <button
                      key={star}
                      type="button"
                      onClick={() => setConfirmRating(star)}
                      className={`text-2xl ${confirmRating && confirmRating >= star ? 'text-yellow-400' : 'text-gray-300'}`}
                    >
                      ⭐
                    </button>
                  ))}
                </div>
              </fieldset>
              <textarea
                value={confirmFeedback}
                onChange={(e) => setConfirmFeedback(e.target.value)}
                rows={3}
                placeholder="Optional feedback..."
                className="w-full rounded-md border border-gray-300 px-3 py-2"
              />
              <div className="flex justify-end gap-3 mt-4">
                <button
                  type="button"
                  onClick={() => setShowConfirmDialog(false)}
                  className="px-4 py-2 border rounded-lg"
                >
                  Cancel
                </button>
                <button
                  type="button"
                  onClick={handleConfirm}
                  className="px-4 py-2 bg-green-600 text-white rounded-lg"
                >
                  Confirm
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Reopen Dialog */}
      {showReopenDialog && (
        <div className="fixed inset-0 z-50 overflow-y-auto">
          <button
            type="button"
            className="fixed inset-0 bg-black bg-opacity-50 cursor-default"
            onClick={() => setShowReopenDialog(false)}
            onKeyDown={(e) => e.key === 'Escape' && setShowReopenDialog(false)}
            aria-label="Close dialog"
          />
          <div className="flex min-h-full items-center justify-center p-4">
            <div className="relative w-full max-w-md bg-white rounded-lg shadow-xl p-6">
              <h2 className="text-lg font-semibold mb-4">Reopen Fault</h2>
              <textarea
                value={reopenReason}
                onChange={(e) => setReopenReason(e.target.value)}
                rows={3}
                placeholder="Why does this need to be reopened?"
                className="w-full rounded-md border border-gray-300 px-3 py-2"
              />
              <div className="flex justify-end gap-3 mt-4">
                <button
                  type="button"
                  onClick={() => setShowReopenDialog(false)}
                  className="px-4 py-2 border rounded-lg"
                >
                  Cancel
                </button>
                <button
                  type="button"
                  onClick={handleReopen}
                  disabled={!reopenReason.trim()}
                  className="px-4 py-2 bg-orange-600 text-white rounded-lg disabled:opacity-50"
                >
                  Reopen
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
