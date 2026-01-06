/**
 * RecipientSelector Component
 *
 * Multi-select component for choosing message recipients.
 */

import { useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { RecipientOption } from '../types';

interface RecipientSelectorProps {
  recipients: RecipientOption[];
  selectedIds: string[];
  onSelectionChange: (ids: string[]) => void;
  isLoading?: boolean;
  placeholder?: string;
}

export function RecipientSelector({
  recipients,
  selectedIds,
  onSelectionChange,
  isLoading,
  placeholder,
}: RecipientSelectorProps) {
  const { t } = useTranslation();
  const [searchQuery, setSearchQuery] = useState('');
  const [isOpen, setIsOpen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const selectedRecipients = recipients.filter((r) => selectedIds.includes(r.id));
  const filteredRecipients = recipients.filter(
    (r) =>
      !selectedIds.includes(r.id) &&
      (r.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        r.email?.toLowerCase().includes(searchQuery.toLowerCase()))
  );

  // Close dropdown when clicking outside
  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (containerRef.current && !containerRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    }
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const handleSelectRecipient = (id: string) => {
    onSelectionChange([...selectedIds, id]);
    setSearchQuery('');
    inputRef.current?.focus();
  };

  const handleRemoveRecipient = (id: string) => {
    onSelectionChange(selectedIds.filter((sid) => sid !== id));
  };

  return (
    <div ref={containerRef} className="relative">
      <div
        className={`min-h-[42px] px-3 py-2 border rounded-md flex flex-wrap gap-2 items-center cursor-text ${
          isOpen ? 'ring-2 ring-blue-500 border-blue-500' : 'border-gray-300'
        }`}
        onClick={() => {
          setIsOpen(true);
          inputRef.current?.focus();
        }}
        onKeyDown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            setIsOpen(true);
            inputRef.current?.focus();
          }
        }}
        role="combobox"
        aria-expanded={isOpen}
        aria-haspopup="listbox"
        aria-controls="recipient-listbox"
        tabIndex={0}
      >
        {/* Selected Recipients */}
        {selectedRecipients.map((recipient) => (
          <span
            key={recipient.id}
            className="inline-flex items-center gap-1 px-2 py-1 bg-blue-100 text-blue-800 text-sm rounded-full"
          >
            {recipient.avatar ? (
              <img src={recipient.avatar} alt="" className="w-4 h-4 rounded-full object-cover" />
            ) : (
              <span className="w-4 h-4 rounded-full bg-blue-300 flex items-center justify-center text-xs">
                {recipient.name.charAt(0).toUpperCase()}
              </span>
            )}
            {recipient.name}
            <button
              type="button"
              onClick={(e) => {
                e.stopPropagation();
                handleRemoveRecipient(recipient.id);
              }}
              className="ml-1 text-blue-600 hover:text-blue-800"
              aria-label={`Remove ${recipient.name}`}
            >
              <svg className="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                <path
                  fillRule="evenodd"
                  d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                  clipRule="evenodd"
                />
              </svg>
            </button>
          </span>
        ))}

        {/* Search Input */}
        <input
          ref={inputRef}
          type="text"
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          onFocus={() => setIsOpen(true)}
          placeholder={
            selectedIds.length === 0 ? placeholder || t('messaging.searchRecipients') : ''
          }
          className="flex-1 min-w-[120px] outline-none text-sm"
        />
      </div>

      {/* Dropdown */}
      {isOpen && (
        <div className="absolute z-10 mt-1 w-full bg-white border border-gray-300 rounded-md shadow-lg max-h-60 overflow-auto">
          {isLoading ? (
            <div className="p-4 text-center text-gray-500 text-sm">{t('common.loading')}</div>
          ) : filteredRecipients.length === 0 ? (
            <div className="p-4 text-center text-gray-500 text-sm">
              {searchQuery ? t('messaging.noRecipientsFound') : t('messaging.noMoreRecipients')}
            </div>
          ) : (
            <div id="recipient-listbox" role="listbox" tabIndex={-1}>
              {filteredRecipients.map((recipient) => (
                <div key={recipient.id}>
                  <button
                    type="button"
                    onClick={() => handleSelectRecipient(recipient.id)}
                    className="w-full px-4 py-2 text-left hover:bg-gray-100 flex items-center gap-3"
                    role="option"
                    aria-selected={false}
                  >
                    {recipient.avatar ? (
                      <img
                        src={recipient.avatar}
                        alt=""
                        className="w-8 h-8 rounded-full object-cover"
                      />
                    ) : (
                      <div className="w-8 h-8 rounded-full bg-gray-300 flex items-center justify-center">
                        <span className="text-gray-600 text-sm font-medium">
                          {recipient.name.charAt(0).toUpperCase()}
                        </span>
                      </div>
                    )}
                    <div>
                      <div className="text-sm font-medium text-gray-900">{recipient.name}</div>
                      {recipient.email && (
                        <div className="text-xs text-gray-500">{recipient.email}</div>
                      )}
                    </div>
                  </button>
                </div>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
