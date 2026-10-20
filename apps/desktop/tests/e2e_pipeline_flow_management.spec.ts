import { test, expect } from '@playwright/test';
import { resolve } from 'path';

test.beforeEach(async ({ page }) => {
  await page.addInitScript(() => {
    localStorage.setItem('paperpilot_onboarding_completed', 'true');
  });
});

test.describe('Pipeline Flow Management: Save, Export, Import', () => {

  test('Preset selection, saving new flow, export and import JSON', async ({ page }) => {
    // Navigate to Pipeline view in the desktop app mock
    await page.goto('/');

    // Switch to Pipeline View
    await page.click('#nav-pipeline');
    await expect(page.locator('.pipeline-header')).toBeVisible();

    // The select box for FlowSelector should be visible
    const flowSelector = page.locator('.flow-select');
    await expect(flowSelector).toBeVisible();

    // 1. Select a preset: Legal Discovery
    await flowSelector.selectOption('preset-legal-discovery');

    // Assert steps appear (preset has 3 steps)
    await expect(page.locator('.step-card')).toHaveCount(3);

    // 2. Add a step to make it custom
    await page.click('#add-pipeline-step-btn');
    const compressTile = page.locator('#op-tile-pdf_compress');
    await expect(compressTile).toBeVisible();
    await compressTile.click();

    await expect(page.locator('.step-card')).toHaveCount(4);

    // Assert selector shows it's a custom/unsaved flow
    await expect(flowSelector).toHaveValue('');

    // 3. Save the custom flow
    const saveButton = page.getByRole('button', { name: /💾 Save Flow/i });
    await expect(saveButton).toBeEnabled();
    await saveButton.click();

    // Fill out the modal
    const modal = page.locator('.modal-drawer');
    await expect(modal).toBeVisible();
    await modal.getByLabel(/Flow Name/i).fill('Quarterly Discovery');
    await modal.getByLabel(/Description/i).fill('Test description');
    await modal.getByRole('button', { name: 'Save Pipeline' }).click();

    // Modal should close
    await expect(modal).not.toBeVisible();

    // Dropdown should have the new flow selected
    const selectedOption = await flowSelector.locator('option:checked').textContent();
    expect(selectedOption).toContain('Quarterly Discovery');

    // 4. Export JSON
    // Intercept download
    const downloadPromise = page.waitForEvent('download');
    await page.getByRole('button', { name: /⬇️ Export JSON/i }).click();
    const download = await downloadPromise;
    expect(download.suggestedFilename()).toMatch(/quarterly-discovery\.json/);

    // Save download to disk to test import
    const downloadPath = resolve('/tmp/', download.suggestedFilename());
    await download.saveAs(downloadPath);

    // 5. Click "New" to clear
    await page.getByRole('button', { name: /✨ New/i }).click();
    await expect(page.locator('.step-card')).toHaveCount(0);

    // 6. Import JSON
    // Setup file chooser for import
    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.getByRole('button', { name: /⬆️ Import JSON/i }).click();
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles(downloadPath);

    // Assert canvas restores accurately
    await expect(page.locator('.step-card')).toHaveCount(4);

    // Assert dropdown correctly selected the imported flow
    const importedOptionText = await flowSelector.locator('option:checked').textContent();
    expect(importedOptionText).toContain('Quarterly Discovery');
  });

});
