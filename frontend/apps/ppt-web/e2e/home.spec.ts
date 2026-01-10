/**
 * Home Page E2E Tests
 * Epic 131: E2E Test Suite
 *
 * Basic smoke tests to verify the app loads and navigation works.
 */

import { expect, test } from '@playwright/test';

test.describe('Home Page', () => {
  test('should load the home page', async ({ page }) => {
    await page.goto('/');

    // Verify the page title or main heading
    await expect(page).toHaveTitle(/Property Management|PPT/i);
  });

  test('should display navigation menu', async ({ page }) => {
    await page.goto('/');

    // Check that main navigation links are visible
    const nav = page.locator('nav');
    await expect(nav).toBeVisible();

    // Verify key navigation links exist
    await expect(page.getByRole('link', { name: /home/i })).toBeVisible();
    await expect(page.getByRole('link', { name: /documents/i })).toBeVisible();
  });

  test('should navigate to login page', async ({ page }) => {
    await page.goto('/');

    // Click login link or navigate directly
    await page.goto('/login');

    // Verify we're on the login page
    await expect(page.getByRole('heading', { name: /sign in/i })).toBeVisible();
  });

  test('should be responsive', async ({ page }) => {
    // Test mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/');

    // Page should still render
    await expect(page.locator('main')).toBeVisible();

    // Test tablet viewport
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.goto('/');
    await expect(page.locator('main')).toBeVisible();

    // Test desktop viewport
    await page.setViewportSize({ width: 1920, height: 1080 });
    await page.goto('/');
    await expect(page.locator('main')).toBeVisible();
  });
});
