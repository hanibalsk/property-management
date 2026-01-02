/**
 * Agency Import Page
 *
 * Property import management (Epic 46).
 */

'use client';

import { CrmConnection, CsvImport, FeedImport } from '@/components/import';
import Link from 'next/link';
import { useState } from 'react';

type ImportTab = 'csv' | 'crm' | 'feed';

export default function AgencyImportPage() {
  const [activeTab, setActiveTab] = useState<ImportTab>('csv');

  return (
    <div className="import-page">
      <div className="page-header">
        <div>
          <Link href="/agency" className="back-link">
            ← Back to Dashboard
          </Link>
          <h1>Import Listings</h1>
          <p className="subtitle">Bulk import properties from various sources</p>
        </div>
      </div>

      <div className="tabs">
        <button
          type="button"
          className={`tab ${activeTab === 'csv' ? 'active' : ''}`}
          onClick={() => setActiveTab('csv')}
        >
          <svg
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            aria-hidden="true"
          >
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
            <polyline points="14 2 14 8 20 8" />
            <line x1="12" y1="18" x2="12" y2="12" />
            <line x1="9" y1="15" x2="15" y2="15" />
          </svg>
          CSV Import
        </button>
        <button
          type="button"
          className={`tab ${activeTab === 'crm' ? 'active' : ''}`}
          onClick={() => setActiveTab('crm')}
        >
          <svg
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            aria-hidden="true"
          >
            <path d="M20 7h-4m0 0V3m0 4l4-4M4 17h4m0 0v4m0-4l-4 4" />
            <circle cx="12" cy="12" r="3" />
          </svg>
          CRM Connections
        </button>
        <button
          type="button"
          className={`tab ${activeTab === 'feed' ? 'active' : ''}`}
          onClick={() => setActiveTab('feed')}
        >
          <svg
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            aria-hidden="true"
          >
            <path d="M4 11a9 9 0 0 1 9 9" />
            <path d="M4 4a16 16 0 0 1 16 16" />
            <circle cx="5" cy="19" r="2" />
          </svg>
          Feed Sources
        </button>
      </div>

      <div className="tab-content">
        {activeTab === 'csv' && <CsvImport />}
        {activeTab === 'crm' && <CrmConnection />}
        {activeTab === 'feed' && <FeedImport />}
      </div>

      <style jsx>{`
        .import-page {
          min-height: 100vh;
          background: #f9fafb;
        }

        .page-header {
          background: #fff;
          border-bottom: 1px solid #e5e7eb;
          padding: 24px 32px;
        }

        .back-link {
          font-size: 14px;
          color: #6b7280;
          text-decoration: none;
          display: inline-block;
          margin-bottom: 8px;
        }

        .back-link:hover {
          color: #2563eb;
        }

        h1 {
          font-size: 1.75rem;
          font-weight: bold;
          color: #111827;
          margin: 0 0 4px;
        }

        .subtitle {
          color: #6b7280;
          margin: 0;
        }

        .tabs {
          display: flex;
          gap: 4px;
          padding: 16px 32px;
          background: #fff;
          border-bottom: 1px solid #e5e7eb;
        }

        .tab {
          display: flex;
          align-items: center;
          gap: 8px;
          padding: 12px 20px;
          background: transparent;
          border: none;
          border-radius: 8px;
          font-size: 14px;
          font-weight: 500;
          color: #6b7280;
          cursor: pointer;
          transition: all 0.2s;
        }

        .tab:hover {
          background: #f3f4f6;
          color: #374151;
        }

        .tab.active {
          background: #eff6ff;
          color: #2563eb;
        }

        .tab-content {
          max-width: 1200px;
          margin: 0 auto;
          padding: 24px;
        }
      `}</style>
    </div>
  );
}
