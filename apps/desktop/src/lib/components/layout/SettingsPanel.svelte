<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import OcrModelDownloader from '$lib/components/settings/OcrModelDownloader.svelte';
  import ModelDownloader from '$lib/components/settings/ModelDownloader.svelte';
  import { aiConfig } from '$lib/state/aiConfig.svelte';

  type SettingsCategory = 'ai' | 'packs' | 'ocr' | 'general' | 'developer';
  let activeCategory = $state<SettingsCategory>('ai');

  let outputDir = $state('');

  function handleBrowse() {
    // Mock browse action
    outputDir = '/home/user/Documents/PaperPilot';
  }

  const categories = [
    { id: 'ai' as const, label: 'AI Engine & Models', icon: '🤖', description: 'Local LLMs, runners & BYOK' },
    { id: 'packs' as const, label: 'Feature Packs', icon: '📦', description: 'Modular extensions & plugins' },
    { id: 'ocr' as const, label: 'OCR & Vision Models', icon: '👁️', description: 'On-device recognition engines' },
    { id: 'general' as const, label: 'General & Storage', icon: '⚙️', description: 'Paths, files & interface' },
    { id: 'developer' as const, label: 'Developer & Gateway', icon: '💻', description: 'REST API, Swagger & Port 7823' },
  ];
</script>

<div class="settings-layout">
  <!-- Left Category Navigation Sidebar -->
  <aside class="settings-sidebar">
    <div class="settings-sidebar-header">
      <h2>Settings</h2>
      <p>Configure PaperPilot</p>
    </div>

    <nav class="settings-nav" aria-label="Settings categories">
      {#each categories as cat}
        <button
          class="nav-tab-btn"
          class:active={activeCategory === cat.id}
          onclick={() => activeCategory = cat.id}
          id="settings-tab-{cat.id}"
        >
          <span class="nav-tab-icon">{cat.icon}</span>
          <div class="nav-tab-text">
            <span class="nav-tab-label">{cat.label}</span>
            <span class="nav-tab-desc">{cat.description}</span>
          </div>
          {#if cat.id === 'developer' && appState.developerMode}
            <span class="live-dot" title="REST API Active">●</span>
          {/if}
        </button>
      {/each}
    </nav>

    <div class="sidebar-footer">
      <div class="version-chip">
        <span class="version-tag">PaperPilot v1.0</span>
        <span class="badge local-badge">Community</span>
      </div>
    </div>
  </aside>

  <!-- Right Full-Width Settings Canvas -->
  <main class="settings-main-canvas">
    {#if activeCategory === 'ai'}
      <div class="category-section active-section">
        <div class="section-title-wrap">
          <div class="icon-bubble">🤖</div>
          <div>
            <h3>AI Engine & Models</h3>
            <p class="section-subtitle">Configure local/offline models, Llamafile runners, and remote BYOK keys.</p>
          </div>
        </div>

        <!-- Direct In-Canvas AI Mode Selector Grid -->
        <div class="setting-group" id="setting-group-ai-modes">
          <h4>Inference Provider & Execution Mode</h4>
          <p class="help-text">Select how PaperPilot runs summarization, classification, and OCR-assisted vision operations.</p>

          <div class="ai-modes-grid">
            <label class="ai-mode-card" class:active={aiConfig.mode === 'offlinenlp'}>
              <div class="mode-card-header">
                <input
                  type="radio"
                  name="canvas-ai-mode"
                  value="offlinenlp"
                  checked={aiConfig.mode === 'offlinenlp'}
                  onchange={() => {
                    aiConfig.mode = 'offlinenlp';
                    aiConfig.testResult = null;
                    aiConfig.saveConfig();
                  }}
                />
                <span class="mode-title-tag">⚡ Offline NLP</span>
              </div>
              <p class="mode-desc">Embedded ONNX model (&lt;1.1 MB). 100% on-device CPU, zero setup, airplane-safe.</p>
              <div class="mode-badge-wrap">
                <span class="badge installed-badge">Default • Zero Network</span>
              </div>
            </label>

            <label class="ai-mode-card" class:active={aiConfig.mode === 'llamafile'}>
              <div class="mode-card-header">
                <input
                  type="radio"
                  name="canvas-ai-mode"
                  value="llamafile"
                  checked={aiConfig.mode === 'llamafile'}
                  onchange={() => {
                    aiConfig.mode = 'llamafile';
                    aiConfig.testResult = null;
                    aiConfig.saveConfig();
                  }}
                />
                <span class="mode-title-tag">🚀 Offline Llamafile</span>
              </div>
              <p class="mode-desc">Runs standalone local LLM GGUF weights via OS background runner process.</p>
              <div class="mode-badge-wrap">
                <span class="badge local-badge">Local GPU/CPU Runner</span>
              </div>
            </label>

            <label class="ai-mode-card" class:active={aiConfig.mode === 'universal'}>
              <div class="mode-card-header">
                <input
                  type="radio"
                  name="canvas-ai-mode"
                  value="universal"
                  checked={aiConfig.mode === 'universal'}
                  onchange={() => {
                    aiConfig.mode = 'universal';
                    aiConfig.testResult = null;
                    aiConfig.saveConfig();
                  }}
                />
                <span class="mode-title-tag">🌐 Universal OpenAI / Ollama</span>
              </div>
              <p class="mode-desc">Connect to local/LAN Ollama, vLLM, LM Studio, or custom inference server.</p>
              <div class="mode-badge-wrap">
                <span class="badge custom-badge">Custom HTTP Endpoint</span>
              </div>
            </label>

            <label class="ai-mode-card" class:active={aiConfig.mode === 'byok'}>
              <div class="mode-card-header">
                <input
                  type="radio"
                  name="canvas-ai-mode"
                  value="byok"
                  checked={aiConfig.mode === 'byok'}
                  onchange={() => {
                    aiConfig.mode = 'byok';
                    aiConfig.testResult = null;
                    aiConfig.saveConfig();
                  }}
                />
                <span class="mode-title-tag">🔑 BYOK Cloud API</span>
              </div>
              <p class="mode-desc">Direct provider API keys (OpenAI GPT-4o, Claude 3.5, Gemini 1.5) in OS Keyring.</p>
              <div class="mode-badge-wrap">
                <span class="badge byok-badge">Secure OS Keyring</span>
              </div>
            </label>
          </div>
        </div>

        <!-- Mode-Specific In-Canvas Configuration -->
        {#if aiConfig.mode === 'llamafile'}
          <div class="setting-group" id="setting-group-llamafile-config">
            <div class="group-header">
              <h4>Local GGUF Models & Llamafile Engine</h4>
              <button class="btn secondary" onclick={() => aiConfig.testConnection()} disabled={aiConfig.testLoading}>
                {aiConfig.testLoading ? 'Verifying...' : '⚡ Test Local Runner Connection'}
              </button>
            </div>
            <ModelDownloader />
            {#if aiConfig.testResult}
              <div class="test-result-bar" class:success={aiConfig.testResult.success} class:error={!aiConfig.testResult.success}>
                <span class="result-badge">{aiConfig.testResult.success ? '🟢 READY' : '🔴 ERROR'}</span>
                <span>{aiConfig.testResult.message}</span>
              </div>
            {/if}
          </div>
        {:else if aiConfig.mode === 'universal'}
          <div class="setting-group" id="setting-group-universal-config">
            <h4>Universal Endpoint Configuration</h4>
            <div class="setting-item">
              <label for="canvas-universal-endpoint">Endpoint URL (OpenAI Compatible / Ollama)</label>
              <div class="input-with-button">
                <input
                  id="canvas-universal-endpoint"
                  type="text"
                  class="form-input"
                  placeholder="http://localhost:11434"
                  bind:value={aiConfig.universalEndpoint}
                  onblur={() => aiConfig.saveConfig()}
                />
                <button class="btn primary" onclick={() => aiConfig.testConnection()} disabled={aiConfig.testLoading}>
                  {aiConfig.testLoading ? 'Testing...' : 'Test Connection ⚡'}
                </button>
              </div>
              <p class="help-text">Compatible with Ollama (<code>http://127.0.0.1:11434/v1</code>), LM Studio, vLLM, and LocalAI.</p>
            </div>
            {#if aiConfig.testResult}
              <div class="test-result-bar" class:success={aiConfig.testResult.success} class:error={!aiConfig.testResult.success}>
                <span class="result-badge">{aiConfig.testResult.success ? '🟢 READY' : '🔴 ERROR'}</span>
                <span>{aiConfig.testResult.message}</span>
              </div>
            {/if}
          </div>
        {:else if aiConfig.mode === 'byok'}
          <div class="setting-group" id="setting-group-byok-config">
            <h4>Bring Your Own Key (BYOK) Credentials</h4>
            <div class="byok-grid">
              <div class="setting-item">
                <label for="canvas-byok-provider">Cloud AI Provider</label>
                <select id="canvas-byok-provider" class="form-input" bind:value={aiConfig.byokProvider} onchange={() => aiConfig.saveConfig()}>
                  <option value="openai">OpenAI (GPT-4o, GPT-4o-mini)</option>
                  <option value="anthropic">Anthropic (Claude 3.5 Sonnet)</option>
                  <option value="gemini">Google (Gemini 1.5 Pro/Flash)</option>
                </select>
              </div>
              <div class="setting-item">
                <label for="canvas-byok-key">API Key (Stored in Secure OS Keyring)</label>
                <div class="input-with-button">
                  <input
                    id="canvas-byok-key"
                    type="password"
                    class="form-input"
                    placeholder={aiConfig.apiKeyPlaceholder || "sk-..."}
                    bind:value={aiConfig.pendingApiKey}
                    onblur={() => aiConfig.saveConfig()}
                  />
                  <button class="btn primary" onclick={() => aiConfig.testConnection()} disabled={aiConfig.testLoading}>
                    {aiConfig.testLoading ? 'Testing...' : 'Verify Key 🔑'}
                  </button>
                </div>
              </div>
            </div>
            {#if aiConfig.testResult}
              <div class="test-result-bar" class:success={aiConfig.testResult.success} class:error={!aiConfig.testResult.success}>
                <span class="result-badge">{aiConfig.testResult.success ? '🟢 READY' : '🔴 ERROR'}</span>
                <span>{aiConfig.testResult.message}</span>
              </div>
            {/if}
          </div>
        {/if}
      </div>

    {:else if activeCategory === 'packs'}
      <div class="category-section active-section">
        <div class="section-title-wrap">
          <div class="icon-bubble">📦</div>
          <div>
            <h3>Feature Packs & Extensions</h3>
            <p class="section-subtitle">Manage optional modular components to optimize storage and memory footprint.</p>
          </div>
        </div>

        <div class="setting-group" id="setting-group-feature-packs">
          <div class="feature-packs-grid">
            {#each appState.featurePacks as pack}
              {#if pack.id !== 'core'}
                <div class="pack-card">
                  <div class="pack-card-header">
                    <label for="pack-toggle-{pack.id}" class="pack-name">{pack.name}</label>
                    <label class="switch">
                      <input
                        type="checkbox"
                        id="pack-toggle-{pack.id}"
                        checked={pack.is_enabled}
                        onchange={async (e) => await appState.toggleFeaturePack(pack.id, e.currentTarget.checked)}
                      />
                      <span class="slider round"></span>
                    </label>
                  </div>
                  <p class="help-text">{pack.description}</p>
                  <div class="pack-footer">
                    {#if !pack.is_installed}
                      <span class="badge install-badge">Requires Download (~{pack.size_mb} MB)</span>
                    {:else}
                      <span class="badge installed-badge">✓ Installed</span>
                    {/if}
                  </div>
                </div>
              {/if}
            {/each}
          </div>
        </div>
      </div>

    {:else if activeCategory === 'ocr'}
      <div class="category-section active-section">
        <div class="section-title-wrap">
          <div class="icon-bubble">👁️</div>
          <div>
            <h3>Optical Character Recognition (OCR)</h3>
            <p class="section-subtitle">Pure-Rust offline text recognition models powered by PaddleOCR & RTen.</p>
          </div>
        </div>

        <div class="setting-group ocr-group">
          <OcrModelDownloader />
        </div>
      </div>

    {:else if activeCategory === 'general'}
      <div class="category-section active-section">
        <div class="section-title-wrap">
          <div class="icon-bubble">⚙️</div>
          <div>
            <h3>General & Storage Preferences</h3>
            <p class="section-subtitle">Manage output locations and application environment defaults.</p>
          </div>
        </div>

        <div class="setting-group">
          <h4>File Export Defaults</h4>
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
            <p class="help-text">Leave blank to default to each document's source folder.</p>
          </div>
        </div>
      </div>

    {:else if activeCategory === 'developer'}
      <div class="category-section active-section">
        <div class="section-title-wrap">
          <div class="icon-bubble">💻</div>
          <div>
            <h3>Developer Mode & Local Gateway</h3>
            <p class="section-subtitle">Expose headless HTTP REST endpoints and interactive OpenAPI Swagger UI.</p>
          </div>
        </div>

        <div class="setting-group" id="setting-group-developer-mode">
          <div class="setting-item toggle-item dev-toggle-row">
            <div class="setting-info">
              <label for="developer-mode-toggle" class="dev-toggle-label">Enable Local REST API & Swagger (Port 7823)</label>
              <p class="help-text">
                Spins up an Axum HTTP REST API and Swagger UI server on <code>127.0.0.1:7823</code> for CLI agents, Python automations, and local curl clients.
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
    {/if}
  </main>
</div>

<style>
  .settings-layout {
    display: flex;
    width: 100%;
    height: 100%;
    min-height: 100%;
    background-color: var(--bg-primary);
    overflow: hidden;
  }

  /* Left Sidebar Navigation */
  .settings-sidebar {
    width: 280px;
    min-width: 280px;
    background: var(--bg-surface);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    padding: 24px 16px;
    gap: 20px;
  }

  .settings-sidebar-header h2 {
    font-size: 1.4rem;
    font-weight: 700;
    color: var(--text-primary);
    margin: 0 0 4px 8px;
  }

  .settings-sidebar-header p {
    font-size: 0.85rem;
    color: var(--text-secondary);
    margin: 0 0 0 8px;
  }

  .settings-nav {
    display: flex;
    flex-direction: column;
    gap: 6px;
    flex: 1;
    overflow-y: auto;
  }

  .nav-tab-btn {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 12px;
    border-radius: var(--border-radius-md);
    background: transparent;
    border: 1px solid transparent;
    color: var(--text-secondary);
    cursor: pointer;
    text-align: left;
    transition: all 0.15s ease-in-out;
    width: 100%;
    position: relative;
  }

  .nav-tab-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .nav-tab-btn.active {
    background: rgba(var(--accent-rgb, 59, 130, 246), 0.12);
    border-color: rgba(var(--accent-rgb, 59, 130, 246), 0.3);
    color: var(--accent-primary, #60a5fa);
  }

  .nav-tab-icon {
    font-size: 1.25rem;
    line-height: 1;
  }

  .nav-tab-text {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
  }

  .nav-tab-label {
    font-size: 0.9rem;
    font-weight: 600;
    color: inherit;
  }

  .nav-tab-desc {
    font-size: 0.72rem;
    color: var(--text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .live-dot {
    color: #4caf50;
    font-size: 0.75rem;
    margin-left: auto;
  }

  .sidebar-footer {
    padding-top: 16px;
    border-top: 1px solid var(--border-color);
  }

  .version-chip {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 10px;
    background: var(--bg-primary);
    border-radius: 6px;
    border: 1px solid var(--border-color);
    font-size: 0.78rem;
    color: var(--text-secondary);
  }

  .local-badge {
    background: rgba(34, 197, 94, 0.15);
    color: #4ade80;
    font-size: 0.7rem;
    padding: 1px 6px;
    border-radius: 4px;
  }

  /* Right Main Canvas */
  .settings-main-canvas {
    flex: 1;
    overflow-y: auto;
    padding: 36px 48px;
    max-width: 1200px;
  }

  .category-section {
    display: flex;
    flex-direction: column;
    gap: 24px;
    animation: fadeIn 0.15s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(4px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .section-title-wrap {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-bottom: 8px;
  }

  .icon-bubble {
    font-size: 1.8rem;
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 12px;
  }

  .section-title-wrap h3 {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--text-primary);
    margin: 0;
  }

  .section-subtitle {
    font-size: 0.9rem;
    color: var(--text-secondary);
    margin: 2px 0 0 0;
  }

  .setting-group {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-lg);
    padding: 24px;
  }

  .group-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }

  .group-header h4, .setting-group h4 {
    font-size: 1.05rem;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0 0 12px 0;
  }

  /* In-Canvas AI Mode Selector */
  .ai-modes-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: 14px;
    margin-top: 14px;
  }

  .ai-mode-card {
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .ai-mode-card:hover {
    border-color: rgba(var(--accent-rgb, 59, 130, 246), 0.5);
    background: rgba(255, 255, 255, 0.02);
  }

  .ai-mode-card.active {
    border-color: var(--accent-primary, #3b82f6);
    background: rgba(59, 130, 246, 0.08);
    box-shadow: 0 0 0 1px var(--accent-primary, #3b82f6);
  }

  .mode-card-header {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .mode-card-header input[type="radio"] {
    accent-color: var(--accent-primary, #3b82f6);
    cursor: pointer;
  }

  .mode-title-tag {
    font-size: 0.98rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .mode-desc {
    font-size: 0.82rem;
    color: var(--text-secondary);
    line-height: 1.4;
    margin: 0;
  }

  .mode-badge-wrap {
    margin-top: auto;
    padding-top: 6px;
  }

  .custom-badge {
    background: rgba(59, 130, 246, 0.15);
    color: #60a5fa;
    font-size: 0.72rem;
    padding: 2px 6px;
    border-radius: 4px;
    border: 1px solid rgba(59, 130, 246, 0.3);
  }

  .byok-badge {
    background: rgba(168, 85, 247, 0.15);
    color: #c084fc;
    font-size: 0.72rem;
    padding: 2px 6px;
    border-radius: 4px;
    border: 1px solid rgba(168, 85, 247, 0.3);
  }

  .test-result-bar {
    margin-top: 14px;
    padding: 10px 14px;
    border-radius: 6px;
    font-size: 0.88rem;
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .test-result-bar.success {
    background: rgba(34, 197, 94, 0.12);
    border: 1px solid rgba(34, 197, 94, 0.3);
    color: #4ade80;
  }

  .test-result-bar.error {
    background: rgba(239, 68, 68, 0.12);
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #f87171;
  }

  .byok-grid {
    display: grid;
    grid-template-columns: 240px 1fr;
    gap: 16px;
    margin-top: 10px;
  }
  .feature-packs-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 16px;
  }

  .pack-card {
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    transition: border-color 0.15s ease;
  }

  .pack-card:hover {
    border-color: rgba(var(--accent-rgb, 59, 130, 246), 0.4);
  }

  .pack-card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .pack-name {
    font-size: 0.95rem;
    font-weight: 600;
    color: var(--text-primary);
    cursor: pointer;
  }

  .pack-footer {
    margin-top: auto;
    padding-top: 8px;
  }

  .installed-badge {
    background: rgba(34, 197, 94, 0.15);
    color: #4ade80;
    font-size: 0.75rem;
    padding: 2px 6px;
    border-radius: 4px;
    border: 1px solid rgba(34, 197, 94, 0.3);
  }

  .setting-item {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  label {
    font-size: 0.9rem;
    font-weight: 500;
    color: var(--text-primary);
  }

  .dev-toggle-row {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: flex-start;
    gap: 20px;
  }

  .dev-toggle-label {
    font-size: 1rem;
    font-weight: 600;
    margin-bottom: 4px;
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

  .pack-toggle {
    padding: 12px;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    margin-bottom: 12px;
  }

  .install-badge {
    display: inline-block;
    margin-top: 6px;
    font-size: 0.75rem;
    padding: 2px 6px;
    border-radius: 4px;
    background: rgba(245, 158, 11, 0.1);
    color: #d97706;
    border: 1px solid rgba(245, 158, 11, 0.3);
  }
</style>
