import { expect, $, $$ } from '@wdio/globals';

describe('PaperPilot Desktop E2E - Batch 2 (10 Tools)', () => {
  const tools = [
    {
      id: 'pdf_merge',
      title: 'Merge Documents',
      inputFmt: '2+ PDFs',
      outputFmt: '.PDF',
      expectedControlSelector: '.empty-list'
    },
    {
      id: 'pdf_split',
      title: 'Split PDF',
      inputFmt: 'PDF',
      outputFmt: 'Multiple .PDFs',
      expectedControlId: '#splitPoints'
    },
    {
      id: 'pdf_delete_pages',
      title: 'Delete Pages',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      expectedControlId: '#deletePages'
    },
    {
      id: 'pdf_extract_pages',
      title: 'Extract PDF Pages',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      expectedControlId: '#extractPages'
    },
    {
      id: 'pdf_reorder_pages',
      title: 'Reorder Pages',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      expectedControlId: '#reorderList'
    },
    {
      id: 'pdf_compress',
      title: 'Compress PDF',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      expectedControlId: '#compressQuality'
    },
    {
      id: 'pdf_crop',
      title: 'Crop Pages',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      expectedControlId: '#cropBox'
    },
    {
      id: 'pdf_scale_pages',
      title: 'Scale Page Dimensions',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      expectedControlId: '#scaleTargetSize'
    },
    {
      id: 'pdf_booklet',
      title: 'Booklet Imposition',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      expectedControlId: '#bookletBindingEdge'
    },
    {
      id: 'pdf_posterize',
      title: 'Multi-Sheet Poster Tile Slicer',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      expectedControlId: '#posterGrid'
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
    console.log('[E2E Batch 2] Switched to All Tools view');
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
    it(`should select, verify format badges, parameters, and run button for ${tool.id} (${tool.title})`, async () => {
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

      // Verify the inspector param container matches the tool id
      const inspectorParam = await $(`#inspector-param-${tool.id.replace('pdf_', '')}`);
      await inspectorParam.waitForDisplayed({ timeout: 5000 });
      expect(await inspectorParam.isDisplayed()).toBe(true);

      // 4. Verify Format Badges
      const formatBadges = await $('[data-testid="tool-format-badges"]');
      await formatBadges.waitForDisplayed({ timeout: 5000 });
      expect(await formatBadges.isDisplayed()).toBe(true);

      const inputBadge = await $('.format-badge.input-badge .badge-value');
      await inputBadge.waitForDisplayed({ timeout: 5000 });
      expect(await inputBadge.isDisplayed()).toBe(true);

      const outputBadge = await $('.format-badge.output-badge .badge-value');
      await outputBadge.waitForDisplayed({ timeout: 5000 });
      expect(await outputBadge.isDisplayed()).toBe(true);

      // 5. Verify Tool-specific Parameter Controls
      if (tool.id === 'pdf_delete_pages') {
        const destructiveWarning = await $('[data-testid="destructive-warning"]');
        await destructiveWarning.waitForDisplayed({ timeout: 5000 });
        expect(await destructiveWarning.isDisplayed()).toBe(true);
      }

      const controlSelector = tool.expectedControlId || tool.expectedControlSelector;
      if (controlSelector) {
        const control = await $(controlSelector);
        await control.waitForDisplayed({ timeout: 10000 });
        expect(await control.isDisplayed()).toBe(true);
      }

      // Interactive UI verification where applicable
      if (tool.id === 'pdf_split') {
        const splitInput = await $('#splitPoints');
        await splitInput.setValue('1-2, 3-5');
        expect(await splitInput.getValue()).toBe('1-2, 3-5');
      } else if (tool.id === 'pdf_delete_pages') {
        const deleteInput = await $('#deletePages');
        await deleteInput.setValue('2, 4');
        expect(await deleteInput.getValue()).toBe('2, 4');
      } else if (tool.id === 'pdf_extract_pages') {
        const extractInput = await $('#extractPages');
        await extractInput.setValue('1, 3');
        expect(await extractInput.getValue()).toBe('1, 3');
      } else if (tool.id === 'pdf_reorder_pages') {
        const reorderInput = await $('#reorderList');
        await reorderInput.setValue('3,2,1');
        expect(await reorderInput.getValue()).toBe('3,2,1');
      } else if (tool.id === 'pdf_crop') {
        const cropInput = await $('#cropBox');
        await cropInput.setValue('10,10,400,600');
        expect(await cropInput.getValue()).toBe('10,10,400,600');
      } else if (tool.id === 'pdf_scale_pages') {
        const scaleSelect = await $('#scaleTargetSize');
        await scaleSelect.selectByAttribute('value', 'Letter');
        expect(await scaleSelect.getValue()).toBe('Letter');
      } else if (tool.id === 'pdf_booklet') {
        const bookletSelect = await $('#bookletBindingEdge');
        await bookletSelect.selectByAttribute('value', 'right');
        expect(await bookletSelect.getValue()).toBe('right');
      } else if (tool.id === 'pdf_posterize') {
        const posterSelect = await $('#posterGrid');
        await posterSelect.selectByAttribute('value', '3x3');
        expect(await posterSelect.getValue()).toBe('3x3');
      }

      // 6. Verify Run Operation Button is rendered and inspectable
      const runBtn = await $('[data-testid="run-operation-btn"]');
      await runBtn.waitForDisplayed({ timeout: 5000 });
      expect(await runBtn.isDisplayed()).toBe(true);

      // 7. Return to tool directory for subsequent tool tests
      const backBtn = await $('#btn-back-tools');
      await backBtn.waitForDisplayed({ timeout: 5000 });
      await backBtn.click();
      await browser.pause(500);
    });
  }
});
