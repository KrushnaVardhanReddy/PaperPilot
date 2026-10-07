<script lang="ts">
  import HeroPlayground from './components/HeroPlayground.svelte';
  import TriSurfaceShowcase from './components/TriSurfaceShowcase.svelte';
  import EmbedGenerator from './components/EmbedGenerator.svelte';
  import ApiExplorer from './components/ApiExplorer.svelte';
  import BenchmarkMatrix from './components/BenchmarkMatrix.svelte';
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
</script>

<div class="app-container">
  <header class="app-header">
    <div class="header-content">
      <div class="logo">
        <img src="/logo.png" alt="PaperPilot Logo" class="icon" />
        <h1>PaperPilot <span class="badge">Web</span></h1>
      </div>

      <div class="cta-banner">
        <span>Need OCR or Offline AI?</span>
        <a href={DOWNLOAD_URL} target="_blank" rel="noopener noreferrer" class="download-btn">
          Download Desktop
        </a>
      </div>
    </div>
  </header>

  <main class="main-content">
    <HeroPlayground />
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
        <a href="/embed-test" target="_blank" rel="noopener noreferrer">Embed Sandbox</a>
        <span class="separator">•</span>
        <a href={docsBaseUrl} target="_blank" rel="noopener noreferrer">Docs</a>
        <span class="separator">•</span>
        <a href={toolsHandbookUrl} target="_blank" rel="noopener noreferrer">44 Tools Handbook</a>
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

  .cta-banner {
    display: flex;
    align-items: center;
    gap: 16px;
    font-size: 0.9rem;
    color: var(--text-secondary);
  }

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

  .trust-badge {
    background-color: rgba(16, 185, 129, 0.1);
    color: #10B981;
    padding: 8px 16px;
    border-radius: var(--border-radius-lg);
    font-size: 0.95rem;
    display: flex;
    align-items: center;
    gap: 8px;
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
