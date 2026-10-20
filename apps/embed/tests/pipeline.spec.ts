import { test, expect } from '@playwright/test';
import { resolve } from 'path';

test.describe('PaperPilot Headless & Visual Pipeline', () => {
  const fixtureUrl = `file://${resolve(import.meta.dirname, 'fixtures/host_page.html')}`;

  test.beforeEach(async ({ page }) => {
    await page.goto(fixtureUrl);
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
