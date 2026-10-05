import { test, expect } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Load test fixtures
const FIXTURES_DIR = path.resolve(__dirname, '../../../tests/e2e_fixtures');
const SINGLE_PAGE_PDF = fs.readFileSync(path.join(FIXTURES_DIR, 'single_page.pdf'));
const MULTI_PAGE_PDF = fs.readFileSync(path.join(FIXTURES_DIR, 'multi_page.pdf'));

interface ToolResult {
  tool: string;
  status: 'PASS' | 'FAIL';
  latency: number;
  outputSize: number;
  notes: string;
}

const scorecard: ToolResult[] = [];

test.describe('WASM 12 Tools E2E Scorecard', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the app to ensure WASM environment is loaded and Web Worker is active
    await page.goto('/');
    // Wait for basic rendering
    await page.waitForSelector('.app-container', { state: 'visible' });
    // Small delay for web worker and WASM init
    await page.waitForTimeout(500);
  });

  async function runWasmTool(page: any, toolName: string, payload: any): Promise<{ length: number; isPdf: boolean; duration: number }> {
    const startTime = performance.now();
    const result = await page.evaluate(async ({ opName, payload }: any) => {
      const { wasmPdfClient } = await import('/src/lib/wasm/index.ts');
      let res: Uint8Array | Uint8Array[];

      const toUint8 = (b64: string) => Uint8Array.from(atob(b64), c => c.charCodeAt(0));

      if (opName === 'merge') {
        res = await wasmPdfClient.merge(payload.files.map(toUint8));
      } else if (opName === 'split') {
        res = await wasmPdfClient.split(toUint8(payload.file), payload.ranges);
      } else if (opName === 'rotate') {
        res = await wasmPdfClient.rotate(toUint8(payload.file), payload.angle, payload.pages);
      } else if (opName === 'compress') {
        res = await wasmPdfClient.compress(toUint8(payload.file));
      } else if (opName === 'encrypt') {
        res = await wasmPdfClient.encrypt(toUint8(payload.file), payload.password);
      } else if (opName === 'watermark') {
        res = await wasmPdfClient.watermark(toUint8(payload.file), payload.text);
      } else if (opName === 'delete_pages') {
        res = await wasmPdfClient.delete_pages(toUint8(payload.file), payload.pages);
      } else if (opName === 'extract_pages') {
        res = await wasmPdfClient.extract_pages(toUint8(payload.file), payload.pages);
      } else if (opName === 'reorder_pages') {
        res = await wasmPdfClient.reorder_pages(toUint8(payload.file), payload.new_order);
      } else if (opName === 'crop') {
        res = await wasmPdfClient.crop(toUint8(payload.file), payload.left, payload.bottom, payload.right, payload.top);
      } else if (opName === 'flatten') {
        res = await wasmPdfClient.flatten(toUint8(payload.file));
      } else if (opName === 'set_metadata') {
        res = await wasmPdfClient.set_metadata(toUint8(payload.file), payload.title, payload.author, payload.subject, payload.keywords);
      } else {
        throw new Error(`Unsupported tool: ${opName}`);
      }

      const bytes = Array.isArray(res) ? res[0] : res;
      const isPdf = bytes[0] === 0x25 && bytes[1] === 0x50 && bytes[2] === 0x44 && bytes[3] === 0x46; // %PDF- check
      return { length: bytes.length, isPdf };
    }, { opName: toolName, payload });

    const duration = performance.now() - startTime;
    return { ...result, duration };
  }

  function recordResult(tool: string, result: { length: number; isPdf: boolean; duration: number }) {
    scorecard.push({
      tool,
      status: result.isPdf && result.length > 0 ? 'PASS' : 'FAIL',
      latency: Math.round(result.duration),
      outputSize: result.length,
      notes: result.isPdf ? 'Valid %PDF- header' : 'Invalid output'
    });
    expect(result.isPdf).toBe(true);
    expect(result.length).toBeGreaterThan(0);
  }

  test('merge', async ({ page }) => {
    const files = [SINGLE_PAGE_PDF.toString('base64'), MULTI_PAGE_PDF.toString('base64')];
    const res = await runWasmTool(page, 'merge', { files });
    recordResult('merge', res);
  });

  test('split', async ({ page }) => {
    const file = MULTI_PAGE_PDF.toString('base64');
    const res = await runWasmTool(page, 'split', { file, ranges: '1' });
    recordResult('split', res);
  });

  test('rotate', async ({ page }) => {
    const file = SINGLE_PAGE_PDF.toString('base64');
    const res = await runWasmTool(page, 'rotate', { file, angle: 90, pages: 'all' });
    recordResult('rotate', res);
  });

  test('compress', async ({ page }) => {
    const file = MULTI_PAGE_PDF.toString('base64');
    const res = await runWasmTool(page, 'compress', { file });
    recordResult('compress', res);
  });

  test('encrypt', async ({ page }) => {
    const file = SINGLE_PAGE_PDF.toString('base64');
    const res = await runWasmTool(page, 'encrypt', { file, password: 'secret123' });
    recordResult('encrypt', res);
  });

  test('watermark', async ({ page }) => {
    const file = SINGLE_PAGE_PDF.toString('base64');
    const res = await runWasmTool(page, 'watermark', { file, text: 'WASM TEST' });
    recordResult('watermark', res);
  });

  test('delete_pages', async ({ page }) => {
    const file = MULTI_PAGE_PDF.toString('base64');
    const res = await runWasmTool(page, 'delete_pages', { file, pages: '2' });
    recordResult('delete_pages', res);
  });

  test('extract_pages', async ({ page }) => {
    const file = MULTI_PAGE_PDF.toString('base64');
    const res = await runWasmTool(page, 'extract_pages', { file, pages: '1' });
    recordResult('extract_pages', res);
  });

  test('reorder_pages', async ({ page }) => {
    const file = MULTI_PAGE_PDF.toString('base64');
    // multi_page.pdf has 5 pages according to pdfinfo
    const res = await runWasmTool(page, 'reorder_pages', { file, new_order: [5, 4, 3, 2, 1] });
    recordResult('reorder_pages', res);
  });

  test('crop', async ({ page }) => {
    const file = SINGLE_PAGE_PDF.toString('base64');
    const res = await runWasmTool(page, 'crop', { file, left: 10, bottom: 10, right: 500, top: 700 });
    recordResult('crop', res);
  });

  test('flatten', async ({ page }) => {
    const file = SINGLE_PAGE_PDF.toString('base64');
    const res = await runWasmTool(page, 'flatten', { file });
    recordResult('flatten', res);
  });

  test('set_metadata', async ({ page }) => {
    const file = SINGLE_PAGE_PDF.toString('base64');
    const res = await runWasmTool(page, 'set_metadata', { file, title: 'E2E Title', author: 'Jules' });
    recordResult('set_metadata', res);
  });

  test.afterAll(async () => {
    const passed = scorecard.filter((s) => s.status === 'PASS').length;
    const failed = scorecard.filter((s) => s.status === 'FAIL').length;

    let report = `# WASM 12 Tools E2E Scorecard\n\n`;
    report += `## Executive Summary\n`;
    report += `- **Total Tested:** ${scorecard.length}\n`;
    report += `- **Passed:** ${passed}\n`;
    report += `- **Failed:** ${failed}\n`;
    report += `- **Environment:** Headless Chromium + WebAssembly (\`wasm32-unknown-unknown\`) + Web Worker\n\n`;

    report += `## Operations Status\n\n`;
    report += `| Tool Name | Status | Web Worker & WASM Latency (ms) | Output Size (bytes) | Verification Notes |\n`;
    report += `|-----------|--------|--------------------------------|---------------------|-------------------|\n`;

    for (const res of scorecard) {
      const statusIcon = res.status === 'PASS' ? '✅ PASS' : '❌ FAIL';
      report += `| \`${res.tool}\` | ${statusIcon} | ${res.latency} | ${res.outputSize} | ${res.notes} |\n`;
    }

    const reportsDir = path.resolve(__dirname, '../../../reports');
    if (!fs.existsSync(reportsDir)) {
      fs.mkdirSync(reportsDir, { recursive: true });
    }
    fs.writeFileSync(path.join(reportsDir, 'WASM_12_TOOLS_E2E_SCORECARD.md'), report);
  });

});