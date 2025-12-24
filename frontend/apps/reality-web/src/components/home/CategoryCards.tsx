/**
 * CategoryCards Component
 *
 * Property type category cards for homepage (Epic 44, Story 44.1).
 */

'use client';

import { useCategoryCounts } from '@ppt/reality-api-client';
import Link from 'next/link';

const categoryIcons: Record<string, JSX.Element> = {
  apartment: (
    <svg
      width="32"
      height="32"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.5"
      aria-hidden="true"
    >
      <rect x="4" y="2" width="16" height="20" rx="2" ry="2" />
      <path d="M9 22v-4h6v4" />
      <path d="M8 6h.01M16 6h.01M8 10h.01M16 10h.01M8 14h.01M16 14h.01" />
    </svg>
  ),
  house: (
    <svg
      width="32"
      height="32"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.5"
      aria-hidden="true"
    >
      <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
      <polyline points="9 22 9 12 15 12 15 22" />
    </svg>
  ),
  land: (
    <svg
      width="32"
      height="32"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.5"
      aria-hidden="true"
    >
      <path d="M2 20h20" />
      <path d="M5 20v-4l4-4 4 4 4-8 4 8v4" />
    </svg>
  ),
  commercial: (
    <svg
      width="32"
      height="32"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.5"
      aria-hidden="true"
    >
      <path d="M2 22h20" />
      <rect x="6" y="8" width="12" height="14" rx="1" />
      <rect x="2" y="12" width="4" height="10" rx="1" />
      <rect x="18" y="12" width="4" height="10" rx="1" />
      <path d="M10 8V4a2 2 0 0 1 2-2h0a2 2 0 0 1 2 2v4" />
    </svg>
  ),
  office: (
    <svg
      width="32"
      height="32"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.5"
      aria-hidden="true"
    >
      <rect x="2" y="7" width="20" height="14" rx="2" ry="2" />
      <path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16" />
    </svg>
  ),
  garage: (
    <svg
      width="32"
      height="32"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.5"
      aria-hidden="true"
    >
      <path d="M5 22V9l7-7 7 7v13H5z" />
      <rect x="8" y="13" width="8" height="9" />
    </svg>
  ),
};

const defaultIcon = (
  <svg
    width="32"
    height="32"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="1.5"
    aria-hidden="true"
  >
    <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
  </svg>
);

export function CategoryCards() {
  const { data: categories, isLoading, error } = useCategoryCounts();

  if (isLoading) {
    return (
      <section className="categories-section">
        <div className="container">
          <div className="skeleton-header" />
          <div className="grid">
            {[1, 2, 3, 4].map((i) => (
              <div key={`cat-skeleton-${i}`} className="skeleton-card" />
            ))}
          </div>
        </div>
        <style jsx>{`
          .categories-section {
            padding: 64px 16px;
            background: #fff;
          }
          .container {
            max-width: 1280px;
            margin: 0 auto;
          }
          .skeleton-header {
            height: 40px;
            width: 250px;
            background: #e5e7eb;
            border-radius: 8px;
            margin: 0 auto 32px;
          }
          .grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
            gap: 16px;
          }
          .skeleton-card {
            height: 120px;
            background: #e5e7eb;
            border-radius: 12px;
          }
        `}</style>
      </section>
    );
  }

  if (error || !categories?.length) {
    return null;
  }

  return (
    <section className="categories-section">
      <div className="container">
        <h2 className="section-title">Browse by Property Type</h2>
        <div className="grid">
          {categories.map((category) => (
            <Link
              key={category.type}
              href={`/listings?propertyType=${category.type}`}
              className="category-card"
            >
              <div className="icon">{categoryIcons[category.type] || defaultIcon}</div>
              <h3 className="label">{category.label}</h3>
              <p className="count">{category.count.toLocaleString()} listings</p>
            </Link>
          ))}
        </div>
      </div>

      <style jsx>{`
        .categories-section {
          padding: 64px 16px;
          background: #fff;
        }

        .container {
          max-width: 1280px;
          margin: 0 auto;
        }

        .section-title {
          font-size: 1.5rem;
          font-weight: bold;
          color: #111827;
          text-align: center;
          margin: 0 0 32px;
        }

        .grid {
          display: grid;
          grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
          gap: 16px;
        }

        .category-card {
          display: flex;
          flex-direction: column;
          align-items: center;
          padding: 24px;
          background: #f9fafb;
          border-radius: 12px;
          text-decoration: none;
          transition: all 0.2s;
          text-align: center;
        }

        .category-card:hover {
          background: #f3f4f6;
          transform: translateY(-2px);
        }

        .icon {
          color: #2563eb;
          margin-bottom: 12px;
        }

        .label {
          font-size: 1rem;
          font-weight: 600;
          color: #111827;
          margin: 0 0 4px;
        }

        .count {
          font-size: 14px;
          color: #6b7280;
          margin: 0;
        }
      `}</style>
    </section>
  );
}
