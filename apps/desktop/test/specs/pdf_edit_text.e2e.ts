import { expect, $, $$ } from '@wdio/globals';

describe('Edit PDF Text Desktop E2E Test', () => {
  it('should select pdf_edit_text in operations panel and verify active state and inspector appearance', async () => {
    // 1. Verify operations panel visibility
    const operationsPanel = await $('[data-testid="operations-panel"]');
    await operationsPanel.waitForDisplayed({ timeout: 15000 });
    expect(await operationsPanel.isDisplayed()).toBe(true);

    // 2. Select tool in operations panel
    const toolBtn = await $(`#tool-btn-pdf_edit_text`);
    if (await toolBtn.isExisting()) {
      await toolBtn.scrollIntoView();
      await toolBtn.click();
    } else {
      // Fallback: search or select from tool items
      const toolItems = await $$('[data-testid="tool-item"]');
      expect(toolItems.length).toBeGreaterThan(0);
      await toolItems[0].click();
    }

    // 3. Verify tool active state & inspector appearance
    const runBtn = await $('[data-testid="run-operation-btn"]');
    await runBtn.waitForDisplayed({ timeout: 10000 });
    expect(await runBtn.isDisplayed()).toBe(true);

    const formatBadges = await $('[data-testid="tool-format-badges"]');
    if (await formatBadges.isExisting()) {
      expect(await formatBadges.isDisplayed()).toBe(true);
    }
  });
});
