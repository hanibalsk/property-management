/**
 * Compare button component for listing cards.
 *
 * Epic 51 - Story 51.1: Add to Comparison
 */

'use client';

import type { ListingSummary } from '@ppt/reality-api-client';
import { useState } from 'react';

import { useComparison } from '../../lib/comparison-context';

interface CompareButtonProps {
  listing: ListingSummary;
  className?: string;
}

export function CompareButton({ listing, className = '' }: CompareButtonProps) {
  const { isInComparison, addToComparison, removeFromComparison, canAddMore } = useComparison();
  const [showMaxWarning, setShowMaxWarning] = useState(false);

  const inComparison = isInComparison(listing.id);

  const handleClick = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();

    if (inComparison) {
      removeFromComparison(listing.id);
    } else {
      if (!canAddMore) {
        setShowMaxWarning(true);
        setTimeout(() => setShowMaxWarning(false), 3000);
        return;
      }
      addToComparison(listing);
    }
  };

  return (
    <div className="compare-button-wrapper">
      <button
        type="button"
        className={`compare-button ${inComparison ? 'active' : ''} ${className}`}
        onClick={handleClick}
        title={inComparison ? 'Remove from comparison' : 'Add to comparison'}
      >
        <svg
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          aria-hidden="true"
        >
          <line x1="18" y1="20" x2="18" y2="10" />
          <line x1="12" y1="20" x2="12" y2="4" />
          <line x1="6" y1="20" x2="6" y2="14" />
        </svg>
        {inComparison ? 'Comparing' : 'Compare'}
      </button>
      {showMaxWarning && (
        <div className="max-warning" role="alert">
          Maximum 4 properties. Remove one first.
        </div>
      )}

      <style jsx>{`
        .compare-button-wrapper {
          position: relative;
          display: inline-block;
        }

        .compare-button {
          display: flex;
          align-items: center;
          gap: 6px;
          padding: 6px 12px;
          border: 1px solid #d1d5db;
          border-radius: 6px;
          background: white;
          color: #6b7280;
          font-size: 13px;
          font-weight: 500;
          cursor: pointer;
          transition: all 0.2s;
        }

        .compare-button:hover {
          border-color: #2563eb;
          color: #2563eb;
        }

        .compare-button.active {
          background: #2563eb;
          border-color: #2563eb;
          color: white;
        }

        .compare-button.active:hover {
          background: #1d4ed8;
        }

        .max-warning {
          position: absolute;
          bottom: calc(100% + 8px);
          left: 50%;
          transform: translateX(-50%);
          white-space: nowrap;
          padding: 8px 12px;
          background: #374151;
          color: white;
          font-size: 12px;
          border-radius: 6px;
          animation: fade-in 0.2s ease;
        }

        .max-warning::after {
          content: '';
          position: absolute;
          top: 100%;
          left: 50%;
          transform: translateX(-50%);
          border: 6px solid transparent;
          border-top-color: #374151;
        }

        @keyframes fade-in {
          from {
            opacity: 0;
            transform: translateX(-50%) translateY(4px);
          }
          to {
            opacity: 1;
            transform: translateX(-50%) translateY(0);
          }
        }
      `}</style>
    </div>
  );
}
