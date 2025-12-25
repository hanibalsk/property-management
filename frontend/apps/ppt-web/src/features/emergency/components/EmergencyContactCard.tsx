/**
 * Emergency Contact Card component for Epic 62.
 *
 * Displays an emergency contact in a card format with action buttons.
 */

import type { EmergencyContact } from '@ppt/api-client';
import { CONTACT_TYPE_LABELS } from '@ppt/api-client';
import type React from 'react';

export interface EmergencyContactCardProps {
  contact: EmergencyContact;
  onEdit?: (contact: EmergencyContact) => void;
  onDelete?: (contact: EmergencyContact) => void;
  showActions?: boolean;
}

export const EmergencyContactCard: React.FC<EmergencyContactCardProps> = ({
  contact,
  onEdit,
  onDelete,
  showActions = true,
}) => {
  const contactTypeLabel =
    CONTACT_TYPE_LABELS[contact.contact_type as keyof typeof CONTACT_TYPE_LABELS] || contact.role;

  return (
    <div className="emergency-contact-card">
      <div className="emergency-contact-card-header">
        <div>
          <h3 className="emergency-contact-card-name">{contact.name}</h3>
          <span className="emergency-contact-card-type">{contactTypeLabel}</span>
        </div>
        {!contact.is_active && <span className="emergency-contact-card-badge">Inactive</span>}
      </div>

      <div className="emergency-contact-card-body">
        <div className="emergency-contact-card-field">
          <span className="emergency-contact-card-label">Role:</span>
          <span className="emergency-contact-card-value">{contact.role}</span>
        </div>

        {contact.phone && (
          <div className="emergency-contact-card-field">
            <span className="emergency-contact-card-label">Phone:</span>
            <a href={`tel:${contact.phone}`} className="emergency-contact-card-value link">
              {contact.phone}
            </a>
          </div>
        )}

        {contact.phone_secondary && (
          <div className="emergency-contact-card-field">
            <span className="emergency-contact-card-label">Secondary Phone:</span>
            <a
              href={`tel:${contact.phone_secondary}`}
              className="emergency-contact-card-value link"
            >
              {contact.phone_secondary}
            </a>
          </div>
        )}

        {contact.email && (
          <div className="emergency-contact-card-field">
            <span className="emergency-contact-card-label">Email:</span>
            <a href={`mailto:${contact.email}`} className="emergency-contact-card-value link">
              {contact.email}
            </a>
          </div>
        )}

        {contact.address && (
          <div className="emergency-contact-card-field">
            <span className="emergency-contact-card-label">Address:</span>
            <span className="emergency-contact-card-value">{contact.address}</span>
          </div>
        )}

        {contact.available_hours && (
          <div className="emergency-contact-card-field">
            <span className="emergency-contact-card-label">Available Hours:</span>
            <span className="emergency-contact-card-value">{contact.available_hours}</span>
          </div>
        )}

        {contact.notes && (
          <div className="emergency-contact-card-field">
            <span className="emergency-contact-card-label">Notes:</span>
            <span className="emergency-contact-card-value">{contact.notes}</span>
          </div>
        )}

        <div className="emergency-contact-card-field">
          <span className="emergency-contact-card-label">Priority:</span>
          <span className="emergency-contact-card-value">{contact.priority_order}</span>
        </div>
      </div>

      {showActions && (onEdit || onDelete) && (
        <div className="emergency-contact-card-actions">
          {onEdit && (
            <button
              type="button"
              onClick={() => onEdit(contact)}
              className="emergency-contact-card-button edit"
              aria-label={`Edit ${contact.name}`}
            >
              Edit
            </button>
          )}
          {onDelete && (
            <button
              type="button"
              onClick={() => onDelete(contact)}
              className="emergency-contact-card-button delete"
              aria-label={`Delete ${contact.name}`}
            >
              Delete
            </button>
          )}
        </div>
      )}
    </div>
  );
};

EmergencyContactCard.displayName = 'EmergencyContactCard';
