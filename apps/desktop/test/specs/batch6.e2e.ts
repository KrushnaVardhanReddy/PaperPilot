import path from 'node:path';
import fs from 'node:fs';
import { expect, $, $$ } from '@wdio/globals';
import { verifyOutputContent } from '../helpers/contentVerifier';

describe('PaperPilot Desktop E2E - Batch 6 (10 Visual, Layout & Structural Operations)', () => {
  const rootDir = path.resolve(__dirname, '../../../../');
  const twoUpBookPdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/two_up_book.pdf');
  const multiPagePdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/multi_page.pdf');
  const singlePagePdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/single_page.pdf');
  const stationeryPdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/stationery.pdf');
  const withBlankPdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/with_blank_pages.pdf');
  const rotatedLandscapePdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/rotated_landscape.pdf');
  const malformedPdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/malformed.pdf');
  const sampleDataCsvPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/data.csv');

  const tools = [
    {
      id: 'pdf_split_scanned_book',
      title: 'Split Scanned Book Spine',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'two_up_book',
      outputPath: '/tmp/desktop_e2e_pdf_split_scanned_book.pdf',
      expectedType: 'pdf' as const,
      expectedControlSelector: '.operation-config'
    },
    {
      id: 'pdf_split_by_outline',
      title: 'Split by Document Outline',
      inputFmt: 'PDF',
      outputFmt: '.ZIP',
      fixtureType: 'multi_page',
      outputPath: '/tmp/desktop_e2e_pdf_split_by_outline.zip',
      expectedType: 'zip' as const,
      expectedControlSelector: '.operation-config'
    },
    {
      id: 'pdf_bookmarks',
      title: 'Extract Bookmarks',
      inputFmt: 'PDF',
      outputFmt: '.JSON',
      fixtureType: 'multi_page',
      outputPath: '/tmp/desktop_e2e_pdf_bookmarks.json',
      expectedType: 'json' as const,
      expectedControlSelector: '.section-desc'
    },
    {
      id: 'pdf_attachments',
      title: 'PDF Portfolio Attachments',
      inputFmt: 'PDF',
      outputFmt: '.JSON',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_attachments.json',
      expectedType: 'json' as const,
      expectedControlSelector: '.segmented'
    },
    {
      id: 'pdf_overlay_underlay',
      title: 'Overlay & Underlay Stationary',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_overlay_underlay.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#stationeryPath'
    },
    {
      id: 'pdf_stitch_canvas',
      title: 'Stitch Continuous Canvas',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'multi_page',
      outputPath: '/tmp/desktop_e2e_pdf_stitch_canvas.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#stitchSpacing'
    },
    {
      id: 'pdf_invert_colors',
      title: 'Invert Colors (Dark Mode)',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_invert_colors.pdf',
      expectedType: 'pdf' as const,
      expectedControlSelector: '.operation-config'
    },
    {
      id: 'pdf_remove_blank',
      title: 'Remove Blank Pages',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'with_blank',
      outputPath: '/tmp/desktop_e2e_pdf_remove_blank.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#blankSensitivity'
    },
    {
      id: 'pdf_auto_rotate',
      title: 'Auto Upright Orientation',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'rotated_landscape',
      outputPath: '/tmp/desktop_e2e_pdf_auto_rotate.pdf',
      expectedType: 'pdf' as const,
      expectedControlSelector: '.section-desc'
    },
    {
      id: 'pdf_repair',
      title: 'Repair PDF',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'malformed',
      outputPath: '/tmp/desktop_e2e_pdf_repair.pdf',
      expectedType: 'pdf' as const,
      expectedControlSelector: '.operation-config'
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
    console.log('[E2E Batch 6] Switched to All Tools view');
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
      await browser.execute(
        (
          fType: string,
          twoUpPath: string,
          multiPath: string,
          singlePath: string,
          withBlankPath: string,
          rotatedPath: string,
          malformedPath: string
        ) => {
          const app = (window as any).appState;
          if (app && typeof app.clearAllDocuments === 'function') {
            app.clearAllDocuments();
          }
          const loadFile = (window as any).__TEST_LOAD_FILE;
          if (typeof loadFile === 'function') {
            if (fType === 'two_up_book') {
              loadFile('two_up_book.pdf', twoUpPath);
            } else if (fType === 'multi_page') {
              loadFile('multi_page.pdf', multiPath);
            } else if (fType === 'single_page') {
              loadFile('single_page.pdf', singlePath);
            } else if (fType === 'with_blank') {
              loadFile('with_blank_pages.pdf', withBlankPath);
            } else if (fType === 'rotated_landscape') {
              loadFile('rotated_landscape.pdf', rotatedPath);
            } else if (fType === 'malformed') {
              loadFile('malformed.pdf', malformedPath);
            }
          }
        },
        tool.fixtureType,
        twoUpBookPdfPath,
        multiPagePdfPath,
        singlePagePdfPath,
        withBlankPdfPath,
        rotatedLandscapePdfPath,
        malformedPdfPath
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

      // Configure tool specific options
      if (tool.id === 'pdf_overlay_underlay') {
        const stationeryInput = await $('#stationeryPath');
        await stationeryInput.setValue(stationeryPdfPath);
      } else if (tool.id === 'pdf_stitch_canvas') {
        const stitchSpacingInput = await $('#stitchSpacing');
        await stitchSpacingInput.setValue('0');
      } else if (tool.id === 'pdf_remove_blank') {
        const blankSensitivityInput = await $('#blankSensitivity');
        await blankSensitivityInput.setValue('95');
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
          return fs.existsSync(tool.outputPath) && fs.statSync(tool.outputPath).size > 0;
        },
        {
          timeout: 25000,
          timeoutMsg: `Expected output file ${tool.outputPath} to be created on disk within 25s`
        }
      );

      // Rigorous content verification
      verifyOutputContent(tool.outputPath, tool.expectedType);

      // Deep specific assertions based on expected tool outputs
      if (tool.id === 'pdf_bookmarks' || tool.id === 'pdf_attachments') {
        const jsonContent = JSON.parse(fs.readFileSync(tool.outputPath, 'utf-8'));
        expect(typeof jsonContent).toBe('object');
      } else if (tool.id === 'pdf_split_by_outline') {
        const buffer = fs.readFileSync(tool.outputPath);
        expect(buffer.subarray(0, 4).toString('ascii')).toBe('PK\x03\x04');
      } else {
        const content = fs.readFileSync(tool.outputPath);
        expect(content.subarray(0, 5).toString('ascii')).toBe('%PDF-');
      }

      console.log(`[E2E Batch 6] Successfully verified output content for ${tool.id} at ${tool.outputPath}`);

      // 8. Clean up created output file
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
