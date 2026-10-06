<script lang="ts">
  let theme = $state('dark');
  let brandColor = $state('#5E6AD2');
  let hideBadge = $state(false);
  let selectedTools = $state(['merge', 'compress']);
  let copyFeedback = $state(false);

  const availableTools = [
    'merge', 'split', 'compress', 'encrypt', 'decrypt', 'watermark',
    'ocr', 'extract', 'reorder', 'rotate', 'metadata'
  ];

  function toggleTool(tool: string) {
    if (selectedTools.includes(tool)) {
      selectedTools = selectedTools.filter(t => t !== tool);
    } else {
      selectedTools = [...selectedTools, tool];
    }
  }

  let embedCode = $derived(
`<script src="https://cdn.usepaperpilot.com/v1/embed.js" async></scr` + `ipt>
<div id="paperpilot-portal"
     data-theme="${theme}"
     data-tools="${selectedTools.join(',')}"
     data-brand-color="${brandColor}"${hideBadge ? '\n     data-hide-badge="true"' : ''}>
</div>`
  );

  async function copyToClipboard() {
    try {
      await navigator.clipboard.writeText(embedCode);
      copyFeedback = true;
      setTimeout(() => { copyFeedback = false; }, 2000);
    } catch (err) {
      console.error('Failed to copy: ', err);
    }
  }
</script>

<section id="embed-generator" class="embed-section">
  <div class="embed-container">
    <div class="embed-header">
      <h2>Interactive Embed Generator</h2>
      <p>Configure your drop-in PDF widget and copy the HTML snippet.</p>
    </div>

    <div class="generator-grid">
      <div class="controls-panel">
        <div class="control-group">
          <label for="theme-select">Theme</label>
          <select id="theme-select" bind:value={theme}>
            <option value="dark">Dark</option>
            <option value="light">Light</option>
            <option value="system">System Default</option>
          </select>
        </div>

        <div class="control-group">
          <label for="brand-color">Brand Color</label>
          <div class="color-picker-wrapper">
            <input id="brand-color" type="color" bind:value={brandColor} />
            <input type="text" class="color-text" bind:value={brandColor} />
          </div>
        </div>

        <div class="control-group">
          <div class="tools-header">
            <span id="tools-label" class="group-label">Available Tools</span>
            <div class="tools-actions">
              <button class="text-btn" onclick={() => selectedTools = [...availableTools]}>Select All</button>
              <button class="text-btn" onclick={() => selectedTools = []}>Clear</button>
            </div>
          </div>
          <div class="tools-grid" role="group" aria-labelledby="tools-label">
            {#each availableTools as tool}
              <button
                class="tool-pill {selectedTools.includes(tool) ? 'active' : ''}"
                onclick={() => toggleTool(tool)}
                aria-pressed={selectedTools.includes(tool)}
              >
                {tool}
              </button>
            {/each}
          </div>
        </div>

        <div class="control-group checkbox-group">
          <input id="hide-badge" type="checkbox" bind:checked={hideBadge} />
          <label for="hide-badge">Hide "Powered by PaperPilot" Badge (Pro)</label>
        </div>
      </div>

      <div class="code-panel">
        <div class="code-header">
          <span class="code-title">index.html</span>
          <button class="copy-btn" onclick={copyToClipboard}>
            {copyFeedback ? '✓ Copied!' : '📋 Copy Code'}
          </button>
        </div>
        <pre class="code-block"><code>{embedCode}</code></pre>
      </div>
    </div>
  </div>
</section>

<style>
  .embed-section {
    padding: 80px 32px;
    background-color: var(--surface-1);
    border-top: 1px solid var(--border-color);
    border-bottom: 1px solid var(--border-color);
  }

  .embed-container {
    max-width: 1200px;
    margin: 0 auto;
  }

  .embed-header {
    text-align: center;
    margin-bottom: 48px;
  }

  .embed-header h2 {
    font-size: 2.5rem;
    margin-bottom: 16px;
  }

  .embed-header p {
    font-size: 1.1rem;
    color: var(--text-secondary);
  }

  .generator-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 48px;
    background-color: var(--surface-0);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-xl);
    padding: 32px;
  }

  .controls-panel {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .control-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .control-group label, .group-label {
    font-weight: 500;
    color: var(--text-primary);
  }

  .tools-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .tools-actions {
    display: flex;
    gap: 8px;
  }

  .text-btn {
    background: none;
    border: none;
    color: var(--accent-primary);
    font-size: 0.85rem;
    cursor: pointer;
    padding: 0;
  }

  .text-btn:hover {
    text-decoration: underline;
  }

  select, input[type="text"].color-text {
    padding: 10px 12px;
    background-color: var(--surface-2);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    color: var(--text-primary);
    font-size: 1rem;
  }

  .color-picker-wrapper {
    display: flex;
    gap: 12px;
    align-items: center;
  }

  input[type="color"] {
    width: 40px;
    height: 40px;
    padding: 0;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    background: none;
  }

  .tools-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }

  .tool-pill {
    padding: 6px 12px;
    background-color: var(--surface-2);
    border: 1px solid var(--border-color);
    border-radius: 20px;
    color: var(--text-secondary);
    font-size: 0.9rem;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .tool-pill:hover {
    background-color: var(--surface-3);
    color: var(--text-primary);
  }

  .tool-pill.active {
    background-color: var(--accent-primary);
    color: white;
    border-color: var(--accent-primary);
  }

  .checkbox-group {
    flex-direction: row;
    align-items: center;
    gap: 12px;
  }

  .code-panel {
    background-color: #1e1e1e; /* VS Code dark theme bg */
    border-radius: var(--border-radius-lg);
    border: 1px solid #333;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .code-header {
    background-color: #2d2d2d;
    padding: 12px 16px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid #1e1e1e;
  }

  .code-title {
    color: #a1a7b3;
    font-size: 0.9rem;
    font-family: monospace;
  }

  .copy-btn {
    background-color: transparent;
    border: 1px solid #444;
    border-radius: 4px;
    color: #fff;
    padding: 4px 12px;
    font-size: 0.85rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .copy-btn:hover {
    background-color: #444;
  }

  .code-block {
    margin: 0;
    padding: 24px;
    color: #d4d4d4;
    font-family: 'Fira Code', Consolas, Monaco, 'Andale Mono', 'Ubuntu Mono', monospace;
    font-size: 0.95rem;
    line-height: 1.5;
    overflow-x: auto;
  }

  @media (max-width: 900px) {
    .generator-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
