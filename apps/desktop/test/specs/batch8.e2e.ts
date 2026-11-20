import path from 'node:path';
import fs from 'node:fs';
import { expect, $, $$ } from '@wdio/globals';
import { verifyOutputContent } from '../helpers/contentVerifier';

describe('PaperPilot Desktop E2E - Batch 8 (10 Document Comparison, Optimization & Alternate Page Operations)', () => {
  const rootDir = path.resolve(__dirname, '../../../../');
  const singlePagePdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/single_page.pdf');
  const multiPagePdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/multi_page.pdf');
  const mergeAPdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/merge_a.pdf');
  const mergeBPdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/merge_b.pdf');
  const img1Path = path.resolve(rootDir, 'tests/e2e_fixtures/real/img1.png');
  const img2Path = path.resolve(rootDir, 'tests/e2e_fixtures/real/img2.png');
  const formPdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/form.pdf');
  const batchCsvPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/batch_data.csv');
  const testMdPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/test.md');
  const testHtmlPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/test.html');

  const tools = [
    {
      id: 'pdf_merge_alternate',
      title: 'Alternating PDF Merge',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'merge_alternate',
      outputPath: '/tmp/desktop_e2e_pdf_merge_alternate.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#mergeAlternateDocA'
    },
    {
      id: 'pdf_burst',
      title: 'Burst PDF',
      inputFmt: 'PDF',
      outputFmt: 'Multiple .PDFs',
      fixtureType: 'multi_page',
      outputPath: '/tmp/desktop_e2e_pdf_burst_dir',
      expectedType: 'burst_dir' as const,
      expectedControlSelector: '.operation-config'
    },
    {
      id: 'pdf_images_to_pdf',
      title: 'Images to PDF',
      inputFmt: 'Images',
      outputFmt: '.PDF',
      fixtureType: 'images',
      outputPath: '/tmp/desktop_e2e_pdf_images_to_pdf.pdf',
      expectedType: 'pdf' as const,
      expectedControlSelector: '.operation-config'
    },
    {
      id: 'pdf_dedup_streams',
      title: 'Lossless Stream Deduplication',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'multi_page',
      outputPath: '/tmp/desktop_e2e_pdf_dedup_streams.pdf',
      expectedType: 'pdf' as const,
      expectedControlSelector: '.operation-config'
    },
    {
      id: 'pdf_linearize',
      title: 'Fast Web View',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'multi_page',
      outputPath: '/tmp/desktop_e2e_pdf_linearize.pdf',
      expectedType: 'pdf' as const,
      expectedControlSelector: '.operation-config'
    },
    {
      id: 'pdf_render',
      title: 'Render Page',
      inputFmt: 'PDF',
      outputFmt: 'Images (PNG/JPEG/WebP)',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_render.png',
      expectedType: 'png' as const,
      expectedControlId: '#renderFormat'
    },
    {
      id: 'pdf_compare',
      title: 'Compare Documents',
      inputFmt: '2 PDFs',
      outputFmt: '.PDF',
      fixtureType: 'compare_two',
      outputPath: '/tmp/desktop_e2e_pdf_compare.json',
      expectedType: 'json' as const,
      expectedControlId: '#compareDocA'
    },
    {
      id: 'batch_fill',
      title: 'Batch Form Fill',
      inputFmt: 'PDF + CSV',
      outputFmt: '.ZIP/.PDF',
      fixtureType: 'form_batch',
      outputPath: '/tmp/desktop_e2e_batch_fill_dir',
      expectedType: 'batch_dir' as const,
      expectedControlId: '#batchCsvPath'
    },
    {
      id: 'md_to_pdf',
      title: 'Markdown to PDF',
      inputFmt: 'Markdown (.md)',
      outputFmt: '.PDF',
      fixtureType: 'markdown',
      outputPath: '/tmp/desktop_e2e_md_to_pdf.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#convertPreset'
    },
    {
      id: 'html_to_pdf',
      title: 'HTML to PDF',
      inputFmt: 'HTML',
      outputFmt: '.PDF',
      fixtureType: 'html',
      outputPath: '/tmp/desktop_e2e_html_to_pdf.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#convertPreset'
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

    // Switch from "Recommended" to "All Tools" tab to make all tools accessible
    const allToolsTab = await $('#context-btn-all');
    await allToolsTab.waitForDisplayed({ timeout: 5000 });
    await allToolsTab.click();
    await browser.pause(500);
    console.log('[E2E Batch 8] Switched to All Tools view');
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

      // Step 1: Clear previous documents and load test fixture(s) via window.__TEST_LOAD_FILE
      await browser.execute(
        (
          fType: string,
          singlePath: string,
          multiPath: string,
          mergeAPath: string,
          mergeBPath: string,
          img1P: string,
          img2P: string,
          formP: string,
          mdP: string,
          htmlP: string
        ) => {
          const app = (window as any).appState;
          if (app && typeof app.clearAllDocuments === 'function') {
            app.clearAllDocuments();
          }
          const loadFile = (window as any).__TEST_LOAD_FILE;
          if (typeof loadFile === 'function') {
            if (fType === 'single_page') {
              loadFile('single_page.pdf', singlePath);
            } else if (fType === 'multi_page') {
              loadFile('multi_page.pdf', multiPath);
            } else if (fType === 'merge_alternate') {
              loadFile('merge_a.pdf', mergeAPath);
              loadFile('merge_b.pdf', mergeBPath);
            } else if (fType === 'images') {
              loadFile('img1.png', img1P);
              loadFile('img2.png', img2P);
            } else if (fType === 'compare_two') {
              loadFile('single_page.pdf', singlePath);
              loadFile('single_page_copy.pdf', singlePath);
            } else if (fType === 'form_batch') {
              loadFile('form.pdf', formP);
            } else if (fType === 'markdown') {
              loadFile('test.md', mdP);
            } else if (fType === 'html') {
              loadFile('test.html', htmlP);
            }
          }
        },
        tool.fixtureType,
        singlePagePdfPath,
        multiPagePdfPath,
        mergeAPdfPath,
        mergeBPdfPath,
        img1Path,
        img2Path,
        formPdfPath,
        testMdPath,
        testHtmlPath
      );

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

      // Verify format badges
      const formatBadges = await $('[data-testid="tool-format-badges"]');
      await formatBadges.waitForDisplayed({ timeout: 5000 });
      expect(await formatBadges.isDisplayed()).toBe(true);

      // 4. Verify Tool-specific Parameter Controls and configure
      const controlSelector = tool.expectedControlId || tool.expectedControlSelector;
      if (controlSelector) {
        if (tool.expectedControlId) {
          const control = await $(tool.expectedControlId);
          await control.waitForDisplayed({ timeout: 10000 });
          expect(await control.isDisplayed()).toBe(true);
        } else if (tool.expectedControlSelector) {
          const controls = await $$(tool.expectedControlSelector);
          expect(controls.length).toBeGreaterThan(0);
          expect(await controls[0].isDisplayed()).toBe(true);
        }
      }

      // Configure tool specific options if applicable
      if (tool.id === 'pdf_render') {
        const renderFormatSelect = await $('#renderFormat');
        if (await renderFormatSelect.isDisplayed()) {
          await renderFormatSelect.selectByAttribute('value', 'png');
        }
      } else if (tool.id === 'batch_fill') {
        const batchCsvInput = await $('#batchCsvPath');
        if (await batchCsvInput.isDisplayed()) {
          await batchCsvInput.setValue(batchCsvPath);
        }
      } else if (tool.id === 'md_to_pdf' || tool.id === 'html_to_pdf') {
        const presetSelect = await $('#convertPreset');
        if (await presetSelect.isDisplayed()) {
          await presetSelect.selectByAttribute('value', 'academic');
        }
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
      await browser.waitUntil(
        () => {
          if (tool.expectedType === 'burst_dir') {
            return fs.existsSync(tool.outputPath) && fs.readdirSync(tool.outputPath).filter(f => f.endsWith('.pdf')).length > 0;
          } else if (tool.expectedType === 'batch_dir') {
            return fs.existsSync(tool.outputPath) && (
              fs.existsSync(path.join(tool.outputPath, 'execution_receipt.json')) ||
              fs.readdirSync(tool.outputPath).filter(f => f.endsWith('.pdf')).length > 0
            );
          } else {
            return fs.existsSync(tool.outputPath) && fs.statSync(tool.outputPath).size > 0;
          }
        },
        {
          timeout: 25000,
          timeoutMsg: `Expected output ${tool.outputPath} to be created on disk within 25s`
        }
      );

      // Rigorous content verification
      if (tool.expectedType === 'burst_dir') {
        const files = fs.readdirSync(tool.outputPath).filter(f => f.endsWith('.pdf'));
        expect(files.length).toBeGreaterThan(0);
        for (const file of files) {
          const singleFilePath = path.join(tool.outputPath, file);
          verifyOutputContent(singleFilePath, 'pdf');
        }
      } else if (tool.expectedType === 'batch_dir') {
        const receiptPath = path.join(tool.outputPath, 'execution_receipt.json');
        if (fs.existsSync(receiptPath)) {
          verifyOutputContent(receiptPath, 'json');
          const receipt = JSON.parse(fs.readFileSync(receiptPath, 'utf-8'));
          expect(receipt.success).toBe(true);
          expect(receipt.succeeded).toBeGreaterThan(0);
        }
        const generatedPdfs = fs.readdirSync(tool.outputPath).filter(f => f.endsWith('.pdf'));
        expect(generatedPdfs.length).toBeGreaterThan(0);
        for (const pdf of generatedPdfs) {
          verifyOutputContent(path.join(tool.outputPath, pdf), 'pdf');
        }
      } else {
        verifyOutputContent(tool.outputPath, tool.expectedType);

        // Deep specific assertions based on expected tool outputs
        if (tool.id === 'pdf_compare') {
          const jsonContent = JSON.parse(fs.readFileSync(tool.outputPath, 'utf-8'));
          expect(typeof jsonContent).toBe('object');
          expect(jsonContent).toHaveProperty('diff_detected');
        } else if (tool.id === 'pdf_render') {
          const buffer = fs.readFileSync(tool.outputPath);
          expect(buffer[0]).toBe(0x89);
          expect(buffer.subarray(1, 4).toString('ascii')).toBe('PNG');
        } else if (['pdf_merge_alternate', 'pdf_images_to_pdf', 'pdf_dedup_streams', 'pdf_linearize', 'md_to_pdf', 'html_to_pdf'].includes(tool.id)) {
          const content = fs.readFileSync(tool.outputPath);
          expect(content.subarray(0, 5).toString('ascii')).toBe('%PDF-');
        }
      }

      console.log(`[E2E Batch 8] Successfully verified output content for ${tool.id} at ${tool.outputPath}`);

      // 8. Clean up created output file/folder
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

