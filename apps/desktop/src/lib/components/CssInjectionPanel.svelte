<script lang="ts">
  let {
    preset = $bindable('github'),
    customCss = $bindable(''),
    pageSize = $bindable('A4'),
    onPreview = () => {},
    onReset = () => {}
  }: {
    preset?: string;
    customCss?: string;
    pageSize?: string;
    onPreview?: () => void;
    onReset?: () => void;
  } = $props();

  let showAdvancedCss = $state(false);

  const presets = [
    { id: 'github', label: 'GitHub Docs', desc: 'Clean sans-serif, syntax highlighting, zebra tables' },
    { id: 'elegant', label: 'Executive Serif', desc: 'Georgia typography, purple accents, refined headers' },
    { id: 'minimal', label: 'Swiss Minimal', desc: 'High-contrast monochrome, bold headers, generous space' },
    { id: 'branded', label: 'Corporate Modern', desc: 'Indigo brand bar, styled badges, modern card tables' },
    { id: 'compact', label: 'Dense Report', desc: 'Optimized 11px font, tight margins, spreadsheet-ready' }
  ];

  function handleFileUpload(event: Event) {
    const input = event.target as HTMLInputElement;
    if (input.files && input.files[0]) {
      const reader = new FileReader();
      reader.onload = (e) => {
        if (e.target?.result) {
          customCss = e.target.result as string;
          showAdvancedCss = true;
        }
      };
      reader.readAsText(input.files[0]);
    }
  }
</script>

<div class="css-injection-panel" id="css-injection-panel">
  <div class="panel-header">
    <div class="header-title">
      <span class="header-icon">🎨</span>
      <h4>PDF Styling & Presets</h4>
    </div>
    <span class="badge-free">CORE FREE</span>
  </div>

  <!-- Preset Selector -->
  <div class="control-group">
    <label for="css-preset-select">Style Theme</label>
    <select id="css-preset-select" bind:value={preset}>
      {#each presets as p}
        <option value={p.id}>{p.label}</option>
      {/each}
    </select>
    <p class="preset-description">
      {presets.find(p => p.id === preset)?.desc}
    </p>
  </div>

  <!-- Page Geometry -->
  <div class="control-row">
    <div class="control-group half">
      <label for="css-pagesize-select">Page Format</label>
      <select id="css-pagesize-select" bind:value={pageSize}>
        <option value="A4">A4 (210 × 297 mm)</option>
        <option value="Letter">US Letter (8.5 × 11 in)</option>
        <option value="Legal">US Legal (8.5 × 14 in)</option>
      </select>
    </div>
    <div class="control-group half">
      <label for="css-margin-preset">Margins</label>
      <select id="css-margin-preset">
        <option value="normal">Normal (20mm)</option>
        <option value="narrow">Narrow (12mm)</option>
        <option value="wide">Wide (25mm)</option>
      </select>
    </div>
  </div>

  <!-- Custom CSS Accordion -->
  <div class="custom-css-section">
    <button
      type="button"
      id="btn-toggle-custom-css"
      class="toggle-btn"
      onclick={() => (showAdvancedCss = !showAdvancedCss)}
    >
      <span>{showAdvancedCss ? '▼' : '►'} Custom CSS Injection</span>
      <span class="subtext">{customCss.trim().length > 0 ? '(Active)' : '(Optional)'}</span>
    </button>

    {#if showAdvancedCss}
      <div class="css-editor-container" id="custom-css-container">
        <div class="editor-actions">
          <label class="btn-secondary btn-file-upload" for="css-file-input">
            📁 Import .css file
            <input
              type="file"
              id="css-file-input"
              accept=".css"
              style="display: none;"
              onchange={handleFileUpload}
            />
          </label>
          {#if customCss.trim().length > 0}
            <button
              type="button"
              id="btn-clear-custom-css"
              class="btn-text-danger"
              onclick={() => (customCss = '')}
            >
              Clear CSS
            </button>
          {/if}
        </div>

        <textarea
          id="custom-css-textarea"
          bind:value={customCss}
          placeholder="/* Enter custom CSS rules here */&#10;body &#123; font-size: 16px; &#125;&#10;h1 &#123; color: #0070f3; &#125;"
          rows="6"
          spellcheck="false"
        ></textarea>
        <div class="css-tip">
          💡 Custom rules are appended after the preset theme to override fonts, colors, and margins.
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .css-injection-panel {
    display: flex;
    flex-direction: column;
    gap: 12px;
    background: var(--surface-secondary, #1e1e24);
    border: 1px solid var(--border-color, #2d2d38);
    border-radius: 8px;
    padding: 14px;
    margin-top: 10px;
  }

  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .header-title {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .header-title h4 {
    margin: 0;
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary, #ffffff);
  }

  .badge-free {
    font-size: 10px;
    font-weight: 700;
    color: #10b981;
    background: rgba(16, 185, 129, 0.15);
    border: 1px solid rgba(16, 185, 129, 0.3);
    padding: 2px 6px;
    border-radius: 4px;
    letter-spacing: 0.05em;
  }

  .control-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .control-row {
    display: flex;
    gap: 10px;
  }

  .half {
    flex: 1;
  }

  label {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-secondary, #a1a1aa);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  select, textarea {
    background: var(--surface-primary, #141418);
    border: 1px solid var(--border-color, #3f3f4e);
    border-radius: 6px;
    color: var(--text-primary, #ffffff);
    padding: 8px 10px;
    font-size: 12px;
    font-family: inherit;
  }

  select:focus, textarea:focus {
    outline: none;
    border-color: var(--brand-accent, #6366f1);
  }

  .preset-description {
    margin: 0;
    font-size: 11px;
    color: var(--text-muted, #71717a);
    font-style: italic;
  }

  .toggle-btn {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    background: transparent;
    border: none;
    padding: 6px 0;
    color: var(--text-secondary, #d4d4d8);
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    text-align: left;
  }

  .toggle-btn:hover {
    color: var(--brand-accent, #818cf8);
  }

  .subtext {
    font-size: 11px;
    color: var(--text-muted, #71717a);
  }

  .css-editor-container {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-top: 6px;
  }

  .editor-actions {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .btn-file-upload {
    font-size: 11px;
    padding: 4px 8px;
    background: var(--surface-tertiary, #272730);
    border: 1px solid var(--border-color, #3f3f4e);
    border-radius: 4px;
    color: var(--text-primary, #ffffff);
    cursor: pointer;
  }

  .btn-file-upload:hover {
    background: var(--surface-hover, #32323e);
  }

  .btn-text-danger {
    background: transparent;
    border: none;
    color: #ef4444;
    font-size: 11px;
    cursor: pointer;
    padding: 2px 4px;
  }

  textarea {
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace;
    font-size: 11px;
    resize: vertical;
    white-space: pre;
  }

  .css-tip {
    font-size: 10.5px;
    color: var(--text-muted, #71717a);
    line-height: 1.4;
  }
</style>