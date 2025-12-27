/**
 * Webhook Secret Dialog Component (Epic 69 - Story 69.3)
 *
 * Displays webhook secret after creation or rotation.
 */

import { useState } from 'react';
import type { CreateWebhookResponse, RotateWebhookSecretResponse } from '../types';

interface WebhookSecretDialogProps {
  isOpen: boolean;
  onClose: () => void;
  webhook: CreateWebhookResponse | null;
  rotatedSecret?: RotateWebhookSecretResponse | null;
}

export function WebhookSecretDialog({
  isOpen,
  onClose,
  webhook,
  rotatedSecret,
}: WebhookSecretDialogProps) {
  const [copied, setCopied] = useState(false);
  const [confirmed, setConfirmed] = useState(false);

  const secret = rotatedSecret?.newSecret || webhook?.secret || '';
  const title = rotatedSecret ? 'Webhook Secret Rotated' : 'Webhook Created';

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(secret);
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

  if (!isOpen || (!webhook && !rotatedSecret)) return null;

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
              <h2 className="text-xl font-semibold">{title}</h2>
              {webhook && <p className="text-sm text-muted-foreground">{webhook.name}</p>}
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
                  Save this secret - you won't see it again!
                </h3>
                <p className="text-sm text-yellow-700 mt-1">
                  Use this secret to verify webhook signatures. Store it securely in your
                  application.
                </p>
              </div>
            </div>
          </div>

          {/* Secret */}
          <div>
            <label className="block text-sm font-medium mb-2">Webhook Secret</label>
            <div className="flex gap-2">
              <code className="flex-1 px-4 py-3 bg-gray-900 text-green-400 rounded-md font-mono text-sm overflow-x-auto">
                {secret}
              </code>
              <button
                type="button"
                onClick={handleCopy}
                className={`px-4 py-2 rounded-md transition-colors ${
                  copied ? 'bg-green-600 text-white' : 'bg-gray-100 hover:bg-gray-200 text-gray-700'
                }`}
              >
                {copied ? 'Copied!' : 'Copy'}
              </button>
            </div>
          </div>

          {/* Verification Example */}
          <div>
            <h4 className="text-sm font-medium mb-2">Signature Verification Example</h4>
            <pre className="p-4 bg-gray-900 text-gray-100 rounded-md text-sm overflow-x-auto">
              <code>
                {`// Node.js example
const crypto = require('crypto');

function verifyWebhookSignature(payload, signature, secret) {
  const expectedSignature = crypto
    .createHmac('sha256', secret)
    .update(payload)
    .digest('hex');

  return crypto.timingSafeEqual(
    Buffer.from(signature),
    Buffer.from('sha256=' + expectedSignature)
  );
}`}
              </code>
            </pre>
          </div>

          {/* Confirmation */}
          <label className="flex items-start gap-3 p-3 bg-gray-50 rounded-md cursor-pointer">
            <input
              type="checkbox"
              checked={confirmed}
              onChange={(e) => setConfirmed(e.target.checked)}
              className="mt-0.5"
            />
            <span className="text-sm">
              I have copied and securely stored this webhook secret. I understand it cannot be
              retrieved again.
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
