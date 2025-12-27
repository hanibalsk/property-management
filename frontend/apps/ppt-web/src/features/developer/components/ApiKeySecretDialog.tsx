/**
 * API Key Secret Dialog Component (Epic 69 - Story 69.1)
 *
 * Displays the newly created API key secret with a warning that it
 * can only be viewed once.
 */

import { useState } from 'react';
import type { CreateApiKeyResponse } from '../types';

interface ApiKeySecretDialogProps {
  isOpen: boolean;
  onClose: () => void;
  apiKey: CreateApiKeyResponse | null;
}

export function ApiKeySecretDialog({ isOpen, onClose, apiKey }: ApiKeySecretDialogProps) {
  const [copied, setCopied] = useState(false);
  const [confirmed, setConfirmed] = useState(false);

  const handleCopy = async () => {
    if (!apiKey) return;

    try {
      await navigator.clipboard.writeText(apiKey.secret);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  };

  const handleClose = () => {
    setCopied(false);
    setConfirmed(false);
    onClose();
  };

  if (!isOpen || !apiKey) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="bg-white rounded-lg shadow-xl max-w-lg w-full mx-4">
        <div className="p-6 border-b">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-green-100 rounded-full">
              <svg
                className="w-6 h-6 text-green-600"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M5 13l4 4L19 7"
                />
              </svg>
            </div>
            <div>
              <h2 className="text-xl font-semibold">API Key Created</h2>
              <p className="text-sm text-muted-foreground">{apiKey.name}</p>
            </div>
          </div>
        </div>

        <div className="p-6 space-y-6">
          {/* Warning */}
          <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-md">
            <div className="flex gap-3">
              <svg
                className="w-5 h-5 text-yellow-600 flex-shrink-0 mt-0.5"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                />
              </svg>
              <div>
                <h3 className="font-medium text-yellow-800">
                  Save this secret key - you won't see it again!
                </h3>
                <p className="text-sm text-yellow-700 mt-1">
                  For security reasons, we cannot show you this key again. Please copy and store it
                  securely.
                </p>
              </div>
            </div>
          </div>

          {/* Secret Key */}
          <div>
            <label className="block text-sm font-medium mb-2">Your API Key</label>
            <div className="flex gap-2">
              <code className="flex-1 px-4 py-3 bg-gray-900 text-green-400 rounded-md font-mono text-sm overflow-x-auto">
                {apiKey.secret}
              </code>
              <button
                type="button"
                onClick={handleCopy}
                className={`px-4 py-2 rounded-md transition-colors ${
                  copied ? 'bg-green-600 text-white' : 'bg-gray-100 hover:bg-gray-200 text-gray-700'
                }`}
              >
                {copied ? (
                  <span className="flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M5 13l4 4L19 7"
                      />
                    </svg>
                    Copied
                  </span>
                ) : (
                  <span className="flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3"
                      />
                    </svg>
                    Copy
                  </span>
                )}
              </button>
            </div>
          </div>

          {/* Key Details */}
          <div className="p-4 bg-gray-50 rounded-md">
            <h4 className="text-sm font-medium mb-2">Key Details</h4>
            <dl className="grid grid-cols-2 gap-2 text-sm">
              <dt className="text-muted-foreground">Key ID</dt>
              <dd className="font-mono text-xs">{apiKey.id}</dd>
              <dt className="text-muted-foreground">Prefix</dt>
              <dd className="font-mono">{apiKey.keyPrefix}</dd>
              <dt className="text-muted-foreground">Scopes</dt>
              <dd>{apiKey.scopes.join(', ')}</dd>
              <dt className="text-muted-foreground">Expires</dt>
              <dd>{apiKey.expiresAt ? formatDate(apiKey.expiresAt) : 'Never'}</dd>
            </dl>
          </div>

          {/* Usage Example */}
          <div>
            <h4 className="text-sm font-medium mb-2">Usage Example</h4>
            <pre className="p-4 bg-gray-900 text-gray-100 rounded-md text-sm overflow-x-auto">
              <code>
                {`curl -X GET "https://api.ppt.example.com/v1/buildings" \\
  -H "Authorization: Bearer ${apiKey.secret}"`}
              </code>
            </pre>
          </div>

          {/* Confirmation Checkbox */}
          <label className="flex items-start gap-3 p-3 bg-gray-50 rounded-md cursor-pointer">
            <input
              type="checkbox"
              checked={confirmed}
              onChange={(e) => setConfirmed(e.target.checked)}
              className="mt-0.5"
            />
            <span className="text-sm">
              I have copied and securely stored this API key. I understand it cannot be retrieved
              again.
            </span>
          </label>

          {/* Actions */}
          <div className="flex justify-end pt-4 border-t">
            <button
              type="button"
              onClick={handleClose}
              disabled={!confirmed}
              className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Done
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  });
}
