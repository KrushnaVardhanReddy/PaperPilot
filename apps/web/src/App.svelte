<script lang="ts">
  import Navbar from './components/Navbar.svelte';
  import HeroPlayground from './components/HeroPlayground.svelte';
  import UseCasesShowcase from './components/UseCasesShowcase.svelte';
  import TriSurfaceShowcase from './components/TriSurfaceShowcase.svelte';
  import EmbedGenerator from './components/EmbedGenerator.svelte';
  import ApiExplorer from './components/ApiExplorer.svelte';
  import BenchmarkMatrix from './components/BenchmarkMatrix.svelte';
  import FaqSection from './components/FaqSection.svelte';
  import FeedbackModal from './components/FeedbackModal.svelte';
  import {
    DOWNLOAD_URL,
    getDocsBaseUrl,
    getToolsHandbookUrl
  } from './constants';

  const docsBaseUrl = getDocsBaseUrl();
  const toolsHandbookUrl = getToolsHandbookUrl();

  let isFeedbackOpen = $state(false);
  let feedbackCategory = $state<'bug' | 'performance' | 'feature' | 'praise'>('bug');

  function openFeedback(cat: 'bug' | 'performance' | 'feature' | 'praise' = 'bug') {
    feedbackCategory = cat;
    isFeedbackOpen = true;
  }
</script>

<div class="app-container">
  <Navbar onOpenFeedback={openFeedback} />

  <main class="main-content">
    <HeroPlayground />
    <UseCasesShowcase />
    <TriSurfaceShowcase />
    <EmbedGenerator />
    <ApiExplorer />
    <BenchmarkMatrix />
    <FaqSection />
  </main>

  <footer class="app-footer">
    <div class="footer-inner">
      <div class="privacy-badge">
        <span class="icon">🔒</span> 100% Client-Side <span class="muted">— Documents never leave your device.</span>
      </div>
      <div class="footer-links">
        <button type="button" class="footer-link-btn" onclick={() => openFeedback('bug')}>
          💬 Feedback & Bugs
        </button>
        <span class="separator">•</span>
        <a href="/embed-test" target="_blank" rel="noopener noreferrer">Embed Sandbox</a>
        <span class="separator">•</span>
        <a href={docsBaseUrl} target="_blank" rel="noopener noreferrer">Docs</a>
        <span class="separator">•</span>
        <a href={toolsHandbookUrl} target="_blank" rel="noopener noreferrer">Tools Handbook</a>
        <span class="separator">•</span>
        <a href={DOWNLOAD_URL} target="_blank" rel="noopener noreferrer">Releases</a>
      </div>
    </div>
  </footer>
</div>

<FeedbackModal
  isOpen={isFeedbackOpen}
  initialCategory={feedbackCategory}
  onClose={() => isFeedbackOpen = false}
/>

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

  .footer-link-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    font-size: 0.85rem;
    cursor: pointer;
    padding: 0;
    transition: color 0.15s;
    font-family: inherit;
  }

  .footer-link-btn:hover {
    color: var(--accent-primary);
  }

  .separator {
    color: var(--border-color);
  }
</style>
