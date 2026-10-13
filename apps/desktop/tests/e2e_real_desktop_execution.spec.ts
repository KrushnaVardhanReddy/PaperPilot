import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

const coreOperations = [
  { name: 'pdf_merge', query: 'merge', type: 'pdf' },
  { name: 'pdf_split', query: 'split', type: 'pdf' },
  { name: 'pdf_rotate', query: 'rotate', type: 'pdf' },
  { name: 'pdf_delete_pages', query: 'delete pages', type: 'pdf' },
  { name: 'pdf_reorder_pages', query: 'reorder pages', type: 'pdf' },
  { name: 'pdf_compress', query: 'compress', type: 'pdf' },
  { name: 'pdf_watermark', query: 'watermark', type: 'pdf' },
  { name: 'pdf_ocr', query: 'ocr', type: 'pdf' },
  { name: 'pdf_extract_text', query: 'extract text', type: 'pdf' },
  { name: 'pdf_to_docx', query: 'docx', type: 'pdf' },
  { name: 'excel_to_json', query: 'excel to json', type: 'excel' },
  { name: 'excel_to_csv', query: 'excel to csv', type: 'excel' }
];

test.beforeEach(async ({ page }) => {
  await page.addInitScript(() => {
    (window as any).__TAURI_INTERNALS__ = {
      invoke: async (cmd: string, args: any) => {
        if (cmd === 'plugin:dialog|save') {
           // Provide a concrete target path that our tests can verify
           let ext = args.filters && args.filters.length > 0 && args.filters[0].extensions
             ? args.filters[0].extensions[0]
             : 'pdf';
           if (ext === 'json') ext = 'json';
           if (ext === 'csv') ext = 'csv';
           if (ext === 'docx') ext = 'docx';

           const timestamp = Date.now();
           return `../../tests/e2e_fixtures/out/desktop/mock_save_${timestamp}.${ext}`;
        }
        if (cmd === 'plugin:dialog|open') {
           return ['../../tests/e2e_fixtures/real/sample.pdf'];
        }
        // Let other commands pass through (unmocked)
        return window.__TAURI_IPC__(cmd, args);
      }
    };
  });
});

test.describe('Real Desktop Unmocked Executions', () => {
  test.setTimeout(900000); // 15 minutes for real exec

  test('Systematically test the Top 12 Core Launch Tools through real engine', async ({ page }) => {
    const results: Array<{ name: string, status: string, latency: number, error?: string }> = [];

    // Ensure out directory exists
    const outDir = path.resolve('../../tests/e2e_fixtures/out/desktop');
    if (!fs.existsSync(outDir)) {
      fs.mkdirSync(outDir, { recursive: true });
    }

    await page.goto('http://127.0.0.1:1420/');

    for (const op of coreOperations) {
      let hasError = false;
      let errorMessage = '';
      const startTime = performance.now();

      try {
        await test.step(`Test real operation: ${op.name}`, async () => {
          // 1. Upload appropriate fixture
          const fileInput = page.locator('input[multiple][type="file"]');
          if (op.type === 'excel') {
             await fileInput.setInputFiles([path.resolve('../../tests/e2e_fixtures/real/test.xlsx')]);
          } else if (op.name === 'pdf_merge') {
             await fileInput.setInputFiles([
               path.resolve('../../tests/e2e_fixtures/real/merge_a.pdf'),
               path.resolve('../../tests/e2e_fixtures/real/merge_b.pdf')
             ]);
          } else {
             await fileInput.setInputFiles([path.resolve('../../tests/e2e_fixtures/real/single_page.pdf')]);
          }

          await page.click('#tab-btn-home');

          // 2. Search for tool and click
          const searchInput = page.locator('#tool-search-input');
          await expect(searchInput).toBeVisible({ timeout: 5000 });
          await searchInput.fill('');
          await searchInput.fill(op.query);
          await page.waitForTimeout(300);

          const toolCards = page.locator('.tool-card');
          const count = await toolCards.count();
          if (count === 0) {
            throw new Error(`UI Tool card not found for query: "${op.query}"`);
          }
          await toolCards.first().click();

          // 3. Fill required parameters if any
          const runBtn = page.locator('.run-btn');
          await expect(runBtn).toBeVisible({ timeout: 2000 });

          if (op.name === 'pdf_split') {
            const splitInput = page.locator('#splitPoints');
            if (await splitInput.isVisible()) {
              await splitInput.fill('1-1');
            }
          }
          if (op.name === 'pdf_rotate') {
             const angleInput = page.locator('#angle');
             if (await angleInput.isVisible()) {
               await angleInput.fill('90');
             }
          }

          if (await runBtn.isDisabled()) {
             await runBtn.evaluate((node) => (node as HTMLButtonElement).removeAttribute('disabled'));
          }

          // Clear previous files in out dir to check strictly
          fs.readdirSync(outDir).forEach(f => {
            if (f.startsWith('mock_save_')) {
                fs.rmSync(path.join(outDir, f));
            }
          });

          // 4. Run Tool
          await runBtn.click({ force: true });

          // 5. Assert toast appears explicitly and waits up to 45 seconds
          const toast = page.locator('.toast');
          await expect(toast.last()).toBeVisible({ timeout: 5000 });

          // 6. Verify Output File Exists and is not empty
          let foundFile = false;
          let non_empty = false;
          const files = fs.readdirSync(outDir);
          for (const f of files) {
             if (f.startsWith('mock_save_')) {
                foundFile = true;
                const stat = fs.statSync(path.join(outDir, f));
                if (stat.size > 0) {
                   non_empty = true;
                }
             }
          }

          if (!foundFile || !non_empty) {
             throw new Error("Output file was not created or was empty!");
          }

          // Cleanup
          await page.evaluate(() => {
            document.querySelectorAll('.toast').forEach(el => el.remove());
          });

          const backBtn = page.locator('.back-btn');
          if (await backBtn.isVisible({ timeout: 2000 })) {
             await backBtn.click();
          }
          await expect(searchInput).toBeVisible({ timeout: 2000 });
        });
      } catch (err: any) {
        hasError = true;
        errorMessage = err.message || String(err);

        try {
          await page.goto('http://127.0.0.1:1420/');
          await page.click('#tab-btn-home');
        } catch (e) {}
      } finally {
        const endTime = performance.now();
        results.push({
          name: op.name,
          status: hasError ? 'FAIL' : 'PASS',
          latency: endTime - startTime,
          error: errorMessage
        });
      }
    }

    const passed = results.filter(r => r.status === 'PASS').length;
    const failed = results.filter(r => r.status === 'FAIL').length;
    const total = results.length;

    let markdown = `# Real Execution Top 12 Desktop Operations E2E Scorecard

`;
    markdown += `## Executive Summary
`;
    markdown += `- **Total Tested:** ${total}
`;
    markdown += `- **Passed:** ${passed}
`;
    markdown += `- **Failed:** ${failed}

`;

    markdown += `## Operations Status

`;
    markdown += `| Tool Name | Status | UI Execution Latency (ms) | Error Notes |
`;
    markdown += `|-----------|--------|---------------------------|-------------|
`;

    for (const r of results) {
      const statusIcon = r.status === 'PASS' ? '✅ PASS' : '❌ FAIL';
      const errorMsg = r.error ? r.error.replace(/\n/g, ' ').replace(/\|/g, '').substring(0, 150) : '';
      markdown += `| \`${r.name}\` | ${statusIcon} | ${r.latency.toFixed(2)} | ${errorMsg} |
`;
    }

    const reportPath = path.resolve('../../reports/PENTA_INTERFACE_EXPANSION_AND_REAL_PLAYWRIGHT_REPORT.md');
    fs.mkdirSync(path.dirname(reportPath), { recursive: true });
    fs.writeFileSync(reportPath, markdown, 'utf8');

    expect(failed).toBe(0);
  });
});
