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

    // 7. Verify Export as Code / API Modal
    const codeApiButton = page.locator('#flow-export-code-btn');
    await expect(codeApiButton).toBeVisible();
    await expect(codeApiButton).toBeEnabled();
    await codeApiButton.click();

    const exportCodeModal = page.locator('#export-code-modal');
    await expect(exportCodeModal).toBeVisible();

    // Verify Tabs and Code Content
    const codeBlock = exportCodeModal.locator('.code-container code');

    // cURL Tab (Default)
    await expect(exportCodeModal.locator('button', { hasText: 'cURL (REST API)' })).toHaveClass(/active/);
    await expect(codeBlock).toContainText('curl -X POST http://localhost:8080/api/v1/workflow/run');

    // CLI Tab
    await exportCodeModal.locator('button', { hasText: 'CLI Command' }).click();
    await expect(codeBlock).toContainText('paperpilot workflow --recipe flow.json');

    // MCP Agent Prompt Tab
    await exportCodeModal.locator('button', { hasText: 'MCP Agent Prompt' }).click();
    await expect(codeBlock).toContainText('Please invoke the `document_workflow` tool using the following recipe:');

    // Embed JS Tab
    await exportCodeModal.locator('button', { hasText: 'Embed JS' }).click();
    await expect(codeBlock).toContainText('window.PaperPilot.runPipeline({');

    // Close Modal
    await exportCodeModal.locator('.close-btn').click();
    await expect(exportCodeModal).not.toBeVisible();
  });

});
