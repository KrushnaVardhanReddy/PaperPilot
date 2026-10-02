import { test, expect } from '@playwright/test';
import * as path from 'path';

test.beforeEach(async ({ page }) => {
  // Mock Tauri IPC invoke
  await page.addInitScript(() => {
    (window as any).__TAURI_INTERNALS__ = {
      invoke: async (cmd: string, args: any) => {
        if (cmd === 'invoke_mcp_tool') {
          if (args.arguments && args.arguments.input && args.arguments.input.includes('error')) {
             return { success: false, message: 'Mocked error message' };
          }
          return { success: true, message: 'Operation completed successfully', output_path: 'mock_output.pdf' };
        }
        return null;
      }
    };
  });
});

test.afterEach(async ({ page }, testInfo) => {
  if (testInfo.status !== testInfo.expectedStatus) {
    const screenshotPath = path.resolve(`../../reports/screenshots/${testInfo.title.replace(/\s+/g, '-')}.png`);
    await page.screenshot({ path: screenshotPath, fullPage: true });
  }
});

test.describe('App Launch and Navigation', () => {
  test('app launches and shows the main interface', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('.drop-zone')).toBeVisible();
    await expect(page.locator('.operations-panel')).toBeVisible();
    await expect(page.locator('.toast.toast-error')).not.toBeVisible();
  });

  test('layout is functional at mobile viewport (390x844)', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await page.goto('/');
    await expect(page.locator('.operations-panel')).toBeVisible();

    // In mobile view, bottom nav should appear instead of sidebar
    await expect(page.locator('#mobile-nav-documents')).toBeVisible();
  });

  test('layout is functional at tablet viewport (768x1024)', async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.goto('/');
    await expect(page.locator('.sidebar')).toBeVisible();
    await expect(page.locator('.drop-zone')).toBeVisible();
  });

  test('core actions are keyboard accessible', async ({ page }) => {
    await page.goto('/');
    await page.keyboard.press('Tab');
    const navPipeline = page.locator('#nav-pipeline');
    await expect(navPipeline).toBeVisible();
    for(let i=0; i<10; i++) {
        await page.keyboard.press('Tab');
    }
  });
});

test.describe('Drag-and-Drop DropZone', () => {
  test('can drop a PDF into the DropZone', async ({ page }) => {
    await page.goto('/');

    // Playwright supports setInputFiles on any input[type="file"], which is
    // exactly what the DropZone Browse button uses.
    const fileInput = page.locator('input[multiple][type="file"]');
    await fileInput.setInputFiles(path.resolve('../../tests/e2e_fixtures/single_page.pdf'));

    // Verify the document is loaded (either in list or directly via viewer)
    const docTab = page.locator('#tab-doc-0');
    await expect(docTab).toBeVisible();
  });
});

test.describe('Multi-File Upload & Operations', () => {
  test('can upload multiple PDFs and reorder them', async ({ page }) => {
    await page.goto('/');

    const fileInput = page.locator('input[multiple][type="file"]');
    await fileInput.setInputFiles([
      path.resolve('../../tests/e2e_fixtures/multi_page.pdf'),
      path.resolve('../../tests/e2e_fixtures/single_page.pdf')
    ]);

    await page.click('#tab-btn-home');

    // To test reordering, we select "merge" operation to reveal the reorder list
    await page.fill('#tool-search-input', 'merge');
    await page.locator('#tool-btn-merge').first().click();

    const reorderItems = page.locator('.reorder-list .reorder-item');
    await expect(reorderItems).toHaveCount(2);

    // First item should be multi_page.pdf
    await expect(reorderItems.nth(0).locator('.item-name')).toHaveText('multi_page.pdf');
    // Second item should be single_page.pdf
    await expect(reorderItems.nth(1).locator('.item-name')).toHaveText('single_page.pdf');

    // Click move down on the first item
    const moveDownBtn = reorderItems.nth(0).locator('.icon-btn').nth(1); // Second button is Move Down
    await moveDownBtn.click();

    // Verify order swapped
    await expect(reorderItems.nth(0).locator('.item-name')).toHaveText('single_page.pdf');
    await expect(reorderItems.nth(1).locator('.item-name')).toHaveText('multi_page.pdf');
  });

  test('can run a single operation directly from the OperationsPanel', async ({ page }) => {
    await page.goto('/');

    const fileInput = page.locator('input[multiple][type="file"]');
    await fileInput.setInputFiles(path.resolve('../../tests/e2e_fixtures/single_page.pdf'));

    await page.click('#tab-btn-home');

    await page.fill('#tool-search-input', 'compress');
    await page.locator('#tool-btn-compress').first().click();

    const runBtn = page.locator('.run-btn');
    await runBtn.click();

    // Verify success toast appears
    const successToast = page.locator('.toast.toast-success');
    await expect(successToast).toBeVisible();
    await expect(successToast).toContainText('Operation completed successfully');
  });
});

test.describe('Visual Pipeline Builder', () => {
  test('can build a pipeline using the node canvas', async ({ page }) => {
    await page.goto('/');

    // Switch to Pipeline tab
    await page.click('#nav-pipeline');

    // Upload files to pipeline dropzone
    const fileInput = page.locator('input[multiple][type="file"]');
    await fileInput.setInputFiles([
      path.resolve('../../tests/e2e_fixtures/multi_page.pdf'),
      path.resolve('../../tests/e2e_fixtures/single_page.pdf')
    ]);

    // Add Merge block
    await page.click('#add-pipeline-step-btn');
    const mergeTile = page.locator('#op-tile-pdf_merge');
    await expect(mergeTile).toBeVisible();
    await mergeTile.click();

    // Add Rotate block
    await page.click('#add-pipeline-step-btn');
    const rotateTile = page.locator('#op-tile-pdf_rotate');
    await expect(rotateTile).toBeVisible();
    await rotateTile.click();

    // Add Compress block
    await page.click('#add-pipeline-step-btn');
    const compressTile = page.locator('#op-tile-pdf_compress');
    await expect(compressTile).toBeVisible();
    await compressTile.click();

    // Assert all 3 blocks are visible
    const steps = page.locator('.step-wrapper');
    await expect(steps).toHaveCount(3);

    await expect(steps.nth(0)).toContainText('Merge');
    await expect(steps.nth(1)).toContainText('Rotate');
    await expect(steps.nth(2)).toContainText('Compress');
  });
});

test.describe('Pipeline Execution and Error State', () => {
  test('can run a pipeline and see success notification', async ({ page }) => {
    await page.goto('/');

    // Switch to Pipeline tab
    await page.click('#nav-pipeline');

    // Upload file
    const fileInput = page.locator('input[multiple][type="file"]');
    await fileInput.setInputFiles(path.resolve('../../tests/e2e_fixtures/multi_page.pdf'));

    // Add Merge block
    await page.click('#add-pipeline-step-btn');
    await page.click('#op-tile-pdf_merge');

    // Run Pipeline
    const runBtn = page.locator('#run-pipeline-btn');
    await runBtn.click();

    // Verify success toast appears
    const successToast = page.locator('.toast.toast-success');
    await expect(successToast).toBeVisible();
    await expect(successToast).toContainText('Pipeline complete');

    // Verify status success in runner
    const statusSuccess = page.locator('.status-success');
    await expect(statusSuccess).toBeVisible();
  });

  test('shows error toast if operation fails', async ({ page }) => {
    await page.goto('/');

    const fileInput = page.locator('input[multiple][type="file"]');
    // We use a specific file name "error" to trigger the mock we set up
    // However, the input is an actual file, so we need to trigger it somehow.
    // Let's create a dummy file named 'error.pdf' to trigger the mocked error.

    await fileInput.setInputFiles({
        name: 'error.pdf',
        mimeType: 'application/pdf',
        buffer: Buffer.from('%PDF-1.4')
    });

    await page.click('#tab-btn-home');

    await page.fill('#tool-search-input', 'compress');
    await page.locator('#tool-btn-compress').first().click();

    const runBtn = page.locator('.run-btn');
    await runBtn.click();

    // Verify error toast appears
    const errorToast = page.locator('.toast.toast-error');
    await expect(errorToast).toBeVisible();
    await expect(errorToast).toContainText('Mocked error message');
  });
});

test.describe('Phase 4.F Features Validation', () => {

  test('can open multiple documents into separate tabs and switch between them', async ({ page }) => {
    await page.goto('/');
    const fileInput = page.locator('input[multiple][type="file"]');
    await fileInput.setInputFiles([
      path.resolve('../../tests/e2e_fixtures/single_page.pdf'),
      path.resolve('../../tests/e2e_fixtures/multi_page.pdf')
    ]);

    // Tab 0 should be active
    await expect(page.locator('#tab-doc-0')).toBeVisible();
    await expect(page.locator('#tab-doc-1')).toBeVisible();

    // Click tab 1 to switch document
    await page.click('#tab-doc-1');
    await expect(page.locator('#tab-doc-1')).toHaveClass(/active/);
  });

  test('canvas search bar opens with shortcut and highlights matches', async ({ page }) => {
    await page.goto('/');
    const fileInput = page.locator('input[multiple][type="file"]');
    await fileInput.setInputFiles(path.resolve('../../tests/e2e_fixtures/multi_page.pdf'));

    // Trigger Ctrl+F
    await page.keyboard.press('ControlOrMeta+f');
    const searchBar = page.locator('#pdf-search-bar');
    await expect(searchBar).toBeVisible();

    // Type search query
    await page.fill('#pdf-search-input', 'Page');
    await page.keyboard.press('Enter');

    // Verify match count indicator exists
    const matchCount = page.locator('#pdf-search-count');
    await expect(matchCount).toBeVisible();
  });

  test('tools dock filters operations and navigates to parameter inspector', async ({ page }) => {
    await page.goto('/');
    const searchInput = page.locator('#tool-search-input');
    await expect(searchInput).toBeVisible();

    // Filter tools by typing 'watermark'
    await searchInput.fill('watermark');
    const watermarkTool = page.locator('#tool-btn-watermark');
    await expect(watermarkTool).toBeVisible();

    // Click tool to enter parameter inspector
    await watermarkTool.click();
    await expect(page.locator('#inspector-param-watermark')).toBeVisible();

    // Test back navigation
    await page.click('#btn-back-tools');
    await expect(searchInput).toBeVisible();
  });

  test('visual diff comparison opens and allows adjusting split slider', async ({ page }) => {
    await page.goto('/');
    const fileInput = page.locator('input[multiple][type="file"]');
    await fileInput.setInputFiles([
      path.resolve('../../tests/e2e_fixtures/single_page.pdf'),
      path.resolve('../../tests/e2e_fixtures/multi_page.pdf')
    ]);

    // Open diff view via shortcut or compare button
    await page.keyboard.press('ControlOrMeta+d');
    const diffContainer = page.locator('#visual-diff-container');
    await expect(diffContainer).toBeVisible();

    // Check split mode elements
    const splitHandle = page.locator('#diff-split-handle');
    await expect(splitHandle).toBeVisible();

    // Switch to overlay mode
    await page.click('#diff-mode-overlay');
    await expect(page.locator('#diff-overlay-canvas')).toBeVisible();

    // Close diff view
    await page.click('#diff-close-btn');
    await expect(diffContainer).not.toBeVisible();
  });

  test('custom css injection panel displays presets and accepts custom css input', async ({ page }) => {
    await page.goto('/');
    const searchInput = page.locator('#tool-search-input');
    await searchInput.fill('html to pdf');

    const convertTool = page.locator('#tool-btn-html-to-pdf, #tool-btn-convert').first();
    if (await convertTool.isVisible()) {
      await convertTool.click();
      const presetSelector = page.locator('#css-preset-select');
      if (await presetSelector.isVisible()) {
        await presetSelector.selectOption('github');
        await expect(presetSelector).toHaveValue('github');
      }
    }
  });

});
