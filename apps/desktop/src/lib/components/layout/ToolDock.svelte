<script lang="ts">
  import { appState } from '$lib/state/app.svelte';

  const aiRecommendations = [
    { icon: '⚡', label: 'Redact PII', type: 'amber' },
    { icon: '✍️', label: 'Validate Signature', type: 'green' },
    { icon: '📑', label: 'Extract Tables', type: 'default' }
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

  function invokeTool(toolLabel: string) {
    console.log('Invoking tool:', toolLabel);
    // In a real implementation, map to appState.openTool(tool)
  }
</script>

<aside class="tool-dock">
  <div class="dock-content">
    <div class="ai-recommendations">
      <div class="ai-header">
        <span class="ai-title">AI Recommendations</span>
        <span class="pulse-dot"></span>
      </div>
      <div class="ai-sublabel">Based on document classification</div>

      <div class="ai-list">
        {#each aiRecommendations as rec}
          <button class="ai-row {rec.type}" onclick={() => invokeTool(rec.label)}>
            <span class="ai-icon">{rec.icon}</span>
            <span class="ai-label">{rec.label}</span>
            <span class="ai-chevron">›</span>
          </button>
        {/each}
      </div>
    </div>

    <div class="tools-sections">
      {#each groupedTools as section}
        <div class="tool-group">
          <div class="group-title">{section.group}</div>
          <div class="group-list">
            {#each section.tools as tool}
              <button class="tool-row" onclick={() => invokeTool(tool.label)}>
                <span class="tool-icon">{tool.icon}</span>
                <span class="tool-label">{tool.label}</span>
                <span class="tool-chevron">›</span>
              </button>
            {/each}
          </div>
        </div>
      {/each}
    </div>
  </div>

  <div class="engine-badge-container">
    <div class="engine-badge">
      🛡 Local Engine ACTIVE · 0 Cloud Bytes
    </div>
  </div>
</aside>

<style>
  .tool-dock {
    width: var(--layout-dock-width, 260px);
    background-color: var(--bg-surface, #1e1e24);
    border-left: 1px solid var(--border-color, #2a2a35);
    display: flex;
    flex-direction: column;
    height: 100%;
    flex-shrink: 0;
  }

  .dock-content {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .ai-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 4px;
  }

  .ai-title {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-primary, #ffffff);
  }

  .pulse-dot {
    width: 8px;
    height: 8px;
    background-color: var(--watching-green, #22c55e);
    border-radius: 50%;
    animation: pulse 2s infinite;
  }

  .ai-sublabel {
    font-size: 10px;
    color: var(--text-muted, #9ca3af);
    margin-bottom: 12px;
  }

  .ai-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .ai-row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 12px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid transparent;
    border-radius: 6px;
    cursor: pointer;
    text-align: left;
    transition: all 0.2s;
  }
  .ai-row:hover {
    background: rgba(255, 255, 255, 0.1);
  }
  .ai-row.amber {
    border-left: 3px solid var(--review-amber, #f59e0b);
  }
  .ai-row.green {
    border-left: 3px solid var(--watching-green, #22c55e);
  }

  .ai-icon, .tool-icon {
    font-size: 16px;
  }

  .ai-label, .tool-label {
    flex: 1;
    font-size: 12px;
    color: var(--text-primary, #ffffff);
  }

  .ai-chevron, .tool-chevron {
    color: var(--text-muted, #9ca3af);
    font-size: 14px;
  }

  .tools-sections {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .group-title {
    font-size: 10px;
    font-weight: 600;
    color: var(--text-muted, #9ca3af);
    margin-bottom: 8px;
    letter-spacing: 0.05em;
  }

  .group-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .tool-row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 6px 8px;
    background: transparent;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    text-align: left;
    transition: background 0.15s;
  }

  .tool-row:hover {
    background: rgba(255, 255, 255, 0.05);
  }

  .engine-badge-container {
    padding: 12px 16px;
    border-top: 1px solid var(--border-color, #2a2a35);
    background: var(--bg-surface, #1e1e24);
  }

  .engine-badge {
    background: var(--engine-green-bg, rgba(34, 197, 94, 0.08));
    border: 1px solid rgba(34, 197, 94, 0.1);
    color: #22c55e;
    font-size: 10px;
    font-weight: 600;
    text-align: center;
    padding: 8px;
    border-radius: 20px;
  }
</style>
