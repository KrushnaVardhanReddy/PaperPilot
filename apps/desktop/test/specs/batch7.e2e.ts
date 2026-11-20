import path from 'node:path';
import fs from 'node:fs';
import { expect, $, $$ } from '@wdio/globals';
import { verifyOutputContent } from '../helpers/contentVerifier';

describe('PaperPilot Desktop E2E - Batch 7 (10 Document & Office Conversion Tools)', () => {
  const rootDir = path.resolve(__dirname, '../../../../');
  const singlePagePdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/single_page.pdf');
  const financialTablesPdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/financial_tables.pdf');
  const jsonDocPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/json_doc.json');

  const tools = [
    {
      id: 'pdf_to_html',
      title: 'PDF to HTML',
      inputFmt: 'PDF',
      outputFmt: '.HTML',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_to_html.html',
      expectedType: 'html' as const,
      expectedControlSelector: '.operation-config'
    },
    {
      id: 'pdf_to_markdown',
      title: 'PDF to Markdown',
      inputFmt: 'PDF',
      outputFmt: '.MD',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_to_markdown.md',
      expectedType: 'text' as const,
      expectedControlSelector: '.operation-config'
    },
    {
      id: 'pdf_to_json',
      title: 'PDF to JSON',
      inputFmt: 'PDF',
      outputFmt: '.JSON',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_to_json.json',
      expectedType: 'json' as const,
      expectedControlSelector: '.operation-config'
    },
    {
      id: 'json_to_pdf',
      title: 'JSON to PDF',
      inputFmt: 'JSON',
      outputFmt: '.PDF',
      fixtureType: 'json_doc',
      outputPath: '/tmp/desktop_e2e_json_to_pdf.pdf',
      expectedType: 'pdf' as const,
      expectedControlSelector: '.operation-config'
    },
    {
      id: 'pdf_to_epub',
      title: 'PDF to EPUB 3.0',
      inputFmt: 'PDF',
      outputFmt: '.EPUB',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_to_epub.epub',
      expectedType: 'zip' as const,
      expectedControlSelector: '.operation-config'
    },
    {
      id: 'pdf_to_pdf_a',
      title: 'Convert to PDF/A',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_to_pdf_a.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#pdfaConformance'
    },
    {
      id: 'pdf_comic_archive',
      title: 'Comic Archive Conversion',
      inputFmt: 'PDF / Images',
      outputFmt: '.CBZ',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_comic_archive.cbz',
      expectedType: 'zip' as const,
      expectedControlId: '#conversionDpi'
    },
    {
      id: 'pdf_to_docx',
      title: 'PDF to Word (DOCX)',
      inputFmt: 'PDF',
      outputFmt: '.DOCX',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_to_docx.docx',
      expectedType: 'zip' as const,
      expectedControlSelector: '.operation-config'
    },
    {
      id: 'pdf_to_xlsx',
      title: 'PDF to Excel (XLSX)',
      inputFmt: 'PDF',
      outputFmt: '.XLSX',
      fixtureType: 'financial_tables',
      outputPath: '/tmp/desktop_e2e_pdf_to_xlsx.xlsx',
      expectedType: 'zip' as const,
      expectedControlSelector: '.operation-config'
    },
    {
      id: 'pdf_to_pptx',
      title: 'PDF to PowerPoint',
      inputFmt: 'PDF',
      outputFmt: '.PPTX',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_to_pptx.pptx',
      expectedType: 'zip' as const,
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
    console.log('[E2E Batch 7] Switched to All Tools view');
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
          singlePath: string,
          financialPath: string,
          jsonPath: string
        ) => {
          const app = (window as any).appState;
          if (app && typeof app.clearAllDocuments === 'function') {
            app.clearAllDocuments();
          }
          const loadFile = (window as any).__TEST_LOAD_FILE;
          if (typeof loadFile === 'function') {
            if (fType === 'single_page') {
              loadFile('single_page.pdf', singlePath);
            } else if (fType === 'financial_tables') {
              loadFile('financial_tables.pdf', financialPath);
            } else if (fType === 'json_doc') {
              loadFile('json_doc.json', jsonPath);
            }
          }
        },
        tool.fixtureType,
        singlePagePdfPath,
        financialTablesPdfPath,
        jsonDocPath
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
      if (tool.id === 'pdf_to_pdf_a') {
        const conformanceSelect = await $('#pdfaConformance');
        if (await conformanceSelect.isDisplayed()) {
          await conformanceSelect.selectByAttribute('value', 'PDF/A-1b');
        }
      } else if (tool.id === 'pdf_comic_archive') {
        const dpiInput = await $('#conversionDpi');
        if (await dpiInput.isDisplayed()) {
          await dpiInput.setValue('150');
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
      if (tool.id === 'pdf_to_json') {
        const jsonContent = JSON.parse(fs.readFileSync(tool.outputPath, 'utf-8'));
        expect(typeof jsonContent).toBe('object');
      } else if (tool.id === 'pdf_to_html') {
        const htmlContent = fs.readFileSync(tool.outputPath, 'utf-8');
        expect(htmlContent.toLowerCase()).toContain('<html');
      } else if (tool.id === 'pdf_to_markdown') {
        const mdContent = fs.readFileSync(tool.outputPath, 'utf-8');
        expect(mdContent.length).toBeGreaterThan(10);
      } else if (tool.id === 'json_to_pdf' || tool.id === 'pdf_to_pdf_a') {
        const content = fs.readFileSync(tool.outputPath);
        expect(content.subarray(0, 5).toString('ascii')).toBe('%PDF-');
      } else if (['pdf_to_epub', 'pdf_comic_archive', 'pdf_to_docx', 'pdf_to_xlsx', 'pdf_to_pptx'].includes(tool.id)) {
        const buffer = fs.readFileSync(tool.outputPath);
        expect(buffer.subarray(0, 4).toString('ascii')).toBe('PK\x03\x04');
      }

      console.log(`[E2E Batch 7] Successfully verified output content for ${tool.id} at ${tool.outputPath}`);

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
