<script lang="ts">
  import { onMount } from 'svelte';
  import MegaMenu from './MegaMenu.svelte';
  import FeaturesDropdown from './FeaturesDropdown.svelte';
  import DesktopDownloadModal from './DesktopDownloadModal.svelte';
  import ToolQuickFinderModal from './ToolQuickFinderModal.svelte';
  import DownloadBadge from './DownloadBadge.svelte';
  import ShareModal from './ShareModal.svelte';
  import {
    GITHUB_REPO_URL,
    DOWNLOAD_URL,
    getDocsBaseUrl,
    ENABLE_DOWNLOAD_COUNT_BADGE,
    MIN_DOWNLOAD_COUNT_THRESHOLD
  } from '../constants';

  interface Props {
    onOpenFeedback: (category?: 'bug' | 'performance' | 'feature' | 'praise') => void;
  }

  let { onOpenFeedback }: Props = $props();

  let isMegaMenuOpen = $state(false);
  let isFeaturesOpen = $state(false);
  let isDownloadModalOpen = $state(false);
  let isQuickFinderOpen = $state(false);
  let downloadCount = $state<number | null>(null);

  const docsUrl = getDocsBaseUrl();

  onMount(async () => {
    if (!ENABLE_DOWNLOAD_COUNT_BADGE) return;

    const repo = 'KrushnaVardhanReddy/PaperPilot';
    const npmPackage = '@usepaperpilot/mcp';

    try {
      const cacheKey = `eco_dls_${repo}_${npmPackage}`;
      const cached = sessionStorage.getItem(cacheKey);
      if (cached) {
        const parsed = JSON.parse(cached);
        if (Date.now() - parsed.timestamp < 1000 * 60 * 15) {
          downloadCount = parsed.count;
          return;
        }
      }

      const [ghRes, npmRes] = await Promise.allSettled([
        fetch(`https://api.github.com/repos/${repo}/releases`),
        fetch(`https://api.npmjs.org/downloads/range/2026-01-01:${new Date().toISOString().slice(0, 10)}/${npmPackage}`)
      ]);

      let ghTotal = 0;
      let npmTotal = 0;
      let latestReleaseTag: string | null = null;

      if (ghRes.status === 'fulfilled' && ghRes.value.ok) {
        const releases = await ghRes.value.json();
        if (Array.isArray(releases) && releases.length > 0) {
          latestReleaseTag = releases[0].tag_name || releases[0].name;
          for (const rel of releases) {
            if (Array.isArray(rel.assets)) {
              for (const asset of rel.assets) {
                ghTotal += Number(asset.download_count) || 0;
              }
            }
          }
        }
      }

      if (npmRes.status === 'fulfilled' && npmRes.value.ok) {
        const npmData = await npmRes.value.json();
        if (Array.isArray(npmData?.downloads)) {
          for (const item of npmData.downloads) {
            npmTotal += Number(item.downloads) || 0;
          }
        }
      }

      const total = ghTotal + npmTotal;
      if (total > 0) {
        downloadCount = total;
        sessionStorage.setItem(
          cacheKey,
          JSON.stringify({
            count: total,
            ghCount: ghTotal,
            npmCount: npmTotal,
            tag: latestReleaseTag,
            timestamp: Date.now()
          })
        );
      }
    } catch {
      // Fallback silently if offline or rate limited
    }
  });

  function formatCount(count: number): string {
    if (count >= 1_000_000) {
      const millions = Math.floor(count / 100_000) / 10;
      return `${millions % 1 === 0 ? millions.toFixed(0) : millions.toFixed(1)}M+`;
    }
    if (count >= 1_000) {
      const thousands = Math.floor(count / 100) / 10;
      return `${thousands % 1 === 0 ? thousands.toFixed(0) : thousands.toFixed(1)}K+`;
    }
    return count.toLocaleString();
  }

  function toggleMegaMenu() {
    isMegaMenuOpen = !isMegaMenuOpen;
    if (isMegaMenuOpen) isFeaturesOpen = false;
  }

  function toggleFeatures() {
    isFeaturesOpen = !isFeaturesOpen;
    if (isFeaturesOpen) isMegaMenuOpen = false;
  }
</script>

<nav class="paperpilot-navbar">
  <div class="navbar-container">
    <!-- Left: Brand Logo & Web Badge -->
    <div class="navbar-brand-section">
      <a href="/" class="brand-logo" onclick={(e) => { e.preventDefault(); window.scrollTo({ top: 0, behavior: 'smooth' }); }}>
        <img src="/logo.png" alt="PaperPilot" class="brand-img" />
        <span class="brand-text">PaperPilot <span class="web-badge">Web</span></span>
      </a>
    </div>

    <!-- Center: Main Navigation Dropdowns & Links -->
    <div class="navbar-links-section">
      <!-- Tools Mega-Menu Trigger -->
      <div class="nav-item-dropdown">
        <button
          type="button"
          class="nav-link-btn"
          class:active={isMegaMenuOpen}
          onclick={toggleMegaMenu}
          aria-expanded={isMegaMenuOpen}
        >
          <span>Tools</span>
          <span class="caret" class:rotate={isMegaMenuOpen}>▾</span>
        </button>
      </div>

      <!-- Features Dropdown Trigger -->
      <div class="nav-item-dropdown">
        <button
          type="button"
          class="nav-link-btn"
          class:active={isFeaturesOpen}
          onclick={toggleFeatures}
          aria-expanded={isFeaturesOpen}
        >
          <span>Features</span>
          <span class="caret" class:rotate={isFeaturesOpen}>▾</span>
        </button>

        <FeaturesDropdown
          isOpen={isFeaturesOpen}
          onClose={() => isFeaturesOpen = false}
        />
      </div>

      <!-- Quick Finder Search Trigger -->
      <button
        type="button"
        class="quick-finder-trigger"
        onclick={() => isQuickFinderOpen = true}
        aria-label="Search all tools"
      >
        <span class="finder-icon">🔍</span>
        <span class="finder-label">Search 101 tools...</span>
        <kbd class="shortcut-pill">⌘K</kbd>
      </button>

      <!-- Desktop App Modal Trigger -->
      <button
        type="button"
        class="nav-text-btn"
        onclick={() => isDownloadModalOpen = true}
      >
        Desktop
      </button>

      <!-- Documentation Link -->
      <a
        href={docsUrl}
        target="_blank"
        rel="noopener noreferrer"
        class="nav-text-link"
      >
        Docs
      </a>

      <!-- GitHub Link with Star Badge -->
      <a
        href={GITHUB_REPO_URL}
        target="_blank"
        rel="noopener noreferrer"
        class="github-nav-link"
        aria-label="GitHub repository"
      >
        <svg class="gh-icon" viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
          <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/>
        </svg>
        <span class="star-badge">★ Star</span>
      </a>
    </div>

    <!-- Right: High-Contrast Download Desktop CTA -->
    <div class="navbar-actions-section">
      <button
        type="button"
        class="desktop-cta-btn"
        onclick={() => isDownloadModalOpen = true}
      >
        <span>Download Desktop</span>
        {#if ENABLE_DOWNLOAD_COUNT_BADGE && downloadCount !== null && downloadCount >= MIN_DOWNLOAD_COUNT_THRESHOLD}
          <span class="dl-count-pill">{formatCount(downloadCount)}</span>
        {/if}
      </button>
    </div>
  </div>

  <!-- Mega Menu Dropdown Mounted under Header -->
  <MegaMenu
    isOpen={isMegaMenuOpen}
    onClose={() => isMegaMenuOpen = false}
    onOpenDesktopModal={() => isDownloadModalOpen = true}
  />
</nav>

<!-- Modals -->
<DesktopDownloadModal
  isOpen={isDownloadModalOpen}
  onClose={() => isDownloadModalOpen = false}
/>

<ToolQuickFinderModal
  isOpen={isQuickFinderOpen}
  onClose={() => isQuickFinderOpen = false}
  onOpenDesktopModal={() => isDownloadModalOpen = true}
/>

<style>
  .paperpilot-navbar {
    position: sticky;
    top: 0;
    z-index: 1000;
    background: rgba(18, 19, 22, 0.92);
    backdrop-filter: blur(12px);
    border-bottom: 1px solid var(--border-color, #27272a);
    height: 72px;
    display: flex;
    align-items: center;
    width: 100%;
  }

  .navbar-container {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    max-width: 1680px;
    margin: 0 auto;
    padding: 0 24px;
    gap: 16px;
  }

  /* Left Brand */
  .navbar-brand-section {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .brand-logo {
    display: flex;
    align-items: center;
    gap: 10px;
    text-decoration: none;
    color: inherit;
  }

  .brand-img {
    width: 32px;
    height: 32px;
    object-fit: contain;
  }

  .brand-text {
    font-size: 1.25rem;
    font-weight: 800;
    letter-spacing: -0.02em;
    color: var(--text-primary, #ffffff);
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .web-badge {
    background: var(--accent-primary, #6366f1);
    color: white;
    font-size: 0.7rem;
    font-weight: 700;
    padding: 2px 7px;
    border-radius: 10px;
    text-transform: uppercase;
  }

  /* Center Navigation */
  .navbar-links-section {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  @media (max-width: 860px) {
    .navbar-links-section {
      display: none;
    }
  }

  .nav-item-dropdown {
    position: relative;
  }

  .nav-link-btn {
    display: flex;
    align-items: center;
    gap: 4px;
    background: transparent;
    border: none;
    color: var(--text-secondary, #d4d4d8);
    font-size: 0.92rem;
    font-weight: 600;
    padding: 8px 12px;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .nav-link-btn:hover,
  .nav-link-btn.active {
    color: var(--text-primary, #ffffff);
    background: rgba(255, 255, 255, 0.06);
  }

  .caret {
    font-size: 0.75rem;
    transition: transform 0.2s ease;
  }

  .caret.rotate {
    transform: rotate(180deg);
  }

  .quick-finder-trigger {
    display: flex;
    align-items: center;
    gap: 8px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--border-color, #27272a);
    padding: 6px 12px;
    border-radius: 10px;
    color: var(--text-secondary, #a1a1aa);
    font-size: 0.85rem;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .quick-finder-trigger:hover {
    background: rgba(255, 255, 255, 0.09);
    border-color: rgba(94, 106, 210, 0.4);
    color: var(--text-primary, #f4f4f5);
  }

  .finder-icon {
    font-size: 0.85rem;
  }

  .shortcut-pill {
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 4px;
    padding: 1px 5px;
    font-size: 0.7rem;
    font-family: inherit;
    color: var(--text-secondary, #d4d4d8);
  }

  .nav-text-btn,
  .nav-text-link {
    background: transparent;
    border: none;
    color: var(--text-secondary, #d4d4d8);
    font-size: 0.92rem;
    font-weight: 600;
    padding: 8px 12px;
    border-radius: 8px;
    cursor: pointer;
    text-decoration: none;
    transition: all 0.15s ease;
  }

  .nav-text-btn:hover,
  .nav-text-link:hover {
    color: var(--text-primary, #ffffff);
    background: rgba(255, 255, 255, 0.06);
  }

  .github-nav-link {
    display: flex;
    align-items: center;
    gap: 6px;
    color: var(--text-secondary, #d4d4d8);
    text-decoration: none;
    font-size: 0.92rem;
    font-weight: 600;
    padding: 8px 12px;
    border-radius: 8px;
    transition: all 0.15s ease;
  }

  .github-nav-link:hover {
    color: var(--text-primary, #ffffff);
    background: rgba(255, 255, 255, 0.06);
  }

  .star-badge {
    background: rgba(245, 158, 11, 0.15);
    color: #fbbf24;
    border: 1px solid rgba(245, 158, 11, 0.3);
    font-size: 0.7rem;
    padding: 2px 6px;
    border-radius: 12px;
    font-weight: 700;
  }

  /* Right Actions */
  .navbar-actions-section {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .feedback-action-btn {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--border-color, #27272a);
    color: var(--text-primary, #f4f4f5);
    padding: 8px 14px;
    border-radius: 10px;
    font-size: 0.88rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
    white-space: nowrap;
  }

  .feedback-action-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    border-color: var(--text-secondary, #a1a1aa);
  }

  .desktop-cta-btn {
    position: relative;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
    color: white;
    border: 1px solid rgba(255, 255, 255, 0.15);
    padding: 9px 18px;
    border-radius: 10px;
    font-size: 0.92rem;
    font-weight: 700;
    cursor: pointer;
    box-shadow: 0 4px 14px rgba(99, 102, 241, 0.35);
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    white-space: nowrap;
    overflow: hidden;
  }

  .dl-count-pill {
    background: rgba(255, 255, 255, 0.18);
    border: 1px solid rgba(255, 255, 255, 0.25);
    color: #ffffff;
    font-size: 0.72rem;
    font-weight: 700;
    padding: 2px 7px;
    border-radius: 9999px;
    line-height: 1.2;
  }

  .desktop-cta-btn:hover {
    transform: translateY(-1px);
    box-shadow: 0 6px 20px rgba(99, 102, 241, 0.5);
    background: linear-gradient(135deg, #4f46e5 0%, #4338ca 100%);
  }
</style>
