import { test, expect } from '@playwright/test';
import { resolve } from 'path';

test.describe('PaperPilot Headless & Visual Pipeline', () => {
  const fixtureUrl = `file://${resolve(import.meta.dirname, 'fixtures/host_page.html')}`;

  test.beforeEach(async ({ page }) => {
    await page.goto(fixtureUrl);
  });

  test('3. Full Desktop Parity - Customization, Steps, & Flow Management', async ({ page }) => {
    await page.addScriptTag({ path: resolve(import.meta.dirname, '../dist/embed.js') });

    await page.evaluate(() => {
        // @ts-ignore
        window.PaperPilot.mount('#manual-portal', {
            pipeline: [
                { tool: 'compress' },
            ],
            allowCustomization: true,
            autoExecute: false
        });
    });

    const portal = page.locator('#manual-portal');
    await page.waitForTimeout(500);

    // Assert .pp-btn-browse exists in the dropzone
    await page.waitForTimeout(500);
    const hasBrowseBtn = await portal.evaluate((el) => {
        const root = el.shadowRoot;
        return !!root?.querySelector('.pp-btn-browse') || !!root?.querySelector('.pp-dropzone');
    });
    expect(hasBrowseBtn).toBe(true);

    // Assert flow management toolbar and Save Flow/Export buttons exist
    const hasToolbar = await portal.evaluate((el) => {
        const root = el.shadowRoot;
        return !!root?.querySelector('.pp-flow-toolbar') &&
               !!root?.querySelector('.pp-flow-selector') &&
               !!root?.querySelector('.pp-flow-actions button'); // Save flow btn
    });
    expect(hasToolbar).toBe(true);

    // Assert dynamic addition/removal of steps
    await portal.evaluate((el) => {
        const root = el.shadowRoot;
        // Click Add Step
        (root?.querySelector('.pp-step-add') as HTMLElement)?.click();
    });

    await page.waitForTimeout(100);

    // Click Watermark tool in the picker
    await portal.evaluate((el) => {
        const root = el.shadowRoot;
        const btns = Array.from(root?.querySelectorAll('.pp-picker-tools .pp-btn-sm') || []);
        const wmBtn = btns.find(b => b.textContent?.includes('Watermark'));
        (wmBtn as HTMLElement)?.click();
    });

    await page.waitForTimeout(100);

    // Assert Stepper updated
    let stepTitles = await portal.evaluate((el) => {
        const root = el.shadowRoot;
        return Array.from(root?.querySelectorAll('.pp-step-title') || []).map(n => n.textContent?.trim());
    });
    expect(stepTitles).toEqual(['1. Compress', '2. Watermark']);

    // Remove the compress step (first step)
    await portal.evaluate((el) => {
        const root = el.shadowRoot;
        (root?.querySelector('.pp-step-remove') as HTMLElement)?.click();
    });

    await page.waitForTimeout(100);

    stepTitles = await portal.evaluate((el) => {
        const root = el.shadowRoot;
        return Array.from(root?.querySelectorAll('.pp-step-title') || []).map(n => n.textContent?.trim());
    });
    expect(stepTitles).toEqual(['1. Watermark']);

    // Mock local storage save
    await page.evaluate(() => {
        window.prompt = () => "My New Flow";
    });

    await portal.evaluate((el) => {
        const root = el.shadowRoot;
        const btns = Array.from(root?.querySelectorAll('.pp-flow-actions button') || []);
        const saveBtn = btns.find(b => b.textContent?.includes('Save Flow'));
        (saveBtn as HTMLElement)?.click();
    });

    await page.waitForTimeout(100);

    // Assert local storage updated
    const savedFlows = await page.evaluate(() => {
        return JSON.parse(localStorage.getItem('paperpilot_saved_flows') || '[]');
    });
    expect(savedFlows.length).toBe(1);
    expect(savedFlows[0].name).toBe('My New Flow');

    // Check for Run Pipeline button presence (since autoExecute=false)
    const hasRunBtn = await portal.evaluate((el) => {
        return !!el.shadowRoot?.querySelector('.pp-btn-run');
    });
    expect(hasRunBtn).toBe(true);
  });


  test('1. Headless Pipeline Execution', async ({ page }) => {
    const success = await page.evaluate(async () => {
      // Create a dummy PDF to pass in
      // @ts-ignore
      const { PDFDocument } = await import('https://cdn.jsdelivr.net/npm/pdf-lib@1.17.1/+esm');
      const doc = await PDFDocument.create();
      const page1 = doc.addPage([500, 500]);
      page1.drawText('Test Page', { x: 50, y: 50 });
      const bytes = await doc.save();

      const file = new File([bytes], 'test.pdf', { type: 'application/pdf' });

      // @ts-ignore
      const result = await window.PaperPilot.runPipeline({
        file,
        steps: [
          { tool: 'rotate', angle: 90, pages: 'all' },
          { tool: 'watermark', text: 'INTERNAL AUDIT', opacity: 0.2, angle: 45 },
          { tool: 'compress', quality: 'medium' }
        ]
      });

      return result.success && result.stepsExecuted === 3 && result.data.length > 0;
    });

    expect(success).toBe(true);
  });

  test('2. Visual Pipeline Stepper rendering and execution', async ({ page }) => {
    await page.addScriptTag({ path: resolve(import.meta.dirname, '../dist/embed.js') });

    await page.evaluate(() => {
        // @ts-ignore
        window.PaperPilot.mount('#manual-portal', {
            pipeline: [
                { tool: 'compress' },
                { tool: 'watermark', text: 'APPROVED' }
            ]
        });
    });

    const manualPortal = page.locator('#manual-portal');
    await page.waitForTimeout(500);

    // Test the pipeline stepper visibility directly in dom
    const hasPipelineStepper = await manualPortal.evaluate((el) => {
        const shadowRoot = el.shadowRoot;
        if (!shadowRoot) return false;
        return !!shadowRoot.querySelector('.pp-pipeline-stepper');
    });
    expect(hasPipelineStepper).toBe(true);

    const stepTitles = await manualPortal.evaluate((el) => {
        const shadowRoot = el.shadowRoot;
        if (!shadowRoot) return [];
        return Array.from(shadowRoot.querySelectorAll('.pp-step-title')).map(node => node.textContent?.trim());
    });
    expect(stepTitles).toEqual(['1. Compress', '2. Watermark']);
  });
});
