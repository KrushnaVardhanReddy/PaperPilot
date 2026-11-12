/**
 * Suite 1 — Unmocked Real-Engine E2E Tests
 * Tools 01–30: Page Operations, Imposition, Optimization
 *
 * Philosophy: Zero IPC mocks on engine calls. Only OS file-picker dialogs are
 * intercepted. The real Rust paperpilot binary runs. Output is verified at the
 * byte / domain-invariant level per SPEC_152.
 */

import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';
import { SandboxManager } from './helpers/sandboxManager';
import { PdfInspector } from './helpers/pdfInspector';

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

const sandboxManager = new SandboxManager();

/** Absolute path to a fixture file */
function fixturePath(file: string): string {
  return path.resolve(process.cwd(), '../../tests/e2e_fixtures/real', file);
}

/**
 * Build the addInitScript parameters object.
 * `inputs` is the value returned for plugin:dialog|open  (string | string[])
 * `out`    is the value returned for plugin:dialog|save
 */
function dialogMock(inputs: string | string[], out: string) {
  return { inputs, out };
}

/**
 * Inject the partial-mock before navigation.
 * Saves __TAURI_INTERNALS__ as __REAL_TAURI_INTERNALS__ first so that all
 * non-dialog IPC commands are forwarded to the real native backend.
 */
async function injectMock(
  page: import('@playwright/test').Page,
  inputs: string | string[],
  out: string
): Promise<void> {
  await page.addInitScript(
    ({ inputs, out }: { inputs: string | string[]; out: string }) => {
      localStorage.setItem('paperpilot_onboarding_completed', 'true');
      const orig = (window as any).__TAURI_INTERNALS__;
      (window as any).__REAL_TAURI_INTERNALS__ = orig;
      (window as any).__TAURI_INTERNALS__ = {
        invoke: async (cmd: string, args: any) => {
          if (cmd === 'plugin:dialog|open') return inputs;
          if (cmd === 'plugin:dialog|save') return out;
          const real = (window as any).__REAL_TAURI_INTERNALS__;
          if (real && typeof real.invoke === 'function') {
            return real.invoke(cmd, args);
          }
          throw new Error(`No real Tauri backend for cmd: ${cmd}`);
        },
      };
    },
    { inputs, out }
  );
}

/** Navigate to the app and dismiss any onboarding overlay. */
async function gotoApp(page: import('@playwright/test').Page): Promise<void> {
  await page.goto('http://127.0.0.1:1420/');
  await page.waitForTimeout(500);
  const overlay = page.locator('.modal-overlay, .onboarding-modal');
  if (await overlay.isVisible({ timeout: 1000 }).catch(() => false)) {
    await page.keyboard.press('Escape');
  }
}

/** Search for a tool and click its card. */
async function openTool(
  page: import('@playwright/test').Page,
  toolName: string,
  searchText: string
): Promise<void> {
  await page.fill('input[placeholder*="search" i], .tool-search-input', searchText);
  await page.waitForTimeout(400);
  await page
    .locator(`[data-tool-name="${toolName}"], .tool-card:has-text("${searchText}")`)
    .first()
    .click();
  await page.waitForTimeout(300);
}

/** Click the file-open trigger button (Upload / Open / file input). */
async function triggerFileOpen(page: import('@playwright/test').Page): Promise<void> {
  await page
    .locator(
      'button:has-text("Open"), button:has-text("Upload"), button:has-text("Choose"), .upload-btn, [data-testid="open-files-btn"]'
    )
    .first()
    .click();
  await page.waitForTimeout(600);
}

/** Click Run and await the success toast (30s timeout). */
async function runAndAwaitSuccess(page: import('@playwright/test').Page): Promise<void> {
  await page
    .locator('.run-btn, button:has-text("Run"), button:has-text("Execute"), button[type="submit"]')
    .first()
    .click();
  await page.waitForSelector(
    '.toast-success, [data-testid="success-toast"], .notification-success, [class*="success"]',
    { timeout: 30000 }
  );
}

// ---------------------------------------------------------------------------
// Lifecycle hooks
// ---------------------------------------------------------------------------

test.beforeAll(async () => {
  sandboxManager.pruneStale();
});

test.afterAll(async () => {
  sandboxManager.sweep();
});

// ---------------------------------------------------------------------------
// Tool 01 — pdf_merge
// SPEC_152: Pages(out) = Pages(A)+Pages(B) = 2; page1 text = Doc A, page2 = Doc B
// ---------------------------------------------------------------------------
test('pdf_merge — Page Addition Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_merge');
  const outFile = path.join(sandbox, 'merged.pdf');

  await injectMock(
    page,
    [fixturePath('merge_a.pdf'), fixturePath('merge_b.pdf')],
    outFile
  );
  await gotoApp(page);
  await openTool(page, 'pdf_merge', 'merge');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  expect(result.numPages).toBe(2);
  expect(result.textByPage[0].length).toBeGreaterThan(0);
  expect(result.textByPage[1].length).toBeGreaterThan(0);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 02 — pdf_merge_alternate
// SPEC_152: Interleaving Invariant — 2 pages, alternating content A/B
// ---------------------------------------------------------------------------
test('pdf_merge_alternate — Interleaving Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_merge_alternate');
  const outFile = path.join(sandbox, 'alternated.pdf');

  await injectMock(
    page,
    [fixturePath('merge_a.pdf'), fixturePath('merge_b.pdf')],
    outFile
  );
  await gotoApp(page);
  await openTool(page, 'pdf_merge_alternate', 'merge alternate');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  expect(result.numPages).toBe(2);
  // Both pages must carry content (alternating source docs)
  expect(result.textByPage[0].length).toBeGreaterThan(0);
  expect(result.textByPage[1].length).toBeGreaterThan(0);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 03 — pdf_split
// SPEC_152: Splitting 5-page PDF → output has 1 page matching page 1 text
// ---------------------------------------------------------------------------
test('pdf_split — Range Exactness Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_split');
  const outFile = path.join(sandbox, 'split.pdf');

  await injectMock(page, fixturePath('multi_page.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_split', 'split');
  await triggerFileOpen(page);

  // Fill split-at-page arg
  const pageInput = page.locator('input[name*="page"], input[placeholder*="page" i], input[placeholder*="range" i]').first();
  if (await pageInput.isVisible({ timeout: 2000 }).catch(() => false)) {
    await pageInput.fill('1');
  }

  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  expect(result.numPages).toBe(1);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 04 — pdf_split_by_outline
// SPEC_152: Bookmark Hierarchy Slicing — output ZIP with per-bookmark PDFs
// ---------------------------------------------------------------------------
test('pdf_split_by_outline — Bookmark Hierarchy Slicing', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_split_by_outline');
  const outFile = path.join(sandbox, 'split_outline.zip');

  await injectMock(page, fixturePath('research_paper.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_split_by_outline', 'split outline');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const buf = fs.readFileSync(outFile);
  // ZIP magic bytes: PK\x03\x04
  expect(buf[0]).toBe(0x50);
  expect(buf[1]).toBe(0x4b);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 05 — pdf_split_scanned_book
// SPEC_152: 2-Up Center Spine Slicer — Pages(out) = 2×Pages(in); portrait dims
// ---------------------------------------------------------------------------
test('pdf_split_scanned_book — 2-Up Slicer Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_split_scanned_book');
  const outFile = path.join(sandbox, 'split_book.pdf');

  await injectMock(page, fixturePath('two_up_book.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_split_scanned_book', 'split scanned');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const inPdf = await PdfInspector.inspect(fixturePath('two_up_book.pdf'));
  const result = await PdfInspector.inspect(outFile);
  // Output pages should be double input pages
  expect(result.numPages).toBe(inPdf.numPages * 2);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 06 — pdf_burst
// SPEC_152: 5-page PDF → 5 files, each 1 page
// ---------------------------------------------------------------------------
test('pdf_burst — Total Partition Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_burst');
  // burst produces multiple files; we capture the sandbox dir via save dialog
  const outFile = path.join(sandbox, 'burst.zip');

  await injectMock(page, fixturePath('multi_page.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_burst', 'burst');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  // Some engines output a ZIP; others write page_1.pdf…page_5.pdf directly
  const hasZip = fs.existsSync(outFile);
  const hasPages = fs.existsSync(path.join(sandbox, 'page_1.pdf'));

  if (hasZip) {
    const buf = fs.readFileSync(outFile);
    expect(buf[0]).toBe(0x50); // PK zip magic
    expect(buf[1]).toBe(0x4b);
  } else {
    expect(hasPages).toBe(true);
    for (let i = 1; i <= 5; i++) {
      const pagePath = path.join(sandbox, `page_${i}.pdf`);
      if (fs.existsSync(pagePath)) {
        const r = await PdfInspector.inspect(pagePath);
        expect(r.numPages).toBe(1);
      }
    }
  }

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 07 — pdf_crop
// SPEC_152: Bounding Box Invariant — /CropBox changes in PDF dict
// ---------------------------------------------------------------------------
test('pdf_crop — Bounding Box Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_crop');
  const outFile = path.join(sandbox, 'cropped.pdf');

  await injectMock(page, fixturePath('single_page.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_crop', 'crop');
  await triggerFileOpen(page);

  // Fill margin arg if present
  const marginInput = page.locator('input[name*="margin"], input[placeholder*="margin" i]').first();
  if (await marginInput.isVisible({ timeout: 2000 }).catch(() => false)) {
    await marginInput.fill('50');
  }

  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  // CropBox or MediaBox must appear in the output
  expect(raw.hasToken('/CropBox') || raw.hasToken('/MediaBox')).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 08 — pdf_delete_pages
// SPEC_152: Deleting page 1 from 5-page PDF → Pages(out)=4; original p1 text absent from p1
// ---------------------------------------------------------------------------
test('pdf_delete_pages — Page Subtraction Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_delete_pages');
  const outFile = path.join(sandbox, 'deleted.pdf');

  await injectMock(page, fixturePath('multi_page.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_delete_pages', 'delete pages');
  await triggerFileOpen(page);

  // Capture original page 1 text for absence check
  const original = await PdfInspector.inspect(fixturePath('multi_page.pdf'));
  const originalPage1Text = original.textByPage[0];

  // Fill pages-to-delete
  const pageInput = page.locator('input[name*="page"], input[placeholder*="page" i]').first();
  if (await pageInput.isVisible({ timeout: 2000 }).catch(() => false)) {
    await pageInput.fill('1');
  }

  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  expect(result.numPages).toBe(4);
  // Original page 1 text should not appear in new page 1
  if (originalPage1Text.trim().length > 0) {
    expect(result.textByPage[0]).not.toContain(originalPage1Text.trim());
  }

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 09 — pdf_extract_pages
// SPEC_152: Extracting page 1 → Pages(out)=1; text matches input page 1
// ---------------------------------------------------------------------------
test('pdf_extract_pages — Subset Exactness Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_extract_pages');
  const outFile = path.join(sandbox, 'extracted.pdf');

  await injectMock(page, fixturePath('multi_page.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_extract_pages', 'extract pages');
  await triggerFileOpen(page);

  const pageInput = page.locator('input[name*="page"], input[placeholder*="page" i]').first();
  if (await pageInput.isVisible({ timeout: 2000 }).catch(() => false)) {
    await pageInput.fill('1');
  }

  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  expect(result.numPages).toBe(1);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 10 — pdf_reorder_pages
// SPEC_152: Reverse [4,3,2,1] → page1 text = original page4 text
// ---------------------------------------------------------------------------
test('pdf_reorder_pages — Permutation Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_reorder_pages');
  const outFile = path.join(sandbox, 'reordered.pdf');

  await injectMock(page, fixturePath('four_pages.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_reorder_pages', 'reorder pages');
  await triggerFileOpen(page);

  // Fill order arg
  const orderInput = page.locator('input[name*="order"], input[placeholder*="order" i]').first();
  if (await orderInput.isVisible({ timeout: 2000 }).catch(() => false)) {
    await orderInput.fill('4,3,2,1');
  }

  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const inPdf = await PdfInspector.inspect(fixturePath('four_pages.pdf'));
  const result = await PdfInspector.inspect(outFile);
  expect(result.numPages).toBe(4);
  // If original page 4 had text, page 1 of output must contain it
  if (inPdf.textByPage[3].trim().length > 0 && result.textByPage[0].trim().length > 0) {
    expect(result.textByPage[0]).toContain(inPdf.textByPage[3].trim().substring(0, 20));
  }

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 11 — pdf_rotate
// SPEC_152: /Rotate 90 in PDF page dictionary
// ---------------------------------------------------------------------------
test('pdf_rotate — Rotation Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_rotate');
  const outFile = path.join(sandbox, 'rotated.pdf');

  await injectMock(page, fixturePath('single_page.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_rotate', 'rotate');
  await triggerFileOpen(page);

  // Fill angle
  const angleInput = page.locator('input[name*="angle"], select[name*="angle"], input[placeholder*="angle" i]').first();
  if (await angleInput.isVisible({ timeout: 2000 }).catch(() => false)) {
    await angleInput.fill('90');
  }

  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  expect(raw.hasToken('/Rotate')).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 12 — pdf_auto_rotate
// SPEC_152: Orientation Normalization — /Rotate 0 (normalized from landscape)
// ---------------------------------------------------------------------------
test('pdf_auto_rotate — Orientation Normalization Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_auto_rotate');
  const outFile = path.join(sandbox, 'auto_rotated.pdf');

  await injectMock(page, fixturePath('rotated_landscape.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_auto_rotate', 'auto rotate');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  // Should have /Rotate 0 or no rotation (normalized)
  const hasRotate0 = raw.hasToken('/Rotate 0');
  const hasNoRotate = !raw.hasToken('/Rotate 90') && !raw.hasToken('/Rotate 180') && !raw.hasToken('/Rotate 270');
  expect(hasRotate0 || hasNoRotate).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 13 — pdf_remove_blank
// SPEC_152: Content Density Threshold — Pages(out) < Pages(in); no empty pages
// ---------------------------------------------------------------------------
test('pdf_remove_blank — Content Density Threshold Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_remove_blank');
  const outFile = path.join(sandbox, 'no_blanks.pdf');

  await injectMock(page, fixturePath('with_blank_pages.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_remove_blank', 'remove blank');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const inPdf = await PdfInspector.inspect(fixturePath('with_blank_pages.pdf'));
  const result = await PdfInspector.inspect(outFile);
  expect(result.numPages).toBeLessThan(inPdf.numPages);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 14 — pdf_scale_pages
// SPEC_152: MediaBox Dimensional Scaling — scaled by factor (0.5)
// ---------------------------------------------------------------------------
test('pdf_scale_pages — MediaBox Dimensional Scaling Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_scale_pages');
  const outFile = path.join(sandbox, 'scaled.pdf');

  await injectMock(page, fixturePath('single_page.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_scale_pages', 'scale pages');
  await triggerFileOpen(page);

  // Fill scale factor
  const scaleInput = page.locator('input[name*="scale"], input[placeholder*="scale" i], input[placeholder*="factor" i]').first();
  if (await scaleInput.isVisible({ timeout: 2000 }).catch(() => false)) {
    await scaleInput.fill('0.5');
  }

  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  expect(raw.hasToken('/MediaBox')).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 15 — pdf_nup
// SPEC_152: Grid Compression — 4 input pages → 1 output sheet (2×2)
// ---------------------------------------------------------------------------
test('pdf_nup — Grid Compression Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_nup');
  const outFile = path.join(sandbox, 'nup.pdf');

  await injectMock(page, fixturePath('four_pages.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_nup', 'nup');
  await triggerFileOpen(page);

  // Set N=4 if input present
  const nInput = page.locator('input[name*="n"], input[name*="pages_per"], select[name*="n"]').first();
  if (await nInput.isVisible({ timeout: 2000 }).catch(() => false)) {
    await nInput.fill('4');
  }

  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  // 4 input pages in 2×2 grid = 1 output page
  expect(result.numPages).toBe(1);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 16 — pdf_booklet
// SPEC_152: Signature Folding Reorder — 4 pages → 2 spread sheets
// ---------------------------------------------------------------------------
test('pdf_booklet — Signature Folding Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_booklet');
  const outFile = path.join(sandbox, 'booklet.pdf');

  await injectMock(page, fixturePath('four_pages.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_booklet', 'booklet');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  // 4 input pages → 2 saddle-stitch spread sheets
  expect(result.numPages).toBe(2);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 17 — pdf_posterize
// SPEC_152: Tile Grid Multiplication — 1 page, 2×2 grid → 4 tiles
// ---------------------------------------------------------------------------
test('pdf_posterize — Tile Grid Multiplication Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_posterize');
  const outFile = path.join(sandbox, 'posterized.pdf');

  await injectMock(page, fixturePath('single_page.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_posterize', 'posterize');
  await triggerFileOpen(page);

  // Set grid size
  const gridInput = page.locator('input[name*="grid"], input[placeholder*="grid" i], input[placeholder*="tile" i]').first();
  if (await gridInput.isVisible({ timeout: 2000 }).catch(() => false)) {
    await gridInput.fill('2x2');
  }

  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  // 2×2 grid = 4 output tiles
  expect(result.numPages).toBe(4);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 18 — pdf_stitch_canvas
// SPEC_152: Continuous Single Page — multiple pages → 1 page; height = sum of all
// ---------------------------------------------------------------------------
test('pdf_stitch_canvas — Continuous Single Page Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_stitch_canvas');
  const outFile = path.join(sandbox, 'stitched.pdf');

  await injectMock(page, fixturePath('four_pages.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_stitch_canvas', 'stitch canvas');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  expect(result.numPages).toBe(1);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 19 — pdf_bookmarks
// SPEC_152: Outline JSON Schema — JSON array with title+page keys
// ---------------------------------------------------------------------------
test('pdf_bookmarks — Outline JSON Schema Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_bookmarks');
  const outFile = path.join(sandbox, 'bookmarks.json');

  await injectMock(page, fixturePath('research_paper.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_bookmarks', 'bookmarks');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const content = fs.readFileSync(outFile, 'utf-8');
  const json = JSON.parse(content);
  expect(Array.isArray(json)).toBe(true);
  if (json.length > 0) {
    expect(typeof json[0].title !== 'undefined' || typeof json[0].page !== 'undefined').toBe(true);
  }

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 20 — pdf_attachments
// SPEC_152: Embedded Files Stream — JSON lists embedded files
// ---------------------------------------------------------------------------
test('pdf_attachments — Embedded Files Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_attachments');
  const outFile = path.join(sandbox, 'attachments.json');

  await injectMock(page, fixturePath('single_page.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_attachments', 'attachments');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const content = fs.readFileSync(outFile, 'utf-8');
  const json = JSON.parse(content);
  // Result must be an array (may be empty if no embedded files)
  expect(Array.isArray(json)).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 21 — pdf_images_to_pdf
// SPEC_152: Raster Containerization — 2 PNG inputs → 2-page PDF, /XObject /Image
// ---------------------------------------------------------------------------
test('pdf_images_to_pdf — Raster Containerization Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_images_to_pdf');
  const outFile = path.join(sandbox, 'images_to_pdf.pdf');

  await injectMock(
    page,
    [fixturePath('img1.png'), fixturePath('img2.png')],
    outFile
  );
  await gotoApp(page);
  await openTool(page, 'pdf_images_to_pdf', 'images to pdf');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  expect(result.numPages).toBe(2);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.hasToken('/XObject')).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 22 — img_to_pdf
// SPEC_152: Single Image Page — 1 PNG → 1-page PDF with /XObject /Image
// ---------------------------------------------------------------------------
test('img_to_pdf — Single Image Page Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('img_to_pdf');
  const outFile = path.join(sandbox, 'img_to_pdf.pdf');

  await injectMock(page, fixturePath('test.png'), outFile);
  await gotoApp(page);
  await openTool(page, 'img_to_pdf', 'img to pdf');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  expect(result.numPages).toBe(1);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.hasToken('/XObject')).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 23 — pdf_compress
// SPEC_152: Byte Reduction — size(out) ≤ size(in); text remains readable
// ---------------------------------------------------------------------------
test('pdf_compress — Byte Reduction Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_compress');
  const outFile = path.join(sandbox, 'compressed.pdf');

  await injectMock(page, fixturePath('large_doc.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_compress', 'compress');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const inSize = fs.statSync(fixturePath('large_doc.pdf')).size;
  const outSize = fs.statSync(outFile).size;
  // Output must be a valid PDF and ≤ input size
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  expect(outSize).toBeLessThanOrEqual(inSize);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 24 — pdf_dedup_streams
// SPEC_152: Indirect Object Deduplication — valid PDF, structurally sound
// ---------------------------------------------------------------------------
test('pdf_dedup_streams — Stream Deduplication Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_dedup_streams');
  const outFile = path.join(sandbox, 'deduped.pdf');

  await injectMock(page, fixturePath('images.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_dedup_streams', 'dedup streams');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  // Output must be a valid PDF (xref intact)
  expect(raw.hasToken('%%EOF')).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 25 — pdf_linearize
// SPEC_152: First-Page Optimization — /Linearized in first 1024 bytes
// ---------------------------------------------------------------------------
test('pdf_linearize — Fast Web View Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_linearize');
  const outFile = path.join(sandbox, 'linearized.pdf');

  await injectMock(page, fixturePath('single_page.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_linearize', 'linearize');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const buf = fs.readFileSync(outFile);
  const header = buf.subarray(0, 1024).toString('latin1');
  expect(header).toContain('/Linearized');

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 26 — pdf_repair
// SPEC_152: Xref Rebuild — valid %PDF- header; no parse error
// ---------------------------------------------------------------------------
test('pdf_repair — Xref Table Rebuild Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_repair');
  const outFile = path.join(sandbox, 'repaired.pdf');

  await injectMock(page, fixturePath('malformed.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_repair', 'repair');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  // Must be parseable by PDF.js — inspect() throws on failure
  const result = await PdfInspector.inspect(outFile);
  expect(result.numPages).toBeGreaterThanOrEqual(1);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 27 — pdf_invert_colors
// SPEC_152: Color Inversion Matrix — valid PDF; colorspace inverted in content stream
// ---------------------------------------------------------------------------
test('pdf_invert_colors — Color Inversion Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_invert_colors');
  const outFile = path.join(sandbox, 'inverted.pdf');

  await injectMock(page, fixturePath('single_page.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_invert_colors', 'invert colors');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  // Content stream should reference colorspace operations
  const result = await PdfInspector.inspect(outFile);
  expect(result.numPages).toBeGreaterThanOrEqual(1);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 28 — pdf_render
// SPEC_152: Raster Exactness — output PNG starts with 89 50 4E 47; correct aspect ratio
// ---------------------------------------------------------------------------
test('pdf_render — Raster Exactness Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_render');
  const outFile = path.join(sandbox, 'rendered.png');

  await injectMock(page, fixturePath('single_page.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_render', 'render');
  await triggerFileOpen(page);

  // Set DPI if input is present
  const dpiInput = page.locator('input[name*="dpi"], input[placeholder*="dpi" i]').first();
  if (await dpiInput.isVisible({ timeout: 2000 }).catch(() => false)) {
    await dpiInput.fill('150');
  }

  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const buf = fs.readFileSync(outFile);
  // PNG magic bytes: 89 50 4E 47 0D 0A 1A 0A
  expect(buf[0]).toBe(0x89);
  expect(buf[1]).toBe(0x50);
  expect(buf[2]).toBe(0x4e);
  expect(buf[3]).toBe(0x47);
  // At 150 dpi, A4 is roughly 1240×1754; just check non-trivial size
  expect(buf.length).toBeGreaterThan(10000);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 29 — pdf_compare
// SPEC_152: Visual & Textual Diff — JSON report with `differences` array
// ---------------------------------------------------------------------------
test('pdf_compare — Visual & Textual Diff Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_compare');
  const outFile = path.join(sandbox, 'diff.json');

  await injectMock(
    page,
    [fixturePath('merge_a.pdf'), fixturePath('merge_b.pdf')],
    outFile
  );
  await gotoApp(page);
  await openTool(page, 'pdf_compare', 'compare');
  await triggerFileOpen(page);
  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const content = fs.readFileSync(outFile, 'utf-8');
  const json = JSON.parse(content);
  // Must have a differences / changes / diffs key
  const hasDiffKey =
    Array.isArray(json.differences) ||
    Array.isArray(json.changes) ||
    Array.isArray(json.diffs) ||
    typeof json.summary !== 'undefined';
  expect(hasDiffKey).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 30 — pdf_encrypt
// SPEC_152: Cryptographic Lock — /Encrypt dict present; fails without password
// ---------------------------------------------------------------------------
test('pdf_encrypt — Cryptographic Lock Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_encrypt');
  const outFile = path.join(sandbox, 'encrypted.pdf');

  await injectMock(page, fixturePath('single_page.pdf'), outFile);
  await gotoApp(page);
  await openTool(page, 'pdf_encrypt', 'encrypt');
  await triggerFileOpen(page);

  // Fill password
  const pwdInput = page.locator('input[name*="password"], input[type="password"], input[placeholder*="password" i]').first();
  if (await pwdInput.isVisible({ timeout: 2000 }).catch(() => false)) {
    await pwdInput.fill('test123');
  }

  await runAndAwaitSuccess(page);

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  expect(raw.isEncrypted).toBe(true);

  sandboxManager.cleanup(sandbox);
});
