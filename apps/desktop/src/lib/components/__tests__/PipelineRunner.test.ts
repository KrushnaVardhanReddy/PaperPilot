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

  it('shows multi-file count in Run button when multiple documents are loaded', () => {
    pipelineState.addStep('pdf_compress');
    pipelineState.addStep('pdf_watermark');
    const f1 = new File(['1'], 'doc1.pdf', { type: 'application/pdf' });
    const f2 = new File(['2'], 'doc2.pdf', { type: 'application/pdf' });
    const f3 = new File(['3'], 'doc3.pdf', { type: 'application/pdf' });
    appState.addDocuments([f1, f2, f3]);

    render(PipelineRunner);
    expect(screen.getByText(/2 steps · 3 files/)).toBeTruthy();
  });
});
