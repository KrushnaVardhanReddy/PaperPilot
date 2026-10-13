<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import AiSettingsModal from '$lib/components/settings/AiSettingsModal.svelte';
  import OcrModelDownloader from '$lib/components/settings/OcrModelDownloader.svelte';
  import { aiConfig } from '$lib/state/aiConfig.svelte';

  type SettingsCategory = 'ai' | 'packs' | 'ocr' | 'general' | 'developer';
  let activeCategory = $state<SettingsCategory>('ai');

  let isAiModalOpen = $state(false);
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
          <button class="btn primary launch-ai-btn" onclick={() => {
            aiConfig.testResult = null;
            isAiModalOpen = true;
          }}>
            Manage AI & Models 🚀
          </button>
        </div>

        <div class="setting-group" id="setting-group-ai-engine">
          <div class="group-header">
            <h4>Runtime Status</h4>
            <span class="ai-pill">Current Mode: <strong>{aiConfig.mode.toUpperCase()}</strong></span>
          </div>

          <div class="ai-status-summary">
            {#if aiConfig.mode === 'llamafile' || aiConfig.mode === 'offlinenlp'}
              <div class="status-banner local">
                <span class="status-icon">🔒</span>
                <div>
                  <strong>100% On-Device & Offline</strong>
                  <p>Inference stays completely private on CPU/GPU with zero network calls.</p>
                </div>
              </div>
            {:else if aiConfig.mode === 'universal'}
              <div class="status-banner remote">
                <span class="status-icon">🌐</span>
                <div>
                  <strong>Custom OpenAI-Compatible Endpoint</strong>
                  <p>Routed through your self-hosted vLLM, Ollama, or LiteLLM gateway.</p>
                </div>
              </div>
            {:else}
              <div class="status-banner cloud">
                <span class="status-icon">🔑</span>
                <div>
                  <strong>BYOK Cloud Provider ({aiConfig.byokProvider || 'OpenAI'})</strong>
                  <p>Authenticated using your custom API credentials.</p>
                </div>
              </div>
            {/if}
          </div>
        </div>
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

<AiSettingsModal bind:isOpen={isAiModalOpen} />

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

  .launch-ai-btn {
    margin-left: auto;
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

  .ai-status-summary {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .status-banner {
    display: flex;
    align-items: flex-start;
    gap: 14px;
    padding: 16px;
    border-radius: 8px;
  }

  .status-banner .status-icon {
    font-size: 1.4rem;
    line-height: 1;
  }

  .status-banner strong {
    font-size: 0.95rem;
    display: block;
    margin-bottom: 2px;
  }

  .status-banner p {
    font-size: 0.85rem;
    margin: 0;
    opacity: 0.85;
  }

  .status-banner.local {
    background: rgba(34, 197, 94, 0.1);
    color: #4ade80;
    border: 1px solid rgba(34, 197, 94, 0.25);
  }

  .status-banner.remote {
    background: rgba(59, 130, 246, 0.1);
    color: #60a5fa;
    border: 1px solid rgba(59, 130, 246, 0.25);
  }

  .status-banner.cloud {
    background: rgba(168, 85, 247, 0.1);
    color: #c084fc;
    border: 1px solid rgba(168, 85, 247, 0.25);
  }

  .ai-pill {
    font-size: 0.85rem;
    color: var(--text-secondary);
    background: var(--bg-primary);
    padding: 4px 10px;
    border-radius: 6px;
    border: 1px solid var(--border-color);
  }

  /* Feature Packs Grid */
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
