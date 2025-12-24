/**
 * HeroSearch Component
 *
 * Hero section with search form for homepage (Epic 44, Story 44.1).
 */

'use client';

import { useRouter } from 'next/navigation';
import { useState } from 'react';

export function HeroSearch() {
  const router = useRouter();
  const [query, setQuery] = useState('');
  const [transactionType, setTransactionType] = useState<'sale' | 'rent'>('sale');

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    const params = new URLSearchParams();
    params.set('transactionType', transactionType);
    if (query.trim()) {
      params.set('q', query.trim());
    }
    router.push(`/listings?${params.toString()}`);
  };

  return (
    <section className="hero">
      <div className="hero-content">
        <h1 className="hero-title">Find Your Perfect Property</h1>
        <p className="hero-subtitle">
          Search thousands of listings across Slovakia, Czech Republic, and beyond.
        </p>

        <form className="search-form" onSubmit={handleSearch}>
          {/* Transaction Type Toggle */}
          <div className="toggle-container">
            <button
              type="button"
              className={`toggle-button ${transactionType === 'sale' ? 'active' : ''}`}
              onClick={() => setTransactionType('sale')}
            >
              Buy
            </button>
            <button
              type="button"
              className={`toggle-button ${transactionType === 'rent' ? 'active' : ''}`}
              onClick={() => setTransactionType('rent')}
            >
              Rent
            </button>
          </div>

          {/* Search Input */}
          <div className="search-input-container">
            <svg
              className="search-icon"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              aria-hidden="true"
            >
              <circle cx="11" cy="11" r="8" />
              <path d="m21 21-4.35-4.35" />
            </svg>
            <input
              type="text"
              className="search-input"
              placeholder="Enter city, address, or keyword..."
              value={query}
              onChange={(e) => setQuery(e.target.value)}
            />
            <button type="submit" className="search-button">
              Search
            </button>
          </div>
        </form>

        {/* Quick Links */}
        <div className="quick-links">
          <span className="quick-links-label">Popular:</span>
          <button
            type="button"
            className="quick-link"
            onClick={() => router.push('/listings?city=Bratislava&transactionType=sale')}
          >
            Bratislava
          </button>
          <button
            type="button"
            className="quick-link"
            onClick={() => router.push('/listings?city=Prague&transactionType=sale')}
          >
            Prague
          </button>
          <button
            type="button"
            className="quick-link"
            onClick={() => router.push('/listings?city=Vienna&transactionType=sale')}
          >
            Vienna
          </button>
        </div>
      </div>

      <style jsx>{`
        .hero {
          background: linear-gradient(135deg, #1e40af 0%, #3b82f6 100%);
          padding: 80px 16px;
          text-align: center;
        }

        .hero-content {
          max-width: 800px;
          margin: 0 auto;
        }

        .hero-title {
          font-size: 2.5rem;
          font-weight: bold;
          color: #fff;
          margin: 0 0 16px;
        }

        @media (min-width: 768px) {
          .hero-title {
            font-size: 3rem;
          }
        }

        .hero-subtitle {
          font-size: 1.125rem;
          color: #bfdbfe;
          margin: 0 0 32px;
        }

        .search-form {
          background: #fff;
          border-radius: 16px;
          padding: 24px;
          box-shadow: 0 10px 40px rgba(0, 0, 0, 0.15);
        }

        .toggle-container {
          display: inline-flex;
          background: #f3f4f6;
          border-radius: 8px;
          padding: 4px;
          margin-bottom: 16px;
        }

        .toggle-button {
          padding: 8px 24px;
          border: none;
          background: transparent;
          border-radius: 6px;
          font-size: 14px;
          font-weight: 500;
          color: #6b7280;
          cursor: pointer;
          transition: all 0.2s;
        }

        .toggle-button.active {
          background: #fff;
          color: #2563eb;
          box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
        }

        .search-input-container {
          display: flex;
          align-items: center;
          gap: 12px;
          background: #f9fafb;
          border: 1px solid #e5e7eb;
          border-radius: 12px;
          padding: 4px 4px 4px 16px;
        }

        .search-icon {
          color: #9ca3af;
          flex-shrink: 0;
        }

        .search-input {
          flex: 1;
          border: none;
          background: transparent;
          font-size: 16px;
          padding: 12px 0;
          outline: none;
        }

        .search-input::placeholder {
          color: #9ca3af;
        }

        .search-button {
          padding: 12px 24px;
          background: #2563eb;
          color: #fff;
          border: none;
          border-radius: 8px;
          font-size: 14px;
          font-weight: 600;
          cursor: pointer;
          transition: background 0.2s;
        }

        .search-button:hover {
          background: #1d4ed8;
        }

        .quick-links {
          display: flex;
          align-items: center;
          justify-content: center;
          gap: 12px;
          margin-top: 24px;
          flex-wrap: wrap;
        }

        .quick-links-label {
          color: #bfdbfe;
          font-size: 14px;
        }

        .quick-link {
          padding: 6px 12px;
          background: rgba(255, 255, 255, 0.1);
          border: 1px solid rgba(255, 255, 255, 0.2);
          border-radius: 20px;
          color: #fff;
          font-size: 14px;
          cursor: pointer;
          transition: all 0.2s;
        }

        .quick-link:hover {
          background: rgba(255, 255, 255, 0.2);
        }
      `}</style>
    </section>
  );
}
