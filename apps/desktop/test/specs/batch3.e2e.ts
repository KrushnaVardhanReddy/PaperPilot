import path from 'node:path';
import fs from 'node:fs';
import { expect, $, $$ } from '@wdio/globals';
import { verifyOutputContent } from '../helpers/contentVerifier';

describe('PaperPilot Desktop E2E - Batch 3 (10 Tools)', () => {
  const rootDir = path.resolve(__dirname, '../../../../');
  const financialPdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/financial_tables.pdf');
  const imageDocPdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/image_doc.pdf');
  const singlePagePdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/single_page.pdf');

  const tools = [
    {
      id: 'pdf_extract_text',
      title: 'PDF to Plain Text',
      inputFmt: 'PDF',
      outputFmt: '.TXT',
      fixtureType: 'financial',
      outputPath: '/tmp/desktop_e2e_pdf_extract_text.txt',
      expectedType: 'text' as const,
      expectedControlSelector: '.field-hint'
    },
    {
      id: 'pdf_extract_tables',
      title: 'Extract Tables',
      inputFmt: 'PDF',
      outputFmt: '.JSON/.CSV',
      fixtureType: 'financial',
      outputPath: '/tmp/desktop_e2e_pdf_extract_tables.json',
      expectedType: 'json' as const,
      expectedControlId: '#tableFormat'
    },
    {
      id: 'pdf_extract_images',
      title: 'Extract Embedded Images',
      inputFmt: 'PDF',
      outputFmt: 'PNG Images',
      fixtureType: 'image_doc',
      outputPath: '/tmp/desktop_e2e_pdf_extract_images_dir',
      expectedType: 'image_dir' as const,
      expectedControlId: '#imageFormat'
    },
    {
      id: 'pdf_search',
      title: 'Search Text',
      inputFmt: 'PDF',
      outputFmt: '.JSON',
      fixtureType: 'financial',
      outputPath: '/tmp/desktop_e2e_pdf_search.json',
      expectedType: 'json' as const,
      expectedControlId: '#textSearchQueryInput'
    },
    {
      id: 'pdf_ocr',
      title: 'OCR Text Recognition',
      inputFmt: 'Scanned PDF / Image',
      outputFmt: '.TXT / PDF',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_ocr.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#ocrOutputFormat'
    },
    {
      id: 'pdf_rag_chunks',
      title: 'RAG Pipeline Chunks',
      inputFmt: 'PDF',
      outputFmt: '.JSON',
      fixtureType: 'financial',
      outputPath: '/tmp/desktop_e2e_pdf_rag_chunks.json',
      expectedType: 'json' as const,
      expectedControlId: '#ragChunkSizeInput'
    },
    {
      id: 'pdf_classify_type',
      title: 'Classify Document',
      inputFmt: 'PDF',
      outputFmt: '.JSON',
      fixtureType: 'financial',
      outputPath: '/tmp/desktop_e2e_pdf_classify_type.json',
      expectedType: 'json' as const,
      expectedControlSelector: '.field-hint'
    },
    {
      id: 'pdf_metadata',
      title: 'Edit / Sanitize Metadata',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'financial',
      outputPath: '/tmp/desktop_e2e_pdf_metadata.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#metadataTitle'
    },
    {
      id: 'pdf_hash',
      title: 'Generate Hash',
      inputFmt: 'PDF',
      outputFmt: '.TXT',
      fixtureType: 'financial',
      outputPath: '/tmp/desktop_e2e_pdf_hash.txt',
      expectedType: 'text' as const,
      expectedControlId: '#hashAlgorithm'
    },
    {
      id: 'pdf_validate',
      title: 'Validate PDF',
      inputFmt: 'PDF',
      outputFmt: '.TXT',
      fixtureType: 'financial',
      outputPath: '/tmp/desktop_e2e_pdf_validate.json',
      expectedType: 'json' as const,
      expectedControlSelector: '.field-hint'
    }
  ];

  before(async () => {
    const sidebar = await $('[data-testid="sidebar"]');
    await sidebar.waitForDisplayed({ timeout: 20000 });
    expect(await sidebar.isDisplayed()).toBe(true);

    const operationsPanel = await $('[data-testid="operations-panel"]');
    await operationsPanel.waitForDisplayed({ timeout: 15000 });
    expect(await operationsPanel.isDisplayed()).toBe(true);

    // Dismiss onboarding modal if present
    const skipBtn = await $('button=Skip for Now');
    if (await skipBtn.isExisting()) {
      try {
        await skipBtn.click();
        await browser.pause(300);
      } catch {
        // Modal might have closed
      }
    }

    // Switch from "Recommended" to "All Tools" tab to make all 100 tools accessible
    const allToolsTab = await $('#context-btn-all');
    await allToolsTab.waitForDisplayed({ timeout: 5000 });
    await allToolsTab.click();
    await browser.pause(500);
    console.log('[E2E Batch 3] Switched to All Tools view');
  });

  afterEach(async () => {
    // Return to tool directory after each test even if assertions failed
    try {
      const backBtn = await $('#btn-back-tools');
      if (await backBtn.isDisplayed()) {
        await backBtn.click();
        await browser.pause(300);
      }
    } catch {
      // Ignore navigation errors in cleanup
    }
  });

  for (const tool of tools) {
    it(`should load fixture, configure, run, and deeply verify output for ${tool.id} (${tool.title})`, async () => {
      // Remove stale output if present
      if (fs.existsSync(tool.outputPath)) {
        fs.rmSync(tool.outputPath, { recursive: true, force: true });
      }

      // Step 1: Clear previous documents and load test fixture via window.__TEST_LOAD_FILE
      await browser.execute((fType: string, finPath: string, imgDocPath: string, singlePath: string) => {
        const app = (window as any).appState;
        if (app && typeof app.clearAllDocuments === 'function') {
          app.clearAllDocuments();
        }
        const loadFile = (window as any).__TEST_LOAD_FILE;
        if (typeof loadFile === 'function') {
          if (fType === 'financial') {
            loadFile('financial_tables.pdf', finPath);
          } else if (fType === 'image_doc') {
            loadFile('image_doc.pdf', imgDocPath);
          } else if (fType === 'single_page') {
            loadFile('single_page.pdf', singlePath);
          }
        }
      }, tool.fixtureType, financialPdfPath, imageDocPdfPath, singlePagePdfPath);

      await browser.pause(300);

      // Ensure All Tools view is active
      const allToolsTab = await $('#context-btn-all');
      if (await allToolsTab.isDisplayed()) {
        await allToolsTab.click();
        await browser.pause(200);
      }

      // 1. Locate tool button directly in directory by id
      const toolBtn = await $(`#tool-btn-${tool.id}`);
      await toolBtn.scrollIntoView({ block: 'center' });
      await toolBtn.waitForDisplayed({ timeout: 10000 });
      expect(await toolBtn.isDisplayed()).toBe(true);

      // 2. Click tool to open Inspector
      await toolBtn.click();

      // 3. Verify active state in Inspector
      const inspectorTitle = await $('.inspector-title');
      await inspectorTitle.waitForDisplayed({ timeout: 5000 });
      expect(await inspectorTitle.isDisplayed()).toBe(true);

      // 4. Verify Tool-specific Parameter Controls and configure
      const controlSelector = tool.expectedControlId || tool.expectedControlSelector;
      if (controlSelector) {
        const control = await $(controlSelector);
        await control.waitForDisplayed({ timeout: 10000 });
        expect(await control.isDisplayed()).toBe(true);
      }

      // Configure tool specific options where necessary
      if (tool.id === 'pdf_extract_tables') {
        const tableFormatSelect = await $('#tableFormat');
        await tableFormatSelect.selectByAttribute('value', 'json');
      } else if (tool.id === 'pdf_search') {
        const searchInput = await $('#textSearchQueryInput');
        await searchInput.setValue('Revenue');
      } else if (tool.id === 'pdf_ocr') {
        const ocrSelect = await $('#ocrOutputFormat');
        await ocrSelect.selectByAttribute('value', 'pdf');
      } else if (tool.id === 'pdf_metadata') {
        const metaTitleInput = await $('#metadataTitle');
        await metaTitleInput.setValue('Verified Batch 3 Release');
      }

      // 5. Explicitly set predictable output path
      const outputPathInput = await $('#output-path-input');
      if (await outputPathInput.isDisplayed()) {
        await outputPathInput.setValue(tool.outputPath);
      }

      // 6. Click Run Operation Button
      const runBtn = await $('[data-testid="run-operation-btn"]');
      await runBtn.waitForDisplayed({ timeout: 5000 });
      expect(await runBtn.isDisplayed()).toBe(true);
      await browser.execute((el: HTMLElement) => {
        el.click();
      }, runBtn);

      // 7. Wait for operation completion: check disk and verify content
      if (tool.expectedType === 'image_dir') {
        await browser.waitUntil(
          () => {
            if (!fs.existsSync(tool.outputPath)) return false;
            const files = fs.readdirSync(tool.outputPath);
            return files.length > 0;
          },
          {
            timeout: 25000,
            timeoutMsg: `Expected output directory ${tool.outputPath} with extracted images within 25s`
          }
        );

        const files = fs.readdirSync(tool.outputPath);
        expect(files.length).toBeGreaterThan(0);
        for (const file of files) {
          const imgFilePath = path.join(tool.outputPath, file);
          const stats = fs.statSync(imgFilePath);
          expect(stats.size).toBeGreaterThan(0);
        }
        console.log(`[E2E Batch 3] Successfully verified extracted images in ${tool.outputPath}`);
      } else {
        await browser.waitUntil(
          () => {
            return fs.existsSync(tool.outputPath) && fs.statSync(tool.outputPath).size > 0;
          },
          {
            timeout: 25000,
            timeoutMsg: `Expected output file ${tool.outputPath} to be created on disk within 25s`
          }
        );

        verifyOutputContent(tool.outputPath, tool.expectedType);
        console.log(`[E2E Batch 3] Successfully verified output content for ${tool.id} at ${tool.outputPath}`);
      }

      // 8. Clean up created output file / directory
      try {
        if (fs.existsSync(tool.outputPath)) {
          fs.rmSync(tool.outputPath, { recursive: true, force: true });
        }
      } catch (err) {
        console.warn('Could not clean up output file:', err);
      }

      // 9. Return to tool directory for subsequent tool tests
      const backBtn = await $('#btn-back-tools');
      await backBtn.waitForDisplayed({ timeout: 5000 });
      await backBtn.click();
      await browser.pause(300);
    });
  }
});
