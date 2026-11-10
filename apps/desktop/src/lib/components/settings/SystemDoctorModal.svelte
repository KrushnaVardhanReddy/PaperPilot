<script lang="ts">
  import { onMount } from 'svelte';
  import { safeInvoke } from '$lib/utils/tauri';
  import { fade, slide } from 'svelte/transition';

  interface CheckResult {
    name: string;
    passed: boolean;
    message: string;
    suggestion: string | null;
  }

  interface DiagnosticReport {
    timestamp: string;
    status: 'healthy' | 'degraded' | 'unhealthy';
    checks: CheckResult[];
  }

  let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

  let report = $state<DiagnosticReport | null>(null);
  let loading = $state(false);
  let errorMessage = $state<string | null>(null);
  let showJsonView = $state(false);
  let copyFeedback = $state(false);

  async function runDiagnostics() {
    loading = true;
    errorMessage = null;
    try {
      // safeInvoke returns null if window.__TAURI_INTERNALS__ is not present (e.g. dev browser or tests)
      const res = await safeInvoke('run_system_doctor');
      if (res) {
        report = res as DiagnosticReport;
      } else {
        // Fallback for browser mock/preview
        report = {
          timestamp: new Date().toISOString(),
          status: 'healthy',
          checks: [
            {
              name: 'temp_dir',
              passed: true,
              message: 'Temp directory /tmp is writable and ephemeral cleanup verified',
              suggestion: null
            },
            {
              name: 'working_dir',
              passed: true,
              message: 'Working directory read/write verified',
              suggestion: null
            },
            {
              name: 'gateway_port',
              passed: true,
              message: 'Gateway port 7823 is available for local REST API',
              suggestion: null
            },
            {
              name: 'ocr_models',
              passed: true,
              message: 'OCR models located at models/ocr/ppocrv4 (detection & recognition weights verified)',
              suggestion: null
            },
            {
              name: 'disk_space',
              passed: true,
              message: 'Scratch storage has >5 GB available',
              suggestion: null
            }
          ]
        };
      }
    } catch (e: any) {
      errorMessage = e?.toString() || 'Failed to run diagnostics';
    } finally {
      loading = false;
    }
  }

  function formatTitle(name: string): string {
    switch (name) {
      case 'temp_dir': return 'Temp Scratch Storage';
      case 'working_dir': return 'Working Directory Permissions';
      case 'gateway_port': return 'REST Gateway Port (7823)';
      case 'ocr_models': return 'OCR & Vision Model Weights';
      case 'disk_space': return 'Scratch Disk Space';
      default: return name;
    }
  }

  function getStatusBadge(status: string) {
    if (status === 'healthy') return { text: 'HEALTHY', color: 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30' };
    if (status === 'degraded') return { text: 'DEGRADED', color: 'bg-amber-500/20 text-amber-400 border-amber-500/30' };
    return { text: 'UNHEALTHY', color: 'bg-rose-500/20 text-rose-400 border-rose-500/30' };
  }

  async function copyJson() {
    if (!report) return;
    try {
      await navigator.clipboard.writeText(JSON.stringify(report, null, 2));
      copyFeedback = true;
      setTimeout(() => { copyFeedback = false; }, 2000);
    } catch (e) {
      console.error(e);
    }
  }

  $effect(() => {
    if (isOpen) {
      runDiagnostics();
    }
  });
</script>

{#if isOpen}
  <div class="modal-backdrop" transition:fade={{ duration: 150 }}>
    <div class="modal-card" transition:slide={{ duration: 200 }}>
      <!-- Header -->
      <div class="modal-header">
        <div class="header-left">
          <span class="icon-bubble">🩺</span>
          <div>
            <h3 class="modal-title">PaperPilot System Doctor</h3>
            <p class="modal-subtitle">Comprehensive native environment and system diagnostics</p>
          </div>
        </div>
        <button class="close-btn" onclick={() => (isOpen = false)} aria-label="Close dialog">✕</button>
      </div>

      <!-- Body -->
      <div class="modal-body">
        {#if loading}
          <div class="loading-state">
            <div class="spinner"></div>
            <p>Probing native filesystem, ports, models, and storage...</p>
          </div>
        {:else if errorMessage}
          <div class="error-banner">
            <span>⚠️ {errorMessage}</span>
            <button class="btn retry-btn" onclick={runDiagnostics}>Retry</button>
          </div>
        {:else if report}
          <!-- Status Banner -->
          <div class="summary-card" class:healthy={report.status === 'healthy'} class:degraded={report.status === 'degraded'} class:unhealthy={report.status === 'unhealthy'}>
            <div class="summary-info">
              <span class="summary-status">
                {#if report.status === 'healthy'}
                  🟢 All Systems Operational
                {:else if report.status === 'degraded'}
                  🟡 Degraded Performance / Recommendations
                {:else}
                  🔴 Critical Issues Detected
                {/if}
              </span>
              <span class="timestamp-label">Checked at: {report.timestamp}</span>
            </div>
            <div class="summary-actions">
              <button class="toggle-btn" onclick={() => (showJsonView = !showJsonView)}>
                {showJsonView ? 'Visual View' : 'Raw JSON'}
              </button>
              <button class="action-btn" onclick={runDiagnostics} title="Rerun Probes">
                🔄 Re-run
              </button>
            </div>
          </div>

          {#if showJsonView}
            <div class="json-container">
              <div class="json-header">
                <span>DiagnosticReport JSON</span>
                <button class="copy-btn" onclick={copyJson}>
                  {copyFeedback ? '✓ Copied!' : 'Copy JSON'}
                </button>
              </div>
              <pre class="json-view"><code>{JSON.stringify(report, null, 2)}</code></pre>
            </div>
          {:else}
            <!-- Checks List -->
            <div class="checks-list">
              {#each report.checks as check}
                <div class="check-item" class:passed={check.passed} class:failed={!check.passed}>
                  <div class="check-icon">
                    {#if check.passed}
                      <span class="badge-pass">✓</span>
                    {:else if check.name === 'ocr_models' || check.name === 'gateway_port'}
                      <span class="badge-warn">!</span>
                    {:else}
                      <span class="badge-fail">✕</span>
                    {/if}
                  </div>
                  <div class="check-details">
                    <div class="check-title-row">
                      <span class="check-title">{formatTitle(check.name)}</span>
                      <span class="status-tag" class:tag-pass={check.passed} class:tag-warn={!check.passed && (check.name === 'ocr_models' || check.name === 'gateway_port')} class:tag-fail={!check.passed && check.name !== 'ocr_models' && check.name !== 'gateway_port'}>
                        {check.passed ? 'PASSED' : (check.name === 'ocr_models' || check.name === 'gateway_port' ? 'WARNING' : 'FAILED')}
                      </span>
                    </div>
                    <p class="check-message">{check.message}</p>
                    {#if check.suggestion}
                      <div class="suggestion-box">
                        <span class="suggestion-icon">↳ Fix:</span>
                        <span class="suggestion-text">{check.suggestion}</span>
                      </div>
                    {/if}
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        {/if}
      </div>

      <!-- Footer -->
      <div class="modal-footer">
        <span class="footer-hint">PaperPilot v0.1.0 • Pure-Rust Core Engine</span>
        <button class="btn secondary-btn" onclick={() => (isOpen = false)}>Close</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-card {
    background: var(--bg-surface, #1e222b);
    border: 1px solid var(--border-color, #2d3343);
    border-radius: 12px;
    width: 90%;
    max-width: 680px;
    max-height: 85vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.5);
    color: var(--text-primary, #e2e8f0);
    overflow: hidden;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid var(--border-color, #2d3343);
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .icon-bubble {
    font-size: 1.5rem;
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-primary, #14171f);
    border: 1px solid var(--border-color, #2d3343);
    border-radius: 10px;
  }

  .modal-title {
    font-size: 1.15rem;
    font-weight: 700;
    margin: 0;
  }

  .modal-subtitle {
    font-size: 0.8rem;
    color: var(--text-secondary, #94a3b8);
    margin: 2px 0 0 0;
  }

  .close-btn {
    background: transparent;
    border: none;
    color: var(--text-muted, #64748b);
    font-size: 1.25rem;
    cursor: pointer;
    padding: 4px;
    transition: color 0.15s;
  }

  .close-btn:hover {
    color: var(--text-primary, #e2e8f0);
  }

  .modal-body {
    padding: 20px;
    overflow-y: auto;
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 40px 0;
    gap: 12px;
    color: var(--text-secondary, #94a3b8);
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid rgba(255, 255, 255, 0.1);
    border-top-color: var(--accent-primary, #3b82f6);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .summary-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    border-radius: 8px;
    border: 1px solid var(--border-color, #2d3343);
    background: var(--bg-primary, #14171f);
  }

  .summary-card.healthy {
    border-color: rgba(34, 197, 94, 0.4);
    background: rgba(34, 197, 94, 0.08);
  }

  .summary-card.degraded {
    border-color: rgba(245, 158, 11, 0.4);
    background: rgba(245, 158, 11, 0.08);
  }

  .summary-card.unhealthy {
    border-color: rgba(239, 68, 68, 0.4);
    background: rgba(239, 68, 68, 0.08);
  }

  .summary-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .summary-status {
    font-size: 0.95rem;
    font-weight: 600;
  }

  .timestamp-label {
    font-size: 0.75rem;
    color: var(--text-muted, #64748b);
  }

  .summary-actions {
    display: flex;
    gap: 8px;
  }

  .action-btn, .toggle-btn {
    padding: 6px 12px;
    background: var(--bg-surface, #1e222b);
    border: 1px solid var(--border-color, #2d3343);
    color: var(--text-primary, #e2e8f0);
    border-radius: 6px;
    font-size: 0.8rem;
    cursor: pointer;
    transition: background 0.15s;
  }

  .action-btn:hover, .toggle-btn:hover {
    background: var(--border-color, #2d3343);
  }

  .checks-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .check-item {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 12px 14px;
    border-radius: 8px;
    border: 1px solid var(--border-color, #2d3343);
    background: var(--bg-primary, #14171f);
  }

  .check-icon {
    font-weight: bold;
    font-size: 1rem;
    line-height: 1.2;
    padding-top: 2px;
  }

  .badge-pass { color: #4ade80; }
  .badge-warn { color: #fbbf24; }
  .badge-fail { color: #f87171; }

  .check-details {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .check-title-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .check-title {
    font-weight: 600;
    font-size: 0.9rem;
    color: var(--text-primary, #e2e8f0);
  }

  .status-tag {
    font-size: 0.68rem;
    font-weight: 700;
    padding: 2px 6px;
    border-radius: 4px;
    letter-spacing: 0.05em;
  }

  .tag-pass { background: rgba(34, 197, 94, 0.15); color: #4ade80; border: 1px solid rgba(34, 197, 94, 0.3); }
  .tag-warn { background: rgba(245, 158, 11, 0.15); color: #fbbf24; border: 1px solid rgba(245, 158, 11, 0.3); }
  .tag-fail { background: rgba(239, 68, 68, 0.15); color: #f87171; border: 1px solid rgba(239, 68, 68, 0.3); }

  .check-message {
    font-size: 0.82rem;
    color: var(--text-secondary, #94a3b8);
    margin: 0;
    line-height: 1.4;
  }

  .suggestion-box {
    margin-top: 4px;
    padding: 6px 10px;
    background: rgba(245, 158, 11, 0.08);
    border-left: 3px solid #fbbf24;
    border-radius: 4px;
    font-size: 0.78rem;
    display: flex;
    gap: 6px;
  }

  .suggestion-icon {
    font-weight: 700;
    color: #fbbf24;
  }

  .suggestion-text {
    color: #e2e8f0;
  }

  .json-container {
    display: flex;
    flex-direction: column;
    border: 1px solid var(--border-color, #2d3343);
    border-radius: 8px;
    background: #0f1117;
    overflow: hidden;
  }

  .json-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 12px;
    background: #14171f;
    border-bottom: 1px solid var(--border-color, #2d3343);
    font-size: 0.78rem;
    color: var(--text-secondary, #94a3b8);
  }

  .copy-btn {
    padding: 3px 8px;
    background: transparent;
    border: 1px solid var(--border-color, #2d3343);
    color: var(--text-primary, #e2e8f0);
    border-radius: 4px;
    font-size: 0.72rem;
    cursor: pointer;
  }

  .copy-btn:hover {
    background: var(--bg-surface, #1e222b);
  }

  .json-view {
    padding: 12px;
    margin: 0;
    font-family: monospace;
    font-size: 0.78rem;
    color: #38bdf8;
    overflow-x: auto;
    max-height: 350px;
  }

  .modal-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 20px;
    border-top: 1px solid var(--border-color, #2d3343);
  }

  .footer-hint {
    font-size: 0.75rem;
    color: var(--text-muted, #64748b);
  }

  .btn {
    padding: 8px 16px;
    border-radius: 6px;
    font-size: 0.88rem;
    font-weight: 600;
    cursor: pointer;
    border: none;
    transition: all 0.15s;
  }

  .secondary-btn {
    background: var(--bg-primary, #14171f);
    color: var(--text-primary, #e2e8f0);
    border: 1px solid var(--border-color, #2d3343);
  }

  .secondary-btn:hover {
    background: var(--border-color, #2d3343);
  }

  .retry-btn {
    background: var(--accent-primary, #3b82f6);
    color: white;
  }
</style>
