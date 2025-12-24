/**
 * Footer Component
 *
 * Site footer for Reality Portal (Epic 44).
 */

import Link from 'next/link';

export function Footer() {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="footer">
      <div className="footer-inner">
        <div className="footer-grid">
          {/* Brand */}
          <div className="footer-section">
            <Link href="/" className="footer-logo">
              Reality Portal
            </Link>
            <p className="footer-description">
              Find your perfect property across Slovakia, Czech Republic, and beyond.
            </p>
          </div>

          {/* Quick Links */}
          <div className="footer-section">
            <h3 className="footer-title">Quick Links</h3>
            <nav className="footer-nav">
              <Link href="/listings?transactionType=sale">Properties for Sale</Link>
              <Link href="/listings?transactionType=rent">Properties for Rent</Link>
              <Link href="/listings">All Listings</Link>
            </nav>
          </div>

          {/* Property Types */}
          <div className="footer-section">
            <h3 className="footer-title">Property Types</h3>
            <nav className="footer-nav">
              <Link href="/listings?propertyType=apartment">Apartments</Link>
              <Link href="/listings?propertyType=house">Houses</Link>
              <Link href="/listings?propertyType=land">Land</Link>
              <Link href="/listings?propertyType=commercial">Commercial</Link>
            </nav>
          </div>

          {/* Company */}
          <div className="footer-section">
            <h3 className="footer-title">Company</h3>
            <nav className="footer-nav">
              <Link href="/about">About Us</Link>
              <Link href="/contact">Contact</Link>
              <Link href="/privacy">Privacy Policy</Link>
              <Link href="/terms">Terms of Service</Link>
            </nav>
          </div>
        </div>

        {/* Bottom */}
        <div className="footer-bottom">
          <p>&copy; {currentYear} Reality Portal. All rights reserved.</p>
        </div>
      </div>

      <style jsx>{`
        .footer {
          background-color: #1f2937;
          color: #e5e7eb;
          margin-top: auto;
        }

        .footer-inner {
          max-width: 1280px;
          margin: 0 auto;
          padding: 48px 16px 24px;
        }

        .footer-grid {
          display: grid;
          grid-template-columns: 1fr;
          gap: 32px;
        }

        @media (min-width: 640px) {
          .footer-grid {
            grid-template-columns: repeat(2, 1fr);
          }
        }

        @media (min-width: 1024px) {
          .footer-grid {
            grid-template-columns: 2fr repeat(3, 1fr);
          }
        }

        .footer-section {
          display: flex;
          flex-direction: column;
          gap: 12px;
        }

        .footer-logo {
          font-size: 1.25rem;
          font-weight: bold;
          color: #fff;
          text-decoration: none;
        }

        .footer-description {
          color: #9ca3af;
          font-size: 14px;
          line-height: 1.6;
          margin: 0;
        }

        .footer-title {
          font-size: 14px;
          font-weight: 600;
          color: #fff;
          text-transform: uppercase;
          letter-spacing: 0.05em;
          margin: 0 0 4px;
        }

        .footer-nav {
          display: flex;
          flex-direction: column;
          gap: 8px;
        }

        .footer-nav a {
          color: #9ca3af;
          text-decoration: none;
          font-size: 14px;
          transition: color 0.2s;
        }

        .footer-nav a:hover {
          color: #fff;
        }

        .footer-bottom {
          margin-top: 48px;
          padding-top: 24px;
          border-top: 1px solid #374151;
          text-align: center;
        }

        .footer-bottom p {
          margin: 0;
          color: #6b7280;
          font-size: 14px;
        }
      `}</style>
    </footer>
  );
}
