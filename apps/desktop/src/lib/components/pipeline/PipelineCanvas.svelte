<script lang="ts">
  import { pipelineState, OPERATION_REGISTRY } from '$lib/state/pipeline.svelte';
  import StepCard from './StepCard.svelte';
  import StepPicker from './StepPicker.svelte';

  let showPicker = $state(false);
  let dragFromIndex = $state<number | null>(null);

  function handleDragStart(event: DragEvent, index: number) {
    dragFromIndex = index;
    event.dataTransfer!.effectAllowed = 'move';
  }

  function handleDragOver(event: DragEvent, index: number) {
    event.preventDefault();
    event.dataTransfer!.dropEffect = 'move';
  }

  function handleDrop(event: DragEvent, toIndex: number) {
    event.preventDefault();
    if (dragFromIndex !== null && dragFromIndex !== toIndex) {
      pipelineState.moveStep(dragFromIndex, toIndex);
    }
    dragFromIndex = null;
  }

  function handleDragEnd() {
    dragFromIndex = null;
  }
</script>

<div class="pipeline-canvas" id="pipeline-canvas">
  <div class="canvas-scroll-wrapper">
    {#each pipelineState.steps as step, i (step.id)}
      <div
        class="step-wrapper"
        role="listitem"
        draggable="true"
        ondragstart={(e) => handleDragStart(e, i)}
        ondragover={(e) => handleDragOver(e, i)}
        ondrop={(e) => handleDrop(e, i)}
        ondragend={handleDragEnd}
        class:drag-over={dragFromIndex !== null && dragFromIndex !== i}
      >
        <StepCard {step} index={i} isRunning={pipelineState.currentStepIndex === i} />
      </div>

      {#if i < pipelineState.steps.length - 1}
        <div class="connector" aria-hidden="true">→</div>
      {/if}
    {/each}

    <button
      class="add-step-btn"
      id="add-pipeline-step-btn"
      onclick={() => showPicker = true}
    >
      + Add Step
    </button>
  </div>
</div>

{#if showPicker}
  <StepPicker onclose={() => showPicker = false} />
{/if}

<style>
.pipeline-canvas {
  width: 100%;
  padding: 24px 32px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  min-height: 140px;
}

.canvas-scroll-wrapper {
  display: flex;
  align-items: center;
  gap: 8px;
  overflow-x: auto;
  padding-bottom: 8px;
  scrollbar-width: thin;
}

.connector {
  font-size: 1.4rem;
  color: var(--text-muted);
  flex-shrink: 0;
  user-select: none;
}

.add-step-btn {
  flex-shrink: 0;
  padding: 10px 18px;
  border: 2px dashed var(--border-color);
  border-radius: var(--border-radius-md);
  background: transparent;
  color: var(--text-muted);
  font-size: 0.9rem;
  cursor: pointer;
  white-space: nowrap;
  transition: all 0.15s ease;
}

.add-step-btn:hover {
  border-color: var(--accent-primary);
  color: var(--accent-primary);
}

.step-wrapper.drag-over {
  opacity: 0.5;
}
</style>
