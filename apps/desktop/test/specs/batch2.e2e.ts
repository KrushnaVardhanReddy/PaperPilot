import path from 'node:path';
import fs from 'node:fs';
import { expect, $, $$ } from '@wdio/globals';
import { verifyOutputContent } from '../helpers/contentVerifier';

describe('PaperPilot Desktop E2E - Batch 2 (10 Tools)', () => {
  const rootDir = path.resolve(__dirname, '../../../../');
  const pdfFixturePath = path.resolve(rootDir, 'tests/e2e_fixtures/real/multi_page.pdf');
  const mergeAPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/merge_a.pdf');
  const mergeBPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/merge_b.pdf');

  const tools = [
    {
      id: 'pdf_merge',
      title: 'Merge Documents',
      inputFmt: '2+ PDFs',
      outputFmt: '.PDF',
      fixtureType: 'merge',
      outputPath: '/tmp/desktop_e2e_pdf_merge.pdf',
      expectedType: 'pdf' as const,
      expectedControlSelector: '.reorder-list'
    },
    {
      id: 'pdf_split',
      title: 'Split PDF',
      inputFmt: 'PDF',
      outputFmt: 'Multiple .PDFs',
      fixtureType: 'pdf',
      outputPath: '/tmp/desktop_e2e_pdf_split_dir',
      expectedType: 'pdf_dir' as const,
      expectedControlId: '#splitPoints'
    },
    {
      id: 'pdf_delete_pages',
      title: 'Delete Pages',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'pdf',
      outputPath: '/tmp/desktop_e2e_pdf_delete_pages.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#deletePages'
    },
    {
      id: 'pdf_extract_pages',
      title: 'Extract PDF Pages',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'pdf',
      outputPath: '/tmp/desktop_e2e_pdf_extract_pages.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#extractPages'
    },
    {
      id: 'pdf_reorder_pages',
      title: 'Reorder Pages',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'pdf',
      outputPath: '/tmp/desktop_e2e_pdf_reorder_pages.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#reorderList'
    },
    {
      id: 'pdf_compress',
      title: 'Compress PDF',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'pdf',
      outputPath: '/tmp/desktop_e2e_pdf_compress.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#compressQuality'
    },
    {
      id: 'pdf_crop',
      title: 'Crop Pages',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'pdf',
      outputPath: '/tmp/desktop_e2e_pdf_crop.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#cropBox'
    },
    {
      id: 'pdf_scale_pages',
      title: 'Scale Page Dimensions',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'pdf',
      outputPath: '/tmp/desktop_e2e_pdf_scale_pages.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#scaleTargetSize'
    },
    {
      id: 'pdf_booklet',
      title: 'Booklet Imposition',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'pdf',
      outputPath: '/tmp/desktop_e2e_pdf_booklet.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#bookletBindingEdge'
    },
    {
      id: 'pdf_posterize',
      title: 'Multi-Sheet Poster Tile Slicer',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'pdf',
      outputPath: '/tmp/desktop_e2e_pdf_posterize.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#posterGrid'
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
    console.log('[E2E Batch 2] Switched to All Tools view');
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
      await browser.execute((fType: string, pdfP: string, mA: string, mB: string) => {
        const app = (window as any).appState;
        if (app && typeof app.clearAllDocuments === 'function') {
          app.clearAllDocuments();
        }
        const loadFile = (window as any).__TEST_LOAD_FILE;
        if (typeof loadFile === 'function') {
          if (fType === 'pdf') {
            loadFile('multi_page.pdf', pdfP);
          } else if (fType === 'merge') {
            loadFile('merge_a.pdf', mA);
            loadFile('merge_b.pdf', mB);
          }
        }
      }, tool.fixtureType, pdfFixturePath, mergeAPath, mergeBPath);

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

      // 4. Verify Tool-specific Parameter Controls
      if (tool.id === 'pdf_delete_pages') {
        const destructiveWarning = await $('[data-testid="destructive-warning"]');
        await destructiveWarning.waitForDisplayed({ timeout: 5000 });
        expect(await destructiveWarning.isDisplayed()).toBe(true);
      }

      const controlSelector = tool.expectedControlId || tool.expectedControlSelector;
      if (controlSelector) {
        const control = await $(controlSelector);
        await control.waitForDisplayed({ timeout: 10000 });
        expect(await control.isDisplayed()).toBe(true);
      }

      // Configure parameter inputs where applicable
      if (tool.id === 'pdf_split') {
        const splitInput = await $('#splitPoints');
        await splitInput.setValue('1-2, 3');
      } else if (tool.id === 'pdf_delete_pages') {
        const deleteInput = await $('#deletePages');
        await deleteInput.setValue('2');
      } else if (tool.id === 'pdf_extract_pages') {
        const extractInput = await $('#extractPages');
        await extractInput.setValue('1-2');
      } else if (tool.id === 'pdf_reorder_pages') {
        const reorderInput = await $('#reorderList');
        await reorderInput.setValue('5,4,3,2,1');
      } else if (tool.id === 'pdf_crop') {
        const cropInput = await $('#cropBox');
        await cropInput.setValue('10,10,300,400');
      } else if (tool.id === 'pdf_scale_pages') {
        const scaleSelect = await $('#scaleTargetSize');
        await scaleSelect.selectByAttribute('value', 'A4');
      } else if (tool.id === 'pdf_booklet') {
        const bookletSelect = await $('#bookletBindingEdge');
        await bookletSelect.selectByAttribute('value', 'left');
      } else if (tool.id === 'pdf_posterize') {
        const posterSelect = await $('#posterGrid');
        await posterSelect.selectByAttribute('value', '2x2');
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
      if (tool.expectedType === 'pdf_dir') {
        await browser.waitUntil(
          () => {
            if (!fs.existsSync(tool.outputPath)) return false;
            const files = fs.readdirSync(tool.outputPath).filter(f => f.endsWith('.pdf'));
            return files.length > 0;
          },
          {
            timeout: 25000,
            timeoutMsg: `Expected output directory ${tool.outputPath} with generated PDFs within 25s`
          }
        );

        const files = fs.readdirSync(tool.outputPath).filter(f => f.endsWith('.pdf'));
        expect(files.length).toBeGreaterThan(0);
        for (const file of files) {
          const splitFilePath = path.join(tool.outputPath, file);
          verifyOutputContent(splitFilePath, 'pdf');
        }
        console.log(`[E2E Batch 2] Successfully verified split output PDFs in ${tool.outputPath}`);
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
        console.log(`[E2E Batch 2] Successfully verified output content for ${tool.id} at ${tool.outputPath}`);
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
