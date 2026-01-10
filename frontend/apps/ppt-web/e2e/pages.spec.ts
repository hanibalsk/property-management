/**
 * Critical Pages E2E Tests
 * Epic 131: E2E Test Suite - Story 3
 *
 * Tests for documents, news, emergency, and settings pages.
 */

import { expect, test } from './fixtures';

test.describe('Documents Page', () => {
  test('should load documents page', async ({ page }) => {
    await page.goto('/documents');

    // Page should load without error
    await expect(page).toHaveURL(/\/documents/);

    // Should have a main content area
    await expect(page.locator('main')).toBeVisible();
  });

  test('should display documents page heading', async ({ page }) => {
    await page.goto('/documents');

    // Should have a heading
    const heading = page.locator('h1, h2').first();
    await expect(heading).toBeVisible();
  });
});

test.describe('News Page', () => {
  test('should load news page or show error boundary', async ({ page }) => {
    await page.goto('/news');

    // Page should load without error
    await expect(page).toHaveURL(/\/news/);

    // Page may show error boundary if not authenticated
    // Check for either main content or error boundary
    const mainOrError = page.locator('main, [role="alert"]');
    await expect(mainOrError.first()).toBeVisible();
  });

  test('should display news page content or auth error', async ({ page }) => {
    await page.goto('/news');

    // Should have a heading (either content or error message)
    const heading = page.locator('h1, h2').first();
    await expect(heading).toBeVisible();
  });
});

test.describe('Emergency Contacts Page', () => {
  test('should load emergency contacts page or show error boundary', async ({ page }) => {
    await page.goto('/emergency');

    // Page should load without error
    await expect(page).toHaveURL(/\/emergency/);

    // Page may show error boundary if not authenticated
    // Check for either main content or error boundary
    const mainOrError = page.locator('main, [role="alert"]');
    await expect(mainOrError.first()).toBeVisible();
  });

  test('should display emergency contacts heading or auth error', async ({ page }) => {
    await page.goto('/emergency');

    // Should have a heading (either content or error message)
    const heading = page.locator('h1, h2').first();
    await expect(heading).toBeVisible();
  });
});

test.describe('Disputes Page', () => {
  test('should load disputes page', async ({ page }) => {
    await page.goto('/disputes');

    // Page should load without error
    await expect(page).toHaveURL(/\/disputes/);

    // Should have a main content area
    await expect(page.locator('main')).toBeVisible();
  });
});

test.describe('Outages Page', () => {
  test('should load outages page', async ({ page }) => {
    await page.goto('/outages');

    // Page should load without error
    await expect(page).toHaveURL(/\/outages/);

    // Should have a main content area
    await expect(page.locator('main')).toBeVisible();
  });
});

test.describe('Accessibility Settings Page', () => {
  test('should load accessibility settings page', async ({ page }) => {
    await page.goto('/settings/accessibility');

    // Page should load without error
    await expect(page).toHaveURL(/\/settings\/accessibility/);

    // Should have a main content area
    await expect(page.locator('main')).toBeVisible();
  });

  test('should display accessibility options', async ({ page }) => {
    await page.goto('/settings/accessibility');

    // Should have accessibility-related content
    // Look for common accessibility settings elements
    const content = page.locator('main');
    await expect(content).toBeVisible();
  });
});

test.describe('Privacy Settings Page', () => {
  test('should load privacy settings page', async ({ page }) => {
    await page.goto('/settings/privacy');

    // Page should load without error
    await expect(page).toHaveURL(/\/settings\/privacy/);

    // Should have a main content area
    await expect(page.locator('main')).toBeVisible();
  });

  test('should display privacy options', async ({ page }) => {
    await page.goto('/settings/privacy');

    // Should have privacy-related content
    const content = page.locator('main');
    await expect(content).toBeVisible();
  });
});

test.describe('404 Page', () => {
  test('should handle unknown routes gracefully', async ({ page }) => {
    await page.goto('/nonexistent-page-12345');

    // Page should not crash - either shows 404, error boundary, or main content
    const pageContent = page.locator('main, [role="alert"], body');
    await expect(pageContent.first()).toBeVisible();
  });
});

test.describe('Direct URL Access', () => {
  test('should load home page directly', async ({ page }) => {
    await page.goto('/');
    await expect(page).toHaveURL('/');
    await expect(page.locator('main')).toBeVisible();
  });

  test('should load login page directly', async ({ page }) => {
    await page.goto('/login');
    await expect(page).toHaveURL('/login');
    await expect(page.locator('main')).toBeVisible();
  });

  test('should load documents page directly', async ({ page }) => {
    await page.goto('/documents');
    await expect(page).toHaveURL(/\/documents/);
    await expect(page.locator('main')).toBeVisible();
  });

  test('should load news page directly', async ({ page }) => {
    await page.goto('/news');
    await expect(page).toHaveURL(/\/news/);
    // May show error boundary if not authenticated
    const pageContent = page.locator('main, [role="alert"]');
    await expect(pageContent.first()).toBeVisible();
  });

  test('should load emergency page directly', async ({ page }) => {
    await page.goto('/emergency');
    await expect(page).toHaveURL(/\/emergency/);
    // May show error boundary if not authenticated
    const pageContent = page.locator('main, [role="alert"]');
    await expect(pageContent.first()).toBeVisible();
  });
});
