<script lang="ts">
  let aiProvider = $state('openai');
  let apiKey = $state('');
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
    <div class="setting-group">
      <h3>AI Configuration</h3>

      <div class="setting-item">
        <label for="aiProvider">AI Provider</label>
        <select id="aiProvider" bind:value={aiProvider} class="form-input">
          <option value="openai">OpenAI</option>
          <option value="anthropic">Anthropic</option>
          <option value="local">Local (Ollama)</option>
        </select>
      </div>

      <div class="setting-item">
        <label for="apiKey">API Key</label>
        <input
          id="apiKey"
          type="password"
          bind:value={apiKey}
          class="form-input"
          placeholder="sk-..."
        />
        <p class="help-text">Your API key is stored securely locally.</p>
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
  </div>
</div>

<style>
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
</style>
