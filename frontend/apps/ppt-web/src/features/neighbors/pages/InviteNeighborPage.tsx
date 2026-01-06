/**
 * InviteNeighborPage Component
 *
 * Page for inviting a new neighbor to the building.
 * Presentational component - receives data as props.
 */

import type React from 'react';
import { useCallback, useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { InviteNeighborFormData, UnitOption } from '../types';
import { NEIGHBOR_ROLE_OPTIONS } from '../types';

export interface InviteNeighborPageProps {
  units: UnitOption[];
  isLoading?: boolean;
  error?: string | null;
  isSubmitting?: boolean;
  successMessage?: string | null;
  onSubmit: (data: InviteNeighborFormData) => void;
  onBack?: () => void;
}

interface FormErrors {
  email?: string;
  unitId?: string;
  role?: string;
}

export const InviteNeighborPage: React.FC<InviteNeighborPageProps> = ({
  units,
  isLoading = false,
  error = null,
  isSubmitting = false,
  successMessage = null,
  onSubmit,
  onBack,
}) => {
  const { t } = useTranslation();
  const [formData, setFormData] = useState<InviteNeighborFormData>({
    email: '',
    firstName: '',
    lastName: '',
    unitId: '',
    role: 'tenant',
    personalMessage: '',
  });
  const [formErrors, setFormErrors] = useState<FormErrors>({});

  const validateForm = useCallback((): boolean => {
    const errors: FormErrors = {};

    if (!formData.email) {
      errors.email = t('neighbors.invitation.errors.emailRequired');
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.email)) {
      errors.email = t('neighbors.invitation.errors.emailInvalid');
    }

    if (!formData.unitId) {
      errors.unitId = t('neighbors.invitation.errors.unitRequired');
    }

    if (!formData.role) {
      errors.role = t('neighbors.invitation.errors.roleRequired');
    }

    setFormErrors(errors);
    return Object.keys(errors).length === 0;
  }, [formData, t]);

  const handleInputChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>) => {
      const { name, value } = e.target;
      setFormData((prev) => ({
        ...prev,
        [name]: value,
      }));
      // Clear error when user starts typing
      if (formErrors[name as keyof FormErrors]) {
        setFormErrors((prev) => ({
          ...prev,
          [name]: undefined,
        }));
      }
    },
    [formErrors]
  );

  const handleSubmit = useCallback(
    (e: React.FormEvent) => {
      e.preventDefault();
      if (validateForm()) {
        onSubmit(formData);
      }
    },
    [formData, onSubmit, validateForm]
  );

  if (isLoading) {
    return (
      <div className="invite-neighbor-page">
        <div className="invite-neighbor-page-loading">
          <div className="invite-neighbor-page-spinner" />
          <p>{t('common.loading')}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="invite-neighbor-page">
      <div className="invite-neighbor-page-header">
        {onBack && (
          <button
            type="button"
            onClick={onBack}
            className="invite-neighbor-page-back"
            aria-label={t('common.back')}
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              className="invite-neighbor-page-back-icon"
            >
              <path d="M19 12H5M12 19l-7-7 7-7" />
            </svg>
            {t('neighbors.backToNeighbors')}
          </button>
        )}
        <div className="invite-neighbor-page-title-section">
          <h1>{t('neighbors.invitation.title')}</h1>
          <p className="invite-neighbor-page-description">
            {t('neighbors.invitation.description')}
          </p>
        </div>
      </div>

      {error && (
        <div className="invite-neighbor-page-error" role="alert">
          <p>{error}</p>
        </div>
      )}

      {successMessage && (
        <div className="invite-neighbor-page-success" role="status">
          <p>{successMessage}</p>
        </div>
      )}

      <form onSubmit={handleSubmit} className="invite-neighbor-page-form">
        <div className="invite-neighbor-page-section">
          <h2 className="invite-neighbor-page-section-title">
            {t('neighbors.invitation.contactSection')}
          </h2>

          <div className="invite-neighbor-page-field">
            <label htmlFor="invite-email" className="invite-neighbor-page-label">
              {t('neighbors.invitation.email')} <span className="required">*</span>
            </label>
            <input
              type="email"
              id="invite-email"
              name="email"
              value={formData.email}
              onChange={handleInputChange}
              placeholder={t('neighbors.invitation.emailPlaceholder')}
              className={`invite-neighbor-page-input ${formErrors.email ? 'error' : ''}`}
              disabled={isSubmitting}
              required
            />
            {formErrors.email && (
              <p className="invite-neighbor-page-field-error">{formErrors.email}</p>
            )}
          </div>

          <div className="invite-neighbor-page-row">
            <div className="invite-neighbor-page-field">
              <label htmlFor="invite-firstName" className="invite-neighbor-page-label">
                {t('neighbors.invitation.firstName')}
              </label>
              <input
                type="text"
                id="invite-firstName"
                name="firstName"
                value={formData.firstName}
                onChange={handleInputChange}
                placeholder={t('neighbors.invitation.firstNamePlaceholder')}
                className="invite-neighbor-page-input"
                disabled={isSubmitting}
              />
            </div>

            <div className="invite-neighbor-page-field">
              <label htmlFor="invite-lastName" className="invite-neighbor-page-label">
                {t('neighbors.invitation.lastName')}
              </label>
              <input
                type="text"
                id="invite-lastName"
                name="lastName"
                value={formData.lastName}
                onChange={handleInputChange}
                placeholder={t('neighbors.invitation.lastNamePlaceholder')}
                className="invite-neighbor-page-input"
                disabled={isSubmitting}
              />
            </div>
          </div>
        </div>

        <div className="invite-neighbor-page-section">
          <h2 className="invite-neighbor-page-section-title">
            {t('neighbors.invitation.unitSection')}
          </h2>

          <div className="invite-neighbor-page-field">
            <label htmlFor="invite-unit" className="invite-neighbor-page-label">
              {t('neighbors.invitation.unit')} <span className="required">*</span>
            </label>
            <select
              id="invite-unit"
              name="unitId"
              value={formData.unitId}
              onChange={handleInputChange}
              className={`invite-neighbor-page-select ${formErrors.unitId ? 'error' : ''}`}
              disabled={isSubmitting}
              required
            >
              <option value="">{t('neighbors.invitation.selectUnit')}</option>
              {units.map((unit) => (
                <option key={unit.id} value={unit.id}>
                  {t('neighbors.unit')} {unit.number}
                  {unit.floor !== undefined && ` (${t('neighbors.floor')} ${unit.floor})`}
                </option>
              ))}
            </select>
            {formErrors.unitId && (
              <p className="invite-neighbor-page-field-error">{formErrors.unitId}</p>
            )}
          </div>

          <div className="invite-neighbor-page-field">
            <label htmlFor="invite-role" className="invite-neighbor-page-label">
              {t('neighbors.invitation.role')} <span className="required">*</span>
            </label>
            <select
              id="invite-role"
              name="role"
              value={formData.role}
              onChange={handleInputChange}
              className={`invite-neighbor-page-select ${formErrors.role ? 'error' : ''}`}
              disabled={isSubmitting}
              required
            >
              {NEIGHBOR_ROLE_OPTIONS.map((option) => (
                <option key={option.value} value={option.value}>
                  {t(option.labelKey)}
                </option>
              ))}
            </select>
            {formErrors.role && (
              <p className="invite-neighbor-page-field-error">{formErrors.role}</p>
            )}
          </div>
        </div>

        <div className="invite-neighbor-page-section">
          <h2 className="invite-neighbor-page-section-title">
            {t('neighbors.invitation.messageSection')}
          </h2>

          <div className="invite-neighbor-page-field">
            <label htmlFor="invite-message" className="invite-neighbor-page-label">
              {t('neighbors.invitation.personalMessage')}
            </label>
            <textarea
              id="invite-message"
              name="personalMessage"
              value={formData.personalMessage}
              onChange={handleInputChange}
              placeholder={t('neighbors.invitation.personalMessagePlaceholder')}
              className="invite-neighbor-page-textarea"
              rows={4}
              disabled={isSubmitting}
            />
            <p className="invite-neighbor-page-field-hint">
              {t('neighbors.invitation.personalMessageHint')}
            </p>
          </div>
        </div>

        <div className="invite-neighbor-page-actions">
          {onBack && (
            <button
              type="button"
              onClick={onBack}
              className="invite-neighbor-page-button secondary"
              disabled={isSubmitting}
            >
              {t('common.cancel')}
            </button>
          )}
          <button
            type="submit"
            className="invite-neighbor-page-button primary"
            disabled={isSubmitting}
          >
            {isSubmitting ? t('common.saving') : t('neighbors.invitation.sendInvitation')}
          </button>
        </div>
      </form>
    </div>
  );
};

InviteNeighborPage.displayName = 'InviteNeighborPage';
