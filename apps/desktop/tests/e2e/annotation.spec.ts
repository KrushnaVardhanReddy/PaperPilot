import { test, expect } from '@playwright/test';

test.describe('Annotation Layer E2E', () => {
  test('should allow creating and listing annotations', async ({ page }) => {
    // Navigate to the app
    await page.goto('/');

    // We mock Tauri internals here for testing without real backend
    await page.addInitScript(() => {
        (window as any).__TAURI_INTERNALS__ = {
            invoke: () => Promise.resolve([])
        };
    });

    // TODO: implement actual UI verification
  });
});
