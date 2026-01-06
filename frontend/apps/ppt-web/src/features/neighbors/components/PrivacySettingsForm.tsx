/**
 * PrivacySettingsForm Component
 *
 * Form for managing user profile visibility settings.
 * Allows users to control what information is visible to others.
 */

import type React from 'react';
import { useCallback, useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { PrivacySettings, VisibilityLevel } from '../types';
import { DEFAULT_PRIVACY_SETTINGS, VISIBILITY_OPTIONS } from '../types';

export interface PrivacySettingsFormProps {
  initialSettings?: PrivacySettings;
  onSubmit: (settings: PrivacySettings) => void;
  onCancel?: () => void;
  isSubmitting?: boolean;
}

export const PrivacySettingsForm: React.FC<PrivacySettingsFormProps> = ({
  initialSettings = DEFAULT_PRIVACY_SETTINGS,
  onSubmit,
  onCancel,
  isSubmitting = false,
}) => {
  const { t } = useTranslation();
  const [settings, setSettings] = useState<PrivacySettings>(initialSettings);

  const handleVisibilityChange = useCallback(
    (field: keyof Omit<PrivacySettings, 'listedInDirectory'>, value: VisibilityLevel) => {
      setSettings((prev) => ({
        ...prev,
        [field]: value,
      }));
    },
    []
  );

  const handleDirectoryToggle = useCallback((checked: boolean) => {
    setSettings((prev) => ({
      ...prev,
      listedInDirectory: checked,
    }));
  }, []);

  const handleSubmit = useCallback(
    (e: React.FormEvent) => {
      e.preventDefault();
      onSubmit(settings);
    },
    [onSubmit, settings]
  );

  const renderVisibilitySelect = (
    field: keyof Omit<PrivacySettings, 'listedInDirectory'>,
    label: string,
    description?: string
  ) => (
    <div className="privacy-settings-field">
      <div className="privacy-settings-field-header">
        <label htmlFor={`privacy-${field}`} className="privacy-settings-label">
          {label}
        </label>
        {description && <p className="privacy-settings-description">{description}</p>}
      </div>
      <select
        id={`privacy-${field}`}
        value={settings[field]}
        onChange={(e) => handleVisibilityChange(field, e.target.value as VisibilityLevel)}
        className="privacy-settings-select"
        disabled={isSubmitting}
      >
        {VISIBILITY_OPTIONS.map((option) => (
          <option key={option.value} value={option.value}>
            {t(option.labelKey)}
          </option>
        ))}
      </select>
    </div>
  );

  return (
    <form onSubmit={handleSubmit} className="privacy-settings-form">
      <div className="privacy-settings-section">
        <h3 className="privacy-settings-section-title">
          {t('neighbors.privacy.directorySection')}
        </h3>

        <div className="privacy-settings-field checkbox">
          <label htmlFor="privacy-listed" className="privacy-settings-checkbox-label">
            <input
              type="checkbox"
              id="privacy-listed"
              checked={settings.listedInDirectory}
              onChange={(e) => handleDirectoryToggle(e.target.checked)}
              disabled={isSubmitting}
              className="privacy-settings-checkbox"
            />
            <span>{t('neighbors.privacy.listedInDirectory')}</span>
          </label>
          <p className="privacy-settings-description">
            {t('neighbors.privacy.listedInDirectoryDescription')}
          </p>
        </div>
      </div>

      <div className="privacy-settings-section">
        <h3 className="privacy-settings-section-title">{t('neighbors.privacy.profileSection')}</h3>

        {renderVisibilitySelect(
          'showName',
          t('neighbors.privacy.showName'),
          t('neighbors.privacy.showNameDescription')
        )}

        {renderVisibilitySelect(
          'showAvatar',
          t('neighbors.privacy.showAvatar'),
          t('neighbors.privacy.showAvatarDescription')
        )}

        {renderVisibilitySelect(
          'showUnit',
          t('neighbors.privacy.showUnit'),
          t('neighbors.privacy.showUnitDescription')
        )}

        {renderVisibilitySelect(
          'showBio',
          t('neighbors.privacy.showBio'),
          t('neighbors.privacy.showBioDescription')
        )}
      </div>

      <div className="privacy-settings-section">
        <h3 className="privacy-settings-section-title">{t('neighbors.privacy.contactSection')}</h3>

        {renderVisibilitySelect(
          'showEmail',
          t('neighbors.privacy.showEmail'),
          t('neighbors.privacy.showEmailDescription')
        )}

        {renderVisibilitySelect(
          'showPhone',
          t('neighbors.privacy.showPhone'),
          t('neighbors.privacy.showPhoneDescription')
        )}
      </div>

      <div className="privacy-settings-section">
        <h3 className="privacy-settings-section-title">{t('neighbors.privacy.otherSection')}</h3>

        {renderVisibilitySelect(
          'showMoveInDate',
          t('neighbors.privacy.showMoveInDate'),
          t('neighbors.privacy.showMoveInDateDescription')
        )}
      </div>

      <div className="privacy-settings-actions">
        {onCancel && (
          <button
            type="button"
            onClick={onCancel}
            className="privacy-settings-button secondary"
            disabled={isSubmitting}
          >
            {t('common.cancel')}
          </button>
        )}
        <button type="submit" className="privacy-settings-button primary" disabled={isSubmitting}>
          {isSubmitting ? t('common.saving') : t('common.save')}
        </button>
      </div>
    </form>
  );
};

PrivacySettingsForm.displayName = 'PrivacySettingsForm';
