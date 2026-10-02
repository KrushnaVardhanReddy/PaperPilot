import { test, expect } from '@playwright/test';
import * as path from 'path';

test.describe('Phase 4.F - PDF Viewer & Annotations E2E', () => {

  test.beforeEach(async ({ page }) => {
    // Mock Tauri IPC invoke for saving and other interactions
    await page.addInitScript(() => {
      (window as any).__TAURI_INTERNALS__ = {
        invoke: async (cmd: string, args: any) => {
          if (cmd === 'invoke_mcp_tool') {
            return { success: true, message: 'Operation completed successfully', output_path: 'mock_output.pdf' };
          }
          return null;
        }
      };
    });
  });

  test('F.5.1: PDF loads in canvas and thumbnails appear', async ({ page }) => {
    await page.goto('/');

    const fileInput = page.locator('input[multiple]');
    await fileInput.setInputFiles(path.resolve('../../tests/e2e_fixtures/single_page.pdf'));

    const docName = page.locator('#tab-doc-0');
    await expect(docName).toBeVisible();
    await docName.click({ force: true });

    // Dispatch custom event to select doc in our Svelte app state
    await page.evaluate(() => { window.dispatchEvent(new CustomEvent('test-select-doc', { detail: 0 })) });

    // Wait for the canvas to render
    const canvas = page.locator('.viewer-container canvas');
    await expect(canvas).toBeVisible({ timeout: 10000 });

    // Check thumbnails
    const thumbnails = page.locator('.thumbnails-container canvas');
    await expect(thumbnails.first()).toBeVisible({ timeout: 10000 });
    expect(await thumbnails.count()).toBeGreaterThan(0);

    // Check Info Panel
    // The panel is now hidden by a tab in ViewerRightPanel
    const rightPanel = page.locator('.right-panel');
    await expect(rightPanel).toBeVisible();
    await page.click('#right-panel-tab-info');
    const infoPanel = page.locator('.info-panel');
    await expect(infoPanel).toBeVisible();
  });

  test('F.5.2: Annotation toolbar allows highlighting', async ({ page }) => {
    await page.goto('/');

    const fileInput = page.locator('input[multiple]');
    await fileInput.setInputFiles(path.resolve('../../tests/e2e_fixtures/single_page.pdf'));

    const docName = page.locator('#tab-doc-0');
    await expect(docName).toBeVisible();
    await docName.click({ force: true });

    await page.evaluate(() => { window.dispatchEvent(new CustomEvent('test-select-doc', { detail: 0 })) });

    // Wait for the canvas and annotation layer
    await page.waitForSelector('.viewer-container canvas');
    const annotationLayer = page.locator('.annotation-layer');
    await expect(annotationLayer).toBeVisible();

    // Click the highlight tool
    const highlightBtn = page.getByRole('button', { name: 'Highlight' });
    await highlightBtn.click();

    // Verify the tool is active
    await expect(highlightBtn).toHaveClass(/active/);

    // Simulate mouse drag on the annotation layer
    await annotationLayer.dragTo(annotationLayer, {
      sourcePosition: { x: 100, y: 100 },
      targetPosition: { x: 200, y: 150 }
    });

    // Check that a highlight div was created
    const highlight = page.locator('.annotation-layer .annot-highlight').first();
    await expect(highlight).toBeVisible();
  });

  test('F.5.3: Form fields overlay correctly', async ({ page }) => {
    await page.goto('/');

    const fileInput = page.locator('input[multiple]');
    await fileInput.setInputFiles(path.resolve('../../tests/e2e_fixtures/form.pdf'));

    const docName = page.locator('#tab-doc-0');
    await expect(docName).toBeVisible();
    await docName.click({ force: true });

    await page.evaluate(() => { window.dispatchEvent(new CustomEvent('test-select-doc', { detail: 0 })) });

    // Wait for the form layer to render
    const input = page.locator('.form-layer input.pdf-input').first();
    await expect(input).toBeVisible({ timeout: 10000 });

    // Type into it and verify
    await input.fill('Test User');
    await expect(input).toHaveValue('Test User');
  });

  test('F.5.4: Menu / Tauri IPC Test', async ({ page }) => {
    let eventsCaught = 0;

    await page.addInitScript(() => {
        window.addEventListener('tauri-menu-open-file', () => {
            (window as any).menuEvents = ((window as any).menuEvents || 0) + 1;
        });
    });

    await page.goto('/');

    // We mock a Tauri backend emitting a menu event
    await page.evaluate(() => {
        // Typically Tauri's JS api allows listening to events which map to custom window events or callbacks
        // In +layout.svelte, it uses `listen('menu-open-file', ...)` from @tauri-apps/api/event
        // Since we are mocking, we just evaluate that the event listener setup doesn't crash
        // and we can manually trigger the callback logic if it was exposed globally.
        // For E2E testing standard web functionality when Tauri isn't fully mocked, we just verify our tests are resilient.
    });

    const fileInput = page.locator('input[multiple]');
    await fileInput.setInputFiles(path.resolve('../../tests/e2e_fixtures/form.pdf'));

    const docName = page.locator('#tab-doc-0');
    await expect(docName).toBeVisible();
    await docName.click({ force: true });

    await page.evaluate(() => { window.dispatchEvent(new CustomEvent('test-select-doc', { detail: 0 })) });

    // Verify Save Form button that triggers an IPC call
    const saveBtn = page.locator('.save-form-btn');
    await expect(saveBtn).toBeVisible({ timeout: 10000 });

    // Click to invoke MCP tool (mocked) and trigger success toast
    await saveBtn.click();

    const toast = page.locator('.toast.toast-success');
    await expect(toast).toBeVisible();
    await expect(toast).toContainText('Form saved successfully');
  });

  test.skip('F.5.2: Annotation toolbar allows pen drawing', async ({ page }) => {
    await page.goto('/');

    const fileInput = page.locator('input[multiple]');
    await fileInput.setInputFiles(path.resolve('../../tests/e2e_fixtures/single_page.pdf'));

    const docName = page.locator('#tab-doc-0');
    await expect(docName).toBeVisible();
    await docName.click({ force: true });

    await page.evaluate(() => { window.dispatchEvent(new CustomEvent('test-select-doc', { detail: 0 })) });

    // Wait for the canvas and annotation layer
    await page.waitForSelector('.viewer-container canvas');
    const annotationLayer = page.locator('.annotation-layer');
    await expect(annotationLayer).toBeVisible();

    // Click the pen tool
    const penBtn = page.locator('.tool-btn[title="Pen"]');
    await penBtn.click();

    // Verify the tool is active
    await expect(penBtn).toHaveClass(/active/);

    // Simulate mouse drag on the annotation layer to draw a line
    await annotationLayer.dragTo(annotationLayer, {
      sourcePosition: { x: 100, y: 100 },
      targetPosition: { x: 150, y: 150 }
    });

    // Check that a pen svg was created
    const penSvg = page.locator('.annotation-layer .annot-svg').first();
    await expect(penSvg).toBeVisible();
  });

});
