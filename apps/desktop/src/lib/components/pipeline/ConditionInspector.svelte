<script lang="ts">
  import { pipelineState } from '$lib/state/pipeline.svelte';
  import type { WorkflowCondition, OnNoMatch } from '$lib/state/pipeline.svelte';
  import ConditionNode from './ConditionNode.svelte';
  import { validateCondition } from '$lib/utils/workflowConditions';
  let selectedStep = $derived(pipelineState.steps.find(s => s.id === pipelineState.selectedStepId));
  let draft = $state<WorkflowCondition | undefined>();
  let onNoMatch = $state<OnNoMatch>('skip');
  let snapshot = '';
  let stepId = '';
  let error = $derived(draft ? validateCondition(draft) : null);
  let saved = $state(false);
  $effect(() => {
    const serialized = JSON.stringify(selectedStep?.condition ?? null);
    if (stepId !== selectedStep?.id || snapshot !== serialized) {
      stepId = selectedStep?.id ?? ''; snapshot = serialized;
      draft = selectedStep?.condition ? JSON.parse(serialized) : undefined;
      onNoMatch = selectedStep?.on_no_match ?? 'skip'; saved = false;
    }
  });
  function apply() {
    if (!selectedStep || error || pipelineState.isRunning) return;
    snapshot = JSON.stringify(draft ?? null);
    if (draft) pipelineState.setStepCondition(selectedStep.id, JSON.parse(snapshot), onNoMatch);
    else pipelineState.removeStepCondition(selectedStep.id);
    saved = true;
  }
</script>

{#if selectedStep}
  <section class="condition-panel" id="step-condition-panel" aria-label="Step condition editor">
    <fieldset disabled={pipelineState.isRunning}>
    <div class="header"><strong>Step condition</strong>
      <button type="button" onclick={() => { draft = undefined; apply(); }}>Clear condition</button>
    </div>
    {#if draft}
      <ConditionNode node={draft} onchange={node => { draft = node; saved = false; }} />
      <label>If conditions do not match
        <select aria-label="On no match" bind:value={onNoMatch} onchange={() => saved = false}>
          <option value="skip">Skip step and continue</option><option value="archive_original">Return original and stop</option><option value="error">Fail workflow</option>
        </select>
      </label>
    {:else}<p>This step runs without a condition.</p><button type="button" onclick={() => { draft = { if_scanned: true }; saved = false; }}>+ Add condition</button>{/if}
    {#if error}<p role="alert">{error}</p>{/if}
    <div class="actions"><button type="button" disabled={!!error || !draft} onclick={apply}>Apply conditions</button>{#if saved}<span role="status">Conditions saved</span>{/if}</div>
    </fieldset>
  </section>
{/if}
<style>
  .condition-panel { margin-top: 16px; padding: 16px; border: 1px solid var(--border-color, #6c6f84); border-radius: 10px; background: var(--bg-surface, #20232e); color: var(--text-primary, #f7f8fc); }
  fieldset { border: 0; padding: 0; margin: 0; min-width: 0; }
  .header, .actions { display: flex; align-items: center; flex-wrap: wrap; justify-content: space-between; gap: 8px; margin-bottom: 12px; }
  label { display: flex; flex-direction: column; gap: 8px; margin-block: 14px; }
  button, select { padding: 8px 10px; color: inherit; background: var(--bg-surface, #20232e); border: 1px solid var(--border-color, #676b80); border-radius: 6px; cursor: pointer; }
  button:disabled { opacity: .5; cursor: default; } [role="alert"] { color: var(--error-color, #ffa0a0); }
  button:focus-visible, select:focus-visible { outline: 2px solid var(--accent-primary, #a0a8ff); outline-offset: 2px; }
</style>
