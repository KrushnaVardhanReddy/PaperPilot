<script lang="ts">
  import { pipelineState } from '$lib/state/pipeline.svelte';
  import { appState } from '$lib/state/app.svelte';
  import { toastState } from '$lib/state/toast.svelte';
  import { jobsState } from '$lib/state/jobs.svelte';
  import { invoke } from '@tauri-apps/api/core';

  async function runPipeline() {
    if (pipelineState.steps.length === 0) {
      toastState.error('Pipeline is empty. Add at least one step.');
      return;
    }
    if (appState.documents.length === 0) {
      toastState.error('No input document. Drop a PDF first.');
      return;
    }

    pipelineState.setRunning(true);
    pipelineState.setLastError(null);

    const originalName = appState.documents[0].name.replace(/\.pdf$/i, '');
    let workingPath = appState.documents[0].name;

    try {
      for (let i = 0; i < pipelineState.steps.length; i++) {
        const step = pipelineState.steps[i];
        pipelineState.setCurrentStepIndex(i);

        const outputPath = `${originalName}_step${i + 1}_${step.operationId}.pdf`;

        // Build args: replace __input__ and __output__ tokens
        const args: Record<string, unknown> = {};
        for (const [k, v] of Object.entries(step.params)) {
          if (v === '__input__') args[k] = workingPath;
          else if (v === '__output__') args[k] = outputPath;
          else if (v === '__all_inputs__') args[k] = appState.documents.map(d => d.name);
          else args[k] = v;
        }

        const jobId = jobsState.addJob(step.operationId, 'running', `Pipeline step ${i + 1}`);

        const result = await invoke('invoke_mcp_tool', {
          toolName: step.operationId,
          arguments: args
        }) as { success: boolean; message: string; output_path?: string };

        if (!result.success) {
          jobsState.updateJobStatus(jobId, 'error', result.message);
          pipelineState.setLastError(`Step ${i + 1} (${step.label}) failed: ${result.message}`);
          toastState.error(`Pipeline failed at step ${i + 1}: ${step.label}`);
          return;
        }

        jobsState.updateJobStatus(jobId, 'success', result.message);
        workingPath = result.output_path ?? outputPath;
      }

      pipelineState.setLastSuccessOutput(workingPath);
      toastState.success(`Pipeline complete! Output: ${workingPath}`);
    } catch (err) {
      const msg = typeof err === 'string' ? err : (err as Error).message;
      pipelineState.setLastError(msg);
      toastState.error(`Pipeline error: ${msg}`);
    } finally {
      pipelineState.setRunning(false);
      pipelineState.setCurrentStepIndex(null);
    }
  }
</script>

<div class="pipeline-runner" id="pipeline-runner">
  {#if pipelineState.steps.length > 0}
    <div class="runner-status">
      {#if pipelineState.isRunning}
        <span class="status-running">
          ⏳ Running step {(pipelineState.currentStepIndex ?? 0) + 1} of {pipelineState.steps.length}...
        </span>
      {:else if pipelineState.lastError}
        <span class="status-error">⚠️ {pipelineState.lastError}</span>
      {:else if pipelineState.lastSuccessOutput}
        <span class="status-success">✅ Done → {pipelineState.lastSuccessOutput}</span>
      {/if}
    </div>
  {/if}

  <div class="runner-controls">
    <button
      class="clear-btn"
      id="clear-pipeline-btn"
      onclick={pipelineState.clearPipeline.bind(pipelineState)}
      disabled={pipelineState.isRunning || pipelineState.steps.length === 0}
    >
      Clear
    </button>
    <button
      class="run-btn"
      id="run-pipeline-btn"
      onclick={runPipeline}
      disabled={pipelineState.isRunning || pipelineState.steps.length === 0}
    >
      {pipelineState.isRunning ? 'Running…' : `▶ Run Pipeline (${pipelineState.steps.length} steps)`}
    </button>
  </div>
</div>

<style>
  .pipeline-runner {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    margin-top: 24px;
  }

  .runner-status {
    font-size: 0.95rem;
    padding: 12px 16px;
    border-radius: var(--border-radius-md);
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
  }

  .status-running {
    color: var(--accent-primary);
    font-weight: 600;
  }

  .status-error {
    color: var(--error-color, #ef4444);
    font-weight: 600;
  }

  .status-success {
    color: var(--success-color, #22c55e);
    font-weight: 600;
  }

  .runner-controls {
    display: flex;
    gap: 12px;
  }

  .clear-btn {
    padding: 10px 20px;
    border: 1px solid var(--border-color);
    background: var(--bg-surface);
    color: var(--text-primary);
    border-radius: var(--border-radius-md);
    cursor: pointer;
    font-size: 1rem;
    transition: background 0.15s;
  }

  .clear-btn:hover:not(:disabled) {
    background: var(--bg-surface-hover);
  }

  .clear-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .run-btn {
    padding: 10px 20px;
    border: none;
    background: var(--accent-primary);
    color: #ffffff;
    border-radius: var(--border-radius-md);
    cursor: pointer;
    font-size: 1rem;
    font-weight: 600;
    transition: opacity 0.15s;
  }

  .run-btn:hover:not(:disabled) {
    opacity: 0.9;
  }

  .run-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
