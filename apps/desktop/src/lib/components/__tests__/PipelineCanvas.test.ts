import { render, screen } from '@testing-library/svelte';
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { pipelineState } from '$lib/state/pipeline.svelte';
import PipelineCanvas from '../pipeline/PipelineCanvas.svelte';

describe('PipelineCanvas', () => {
  beforeEach(() => {
    pipelineState.clearPipeline();
    // Mock matchMedia for SvelteFlow
    Object.defineProperty(window, 'matchMedia', {
      writable: true,
      value: vi.fn().mockImplementation((query: string) => ({
        matches: false,
        media: query,
        onchange: null,
        addListener: vi.fn(), // deprecated
        removeListener: vi.fn(), // deprecated
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
      })),
    });

    // Mock ResizeObserver for SvelteFlow
    global.ResizeObserver = class ResizeObserver {
      observe() {}
      unobserve() {}
      disconnect() {}
    };
  });

  it('renders the SvelteFlow canvas container', () => {
    const { container } = render(PipelineCanvas);
    expect(container.querySelector('.pipeline-canvas-container')).toBeTruthy();
  });

  it('renders node elements for each step in state', () => {
    pipelineState.addStep('pdf_compress');
    pipelineState.addStep('pdf_watermark');
    const { container } = render(PipelineCanvas);
    expect(pipelineState.steps.length).toBe(2);
  });

  it('selects a step when selectStep is called', () => {
    pipelineState.addStep('pdf_compress');
    const step = pipelineState.steps[0];
    pipelineState.selectStep(null);
    expect(pipelineState.selectedStepId).toBeNull();

    pipelineState.selectStep(step.id);
    expect(pipelineState.selectedStepId).toBe(step.id);
  });

  it('renders properties in StepConfigPanel and updates parameter values', async () => {
    const { default: StepConfigPanel } = await import('../pipeline/StepConfigPanel.svelte');
    const { fireEvent } = await import('@testing-library/svelte');
    pipelineState.addStep('pdf_compress');
    const step = pipelineState.steps[0];
    pipelineState.selectStep(step.id);

    const { container } = render(StepConfigPanel);
    const select = container.querySelector('select');
    expect(select).toBeTruthy();
    expect(select?.value).toBe('medium');

    // Simulate changing compression level
    if (select) {
      await fireEvent.change(select, { target: { value: 'high' } });
      expect(pipelineState.steps[0].params.level).toBe('high');
    }
  });

  it('renders clean fallback message for operations without params', async () => {
    const { default: StepConfigPanel } = await import('../pipeline/StepConfigPanel.svelte');
    pipelineState.addStep('pdf_linearize');
    const step = pipelineState.steps[0];
    pipelineState.selectStep(step.id);

    const { container } = render(StepConfigPanel);
    expect(container.querySelector('.no-params-card')).toBeTruthy();
  });

  it('keeps action tool identity when execution condition is set', () => {
    pipelineState.addStep('pdf_rotate');
    const step = pipelineState.steps[0];
    pipelineState.steps[0].condition = { if_page_count_gt: 2 };

    const { container } = render(PipelineCanvas);
    expect(pipelineState.steps[0].operationId).toBe('pdf_rotate');
    expect(container.querySelector('.pipeline-canvas-container')).toBeTruthy();
  });

  it('organizes node positions sequentially on autoLayout', () => {
    pipelineState.addStep('pdf_rotate');
    pipelineState.addStep('pdf_compress');
    pipelineState.autoLayout();

    expect(pipelineState.steps[0].position?.x).toBe(320); // 60 + 1 * 260
    expect(pipelineState.steps[1].position?.x).toBe(580); // 60 + 2 * 260
  });

  it('renders condition badge when condition is present', async () => {
    pipelineState.addStep('pdf_compress');
    pipelineState.addStep('pdf_watermark');

    // add condition to second step
    const step2 = pipelineState.steps[1];
    pipelineState.setStepCondition(step2.id, { if_scanned: true }, 'skip');

    render(PipelineCanvas);

    // Check for node condition badge
    expect(screen.getByText(/IF: is scanned/)).toBeTruthy();
  });

});
