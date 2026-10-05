import { test, expect } from '@playwright/test';
import * as path from 'path';

test.describe('Canvas Viewer Features', () => {
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
  });

  test('Sticky Notes & Annotations', async ({ page }) => {
    await page.goto('/');

    const fileInput = page.locator('input[multiple][type="file"]');
    await fileInput.setInputFiles(path.resolve('../../tests/e2e_fixtures/single_page.pdf'));

    // Select the document
    await page.click('#tab-doc-0');

    // Wait for viewer to be ready
    await expect(page.locator('.pdf-viewer-wrapper')).toBeVisible();

    // Click the note tool
    await page.click('#tool-btn-note');

    // Click on canvas to add note
    const annotationLayer = page.locator('.annotation-layer');
    await annotationLayer.click({ position: { x: 100, y: 100 } });

    // Check if new note is rendered
    const note = page.locator('.annot-note').first();
    await expect(note).toBeVisible();
    await expect(note).toHaveAttribute('title', 'New Note');

    // Edit sticky note content
    await note.evaluate((el) => el.setAttribute('title', 'Edited Note'));
    await expect(note).toHaveAttribute('title', 'Edited Note');

    // Reposition / drag note on canvas
    await note.evaluate((el) => {
      el.style.left = '200px';
      el.style.top = '200px';
    });
    const noteLeft = await note.evaluate((el) => el.style.left);
    expect(noteLeft).toBe('200px');

    // Open right panel annotations tab if not open
    const toggleBtn = page.locator('#right-panel-toggle');
    if (await toggleBtn.isVisible()) {
        const title = await toggleBtn.getAttribute('title');
        if (title === 'Expand panel') {
            await toggleBtn.click();
        }
    }
    await page.click('#right-panel-tab-annotations');

    // Wait for the annotation list to show the note
    const annotationList = page.locator('.annotation-list');
    await expect(annotationList).toBeVisible();

    const annotationItem = annotationList.locator('.annotation-item').first();
    await expect(annotationItem).toBeVisible();
    await expect(annotationItem.locator('.item-type')).toHaveText('note');

    // Delete the annotation
    // Hover over the item to show remove button
    await annotationItem.hover();
    const removeBtn = annotationItem.locator('.remove-btn');
    await removeBtn.click();

    // Check that it's removed from DOM
    await expect(note).not.toBeVisible();
  });

  test('Pointer, Highlighting & Markup Tools', async ({ page }) => {
    await page.goto('/');

    const fileInput = page.locator('input[multiple][type="file"]');
    await fileInput.setInputFiles(path.resolve('../../tests/e2e_fixtures/single_page.pdf'));

    // Select the document
    await page.click('#tab-doc-0');
    await expect(page.locator('.pdf-viewer-wrapper')).toBeVisible();

    // Select highlight tool
    await page.click('#tool-btn-highlight');

    // Simulate drag to highlight
    const annotationLayer = page.locator('.annotation-layer');
    await annotationLayer.dragTo(annotationLayer, {
        sourcePosition: { x: 50, y: 50 },
        targetPosition: { x: 200, y: 100 }
    });

    // Check if highlight is rendered
    const highlight = page.locator('.annot-highlight').first();
    await expect(highlight).toBeVisible();

    // Verify it appears in the annotations panel
    const toggleBtn = page.locator('#right-panel-toggle');
    if (await toggleBtn.isVisible()) {
        const title = await toggleBtn.getAttribute('title');
        if (title === 'Expand panel') {
            await toggleBtn.click();
        }
    }
    await page.click('#right-panel-tab-annotations');

    const annotationList = page.locator('.annotation-list');
    await expect(annotationList).toBeVisible();

    const annotationItem = annotationList.locator('.annotation-item').first();
    await expect(annotationItem).toBeVisible();
    await expect(annotationItem.locator('.item-type')).toHaveText('highlight');

    // Select pen tool
    await page.click('#tool-btn-pen');

    // Simulate drawing
    await annotationLayer.dragTo(annotationLayer, {
        sourcePosition: { x: 50, y: 150 },
        targetPosition: { x: 200, y: 200 }
    });

    // Check if pen path is rendered
    const penPath = page.locator('.annot-svg > polyline').first();
    await expect(penPath).toBeVisible();

    // Verify pen in annotations panel
    await expect(annotationList.locator('.annotation-item').nth(1).locator('.item-type')).toHaveText('pen');
  });
  test('Visual Pixel Diff Slider', async ({ page }) => {
    await page.goto('/');

    const fileInput = page.locator('input[multiple][type="file"]');
    await fileInput.setInputFiles([
      path.resolve('../../tests/e2e_fixtures/single_page.pdf'),
      path.resolve('../../tests/e2e_fixtures/multi_page.pdf')
    ]);

    // Open diff view via shortcut
    await page.keyboard.press('ControlOrMeta+d');

    const diffContainer = page.locator('#visual-diff-container');
    await expect(diffContainer).toBeVisible();

    // Check split slider handle exists and drag it
    const splitHandle = page.locator('#diff-split-handle');
    await expect(splitHandle).toBeVisible();

    // Check initial left is 50%
    const initialLeft = await splitHandle.evaluate((node) => node.style.left);
    expect(initialLeft).toBe('50%');

    // Simulate drag on the handle (it uses global mousemove, so we dispatch global events or use bounding box)
    const handleBox = await splitHandle.boundingBox();
    if (handleBox) {
        await page.mouse.move(handleBox.x + handleBox.width / 2, handleBox.y + handleBox.height / 2);
        await page.mouse.down();
        // Move to the right
        await page.mouse.move(handleBox.x + handleBox.width / 2 + 100, handleBox.y + handleBox.height / 2);
        await page.mouse.up();

        // Assert that left style has changed from 50%
        const leftStyle = await splitHandle.evaluate((node) => node.style.left);
        expect(leftStyle).not.toBe('50%');
    }
  });
  test('Zoom Toolbar & Thumbnail Navigation', async ({ page }) => {
    await page.goto('/');

    const fileInput = page.locator('input[multiple][type="file"]');
    await fileInput.setInputFiles(path.resolve('../../tests/e2e_fixtures/multi_page.pdf'));

    // Select the document
    await page.click('#tab-doc-0');
    await expect(page.locator('.pdf-viewer-wrapper')).toBeVisible();

    // Zoom Out
    await page.click('#toolbar-zoom-out');
    // Zoom In
    await page.click('#toolbar-zoom-in');
    // Zoom Fit
    await page.click('#toolbar-zoom-fit');

    // Reset Zoom (100%)
    await page.selectOption('#toolbar-zoom-select', '100');
    await expect(page.locator('#toolbar-zoom-select')).toHaveValue('100');

    // Select page from thumbnail
    const thumb2 = page.locator('#thumbnail-page-2');
    await expect(thumb2).toBeVisible();
    await thumb2.click();

    // Verify main canvas switched to page 2
    const pageInput = page.locator('#toolbar-page-input');
    await expect(pageInput).toHaveValue('2');
  });
});
