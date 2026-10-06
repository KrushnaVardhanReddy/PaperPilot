<script lang="ts">
  import HeroPlayground from './components/HeroPlayground.svelte';
  import TriSurfaceShowcase from './components/TriSurfaceShowcase.svelte';
  import EmbedGenerator from './components/EmbedGenerator.svelte';
  import ApiExplorer from './components/ApiExplorer.svelte';
  import BenchmarkMatrix from './components/BenchmarkMatrix.svelte';
</script>

<div class="app-container">
  <header class="app-header">
    <div class="header-content">
      <div class="logo">
        <img src="/logo.png" alt="PaperPilot Logo" class="icon" />
        <h1>PaperPilot <span class="badge">Web</span></h1>
      </div>
      <div class="cta-banner">
        <span>Need OCR, Semantic Search, or Offline AI?</span>
        <a href="https://paperpilot.app/download" target="_blank" rel="noopener noreferrer" class="download-btn">
          Download PaperPilot Desktop (Free & Open Source)
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
    <div class="trust-badge">
      🔒 <strong>100% Client-Side Processing</strong> — Your documents never leave your device.
    </div>
    <div class="footer-links">
      <a href="https://paperpilot.app/download/linux" target="_blank" rel="noopener noreferrer">Linux (.deb / AppImage)</a>
      <span class="separator">•</span>
      <a href="https://paperpilot.app/download/windows" target="_blank" rel="noopener noreferrer">Windows (.exe / .msi)</a>
      <span class="separator">•</span>
      <a href="https://paperpilot.app/download/macos" target="_blank" rel="noopener noreferrer">macOS (.dmg)</a>
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
    max-width: 1200px;
    margin: 0 auto;
    width: 100%;
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
    padding: 24px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
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
