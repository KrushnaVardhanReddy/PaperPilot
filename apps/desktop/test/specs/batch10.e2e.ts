import path from 'node:path';
import fs from 'node:fs';
import { expect, $, $$ } from '@wdio/globals';
import { verifyOutputContent } from '../helpers/contentVerifier';

describe('PaperPilot Desktop E2E - Batch 10 (Automation, Hot-Folder Watchers, Direct Conversions & AI Assistants)', () => {
  const rootDir = path.resolve(__dirname, '../../../../');
  const singlePagePdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/single_page.pdf');
  const textPiiPdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/text_pii.pdf');
  const formPdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/form.pdf');
  const batchCsvPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/batch_data.csv');
  const testHtmlPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/test.html');
  const testMdPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/test.md');
  const testPngPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/test.png');
  const watcherTestDir = '/tmp/paperpilot_desktop_hotfolder_test';

  const tools = [
    {
      id: 'recipe_run',
      title: 'Instant Pipeline Recipe',
      inputFmt: '.ppflow',
      outputFmt: '.PDF',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_recipe_run.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#recipeIdSelect'
    },
    {
      id: 'cli_ask',
      title: 'AI Document Assistant',
      inputFmt: 'PDF + Query',
      outputFmt: 'Text / Card',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_cli_ask.txt',
      expectedType: 'text' as const,
      expectedControlId: '#askPromptInput'
    },
    {
      id: 'watcher_start',
      title: 'Start Hot-Folder Watcher',
      inputFmt: 'Folder',
      outputFmt: 'Daemon',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_watcher_start.json',
      expectedType: 'json' as const,
      expectedControlId: '#watcherDirInput'
    },
    {
      id: 'watcher_stop',
      title: 'Stop Hot-Folder Watcher',
      inputFmt: 'Daemon',
      outputFmt: 'Status',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_watcher_stop.json',
      expectedType: 'json' as const,
      expectedControlId: '#watcherIdInput'
    },
    {
      id: 'pdf_convert_html',
      title: 'HTML to PDF Direct',
      inputFmt: 'HTML',
      outputFmt: '.PDF',
      fixtureType: 'html',
      outputPath: '/tmp/desktop_e2e_pdf_convert_html.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#convertPreset'
    },
    {
      id: 'pdf_convert_markdown',
      title: 'Markdown to PDF Direct',
      inputFmt: 'Markdown',
      outputFmt: '.PDF',
      fixtureType: 'markdown',
      outputPath: '/tmp/desktop_e2e_pdf_convert_markdown.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#convertPreset'
    },
    {
      id: 'cli_fill_batch',
      title: 'Automated Batch Engine',
      inputFmt: 'PDF + CSV',
      outputFmt: '.ZIP/.PDF',
      fixtureType: 'form_batch',
      outputPath: '/tmp/desktop_e2e_cli_fill_batch_dir',
      expectedType: 'batch_dir' as const,
      expectedControlId: '#batchCsvPath'
    },
    {
      id: 'img_to_pdf',
      title: 'Images to PDF',
      inputFmt: 'JPG, PNG, WebP',
      outputFmt: '.PDF',
      fixtureType: 'png_image',
      outputPath: '/tmp/desktop_e2e_img_to_pdf.pdf',
      expectedType: 'pdf' as const,
      expectedControlSelector: '.operation-config'
    },
    {
      id: 'pdf_extract_patterns',
      title: 'Extract Keywords & Patterns',
      inputFmt: 'PDF',
      outputFmt: '.JSON',
      fixtureType: 'text_pii',
      outputPath: '/tmp/desktop_e2e_pdf_extract_patterns.json',
      expectedType: 'json' as const,
      expectedControlId: '#extractPatternsKeywords'
    },
    {
      id: 'pdf_document_pii_scan',
      title: 'PII Scanner',
      inputFmt: 'PDF',
      outputFmt: '.JSON',
      fixtureType: 'text_pii',
      outputPath: '/tmp/desktop_e2e_pdf_document_pii_scan.json',
      expectedType: 'json' as const,
      expectedControlId: '#piiScanSsn'
    }
  ];

  before(async () => {
    // Ensure test hot-folder directory exists
    if (!fs.existsSync(watcherTestDir)) {
      fs.mkdirSync(watcherTestDir, { recursive: true });
    }

    const sidebar = await $('[data-testid="sidebar"]');
    await sidebar.waitForDisplayed({ timeout: 20000 });
    expect(await sidebar.isDisplayed()).toBe(true);

    const operationsPanel = await $('[data-testid="operations-panel"]');
    await operationsPanel.waitForDisplayed({ timeout: 15000 });
    expect(await operationsPanel.isDisplayed()).toBe(true);

    // Dismiss onboarding modal if present
    const skipBtn = await $('button=Skip for Now');
    if (await skipBtn.isDisplayed()) {
      await skipBtn.click();
      await browser.pause(300);
    }
  });

  afterEach(async () => {
    try {
      // Return to directory view if still in operation view
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
          piiP: string,
          formP: string,
          htmlP: string,
          mdP: string,
          pngP: string
        ) => {
          const app = (window as any).appState;
          if (app && typeof app.clearAllDocuments === 'function') {
            app.clearAllDocuments();
          }
          const loadFile = (window as any).__TEST_LOAD_FILE;
          if (typeof loadFile === 'function') {
            if (fType === 'single_page') {
              loadFile('single_page.pdf', singleP);
            } else if (fType === 'text_pii') {
              loadFile('text_pii.pdf', piiP);
            } else if (fType === 'form_batch') {
              loadFile('form.pdf', formP);
            } else if (fType === 'html') {
              loadFile('test.html', htmlP);
            } else if (fType === 'markdown') {
              loadFile('test.md', mdP);
            } else if (fType === 'png_image') {
              loadFile('test.png', pngP);
            }
          }
        },
        tool.fixtureType,
        singlePagePdfPath,
        textPiiPdfPath,
        formPdfPath,
        testHtmlPath,
        testMdPath,
        testPngPath
      );

      await browser.pause(300);

      // Ensure All Tools view is active
      const allToolsTab = await $('#context-btn-all');
      if (await allToolsTab.isDisplayed()) {
        await allToolsTab.click();
        await browser.pause(200);
      }

      // 1. Locate tool button directly in directory by id or alias class
      const toolBtn = await $(`#tool-btn-${tool.id}, .tool-btn-${tool.id}`);
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
      if (tool.id === 'recipe_run') {
        const recipeSelect = await $('#recipeIdSelect');
        if (await recipeSelect.isDisplayed()) {
          await recipeSelect.selectByAttribute('value', 'preset-clean-scan');
        }
      } else if (tool.id === 'cli_ask') {
        const promptInput = await $('#askPromptInput');
        if (await promptInput.isDisplayed()) {
          await promptInput.setValue('rotate 90 degrees');
        }
      } else if (tool.id === 'watcher_start') {
        const watcherDirInput = await $('#watcherDirInput');
        if (await watcherDirInput.isDisplayed()) {
          await watcherDirInput.setValue(watcherTestDir);
        }
      } else if (tool.id === 'watcher_stop') {
        const watcherIdInput = await $('#watcherIdInput');
        if (await watcherIdInput.isDisplayed()) {
          await watcherIdInput.setValue('default');
        }
      } else if (tool.id === 'pdf_convert_html' || tool.id === 'pdf_convert_markdown') {
        const presetSelect = await $('#convertPreset');
        if (await presetSelect.isDisplayed()) {
          await presetSelect.selectByAttribute('value', 'academic');
        }
      } else if (tool.id === 'cli_fill_batch') {
        const batchCsvInput = await $('#batchCsvPath');
        if (await batchCsvInput.isDisplayed()) {
          await batchCsvInput.setValue(batchCsvPath);
        }
      } else if (tool.id === 'pdf_extract_patterns') {
        const keywordsInput = await $('#extractPatternsKeywords');
        if (await keywordsInput.isDisplayed()) {
          await keywordsInput.setValue('CONFIDENTIAL,PII,Invoice');
        }
      } else if (tool.id === 'pdf_document_pii_scan') {
        const ssnCheckbox = await $('#piiScanSsn');
        if (await ssnCheckbox.isDisplayed() && !(await ssnCheckbox.isSelected())) {
          await ssnCheckbox.click();
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
          if (tool.expectedType === 'batch_dir') {
            const receiptPath = path.join(tool.outputPath, 'execution_receipt.json');
            return (
              fs.existsSync(tool.outputPath) &&
              fs.existsSync(receiptPath) &&
              fs.statSync(receiptPath).size > 0
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
      if (tool.expectedType === 'batch_dir') {
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
        if (tool.expectedType === 'pdf') {
          const buffer = fs.readFileSync(tool.outputPath);
          expect(buffer.subarray(0, 5).toString('ascii')).toBe('%PDF-');
        } else if (tool.id === 'cli_ask') {
          const textContent = fs.readFileSync(tool.outputPath, 'utf-8');
          expect(textContent.length).toBeGreaterThan(0);
          expect(
            textContent.toLowerCase().includes('plan') ||
            textContent.toLowerCase().includes('rotate') ||
            textContent.toLowerCase().includes('intent')
          ).toBe(true);
        } else if (tool.id === 'watcher_start') {
          const jsonContent = JSON.parse(fs.readFileSync(tool.outputPath, 'utf-8'));
          expect(typeof jsonContent).toBe('object');
          expect(jsonContent.success).toBe(true);
          expect(jsonContent).toHaveProperty('watcher_id');
        } else if (tool.id === 'watcher_stop') {
          const jsonContent = JSON.parse(fs.readFileSync(tool.outputPath, 'utf-8'));
          expect(typeof jsonContent).toBe('object');
          expect(jsonContent.success).toBe(true);
        } else if (tool.id === 'pdf_extract_patterns') {
          const jsonContent = JSON.parse(fs.readFileSync(tool.outputPath, 'utf-8'));
          expect(typeof jsonContent).toBe('object');
          expect(jsonContent).toHaveProperty('total_matches');
          expect(jsonContent).toHaveProperty('findings');
          expect(Array.isArray(jsonContent.findings)).toBe(true);
        } else if (tool.id === 'pdf_document_pii_scan') {
          const jsonContent = JSON.parse(fs.readFileSync(tool.outputPath, 'utf-8'));
          expect(typeof jsonContent).toBe('object');
          expect(jsonContent).toHaveProperty('total_findings');
          const findings = jsonContent.findings || jsonContent.entities;
          expect(Array.isArray(findings)).toBe(true);
          expect(findings.length).toBeGreaterThan(0);
        }
      }

      console.log(`[E2E Batch 10] Successfully verified output content for ${tool.id} at ${tool.outputPath}`);

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
