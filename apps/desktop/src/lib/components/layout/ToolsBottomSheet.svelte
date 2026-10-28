<script lang="ts">
  import { appState } from '$lib/state/app.svelte';

  const aiRecommendations = [
    { icon: '⚡', label: 'Redact PII' },
    { icon: '✍️', label: 'Sign' },
    { icon: '📑', label: 'OCR' },
    { icon: '🗜️', label: 'Compress' }
  ];

  const groupedTools = [
    {
      group: 'SIGNING',
      tools: [
        { icon: '✍️', label: 'Sign with Certificate' },
        { icon: '⏰', label: 'Timestamp' },
        { icon: '🖊', label: 'Sign' }
      ]
    },
    {
      group: 'DOCUMENT SECURITY',
      tools: [
        { icon: '🛡', label: 'Add Password' },
        { icon: '🔏', label: 'Add Watermark' },
        { icon: '🧹', label: 'Sanitize' },
        { icon: '📄', label: 'Flatten' },
        { icon: '🔐', label: 'Change Permissions' }
      ]
    },
    {
      group: 'REVIEW',
      tools: [
        { icon: '🏷', label: 'Edit Metadata' },
        { icon: '📑', label: 'Edit Table of Contents' }
      ]
    },
    {
      group: 'PAGE FORMATTING',
      tools: [
        { icon: '✂️', label: 'Crop' },
        { icon: '🔄', label: 'Rotate' },
        { icon: '✂️', label: 'Split' },
        { icon: '📋', label: 'Reorganize Pages' },
        { icon: '📐', label: 'Resize Pages' }
      ]
    }
  ];

  let isOpen = $state(false);

  export function toggle() {
    isOpen = !isOpen;
  }

  function handleOutsideClick() {
    if (isOpen) {
      isOpen = false;
    }
  }

  function invokeTool(toolLabel: string) {
    console.log('Invoking tool (mobile):', toolLabel);
    isOpen = false;
  }
</script>

{#if isOpen}
  <div class="bottom-sheet-overlay" onclick={handleOutsideClick} onkeydown={(e) => e.key === 'Escape' && (isOpen = false)} tabindex="0" role="button" aria-label="Close tools overlay"></div>
{/if}

<div class="bottom-sheet" class:open={isOpen}>
  <div class="drag-handle-container" onclick={toggle} onkeydown={(e) => e.key === 'Enter' && toggle()} tabindex="0" role="button" aria-label="Toggle bottom sheet">
    <div class="drag-handle"></div>
  </div>

  <div class="sheet-content">
    <div class="carousel-container">
      {#each aiRecommendations as rec}
        <button class="ai-chip" onclick={() => invokeTool(rec.label)}>
          <span class="chip-icon">{rec.icon}</span>
          <span class="chip-label">{rec.label}</span>
        </button>
      {/each}
    </div>

    <div class="tools-accordion">
      {#each groupedTools as section}
        <details class="tool-group" open>
          <summary class="group-title">{section.group}</summary>
          <div class="group-list">
            {#each section.tools as tool}
              <button class="tool-row" onclick={() => invokeTool(tool.label)}>
                <span class="tool-icon">{tool.icon}</span>
                <span class="tool-label">{tool.label}</span>
                <span class="tool-chevron">›</span>
              </button>
            {/each}
          </div>
        </details>
      {/each}
    </div>
  </div>

  <div class="engine-badge-container">
    <div class="engine-badge">
      🛡 Local Engine ACTIVE · 0 Cloud Bytes
    </div>
  </div>
</div>

<style>
  .bottom-sheet-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.5);
    z-index: 60;
  }

  .bottom-sheet {
    position: fixed;
    bottom: 0;
    left: 0;
    width: 100vw;
    max-height: 85vh;
    background-color: var(--bg-surface, #1e1e24);
    border-radius: 24px 24px 0 0;
    z-index: 70;
    transform: translateY(100%);
    transition: transform 0.3s cubic-bezier(0.25, 1, 0.5, 1);
    display: flex;
    flex-direction: column;
    padding-bottom: var(--safe-area-bottom, 0px);
  }

  .bottom-sheet.open {
    transform: translateY(0);
  }

  .drag-handle-container {
    padding: 12px;
    display: flex;
    justify-content: center;
    cursor: pointer;
  }

  .drag-handle {
    width: 40px;
    height: 4px;
    background-color: var(--border-color, #2a2a35);
    border-radius: 2px;
  }

  .sheet-content {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    padding-bottom: 80px; /* space for bottom nav if needed */
  }

  .carousel-container {
    display: flex;
    gap: 8px;
    padding: 0 16px 16px;
    overflow-x: auto;
    scroll-snap-type: x mandatory;
    scrollbar-width: none;
  }
  .carousel-container::-webkit-scrollbar {
    display: none;
  }

  .ai-chip {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 10px 16px;
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 24px;
    scroll-snap-align: start;
    flex-shrink: 0;
    cursor: pointer;
  }

  .chip-icon {
    font-size: 16px;
  }

  .chip-label {
    font-size: 14px;
    font-weight: 500;
    color: var(--text-primary, #ffffff);
  }

  .tools-accordion {
    padding: 0 16px 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .tool-group {
    border-bottom: 1px solid var(--border-color, #2a2a35);
    padding-bottom: 8px;
  }

  .group-title {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-muted, #9ca3af);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    padding: 12px 0;
    cursor: pointer;
    list-style: none;
  }

  .group-title::-webkit-details-marker {
    display: none;
  }

  .group-list {
    display: flex;
    flex-direction: column;
  }

  .tool-row {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 12px 8px; /* 48px min touch target */
    min-height: 48px;
    background: transparent;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    text-align: left;
  }
  .tool-row:active {
    background: rgba(255, 255, 255, 0.05);
  }

  .tool-icon {
    font-size: 20px;
  }

  .tool-label {
    flex: 1;
    font-size: 15px;
    color: var(--text-primary, #ffffff);
  }

  .tool-chevron {
    color: var(--text-muted, #9ca3af);
    font-size: 18px;
  }

  .engine-badge-container {
    padding: 16px;
    background: var(--bg-surface, #1e1e24);
    border-top: 1px solid var(--border-color, #2a2a35);
  }

  .engine-badge {
    background: var(--engine-green-bg, rgba(34, 197, 94, 0.08));
    border: 1px solid rgba(34, 197, 94, 0.1);
    color: #22c55e;
    font-size: 12px;
    font-weight: 600;
    text-align: center;
    padding: 12px;
    border-radius: 24px;
  }
</style>
