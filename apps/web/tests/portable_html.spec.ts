import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const portableHtmlPath = path.resolve(__dirname, '../../../dist-portable/paperpilot-portable.html');

test.describe('PaperPilot Portable Air-Gapped HTML Sandbox', () => {
  test('file exists and is self-contained without external network calls', async ({ page }) => {
    expect(fs.existsSync(portableHtmlPath)).toBe(true);

    const networkRequests: string[] = [];

    // Intercept all network traffic
    page.on('request', (req) => {
      const url = req.url();
      // Allow file:// navigation of the document itself and data/blob URLs
      if (!url.startsWith('file:') && !url.startsWith('data:') && !url.startsWith('blob:')) {
        networkRequests.push(url);
      }
    });

    // Navigate to local portable file
    await page.goto(`file://${portableHtmlPath}`);

    // Verify Title and Air-Gapped Badge
    await expect(page.locator('h1')).toHaveText('PaperPilot Portable');
    await expect(page.locator('.badge-airgap')).toContainText('100% Air-Gapped');

    // Wait for WASM in-memory initialization
    const executeBtn = page.locator('#executeBtn');
    await expect(executeBtn).toHaveText('Run Merge', { timeout: 15000 });

    // Assert strictly ZERO external network requests were dispatched
    expect(networkRequests).toEqual([]);
  });

  test('renders tool controls when switching tools', async ({ page }) => {
    await page.goto(`file://${portableHtmlPath}`);

    // Click Rotate
    await page.click('button[data-tool="rotate"]');
    const paramInput = page.locator('#toolParamInput');
    await expect(paramInput).toBeVisible();
    await expect(paramInput).toHaveValue('90');

    // Click Split
    await page.click('button[data-tool="split"]');
    await expect(paramInput).toBeVisible();
    await expect(paramInput).toHaveValue('1');

    // Click Merge
    await page.click('button[data-tool="merge"]');
    await expect(paramInput).toBeHidden();
  });
});
