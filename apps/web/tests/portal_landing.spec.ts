import { test, expect } from '@playwright/test';

test.describe('usepaperpilot.com Official Web Portal', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('Hero playground loads and handles UI interactions', async ({ page }) => {
    // Check headline and tagline
    await expect(page.locator('.hero-title')).toHaveText('The Pure-Rust, Zero-Cloud PDF Powerhouse.');
    await expect(page.locator('.hero-tagline')).toContainText('100% Client-Side');

    // Check CTAs
    const downloadBtn = page.locator('a.cta-btn.primary');
    await expect(downloadBtn).toBeVisible();
    await expect(downloadBtn).toHaveAttribute('href', 'https://github.com/paperpilot/paperpilot/releases');

    const embedBtn = page.locator('button.cta-btn.secondary');
    await expect(embedBtn).toBeVisible();

    // Scroll down to embed generator
    await embedBtn.click();
    // Assuming smooth scroll completes, check if section is visible in viewport
    await expect(page.locator('#embed-generator')).toBeInViewport();

    // Playground window frame is visible
    await expect(page.locator('.playground-wrapper')).toBeVisible();

    // Check if tools sidebar is present (from OperationsView)
    await expect(page.locator('.tools-sidebar')).toBeVisible();
  });

  test('Tri-surface showcase links function', async ({ page }) => {
    // Check desktop app download link
    const desktopLink = page.locator('.surface-card').nth(0).locator('.card-link');
    await expect(desktopLink).toHaveAttribute('href', 'https://paperpilot.app/download');

    // Check configure widget button scrolls
    const widgetBtn = page.locator('.surface-card').nth(1).locator('.card-link');
    await widgetBtn.click();
    await expect(page.locator('#embed-generator')).toBeInViewport();

    // Check API explorer button scrolls
    const apiBtn = page.locator('.surface-card').nth(2).locator('.card-link');
    await apiBtn.click();
    await expect(page.locator('#api-explorer')).toBeInViewport();
  });

  test('Embed generator updates reactively and copies code', async ({ page, browserName, context }) => {
    test.skip(browserName === 'webkit', 'Clipboard API can be flaky in WebKit headless');
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    // Navigate to generator
    await page.locator('button.cta-btn.secondary').click();

    // Change theme to light
    await page.locator('#theme-select').selectOption('light');

    // Toggle 'merge' (initially active, should deactivate)
    await page.locator('.tool-pill', { hasText: 'merge' }).click();

    // Toggle 'watermark' (initially inactive, should activate)
    await page.locator('.tool-pill', { hasText: 'watermark' }).click();

    // Toggle hide badge
    await page.locator('#hide-badge').check();

    // Verify code block updates
    const codeBlock = page.locator('.code-block code');
    await expect(codeBlock).toContainText('data-theme="light"');
    await expect(codeBlock).not.toContainText('merge');
    await expect(codeBlock).toContainText('watermark');
    await expect(codeBlock).toContainText('data-hide-badge="true"');

    // Test copy button
    const copyBtn = page.locator('.copy-btn');

    // Wait for clipboard to be populated correctly by giving the page time to respond
    await copyBtn.click();
    await expect(copyBtn).toHaveText('✓ Copied!');

    // Verify clipboard content
    const clipboardText = await page.evaluate("navigator.clipboard.readText()");
    expect(clipboardText).toContain('data-theme="light"');
  });

  test('API explorer tab switching works', async ({ page }) => {
    // Navigate to API explorer
    await page.locator('.surface-card').nth(2).locator('.card-link').click();

    // Initial state (cURL)
    await expect(page.locator('.code-content')).toContainText('curl -X POST');

    // Switch to TypeScript
    await page.locator('.tab', { hasText: 'TypeScript' }).click();
    await expect(page.locator('.code-content')).toContainText('import { PaperPilot } from \'@paperpilot/client\'');

    // Switch to Python
    await page.locator('.tab', { hasText: 'Python' }).click();
    await expect(page.locator('.code-content')).toContainText('from paperpilot import PaperPilot');

    // Switch to MCP
    await page.locator('.tab', { hasText: 'MCP' }).click();
    await expect(page.locator('.code-content')).toContainText('"command": "npx"');
  });
});
