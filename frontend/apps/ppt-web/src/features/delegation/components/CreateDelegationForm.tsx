/**
 * CreateDelegationForm component - form to create a new delegation.
 * Epic 3: Ownership Management (Story 3.4.1)
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { DelegationScope } from './DelegationCard';

export interface CreateDelegationFormData {
  delegateUserId: string;
  unitId?: string;
  scopes: DelegationScope[];
  startDate?: string;
  endDate?: string;
}

interface CreateDelegationFormProps {
  users: Array<{ id: string; name: string; email: string }>;
  units: Array<{ id: string; designation: string }>;
  isSubmitting?: boolean;
  onSubmit: (data: CreateDelegationFormData) => void;
  onCancel: () => void;
}

const AVAILABLE_SCOPES: DelegationScope[] = ['all', 'voting', 'documents', 'faults', 'financial'];

export function CreateDelegationForm({
  users,
  units,
  isSubmitting,
  onSubmit,
  onCancel,
}: CreateDelegationFormProps) {
  const { t } = useTranslation();
  const [delegateUserId, setDelegateUserId] = useState('');
  const [unitId, setUnitId] = useState('');
  const [scopes, setScopes] = useState<DelegationScope[]>(['all']);
  const [startDate, setStartDate] = useState('');
  const [endDate, setEndDate] = useState('');
  const [errors, setErrors] = useState<Record<string, string>>({});

  const scopeLabels: Record<DelegationScope, string> = {
    all: t('delegation.scopeAll'),
    voting: t('delegation.scopeVoting'),
    documents: t('delegation.scopeDocuments'),
    faults: t('delegation.scopeFaults'),
    financial: t('delegation.scopeFinancial'),
  };

  const handleScopeToggle = (scope: DelegationScope) => {
    if (scope === 'all') {
      setScopes(['all']);
    } else {
      const newScopes = scopes.filter((s) => s !== 'all');
      if (newScopes.includes(scope)) {
        const filtered = newScopes.filter((s) => s !== scope);
        setScopes(filtered.length > 0 ? filtered : ['all']);
      } else {
        setScopes([...newScopes, scope]);
      }
    }
  };

  const validate = (): boolean => {
    const newErrors: Record<string, string> = {};

    if (!delegateUserId) {
      newErrors.delegateUserId = t('delegation.errors.delegateRequired');
    }

    if (scopes.length === 0) {
      newErrors.scopes = t('delegation.errors.scopeRequired');
    }

    if (startDate && endDate && new Date(startDate) > new Date(endDate)) {
      newErrors.endDate = t('delegation.errors.endDateBeforeStart');
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!validate()) return;

    onSubmit({
      delegateUserId,
      unitId: unitId || undefined,
      scopes,
      startDate: startDate || undefined,
      endDate: endDate || undefined,
    });
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Delegate User Selection */}
      <div>
        <label htmlFor="delegateUserId" className="block text-sm font-medium text-gray-700">
          {t('delegation.delegate')} <span className="text-red-500">*</span>
        </label>
        <select
          id="delegateUserId"
          value={delegateUserId}
          onChange={(e) => setDelegateUserId(e.target.value)}
          className={`mt-1 block w-full rounded-md border px-3 py-2 shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 ${
            errors.delegateUserId ? 'border-red-500' : 'border-gray-300'
          }`}
        >
          <option value="">{t('delegation.selectDelegate')}</option>
          {users.map((user) => (
            <option key={user.id} value={user.id}>
              {user.name} ({user.email})
            </option>
          ))}
        </select>
        {errors.delegateUserId && (
          <p className="mt-1 text-sm text-red-600">{errors.delegateUserId}</p>
        )}
      </div>

      {/* Unit Selection */}
      <div>
        <label htmlFor="unitId" className="block text-sm font-medium text-gray-700">
          {t('delegation.unit')} <span className="text-gray-400">({t('common.optional')})</span>
        </label>
        <select
          id="unitId"
          value={unitId}
          onChange={(e) => setUnitId(e.target.value)}
          className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="">{t('delegation.allUnits')}</option>
          {units.map((unit) => (
            <option key={unit.id} value={unit.id}>
              {unit.designation}
            </option>
          ))}
        </select>
        <p className="mt-1 text-xs text-gray-500">{t('delegation.unitHelp')}</p>
      </div>

      {/* Scopes Selection */}
      <div>
        <span className="block text-sm font-medium text-gray-700 mb-2">
          {t('delegation.scopes')} <span className="text-red-500">*</span>
        </span>
        <div className="space-y-2">
          {AVAILABLE_SCOPES.map((scope) => (
            <label key={scope} className="flex items-center gap-2">
              <input
                type="checkbox"
                checked={scopes.includes(scope)}
                onChange={() => handleScopeToggle(scope)}
                className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
              />
              <span className="text-sm text-gray-700">{scopeLabels[scope]}</span>
            </label>
          ))}
        </div>
        {errors.scopes && <p className="mt-1 text-sm text-red-600">{errors.scopes}</p>}
      </div>

      {/* Date Range */}
      <div className="grid grid-cols-2 gap-4">
        <div>
          <label htmlFor="startDate" className="block text-sm font-medium text-gray-700">
            {t('delegation.startDate')}{' '}
            <span className="text-gray-400">({t('common.optional')})</span>
          </label>
          <input
            type="date"
            id="startDate"
            value={startDate}
            onChange={(e) => setStartDate(e.target.value)}
            className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <p className="mt-1 text-xs text-gray-500">{t('delegation.startDateHelp')}</p>
        </div>
        <div>
          <label htmlFor="endDate" className="block text-sm font-medium text-gray-700">
            {t('delegation.endDate')}{' '}
            <span className="text-gray-400">({t('common.optional')})</span>
          </label>
          <input
            type="date"
            id="endDate"
            value={endDate}
            onChange={(e) => setEndDate(e.target.value)}
            className={`mt-1 block w-full rounded-md border px-3 py-2 shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 ${
              errors.endDate ? 'border-red-500' : 'border-gray-300'
            }`}
          />
          {errors.endDate && <p className="mt-1 text-sm text-red-600">{errors.endDate}</p>}
        </div>
      </div>

      {/* Actions */}
      <div className="flex justify-end gap-3 pt-4 border-t">
        <button
          type="button"
          onClick={onCancel}
          disabled={isSubmitting}
          className="px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 disabled:opacity-50"
        >
          {t('common.cancel')}
        </button>
        <button
          type="submit"
          disabled={isSubmitting}
          className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50"
        >
          {isSubmitting ? t('common.saving') : t('delegation.createDelegation')}
        </button>
      </div>
    </form>
  );
}
