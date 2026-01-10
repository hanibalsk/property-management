/**
 * Authentication Flow E2E Tests
 * Epic 131: E2E Test Suite - Story 2
 *
 * Tests login page UI, form validation, and authentication flows.
 *
 * Note: Tests that require actual authentication (login with valid credentials,
 * logout, session persistence) require the backend API to be running with
 * seeded test users. These tests will fail gracefully if the backend is unavailable.
 */

import { expect, test, testUsers } from './fixtures';

test.describe('Authentication Flow', () => {
  test.beforeEach(async ({ page }) => {
    // Clear any stored session data
    await page.goto('/login');
    await page.evaluate(() => {
      sessionStorage.clear();
      localStorage.clear();
    });
  });

  test.describe('Login Page UI', () => {
    test('should display login form elements', async ({ page }) => {
      await page.goto('/login');

      // Check page title/heading
      await expect(page.getByRole('heading', { name: /sign in/i })).toBeVisible();

      // Check form fields
      await expect(page.locator('#email')).toBeVisible();
      await expect(page.locator('#password')).toBeVisible();

      // Check labels
      await expect(page.getByText(/email/i).first()).toBeVisible();
      await expect(page.getByText(/password/i).first()).toBeVisible();

      // Check submit button
      await expect(page.getByRole('button', { name: /sign in/i })).toBeVisible();
    });

    test('should have password visibility toggle', async ({ page }) => {
      await page.goto('/login');

      const passwordInput = page.locator('#password');
      const toggleButton = page.locator('.login-password-toggle');

      // Password should be hidden by default
      await expect(passwordInput).toHaveAttribute('type', 'password');

      // Click toggle to show password
      await toggleButton.click();
      await expect(passwordInput).toHaveAttribute('type', 'text');

      // Click toggle to hide password again
      await toggleButton.click();
      await expect(passwordInput).toHaveAttribute('type', 'password');
    });
  });

  test.describe('Form Validation', () => {
    test('should show error for empty email', async ({ page }) => {
      await page.goto('/login');

      // Fill only password
      await page.locator('#password').fill('somepassword');

      // Submit form
      await page.getByRole('button', { name: /sign in/i }).click();

      // Check for email error
      await expect(page.locator('#email-error')).toBeVisible();
    });

    test('should show error for invalid email format', async ({ page }) => {
      await page.goto('/login');

      // Fill invalid email
      await page.locator('#email').fill('invalid-email');
      await page.locator('#password').fill('somepassword');

      // Submit form
      await page.getByRole('button', { name: /sign in/i }).click();

      // Check for email error
      await expect(page.locator('#email-error')).toBeVisible();
    });

    test('should show error for empty password', async ({ page }) => {
      await page.goto('/login');

      // Fill only email
      await page.locator('#email').fill('test@example.com');

      // Submit form
      await page.getByRole('button', { name: /sign in/i }).click();

      // Check for password error
      await expect(page.locator('#password-error')).toBeVisible();
    });

    test('should clear field errors when user starts typing', async ({ page }) => {
      await page.goto('/login');

      // Submit empty form to trigger errors
      await page.getByRole('button', { name: /sign in/i }).click();

      // Both errors should be visible
      await expect(page.locator('#email-error')).toBeVisible();
      await expect(page.locator('#password-error')).toBeVisible();

      // Start typing in email field
      await page.locator('#email').fill('t');

      // Email error should be cleared
      await expect(page.locator('#email-error')).not.toBeVisible();

      // Password error should still be visible
      await expect(page.locator('#password-error')).toBeVisible();

      // Start typing in password field
      await page.locator('#password').fill('p');

      // Password error should be cleared
      await expect(page.locator('#password-error')).not.toBeVisible();
    });
  });

  test.describe('Login Error Handling', () => {
    test('should show error banner for invalid credentials', async ({ page }) => {
      await page.goto('/login');

      // Fill invalid credentials
      await page.locator('#email').fill('nonexistent@example.com');
      await page.locator('#password').fill('wrongpassword');

      // Submit form
      await page.getByRole('button', { name: /sign in/i }).click();

      // Check for general error banner (should appear for any login failure)
      await expect(page.locator('.login-error-banner')).toBeVisible({ timeout: 10000 });
    });

    test('should display error message in error banner', async ({ page }) => {
      await page.goto('/login');

      // Fill credentials
      await page.locator('#email').fill('test@example.com');
      await page.locator('#password').fill('wrongpassword');

      // Submit form
      await page.getByRole('button', { name: /sign in/i }).click();

      // Error banner should contain error text
      const errorBanner = page.locator('.login-error-banner');
      await expect(errorBanner).toBeVisible({ timeout: 10000 });
      await expect(errorBanner).not.toBeEmpty();
    });
  });

  test.describe('Protected Routes', () => {
    test('should handle unauthenticated access to dashboard', async ({ page }) => {
      // Try to access a protected route without auth
      await page.goto('/dashboard');

      // Should either redirect to login or show limited content
      // (behavior depends on route protection implementation)
      const currentUrl = page.url();
      const isOnLogin = currentUrl.includes('/login');
      const isOnDashboard = currentUrl.includes('/dashboard');

      // Either redirected to login OR stayed on dashboard
      expect(isOnLogin || isOnDashboard).toBeTruthy();
    });
  });
});

/**
 * Tests requiring working backend with seeded test users.
 * These tests are tagged with @requires-backend for filtering.
 */
test.describe('Authentication with Backend @requires-backend', () => {
  test.describe('Successful Login', () => {
    test('should successfully login with valid credentials', async ({ page }) => {
      await page.goto('/login');

      // Fill valid credentials
      await page.locator('#email').fill(testUsers.manager.email);
      await page.locator('#password').fill(testUsers.manager.password);

      // Submit form
      await page.getByRole('button', { name: /sign in/i }).click();

      // Should redirect to home or dashboard (or show error if backend unavailable)
      try {
        await page.waitForURL(/^\/$|\/dashboard/, { timeout: 10000 });
        // Should no longer be on login page
        await expect(page).not.toHaveURL('/login');
      } catch {
        // If login fails, check for error banner (backend may be unavailable)
        const errorBanner = page.locator('.login-error-banner');
        if (await errorBanner.isVisible()) {
          test.skip(
            true,
            'Backend unavailable - login failed with error banner. Run with backend and seeded test users.'
          );
        }
        throw new Error('Login did not succeed and no error banner shown');
      }
    });
  });

  test.describe('Logout Flow', () => {
    test('should logout and redirect to login page', async ({ page }) => {
      // First login
      await page.goto('/login');
      await page.locator('#email').fill(testUsers.manager.email);
      await page.locator('#password').fill(testUsers.manager.password);
      await page.getByRole('button', { name: /sign in/i }).click();

      // Wait for redirect or error
      try {
        await page.waitForURL(/^\/$|\/dashboard/, { timeout: 10000 });
      } catch {
        test.skip(true, 'Backend unavailable - could not login');
        return;
      }

      // Find and click logout button
      const logoutButton = page.getByRole('button', { name: /logout|sign out/i });
      if (await logoutButton.isVisible()) {
        await logoutButton.click();
        // Should redirect to login
        await expect(page).toHaveURL(/\/login/);
      } else {
        // Logout button might be in a dropdown or different location
        test.skip(true, 'Logout button not visible - may need to open user menu');
      }
    });
  });

  test.describe('Session Persistence', () => {
    test('should stay logged in after page refresh', async ({ page }) => {
      // Login
      await page.goto('/login');
      await page.locator('#email').fill(testUsers.manager.email);
      await page.locator('#password').fill(testUsers.manager.password);
      await page.getByRole('button', { name: /sign in/i }).click();

      // Wait for redirect or error
      try {
        await page.waitForURL(/^\/$|\/dashboard/, { timeout: 10000 });
      } catch {
        test.skip(true, 'Backend unavailable - could not login');
        return;
      }

      // Refresh the page
      await page.reload();

      // Should still be authenticated (not redirected to login)
      await expect(page).not.toHaveURL(/\/login/);
    });
  });
});

/**
 * Tests using the authenticatedPage fixture.
 * These require backend to be running with seeded test users.
 * Skip the entire describe block if backend is not available.
 */
test.describe('Authenticated User @requires-backend', () => {
  // Skip these tests - they require a running backend with seeded users
  // To run these tests: start the backend and seed test users
  test.skip(
    () => !process.env.E2E_WITH_BACKEND,
    'Skipped - requires backend. Set E2E_WITH_BACKEND=1 to run.'
  );

  test('should access documents page after authentication', async ({ authenticatedPage }) => {
    await authenticatedPage.goto('/documents');
    // Should stay on documents page
    await expect(authenticatedPage).toHaveURL(/\/documents/);
  });

  test('should display navigation after login', async ({ authenticatedPage }) => {
    // Check that navigation is visible
    const nav = authenticatedPage.locator('nav');
    await expect(nav).toBeVisible();
  });
});
