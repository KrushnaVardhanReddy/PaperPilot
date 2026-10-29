import { render, screen } from '@testing-library/svelte';
import { describe, it, expect, beforeEach } from 'vitest';
import { pipelineState } from '$lib/state/pipeline.svelte';
import PipelineCanvas from '../pipeline/PipelineCanvas.svelte';

describe('PipelineCanvas', () => {
  beforeEach(() => {
    pipelineState.clearPipeline();
  });

  it('renders the Add Step button when empty', () => {
    render(PipelineCanvas);
    expect(screen.getByText('+ Add Step')).toBeTruthy();
  });

  it('renders step cards for each step in state', () => {
    pipelineState.addStep('pdf_compress');
    pipelineState.addStep('pdf_watermark');
    render(PipelineCanvas);
    // There are 2 step cards plus the "+ Add Step" button
    expect(screen.getAllByText(/Step/)).toHaveLength(3);
  });

  it('renders condition badge and TRUE connector when condition is present', async () => {
    pipelineState.addStep('pdf_compress');
    pipelineState.addStep('pdf_watermark');

    // add condition to second step
    const step2 = pipelineState.steps[1];
    pipelineState.setStepCondition(step2.id, { if_scanned: true }, 'skip');

    render(PipelineCanvas);

    // Check for connector badge
    expect(screen.getByText('[TRUE ➔]')).toBeTruthy();
  });

});
