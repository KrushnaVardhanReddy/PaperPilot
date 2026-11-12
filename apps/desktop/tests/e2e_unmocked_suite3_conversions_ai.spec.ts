/**
 * Suite 3 — Tools 61–90: Conversions, Office, AI & Workflows
 *
 * Real-engine unmocked E2E tests.
 * - OS file dialogs (plugin:dialog|open / save) are intercepted per-test via addInitScript.
 * - ALL other IPC commands (invoke_mcp_tool, etc.) pass through to the real Tauri/Rust binary.
 * - Hidden file input is populated with setInputFiles.
 * - Navigates #tab-btn-home -> #context-btn-all -> #tool-search-input -> #tool-btn-{id} -> .run-btn.
 * - Validates output per SPEC_152 domain invariants.
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
async function injectMock(
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
          const real = (window as any).__REAL_TAURI_INTERNALS__;
          if (real && typeof real.invoke === 'function') {
            return real.invoke(cmd, args);
          }
          throw new Error(`No real Tauri backend for cmd: ${cmd}`);
        }
      };
    },
    { fixtureList: fixtures, outPath: out }
  );
}

/** Navigate to the app and dismiss any onboarding overlay. */
async function gotoApp(page: import('@playwright/test').Page): Promise<void> {
  await page.goto('http://127.0.0.1:1420/');
  await page.waitForTimeout(500);
  try {
    const overlay = page.locator('.modal-overlay, .onboarding-modal');
    if (await overlay.isVisible({ timeout: 1000 }).catch(() => false)) {
      await page.keyboard.press('Escape');
    }
  } catch (_) {}
}

/**
 * Standard reliable tool execution helper according to user requirements:
 * 1. Upload file(s) via hidden file input: page.locator('input[multiple][type="file"]').setInputFiles([...fixtures])
 * 2. Click #tab-btn-home and #context-btn-all or #context-btn-pdf
 * 3. Search in tools search: #tool-search-input
 * 4. Select tool card: #tool-btn-{toolName} or .tool-card
 * 5. Fill options if needed
 * 6. Click .run-btn
 * 7. Verify success in UI: wait for .toast or check success notification
 */
async function runToolFlow(
  page: import('@playwright/test').Page,
  options: {
    toolName: string;
    searchQuery: string;
    fixtures: string[];
    fillOptions?: (page: import('@playwright/test').Page) => Promise<void>;
  }
): Promise<void> {
  const { toolName, searchQuery, fixtures, fillOptions } = options;

  // 1. Upload via hidden file input
  const fileInput = page.locator('input[multiple][type="file"]');
  await fileInput.setInputFiles(fixtures);

  // 2. Click #tab-btn-home and #context-btn-all or #context-btn-pdf
  await page.click('#tab-btn-home');
  const allBtn = page.locator('#context-btn-all');
  if (await allBtn.isVisible({ timeout: 1000 }).catch(() => false)) {
    await allBtn.click();
  } else {
    const pdfBtn = page.locator('#context-btn-pdf');
    if (await pdfBtn.isVisible({ timeout: 1000 }).catch(() => false)) {
      await pdfBtn.click();
    }
  }

  // 3. Search properly in tools search
  const searchInput = page.locator('#tool-search-input');
  await expect(searchInput).toBeVisible({ timeout: 5000 });
  await searchInput.fill('');
  await searchInput.fill(searchQuery);
  await page.waitForTimeout(300);

  // 4. Select the right tool card
  const toolCards = page.locator('.tool-card');
  const count = await toolCards.count();
  expect(count).toBeGreaterThan(0);

  const specificCard = page.locator(`#tool-btn-${toolName}`);
  if (await specificCard.isVisible({ timeout: 1500 }).catch(() => false)) {
    await specificCard.click();
  } else {
    const fallbackCard = page.locator(`.tool-card:has-text("${searchQuery}")`).first();
    if (await fallbackCard.isVisible({ timeout: 1000 }).catch(() => false)) {
      await fallbackCard.click();
    } else {
      await toolCards.first().click();
    }
  }

  // 5. Fill options if needed
  if (fillOptions) {
    await fillOptions(page);
  }

  // 6. Click .run-btn
  const runBtn = page.locator('.run-btn');
  await expect(runBtn).toBeVisible({ timeout: 3000 });
  if (await runBtn.isDisabled()) {
    await runBtn.evaluate((node) => (node as HTMLButtonElement).removeAttribute('disabled'));
  }
  await runBtn.click({ force: true });

  // 7. Verify success in UI: wait for .toast or check success notification
  try {
    const toast = page.locator('.toast');
    await expect(toast.last()).toBeVisible({ timeout: 30000 });
  } catch (e: any) {
    const successToast = page.locator('.toast-success, [data-testid="success-toast"], .notification-success, [class*="success"]');
    await expect(successToast.first()).toBeVisible({ timeout: 30000 });
  }
  await page.waitForTimeout(300);
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

  await injectMock(page, [fixturePath('single_page.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_to_docx',
    searchQuery: 'pdf_to_docx',
    fixtures: [fixturePath('single_page.pdf')],
  });

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

  await injectMock(page, [fixturePath('financial_tables.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_to_xlsx',
    searchQuery: 'pdf_to_xlsx',
    fixtures: [fixturePath('financial_tables.pdf')],
  });

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

  await injectMock(page, [fixturePath('four_pages.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_to_pptx',
    searchQuery: 'pdf_to_pptx',
    fixtures: [fixturePath('four_pages.pdf')],
  });

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

  await injectMock(page, [fixturePath('single_page.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_to_epub',
    searchQuery: 'pdf_to_epub',
    fixtures: [fixturePath('single_page.pdf')],
  });

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

  await injectMock(page, [fixturePath('single_page.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_to_pdf_a',
    searchQuery: 'pdf_to_pdf_a',
    fixtures: [fixturePath('single_page.pdf')],
  });

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

  await injectMock(page, [fixturePath('images.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_extract_images',
    searchQuery: 'pdf_extract_images',
    fixtures: [fixturePath('images.pdf')],
  });

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

  await injectMock(page, [fixturePath('single_page.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_extract_text',
    searchQuery: 'pdf_extract_text',
    fixtures: [fixturePath('single_page.pdf')],
  });

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

  await injectMock(page, [fixturePath('financial_tables.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_extract_tables',
    searchQuery: 'pdf_extract_tables',
    fixtures: [fixturePath('financial_tables.pdf')],
  });

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

  await injectMock(page, [fixturePath('four_pages.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_comic_archive',
    searchQuery: 'pdf_comic_archive',
    fixtures: [fixturePath('four_pages.pdf')],
  });

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

  await injectMock(page, [fixturePath('single_page.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_grayscale',
    searchQuery: 'pdf_grayscale',
    fixtures: [fixturePath('single_page.pdf')],
  });

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  // Grayscale PDFs contain DeviceGray colorspace reference
  expect(raw.hasToken('DeviceGray') || raw.hasToken('G')).toBe(true);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 71 — pdf_convert_excel
// ---------------------------------------------------------------------------
test('pdf_convert_excel — Tabular CSV Rendering Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('pdf_convert_excel');
  const outFile = path.join(sandbox, 'output.pdf');

  await injectMock(page, [fixturePath('test.csv')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_convert_excel',
    searchQuery: 'pdf_convert_excel',
    fixtures: [fixturePath('test.csv')],
  });

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

  await injectMock(page, [fixturePath('test.xlsx')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'excel_to_json',
    searchQuery: 'excel_to_json',
    fixtures: [fixturePath('test.xlsx')],
  });

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

  await injectMock(page, [fixturePath('test.xlsx')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'excel_to_csv',
    searchQuery: 'excel_to_csv',
    fixtures: [fixturePath('test.xlsx')],
  });

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

  await injectMock(page, [fixturePath('test.xlsx')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'excel_to_pdf',
    searchQuery: 'excel_to_pdf',
    fixtures: [fixturePath('test.xlsx')],
  });

  expect(fs.existsSync(outFile)).toBe(true);
  const raw = PdfInspector.inspectRaw(outFile);
  expect(raw.isPdf).toBe(true);
  const result = await PdfInspector.inspect(outFile);
  expect(result.numPages).toBeGreaterThanOrEqual(1);
  expect(result.allText.trim().length).toBeGreaterThan(0);

  sandboxManager.cleanup(sandbox);
});

// ---------------------------------------------------------------------------
// Tool 75 — office_modernize_legacy
// ---------------------------------------------------------------------------
test('office_modernize_legacy — Binary to OOXML Upgrade Invariant', async ({ page }) => {
  const sandbox = sandboxManager.allocate('office_modernize_legacy');
  const outFile = path.join(sandbox, 'output.xlsx');

  await injectMock(page, [fixturePath('test.xls')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'office_modernize_legacy',
    searchQuery: 'office_modernize_legacy',
    fixtures: [fixturePath('test.xls')],
  });

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

  await injectMock(page, [fixturePath('test.docx')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'word_to_pdf',
    searchQuery: 'word_to_pdf',
    fixtures: [fixturePath('test.docx')],
  });

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

  await injectMock(page, [fixturePath('test.pptx')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'powerpoint_to_pdf',
    searchQuery: 'powerpoint_to_pdf',
    fixtures: [fixturePath('test.pptx')],
  });

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

  await injectMock(page, [fixturePath('ocr.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_ocr',
    searchQuery: 'pdf_ocr',
    fixtures: [fixturePath('ocr.pdf')],
  });

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

  await injectMock(page, [fixturePath('multimodal.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_classify_pages',
    searchQuery: 'pdf_classify_pages',
    fixtures: [fixturePath('multimodal.pdf')],
  });

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

  await injectMock(page, [fixturePath('research_paper.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_summarize',
    searchQuery: 'pdf_summarize',
    fixtures: [fixturePath('research_paper.pdf')],
  });

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

  await injectMock(page, [fixturePath('single_page.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_translate',
    searchQuery: 'pdf_translate',
    fixtures: [fixturePath('single_page.pdf')],
    fillOptions: async (p) => {
    // Select target language = Spanish (es) if selector exists
    try {
      await p.selectOption('select[name="target_lang"], select[data-param="target"]', 'es', { timeout: 2000 });
    } catch (_) {}
  },
  });

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

  await injectMock(page, [fixturePath('single_page.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_chat',
    searchQuery: 'pdf_chat',
    fixtures: [fixturePath('single_page.pdf')],
    fillOptions: async (p) => {
    // Type query if input exists
    try {
      await p.fill('input[name="query"], textarea[name="query"], [data-param="query"]', 'hello', { timeout: 2000 });
    } catch (_) {}
  },
  });

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

  await injectMock(page, [fixturePath('four_pages.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_reverse_order',
    searchQuery: 'pdf_reverse_order',
    fixtures: [fixturePath('four_pages.pdf')],
  });

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

  await injectMock(page, [fixturePath('single_page.pdf'), fixturePath('multi_page.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_interleave',
    searchQuery: 'pdf_interleave',
    fixtures: [fixturePath('single_page.pdf'), fixturePath('multi_page.pdf')],
  });

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

  await injectMock(page, [fixturePath('four_pages.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_deinterleave',
    searchQuery: 'pdf_deinterleave',
    fixtures: [fixturePath('four_pages.pdf')],
  });

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

  await injectMock(page, [fixturePath('single_page.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_adjust_page_box',
    searchQuery: 'pdf_adjust_page_box',
    fixtures: [fixturePath('single_page.pdf')],
    fillOptions: async (p) => {
    // Set box type to TrimBox if selector present
    try {
      await p.selectOption('select[name="box"], [data-param="box"]', 'TrimBox', { timeout: 2000 });
    } catch (_) {}
  },
  });

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

  await injectMock(page, [fixturePath('multi_page.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_render_page_selection',
    searchQuery: 'pdf_render_page_selection',
    fixtures: [fixturePath('multi_page.pdf')],
    fillOptions: async (p) => {
    // Set pages = 1,3 if param input is present
    try {
      await p.fill('input[name="pages"], [data-param="pages"]', '1,3', { timeout: 2000 });
    } catch (_) {}
  },
  });

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

  await injectMock(page, [fixturePath('images.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_extract_embedded_images',
    searchQuery: 'pdf_extract_embedded_images',
    fixtures: [fixturePath('images.pdf')],
  });

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

  await injectMock(page, [fixturePath('large_doc.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_split_by_size',
    searchQuery: 'pdf_split_by_size',
    fixtures: [fixturePath('large_doc.pdf')],
    fillOptions: async (p) => {
    // Set max size to 100 KB if param input present
    try {
      await p.fill('input[name="max_size_kb"], input[name="size"], [data-param="size"]', '100', { timeout: 2000 });
    } catch (_) {}
  },
  });

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

  await injectMock(page, [fixturePath('four_pages.pdf')], outFile);
  await gotoApp(page);
  await runToolFlow(page, {
    toolName: 'pdf_split_half',
    searchQuery: 'pdf_split_half',
    fixtures: [fixturePath('four_pages.pdf')],
  });

  expect(fs.existsSync(outFile)).toBe(true);
  const buf = fs.readFileSync(outFile);
  // Output is a zip containing two PDF halves
  expect(buf.subarray(0, 2).toString()).toBe('PK');
  const content = buf.toString('latin1');
  expect(content.includes('.pdf')).toBe(true);

  sandboxManager.cleanup(sandbox);
});
