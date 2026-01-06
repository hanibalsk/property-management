/**
 * DelegationDetailPage - displays full delegation details with actions.
 * Epic 3: Ownership Management (Story 3.4) - UC-28 Delegation History
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { DelegationActivityItem } from '../components/DelegationActivityItem';
import type { DelegationScope, DelegationStatus } from '../components/DelegationCard';
import type { DelegationActivity } from '../types';

export interface DelegationDetail {
  id: string;
  ownerUserId: string;
  delegateUserId: string;
  unitId?: string;
  scopes: DelegationScope[];
  status: DelegationStatus;
  startDate: string;
  endDate?: string;
  acceptedAt?: string;
  revokedAt?: string;
  revokedReason?: string;
  createdAt: string;
  // Display fields
  ownerName: string;
  ownerEmail: string;
  delegateName: string;
  delegateEmail: string;
  unitDesignation?: string;
}

interface DelegationDetailPageProps {
  delegation: DelegationDetail;
  activities?: DelegationActivity[];
  isCurrentUserOwner: boolean;
  isCurrentUserDelegate: boolean;
  isLoading?: boolean;
  onBack: () => void;
  onAccept: () => void;
  onDecline: () => void;
  onRevoke: (reason?: string) => void;
}

const statusColors: Record<DelegationStatus, string> = {
  pending: 'bg-yellow-100 text-yellow-800',
  active: 'bg-green-100 text-green-800',
  revoked: 'bg-red-100 text-red-800',
  expired: 'bg-gray-100 text-gray-800',
  declined: 'bg-red-100 text-red-800',
};

export function DelegationDetailPage({
  delegation,
  activities = [],
  isCurrentUserOwner,
  isCurrentUserDelegate,
  isLoading,
  onBack,
  onAccept,
  onDecline,
  onRevoke,
}: DelegationDetailPageProps) {
  const { t } = useTranslation();
  const [showRevokeDialog, setShowRevokeDialog] = useState(false);
  const [revokeReason, setRevokeReason] = useState('');
  const [showActivityLog, setShowActivityLog] = useState(true);

  const statusLabels: Record<DelegationStatus, string> = {
    pending: t('delegation.statusPending'),
    active: t('delegation.statusActive'),
    revoked: t('delegation.statusRevoked'),
    expired: t('delegation.statusExpired'),
    declined: t('delegation.statusDeclined'),
  };

  const scopeLabels: Record<DelegationScope, string> = {
    all: t('delegation.scopeAll'),
    voting: t('delegation.scopeVoting'),
    documents: t('delegation.scopeDocuments'),
    faults: t('delegation.scopeFaults'),
    financial: t('delegation.scopeFinancial'),
  };

  const canAccept = isCurrentUserDelegate && delegation.status === 'pending';
  const canDecline = isCurrentUserDelegate && delegation.status === 'pending';
  const canRevoke =
    isCurrentUserOwner && (delegation.status === 'pending' || delegation.status === 'active');

  const handleRevoke = () => {
    onRevoke(revokeReason || undefined);
    setShowRevokeDialog(false);
    setRevokeReason('');
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
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M15 19l-7-7 7-7"
            />
          </svg>
          {t('delegation.backToDelegations')}
        </button>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold text-gray-900">{t('delegation.detailTitle')}</h1>
            <div className="mt-2 flex items-center gap-3">
              <span
                className={`px-3 py-1 text-sm font-medium rounded ${statusColors[delegation.status]}`}
              >
                {statusLabels[delegation.status]}
              </span>
            </div>
          </div>
          <div className="flex gap-2">
            {canAccept && (
              <button
                type="button"
                onClick={onAccept}
                className="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700"
              >
                {t('delegation.accept')}
              </button>
            )}
            {canDecline && (
              <button
                type="button"
                onClick={onDecline}
                className="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700"
              >
                {t('delegation.decline')}
              </button>
            )}
            {canRevoke && (
              <button
                type="button"
                onClick={() => setShowRevokeDialog(true)}
                className="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700"
              >
                {t('delegation.revoke')}
              </button>
            )}
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Owner Info */}
        <div className="bg-white rounded-lg shadow p-6">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">{t('delegation.owner')}</h2>
          <dl className="space-y-3 text-sm">
            <div>
              <dt className="text-gray-500">{t('delegation.name')}</dt>
              <dd className="font-medium">{delegation.ownerName}</dd>
            </div>
            <div>
              <dt className="text-gray-500">{t('delegation.email')}</dt>
              <dd className="font-medium">{delegation.ownerEmail}</dd>
            </div>
          </dl>
        </div>

        {/* Delegate Info */}
        <div className="bg-white rounded-lg shadow p-6">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">{t('delegation.delegate')}</h2>
          <dl className="space-y-3 text-sm">
            <div>
              <dt className="text-gray-500">{t('delegation.name')}</dt>
              <dd className="font-medium">{delegation.delegateName}</dd>
            </div>
            <div>
              <dt className="text-gray-500">{t('delegation.email')}</dt>
              <dd className="font-medium">{delegation.delegateEmail}</dd>
            </div>
          </dl>
        </div>

        {/* Delegation Details */}
        <div className="bg-white rounded-lg shadow p-6 lg:col-span-2">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">{t('delegation.details')}</h2>
          <dl className="grid grid-cols-1 md:grid-cols-2 gap-6 text-sm">
            {delegation.unitDesignation && (
              <div>
                <dt className="text-gray-500">{t('delegation.unit')}</dt>
                <dd className="font-medium">{delegation.unitDesignation}</dd>
              </div>
            )}
            <div>
              <dt className="text-gray-500">{t('delegation.scopes')}</dt>
              <dd className="flex flex-wrap gap-2 mt-1">
                {delegation.scopes.map((scope) => (
                  <span key={scope} className="px-2 py-1 text-xs bg-blue-50 text-blue-700 rounded">
                    {scopeLabels[scope]}
                  </span>
                ))}
              </dd>
            </div>
            <div>
              <dt className="text-gray-500">{t('delegation.startDate')}</dt>
              <dd className="font-medium">{new Date(delegation.startDate).toLocaleDateString()}</dd>
            </div>
            {delegation.endDate && (
              <div>
                <dt className="text-gray-500">{t('delegation.endDate')}</dt>
                <dd className="font-medium">{new Date(delegation.endDate).toLocaleDateString()}</dd>
              </div>
            )}
            <div>
              <dt className="text-gray-500">{t('delegation.createdAt')}</dt>
              <dd className="font-medium">{new Date(delegation.createdAt).toLocaleString()}</dd>
            </div>
            {delegation.acceptedAt && (
              <div>
                <dt className="text-gray-500">{t('delegation.acceptedAt')}</dt>
                <dd className="font-medium">{new Date(delegation.acceptedAt).toLocaleString()}</dd>
              </div>
            )}
            {delegation.revokedAt && (
              <div>
                <dt className="text-gray-500">{t('delegation.revokedAt')}</dt>
                <dd className="font-medium">{new Date(delegation.revokedAt).toLocaleString()}</dd>
              </div>
            )}
            {delegation.revokedReason && (
              <div className="md:col-span-2">
                <dt className="text-gray-500">{t('delegation.revokeReason')}</dt>
                <dd className="font-medium">{delegation.revokedReason}</dd>
              </div>
            )}
          </dl>
        </div>

        {/* Activity Log Section */}
        <div className="bg-white rounded-lg shadow p-6 lg:col-span-2">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold text-gray-900">
              {t('delegation.history.activityLog')}
            </h2>
            <button
              type="button"
              onClick={() => setShowActivityLog(!showActivityLog)}
              className="text-sm text-blue-600 hover:text-blue-800 flex items-center gap-1"
            >
              {showActivityLog ? (
                <>
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M19 9l-7 7-7-7"
                    />
                  </svg>
                  {t('delegation.history.collapse')}
                </>
              ) : (
                <>
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M9 5l7 7-7 7"
                    />
                  </svg>
                  {t('delegation.history.expand')}
                </>
              )}
            </button>
          </div>

          {showActivityLog &&
            (activities.length === 0 ? (
              <div className="text-center py-8 text-gray-500">
                <svg
                  className="mx-auto h-10 w-10 text-gray-400 mb-2"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                  />
                </svg>
                <p>{t('delegation.history.noActivitiesForDelegation')}</p>
              </div>
            ) : (
              <div className="space-y-0">
                {activities.map((activity, index) => (
                  <DelegationActivityItem
                    key={activity.id}
                    activity={activity}
                    showFullDate
                    isFirst={index === 0}
                    isLast={index === activities.length - 1}
                  />
                ))}
              </div>
            ))}
        </div>
      </div>

      {/* Revoke Dialog */}
      {showRevokeDialog && (
        <div className="fixed inset-0 z-50 overflow-y-auto">
          <button
            type="button"
            className="fixed inset-0 bg-black bg-opacity-50 cursor-default"
            onClick={() => setShowRevokeDialog(false)}
            onKeyDown={(e) => e.key === 'Escape' && setShowRevokeDialog(false)}
            aria-label={t('common.close')}
          />
          <div className="flex min-h-full items-center justify-center p-4">
            <div className="relative w-full max-w-md bg-white rounded-lg shadow-xl p-6">
              <h2 className="text-lg font-semibold mb-4">{t('delegation.revokeDelegation')}</h2>
              <p className="text-sm text-gray-600 mb-4">{t('delegation.revokeConfirmation')}</p>
              <textarea
                value={revokeReason}
                onChange={(e) => setRevokeReason(e.target.value)}
                rows={3}
                placeholder={t('delegation.revokeReasonPlaceholder')}
                className="w-full rounded-md border border-gray-300 px-3 py-2"
              />
              <div className="flex justify-end gap-3 mt-4">
                <button
                  type="button"
                  onClick={() => setShowRevokeDialog(false)}
                  className="px-4 py-2 border rounded-lg"
                >
                  {t('common.cancel')}
                </button>
                <button
                  type="button"
                  onClick={handleRevoke}
                  className="px-4 py-2 bg-red-600 text-white rounded-lg"
                >
                  {t('delegation.revoke')}
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
