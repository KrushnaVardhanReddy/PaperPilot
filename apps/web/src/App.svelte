<script lang="ts">
  import { onMount } from 'svelte';
  import HeroPlayground from './components/HeroPlayground.svelte';
  import UseCasesShowcase from './components/UseCasesShowcase.svelte';
  import TriSurfaceShowcase from './components/TriSurfaceShowcase.svelte';
  import EmbedGenerator from './components/EmbedGenerator.svelte';
  import ApiExplorer from './components/ApiExplorer.svelte';
  import BenchmarkMatrix from './components/BenchmarkMatrix.svelte';
  import ShareModal from './components/ShareModal.svelte';
  import DownloadBadge from './components/DownloadBadge.svelte';
  import RoadmapModal from './components/RoadmapModal.svelte';
  import {
    DOWNLOAD_URL,
    DOWNLOAD_LINUX_URL,
    DOWNLOAD_WINDOWS_URL,
    DOWNLOAD_MACOS_URL,
    getDocsBaseUrl,
    getToolsHandbookUrl
  } from './constants';

  const docsBaseUrl = getDocsBaseUrl();
  const toolsHandbookUrl = getToolsHandbookUrl();
  const roadmapPath = `${import.meta.env.BASE_URL}roadmap`;
  let showRoadmap = $state(false);
  let roadmapReturnUrl = import.meta.env.BASE_URL;

  function isRoadmapLocation() {
    return window.location.pathname.replace(/\/$/, '') === roadmapPath;
  }

  function openRoadmap(event: MouseEvent) {
    if (event.button !== 0 || event.ctrlKey || event.metaKey || event.shiftKey || event.altKey) return;
    event.preventDefault();
    if (!showRoadmap) {
      roadmapReturnUrl = window.location.pathname + window.location.search + window.location.hash;
      window.history.pushState(null, '', roadmapPath);
      showRoadmap = true;
    }
  }

  function closeRoadmap() {
    showRoadmap = false;
    if (isRoadmapLocation()) window.history.replaceState(null, '', roadmapReturnUrl);
  }

  function syncRoadmapLocation() { showRoadmap = isRoadmapLocation(); }
  onMount(syncRoadmapLocation);
</script>

<svelte:window onpopstate={syncRoadmapLocation} />

{#if showRoadmap}<RoadmapModal onclose={closeRoadmap} />{/if}

<div class="app-container">
  <header class="app-header">
    <div class="header-content">
      <div class="logo">
        <img src="/logo.png" alt="PaperPilot Logo" class="icon" />
        <h1>PaperPilot <span class="badge">Web</span></h1>
      </div>

      <div class="header-actions">
        <DownloadBadge />
        <ShareModal />
        <a class="roadmap-trigger" href={roadmapPath} onclick={openRoadmap}>Roadmap <span aria-hidden="true">↗</span></a>
        <div class="cta-banner">
          <span>Need OCR or Offline AI?</span>
          <a href={DOWNLOAD_URL} target="_blank" rel="noopener noreferrer" class="download-btn">
            Download Desktop
          </a>
        </div>
      </div>
    </div>
  </header>

  <main class="main-content">
    <HeroPlayground />
    <UseCasesShowcase />
    <TriSurfaceShowcase />
    <EmbedGenerator />
    <ApiExplorer />
    <BenchmarkMatrix />
  </main>

  <footer class="app-footer">
    <div class="footer-inner">
      <div class="privacy-badge">
        <span class="icon">🔒</span> 100% Client-Side <span class="muted">— Documents never leave your device.</span>
      </div>
      <div class="footer-links">
        <a href={roadmapPath} onclick={openRoadmap}>Roadmap</a>
        <span class="separator">•</span>
        <a href="/embed-test" target="_blank" rel="noopener noreferrer">Embed Sandbox</a>
        <span class="separator">•</span>
        <a href={docsBaseUrl} target="_blank" rel="noopener noreferrer">Docs</a>
        <span class="separator">•</span>
        <a href={toolsHandbookUrl} target="_blank" rel="noopener noreferrer">50+ Tools Handbook</a>
        <span class="separator">•</span>
        <a href={DOWNLOAD_LINUX_URL} target="_blank" rel="noopener noreferrer">Linux</a>
        <span class="separator">•</span>
        <a href={DOWNLOAD_WINDOWS_URL} target="_blank" rel="noopener noreferrer">Windows</a>
        <span class="separator">•</span>
        <a href={DOWNLOAD_MACOS_URL} target="_blank" rel="noopener noreferrer">macOS</a>
      </div>
    </div>
  </footer>
</div>

<style>
  :global(body) {
    background-color: var(--bg-primary);
    color: var(--text-primary);
    font-family: var(--font-sans);
    margin: 0;
    padding: 0;
  }

  .app-container {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }

  .app-header {
    background-color: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    padding: 16px 32px;
  }

  .header-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
    max-width: 1600px;
    margin: 0 auto;
    width: 100%;
    gap: 24px;
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .logo .icon {
    width: 32px;
    height: 32px;
  }

  .logo h1 {
    font-size: 1.25rem;
    font-weight: 700;
    margin: 0;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .badge {
    background-color: var(--accent-primary);
    color: white;
    font-size: 0.75rem;
    padding: 2px 8px;
    border-radius: 12px;
    font-weight: 600;
    text-transform: uppercase;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .cta-banner {
    display: flex;
    align-items: center;
    gap: 16px;
    font-size: 0.9rem;
    color: var(--text-secondary);
  }

  .roadmap-trigger { color: var(--text-primary); border: 1px solid var(--border-color); background: var(--bg-surface); border-radius: 8px; padding: 8px 12px; font-size: .85rem; white-space: nowrap; }
  .roadmap-trigger:hover { border-color: var(--accent-primary); color: #c5ceff; }
  .roadmap-trigger:focus-visible { outline: 2px solid var(--accent-primary); outline-offset: 3px; }

  .download-btn {
    background-color: var(--accent-primary);
    color: white;
    padding: 8px 16px;
    border-radius: var(--border-radius-md);
    text-decoration: none;
    font-weight: 600;
    transition: background-color var(--transition-fast);
  }

  .download-btn:hover {
    background-color: var(--accent-hover);
  }

  .main-content {
    flex: 1;
    background-color: var(--bg-primary);
    overflow-y: auto;
  }

  .app-footer {
    background-color: var(--bg-secondary);
    border-top: 1px solid var(--border-color);
    padding: 8px 32px;
    height: 48px;
    box-sizing: border-box;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .footer-inner {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    max-width: 1400px;
  }

  .privacy-badge {
    background-color: rgba(16, 185, 129, 0.08);
    color: #10B981;
    padding: 4px 12px;
    border-radius: 12px;
    font-size: 0.8rem;
    border: 1px solid rgba(16, 185, 129, 0.2);
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .privacy-badge .muted {
    opacity: 0.8;
  }

  .footer-links {
    display: flex;
    gap: 12px;
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .footer-links a {
    color: var(--text-secondary);
  }

  .footer-links a:hover {
    color: var(--accent-primary);
  }

  .separator {
    color: var(--border-color);
  }

  @media (max-width: 768px) {
    .app-header { padding: 16px; }
    .header-actions { flex-wrap: wrap; justify-content: center; gap: 10px; max-width: 100%; }
    .roadmap-trigger { min-height: 44px; display: inline-flex; align-items: center; gap: 6px; }
    .app-footer { height: auto; min-height: 48px; padding: 12px 16px; }
    .footer-inner, .footer-links { flex-wrap: wrap; justify-content: center; gap: 10px; }
    .privacy-badge { flex-wrap: wrap; justify-content: center; }
    .header-content {
      flex-direction: column;
      gap: 16px;
      text-align: center;
    }

    .cta-banner {
      flex-direction: column;
      gap: 8px;
    }
  }
</style>
