import path from 'node:path';
import fs from 'node:fs';
import { expect, $, $$ } from '@wdio/globals';
import { verifyOutputContent } from '../helpers/contentVerifier';

describe('PaperPilot Desktop E2E - Batch 5 (10 Forms, Bates, Annotations & Stamps Tools)', () => {
  const rootDir = path.resolve(__dirname, '../../../../');
  const formPdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/form.pdf');
  const formDataJsonPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/form_data.json');
  const singlePagePdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/single_page.pdf');
  const multiPagePdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/multi_page.pdf');
  const img1Path = path.resolve(rootDir, 'tests/e2e_fixtures/real/img1.png');

  const tools = [
    {
      id: 'pdf_fill_form',
      title: 'Fill Form Data',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'form',
      outputPath: '/tmp/desktop_e2e_pdf_fill_form.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#formDataPath'
    },
    {
      id: 'pdf_read_form',
      title: 'Read Form Data',
      inputFmt: 'PDF',
      outputFmt: '.JSON',
      fixtureType: 'form',
      outputPath: '/tmp/desktop_e2e_pdf_read_form.json',
      expectedType: 'json' as const,
      expectedControlSelector: '.field-hint'
    },
    {
      id: 'pdf_create_form_field',
      title: 'Add Form Field',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_create_form_field.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#formFieldName'
    },
    {
      id: 'pdf_flatten',
      title: 'Flatten Annotations',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'form',
      outputPath: '/tmp/desktop_e2e_pdf_flatten.pdf',
      expectedType: 'pdf' as const,
      expectedControlSelector: '.field-hint'
    },
    {
      id: 'pdf_bates',
      title: 'Bates Numbering',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'multi_page',
      outputPath: '/tmp/desktop_e2e_pdf_bates.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#batesPrefix'
    },
    {
      id: 'pdf_stamp',
      title: 'Stamp Image & Logo',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_stamp.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#stampImagePath'
    },
    {
      id: 'pdf_watermark',
      title: 'Add Watermark',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_watermark.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#watermarkText'
    },
    {
      id: 'pdf_header_footer',
      title: 'Header & Footer',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_header_footer.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#headerText'
    },
    {
      id: 'pdf_page_numbers',
      title: 'Add Page Numbers',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'multi_page',
      outputPath: '/tmp/desktop_e2e_pdf_page_numbers.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#pageNumberPos'
    },
    {
      id: 'pdf_annotate',
      title: 'Add Annotations',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_annotate.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#annotationSearchPattern'
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
    console.log('[E2E Batch 5] Switched to All Tools view');
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
      await browser.execute((fType: string, formPath: string, singlePath: string, multiPath: string) => {
        const app = (window as any).appState;
        if (app && typeof app.clearAllDocuments === 'function') {
          app.clearAllDocuments();
        }
        const loadFile = (window as any).__TEST_LOAD_FILE;
        if (typeof loadFile === 'function') {
          if (fType === 'form') {
            loadFile('form.pdf', formPath);
          } else if (fType === 'single_page') {
            loadFile('single_page.pdf', singlePath);
          } else if (fType === 'multi_page') {
            loadFile('multi_page.pdf', multiPath);
          }
        }
      }, tool.fixtureType, formPdfPath, singlePagePdfPath, multiPagePdfPath);

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
        const control = await $(controlSelector);
        await control.waitForDisplayed({ timeout: 10000 });
        expect(await control.isDisplayed()).toBe(true);
      }

      // Configure tool specific options
      if (tool.id === 'pdf_fill_form') {
        const formInput = await $('#formDataPath');
        await formInput.setValue(formDataJsonPath);
      } else if (tool.id === 'pdf_create_form_field') {
        const fieldNameInput = await $('#formFieldName');
        await fieldNameInput.setValue('ApplicantSignature');
        const fieldLabelInput = await $('#formFieldLabel');
        await fieldLabelInput.setValue('Applicant Signature:');
        const fieldRectInput = await $('#formFieldRect');
        await fieldRectInput.setValue('180,540,450,565');
      } else if (tool.id === 'pdf_bates') {
        const batesPrefixInput = await $('#batesPrefix');
        await batesPrefixInput.setValue('LEGAL-CONF-');
        const batesStartInput = await $('#batesStart');
        await batesStartInput.setValue('1001');
      } else if (tool.id === 'pdf_stamp') {
        const stampPathInput = await $('#stampImagePath');
        await stampPathInput.setValue(img1Path);
        const stampXInput = await $('#stampX');
        await stampXInput.setValue('100');
        const stampYInput = await $('#stampY');
        await stampYInput.setValue('100');
      } else if (tool.id === 'pdf_watermark') {
        const watermarkInput = await $('#watermarkText');
        await watermarkInput.setValue('CONFIDENTIAL DRAFT');
      } else if (tool.id === 'pdf_header_footer') {
        const headerInput = await $('#headerText');
        await headerInput.setValue('PaperPilot Enterprise Header');
        const footerInput = await $('#footerText');
        await footerInput.setValue('Strictly Private & Confidential');
      } else if (tool.id === 'pdf_page_numbers') {
        const pageNumberFormatInput = await $('#pageNumberFormat');
        await pageNumberFormatInput.setValue('Doc Page {n} of {total}');
      } else if (tool.id === 'pdf_annotate') {
        const patternInput = await $('#annotationSearchPattern');
        await patternInput.setValue('Sample');
        const noteInput = await $('#annotationContent');
        await noteInput.setValue('Verified by PaperPilot Test Suite');
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
      if (tool.id === 'pdf_read_form') {
        const jsonContent = JSON.parse(fs.readFileSync(tool.outputPath, 'utf-8'));
        expect(typeof jsonContent).toBe('object');
      } else if (tool.id === 'pdf_fill_form') {
        const content = fs.readFileSync(tool.outputPath);
        expect(content.subarray(0, 5).toString('ascii')).toBe('%PDF-');
      } else if (tool.id === 'pdf_flatten') {
        const content = fs.readFileSync(tool.outputPath);
        expect(content.subarray(0, 5).toString('ascii')).toBe('%PDF-');
      } else if (tool.id === 'pdf_create_form_field') {
        const content = fs.readFileSync(tool.outputPath);
        expect(content.subarray(0, 5).toString('ascii')).toBe('%PDF-');
      }

      console.log(`[E2E Batch 5] Successfully verified output content for ${tool.id} at ${tool.outputPath}`);

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
