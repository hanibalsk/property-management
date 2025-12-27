/**
 * Enhanced Due Diligence (EDD) Record Card Component (Epic 67, Story 67.2).
 *
 * Displays EDD record details including status, documents, and compliance notes.
 */

import type React from 'react';

export type EddStatus =
  | 'required'
  | 'in_progress'
  | 'pending_documents'
  | 'under_review'
  | 'completed'
  | 'expired';

export type DocumentVerificationStatus = 'pending' | 'verified' | 'rejected' | 'expired';

export interface EddDocument {
  id: string;
  document_type: string;
  original_filename: string;
  verification_status: DocumentVerificationStatus;
  verified_at?: string;
  expiry_date?: string;
  uploaded_at: string;
}

export interface ComplianceNote {
  id: string;
  content: string;
  added_by_name: string;
  added_at: string;
}

export interface EddRecord {
  id: string;
  aml_assessment_id: string;
  party_id: string;
  status: EddStatus;
  source_of_wealth?: string;
  source_of_funds?: string;
  beneficial_ownership?: unknown;
  documents_requested: string[];
  documents_received: EddDocument[];
  compliance_notes: ComplianceNote[];
  initiated_at: string;
  initiated_by: string;
  completed_at?: string;
  next_review_date?: string;
}

export interface EddRecordCardProps {
  record: EddRecord;
  onUploadDocument?: (eddId: string) => void;
  onAddNote?: (eddId: string) => void;
  onComplete?: (eddId: string) => void;
  onVerifyDocument?: (eddId: string, docId: string) => void;
  showActions?: boolean;
  isManager?: boolean;
}

const getStatusLabel = (status: EddStatus): string => {
  switch (status) {
    case 'required':
      return 'Required';
    case 'in_progress':
      return 'In Progress';
    case 'pending_documents':
      return 'Pending Documents';
    case 'under_review':
      return 'Under Review';
    case 'completed':
      return 'Completed';
    case 'expired':
      return 'Expired';
    default:
      return status;
  }
};

const getDocStatusLabel = (status: DocumentVerificationStatus): string => {
  switch (status) {
    case 'pending':
      return 'Pending Verification';
    case 'verified':
      return 'Verified';
    case 'rejected':
      return 'Rejected';
    case 'expired':
      return 'Expired';
    default:
      return status;
  }
};

const formatDate = (dateStr: string): string => {
  return new Date(dateStr).toLocaleDateString('en-GB', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
};

const formatDocumentType = (type: string): string => {
  return type
    .split('_')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
};

export const EddRecordCard: React.FC<EddRecordCardProps> = ({
  record,
  onUploadDocument,
  onAddNote,
  onComplete,
  onVerifyDocument,
  showActions = true,
  isManager = false,
}) => {
  const pendingDocs = record.documents_requested.filter(
    (docType) => !record.documents_received.some((d) => d.document_type === docType)
  );

  const canComplete =
    record.status === 'in_progress' &&
    pendingDocs.length === 0 &&
    record.documents_received.every((d) => d.verification_status === 'verified');

  return (
    <div className="edd-record-card">
      <div className="edd-record-header">
        <div className="edd-record-title">
          <h3>Enhanced Due Diligence</h3>
          <span className={`edd-status-badge ${record.status}`}>{getStatusLabel(record.status)}</span>
        </div>
        <div className="edd-record-meta">
          <span>Initiated: {formatDate(record.initiated_at)}</span>
          {record.next_review_date && <span>Next Review: {formatDate(record.next_review_date)}</span>}
        </div>
      </div>

      {/* Source of Wealth/Funds */}
      <div className="edd-sources-section">
        <h4>Source Information</h4>
        <div className="edd-sources-grid">
          <div className="edd-source-item">
            <label>Source of Wealth</label>
            <p>{record.source_of_wealth || 'Not documented'}</p>
          </div>
          <div className="edd-source-item">
            <label>Source of Funds</label>
            <p>{record.source_of_funds || 'Not documented'}</p>
          </div>
        </div>
      </div>

      {/* Documents Section */}
      <div className="edd-documents-section">
        <h4>Required Documents</h4>

        {/* Documents Received */}
        {record.documents_received.length > 0 && (
          <div className="edd-documents-received">
            <h5>Documents Received</h5>
            <ul className="edd-documents-list">
              {record.documents_received.map((doc) => (
                <li key={doc.id} className={`edd-document-item ${doc.verification_status}`}>
                  <div className="edd-document-info">
                    <span className="edd-document-type">{formatDocumentType(doc.document_type)}</span>
                    <span className="edd-document-filename">{doc.original_filename}</span>
                    <span className={`edd-document-status ${doc.verification_status}`}>
                      {getDocStatusLabel(doc.verification_status)}
                    </span>
                    {doc.expiry_date && (
                      <span className="edd-document-expiry">Expires: {formatDate(doc.expiry_date)}</span>
                    )}
                  </div>
                  {isManager && doc.verification_status === 'pending' && onVerifyDocument && (
                    <button
                      type="button"
                      className="edd-verify-button"
                      onClick={() => onVerifyDocument(record.id, doc.id)}
                    >
                      Verify
                    </button>
                  )}
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* Pending Documents */}
        {pendingDocs.length > 0 && (
          <div className="edd-documents-pending">
            <h5>Pending Documents</h5>
            <ul className="edd-pending-list">
              {pendingDocs.map((docType, index) => (
                <li key={index} className="edd-pending-item">
                  <span className="edd-pending-icon">!</span>
                  <span>{formatDocumentType(docType)}</span>
                </li>
              ))}
            </ul>
          </div>
        )}
      </div>

      {/* Compliance Notes */}
      <div className="edd-notes-section">
        <h4>Compliance Notes</h4>
        {record.compliance_notes.length > 0 ? (
          <ul className="edd-notes-list">
            {record.compliance_notes.map((note) => (
              <li key={note.id} className="edd-note-item">
                <div className="edd-note-content">{note.content}</div>
                <div className="edd-note-meta">
                  <span className="edd-note-author">{note.added_by_name}</span>
                  <span className="edd-note-date">{formatDate(note.added_at)}</span>
                </div>
              </li>
            ))}
          </ul>
        ) : (
          <p className="edd-no-notes">No compliance notes recorded.</p>
        )}
      </div>

      {/* Actions */}
      {showActions && (
        <div className="edd-record-actions">
          {onUploadDocument && record.status !== 'completed' && (
            <button
              type="button"
              className="edd-action-button secondary"
              onClick={() => onUploadDocument(record.id)}
            >
              Upload Document
            </button>
          )}
          {isManager && onAddNote && (
            <button
              type="button"
              className="edd-action-button secondary"
              onClick={() => onAddNote(record.id)}
            >
              Add Note
            </button>
          )}
          {isManager && canComplete && onComplete && (
            <button
              type="button"
              className="edd-action-button primary"
              onClick={() => onComplete(record.id)}
            >
              Complete EDD
            </button>
          )}
        </div>
      )}

      {/* Completed Info */}
      {record.completed_at && (
        <div className="edd-completed-info">
          <span>Completed on {formatDate(record.completed_at)}</span>
        </div>
      )}
    </div>
  );
};

EddRecordCard.displayName = 'EddRecordCard';
