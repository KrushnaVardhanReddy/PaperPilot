<script lang="ts">
  import Navbar from './components/Navbar.svelte';
  import HeroPlayground from './components/HeroPlayground.svelte';
  import DeveloperTabsSection from './components/DeveloperTabsSection.svelte';
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
  let showScrollTop = $state(false);

  function openFeedback(cat: 'bug' | 'performance' | 'feature' | 'praise' = 'bug') {
    feedbackCategory = cat;
    isFeedbackOpen = true;
  }

  function handleScroll() {
    showScrollTop = window.scrollY > 600;
  }

  function scrollToPlayground() {
    const el = document.getElementById('wasm-playground');
    if (el) {
      el.scrollIntoView({ behavior: 'smooth', block: 'start' });
    } else {
      window.scrollTo({ top: 0, behavior: 'smooth' });
    }
  }
</script>

<svelte:window onscroll={handleScroll} />

<div class="app-container">
  <Navbar onOpenFeedback={openFeedback} />

  <main class="main-content">
    <HeroPlayground />
    <section class="social-proof-strip" aria-label="Platform highlights">
      <div class="social-proof-inner">
        <span class="proof-item">🔒 100% Private WASM</span>
        <span class="proof-sep" aria-hidden="true">•</span>
        <span class="proof-item">⚡ 50+ Browser Tools</span>
        <span class="proof-sep" aria-hidden="true">•</span>
        <span class="proof-item">🌐 100% Open Source</span>
        <span class="proof-sep" aria-hidden="true">•</span>
        <span class="proof-item">🚫 Zero File Uploads</span>
      </div>
    </section>
    <DeveloperTabsSection />
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
        <span class="separator">•</span>
        <a href="https://github.com/KrushnaVardhanReddy/PaperPilot" target="_blank" rel="noopener noreferrer">GitHub</a>
        <span class="separator">•</span>
        <a href="https://github.com/KrushnaVardhanReddy/PaperPilot/releases" target="_blank" rel="noopener noreferrer">Changelog</a>
      </div>
    </div>
  </footer>
</div>

<FeedbackModal
  isOpen={isFeedbackOpen}
  initialCategory={feedbackCategory}
  onClose={() => isFeedbackOpen = false}
/>

{#if showScrollTop}
  <button
    type="button"
    class="scroll-playground-btn"
    onclick={scrollToPlayground}
    aria-label="Scroll back to playground"
  >
    ↑ Playground
  </button>
{/if}

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

  .social-proof-strip {
    border-top: 1px solid var(--border-color);
    border-bottom: 1px solid var(--border-color);
    background-color: var(--bg-secondary);
    padding: 12px 24px;
  }

  .social-proof-inner {
    max-width: 1200px;
    margin: 0 auto;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: center;
    gap: 12px 20px;
    font-size: 0.85rem;
    color: var(--text-secondary);
    letter-spacing: 0.01em;
  }

  .proof-item {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    color: var(--text-primary);
    font-weight: 500;
  }

  .proof-sep {
    color: var(--text-secondary);
    opacity: 0.5;
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

  .scroll-playground-btn {
    position: fixed;
    right: 24px;
    bottom: 64px;
    z-index: 50;
    background-color: var(--bg-surface);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
    border-radius: 999px;
    padding: 8px 14px;
    font-family: inherit;
    font-size: 0.82rem;
    font-weight: 500;
    cursor: pointer;
    box-shadow: 0 4px 14px rgba(0, 0, 0, 0.35);
    transition: border-color 0.15s ease, color 0.15s ease, transform 0.15s ease;
  }

  .scroll-playground-btn:hover {
    border-color: var(--accent-primary);
    color: var(--accent-primary);
    transform: translateY(-2px);
  }
</style>
