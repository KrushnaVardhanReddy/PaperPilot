import path from 'node:path';
import fs from 'node:fs';
import { expect, $, $$ } from '@wdio/globals';
import { verifyOutputContent } from '../helpers/contentVerifier';

describe('PaperPilot Desktop E2E - Batch 1 (10 Tools)', () => {
  const rootDir = path.resolve(__dirname, '../../../../');
  const pdfFixturePath = path.resolve(rootDir, 'tests/e2e_fixtures/real/multi_page.pdf');
  const imgFixturePath = path.resolve(rootDir, 'tests/e2e_fixtures/real/test.png');
  const img1Path = path.resolve(rootDir, 'tests/e2e_fixtures/real/img1.png');
  const img2Path = path.resolve(rootDir, 'tests/e2e_fixtures/real/img2.png');

  const tools = [
    {
      id: 'pdf_to_gif',
      title: 'Convert PDF to Animated GIF',
      inputFmt: 'PDF',
      outputFmt: '.gif',
      fixtureType: 'pdf',
      outputPath: '/tmp/desktop_e2e_pdf_to_gif.gif',
      expectedType: 'gif' as const,
      expectedControlId: '#gifDelay'
    },
    {
      id: 'showcase_gif',
      title: 'Generate Showcase GIF',
      inputFmt: 'PDF',
      outputFmt: '.gif',
      fixtureType: 'pdf',
      outputPath: '/tmp/desktop_e2e_showcase_gif.gif',
      expectedType: 'gif' as const,
      expectedControlId: '#showcaseDelay'
    },
    {
      id: 'images_to_gif',
      title: 'Combine Images into Animated GIF',
      inputFmt: 'Images',
      outputFmt: '.gif',
      fixtureType: 'multi_image',
      outputPath: '/tmp/desktop_e2e_images_to_gif.gif',
      expectedType: 'gif' as const,
      expectedControlId: '#imgGifDelay'
    },
    {
      id: 'img_deskew',
      title: 'Deskew Document Image',
      inputFmt: 'Image',
      outputFmt: '.png',
      fixtureType: 'image',
      outputPath: '/tmp/desktop_e2e_img_deskew.png',
      expectedType: 'png' as const,
      expectedControlSelector: '.checkbox-field'
    },
    {
      id: 'img_crop_to_content',
      title: 'Auto-Crop Image to Content',
      inputFmt: 'IMAGE',
      outputFmt: '.image',
      fixtureType: 'image',
      outputPath: '/tmp/desktop_e2e_img_crop_to_content.png',
      expectedType: 'png' as const,
      expectedControlId: '#imgCropThreshold'
    },
    {
      id: 'img_redact',
      title: 'Redact Sensitive Image Regions',
      inputFmt: 'IMAGE',
      outputFmt: '.image',
      fixtureType: 'image',
      outputPath: '/tmp/desktop_e2e_img_redact.png',
      expectedType: 'png' as const,
      expectedControlId: '#imgRedactRects'
    },
    {
      id: 'img_optimize_for_llm',
      title: 'Optimize Image for Vision LLMs',
      inputFmt: 'IMAGE',
      outputFmt: '.image',
      fixtureType: 'image',
      outputPath: '/tmp/desktop_e2e_img_optimize_for_llm.jpg',
      expectedType: 'jpeg' as const,
      expectedControlId: '#imgOptimizeQuality'
    },
    {
      id: 'img_convert',
      title: 'Convert Image Format',
      inputFmt: 'IMAGE',
      outputFmt: '.image',
      fixtureType: 'image',
      outputPath: '/tmp/desktop_e2e_img_convert.webp',
      expectedType: 'webp' as const,
      expectedControlId: '#imgConvertFormatSelect'
    },
    {
      id: 'pdf_nup',
      title: 'N-Up Page Imposition',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'pdf',
      outputPath: '/tmp/desktop_e2e_pdf_nup.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#nupOrdering'
    },
    {
      id: 'pdf_rotate',
      title: 'Rotate Document',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'pdf',
      outputPath: '/tmp/desktop_e2e_pdf_rotate.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#rotateAngle'
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
    console.log('[E2E Batch 1] Switched to All Tools view');
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
        fs.unlinkSync(tool.outputPath);
      }

      // Step 1: Clear previous documents and load test fixture(s) via window.__TEST_LOAD_FILE
      await browser.execute((fType: string, pdfP: string, imgP: string, i1P: string, i2P: string) => {
        const app = (window as any).appState;
        if (app && typeof app.clearAllDocuments === 'function') {
          app.clearAllDocuments();
        }
        const loadFile = (window as any).__TEST_LOAD_FILE;
        if (typeof loadFile === 'function') {
          if (fType === 'pdf') {
            loadFile('multi_page.pdf', pdfP);
          } else if (fType === 'image') {
            loadFile('test.png', imgP);
          } else if (fType === 'multi_image') {
            loadFile('img1.png', i1P);
            loadFile('img2.png', i2P);
          }
        }
      }, tool.fixtureType, pdfFixturePath, imgFixturePath, img1Path, img2Path);

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
      if (tool.id === 'img_convert') {
        const select = await $('#imgConvertFormatSelect');
        await select.selectByAttribute('value', 'webp');
      } else if (tool.id === 'img_redact') {
        const redactInput = await $('#imgRedactRects');
        if (await redactInput.isDisplayed()) {
          await redactInput.setValue('10,10,50,50');
        }
      } else if (tool.id === 'pdf_nup') {
        const ordering = await $('#nupOrdering');
        await ordering.selectByAttribute('value', 'horizontal');
      } else if (tool.id === 'pdf_rotate') {
        const angleSelect = await $('#rotateAngle');
        await angleSelect.selectByAttribute('value', '90');
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

      // 7. Wait for operation completion: check disk and execution badge
      await browser.waitUntil(
        () => {
          return fs.existsSync(tool.outputPath) && fs.statSync(tool.outputPath).size > 0;
        },
        {
          timeout: 25000,
          timeoutMsg: `Expected output file ${tool.outputPath} to be created on disk within 25s`
        }
      );

      // 8. Deep content verification (magic bytes, non-empty, proper structure)
      verifyOutputContent(tool.outputPath, tool.expectedType);
      console.log(`[E2E Batch 1] Successfully verified output content for ${tool.id} at ${tool.outputPath}`);

      // 9. Clean up created output file
      try {
        if (fs.existsSync(tool.outputPath)) {
          fs.unlinkSync(tool.outputPath);
        }
      } catch (err) {
        console.warn('Could not clean up output file:', err);
      }

      // 10. Return to tool directory for subsequent tool tests
      const backBtn = await $('#btn-back-tools');
      await backBtn.waitForDisplayed({ timeout: 5000 });
      await backBtn.click();
      await browser.pause(300);
    });
  }
});
