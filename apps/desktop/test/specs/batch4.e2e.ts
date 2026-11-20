import path from 'node:path';
import fs from 'node:fs';
import { expect, $, $$ } from '@wdio/globals';
import { verifyOutputContent } from '../helpers/contentVerifier';

describe('PaperPilot Desktop E2E - Batch 4 (10 Security & Privacy Tools)', () => {
  const rootDir = path.resolve(__dirname, '../../../../');
  const singlePagePdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/single_page.pdf');
  const textPiiPdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/text_pii.pdf');
  const encryptedPdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/encrypted.pdf');
  const signedPdfPath = path.resolve(rootDir, 'tests/e2e_fixtures/real/signed.pdf');
  const dummyP12Path = path.resolve(rootDir, 'tests/e2e_fixtures/real/dummy.p12');

  const tools = [
    {
      id: 'pdf_redact',
      title: 'Redact Text',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'text_pii',
      outputPath: '/tmp/desktop_e2e_pdf_redact.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#redactQuery'
    },
    {
      id: 'pdf_encrypt',
      title: 'Encrypt with Password',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_encrypt.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#passwordField'
    },
    {
      id: 'pdf_decrypt',
      title: 'Remove Password',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'encrypted',
      outputPath: '/tmp/desktop_e2e_pdf_decrypt.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#passwordField'
    },
    {
      id: 'pdf_sign',
      title: 'Sign Document',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_sign.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#signCertPath'
    },
    {
      id: 'pdf_validate_signatures',
      title: 'Validate Digital Signatures',
      inputFmt: 'PDF',
      outputFmt: '.JSON',
      fixtureType: 'signed',
      outputPath: '/tmp/desktop_e2e_pdf_validate_signatures.json',
      expectedType: 'json' as const,
      expectedControlSelector: '.field-hint'
    },
    {
      id: 'pdf_remove_signatures',
      title: 'Remove Digital Signatures',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'signed',
      outputPath: '/tmp/desktop_e2e_pdf_remove_signatures.pdf',
      expectedType: 'pdf' as const,
      expectedControlSelector: '.field-hint'
    },
    {
      id: 'pdf_sanitize',
      title: 'Sanitize Document',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_sanitize.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#sanitizeScripts'
    },
    {
      id: 'pdf_change_permissions',
      title: 'Modify PDF Permissions',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_change_permissions.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#permOwnerPassword'
    },
    {
      id: 'pdf_timestamp',
      title: 'Cryptographic Timestamp TSA',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      fixtureType: 'single_page',
      outputPath: '/tmp/desktop_e2e_pdf_timestamp.pdf',
      expectedType: 'pdf' as const,
      expectedControlId: '#tsaServerUrl'
    },
    {
      id: 'document_pii_scan',
      title: 'PII Scanner',
      inputFmt: 'PDF',
      outputFmt: '.JSON',
      fixtureType: 'text_pii',
      outputPath: '/tmp/desktop_e2e_document_pii_scan.json',
      expectedType: 'json' as const,
      expectedControlId: '#piiScanSsn'
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
    console.log('[E2E Batch 4] Switched to All Tools view');
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
      await browser.execute((fType: string, singlePath: string, textPiiPath: string, encPath: string, sigPath: string) => {
        const app = (window as any).appState;
        if (app && typeof app.clearAllDocuments === 'function') {
          app.clearAllDocuments();
        }
        const loadFile = (window as any).__TEST_LOAD_FILE;
        if (typeof loadFile === 'function') {
          if (fType === 'single_page') {
            loadFile('single_page.pdf', singlePath);
          } else if (fType === 'text_pii') {
            loadFile('text_pii.pdf', textPiiPath);
          } else if (fType === 'encrypted') {
            loadFile('encrypted.pdf', encPath);
          } else if (fType === 'signed') {
            loadFile('signed.pdf', sigPath);
          }
        }
      }, tool.fixtureType, singlePagePdfPath, textPiiPdfPath, encryptedPdfPath, signedPdfPath);

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
      if (tool.id === 'pdf_redact') {
        const redactQueryInput = await $('#redactQuery');
        await redactQueryInput.setValue('CONFIDENTIAL');
      } else if (tool.id === 'pdf_encrypt') {
        const passwordInput = await $('#passwordField');
        await passwordInput.setValue('SecretPassword123');
      } else if (tool.id === 'pdf_decrypt') {
        const passwordInput = await $('#passwordField');
        await passwordInput.setValue('Secret123');
      } else if (tool.id === 'pdf_sign') {
        const signCertInput = await $('#signCertPath');
        await signCertInput.setValue(dummyP12Path);
        const signPasswordInput = await $('#signPassword');
        await signPasswordInput.setValue('test');
      } else if (tool.id === 'pdf_change_permissions') {
        const ownerPasswordInput = await $('#permOwnerPassword');
        await ownerPasswordInput.setValue('paperpilot');
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

      // Tool-specific deep assertions
      if (tool.id === 'pdf_encrypt') {
        const content = fs.readFileSync(tool.outputPath);
        expect(content.toString('latin1')).toContain('/Encrypt');
      } else if (tool.id === 'pdf_decrypt') {
        const content = fs.readFileSync(tool.outputPath);
        expect(content.toString('latin1')).not.toContain('/Encrypt');
      } else if (tool.id === 'pdf_validate_signatures') {
        const jsonContent = JSON.parse(fs.readFileSync(tool.outputPath, 'utf-8'));
        expect(typeof jsonContent).toBe('object');
      } else if (tool.id === 'document_pii_scan') {
        const jsonContent = JSON.parse(fs.readFileSync(tool.outputPath, 'utf-8'));
        expect(typeof jsonContent).toBe('object');
      }

      console.log(`[E2E Batch 4] Successfully verified output content for ${tool.id} at ${tool.outputPath}`);

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
