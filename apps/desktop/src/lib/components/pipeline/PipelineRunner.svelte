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

    // Initial working files from all loaded documents in appState
    let workingFiles: string[] = appState.documents.map((d, idx) => appState.documentPaths[idx] || d.name);
    console.log('[PipelineRunner] Initial working files:', workingFiles);

    try {
      for (let i = 0; i < pipelineState.steps.length; i++) {
        const step = pipelineState.steps[i];
        pipelineState.setCurrentStepIndex(i);

        const nextWorkingFiles: string[] = [];
        const takesAllInputs = Object.values(step.params).includes('__all_inputs__');

        if (takesAllInputs) {
          // Multi-file aggregation operation (e.g. Merge PDFs into single output)
          const firstPath = workingFiles[0] || 'document.pdf';
          const docDir = firstPath.includes('/') ? firstPath.substring(0, firstPath.lastIndexOf('/')) : '';
          const fileName = `merged_step${i + 1}_${step.operationId}.pdf`;
          const outputPath = docDir ? `${docDir}/${fileName}` : fileName;

          const args: Record<string, unknown> = {};
          for (const [k, v] of Object.entries(step.params)) {
            if (v === '__all_inputs__') args[k] = workingFiles;
            else if (v === '__input__') args[k] = workingFiles[0];
            else if (v === '__output__') args[k] = outputPath;
            else args[k] = v;
          }

          const label = `Pipeline step ${i + 1} (${step.label} on ${workingFiles.length} files)`;
          const jobId = jobsState.addJob(step.operationId, 'running', label);

          if (appState.zeroTrace) {
            args._zero_trace = true;
          }
          const result = await invoke('invoke_mcp_tool', {
            toolName: step.operationId,
            arguments: args
          }) as {
            success: boolean;
            message: string;
            output_path?: string;
            data?: { files?: string[] };
          };

          if (!result.success) {
            jobsState.updateJobStatus(jobId, 'error', result.message);
            pipelineState.setLastError(`Step ${i + 1} (${step.label}) failed: ${result.message}`);
            toastState.error(`Pipeline failed at step ${i + 1}: ${step.label}`);
            return;
          }

          jobsState.updateJobStatus(jobId, 'success', result.message);
          nextWorkingFiles.push(result.output_path ?? outputPath);
        } else {
          // Batch execution: run this step across all files currently in the pipeline
          for (let fileIdx = 0; fileIdx < workingFiles.length; fileIdx++) {
            const currentInput = workingFiles[fileIdx];
            const docDir = currentInput.includes('/') ? currentInput.substring(0, currentInput.lastIndexOf('/')) : '';
            const baseName = currentInput.includes('/')
              ? currentInput.substring(currentInput.lastIndexOf('/') + 1).replace(/\.pdf$/i, '')
              : currentInput.replace(/\.pdf$/i, '');

            const fileSuffix = workingFiles.length > 1 && baseName === 'part' ? `_${fileIdx + 1}` : '';
            const fileName = `${baseName}${fileSuffix}_step${i + 1}_${step.operationId}.pdf`;
            const dirName = `${baseName}${fileSuffix}_step${i + 1}_${step.operationId}_dir`;
            const outputPath = docDir ? `${docDir}/${fileName}` : fileName;
            const outputDir = docDir ? `${docDir}/${dirName}` : dirName;

            // Build args: replace __input__ and __output__ tokens
            const args: Record<string, unknown> = {};
            for (const [k, v] of Object.entries(step.params)) {
              if (v === '__input__') args[k] = currentInput;
              else if (v === '__output__') args[k] = outputPath;
              else if (v === '__output_dir__') args[k] = outputDir;
              else if (v === '__all_inputs__') args[k] = workingFiles;
              else args[k] = v;
            }

            const label = workingFiles.length > 1
              ? `Step ${i + 1}: ${step.label} (${fileIdx + 1}/${workingFiles.length})`
              : `Step ${i + 1}: ${step.label}`;
            const jobId = jobsState.addJob(step.operationId, 'running', label);

            if (appState.zeroTrace) {
              args._zero_trace = true;
            }
            const result = await invoke('invoke_mcp_tool', {
              toolName: step.operationId,
              arguments: args
            }) as {
              success: boolean;
              message: string;
              output_path?: string;
              data?: { files?: string[] };
            };

            if (!result.success) {
              jobsState.updateJobStatus(jobId, 'error', result.message);
              pipelineState.setLastError(`Step ${i + 1} (${step.label}) failed on ${baseName}: ${result.message}`);
              toastState.error(`Pipeline failed at step ${i + 1}: ${step.label}`);
              return;
            }

            jobsState.updateJobStatus(jobId, 'success', result.message);

            if (result.data?.files && Array.isArray(result.data.files) && result.data.files.length > 0) {
              // Fan-out: add all generated split files to next pipeline stage
              nextWorkingFiles.push(...result.data.files);
            } else {
              const resolved = result.output_path ?? outputPath;
              if (resolved.endsWith('_dir')) {
                nextWorkingFiles.push(`${resolved}/split_1.pdf`);
              } else {
                nextWorkingFiles.push(resolved);
              }
            }
          }
        }

        workingFiles = nextWorkingFiles;
      }

      const finalSummary = workingFiles.length === 1
        ? workingFiles[0]
        : `${workingFiles.length} files generated (${workingFiles[0]}, ...)`;
      pipelineState.setLastSuccessOutput(finalSummary);
      toastState.success(`Pipeline complete! Output: ${finalSummary}`);
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
      {#if pipelineState.isRunning}
        Running…
      {:else if appState.documents.length > 1}
        ▶ Run Pipeline ({pipelineState.steps.length} steps · {appState.documents.length} files)
      {:else}
        ▶ Run Pipeline ({pipelineState.steps.length} steps)
      {/if}
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
