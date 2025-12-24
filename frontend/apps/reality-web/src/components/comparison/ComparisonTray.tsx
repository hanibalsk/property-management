/**
 * Floating comparison tray component.
 *
 * Epic 51 - Story 51.1: Add to Comparison
 */

'use client';

import Link from 'next/link';

import { useComparison } from '../../lib/comparison-context';

export function ComparisonTray() {
  const { listings, removeFromComparison, clearComparison } = useComparison();

  if (listings.length === 0) {
    return null;
  }

  return (
    <div className="comparison-tray">
      <div className="tray-header">
        <span className="count">{listings.length}/4 properties to compare</span>
        <button type="button" className="clear-btn" onClick={clearComparison}>
          Clear all
        </button>
      </div>

      <div className="listings-preview">
        {listings.map((listing) => (
          <div key={listing.id} className="preview-item">
            <div className="preview-image">
              {listing.primaryPhoto ? (
                <img src={listing.primaryPhoto.thumbnailUrl} alt={`Thumbnail for ${listing.title}`} />
              ) : (
                <div className="no-image">🏠</div>
              )}
            </div>
            <button
              type="button"
              className="remove-btn"
              onClick={() => removeFromComparison(listing.id)}
              aria-label={`Remove ${listing.title} from comparison`}
            >
              ×
            </button>
          </div>
        ))}
        {/* Empty slots */}
        {Array.from({ length: 4 - listings.length }).map((_, i) => (
          // biome-ignore lint/suspicious/noArrayIndexKey: static empty slots
          <div key={`empty-${i}`} className="preview-item empty">
            <div className="preview-image empty-slot">
              <span>+</span>
            </div>
          </div>
        ))}
      </div>

      <Link
        href="/compare"
        className={`compare-btn ${listings.length < 2 ? 'disabled' : ''}`}
        aria-disabled={listings.length < 2}
        onClick={(e) => listings.length < 2 && e.preventDefault()}
      >
        Compare {listings.length < 2 ? '(need at least 2)' : `${listings.length} properties`}
      </Link>

      <style jsx>{`
        .comparison-tray {
          position: fixed;
          bottom: 24px;
          left: 50%;
          transform: translateX(-50%);
          background: white;
          border-radius: 16px;
          box-shadow:
            0 10px 40px rgba(0, 0, 0, 0.15),
            0 2px 10px rgba(0, 0, 0, 0.1);
          padding: 16px;
          z-index: 1000;
          display: flex;
          flex-direction: column;
          gap: 12px;
          min-width: 360px;
        }

        .tray-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
        }

        .count {
          font-size: 14px;
          color: #374151;
          font-weight: 500;
        }

        .clear-btn {
          background: none;
          border: none;
          color: #6b7280;
          font-size: 13px;
          cursor: pointer;
          padding: 4px 8px;
        }

        .clear-btn:hover {
          color: #ef4444;
        }

        .listings-preview {
          display: flex;
          gap: 8px;
        }

        .preview-item {
          position: relative;
          width: 72px;
          height: 54px;
        }

        .preview-image {
          width: 100%;
          height: 100%;
          border-radius: 8px;
          overflow: hidden;
          background: #f3f4f6;
        }

        .preview-image img {
          width: 100%;
          height: 100%;
          object-fit: cover;
        }

        .preview-image.empty-slot {
          display: flex;
          align-items: center;
          justify-content: center;
          border: 2px dashed #d1d5db;
          color: #9ca3af;
          font-size: 20px;
        }

        .no-image {
          width: 100%;
          height: 100%;
          display: flex;
          align-items: center;
          justify-content: center;
          background: #e5e7eb;
          font-size: 20px;
        }

        .remove-btn {
          position: absolute;
          top: -6px;
          right: -6px;
          width: 20px;
          height: 20px;
          border-radius: 50%;
          background: #ef4444;
          color: white;
          border: 2px solid white;
          font-size: 14px;
          line-height: 1;
          cursor: pointer;
          display: flex;
          align-items: center;
          justify-content: center;
        }

        .remove-btn:hover {
          background: #dc2626;
        }

        .compare-btn {
          display: block;
          text-align: center;
          background: #2563eb;
          color: white;
          padding: 12px 24px;
          border-radius: 8px;
          text-decoration: none;
          font-weight: 600;
          font-size: 14px;
          transition: background 0.2s;
        }

        .compare-btn:hover:not(.disabled) {
          background: #1d4ed8;
        }

        .compare-btn.disabled {
          background: #9ca3af;
          cursor: not-allowed;
        }

        @media (max-width: 480px) {
          .comparison-tray {
            min-width: unset;
            left: 16px;
            right: 16px;
            transform: none;
          }
        }
      `}</style>
    </div>
  );
}
