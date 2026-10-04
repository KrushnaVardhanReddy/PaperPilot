import { test, expect } from '@playwright/test';

test.describe('PaperPilot E2E Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Mock Tauri IPC invoke
    await page.addInitScript(() => {
      (window as any).__TAURI_INTERNALS__ = {
        invoke: async (cmd: string, args: any) => {
          if (cmd === 'invoke_mcp_tool') {
            return {
              success: true,
              message: `${args.toolName} completed successfully (mocked)`,
              data: {}
            };
          }
          return {};
        }
      };
    });

    await page.goto('/');
  });

  test('Test 1: App Layout', async ({ page }) => {
    // Verify Sidebar renders
    const sidebar = page.locator('.sidebar');
    await expect(sidebar).toBeVisible();

    // Verify Main Content renders
    const mainContent = page.locator('.main-content');
    await expect(mainContent).toBeVisible();

    // Verify Brand title is correct
    await expect(page.locator('.brand-title')).toHaveText('PaperPilot');
  });

  test('Test 2: Settings Panel', async ({ page }) => {
    // Click Settings in Sidebar
    await page.getByRole('button', { name: /Settings/i }).click();

    // Verify Settings Panel opens
    const settingsPanel = page.locator('.settings-panel');
    await expect(settingsPanel).toBeVisible();

    // Change AI Provider
    const aiProviderSelect = page.locator('#aiProvider');
    await aiProviderSelect.selectOption('anthropic');

    // Verify the value updated
    await expect(aiProviderSelect).toHaveValue('anthropic');
  });

  test('Test 3: Operations Execution', async ({ page }) => {
    // Navigate to Home to ensure we're on the documents view
    await page.getByRole('button', { name: /Home/i }).click();

    // Upload mock documents (merge requires at least 2)
    const fileInput = page.locator('input[multiple][type="file"]');
    await fileInput.setInputFiles([
      {
        name: 'test_document1.pdf',
        mimeType: 'application/pdf',
        buffer: Buffer.from('%PDF-1.4\n%EOF\n')
      },
      {
        name: 'test_document2.pdf',
        mimeType: 'application/pdf',
        buffer: Buffer.from('%PDF-1.4\n%EOF\n')
      }
    ]);

    await page.click('#tab-btn-home');



    await page.evaluate(() => { window.dispatchEvent(new CustomEvent('test-select-doc', { detail: null })) });
    await page.waitForSelector('#tool-search-input', { timeout: 10000 });
    // Select "Merge" operation
    const opSelect = page.locator('#tool-search-input');
    await opSelect.fill('merge');
    await page.locator('#tool-btn-merge').first().click();


    // Verify UI inputs for Merge appear (reorder list should have the file)
    const reorderList = page.locator('.reorder-list');
    await expect(reorderList).toBeVisible();
    await expect(page.locator('.item-name').first()).toHaveText('test_document1.pdf');

    // Click "Run Merge" button
    const runBtn = page.locator('.run-btn');
    await expect(runBtn).toBeEnabled();
    await runBtn.click();

    // Verify the Toast notification appears
    const successToast = page.locator('.toast-success');
    await expect(successToast).toBeVisible();
    await expect(page.locator('.toast-message')).toContainText('saved to');

    // Verify Job History list in the Sidebar updates with a new job entry
    const jobItem = page.locator('.job-item').first();
    await expect(jobItem).toBeVisible();
    await expect(jobItem.locator('.job-name')).toHaveText('pdf_merge');
  });
});
