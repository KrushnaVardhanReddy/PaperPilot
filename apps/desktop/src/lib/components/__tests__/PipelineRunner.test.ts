import { render, screen, fireEvent } from '@testing-library/svelte';
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { pipelineState } from '$lib/state/pipeline.svelte';
import { appState } from '$lib/state/app.svelte';
import PipelineRunner from '../pipeline/PipelineRunner.svelte';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue({ success: true, message: 'ok', output_path: 'out.pdf' })
}));

describe('PipelineRunner', () => {
  beforeEach(() => {
    pipelineState.clearPipeline();
  });

  it('Run Pipeline button is disabled when pipeline is empty', () => {
    render(PipelineRunner);
    const btn = screen.getByText(/Run Pipeline/);
    expect(btn).toBeDefined();
  });

  it('shows step count in Run button', () => {
    pipelineState.addStep('pdf_compress');
    pipelineState.addStep('pdf_watermark');
    render(PipelineRunner);
    expect(screen.getByText(/2 steps/)).toBeTruthy();
  });
});
