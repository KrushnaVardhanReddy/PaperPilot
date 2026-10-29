<script lang="ts">
  import { appState } from '$lib/state/app.svelte';

  let groupedTools = {
    'SIGNING': [
      { id: 'sign-cert', icon: '✍️', name: 'Sign with Certificate' },
      { id: 'timestamp', icon: '⏰', name: 'Timestamp' },
      { id: 'sign', icon: '🖊', name: 'Sign' }
    ],
    'DOCUMENT SECURITY': [
      { id: 'add-pass', icon: '🛡', name: 'Add Password' },
      { id: 'watermark', icon: '🔏', name: 'Add Watermark' },
      { id: 'sanitize', icon: '🧹', name: 'Sanitize' },
      { id: 'flatten', icon: '📄', name: 'Flatten' },
      { id: 'perms', icon: '🔐', name: 'Change Permissions' }
    ],
    'REVIEW': [
      { id: 'metadata', icon: '🏷', name: 'Edit Metadata' },
      { id: 'toc', icon: '📑', name: 'Edit Table of Contents' }
    ],
    'PAGE FORMATTING': [
      { id: 'crop', icon: '✂️', name: 'Crop' },
      { id: 'rotate', icon: '🔄', name: 'Rotate' },
      { id: 'split', icon: '✂️', name: 'Split' },
      { id: 'reorg', icon: '📋', name: 'Reorganize Pages' },
      { id: 'resize', icon: '📐', name: 'Resize Pages' }
    ]
  };

  function invokeTool(toolId: string) { appState.openTool({ id: toolId }); }
</script>

<div class="tool-dock-container">
  <div class="dock-header">
    <h3>AI Recommendations</h3>
    <span class="pulse-dot"></span>
  </div>
  <p class="sub-label">Based on detected contract</p>

  <div class="recommendations">
    <button class="rec-row amber-border" onclick={() => invokeTool('redact-pii')}>
      <span class="icon">⚡</span>
      <span class="name">Redact PII</span>
      <span class="chevron">›</span>
    </button>
    <button class="rec-row green-border" onclick={() => invokeTool('validate-sig')}>
      <span class="icon">✍️</span>
      <span class="name">Validate Signature</span>
      <span class="chevron">›</span>
    </button>
  </div>

  <div class="tools-scroll-area">
    {#each Object.entries(groupedTools) as [groupName, tools]}
      <div class="tool-group">
        <h4>{groupName}</h4>
        <ul class="tool-list">
          {#each tools as tool}
            <li>
              <button class="tool-row" onclick={() => invokeTool(tool.id)}>
                <span class="icon">{tool.icon}</span>
                <span class="name">{tool.name}</span>
                <span class="chevron">›</span>
              </button>
            </li>
          {/each}
        </ul>
      </div>
    {/each}
  </div>

  <div class="engine-badge-wrapper">
    <div class="engine-badge">
      🛡 Local Engine ACTIVE · 0 Cloud Bytes
    </div>
  </div>
</div>

<style>
  .tool-dock-container { width: var(--layout-dock-width); height: 100%; background-color: var(--surface-1); border-left: 1px solid var(--border-color); display: flex; flex-direction: column; overflow: hidden; }
  .dock-header { display: flex; justify-content: space-between; align-items: center; padding: 16px 16px 4px 16px; }
  .dock-header h3 { font-size: 0.85rem; font-weight: 600; color: var(--text-primary); margin: 0; }
  .pulse-dot { width: 8px; height: 8px; border-radius: 50%; background-color: var(--watching-green); box-shadow: 0 0 8px rgba(34, 197, 94, 0.6); animation: pulse 2s infinite; }
  @keyframes pulse { 0% { transform: scale(1); opacity: 1; } 50% { transform: scale(1.5); opacity: 0.5; } 100% { transform: scale(1); opacity: 1; } }
  .sub-label { font-size: 0.7rem; color: var(--text-muted); padding: 0 16px 12px 16px; margin: 0; border-bottom: 1px solid var(--border-color); }
  .recommendations { padding: 12px 16px; display: flex; flex-direction: column; gap: 8px; border-bottom: 1px solid var(--border-color); }
  .rec-row { display: flex; align-items: center; width: 100%; background: var(--surface-2); border: 1px solid var(--border-color); border-radius: 6px; padding: 10px 12px; cursor: pointer; text-align: left; transition: background 0.2s; }
  .rec-row:hover { background: var(--surface-3); }
  .amber-border { border-left: 3px solid var(--review-amber); }
  .green-border { border-left: 3px solid var(--watching-green); }
  .rec-row .icon { margin-right: 8px; font-size: 1.1rem; }
  .rec-row .name { flex: 1; font-size: 0.85rem; color: var(--text-primary); font-weight: 500; }
  .rec-row .chevron { color: var(--text-muted); }
  .tools-scroll-area { flex: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 24px; }
  .tool-group h4 { font-size: 0.7rem; font-weight: 600; text-transform: uppercase; color: var(--text-muted); margin: 0 0 8px 0; letter-spacing: 0.05em; }
  .tool-list { list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 4px; }
  .tool-row { display: flex; align-items: center; width: 100%; background: transparent; border: none; padding: 6px 8px; border-radius: 4px; cursor: pointer; text-align: left; }
  .tool-row:hover { background: var(--surface-2); }
  .tool-row .icon { width: 24px; font-size: 1rem; margin-right: 8px; text-align: center; }
  .tool-row .name { flex: 1; font-size: 0.85rem; color: var(--text-secondary); }
  .tool-row:hover .name { color: var(--text-primary); }
  .tool-row .chevron { color: var(--text-muted); font-size: 1.1rem; }
  .engine-badge-wrapper { padding: 16px; background: var(--surface-1); border-top: 1px solid var(--border-color); }
  .engine-badge { background: var(--engine-green-bg); color: var(--watching-green); border: 1px solid rgba(34, 197, 94, 0.2); border-radius: 20px; padding: 8px 12px; font-size: 0.7rem; font-weight: 600; text-align: center; letter-spacing: 0.02em; }
</style>
