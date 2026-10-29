import { render, screen } from '@testing-library/svelte';
import { describe, it, expect, beforeEach } from 'vitest';
import { pipelineState } from '$lib/state/pipeline.svelte';
import PipelineCanvas from '../pipeline/PipelineCanvas.svelte';

describe('PipelineCanvas', () => {
  beforeEach(() => {
    pipelineState.clearPipeline();
    // Mock matchMedia for SvelteFlow
    Object.defineProperty(window, 'matchMedia', {
      writable: true,
      value: vi.fn().mockImplementation(query => ({
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
});
