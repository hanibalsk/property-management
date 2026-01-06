/**
 * PrivacySettingsPage Component
 *
 * Page for managing user profile visibility settings.
 * Presentational component - receives data as props.
 */

import type React from 'react';
import { useTranslation } from 'react-i18next';
import { PrivacySettingsForm } from '../components';
import type { PrivacySettings } from '../types';

export interface PrivacySettingsPageProps {
  settings?: PrivacySettings;
  isLoading?: boolean;
  error?: string | null;
  isSubmitting?: boolean;
  successMessage?: string | null;
  onSubmit: (settings: PrivacySettings) => void;
  onBack?: () => void;
}

export const PrivacySettingsPage: React.FC<PrivacySettingsPageProps> = ({
  settings,
  isLoading = false,
  error = null,
  isSubmitting = false,
  successMessage = null,
  onSubmit,
  onBack,
}) => {
  const { t } = useTranslation();

  if (isLoading) {
    return (
      <div className="privacy-settings-page">
        <div className="privacy-settings-page-loading">
          <div className="privacy-settings-page-spinner" />
          <p>{t('neighbors.privacy.loading')}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="privacy-settings-page">
      <div className="privacy-settings-page-header">
        {onBack && (
          <button
            type="button"
            onClick={onBack}
            className="privacy-settings-page-back"
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
              className="privacy-settings-page-back-icon"
            >
              <path d="M19 12H5M12 19l-7-7 7-7" />
            </svg>
            {t('common.back')}
          </button>
        )}
        <div className="privacy-settings-page-title-section">
          <h1>{t('neighbors.privacy.title')}</h1>
          <p className="privacy-settings-page-description">{t('neighbors.privacy.description')}</p>
        </div>
      </div>

      {error && (
        <div className="privacy-settings-page-error" role="alert">
          <p>{error}</p>
        </div>
      )}

      {successMessage && (
        <div className="privacy-settings-page-success" role="status">
          <p>{successMessage}</p>
        </div>
      )}

      <div className="privacy-settings-page-content">
        <div className="privacy-settings-page-info">
          <h2>{t('neighbors.privacy.infoTitle')}</h2>
          <p>{t('neighbors.privacy.infoDescription')}</p>
          <ul className="privacy-settings-page-info-list">
            <li>
              <strong>{t('neighbors.privacy.visibilityPublic')}:</strong>{' '}
              {t('neighbors.privacy.visibilityPublicDescription')}
            </li>
            <li>
              <strong>{t('neighbors.privacy.visibilityBuilding')}:</strong>{' '}
              {t('neighbors.privacy.visibilityBuildingDescription')}
            </li>
            <li>
              <strong>{t('neighbors.privacy.visibilityNeighbors')}:</strong>{' '}
              {t('neighbors.privacy.visibilityNeighborsDescription')}
            </li>
            <li>
              <strong>{t('neighbors.privacy.visibilityPrivate')}:</strong>{' '}
              {t('neighbors.privacy.visibilityPrivateDescription')}
            </li>
          </ul>
        </div>

        <div className="privacy-settings-page-form">
          <PrivacySettingsForm
            initialSettings={settings}
            onSubmit={onSubmit}
            onCancel={onBack}
            isSubmitting={isSubmitting}
          />
        </div>
      </div>
    </div>
  );
};

PrivacySettingsPage.displayName = 'PrivacySettingsPage';
