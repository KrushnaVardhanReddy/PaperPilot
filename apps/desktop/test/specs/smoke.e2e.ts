import { expect, $, $$ } from '@wdio/globals';

describe('PaperPilot Desktop Smoke Test', () => {
  it('should launch the desktop app and verify presence of UI components', async () => {
    // 1. Verify window exists and page loads
    const sidebar = await $('[data-testid="sidebar"]');
    await sidebar.waitForDisplayed({ timeout: 20000 });
    expect(await sidebar.isDisplayed()).toBe(true);
    console.log('[E2E Smoke] Verified sidebar presence');

    // 2. Verify operations panel visibility
    const operationsPanel = await $('[data-testid="operations-panel"]');
    await operationsPanel.waitForDisplayed({ timeout: 15000 });
    expect(await operationsPanel.isDisplayed()).toBe(true);
    console.log('[E2E Smoke] Verified operations panel presence');

    // 3. Verify tool items are rendered in tool directory
    const toolItems = await $$('[data-testid="tool-item"]');
    console.log(`[E2E Smoke] Found ${toolItems.length} tool items rendered`);
    expect(toolItems.length).toBeGreaterThan(0);

    // 4. Click the first tool to open inspector and verify run operation button
    await toolItems[0].click();
    console.log('[E2E Smoke] Selected first tool to inspect');

    const runBtn = await $('[data-testid="run-operation-btn"]');
    await runBtn.waitForDisplayed({ timeout: 10000 });
    expect(await runBtn.isDisplayed()).toBe(true);
    console.log('[E2E Smoke] Verified run button presence in tool inspector');
  });
});
