/**
 * ProtectedRoute Component
 *
 * Wrapper for protected pages that require authentication (Epic 44, Story 44.4).
 */

'use client';

import { useAuth } from '@/lib/auth-context';
import { usePathname } from 'next/navigation';
import type { ReactNode } from 'react';

interface ProtectedRouteProps {
  children: ReactNode;
  fallback?: ReactNode;
}

export function ProtectedRoute({ children, fallback }: ProtectedRouteProps) {
  const { isLoading, isAuthenticated, login } = useAuth();
  const pathname = usePathname();

  if (isLoading) {
    return (
      <div className="loading-container">
        <div className="spinner" />
        <p>Loading...</p>
        <style jsx>{`
          .loading-container {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            min-height: 400px;
            color: #6b7280;
          }
          .spinner {
            width: 40px;
            height: 40px;
            border: 3px solid #e5e7eb;
            border-top-color: #2563eb;
            border-radius: 50%;
            animation: spin 1s linear infinite;
            margin-bottom: 16px;
          }
          @keyframes spin {
            to {
              transform: rotate(360deg);
            }
          }
        `}</style>
      </div>
    );
  }

  if (!isAuthenticated) {
    if (fallback) {
      return <>{fallback}</>;
    }

    return (
      <div className="auth-required">
        <svg
          width="64"
          height="64"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="1"
          aria-hidden="true"
        >
          <rect x="3" y="11" width="18" height="11" rx="2" ry="2" />
          <path d="M7 11V7a5 5 0 0 1 10 0v4" />
        </svg>
        <h2 className="title">Sign in required</h2>
        <p className="text">Please sign in to access this page.</p>
        <button type="button" className="sign-in-button" onClick={() => login(pathname)}>
          Sign In
        </button>
        <style jsx>{`
          .auth-required {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            min-height: 400px;
            text-align: center;
            padding: 32px;
            color: #6b7280;
          }
          .title {
            font-size: 1.5rem;
            font-weight: 600;
            color: #111827;
            margin: 24px 0 8px;
          }
          .text {
            margin: 0 0 24px;
          }
          .sign-in-button {
            padding: 12px 32px;
            background: #2563eb;
            color: #fff;
            border: none;
            border-radius: 8px;
            font-size: 16px;
            font-weight: 600;
            cursor: pointer;
          }
          .sign-in-button:hover {
            background: #1d4ed8;
          }
        `}</style>
      </div>
    );
  }

  return <>{children}</>;
}
