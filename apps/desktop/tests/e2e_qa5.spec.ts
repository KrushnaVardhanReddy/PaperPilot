import { test, expect } from '@playwright/test';
import * as path from 'path';

test.describe('QA-5: Annotation & Markup Tools', () => {

  test.beforeEach(async ({ page }) => {
    // Mock Tauri IPC
    await page.addInitScript(() => {
      (window as any).__TAURI_INTERNALS__ = {
        invoke: async (cmd: string, args: any) => {
          return null;
        }
      };
    });
  });

  test('Tool buttons toggle and activate properly, annotations work, count updates', async ({ page }) => {
    await page.goto('/');

    const fileInput = page.locator('input[multiple][type="file"]');
    await fileInput.setInputFiles(path.resolve('../../tests/e2e_fixtures/single_page.pdf'));

    const docName = page.locator('#tab-doc-0');
    await expect(docName).toBeVisible();
    await docName.click({ force: true });

    await page.waitForSelector('.viewer-container canvas');

    const ptrBtn = page.locator('#tool-btn-none');
    const noteBtn = page.locator('#tool-btn-note');
    const hlBtn = page.locator('#tool-btn-highlight');
    const ulBtn = page.locator('#tool-btn-underline');
    const stBtn = page.locator('#tool-btn-strikethrough');
    const penBtn = page.locator('#tool-btn-pen');

    // Pointer is active initially
    await expect(ptrBtn).toHaveClass(/active/);
    await expect(noteBtn).not.toHaveClass(/active/);

    // Toggle Note
    await noteBtn.click();
    await expect(noteBtn).toHaveClass(/active/);
    await expect(ptrBtn).not.toHaveClass(/active/);

    // Toggle Highlight
    await hlBtn.click();
    await expect(hlBtn).toHaveClass(/active/);

    // Toggle Underline
    await ulBtn.click();
    await expect(ulBtn).toHaveClass(/active/);

    // Toggle Strike
    await stBtn.click();
    await expect(stBtn).toHaveClass(/active/);

    // Toggle Pen
    await penBtn.click();
    await expect(penBtn).toHaveClass(/active/);

    // Return to Pointer
    await ptrBtn.click();
    await expect(ptrBtn).toHaveClass(/active/);

    // Select highlight again to test dragging
    await hlBtn.click();

    // check annotations count
    const badge = page.locator('#right-panel-tab-annotations'); // the Annotations tab
    await expect(badge).toContainText('Annotations (0)');

    // add highlight
    const annotationLayer = page.locator('.annotation-layer');
    await annotationLayer.dragTo(annotationLayer, {
      sourcePosition: { x: 100, y: 100 },
      targetPosition: { x: 200, y: 150 }
    });

    await expect(badge).toContainText('Annotations (1)');

    // Open right panel annotation tab
    await badge.click();

    // Check panel has 1 item
    const annotationItem = page.locator('.annotation-item').first();
    await expect(annotationItem).toBeVisible();

    // hover and delete
    await annotationItem.hover();
    const delBtn = annotationItem.locator('.remove-btn');
    await expect(delBtn).toBeVisible();
    await delBtn.click();

    await expect(badge).toContainText('Annotations (0)');
    await expect(annotationItem).not.toBeVisible();
  });
});
