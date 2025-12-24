/**
 * ContactForm Component
 *
 * Contact form for listing inquiries (Epic 44, Story 44.3 & 44.6).
 */

'use client';

import type { CreateInquiryRequest, InquiryType, ListingAgent } from '@ppt/reality-api-client';
import { useCreateInquiry } from '@ppt/reality-api-client';
import { useState } from 'react';

interface ContactFormProps {
  listingId: string;
  agent: ListingAgent;
}

export function ContactForm({ listingId, agent }: ContactFormProps) {
  const [formData, setFormData] = useState({
    name: '',
    email: '',
    phone: '',
    message: '',
    inquiryType: 'general' as InquiryType,
  });
  const [showSuccess, setShowSuccess] = useState(false);

  const createInquiry = useCreateInquiry();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const request: CreateInquiryRequest = {
      listingId,
      type: formData.inquiryType,
      message: formData.message,
      name: formData.name,
      email: formData.email,
      phone: formData.phone || undefined,
    };

    try {
      await createInquiry.mutateAsync(request);
      setShowSuccess(true);
      setFormData({
        name: '',
        email: '',
        phone: '',
        message: '',
        inquiryType: 'general',
      });
    } catch {
      // Error handled by mutation state
    }
  };

  if (showSuccess) {
    return (
      <div className="contact-form success">
        <div className="success-icon">
          <svg
            width="48"
            height="48"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            aria-hidden="true"
          >
            <circle cx="12" cy="12" r="10" />
            <path d="M9 12l2 2 4-4" />
          </svg>
        </div>
        <h3 className="success-title">Message Sent!</h3>
        <p className="success-text">
          Your inquiry has been sent to {agent.name}. They will get back to you soon.
        </p>
        <button type="button" className="new-message-button" onClick={() => setShowSuccess(false)}>
          Send another message
        </button>
        <style jsx>{`
          .contact-form.success {
            background: #fff;
            border-radius: 12px;
            padding: 32px;
            text-align: center;
          }
          .success-icon {
            color: #10b981;
            margin-bottom: 16px;
          }
          .success-title {
            font-size: 1.25rem;
            font-weight: 600;
            color: #111827;
            margin: 0 0 8px;
          }
          .success-text {
            color: #6b7280;
            margin: 0 0 24px;
          }
          .new-message-button {
            padding: 8px 16px;
            background: transparent;
            border: 1px solid #e5e7eb;
            border-radius: 8px;
            font-size: 14px;
            color: #374151;
            cursor: pointer;
          }
          .new-message-button:hover {
            background: #f9fafb;
          }
        `}</style>
      </div>
    );
  }

  return (
    <div className="contact-form">
      {/* Agent Info */}
      <div className="agent-info">
        <div className="agent-avatar">
          {agent.avatarUrl ? (
            <img src={agent.avatarUrl} alt={agent.name} />
          ) : (
            <span>{agent.name.charAt(0).toUpperCase()}</span>
          )}
        </div>
        <div className="agent-details">
          <p className="agent-name">{agent.name}</p>
          {agent.agencyName && <p className="agent-agency">{agent.agencyName}</p>}
        </div>
      </div>

      {/* Form */}
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor="inquiryType" className="form-label">
            I want to
          </label>
          <select
            id="inquiryType"
            className="form-select"
            value={formData.inquiryType}
            onChange={(e) =>
              setFormData({ ...formData, inquiryType: e.target.value as InquiryType })
            }
          >
            <option value="general">Ask a question</option>
            <option value="viewing_request">Schedule a viewing</option>
            <option value="price_negotiation">Discuss price</option>
            <option value="availability">Check availability</option>
          </select>
        </div>

        <div className="form-group">
          <label htmlFor="name" className="form-label">
            Name
          </label>
          <input
            id="name"
            type="text"
            className="form-input"
            value={formData.name}
            onChange={(e) => setFormData({ ...formData, name: e.target.value })}
            required
          />
        </div>

        <div className="form-group">
          <label htmlFor="email" className="form-label">
            Email
          </label>
          <input
            id="email"
            type="email"
            className="form-input"
            value={formData.email}
            onChange={(e) => setFormData({ ...formData, email: e.target.value })}
            required
          />
        </div>

        <div className="form-group">
          <label htmlFor="phone" className="form-label">
            Phone <span className="optional">(optional)</span>
          </label>
          <input
            id="phone"
            type="tel"
            className="form-input"
            value={formData.phone}
            onChange={(e) => setFormData({ ...formData, phone: e.target.value })}
          />
        </div>

        <div className="form-group">
          <label htmlFor="message" className="form-label">
            Message
          </label>
          <textarea
            id="message"
            className="form-textarea"
            rows={4}
            value={formData.message}
            onChange={(e) => setFormData({ ...formData, message: e.target.value })}
            placeholder="Hi, I'm interested in this property..."
            required
          />
        </div>

        {createInquiry.error && (
          <div className="error-message">Failed to send message. Please try again.</div>
        )}

        <button type="submit" className="submit-button" disabled={createInquiry.isPending}>
          {createInquiry.isPending ? 'Sending...' : 'Send Message'}
        </button>

        <p className="privacy-notice">By submitting, you agree to our privacy policy.</p>
      </form>

      <style jsx>{`
        .contact-form {
          background: #fff;
          border-radius: 12px;
          padding: 24px;
          box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
        }

        .agent-info {
          display: flex;
          align-items: center;
          gap: 12px;
          padding-bottom: 16px;
          margin-bottom: 16px;
          border-bottom: 1px solid #e5e7eb;
        }

        .agent-avatar {
          width: 48px;
          height: 48px;
          border-radius: 50%;
          background: #2563eb;
          color: #fff;
          display: flex;
          align-items: center;
          justify-content: center;
          font-size: 18px;
          font-weight: 600;
          overflow: hidden;
        }

        .agent-avatar img {
          width: 100%;
          height: 100%;
          object-fit: cover;
        }

        .agent-name {
          font-weight: 600;
          color: #111827;
          margin: 0;
        }

        .agent-agency {
          font-size: 14px;
          color: #6b7280;
          margin: 2px 0 0;
        }

        .form-group {
          margin-bottom: 16px;
        }

        .form-label {
          display: block;
          font-size: 14px;
          font-weight: 500;
          color: #374151;
          margin-bottom: 6px;
        }

        .optional {
          font-weight: 400;
          color: #9ca3af;
        }

        .form-input,
        .form-select,
        .form-textarea {
          width: 100%;
          padding: 10px 12px;
          border: 1px solid #e5e7eb;
          border-radius: 8px;
          font-size: 14px;
          color: #111827;
        }

        .form-input:focus,
        .form-select:focus,
        .form-textarea:focus {
          outline: none;
          border-color: #2563eb;
          box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
        }

        .form-textarea {
          resize: vertical;
          min-height: 100px;
        }

        .error-message {
          padding: 12px;
          background: #fef2f2;
          border: 1px solid #fecaca;
          border-radius: 8px;
          color: #dc2626;
          font-size: 14px;
          margin-bottom: 16px;
        }

        .submit-button {
          width: 100%;
          padding: 12px;
          background: #2563eb;
          color: #fff;
          border: none;
          border-radius: 8px;
          font-size: 16px;
          font-weight: 600;
          cursor: pointer;
          transition: background 0.2s;
        }

        .submit-button:hover:not(:disabled) {
          background: #1d4ed8;
        }

        .submit-button:disabled {
          opacity: 0.7;
          cursor: not-allowed;
        }

        .privacy-notice {
          font-size: 12px;
          color: #9ca3af;
          text-align: center;
          margin: 12px 0 0;
        }
      `}</style>
    </div>
  );
}
