import path from 'node:path';
import fs from 'node:fs';
import { expect, $, $$ } from '@wdio/globals';
import { verifyOutputContent } from '../helpers/contentVerifier';

describe('PaperPilot Desktop E2E - Batch 9 (10 Office, Excel & Search/Pipeline Operations)', () => {
  const rootDir = path.resolve(__dirname, '../../../../');
  const singlePagePdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/single_page.pdf');
  const searchTestPdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/search_test.pdf');
  const researchPaperPdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/research_paper.pdf');
  const testXlsxPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/test.xlsx');
  const testDocxPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/test.docx');
  const testXlsPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/test.xls');
  const dataCsvPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/data.csv');

  const tools = [
    {
      id: 'pdf_convert_excel',
      title: 'CSV to PDF',
      inputFmt: 'CSV',
      outputFmt: '.PDF',
      fixtureType: 'data_csv',
      outputPath: '/tmp/desktop_e2e_pdf_convert_excel.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#convertPreset'
    },
    {
      id: 'excel_to_json',
      title: 'Excel to JSON',
      inputFmt: 'XLSX, XLS, CSV',
      outputFmt: '.JSON',
      fixtureType: 'test_xlsx',
      outputPath: '/tmp/desktop_e2e_excel_to_json.json',
      expectedType: 'json' as const,
      expectedControlId: '#excelSheet'
    },
    {
      id: 'excel_to_csv',
      title: 'Excel to CSV',
      inputFmt: 'XLSX, XLS',
      outputFmt: '.CSV',
      fixtureType: 'test_xlsx',
      outputPath: '/tmp/desktop_e2e_excel_to_csv.csv',
      expectedType: 'text' as const,
      expectedControlId: '#excelCsvSheet'
    },
    {
      id: 'office_to_markdown',
      title: 'Convert to Markdown',
      inputFmt: 'DOCX, XLSX, PPTX',
      outputFmt: '.MD',
      fixtureType: 'test_docx',
      outputPath: '/tmp/desktop_e2e_office_to_markdown.md',
      expectedType: 'text' as const,
      expectedControlSelector: '.operation-config'
    },
    {
      id: 'office_extract_text',
      title: 'Office to Plain Text',
      inputFmt: 'DOCX, XLSX, PPTX',
      outputFmt: '.TXT',
      fixtureType: 'test_docx',
      outputPath: '/tmp/desktop_e2e_office_extract_text.txt',
      expectedType: 'text' as const,
      expectedControlSelector: '.operation-config'
    },
    {
      id: 'office_modernize_legacy',
      title: 'Modernize Legacy Office',
      inputFmt: 'DOC, XLS, PPT',
      outputFmt: '.DOCX, .XLSX, .PPTX',
      fixtureType: 'test_xls',
      outputPath: '/tmp/desktop_e2e_office_modernize_legacy.xlsx',
      expectedType: 'zip' as const,
      expectedControlSelector: '.operation-config'
    },
    {
      id: 'workspace_search',
      title: 'Workspace Directory Search',
      inputFmt: 'Workspace',
      outputFmt: '.JSON',
      fixtureType: 'search_test',
      outputPath: '/tmp/desktop_e2e_workspace_search.json',
      expectedType: 'json' as const,
      expectedControlId: '#workspaceSearchQueryInput'
    },
    {
      id: 'semantic_search',
      title: 'Semantic Vector Search',
      inputFmt: 'PDF / Workspace',
      outputFmt: '.JSON',
      fixtureType: 'research_paper',
      outputPath: '/tmp/desktop_e2e_semantic_search.json',
      expectedType: 'json' as const,
      expectedControlId: '#semanticSearchQueryInput'
    },
    {
      id: 'pdf_chunk',
      title: 'Semantic Chunking',
      inputFmt: 'PDF',
      outputFmt: '.JSON',
      fixtureType: 'research_paper',
      outputPath: '/tmp/desktop_e2e_pdf_chunk.json',
      expectedType: 'json' as const,
      expectedControlId: '#chunkTargetTokensInput'
    },
    {
      id: 'document_workflow',
      title: 'Document Workflow Runner',
      inputFmt: '.ppflow',
      outputFmt: '.PDF/.JSON',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_document_workflow.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#workflowPresetSelect'
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
    console.log('[E2E Batch 9] Switched to All Tools view');
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
          singleP: string,
          searchP: string,
          researchP: string,
          xlsxP: string,
          docxP: string,
          xlsP: string,
          csvP: string
        ) => {
          const app = (window as any).appState;
          if (app && typeof app.clearAllDocuments === 'function') {
            app.clearAllDocuments();
          }
          const loadFile = (window as any).__TEST_LOAD_FILE;
          if (typeof loadFile === 'function') {
            if (fType === 'single_page') {
              loadFile('single_page.pdf', singleP);
            } else if (fType === 'search_test') {
              loadFile('search_test.pdf', searchP);
            } else if (fType === 'research_paper') {
              loadFile('research_paper.pdf', researchP);
            } else if (fType === 'test_xlsx') {
              loadFile('test.xlsx', xlsxP);
            } else if (fType === 'test_docx') {
              loadFile('test.docx', docxP);
            } else if (fType === 'test_xls') {
              loadFile('test.xls', xlsP);
            } else if (fType === 'data_csv') {
              loadFile('data.csv', csvP);
            }
          }
        },
        tool.fixtureType,
        singlePagePdfPath,
        searchTestPdfPath,
        researchPaperPdfPath,
        testXlsxPath,
        testDocxPath,
        testXlsPath,
        dataCsvPath
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
      if (tool.id === 'workspace_search') {
        const queryInput = await $('#workspaceSearchQueryInput');
        if (await queryInput.isDisplayed()) {
          await queryInput.setValue('PaperPilot');
        }
      } else if (tool.id === 'semantic_search') {
        const queryInput = await $('#semanticSearchQueryInput');
        if (await queryInput.isDisplayed()) {
          await queryInput.setValue('document');
        }
      } else if (tool.id === 'pdf_convert_excel') {
        const presetSelect = await $('#convertPreset');
        if (await presetSelect.isDisplayed()) {
          await presetSelect.selectByAttribute('value', 'academic');
        }
      } else if (tool.id === 'document_workflow') {
        const presetSelect = await $('#workflowPresetSelect');
        if (await presetSelect.isDisplayed()) {
          await presetSelect.selectByAttribute('value', 'preset-clean-scan');
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
          timeoutMsg: `Expected output ${tool.outputPath} to be created on disk within 25s`
        }
      );

      // Rigorous content verification
      verifyOutputContent(tool.outputPath, tool.expectedType);

      // Deep specific assertions based on expected tool outputs
      if (tool.expectedType === 'pdf') {
        const buffer = fs.readFileSync(tool.outputPath);
        expect(buffer.subarray(0, 5).toString('ascii')).toBe('%PDF-');
      } else if (tool.id === 'excel_to_json') {
        const jsonContent = JSON.parse(fs.readFileSync(tool.outputPath, 'utf-8'));
        expect(typeof jsonContent).toBe('object');
        expect(jsonContent).toHaveProperty('sheets');
        expect(Array.isArray(jsonContent.sheets)).toBe(true);
      } else if (tool.id === 'workspace_search') {
        const jsonContent = JSON.parse(fs.readFileSync(tool.outputPath, 'utf-8'));
        expect(typeof jsonContent).toBe('object');
        expect(jsonContent).toHaveProperty('formats_searched');
      } else if (tool.id === 'semantic_search') {
        const jsonContent = JSON.parse(fs.readFileSync(tool.outputPath, 'utf-8'));
        expect(typeof jsonContent).toBe('object');
        expect(jsonContent).toHaveProperty('results');
        expect(Array.isArray(jsonContent.results)).toBe(true);
      } else if (tool.id === 'pdf_chunk') {
        const jsonContent = JSON.parse(fs.readFileSync(tool.outputPath, 'utf-8'));
        expect(typeof jsonContent).toBe('object');
        expect(jsonContent).toHaveProperty('chunks');
        expect(Array.isArray(jsonContent.chunks)).toBe(true);
      } else if (tool.id === 'excel_to_csv') {
        const textContent = fs.readFileSync(tool.outputPath, 'utf-8');
        expect(textContent.length).toBeGreaterThan(0);
        expect(textContent).toContain(',');
      } else if (tool.id === 'office_to_markdown') {
        const textContent = fs.readFileSync(tool.outputPath, 'utf-8');
        expect(textContent.length).toBeGreaterThan(0);
      } else if (tool.id === 'office_extract_text') {
        const textContent = fs.readFileSync(tool.outputPath, 'utf-8');
        expect(textContent.length).toBeGreaterThan(0);
      } else if (tool.id === 'office_modernize_legacy') {
        // Modernized .xlsx is an OpenXML ZIP archive
        const buffer = fs.readFileSync(tool.outputPath);
        expect(buffer[0]).toBe(0x50);
        expect(buffer[1]).toBe(0x4b);
      }

      console.log(`[E2E Batch 9] Successfully verified output content for ${tool.id} at ${tool.outputPath}`);

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
