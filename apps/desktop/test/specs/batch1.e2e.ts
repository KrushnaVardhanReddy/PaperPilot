import { expect, $, $$ } from '@wdio/globals';

describe('PaperPilot Desktop E2E - Batch 1 (10 Tools)', () => {
  const tools = [
    {
      id: 'pdf_to_gif',
      title: 'Convert PDF to Animated GIF',
      inputFmt: 'PDF',
      outputFmt: '.gif',
      expectedControlId: '#gifDelay'
    },
    {
      id: 'showcase_gif',
      title: 'Generate Showcase GIF',
      inputFmt: 'PDF',
      outputFmt: '.gif',
      expectedControlId: '#showcaseDelay'
    },
    {
      id: 'images_to_gif',
      title: 'Combine Images into Animated GIF',
      inputFmt: 'Images',
      outputFmt: '.gif',
      expectedControlId: '#imgGifDelay'
    },
    {
      id: 'img_deskew',
      title: 'Deskew Document Image',
      inputFmt: 'Image',
      outputFmt: '.png',
      expectedControlSelector: '.checkbox-field'
    },
    {
      id: 'img_crop_to_content',
      title: 'Auto-Crop Image to Content',
      inputFmt: 'IMAGE',
      outputFmt: '.image',
      expectedControlId: '#imgCropThreshold'
    },
    {
      id: 'img_redact',
      title: 'Redact Sensitive Image Regions',
      inputFmt: 'IMAGE',
      outputFmt: '.image',
      expectedControlId: '#imgRedactRects'
    },
    {
      id: 'img_optimize_for_llm',
      title: 'Optimize Image for Vision LLMs',
      inputFmt: 'IMAGE',
      outputFmt: '.image',
      expectedControlId: '#imgOptimizeQuality'
    },
    {
      id: 'img_convert',
      title: 'Convert Image Format',
      inputFmt: 'IMAGE',
      outputFmt: '.image',
      expectedControlId: '#imgConvertFormatSelect'
    },
    {
      id: 'pdf_nup',
      title: 'N-Up Page Imposition',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
      expectedControlId: '#nupOrdering'
    },
    {
      id: 'pdf_rotate',
      title: 'Rotate Document',
      inputFmt: 'PDF',
      outputFmt: '.PDF',
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

  for (const tool of tools) {
    it(`should select, verify format badges, parameters, and run button for ${tool.id} (${tool.title})`, async () => {
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
      const controlSelector = tool.expectedControlId || tool.expectedControlSelector;
      if (controlSelector) {
        const control = await $(controlSelector);
        await control.waitForDisplayed({ timeout: 10000 });
        expect(await control.isDisplayed()).toBe(true);
      }

      // Interactive UI verification where applicable
      if (tool.id === 'img_convert') {
        const select = await $('#imgConvertFormatSelect');
        await select.selectByAttribute('value', 'webp');
        const qualitySlider = await $('#imgConvertQuality');
        await qualitySlider.waitForDisplayed({ timeout: 3000 });
        expect(await qualitySlider.isDisplayed()).toBe(true);
      } else if (tool.id === 'img_deskew') {
        const checkbox = await $('.checkbox-label input[type="checkbox"]');
        await checkbox.click(); // Toggle auto-detect off
        const manualSlider = await $('#imgDeskewManualAngle');
        await manualSlider.waitForDisplayed({ timeout: 3000 });
        expect(await manualSlider.isDisplayed()).toBe(true);
        await checkbox.click(); // Toggle back on
      } else if (tool.id === 'pdf_nup') {
        const ordering = await $('#nupOrdering');
        await ordering.selectByAttribute('value', 'vertical');
        expect(await ordering.getValue()).toBe('vertical');
      } else if (tool.id === 'pdf_rotate') {
        const angleSelect = await $('#rotateAngle');
        await angleSelect.selectByAttribute('value', '180');
        expect(await angleSelect.getValue()).toBe('180');
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

      // Ensure All Tools view is active for next tool
      const allToolsTab = await $('#context-btn-all');
      await allToolsTab.waitForDisplayed({ timeout: 5000 });
    });
  }
});
