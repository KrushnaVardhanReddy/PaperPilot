<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import AiSettingsModal from '$lib/components/settings/AiSettingsModal.svelte';
  import { aiConfig } from '$lib/state/aiConfig.svelte';

  let isAiModalOpen = $state(false);
  let outputDir = $state('');

  function handleBrowse() {
    // Mock browse action
    outputDir = '/home/user/Documents/PaperPilot';
  }
</script>

<div class="settings-panel">
  <div class="settings-header">
    <h2>Settings</h2>
    <p>Configure PaperPilot preferences</p>
  </div>

  <div class="settings-content">
    <div class="setting-group" id="setting-group-ai-engine">
      <div class="ai-header-row">
        <div>
          <h3>AI Engine & Models</h3>
          <p class="help-text">Configure local/offline models, Llamafile runners, and remote BYOK keys.</p>
        </div>
        <button class="btn primary" onclick={() => {
          aiConfig.testResult = null;
          isAiModalOpen = true;
        }}>
          Manage AI & Models 🚀
        </button>
      </div>

      <div class="ai-status-summary">
        <span class="ai-pill">Current Mode: <strong>{aiConfig.mode.toUpperCase()}</strong></span>
        {#if aiConfig.mode === 'llamafile' || aiConfig.mode === 'offlinenlp'}
          <span class="ai-badge local">🔒 100% On-Device / Zero Cloud</span>
        {:else if aiConfig.mode === 'universal'}
          <span class="ai-badge remote">🌐 Custom Endpoint</span>
        {:else}
          <span class="ai-badge cloud">🔑 BYOK Cloud ({aiConfig.byokProvider || 'OpenAI'})</span>
        {/if}
      </div>
    </div>

    <div class="setting-group">
      <h3>General</h3>

      <div class="setting-item">
        <label for="outputDir">Default Output Directory</label>
        <div class="input-with-button">
          <input
            id="outputDir"
            type="text"
            bind:value={outputDir}
            class="form-input"
            placeholder="Select directory..."
            readonly
          />
          <button class="btn secondary" onclick={handleBrowse}>Browse</button>
        </div>
      </div>
    </div>

    <div class="setting-group" id="setting-group-developer-mode">
      <h3>Developer Mode & Local REST API</h3>
      <div class="setting-item toggle-item">
        <div class="setting-info">
          <label for="developer-mode-toggle">Enable Local REST API & Swagger (Port 7823)</label>
          <p class="help-text">
            Spins up a local REST API and Swagger UI server at <code>http://127.0.0.1:7823</code> for Python scripts, homelab automations, and external clients.
          </p>
        </div>
        <label class="switch">
          <input
            type="checkbox"
            id="developer-mode-toggle"
            checked={appState.developerMode}
            onchange={async (e) => await appState.toggleDeveloperMode(e.currentTarget.checked)}
          />
          <span class="slider round"></span>
        </label>
      </div>
      {#if appState.developerMode}
        <div class="dev-mode-status" id="dev-mode-status-info">
          <span class="status-indicator active">● Running</span>
          <span>API Endpoint: <code>http://127.0.0.1:{appState.gatewayPort}</code></span>
          <a href="http://127.0.0.1:{appState.gatewayPort}/swagger-ui" target="_blank" rel="noreferrer" class="swagger-link" id="open-swagger-external-btn">
            Open Swagger UI ↗
          </a>
        </div>
      {/if}
    </div>
  </div>
</div>

<AiSettingsModal bind:isOpen={isAiModalOpen} />

<style>
  .ai-header-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }

  .ai-status-summary {
    display: flex;
    align-items: center;
    gap: 12px;
    flex-wrap: wrap;
    padding-top: 8px;
  }

  .ai-pill {
    font-size: 0.85rem;
    color: var(--text-secondary);
    background: var(--bg-primary);
    padding: 4px 10px;
    border-radius: 6px;
    border: 1px solid var(--border-color);
  }

  .ai-badge {
    font-size: 0.8rem;
    padding: 3px 8px;
    border-radius: 6px;
    font-weight: 500;
  }

  .ai-badge.local {
    background: rgba(34, 197, 94, 0.15);
    color: #4ade80;
    border: 1px solid rgba(34, 197, 94, 0.3);
  }

  .ai-badge.remote {
    background: rgba(59, 130, 246, 0.15);
    color: #60a5fa;
    border: 1px solid rgba(59, 130, 246, 0.3);
  }

  .ai-badge.cloud {
    background: rgba(168, 85, 247, 0.15);
    color: #c084fc;
    border: 1px solid rgba(168, 85, 247, 0.3);
  }

  .settings-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: 32px;
    max-width: 800px;
    margin: 0 auto;
    width: 100%;
  }

  .settings-header {
    margin-bottom: 32px;
  }

  .settings-header h2 {
    font-size: 1.75rem;
    margin-bottom: 8px;
    color: var(--text-primary);
  }

  .settings-header p {
    color: var(--text-secondary);
  }

  .settings-content {
    display: flex;
    flex-direction: column;
    gap: 32px;
  }

  .setting-group {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-lg);
    padding: 24px;
  }

  .setting-group h3 {
    font-size: 1.1rem;
    margin-bottom: 20px;
    color: var(--text-primary);
    border-bottom: 1px solid var(--border-color);
    padding-bottom: 12px;
  }

  .setting-item {
    margin-bottom: 20px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .setting-item:last-child {
    margin-bottom: 0;
  }

  label {
    font-size: 0.9rem;
    font-weight: 500;
    color: var(--text-primary);
  }

  .form-input {
    width: 100%;
    padding: 10px 12px;
    background-color: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    color: var(--text-primary);
    font-family: inherit;
    font-size: 0.95rem;
    transition: border-color var(--transition-fast);
  }

  .form-input:focus {
    outline: none;
    border-color: var(--accent-primary);
  }

  select.form-input {
    appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%23A1A7B3' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 12px center;
    padding-right: 40px;
  }

  .help-text {
    font-size: 0.8rem;
    color: var(--text-muted);
  }

  .input-with-button {
    display: flex;
    gap: 8px;
  }

  .input-with-button .form-input {
    flex: 1;
  }

  .btn {
    padding: 10px 16px;
    border: none;
    border-radius: var(--border-radius-md);
    font-weight: 500;
    cursor: pointer;
    transition: all var(--transition-fast);
    font-family: inherit;
    font-size: 0.95rem;
  }

  .btn.secondary {
    background-color: var(--bg-secondary);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
  }

  .btn.secondary:hover {
    background-color: var(--bg-surface-hover);
    border-color: var(--text-muted);
  }

  /* Toggle Switch Styles */
  .setting-item.toggle-item {
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
  }

  .setting-info {
    flex: 1;
    margin-right: 20px;
  }

  .switch {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 24px;
    flex-shrink: 0;
  }

  .switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: var(--bg-secondary);
    transition: .3s;
    border: 1px solid var(--border-color);
  }

  .slider:before {
    position: absolute;
    content: "";
    height: 18px;
    width: 18px;
    left: 2px;
    bottom: 2px;
    background-color: var(--text-secondary);
    transition: .3s;
  }

  input:checked + .slider {
    background-color: var(--accent-primary);
    border-color: var(--accent-primary);
  }

  input:focus + .slider {
    box-shadow: 0 0 1px var(--accent-primary);
  }

  input:checked + .slider:before {
    transform: translateX(20px);
    background-color: #fff;
  }

  .slider.round {
    border-radius: 24px;
  }

  .slider.round:before {
    border-radius: 50%;
  }

  .dev-mode-status {
    margin-top: 16px;
    padding: 12px 16px;
    background-color: rgba(33, 150, 243, 0.1);
    border: 1px solid rgba(33, 150, 243, 0.3);
    border-radius: var(--border-radius-md);
    display: flex;
    align-items: center;
    gap: 16px;
    font-size: 0.9rem;
    color: var(--text-primary);
  }

  .status-indicator {
    font-weight: bold;
    color: var(--text-muted);
  }

  .status-indicator.active {
    color: #4caf50;
  }

  code {
    background-color: var(--bg-primary);
    padding: 2px 6px;
    border-radius: 4px;
    font-family: monospace;
    font-size: 0.85rem;
    color: var(--accent-primary);
  }

  .swagger-link {
    margin-left: auto;
    color: var(--accent-primary);
    text-decoration: none;
    font-weight: 500;
  }

  .swagger-link:hover {
    text-decoration: underline;
  }
</style>
