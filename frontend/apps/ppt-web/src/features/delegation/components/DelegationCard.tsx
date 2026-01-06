/**
 * DelegationCard component - displays a delegation summary card in lists.
 * Epic 3: Ownership Management (Story 3.4)
 */

import { useTranslation } from 'react-i18next';

export type DelegationStatus = 'pending' | 'active' | 'revoked' | 'expired' | 'declined';

export type DelegationScope = 'all' | 'voting' | 'documents' | 'faults' | 'financial';

export interface DelegationSummary {
  id: string;
  ownerUserId: string;
  delegateUserId: string;
  unitId?: string;
  scopes: DelegationScope[];
  status: DelegationStatus;
  startDate: string;
  endDate?: string;
  // Display fields
  ownerName?: string;
  delegateName?: string;
  unitDesignation?: string;
}

interface DelegationCardProps {
  delegation: DelegationSummary;
  viewType: 'given' | 'received';
  onView?: (id: string) => void;
  onAccept?: (id: string) => void;
  onDecline?: (id: string) => void;
  onRevoke?: (id: string) => void;
}

const statusColors: Record<DelegationStatus, string> = {
  pending: 'bg-yellow-100 text-yellow-800',
  active: 'bg-green-100 text-green-800',
  revoked: 'bg-red-100 text-red-800',
  expired: 'bg-gray-100 text-gray-800',
  declined: 'bg-red-100 text-red-800',
};

export function DelegationCard({
  delegation,
  viewType,
  onView,
  onAccept,
  onDecline,
  onRevoke,
}: DelegationCardProps) {
  const { t } = useTranslation();

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

  const canAccept = viewType === 'received' && delegation.status === 'pending';
  const canDecline = viewType === 'received' && delegation.status === 'pending';
  const canRevoke =
    viewType === 'given' && (delegation.status === 'pending' || delegation.status === 'active');

  const displayName = viewType === 'given' ? delegation.delegateName : delegation.ownerName;
  const relationLabel =
    viewType === 'given' ? t('delegation.delegateTo') : t('delegation.delegateFrom');

  return (
    <div className="bg-white rounded-lg shadow p-4 hover:shadow-md transition-shadow">
      <div className="flex items-start justify-between">
        <div className="flex-1">
          <div className="flex items-center gap-2">
            <h3 className="text-lg font-semibold text-gray-900">
              {relationLabel}: {displayName || t('delegation.unknownUser')}
            </h3>
          </div>
          <div className="mt-2 flex items-center gap-2 flex-wrap">
            <span
              className={`px-2 py-1 text-xs font-medium rounded ${statusColors[delegation.status]}`}
            >
              {statusLabels[delegation.status]}
            </span>
            {delegation.unitDesignation && (
              <span className="text-xs text-gray-500">
                {t('delegation.unit')}: {delegation.unitDesignation}
              </span>
            )}
          </div>
          <div className="mt-2 flex items-center gap-2 flex-wrap">
            <span className="text-xs text-gray-500">{t('delegation.scopes')}:</span>
            {delegation.scopes.map((scope) => (
              <span key={scope} className="px-2 py-0.5 text-xs bg-blue-50 text-blue-700 rounded">
                {scopeLabels[scope]}
              </span>
            ))}
          </div>
          <div className="mt-2 text-xs text-gray-400">
            {t('delegation.startDate')}: {new Date(delegation.startDate).toLocaleDateString()}
            {delegation.endDate && (
              <>
                {' '}
                | {t('delegation.endDate')}: {new Date(delegation.endDate).toLocaleDateString()}
              </>
            )}
          </div>
        </div>
      </div>

      <div className="mt-4 flex items-center gap-2 border-t pt-3">
        <button
          type="button"
          onClick={() => onView?.(delegation.id)}
          className="text-sm text-blue-600 hover:text-blue-800"
        >
          {t('common.view')}
        </button>
        {canAccept && (
          <button
            type="button"
            onClick={() => onAccept?.(delegation.id)}
            className="text-sm text-green-600 hover:text-green-800"
          >
            {t('delegation.accept')}
          </button>
        )}
        {canDecline && (
          <button
            type="button"
            onClick={() => onDecline?.(delegation.id)}
            className="text-sm text-red-600 hover:text-red-800"
          >
            {t('delegation.decline')}
          </button>
        )}
        {canRevoke && (
          <button
            type="button"
            onClick={() => onRevoke?.(delegation.id)}
            className="text-sm text-red-600 hover:text-red-800"
          >
            {t('delegation.revoke')}
          </button>
        )}
      </div>
    </div>
  );
}
