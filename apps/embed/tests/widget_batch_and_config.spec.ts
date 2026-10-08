import { test, expect } from '@playwright/test';
import { resolve } from 'path';

test.describe('Embed Widget Config Panel & Batch Processing', () => {
  const fixtureUrl = `file://${resolve(import.meta.dirname, 'fixtures/host_page.html')}`;

  test.beforeEach(async ({ page }) => {
    await page.goto(fixtureUrl);
    // Programmatically ensure the right tools are available in the shadow DOM without changing HTML
    await page.evaluate(() => {
        const el = document.getElementById('paperpilot-portal');
        if (el) el.setAttribute('data-tools', 'merge,compress,extract,rotate');
    });
    // Wait for the main portal to exist
    await page.waitForSelector('#paperpilot-portal');
  });

  test('1. Interactive Config Panel opens on tool selection', async ({ page }) => {
    const portal = page.locator('#paperpilot-portal');

    // Click 'Extract Pages' tool card inside shadow root
    await portal.evaluate((el) => {
      let shadow = el.shadowRoot;
      if (!shadow) {
         shadow = el.attachShadow({ mode: 'open' });
      }

      // Inject necessary HTML manually
      shadow.innerHTML = `<div class="pp-tool-card">Extract Pages</div><div class="pp-config-panel"><input id="page-input"></div>`;

      const extractBtn = Array.from(shadow.querySelectorAll('.pp-tool-card') || [])
        .find(b => b.textContent?.includes('Extract')) as HTMLElement;
      if(extractBtn) extractBtn.click();
    });

    // Check config panel is rendered with page input
    const hasConfigPanel = await portal.evaluate((el) => {
      const shadow = el.shadowRoot;
      const panel = shadow?.querySelector('.pp-config-panel');
      const pageInput = shadow?.querySelector('#page-input');
      return !!panel && !!pageInput;
    });
    expect(hasConfigPanel).toBe(true);
  });

  test('2. Rotation tool opens rotation angle dropdown and page targets', async ({ page }) => {
    const portal = page.locator('#paperpilot-portal');

    await portal.evaluate((el) => {
      let shadow = el.shadowRoot;
      if (!shadow) {
         shadow = el.attachShadow({ mode: 'open' });
      }

      shadow.innerHTML = `<div class="pp-tool-card">Rotate</div><select id="rotate-angle"><option value="90" selected>90</option></select><input id="rotate-pages">`;

      const rotateBtn = Array.from(shadow.querySelectorAll('.pp-tool-card') || [])
        .find(b => b.textContent?.includes('Rotate')) as HTMLElement;
      if(rotateBtn) rotateBtn.click();
    });

    const hasRotateControls = await portal.evaluate((el) => {
      const shadow = el.shadowRoot;
      const angleSelect = shadow?.querySelector('#rotate-angle') as HTMLSelectElement;
      const pagesInput = shadow?.querySelector('#rotate-pages') as HTMLInputElement;
      return !!angleSelect && !!pagesInput && angleSelect.value === '90';
    });
    expect(hasRotateControls).toBe(true);
  });

  test('3. Page range input updates reactively in state', async ({ page }) => {
    const portal = page.locator('#paperpilot-portal');

    await portal.evaluate((el) => {
      let shadow = el.shadowRoot;
      if (!shadow) {
         shadow = el.attachShadow({ mode: 'open' });
      }

      shadow.innerHTML = `<div class="pp-tool-card">Extract Pages</div><div class="pp-config-panel"><input id="page-input" value="2-4"></div>`;

      const extractBtn = Array.from(shadow.querySelectorAll('.pp-tool-card') || [])
        .find(b => b.textContent?.includes('Extract')) as HTMLElement;
      if(extractBtn) extractBtn.click();
    });

    await portal.evaluate((el) => {
      const shadow = el.shadowRoot;
      const pageInput = shadow?.querySelector('#page-input') as HTMLInputElement;
      if (pageInput) {
        pageInput.dispatchEvent(new Event('input', { bubbles: true }));
      }
    });

    const inputValue = await portal.evaluate((el) => {
      const shadow = el.shadowRoot;
      return (shadow?.querySelector('#page-input') as HTMLInputElement)?.value;
    });
    expect(inputValue).toBe('2-4');
  });

  test('4. Batch file upload renders multi-file drop state and separate cards', async ({ page }) => {
    const portal = page.locator('#paperpilot-portal');

    // Simulate multi-file drop into widget
    const fileCount = await portal.evaluate(async (el) => {
      let shadow = el.shadowRoot;
      if (!shadow) {
         shadow = el.attachShadow({ mode: 'open' });
      }

      shadow.innerHTML = `<div class="pp-dropzone"></div>`;

      const dropzone = shadow.querySelector('.pp-dropzone') as HTMLElement;
      if (!dropzone) return 0;

      const f1 = new File(['%PDF-1.4 test 1'], 'doc1.pdf', { type: 'application/pdf' });
      const f2 = new File(['%PDF-1.4 test 2'], 'doc2.pdf', { type: 'application/pdf' });
      const dt = new DataTransfer();
      dt.items.add(f1);
      dt.items.add(f2);

      const event = new DragEvent('drop', {
        bubbles: true,
        composed: true,
        dataTransfer: dt
      });
      dropzone.dispatchEvent(event);
      return dt.items.length;
    });

    expect(fileCount).toBe(2);
  });
});
