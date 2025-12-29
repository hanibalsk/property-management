/**
 * Login Page Component.
 *
 * Provides a login form with email/password authentication.
 * Handles validation, error display, and redirect after successful login.
 *
 * @see Story 79.2 - Authentication Flow Implementation
 */

import type React from 'react';
import { useCallback, useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { AuthError, useAuth } from '../contexts/AuthContext';
import './LoginPage.css';

// ============================================================================
// Constants
// ============================================================================

const RETURN_URL_KEY = 'ppt_return_url';

// ============================================================================
// Types
// ============================================================================

interface FormErrors {
  email?: string;
  password?: string;
  general?: string;
}

// ============================================================================
// Validation
// ============================================================================

/**
 * Validates email format.
 */
function isValidEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

/**
 * Validates the login form and returns any errors.
 */
function validateForm(email: string, password: string): FormErrors {
  const errors: FormErrors = {};

  if (!email.trim()) {
    errors.email = 'Email is required';
  } else if (!isValidEmail(email.trim())) {
    errors.email = 'Please enter a valid email address';
  }

  if (!password) {
    errors.password = 'Password is required';
  }

  return errors;
}

/**
 * Maps authentication error codes to user-friendly messages.
 */
function getErrorMessage(error: unknown): string {
  if (error instanceof AuthError) {
    switch (error.code) {
      case 'INVALID_CREDENTIALS':
        return 'Invalid email or password. Please try again.';
      case 'ACCOUNT_LOCKED':
        return 'Your account has been locked due to too many failed attempts. Please contact support.';
      case 'ACCOUNT_DISABLED':
        return 'Your account has been disabled. Please contact support.';
      case 'SESSION_EXPIRED':
        return 'Your session has expired. Please log in again.';
      case 'NETWORK_ERROR':
        return 'Unable to connect to the server. Please check your internet connection.';
      default:
        return 'An unexpected error occurred. Please try again.';
    }
  }

  if (error instanceof Error) {
    // Check for network errors
    if (error.message.includes('fetch') || error.message.includes('network')) {
      return 'Unable to connect to the server. Please check your internet connection.';
    }
    return error.message;
  }

  return 'An unexpected error occurred. Please try again.';
}

// ============================================================================
// Component
// ============================================================================

/**
 * Login Page Component.
 *
 * Displays a login form and handles authentication.
 * After successful login, redirects to the dashboard or stored return URL.
 */
export function LoginPage() {
  const navigate = useNavigate();
  const { login, isAuthenticated, isLoading: authLoading } = useAuth();

  // Form state
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [errors, setErrors] = useState<FormErrors>({});
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [showPassword, setShowPassword] = useState(false);

  /**
   * Gets the stored return URL and clears it from storage.
   */
  const getAndClearReturnUrl = useCallback((): string | null => {
    try {
      const returnUrl = sessionStorage.getItem(RETURN_URL_KEY);
      if (returnUrl) {
        sessionStorage.removeItem(RETURN_URL_KEY);
      }
      return returnUrl;
    } catch {
      return null;
    }
  }, []);

  // Redirect if already authenticated
  useEffect(() => {
    if (isAuthenticated && !authLoading) {
      const returnUrl = getAndClearReturnUrl();
      navigate(returnUrl || '/', { replace: true });
    }
  }, [isAuthenticated, authLoading, navigate, getAndClearReturnUrl]);

  /**
   * Handles email input change.
   */
  const handleEmailChange = useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      const value = event.target.value;
      setEmail(value);

      // Clear email error when user starts typing
      if (errors.email) {
        setErrors((prev) => ({ ...prev, email: undefined }));
      }
    },
    [errors.email]
  );

  /**
   * Handles password input change.
   */
  const handlePasswordChange = useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      const value = event.target.value;
      setPassword(value);

      // Clear password error when user starts typing
      if (errors.password) {
        setErrors((prev) => ({ ...prev, password: undefined }));
      }
    },
    [errors.password]
  );

  /**
   * Toggles password visibility.
   */
  const handleTogglePassword = useCallback(() => {
    setShowPassword((prev) => !prev);
  }, []);

  /**
   * Handles form submission.
   */
  const handleSubmit = useCallback(
    async (event: React.FormEvent<HTMLFormElement>) => {
      event.preventDefault();

      // Clear previous errors
      setErrors({});

      // Validate form
      const validationErrors = validateForm(email, password);
      if (Object.keys(validationErrors).length > 0) {
        setErrors(validationErrors);
        return;
      }

      setIsSubmitting(true);

      try {
        await login({ email: email.trim(), password });

        // Redirect to return URL or dashboard
        const returnUrl = getAndClearReturnUrl();
        navigate(returnUrl || '/', { replace: true });
      } catch (error) {
        setErrors({ general: getErrorMessage(error) });
      } finally {
        setIsSubmitting(false);
      }
    },
    [email, password, login, navigate, getAndClearReturnUrl]
  );

  // Show loading state while checking auth
  if (authLoading) {
    return (
      <div className="login-page">
        <div className="login-loading">
          <div className="login-spinner" aria-label="Checking authentication status" />
        </div>
      </div>
    );
  }

  // Don't render form if already authenticated (will redirect)
  if (isAuthenticated) {
    return null;
  }

  const isFormDisabled = isSubmitting;

  return (
    <div className="login-page">
      <div className="login-container">
        <div className="login-header">
          <h1 className="login-title">Sign in</h1>
          <p className="login-subtitle">Welcome back! Please enter your credentials to continue.</p>
        </div>

        <form className="login-form" onSubmit={handleSubmit} noValidate>
          {/* General error message */}
          {errors.general && (
            <div className="login-error-banner" role="alert" aria-live="polite">
              <span className="login-error-icon" aria-hidden="true">
                !
              </span>
              <span>{errors.general}</span>
            </div>
          )}

          {/* Email field */}
          <div className="login-field">
            <label htmlFor="email" className="login-label">
              Email address
            </label>
            <input
              id="email"
              name="email"
              type="email"
              autoComplete="email"
              value={email}
              onChange={handleEmailChange}
              disabled={isFormDisabled}
              className={`login-input ${errors.email ? 'login-input--error' : ''}`}
              aria-invalid={errors.email ? 'true' : 'false'}
              aria-describedby={errors.email ? 'email-error' : undefined}
            />
            {errors.email && (
              <span id="email-error" className="login-field-error" role="alert">
                {errors.email}
              </span>
            )}
          </div>

          {/* Password field */}
          <div className="login-field">
            <label htmlFor="password" className="login-label">
              Password
            </label>
            <div className="login-password-wrapper">
              <input
                id="password"
                name="password"
                type={showPassword ? 'text' : 'password'}
                autoComplete="current-password"
                value={password}
                onChange={handlePasswordChange}
                disabled={isFormDisabled}
                className={`login-input ${errors.password ? 'login-input--error' : ''}`}
                aria-invalid={errors.password ? 'true' : 'false'}
                aria-describedby={errors.password ? 'password-error' : undefined}
              />
              <button
                type="button"
                className="login-password-toggle"
                onClick={handleTogglePassword}
                disabled={isFormDisabled}
                aria-label={showPassword ? 'Hide password' : 'Show password'}
              >
                {showPassword ? 'Hide' : 'Show'}
              </button>
            </div>
            {errors.password && (
              <span id="password-error" className="login-field-error" role="alert">
                {errors.password}
              </span>
            )}
          </div>

          {/* Submit button */}
          <button type="submit" className="login-submit" disabled={isFormDisabled}>
            {isSubmitting ? (
              <>
                <span className="login-spinner login-spinner--small" aria-hidden="true" />
                <span>Signing in...</span>
              </>
            ) : (
              'Sign in'
            )}
          </button>
        </form>
      </div>
    </div>
  );
}

LoginPage.displayName = 'LoginPage';
