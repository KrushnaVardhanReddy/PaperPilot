<script lang="ts">
  import {
    buildGitHubIssueUrl,
    detectEnvironment,
    openExternalUrl,
    type FeedbackCategory,
    type FeedbackData,
    GITHUB_BASE_URL,
    GITHUB_DISCUSSIONS_URL
  } from '$lib/utils/feedback';
  import { appState } from '$lib/state/app.svelte';
  import { allTools } from '$lib/types/toolList';

  let {
    isOpen = false,
    initialCategory = 'bug',
    initialTool = '',
    initialError = '',
    onClose
  }: {
    isOpen?: boolean;
    initialCategory?: FeedbackCategory;
    initialTool?: string;
    initialError?: string;
    onClose: () => void;
  } = $props();

  let category = $state<FeedbackCategory>('bug');
  let subcategory = $state<string>('🐞 Functional Bug / Crash');
  let title = $state<string>('');
  let description = $state<string>('');
  let affectedTool = $state<string>('');
  let reproductionSteps = $state<string>('');
  let expectedBehavior = $state<string>('');
  let errorTrace = $state<string>('');

  // Performance specific
  let fileSize = $state<string>('');
  let processingTime = $state<string>('');
  let memoryUsage = $state<string>('');

  // Feature specific
  let proposedSolution = $state<string>('');
  let consideredAlternatives = $state<string>('');

  // Auto-detected environment
  let envInfo = $state<{ appVersion: string; clientType: string; osEnvironment: string }>({
    appVersion: '0.8.2',
    clientType: 'Desktop (Tauri)',
    osEnvironment: 'Linux'
  });

  // Unique list of sorted tool IDs
  const toolOptions = Array.from(new Set(allTools.map((t) => t.id))).sort();

  // Watch modal open and populate initial values
  $effect(() => {
    if (isOpen) {
      envInfo = detectEnvironment();
      category = initialCategory || 'bug';
      if (category === 'bug') {
        subcategory = '🐞 Functional Bug / Crash';
      } else if (category === 'feature') {
        subcategory = '💡 New Tool Request';
      }

      if (initialTool) {
        affectedTool = initialTool;
      } else if (appState?.activeTool?.id) {
        affectedTool = appState.activeTool.id;
      } else {
        affectedTool = '';
      }

      if (initialError) {
        errorTrace = initialError;
        if (!title) {
          title = `Error executing ${affectedTool || 'operation'}`;
        }
      }
    }
  });

  function selectCategory(cat: FeedbackCategory) {
    category = cat;
    if (cat === 'bug') {
      subcategory = '🐞 Functional Bug / Crash';
    } else if (cat === 'performance') {
      subcategory = '⚡ Performance / Slowdown / Memory';
    } else if (cat === 'feature') {
      subcategory = '💡 New Tool Request';
    } else if (cat === 'praise') {
      subcategory = '⭐ Praise & Testimonial';
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && isOpen) {
      onClose();
    }
  }

  async function handleSubmit() {
    const data: FeedbackData = {
      category,
      subcategory,
      title: title || (category === 'praise' ? 'Loving PaperPilot!' : 'Issue report'),
      description,
      affectedTool,
      reproductionSteps,
      expectedBehavior,
      errorTrace,
      fileSize,
      processingTime,
      memoryUsage,
      proposedSolution,
      consideredAlternatives,
      appVersion: envInfo.appVersion,
      clientType: envInfo.clientType,
      osEnvironment: envInfo.osEnvironment
    };

    const targetUrl = buildGitHubIssueUrl(data);
    await openExternalUrl(targetUrl);
    onClose();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
  <div class="feedback-backdrop" onclick={onClose} role="presentation">
    <div
      class="feedback-modal"
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      aria-labelledby="feedback-title"
    >
      <header class="modal-header">
        <div class="header-title-wrap">
          <span class="header-icon">💬</span>
          <div>
            <h2 id="feedback-title">Feedback & Issue Reporting</h2>
            <p class="subtitle">Direct-to-GitHub issue tracker for 100% transparent development.</p>
          </div>
        </div>
        <button class="close-btn" onclick={onClose} aria-label="Close modal">×</button>
      </header>

      <!-- Category Tabs -->
      <nav class="category-tabs" aria-label="Feedback categories">
        <button
          type="button"
          class="tab-btn"
          class:active={category === 'bug'}
          onclick={() => selectCategory('bug')}
        >
          <span class="tab-icon">🐞</span>
          <span>Bug / Glitch</span>
        </button>
        <button
          type="button"
          class="tab-btn"
          class:active={category === 'performance'}
          onclick={() => selectCategory('performance')}
        >
          <span class="tab-icon">⚡</span>
          <span>Performance</span>
        </button>
        <button
          type="button"
          class="tab-btn"
          class:active={category === 'feature'}
          onclick={() => selectCategory('feature')}
        >
          <span class="tab-icon">💡</span>
          <span>Feature Request</span>
        </button>
        <button
          type="button"
          class="tab-btn"
          class:active={category === 'praise'}
          onclick={() => selectCategory('praise')}
        >
          <span class="tab-icon">⭐</span>
          <span>Praise & Support</span>
        </button>
      </nav>

      <form class="modal-body" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
        <!-- Subcategory picker -->
        {#if category === 'bug'}
          <div class="form-group">
            <label for="bug-subcategory">Bug Classification</label>
            <select id="bug-subcategory" bind:value={subcategory}>
              <option value="🐞 Functional Bug / Crash">🐞 Functional Bug / Crash (Operation failed, corrupted output)</option>
              <option value="⚡ Performance / Slowdown / Memory">⚡ Performance / Slowdown / Memory (Lag, freeze)</option>
              <option value="🖥️ UI / Layout Glitch">🖥️ UI / Layout Glitch (Misaligned buttons, dark/light theme)</option>
            </select>
          </div>
        {:else if category === 'feature'}
          <div class="form-group">
            <label for="feature-subcategory">Request Type</label>
            <select id="feature-subcategory" bind:value={subcategory}>
              <option value="💡 New Tool Request">💡 New Tool Request (Tool not yet in the 101 suite)</option>
              <option value="🚀 Enhance Existing Tool">🚀 Enhance Existing Tool (Extra options, parameter controls)</option>
              <option value="💬 General Feedback / UX Improvement">💬 General Feedback / UX Improvement</option>
            </select>
          </div>
        {/if}

        {#if category === 'praise'}
          <div class="praise-card">
            <div class="praise-banner">
              <span class="heart-icon">❤️</span>
              <div>
                <h3>Thank you for supporting PaperPilot!</h3>
                <p>PaperPilot is built as a zero-cloud, 100% private PDF workspace for developers, power users, and AI agents.</p>
              </div>
            </div>

            <div class="form-group">
              <label for="praise-title">Shoutout / Headline (Optional)</label>
              <input
                id="praise-title"
                type="text"
                bind:value={title}
                placeholder="e.g. Blazingly fast tool replacement!"
              />
            </div>

            <div class="form-group">
              <label for="praise-desc">Your Testimonial / Story</label>
              <textarea
                id="praise-desc"
                rows="3"
                bind:value={description}
                placeholder="What workflow did PaperPilot simplify for you? How did it help your team or agents?"
              ></textarea>
            </div>

            <div class="praise-actions">
              <button
                type="button"
                class="secondary-btn"
                onclick={() => openExternalUrl(GITHUB_BASE_URL)}
              >
                ⭐ Star PaperPilot on GitHub
              </button>
              <button
                type="button"
                class="secondary-btn"
                onclick={() => openExternalUrl(`${GITHUB_DISCUSSIONS_URL}/categories/show-and-tell`)}
              >
                💬 Read Community Showcase
              </button>
            </div>
          </div>
        {:else}
          <!-- Title / Summary -->
          <div class="form-group">
            <label for="feedback-title-input">
              {category === 'feature' ? 'Feature Title / Concept' : 'Issue Summary'} <span class="required">*</span>
            </label>
            <input
              id="feedback-title-input"
              type="text"
              required
              bind:value={title}
              placeholder={category === 'feature' ? 'e.g. Add Bates Numbering or CAD DWG to PDF' : 'e.g. Failed to split encrypted PDF with outline'}
            />
          </div>

          <!-- Affected Tool -->
          <div class="form-row">
            <div class="form-group flex-1">
              <label for="affected-tool-select">Affected / Targeted Tool</label>
              <select id="affected-tool-select" bind:value={affectedTool}>
                <option value="">(None / General Workspace)</option>
                {#each toolOptions as toolId}
                  <option value={toolId}>{toolId}</option>
                {/each}
              </select>
            </div>

            {#if category === 'performance'}
              <div class="form-group flex-1">
                <label for="perf-file-size">Input File Size / Pages</label>
                <input
                  id="perf-file-size"
                  type="text"
                  bind:value={fileSize}
                  placeholder="e.g. 150 MB, 1,200 pages"
                />
              </div>
            {/if}
          </div>

          {#if category === 'performance'}
            <div class="form-row">
              <div class="form-group flex-1">
                <label for="perf-time">Observed Processing Time</label>
                <input
                  id="perf-time"
                  type="text"
                  bind:value={processingTime}
                  placeholder="e.g. 45s or UI froze for 10s"
                />
              </div>
              <div class="form-group flex-1">
                <label for="perf-memory">RAM / Memory Observed</label>
                <input
                  id="perf-memory"
                  type="text"
                  bind:value={memoryUsage}
                  placeholder="e.g. Spiked to 2.4 GB"
                />
              </div>
            </div>
          {/if}

          <!-- Description -->
          <div class="form-group">
            <label for="feedback-description">
              {category === 'feature' ? 'Problem Statement & Proposed Solution' : 'What Happened?'} <span class="required">*</span>
            </label>
            <textarea
              id="feedback-description"
              rows="3"
              required
              bind:value={description}
              placeholder={category === 'feature'
                ? 'Describe the use case and what CLI / UI / MCP capabilities are desired...'
                : 'Describe what you were attempting to do and the failure observed...'}
            ></textarea>
          </div>

          {#if category === 'feature'}
            <div class="form-group">
              <label for="considered-alternatives">Considered Alternatives (Optional)</label>
              <input
                id="considered-alternatives"
                type="text"
                bind:value={consideredAlternatives}
                placeholder="e.g. Currently using Poppler, PDFtk, or Acrobat"
              />
            </div>
          {:else}
            <!-- Reproduction steps -->
            <div class="form-group">
              <label for="repro-steps">Steps to Reproduce (Optional)</label>
              <textarea
                id="repro-steps"
                rows="2"
                bind:value={reproductionSteps}
                placeholder="1. Open document&#10;2. Run operation with settings...&#10;3. See error"
              ></textarea>
            </div>

            <!-- Error trace -->
            {#if errorTrace || category === 'bug'}
              <div class="form-group">
                <label for="error-trace">Error Logs / Stack Trace (Sanitized)</label>
                <textarea
                  id="error-trace"
                  class="font-mono text-xs"
                  rows="2"
                  bind:value={errorTrace}
                  placeholder="Paste terminal logs or error details..."
                ></textarea>
              </div>
            {/if}
          {/if}
        {/if}

        <!-- Auto-collected environment info -->
        <div class="env-info-pill">
          <span class="env-badge">🔒 Privacy Preserved</span>
          <span class="env-item">Client: <strong>{envInfo.clientType}</strong></span>
          <span class="env-sep">•</span>
          <span class="env-item">OS: <strong>{envInfo.osEnvironment}</strong></span>
          <span class="env-sep">•</span>
          <span class="env-item">Version: <strong>v{envInfo.appVersion}</strong></span>
        </div>

        <!-- Footer Actions -->
        <footer class="modal-footer">
          <button type="button" class="cancel-btn" onclick={onClose}>
            Cancel
          </button>

          {#if category === 'praise'}
            <button
              type="submit"
              class="primary-submit-btn"
            >
              <span>Share on GitHub Discussions</span>
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
                <polyline points="15 3 21 3 21 9"></polyline>
                <line x1="10" y1="14" x2="21" y2="3"></line>
              </svg>
            </button>
          {:else}
            <button
              type="submit"
              class="primary-submit-btn"
            >
              <span>Submit on GitHub</span>
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
                <polyline points="15 3 21 3 21 9"></polyline>
                <line x1="10" y1="14" x2="21" y2="3"></line>
              </svg>
            </button>
          {/if}
        </footer>
      </form>
    </div>
  </div>
{/if}

<style>
  .feedback-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.75);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
    padding: 16px;
  }

  .feedback-modal {
    background: var(--bg-secondary, #1e293b);
    border: 1px solid var(--border-color, #334155);
    border-radius: 12px;
    width: 100%;
    max-width: 620px;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5), 0 8px 10px -6px rgba(0, 0, 0, 0.5);
    overflow: hidden;
    color: var(--text-primary, #f8fafc);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 16px 20px;
    border-bottom: 1px solid var(--border-color, #334155);
    background: var(--bg-surface, #0f172a);
  }

  .header-title-wrap {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .header-icon {
    font-size: 24px;
  }

  .modal-header h2 {
    margin: 0;
    font-size: 1.15rem;
    font-weight: 600;
  }

  .subtitle {
    margin: 2px 0 0 0;
    font-size: 0.8rem;
    color: var(--text-secondary, #94a3b8);
  }

  .close-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary, #94a3b8);
    font-size: 24px;
    cursor: pointer;
    line-height: 1;
    padding: 4px;
    border-radius: 4px;
  }

  .close-btn:hover {
    color: var(--text-primary, #f8fafc);
    background: rgba(255, 255, 255, 0.1);
  }

  .category-tabs {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    background: var(--bg-surface, #0f172a);
    border-bottom: 1px solid var(--border-color, #334155);
    padding: 4px 8px;
    gap: 4px;
  }

  .tab-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 4px;
    background: transparent;
    border: none;
    padding: 8px 4px;
    border-radius: 6px;
    color: var(--text-secondary, #94a3b8);
    font-size: 0.75rem;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .tab-btn:hover {
    color: var(--text-primary, #f8fafc);
    background: rgba(255, 255, 255, 0.05);
  }

  .tab-btn.active {
    background: var(--bg-secondary, #1e293b);
    color: #38bdf8;
    font-weight: 600;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
  }

  .tab-icon {
    font-size: 1.1rem;
  }

  .modal-body {
    padding: 16px 20px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .form-row {
    display: flex;
    gap: 12px;
  }

  .flex-1 {
    flex: 1;
  }

  label {
    font-size: 0.8rem;
    font-weight: 500;
    color: var(--text-secondary, #94a3b8);
  }

  .required {
    color: #ef4444;
  }

  input, select, textarea {
    background: var(--bg-surface, #0f172a);
    border: 1px solid var(--border-color, #334155);
    color: var(--text-primary, #f8fafc);
    border-radius: 6px;
    padding: 8px 10px;
    font-size: 0.85rem;
    outline: none;
    font-family: inherit;
    transition: border-color 0.15s ease;
  }

  input:focus, select:focus, textarea:focus {
    border-color: #38bdf8;
    box-shadow: 0 0 0 1px #38bdf8;
  }

  .praise-card {
    display: flex;
    flex-direction: column;
    gap: 14px;
    background: rgba(56, 189, 248, 0.05);
    border: 1px dashed rgba(56, 189, 248, 0.3);
    border-radius: 8px;
    padding: 16px;
  }

  .praise-banner {
    display: flex;
    align-items: flex-start;
    gap: 12px;
  }

  .heart-icon {
    font-size: 24px;
  }

  .praise-banner h3 {
    margin: 0;
    font-size: 0.95rem;
    color: var(--text-primary, #f8fafc);
  }

  .praise-banner p {
    margin: 4px 0 0 0;
    font-size: 0.8rem;
    color: var(--text-secondary, #94a3b8);
  }

  .praise-actions {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .secondary-btn {
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid var(--border-color, #334155);
    color: var(--text-primary, #f8fafc);
    border-radius: 6px;
    padding: 6px 12px;
    font-size: 0.8rem;
    cursor: pointer;
    transition: background 0.15s;
  }

  .secondary-btn:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  .env-info-pill {
    display: flex;
    align-items: center;
    gap: 8px;
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid var(--border-color, #334155);
    border-radius: 6px;
    padding: 6px 10px;
    font-size: 0.75rem;
    color: var(--text-secondary, #94a3b8);
  }

  .env-badge {
    background: rgba(16, 185, 129, 0.15);
    color: #10b981;
    font-weight: 500;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 0.7rem;
  }

  .env-sep {
    opacity: 0.4;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 8px;
    padding-top: 12px;
    border-top: 1px solid var(--border-color, #334155);
  }

  .cancel-btn {
    background: transparent;
    border: 1px solid var(--border-color, #334155);
    color: var(--text-secondary, #94a3b8);
    border-radius: 6px;
    padding: 8px 14px;
    font-size: 0.85rem;
    cursor: pointer;
  }

  .cancel-btn:hover {
    color: var(--text-primary, #f8fafc);
    background: rgba(255, 255, 255, 0.05);
  }

  .primary-submit-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    background: #0284c7;
    border: none;
    color: #ffffff;
    font-weight: 500;
    border-radius: 6px;
    padding: 8px 16px;
    font-size: 0.85rem;
    cursor: pointer;
    transition: background 0.15s;
  }

  .primary-submit-btn:hover {
    background: #0369a1;
  }
</style>
