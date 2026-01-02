/**
 * Header Component
 *
 * Main navigation header for Reality Portal (Epic 44).
 * Includes language switcher for i18n support (Epic 111).
 */

'use client';

import { useAuth } from '@/lib/auth-context';
import { useTranslations } from 'next-intl';
import { useState } from 'react';
import { Link } from '../../i18n/routing';
import { LanguageSwitcher } from './LanguageSwitcher';

export function Header() {
  const { user, isLoading, isAuthenticated, login, logout } = useAuth();
  const [showDropdown, setShowDropdown] = useState(false);
  const [showMobileMenu, setShowMobileMenu] = useState(false);
  const t = useTranslations();

  return (
    <header className="header">
      <div className="header-inner">
        {/* Logo */}
        <Link href="/" className="logo">
          Reality Portal
        </Link>

        {/* Desktop Navigation */}
        <nav className="nav-desktop">
          <Link href="/listings?transactionType=sale" className="nav-link">
            {t('search.sale')}
          </Link>
          <Link href="/listings?transactionType=rent" className="nav-link">
            {t('search.rent')}
          </Link>
          <Link href="/listings" className="nav-link">
            {t('nav.allListings')}
          </Link>
        </nav>

        {/* Auth Section */}
        <div className="auth-section">
          <LanguageSwitcher />

          {isLoading ? (
            <div className="skeleton" />
          ) : isAuthenticated ? (
            <div
              className="user-container"
              onMouseEnter={() => setShowDropdown(true)}
              onMouseLeave={() => setShowDropdown(false)}
            >
              <button type="button" className="user-button">
                <div className="avatar">{user?.name.charAt(0).toUpperCase()}</div>
                <span className="user-name">{user?.name}</span>
              </button>

              {showDropdown && (
                <div className="dropdown">
                  <div className="dropdown-header">
                    <p className="dropdown-name">{user?.name}</p>
                    <p className="dropdown-email">{user?.email}</p>
                  </div>
                  <div className="dropdown-menu">
                    <Link href="/favorites" className="menu-item">
                      {t('common.favorites')}
                    </Link>
                    <Link href="/saved-searches" className="menu-item">
                      {t('nav.savedSearches')}
                    </Link>
                    <Link href="/inquiries" className="menu-item">
                      {t('nav.myInquiries')}
                    </Link>
                    <Link href="/profile" className="menu-item">
                      {t('nav.profile')}
                    </Link>
                    <button type="button" onClick={logout} className="sign-out-button">
                      {t('common.logout')}
                    </button>
                  </div>
                </div>
              )}
            </div>
          ) : (
            <button type="button" onClick={() => login()} className="sign-in-button">
              {t('common.login')}
            </button>
          )}

          {/* Mobile Menu Toggle */}
          <button
            type="button"
            className="mobile-menu-toggle"
            onClick={() => setShowMobileMenu(!showMobileMenu)}
            aria-label="Toggle menu"
          >
            <svg
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              aria-hidden="true"
            >
              {showMobileMenu ? (
                <path d="M6 18L18 6M6 6l12 12" />
              ) : (
                <path d="M4 6h16M4 12h16M4 18h16" />
              )}
            </svg>
          </button>
        </div>
      </div>

      {/* Mobile Navigation */}
      {showMobileMenu && (
        <nav className="nav-mobile">
          <Link
            href="/listings?transactionType=sale"
            className="nav-link-mobile"
            onClick={() => setShowMobileMenu(false)}
          >
            {t('search.sale')}
          </Link>
          <Link
            href="/listings?transactionType=rent"
            className="nav-link-mobile"
            onClick={() => setShowMobileMenu(false)}
          >
            {t('search.rent')}
          </Link>
          <Link
            href="/listings"
            className="nav-link-mobile"
            onClick={() => setShowMobileMenu(false)}
          >
            {t('nav.allListings')}
          </Link>
          {isAuthenticated && (
            <>
              <Link
                href="/favorites"
                className="nav-link-mobile"
                onClick={() => setShowMobileMenu(false)}
              >
                {t('common.favorites')}
              </Link>
              <Link
                href="/saved-searches"
                className="nav-link-mobile"
                onClick={() => setShowMobileMenu(false)}
              >
                {t('nav.savedSearches')}
              </Link>
              <Link
                href="/inquiries"
                className="nav-link-mobile"
                onClick={() => setShowMobileMenu(false)}
              >
                {t('nav.myInquiries')}
              </Link>
            </>
          )}
        </nav>
      )}

      <style jsx>{`
        .header {
          background-color: #fff;
          border-bottom: 1px solid #e5e7eb;
          position: sticky;
          top: 0;
          z-index: 50;
        }

        .header-inner {
          max-width: 1280px;
          margin: 0 auto;
          padding: 16px;
          display: flex;
          justify-content: space-between;
          align-items: center;
        }

        .logo {
          font-size: 1.5rem;
          font-weight: bold;
          color: #2563eb;
          text-decoration: none;
        }

        .nav-desktop {
          display: none;
          gap: 32px;
        }

        @media (min-width: 768px) {
          .nav-desktop {
            display: flex;
          }
        }

        .nav-link {
          color: #374151;
          text-decoration: none;
          font-weight: 500;
          transition: color 0.2s;
        }

        .nav-link:hover {
          color: #2563eb;
        }

        .auth-section {
          display: flex;
          align-items: center;
          gap: 16px;
        }

        .skeleton {
          height: 40px;
          width: 96px;
          background-color: #e5e7eb;
          border-radius: 8px;
        }

        .sign-in-button {
          padding: 8px 16px;
          background-color: #2563eb;
          color: #fff;
          border-radius: 8px;
          border: none;
          cursor: pointer;
          font-size: 14px;
          font-weight: 500;
        }

        .sign-in-button:hover {
          background-color: #1d4ed8;
        }

        .user-container {
          position: relative;
        }

        .user-button {
          display: flex;
          align-items: center;
          gap: 8px;
          padding: 8px;
          border-radius: 8px;
          border: none;
          background-color: transparent;
          cursor: pointer;
        }

        .user-button:hover {
          background-color: #f3f4f6;
        }

        .avatar {
          width: 32px;
          height: 32px;
          border-radius: 50%;
          background-color: #2563eb;
          color: #fff;
          display: flex;
          align-items: center;
          justify-content: center;
          font-weight: 600;
          font-size: 14px;
        }

        .user-name {
          font-size: 14px;
          font-weight: 500;
          color: #374151;
          display: none;
        }

        @media (min-width: 768px) {
          .user-name {
            display: block;
          }
        }

        .dropdown {
          position: absolute;
          right: 0;
          top: 100%;
          margin-top: 8px;
          width: 220px;
          background-color: #fff;
          border-radius: 8px;
          box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
          border: 1px solid #e5e7eb;
          z-index: 100;
        }

        .dropdown-header {
          padding: 12px;
          border-bottom: 1px solid #e5e7eb;
        }

        .dropdown-name {
          font-size: 14px;
          font-weight: 500;
          color: #111827;
          margin: 0;
        }

        .dropdown-email {
          font-size: 12px;
          color: #6b7280;
          margin: 4px 0 0;
        }

        .dropdown-menu {
          padding: 4px;
        }

        .menu-item {
          display: block;
          width: 100%;
          padding: 8px 12px;
          font-size: 14px;
          color: #374151;
          text-decoration: none;
          border-radius: 4px;
        }

        .menu-item:hover {
          background-color: #f3f4f6;
        }

        .sign-out-button {
          display: block;
          width: 100%;
          padding: 8px 12px;
          font-size: 14px;
          color: #dc2626;
          text-align: left;
          border: none;
          background-color: transparent;
          cursor: pointer;
          border-radius: 4px;
        }

        .sign-out-button:hover {
          background-color: #fef2f2;
        }

        .mobile-menu-toggle {
          display: flex;
          padding: 8px;
          border: none;
          background: transparent;
          cursor: pointer;
          color: #374151;
        }

        @media (min-width: 768px) {
          .mobile-menu-toggle {
            display: none;
          }
        }

        .nav-mobile {
          display: flex;
          flex-direction: column;
          padding: 8px 16px 16px;
          border-top: 1px solid #e5e7eb;
          background: #fff;
        }

        @media (min-width: 768px) {
          .nav-mobile {
            display: none;
          }
        }

        .nav-link-mobile {
          padding: 12px 0;
          color: #374151;
          text-decoration: none;
          font-weight: 500;
          border-bottom: 1px solid #f3f4f6;
        }

        .nav-link-mobile:last-child {
          border-bottom: none;
        }
      `}</style>
    </header>
  );
}
