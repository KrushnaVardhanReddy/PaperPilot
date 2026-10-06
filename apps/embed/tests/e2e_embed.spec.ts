import { test, expect } from '@playwright/test';
import { resolve } from 'path';

test.describe('PaperPilot Embedded Widget', () => {
  const fixtureUrl = `file://${resolve(import.meta.dirname, 'fixtures/host_page.html')}`;

  test.beforeEach(async ({ page }) => {
    await page.goto(fixtureUrl);
  });

  test('1. Shadow DOM Encapsulation', async ({ page }) => {
    // The widget is mounted in #paperpilot-portal
    const portal = page.locator('#paperpilot-portal');

    // Evaluate the computed style inside the shadow root, not the host element
    const isHostRed = await portal.evaluate((el) => {
        return window.getComputedStyle(el).color === 'rgb(255, 0, 0)';
    });
    expect(isHostRed).toBe(true); // Host is adversarial

    const widgetColor = await portal.evaluate((el) => {
        const shadowRoot = el.shadowRoot;
        if (!shadowRoot) return null;
        const widget = shadowRoot.querySelector('.pp-widget');
        if (!widget) return null;
        return window.getComputedStyle(widget).color;
    });

    // #e2e8f0 in dark mode theme
    expect(widgetColor).toBe('rgb(226, 232, 240)');
    expect(widgetColor).not.toBe('rgb(255, 0, 0)');

    // Check custom property override from data-brand-color
    const brandColorVar = await portal.evaluate((el) => {
        const shadowRoot = el.shadowRoot;
        if (!shadowRoot) return null;
        const widget = shadowRoot.querySelector('.pp-widget');
        if (!widget) return null;
        return window.getComputedStyle(widget).getPropertyValue('--pp-brand-color').trim();
    });

    expect(brandColorVar).toBe('#ff00ff');
  });

  test('2. Tool Filtering', async ({ page }) => {
    const portal = page.locator('#paperpilot-portal');

    const toolCount = await portal.evaluate((el) => {
        const shadowRoot = el.shadowRoot;
        if (!shadowRoot) return 0;
        return shadowRoot.querySelectorAll('.pp-tool-card').length;
    });

    expect(toolCount).toBe(2);

    const toolNames = await portal.evaluate((el) => {
        const shadowRoot = el.shadowRoot;
        if (!shadowRoot) return [];
        return Array.from(shadowRoot.querySelectorAll('.pp-tool-card')).map(node => node.textContent?.trim());
    });

    expect(toolNames).toContain('Merge PDF');
    expect(toolNames).toContain('Compress PDF');
  });

  test('3. Badge Visibility (on by default)', async ({ page }) => {
    const portal = page.locator('#paperpilot-portal');

    const hasBadge = await portal.evaluate((el) => {
        const shadowRoot = el.shadowRoot;
        if (!shadowRoot) return false;
        return !!shadowRoot.querySelector('.pp-badge');
    });

    expect(hasBadge).toBe(true);
  });

  test('4. Badge Hidden', async ({ page }) => {
    const noBadgePortal = page.locator('#nobadge-portal');

    const hasBadge = await noBadgePortal.evaluate((el) => {
        const shadowRoot = el.shadowRoot;
        if (!shadowRoot) return true; // Fail safe
        return !!shadowRoot.querySelector('.pp-badge');
    });

    expect(hasBadge).toBe(false);
  });

  test('5. Programmatic Mount/Unmount', async ({ page }) => {
    // Mount programmatically
    await page.evaluate(() => {
        // @ts-ignore
        window.PaperPilot.mount('#manual-portal', {
            tools: ['split'],
            theme: 'light',
            brandColor: '#00ff00'
        });
    });

    const manualPortal = page.locator('#manual-portal');

    const shadowRootExists = await manualPortal.evaluate((el) => {
        return !!el.shadowRoot;
    });
    expect(shadowRootExists).toBe(true);

    const toolNames = await manualPortal.evaluate((el) => {
        const shadowRoot = el.shadowRoot;
        if (!shadowRoot) return [];
        return Array.from(shadowRoot.querySelectorAll('.pp-tool-card')).map(node => node.textContent?.trim());
    });
    expect(toolNames).toEqual(['Split PDF']);

    // Unmount
    await page.evaluate(() => {
        // @ts-ignore
        window.PaperPilot.unmount('#manual-portal');
    });

    const isUnmounted = await manualPortal.evaluate((el) => {
        return el.shadowRoot?.innerHTML === '';
    });
    expect(isUnmounted).toBe(true);
  });
});
