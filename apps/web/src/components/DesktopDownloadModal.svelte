<script lang="ts">
  import {
    DOWNLOAD_URL,
    DOWNLOAD_LINUX_URL,
    DOWNLOAD_WINDOWS_URL,
    DOWNLOAD_MACOS_URL
  } from '../constants';

  interface Props {
    isOpen: boolean;
    onClose: () => void;
  }

  let { isOpen = $bindable(false), onClose }: Props = $props();

  const platforms = [
    {
      name: 'Windows',
      icon: '🪟',
      desc: 'Windows 10 / 11 (x64, ARM64)',
      btnText: 'Download .msi / .exe',
      url: DOWNLOAD_WINDOWS_URL,
      tag: 'Installer'
    },
    {
      name: 'macOS',
      icon: '🍎',
      desc: 'Apple Silicon (M1/M2/M3/M4) & Intel',
      btnText: 'Download .dmg',
      url: DOWNLOAD_MACOS_URL,
      tag: 'Universal'
    },
    {
      name: 'Linux',
      icon: '🐧',
      desc: 'Ubuntu, Fedora, Arch, Debian',
      btnText: 'Download .AppImage / .deb',
      url: DOWNLOAD_LINUX_URL,
      tag: 'AppImage / Deb'
    }
  ];
</script>

{#if isOpen}
  <div class="download-backdrop" onclick={onClose} role="presentation">
    <!-- svelte-ignore a11y_interactive_supports_focus -->
    <div
      class="download-modal"
      role="dialog"
      tabindex="-1"
      aria-modal="true"
      aria-label="Download PaperPilot Desktop"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="modal-header">
        <div class="header-titles">
          <h3>⚡ Download PaperPilot Desktop</h3>
          <p>Unrestricted native speed, offline OCR models, folder watchers, and batch automation.</p>
        </div>
        <button class="close-x" onclick={onClose} aria-label="Close modal">✕</button>
      </div>

      <div class="modal-body">
        <div class="platforms-grid">
          {#each platforms as p}
            <div class="platform-card">
              <div class="platform-head">
                <span class="plat-icon">{p.icon}</span>
                <div>
                  <h4 class="plat-name">{p.name}</h4>
                  <span class="plat-tag">{p.tag}</span>
                </div>
              </div>
              <p class="plat-desc">{p.desc}</p>
              <a
                href={p.url}
                target="_blank"
                rel="noopener noreferrer"
                class="download-action-btn"
              >
                <span>{p.btnText}</span>
                <span class="btn-arrow">↓</span>
              </a>
            </div>
          {/each}
        </div>

        <div class="desktop-perks">
          <div class="perk-item">
            <span class="perk-icon">🧠</span>
            <div class="perk-text">
              <strong>Offline Neural OCR & SLM</strong>
              <p>Run full document intelligence models without internet access.</p>
            </div>
          </div>
          <div class="perk-item">
            <span class="perk-icon">📂</span>
            <div class="perk-text">
              <strong>Hot-Folder Watchers</strong>
              <p>Auto-process scanned receipts and incoming PDFs in background folders.</p>
            </div>
          </div>
          <div class="perk-item">
            <span class="perk-icon">🛡️</span>
            <div class="perk-text">
              <strong>Zero Telemetry</strong>
              <p>Free, open source, and 100% telemetry-free.</p>
            </div>
          </div>
        </div>
      </div>

      <div class="modal-footer">
        <span class="release-info">
          Latest stable release on GitHub • Fully Open Source (Apache-2.0)
        </span>
        <a
          href={DOWNLOAD_URL}
          target="_blank"
          rel="noopener noreferrer"
          class="github-releases-link"
        >
          View all releases & source →
        </a>
      </div>
    </div>
  </div>
{/if}

<style>
  .download-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.75);
    backdrop-filter: blur(6px);
    z-index: 9999;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px;
    animation: fadeIn 0.15s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .download-modal {
    background: var(--bg-secondary, #18181b);
    border: 1px solid var(--border-color, #27272a);
    border-radius: 18px;
    width: 100%;
    max-width: 680px;
    box-shadow: 0 25px 60px -15px rgba(0, 0, 0, 0.8);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    outline: none;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 24px 28px 16px;
    border-bottom: 1px solid var(--border-color, #27272a);
    background: var(--bg-primary, #09090b);
  }

  .header-titles h3 {
    margin: 0 0 6px 0;
    font-size: 1.35rem;
    color: var(--text-primary, #f4f4f5);
  }

  .header-titles p {
    margin: 0;
    font-size: 0.88rem;
    color: var(--text-secondary, #a1a1aa);
  }

  .close-x {
    background: transparent;
    border: none;
    font-size: 1.25rem;
    color: var(--text-secondary, #71717a);
    cursor: pointer;
    padding: 4px;
    border-radius: 6px;
    line-height: 1;
  }

  .close-x:hover {
    color: var(--text-primary, #f4f4f5);
    background: rgba(255, 255, 255, 0.08);
  }

  .modal-body {
    padding: 24px 28px;
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .platforms-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 16px;
  }

  @media (max-width: 640px) {
    .platforms-grid {
      grid-template-columns: 1fr;
    }
  }

  .platform-card {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 12px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    transition: all 0.2s ease;
  }

  .platform-card:hover {
    border-color: rgba(94, 106, 210, 0.4);
    background: rgba(94, 106, 210, 0.05);
  }

  .platform-head {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .plat-icon {
    font-size: 1.75rem;
  }

  .plat-name {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary, #f4f4f5);
  }

  .plat-tag {
    font-size: 0.7rem;
    color: var(--text-secondary, #a1a1aa);
    background: rgba(255, 255, 255, 0.06);
    padding: 1px 6px;
    border-radius: 4px;
  }

  .plat-desc {
    margin: 0;
    font-size: 0.78rem;
    color: var(--text-secondary, #a1a1aa);
    line-height: 1.35;
    flex: 1;
  }

  .download-action-btn {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    background: var(--accent-primary, #6366f1);
    color: white;
    text-decoration: none;
    border-radius: 8px;
    font-size: 0.85rem;
    font-weight: 600;
    transition: background 0.15s ease;
  }

  .download-action-btn:hover {
    background: var(--accent-hover, #4f46e5);
  }

  .desktop-perks {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 16px;
    padding-top: 16px;
    border-top: 1px solid var(--border-color, #27272a);
  }

  @media (max-width: 640px) {
    .desktop-perks {
      grid-template-columns: 1fr;
    }
  }

  .perk-item {
    display: flex;
    gap: 10px;
  }

  .perk-icon {
    font-size: 1.25rem;
    flex-shrink: 0;
  }

  .perk-text strong {
    display: block;
    font-size: 0.82rem;
    color: var(--text-primary, #f4f4f5);
    margin-bottom: 2px;
  }

  .perk-text p {
    margin: 0;
    font-size: 0.75rem;
    color: var(--text-secondary, #a1a1aa);
    line-height: 1.3;
  }

  .modal-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 14px 28px;
    background: var(--bg-primary, #09090b);
    border-top: 1px solid var(--border-color, #27272a);
    font-size: 0.8rem;
    color: var(--text-secondary, #71717a);
    flex-wrap: wrap;
    gap: 8px;
  }

  .github-releases-link {
    color: var(--accent-primary, #6366f1);
    text-decoration: none;
    font-weight: 500;
  }

  .github-releases-link:hover {
    text-decoration: underline;
  }
</style>
