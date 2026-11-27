<script lang="ts">
  import MegaMenu from './MegaMenu.svelte';
  import FeaturesDropdown from './FeaturesDropdown.svelte';
  import DesktopDownloadModal from './DesktopDownloadModal.svelte';
  import ToolQuickFinderModal from './ToolQuickFinderModal.svelte';
  import DownloadBadge from './DownloadBadge.svelte';
  import ShareModal from './ShareModal.svelte';
  import {
    GITHUB_REPO_URL,
    DOWNLOAD_URL,
    getDocsBaseUrl
  } from '../constants';

  interface Props {
    onOpenFeedback: (category?: 'bug' | 'performance' | 'feature' | 'praise') => void;
  }

  let { onOpenFeedback }: Props = $props();

  let isMegaMenuOpen = $state(false);
  let isFeaturesOpen = $state(false);
  let isDownloadModalOpen = $state(false);
  let isQuickFinderOpen = $state(false);

  const docsUrl = getDocsBaseUrl();

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

      <!-- Trust Badge in Navbar -->
      <div class="navbar-trust-pill" title="All processing happens on your machine via WebAssembly">
        <span class="pulse-indicator"></span>
        <span class="trust-text">100% Client-Side WASM</span>
        <span class="trust-extra">— Zero Uploads</span>
      </div>
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
        Desktop App
      </button>

      <!-- Documentation Link -->
      <a
        href={docsUrl}
        target="_blank"
        rel="noopener noreferrer"
        class="nav-text-link"
      >
        Documentation
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
        <span>GitHub</span>
        <span class="star-badge">★ Star</span>
      </a>
    </div>

    <!-- Right: Feedback & High-Contrast Download Desktop CTA -->
    <div class="navbar-actions-section">
      <button
        type="button"
        class="feedback-action-btn"
        onclick={() => onOpenFeedback('bug')}
      >
        💬 Feedback & Bugs
      </button>

      <DownloadBadge />
      <ShareModal />

      <button
        type="button"
        class="desktop-cta-btn"
        onclick={() => isDownloadModalOpen = true}
      >
        <span>Download Desktop</span>
        <span class="cta-shine"></span>
      </button>
    </div>
  </div>

  <!-- Mega Menu Dropdown Mounted under Header -->
  <MegaMenu
    isOpen={isMegaMenuOpen}
    onClose={() => isMegaMenuOpen = false}
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

  .navbar-trust-pill {
    display: flex;
    align-items: center;
    gap: 6px;
    background: rgba(16, 185, 129, 0.08);
    border: 1px solid rgba(16, 185, 129, 0.25);
    padding: 4px 10px;
    border-radius: 20px;
    font-size: 0.78rem;
    color: #10b981;
    font-weight: 600;
  }

  @media (max-width: 1250px) {
    .trust-extra {
      display: none;
    }
  }

  @media (max-width: 1024px) {
    .navbar-trust-pill {
      display: none;
    }
  }

  .pulse-indicator {
    width: 7px;
    height: 7px;
    background-color: #10b981;
    border-radius: 50%;
    box-shadow: 0 0 8px #10b981;
    animation: pulseGlow 2s infinite;
  }

  @keyframes pulseGlow {
    0% { transform: scale(0.9); opacity: 0.7; }
    50% { transform: scale(1.2); opacity: 1; }
    100% { transform: scale(0.9); opacity: 0.7; }
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

  .desktop-cta-btn:hover {
    transform: translateY(-1px);
    box-shadow: 0 6px 20px rgba(99, 102, 241, 0.5);
    background: linear-gradient(135deg, #4f46e5 0%, #4338ca 100%);
  }

  .cta-shine {
    position: absolute;
    top: 0;
    left: -100%;
    width: 50%;
    height: 100%;
    background: linear-gradient(
      90deg,
      transparent,
      rgba(255, 255, 255, 0.25),
      transparent
    );
    transform: skewX(-20deg);
    animation: shine 4s infinite;
  }

  @keyframes shine {
    0% { left: -100%; }
    20% { left: 200%; }
    100% { left: 200%; }
  }
</style>
