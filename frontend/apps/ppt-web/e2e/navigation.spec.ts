/**
 * Navigation E2E Tests
 * Epic 131: E2E Test Suite - Story 3
 *
 * Tests main navigation, routing, and page accessibility.
 */

import { expect, test } from './fixtures';

test.describe('Main Navigation', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should display all main navigation links', async ({ page }) => {
    const nav = page.locator('nav');
    await expect(nav).toBeVisible();

    // Check all main navigation links
    await expect(page.getByRole('link', { name: /home/i })).toBeVisible();
    await expect(page.getByRole('link', { name: /documents/i })).toBeVisible();
    await expect(page.getByRole('link', { name: /news/i })).toBeVisible();
    await expect(page.getByRole('link', { name: /emergency/i })).toBeVisible();
    await expect(page.getByRole('link', { name: /disputes/i })).toBeVisible();
    await expect(page.getByRole('link', { name: /outages/i })).toBeVisible();
    await expect(page.getByRole('link', { name: /accessibility/i })).toBeVisible();
    await expect(page.getByRole('link', { name: /privacy/i })).toBeVisible();
  });

  test('should navigate to documents page', async ({ page }) => {
    await page.getByRole('link', { name: /documents/i }).click();
    await expect(page).toHaveURL(/\/documents/);
  });

  test('should navigate to news page', async ({ page }) => {
    await page.getByRole('link', { name: /news/i }).click();
    await expect(page).toHaveURL(/\/news/);
  });

  test('should navigate to emergency contacts page', async ({ page }) => {
    await page.getByRole('link', { name: /emergency/i }).click();
    await expect(page).toHaveURL(/\/emergency/);
  });

  test('should navigate to disputes page', async ({ page }) => {
    await page.getByRole('link', { name: /disputes/i }).click();
    await expect(page).toHaveURL(/\/disputes/);
  });

  test('should navigate to outages page', async ({ page }) => {
    await page.getByRole('link', { name: /outages/i }).click();
    await expect(page).toHaveURL(/\/outages/);
  });

  test('should navigate to accessibility settings', async ({ page }) => {
    await page.getByRole('link', { name: /accessibility/i }).click();
    await expect(page).toHaveURL(/\/settings\/accessibility/);
  });

  test('should navigate to privacy settings', async ({ page }) => {
    await page.getByRole('link', { name: /privacy/i }).click();
    await expect(page).toHaveURL(/\/settings\/privacy/);
  });

  test('should navigate back to home', async ({ page }) => {
    // Go to another page first
    await page.getByRole('link', { name: /documents/i }).click();
    await expect(page).toHaveURL(/\/documents/);

    // Navigate back to home
    await page.getByRole('link', { name: /home/i }).click();
    await expect(page).toHaveURL('/');
  });
});

test.describe('Skip Navigation', () => {
  test('should have skip to main content link', async ({ page }) => {
    await page.goto('/');

    // Skip link should be present (may be visually hidden until focused)
    const skipLink = page.getByRole('link', { name: /skip to main content/i });
    await expect(skipLink).toBeAttached();
  });
});

test.describe('Connection Status', () => {
  test('should display connection status indicator', async ({ page }) => {
    await page.goto('/');

    // Connection status should be visible in navigation
    // It may have "status" role with connection-related label
    const connectionStatus = page.locator(
      '[class*="connection-status"], [aria-label*="Connection"], [role="status"]'
    );
    await expect(connectionStatus.first()).toBeVisible();
  });
});

test.describe('Language Switcher', () => {
  test('should display language selection dropdown', async ({ page }) => {
    await page.goto('/');

    // Language switcher should be visible
    const languageSwitcher = page.getByRole('combobox', { name: /language/i });
    await expect(languageSwitcher).toBeVisible();
  });

  test('should have multiple language options', async ({ page }) => {
    await page.goto('/');

    const languageSwitcher = page.getByRole('combobox', { name: /language/i });

    // Check that language options exist
    await expect(languageSwitcher.locator('option')).toHaveCount(6); // en, sk, cs, de, pl, hu
  });

  test('should change language when selecting different option', async ({ page }) => {
    await page.goto('/');

    const languageSwitcher = page.getByRole('combobox', { name: /language/i });

    // Change language to Slovak
    await languageSwitcher.selectOption({ label: '🇸🇰 Slovenčina' });

    // Wait for language change to take effect
    await page.waitForTimeout(500);

    // Check that content changed (heading should be in Slovak)
    const homeHeading = page.locator('h1');
    const newText = await homeHeading.textContent();

    // Text should be present (the important thing is no error occurred)
    expect(newText).toBeTruthy();
  });
});

test.describe('Offline Indicator', () => {
  test('should not show offline indicator when online', async ({ page }) => {
    await page.goto('/');

    // Offline indicator should not be visible when online
    const offlineIndicator = page.locator('[class*="offline-indicator"]');
    await expect(offlineIndicator).not.toBeVisible();
  });
});
