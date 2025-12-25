/**
 * Emergency Contact Directory Page for Epic 62.
 *
 * Main page for viewing and managing emergency contacts.
 */

import {
  CONTACT_TYPE_LABELS,
  createEmergencyContact,
  deleteEmergencyContact,
  listEmergencyContacts,
  updateEmergencyContact,
} from '@ppt/api-client';
import type { CreateEmergencyContact, EmergencyContact } from '@ppt/api-client';
import type React from 'react';
import { useEffect, useState } from 'react';
import { EmergencyContactForm, EmergencyContactsList } from '../components';
import '../styles/emergency-contacts.css';

export const EmergencyContactDirectoryPage: React.FC = () => {
  // TODO: Replace with actual organization ID from auth context
  const organizationId = 'org-123';

  const [contacts, setContacts] = useState<EmergencyContact[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showForm, setShowForm] = useState(false);
  const [editingContact, setEditingContact] = useState<EmergencyContact | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  // Filters
  const [filterType, setFilterType] = useState<string>('');
  const [showInactive, setShowInactive] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');

  // Load contacts
  useEffect(() => {
    const loadContacts = async () => {
      try {
        setIsLoading(true);
        setError(null);

        const data = await listEmergencyContacts({
          organization_id: organizationId,
          ...(filterType && { contact_type: filterType }),
          ...(!showInactive && { is_active: true }),
        });

        setContacts(data);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load emergency contacts');
        console.error('Failed to load emergency contacts:', err);
      } finally {
        setIsLoading(false);
      }
    };

    loadContacts();
  }, [filterType, showInactive]);

  // Filter contacts by search query
  const filteredContacts = contacts.filter((contact) => {
    if (!searchQuery) return true;

    const query = searchQuery.toLowerCase();
    return (
      contact.name.toLowerCase().includes(query) ||
      contact.role.toLowerCase().includes(query) ||
      contact.phone?.toLowerCase().includes(query) ||
      contact.email?.toLowerCase().includes(query)
    );
  });

  // Reload contacts function for handlers
  const reloadContacts = async () => {
    try {
      const data = await listEmergencyContacts({
        organization_id: organizationId,
        ...(filterType && { contact_type: filterType }),
        ...(!showInactive && { is_active: true }),
      });
      setContacts(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to reload emergency contacts');
      console.error('Failed to reload emergency contacts:', err);
    }
  };

  const handleCreate = async (data: CreateEmergencyContact) => {
    try {
      setIsSubmitting(true);
      setError(null);

      await createEmergencyContact(organizationId, data);

      setShowForm(false);
      await reloadContacts();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create emergency contact');
      console.error('Failed to create emergency contact:', err);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleUpdate = async (data: CreateEmergencyContact) => {
    if (!editingContact) return;

    try {
      setIsSubmitting(true);
      setError(null);

      await updateEmergencyContact(editingContact.id, organizationId, data);

      setEditingContact(null);
      setShowForm(false);
      await reloadContacts();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to update emergency contact');
      console.error('Failed to update emergency contact:', err);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleDelete = async (contact: EmergencyContact) => {
    if (!confirm(`Are you sure you want to delete ${contact.name}?`)) {
      return;
    }

    try {
      setError(null);
      await deleteEmergencyContact(contact.id, organizationId);
      await reloadContacts();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to delete emergency contact');
      console.error('Failed to delete emergency contact:', err);
    }
  };

  const handleEdit = (contact: EmergencyContact) => {
    setEditingContact(contact);
    setShowForm(true);
  };

  const handleCancelForm = () => {
    setShowForm(false);
    setEditingContact(null);
  };

  const handleAddNew = () => {
    setEditingContact(null);
    setShowForm(true);
  };

  return (
    <div className="emergency-contact-directory-page">
      <div className="emergency-contact-directory-header">
        <h1>Emergency Contact Directory</h1>
        <p>Manage important emergency contacts for your building or organization.</p>
      </div>

      {error && (
        <div className="emergency-contact-directory-error" role="alert">
          {error}
        </div>
      )}

      {!showForm && (
        <div className="emergency-contact-directory-controls">
          <div className="emergency-contact-directory-filters">
            <div className="emergency-contact-directory-filter">
              <label htmlFor="search">Search</label>
              <input
                type="search"
                id="search"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="Search contacts..."
                className="emergency-contact-directory-search"
              />
            </div>

            <div className="emergency-contact-directory-filter">
              <label htmlFor="filterType">Type</label>
              <select
                id="filterType"
                value={filterType}
                onChange={(e) => setFilterType(e.target.value)}
                className="emergency-contact-directory-select"
              >
                <option value="">All Types</option>
                {Object.entries(CONTACT_TYPE_LABELS).map(([value, label]) => (
                  <option key={value} value={value}>
                    {label}
                  </option>
                ))}
              </select>
            </div>

            <div className="emergency-contact-directory-filter checkbox">
              <label htmlFor="showInactive">
                <input
                  type="checkbox"
                  id="showInactive"
                  checked={showInactive}
                  onChange={(e) => setShowInactive(e.target.checked)}
                />
                Show inactive
              </label>
            </div>
          </div>

          <button
            type="button"
            onClick={handleAddNew}
            className="emergency-contact-directory-add-button"
          >
            Add Emergency Contact
          </button>
        </div>
      )}

      {showForm ? (
        <div className="emergency-contact-directory-form-container">
          <EmergencyContactForm
            contact={editingContact || undefined}
            onSubmit={editingContact ? handleUpdate : handleCreate}
            onCancel={handleCancelForm}
            isSubmitting={isSubmitting}
          />
        </div>
      ) : isLoading ? (
        <div className="emergency-contact-directory-loading">Loading emergency contacts...</div>
      ) : (
        <EmergencyContactsList
          contacts={filteredContacts}
          onEdit={handleEdit}
          onDelete={handleDelete}
          showActions={true}
          groupByType={true}
        />
      )}
    </div>
  );
};

EmergencyContactDirectoryPage.displayName = 'EmergencyContactDirectoryPage';
