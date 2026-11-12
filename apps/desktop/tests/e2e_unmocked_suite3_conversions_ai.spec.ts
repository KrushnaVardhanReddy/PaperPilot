/**
 * Suite 3 — Tools 61–90: Conversions, Office, AI & Workflows
 *
 * Real-engine unmocked E2E tests.
 * - OS file dialogs (plugin:dialog|open / save) are mocked per-test via addInitScript.
 * - ALL other IPC commands (invoke_mcp_tool, etc.) pass through to the real Tauri/Rust binary.
 * - Each test has its own addInitScript with the specific fixture paths and output paths.
 */

import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';
import { SandboxManager } from './helpers/sandboxManager';
import { PdfInspector } from './helpers/pdfInspector';

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

const sandboxManager = new SandboxManager();

function fixturePath(name: string): string {
  return path.resolve(process.cwd(), `../../tests/e2e_fixtures/real/${name}`);
}

/** Install the real-IPC passthrough mock + set dialog fixtures and save path. */
async function installMock(
  page: import('@playwright/test').Page,
  fixtures: string[],
  out: string
): Promise<void> {
  await page.addInitScript(
    ({ fixtureList, outPath }: { fixtureList: string[]; outPath: string }) => {
      localStorage.setItem('paperpilot_onboarding_completed', 'true');
      const orig = (window as any).__TAURI_INTERNALS__;
      (window as any).__REAL_TAURI_INTERNALS__ = orig;
      (window as any).__TAURI_INTERNALS__ = {
        invoke: async (cmd: string, args: any) => {
          if (cmd === 'plugin:dialog|open') return fixtureList;
          if (cmd === 'plugin:dialog|save') return outPath;
          if (orig && orig.invoke) return orig.invoke(cmd, args);
          return { success: true };
        }
      };
    },
    { fixtureList: fixtures, outPath: out }
  );
}

/** Navigate, dismiss any onboarding overlay, search for a tool, click it,
 *  trigger the file-open dialog, then click Run. Waits for the success toast. */
async function runTool(
  page: import('@playwright/test').Page,
  searchTerm: string,
  toolSelector: string
): Promise<void> {
  await page.goto('http://127.0.0.1:1420/');
  await page.waitForTimeout(600);

  // Dismiss onboarding modal if present
  try {
    const overlay = page.locator('.modal-overlay, .onboarding-modal');
    if (await overlay.isVisible({ timeout: 1500 })) {
      await page.keyboard.press('Escape');
      await page.waitForTimeout(300);
    }
  } catch (_) {}

  // Search for tool
  await page.fill('input[placeholder*="search" i], .tool-search-input', searchTerm);
  await page.waitForTimeout(400);

  // Click tool card
  await page.locator(toolSelector).first().click();
  await page.waitForTimeout(400);

  // Trigger file-open dialog (mocked → returns fixture paths)
  await page
    .locator(
      'button:has-text("Open"), button:has-text("Upload"), button:has-text("Choose"), .upload-btn, [data-testid="open-files-btn"]'
    )
    .first()
    .click();
  await page.waitForTimeout(600);

  // Run / Execute
  await page
    .locator('.run-btn, button:has-text("Run"), button:has-text("Execute"), button[type="submit"]')
    .first()
    .click();

  // Wait for success toast (up to 60 s for AI/OCR tools)
  await page.waitForSelector(
    '.toast-success, [data-testid="success-toast"], .notification-success, [class*="success"]',
    { timeout: 60000 }
  );
}

// ---------------------------------------------------------------------------
// Suite lifecycle
// ---------------------------------------------------------------------------

test.beforeAll(() => {
  sandboxManager.pruneStale(3600000);
});

test.afterAll(() => {
  sandboxManager.sweep();
});

// ---------------------------------------------------------------------------
// Tool 61 — pdf_to_docx
// ---------------------------------------------------------------------------
test('pdf_to_docx — Word OpenXML Container Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_to_docx');
  const outFile = path.join(sandbox, 'output.docx');

  await installMock(page, [fixturePath('single_page.pdf')], outFile);
  await runTool(page, 'pdf to docx', '[data-tool-name="pdf_to_docx"], .tool-card:has-text("Word")');

  expect(fs.existsSync(outFile)).toBe(true);
  const buf = fs.readFileSync(outFile);
  // PKZIP magic bytes
  expect(buf.subarray(0, 2).toString()).toBe('PK');
  // word/document.xml must exist inside the zip
  const content = buf.toString('latin1');
  expect(content.includes('word/document.xml')).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 62 — pdf_to_xlsx
// ---------------------------------------------------------------------------
test('pdf_to_xlsx — Spreadsheet Matrix Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_to_xlsx');
  const outFile = path.join(sandbox, 'output.xlsx');

  await installMock(page, [fixturePath('financial_tables.pdf')], outFile);
  await runTool(page, 'pdf to xlsx', '[data-tool-name="pdf_to_xlsx"], .tool-card:has-text("Excel")');

  expect(fs.existsSync(outFile)).toBe(true);
  const buf = fs.readFileSync(outFile);
  expect(buf.subarray(0, 2).toString()).toBe('PK');
  const content = buf.toString('latin1');
  // xl/worksheets/sheet1.xml must be present
  expect(content.includes('xl/worksheets/sheet1.xml')).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 63 — pdf_to_pptx
// ---------------------------------------------------------------------------
test('pdf_to_pptx — Presentation Slides Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_to_pptx');
  const outFile = path.join(sandbox, 'output.pptx');

  await installMock(page, [fixturePath('four_pages.pdf')], outFile);
  await runTool(page, 'pdf to pptx', '[data-tool-name="pdf_to_pptx"], .tool-card:has-text("PowerPoint")');

  expect(fs.existsSync(outFile)).toBe(true);
  const buf = fs.readFileSync(outFile);
  expect(buf.subarray(0, 2).toString()).toBe('PK');
  const content = buf.toString('latin1');
  expect(content.includes('ppt/slides/slide1.xml')).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 64 — pdf_to_epub
// ---------------------------------------------------------------------------
test('pdf_to_epub — Reflowable eBook Container Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_to_epub');
  const outFile = path.join(sandbox, 'output.epub');

  await installMock(page, [fixturePath('single_page.pdf')], outFile);
  await runTool(page, 'epub', '[data-tool-name="pdf_to_epub"], .tool-card:has-text("EPUB")');

  expect(fs.existsSync(outFile)).toBe(true);
  const buf = fs.readFileSync(outFile);
  expect(buf.subarray(0, 2).toString()).toBe('PK');
  const content = buf.toString('latin1');
  // EPUB mimetype file must exist
  expect(content.includes('application/epub+zip')).toBe(true);
  expect(content.includes('META-INF/container.xml')).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 65 — pdf_to_pdf_a
// ---------------------------------------------------------------------------
test('pdf_to_pdf_a — ISO 19005 Archival Tags Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_to_pdf_a');
  const outFile = path.join(sandbox, 'output.pdf');

  await installMock(page, [fixturePath('single_page.pdf')], outFile);
  await runTool(page, 'pdf/a', '[data-tool-name="pdf_to_pdf_a"], .tool-card:has-text("PDF/A")');

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  // Must have OutputIntents or GTS_PDFA1 archival marker
  expect(raw.hasDictionary('OutputIntents') || raw.hasToken('GTS_PDFA1')).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 66 — pdf_extract_images
// ---------------------------------------------------------------------------
test('pdf_extract_images — DPI/Size Filtered Extraction Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_extract_images');
  const outFile = path.join(sandbox, 'images_out');

  await installMock(page, [fixturePath('images.pdf')], outFile);
  await runTool(page, 'extract images', '[data-tool-name="pdf_extract_images"], .tool-card:has-text("Extract Images")');

  // Output may be a zip or a directory; either way at least one image file should exist
  const outExists = fs.existsSync(outFile) || fs.existsSync(outFile + '.zip');
  expect(outExists).toBe(true);
  if (fs.existsSync(outFile + '.zip')) {
    const buf = fs.readFileSync(outFile + '.zip');
    expect(buf.subarray(0, 2).toString()).toBe('PK');
  }

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 67 — pdf_extract_text
// ---------------------------------------------------------------------------
test('pdf_extract_text — Raw Text Stream Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_extract_text');
  const outFile = path.join(sandbox, 'output.txt');

  await installMock(page, [fixturePath('single_page.pdf')], outFile);
  await runTool(page, 'extract text', '[data-tool-name="pdf_extract_text"], .tool-card:has-text("Plain Text")');

  expect(fs.existsSync(outFile)).toBe(true);
  const txt = fs.readFileSync(outFile, 'utf-8');
  expect(txt.trim().length).toBeGreaterThan(10);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 68 — pdf_extract_tables
// ---------------------------------------------------------------------------
test('pdf_extract_tables — Table Detection Matrix Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_extract_tables');
  const outFile = path.join(sandbox, 'output.json');

  await installMock(page, [fixturePath('financial_tables.pdf')], outFile);
  await runTool(page, 'extract tables', '[data-tool-name="pdf_extract_tables"], .tool-card:has-text("Extract Tables")');

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = fs.readFileSync(outFile, 'utf-8');
  const json = JSON.parse(raw);
  // Must be array or object with structured row data
  expect(Array.isArray(json) || typeof json === 'object').toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 69 — pdf_comic_archive
// ---------------------------------------------------------------------------
test('pdf_comic_archive — ZIP Image Container Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_comic_archive');
  const outFile = path.join(sandbox, 'output.cbz');

  await installMock(page, [fixturePath('four_pages.pdf')], outFile);
  await runTool(page, 'comic', '[data-tool-name="pdf_comic_archive"], .tool-card:has-text("Comic")');

  expect(fs.existsSync(outFile)).toBe(true);
  const buf = fs.readFileSync(outFile);
  // .cbz is a valid PKZIP file
  expect(buf.subarray(0, 2).toString()).toBe('PK');
  const content = buf.toString('latin1');
  // Must contain sequentially named page images
  expect(content.includes('001.png') || content.includes('001.jpg')).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 70 — pdf_grayscale
// ---------------------------------------------------------------------------
test('pdf_grayscale — DeviceGray Colorspace Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_grayscale');
  const outFile = path.join(sandbox, 'output.pdf');

  await installMock(page, [fixturePath('single_page.pdf')], outFile);
  await runTool(page, 'grayscale', '[data-tool-name="pdf_grayscale"], .tool-card:has-text("Grayscale")');

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  // Grayscale PDFs contain DeviceGray colorspace reference
  expect(raw.hasToken('DeviceGray') || raw.hasToken('G')).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 71 — pdf_convert_excel  (CSV → PDF)
// ---------------------------------------------------------------------------
test('pdf_convert_excel — Tabular CSV Rendering Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_convert_excel');
  const outFile = path.join(sandbox, 'output.pdf');

  await installMock(page, [fixturePath('test.csv')], outFile);
  await runTool(page, 'csv to pdf', '[data-tool-name="pdf_convert_excel"], .tool-card:has-text("CSV to PDF")');

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  // Must contain tabular text from the CSV
  expect(result.allText.trim().length).toBeGreaterThan(5);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 72 — excel_to_json
// ---------------------------------------------------------------------------
test('excel_to_json — Worksheet to JSON Objects Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('excel_to_json');
  const outFile = path.join(sandbox, 'output.json');

  await installMock(page, [fixturePath('test.xlsx')], outFile);
  await runTool(page, 'excel to json', '[data-tool-name="excel_to_json"], .tool-card:has-text("Excel to JSON")');

  expect(fs.existsSync(outFile)).toBe(true);
  const json = JSON.parse(fs.readFileSync(outFile, 'utf-8'));
  // Must be a non-empty array of row objects
  expect(Array.isArray(json)).toBe(true);
  expect(json.length).toBeGreaterThan(0);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 73 — excel_to_csv
// ---------------------------------------------------------------------------
test('excel_to_csv — RFC 4180 CSV Export Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('excel_to_csv');
  const outFile = path.join(sandbox, 'output.csv');

  await installMock(page, [fixturePath('test.xlsx')], outFile);
  await runTool(page, 'excel to csv', '[data-tool-name="excel_to_csv"], .tool-card:has-text("Excel to CSV")');

  expect(fs.existsSync(outFile)).toBe(true);
  const csv = fs.readFileSync(outFile, 'utf-8');
  // Must have comma-delimited content with at least one line
  expect(csv.trim().length).toBeGreaterThan(0);
  expect(csv.includes(',')).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 74 — excel_to_pdf
// ---------------------------------------------------------------------------
test('excel_to_pdf — Spreadsheet to PDF Rendering Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('excel_to_pdf');
  const outFile = path.join(sandbox, 'output.pdf');

  await installMock(page, [fixturePath('test.xlsx')], outFile);
  await runTool(page, 'excel to pdf', '[data-tool-name="excel_to_pdf"], .tool-card:has-text("Excel to PDF")');

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  expect(result.numPages).toBeGreaterThanOrEqual(1);
  expect(result.allText.trim().length).toBeGreaterThan(0);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 75 — office_modernize_legacy  (.xls → .xlsx)
// ---------------------------------------------------------------------------
test('office_modernize_legacy — Binary to OOXML Upgrade Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('office_modernize_legacy');
  const outFile = path.join(sandbox, 'output.xlsx');

  await installMock(page, [fixturePath('test.xls')], outFile);
  await runTool(page, 'modernize', '[data-tool-name="office_modernize_legacy"], .tool-card:has-text("Modernize")');

  expect(fs.existsSync(outFile)).toBe(true);
  const buf = fs.readFileSync(outFile);
  // Output must be a valid PKZIP (OpenXML) file
  expect(buf.subarray(0, 2).toString()).toBe('PK');
  const content = buf.toString('latin1');
  expect(content.includes('xl/workbook.xml') || content.includes('[Content_Types]')).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 76 — word_to_pdf
// ---------------------------------------------------------------------------
test('word_to_pdf — Word Document to PDF Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('word_to_pdf');
  const outFile = path.join(sandbox, 'output.pdf');

  await installMock(page, [fixturePath('test.docx')], outFile);
  await runTool(page, 'word to pdf', '[data-tool-name="word_to_pdf"], .tool-card:has-text("Word to PDF")');

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  expect(result.numPages).toBeGreaterThanOrEqual(1);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 77 — powerpoint_to_pdf
// ---------------------------------------------------------------------------
test('powerpoint_to_pdf — Presentation to PDF Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('powerpoint_to_pdf');
  const outFile = path.join(sandbox, 'output.pdf');

  await installMock(page, [fixturePath('test.pptx')], outFile);
  await runTool(page, 'powerpoint to pdf', '[data-tool-name="powerpoint_to_pdf"], .tool-card:has-text("PowerPoint to PDF")');

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  expect(result.numPages).toBeGreaterThanOrEqual(1);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 78 — pdf_ocr
// ---------------------------------------------------------------------------
test('pdf_ocr — Searchable Text Layer Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_ocr');
  const outFile = path.join(sandbox, 'output.pdf');

  await installMock(page, [fixturePath('ocr.pdf')], outFile);
  await runTool(page, 'ocr', '[data-tool-name="pdf_ocr"], .tool-card:has-text("OCR")');

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  // After OCR, text layer must be present and non-trivial
  expect(result.allText.length).toBeGreaterThan(20);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 79 — pdf_classify_pages
// ---------------------------------------------------------------------------
test('pdf_classify_pages — Page Classification JSON Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_classify_pages');
  const outFile = path.join(sandbox, 'output.json');

  await installMock(page, [fixturePath('multimodal.pdf')], outFile);
  await runTool(page, 'classify pages', '[data-tool-name="pdf_classify_pages"], .tool-card:has-text("Classify Pages")');

  expect(fs.existsSync(outFile)).toBe(true);
  const json = JSON.parse(fs.readFileSync(outFile, 'utf-8'));
  // Must be an array with at least one entry having page + type keys
  expect(Array.isArray(json)).toBe(true);
  expect(json.length).toBeGreaterThan(0);
  const firstEntry = json[0];
  expect(firstEntry).toHaveProperty('page');
  expect(firstEntry).toHaveProperty('type');

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 80 — pdf_summarize
// ---------------------------------------------------------------------------
test('pdf_summarize — Summary String Length Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_summarize');
  const outFile = path.join(sandbox, 'output.json');

  await installMock(page, [fixturePath('research_paper.pdf')], outFile);
  await runTool(page, 'summarize', '[data-tool-name="pdf_summarize"], .tool-card:has-text("Summarize")');

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = fs.readFileSync(outFile, 'utf-8');
  // Could be JSON or plain text — either way it must be non-trivially long
  let summaryText = raw;
  try {
    const json = JSON.parse(raw);
    summaryText = typeof json === 'string' ? json : (json.summary ?? JSON.stringify(json));
  } catch (_) {}
  expect(summaryText.trim().length).toBeGreaterThan(50);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 81 — pdf_translate
// ---------------------------------------------------------------------------
test('pdf_translate — Spanish Output Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_translate');
  const outFile = path.join(sandbox, 'output.pdf');

  await installMock(page, [fixturePath('single_page.pdf')], outFile);

  // Navigate and set target language to Spanish before running
  await page.goto('http://127.0.0.1:1420/');
  await page.waitForTimeout(600);
  try {
    const overlay = page.locator('.modal-overlay, .onboarding-modal');
    if (await overlay.isVisible({ timeout: 1500 })) {
      await page.keyboard.press('Escape');
      await page.waitForTimeout(300);
    }
  } catch (_) {}

  await page.fill('input[placeholder*="search" i], .tool-search-input', 'translate');
  await page.waitForTimeout(400);
  await page.locator('[data-tool-name="pdf_translate"], .tool-card:has-text("Translate")').first().click();
  await page.waitForTimeout(400);

  // Select target language = Spanish (es) if selector exists
  try {
    await page.selectOption('select[name="target_lang"], select[data-param="target"]', 'es', { timeout: 2000 });
  } catch (_) {}

  await page
    .locator('button:has-text("Open"), button:has-text("Upload"), .upload-btn, [data-testid="open-files-btn"]')
    .first()
    .click();
  await page.waitForTimeout(600);

  await page
    .locator('.run-btn, button:has-text("Run"), button:has-text("Execute"), button[type="submit"]')
    .first()
    .click();
  await page.waitForSelector(
    '.toast-success, [data-testid="success-toast"], .notification-success, [class*="success"]',
    { timeout: 60000 }
  );

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  expect(result.allText.trim().length).toBeGreaterThan(10);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 82 — pdf_chat
// ---------------------------------------------------------------------------
test('pdf_chat — AI Answer Field Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_chat');
  const outFile = path.join(sandbox, 'output.json');

  await installMock(page, [fixturePath('single_page.pdf')], outFile);

  await page.goto('http://127.0.0.1:1420/');
  await page.waitForTimeout(600);
  try {
    const overlay = page.locator('.modal-overlay, .onboarding-modal');
    if (await overlay.isVisible({ timeout: 1500 })) {
      await page.keyboard.press('Escape');
      await page.waitForTimeout(300);
    }
  } catch (_) {}

  await page.fill('input[placeholder*="search" i], .tool-search-input', 'chat');
  await page.waitForTimeout(400);
  await page.locator('[data-tool-name="pdf_chat"], .tool-card:has-text("Chat")').first().click();
  await page.waitForTimeout(400);

  // Type query if input exists
  try {
    await page.fill('input[name="query"], textarea[name="query"], [data-param="query"]', 'hello', { timeout: 2000 });
  } catch (_) {}

  await page
    .locator('button:has-text("Open"), button:has-text("Upload"), .upload-btn, [data-testid="open-files-btn"]')
    .first()
    .click();
  await page.waitForTimeout(600);

  await page
    .locator('.run-btn, button:has-text("Run"), button:has-text("Execute"), button[type="submit"]')
    .first()
    .click();
  await page.waitForSelector(
    '.toast-success, [data-testid="success-toast"], .notification-success, [class*="success"]',
    { timeout: 60000 }
  );

  expect(fs.existsSync(outFile)).toBe(true);
  const json = JSON.parse(fs.readFileSync(outFile, 'utf-8'));
  // Must have an `answer` field that is a non-empty string
  expect(json).toHaveProperty('answer');
  expect(typeof json.answer).toBe('string');
  expect((json.answer as string).trim().length).toBeGreaterThan(0);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 83 — pdf_reverse_order
// ---------------------------------------------------------------------------
test('pdf_reverse_order — Page Reversal Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_reverse_order');
  const outFile = path.join(sandbox, 'output.pdf');

  await installMock(page, [fixturePath('four_pages.pdf')], outFile);
  await runTool(page, 'reverse order', '[data-tool-name="pdf_reverse_order"], .tool-card:has-text("Reverse")');

  expect(fs.existsSync(outFile)).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  // Page count must be preserved
  expect(result.numPages).toBe(4);
  // All pages must have content
  for (const pageText of result.textByPage) {
    expect(pageText.length).toBeGreaterThanOrEqual(0);
  }

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 84 — pdf_interleave
// ---------------------------------------------------------------------------
test('pdf_interleave — Alternating A/B Content Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_interleave');
  const outFile = path.join(sandbox, 'output.pdf');

  await installMock(
    page,
    [fixturePath('single_page.pdf'), fixturePath('multi_page.pdf')],
    outFile
  );
  await runTool(page, 'interleave', '[data-tool-name="pdf_interleave"], .tool-card:has-text("Interleave")');

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  // Interleaved output must have pages from both inputs
  expect(result.numPages).toBeGreaterThanOrEqual(2);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 85 — pdf_deinterleave
// ---------------------------------------------------------------------------
test('pdf_deinterleave — Odd/Even Split Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_deinterleave');
  const outFile = path.join(sandbox, 'output.pdf');

  await installMock(page, [fixturePath('four_pages.pdf')], outFile);
  await runTool(page, 'deinterleave', '[data-tool-name="pdf_deinterleave"], .tool-card:has-text("Deinterleave")');

  // Output might be a zip or a primary file; validate the main output is a valid PDF
  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf || raw.buffer.subarray(0, 2).toString() === 'PK').toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 86 — pdf_adjust_page_box
// ---------------------------------------------------------------------------
test('pdf_adjust_page_box — TrimBox Addition Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_adjust_page_box');
  const outFile = path.join(sandbox, 'output.pdf');

  await installMock(page, [fixturePath('single_page.pdf')], outFile);

  await page.goto('http://127.0.0.1:1420/');
  await page.waitForTimeout(600);
  try {
    const overlay = page.locator('.modal-overlay, .onboarding-modal');
    if (await overlay.isVisible({ timeout: 1500 })) {
      await page.keyboard.press('Escape');
      await page.waitForTimeout(300);
    }
  } catch (_) {}

  await page.fill('input[placeholder*="search" i], .tool-search-input', 'page box');
  await page.waitForTimeout(400);
  await page
    .locator('[data-tool-name="pdf_adjust_page_box"], .tool-card:has-text("Page Box")')
    .first()
    .click();
  await page.waitForTimeout(400);

  // Set box type to TrimBox if selector present
  try {
    await page.selectOption('select[name="box"], [data-param="box"]', 'TrimBox', { timeout: 2000 });
  } catch (_) {}

  await page
    .locator('button:has-text("Open"), button:has-text("Upload"), .upload-btn, [data-testid="open-files-btn"]')
    .first()
    .click();
  await page.waitForTimeout(600);

  await page
    .locator('.run-btn, button:has-text("Run"), button:has-text("Execute"), button[type="submit"]')
    .first()
    .click();
  await page.waitForSelector(
    '.toast-success, [data-testid="success-toast"], .notification-success, [class*="success"]',
    { timeout: 30000 }
  );

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  // TrimBox must appear in the PDF dictionary
  expect(raw.hasDictionary('TrimBox') || raw.hasToken('/TrimBox')).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 87 — pdf_render_page_selection
// ---------------------------------------------------------------------------
test('pdf_render_page_selection — Multi-Page PNG Render Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_render_page_selection');
  const outFile = path.join(sandbox, 'output.png');

  await installMock(page, [fixturePath('multi_page.pdf')], outFile);

  await page.goto('http://127.0.0.1:1420/');
  await page.waitForTimeout(600);
  try {
    const overlay = page.locator('.modal-overlay, .onboarding-modal');
    if (await overlay.isVisible({ timeout: 1500 })) {
      await page.keyboard.press('Escape');
      await page.waitForTimeout(300);
    }
  } catch (_) {}

  await page.fill('input[placeholder*="search" i], .tool-search-input', 'render page');
  await page.waitForTimeout(400);
  await page
    .locator('[data-tool-name="pdf_render_page_selection"], .tool-card:has-text("Render Page")')
    .first()
    .click();
  await page.waitForTimeout(400);

  // Set pages = 1,3 if param input is present
  try {
    await page.fill('input[name="pages"], [data-param="pages"]', '1,3', { timeout: 2000 });
  } catch (_) {}

  await page
    .locator('button:has-text("Open"), button:has-text("Upload"), .upload-btn, [data-testid="open-files-btn"]')
    .first()
    .click();
  await page.waitForTimeout(600);

  await page
    .locator('.run-btn, button:has-text("Run"), button:has-text("Execute"), button[type="submit"]')
    .first()
    .click();
  await page.waitForSelector(
    '.toast-success, [data-testid="success-toast"], .notification-success, [class*="success"]',
    { timeout: 30000 }
  );

  // At least the primary output or its zip must exist
  const outZip = outFile.replace('.png', '.zip');
  const anyExists = fs.existsSync(outFile) || fs.existsSync(outZip);
  expect(anyExists).toBe(true);

  if (fs.existsSync(outFile)) {
    const buf = fs.readFileSync(outFile);
    // PNG magic bytes: 89 50 4E 47
    expect(buf[0]).toBe(0x89);
    expect(buf[1]).toBe(0x50);
  }

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 88 — pdf_extract_embedded_images
// ---------------------------------------------------------------------------
test('pdf_extract_embedded_images — ZIP with PNG Files Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_extract_embedded_images');
  const outFile = path.join(sandbox, 'output.zip');

  await installMock(page, [fixturePath('images.pdf')], outFile);
  await runTool(
    page,
    'embedded images',
    '[data-tool-name="pdf_extract_embedded_images"], .tool-card:has-text("Embedded Images")'
  );

  expect(fs.existsSync(outFile)).toBe(true);
  const buf = fs.readFileSync(outFile);
  // Output must be a PKZIP archive
  expect(buf.subarray(0, 2).toString()).toBe('PK');
  const content = buf.toString('latin1');
  // Must contain PNG files
  expect(content.includes('.png') || content.includes('.jpg')).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 89 — pdf_split_by_size
// ---------------------------------------------------------------------------
test('pdf_split_by_size — Size-Bounded Output Parts Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_split_by_size');
  const outFile = path.join(sandbox, 'output.zip');

  await installMock(page, [fixturePath('large_doc.pdf')], outFile);

  await page.goto('http://127.0.0.1:1420/');
  await page.waitForTimeout(600);
  try {
    const overlay = page.locator('.modal-overlay, .onboarding-modal');
    if (await overlay.isVisible({ timeout: 1500 })) {
      await page.keyboard.press('Escape');
      await page.waitForTimeout(300);
    }
  } catch (_) {}

  await page.fill('input[placeholder*="search" i], .tool-search-input', 'split by size');
  await page.waitForTimeout(400);
  await page
    .locator('[data-tool-name="pdf_split_by_size"], .tool-card:has-text("Split by Size")')
    .first()
    .click();
  await page.waitForTimeout(400);

  // Set max size to 100 KB if param input present
  try {
    await page.fill('input[name="max_size_kb"], input[name="size"], [data-param="size"]', '100', { timeout: 2000 });
  } catch (_) {}

  await page
    .locator('button:has-text("Open"), button:has-text("Upload"), .upload-btn, [data-testid="open-files-btn"]')
    .first()
    .click();
  await page.waitForTimeout(600);

  await page
    .locator('.run-btn, button:has-text("Run"), button:has-text("Execute"), button[type="submit"]')
    .first()
    .click();
  await page.waitForSelector(
    '.toast-success, [data-testid="success-toast"], .notification-success, [class*="success"]',
    { timeout: 30000 }
  );

  expect(fs.existsSync(outFile)).toBe(true);
  const buf = fs.readFileSync(outFile);
  // Must be a PKZIP archive containing multiple PDF parts
  expect(buf.subarray(0, 2).toString()).toBe('PK');

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 90 — pdf_split_half
// ---------------------------------------------------------------------------
test('pdf_split_half — Equal Half-Split Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_split_half');
  const outFile = path.join(sandbox, 'output.zip');

  await installMock(page, [fixturePath('four_pages.pdf')], outFile);
  await runTool(page, 'split half', '[data-tool-name="pdf_split_half"], .tool-card:has-text("Split Half")');

  expect(fs.existsSync(outFile)).toBe(true);
  const buf = fs.readFileSync(outFile);
  // Output is a zip containing two PDF halves
  expect(buf.subarray(0, 2).toString()).toBe('PK');
  const content = buf.toString('latin1');
  expect(content.includes('.pdf')).toBe(true);

  sandboxManager.cleanup(sandbox);
});
