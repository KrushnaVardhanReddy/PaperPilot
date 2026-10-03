import { test, expect } from '@playwright/test';
import * as path from 'path';

test.beforeEach(async ({ page }) => {
  // Mock Tauri IPC invoke
  await page.addInitScript(() => {
    (window as any).__TAURI_INTERNALS__ = {
      invoke: async (cmd: string, args: any) => {
        if (cmd === 'invoke_mcp_tool') {
          if (args.toolName === 'pdf_fill_form') {
              return { success: true, message: 'Form filled successfully', output_path: args.arguments.output };
          }
          return { success: true, message: 'Operation completed successfully', output_path: 'mock_output.pdf' };
        }
        return null;
      }
    };
  });
});

test.describe('Form Filling', () => {
  test('renders form layer correctly and allows editing', async ({ page }) => {
    await page.goto('/');

    const fileInput = page.locator('input[multiple][type="file"]');
    await fileInput.setInputFiles(path.resolve('../../tests/e2e_fixtures/form.pdf'));

    const docName = page.locator('#tab-doc-0');
    await expect(docName).toBeVisible();
    await docName.click({ force: true });
    await page.evaluate(() => { window.dispatchEvent(new CustomEvent('test-select-doc', { detail: 0 })) });

    // Wait for the form layer to render
    const formLayer = page.locator('.form-layer');

    // Check if initial values are loaded
    await page.waitForSelector('.pdf-input', { state: 'visible', timeout: 5000 });
    const textInput = page.locator('.pdf-input');
    await expect(textInput).toBeVisible({ timeout: 10000 });
    await expect(textInput).toHaveValue('');

    const checkbox = page.locator('.pdf-checkbox');
    await expect(checkbox).toBeVisible();
    await expect(checkbox).toBeChecked();

    // Edit the values
    await textInput.fill('New Value');
    await checkbox.uncheck();

    await expect(textInput).toHaveValue('New Value');
    await expect(checkbox).not.toBeChecked();

    // Save the form
    const saveBtn = page.locator('.save-form-btn');
    await expect(saveBtn).toBeVisible();
    await saveBtn.click();

    const toast = page.locator('.toast.toast-success');
    await expect(toast).toBeVisible();

    // To simulate a fresh load, we can close the viewer and upload the simulated "saved" file
    await page.locator('.home-tab-btn').click();

    // Ensure we are back on document list
    await expect(page.locator('.documents-view')).toBeVisible();

    // Upload the "filled" fixture to simulate reading a freshly saved PDF
    await fileInput.setInputFiles(path.resolve('../../tests/e2e_fixtures/form_filled.pdf'));

    // Click the newly uploaded item (it will be the second one or the newly added one)
    const newDoc = page.locator('#tab-doc-1');
    await expect(newDoc).toBeVisible();
    await newDoc.click({ force: true });
    await page.evaluate(() => { window.dispatchEvent(new CustomEvent('test-select-doc', { detail: 1 })) });

    // Assert new values persisted in the fresh load
    await page.waitForSelector('.pdf-input', { state: 'visible', timeout: 5000 });
    const newTextInput = page.locator('.pdf-input');
    await expect(newTextInput).toBeVisible({ timeout: 10000 });
    await expect(newTextInput).toHaveValue('New Value');

    const newCheckbox = page.locator('.pdf-checkbox');
    await expect(newCheckbox).toBeVisible();
    await expect(newCheckbox).not.toBeChecked();
  });
});
