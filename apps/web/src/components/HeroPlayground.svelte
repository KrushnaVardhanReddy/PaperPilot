<script lang="ts">
  import OperationsView from '../views/OperationsView.svelte';
</script>

<div class="hero-section">
  <div class="hero-content">
    <h1 class="hero-title">The Pure-Rust, Zero-Cloud PDF Powerhouse.</h1>
    <p class="hero-tagline">100% Client-Side. 0 Bytes Uploaded. Sub-10ms Execution.</p>

    <div class="hero-ctas">
      <a href="https://github.com/paperpilot/paperpilot/releases" target="_blank" rel="noopener noreferrer" class="cta-btn primary">
        <span class="icon">⚡</span> Download Desktop <span class="muted">(Free & Open Source)</span>
      </a>
      <button class="cta-btn secondary" onclick={() => document.getElementById('embed-generator')?.scrollIntoView({ behavior: 'smooth' })}>
        <span class="icon">&lt; / &gt;</span> Get Embed Code
      </button>
    </div>
  </div>

  <div class="hero-playground">
    <div class="playground-wrapper">
      <div class="playground-header">
        <div class="window-controls">
          <span class="dot close"></span>
          <span class="dot minimize"></span>
          <span class="dot maximize"></span>
        </div>
        <div class="window-title">PaperPilot WASM Engine</div>
      </div>
      <div class="playground-content">
        <OperationsView />
      </div>
    </div>
  </div>
</div>

<style>
  .hero-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 64px 32px 32px;
    background: radial-gradient(circle at top center, rgba(94, 106, 210, 0.15) 0%, transparent 70%);
    min-height: 80vh;
  }

  .hero-content {
    text-align: center;
    max-width: 800px;
    margin-bottom: 48px;
  }

  .hero-title {
    font-size: 3.5rem;
    font-weight: 800;
    line-height: 1.1;
    margin-bottom: 24px;
    background: linear-gradient(to right, #ffffff, #a1a7b3);
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  :global(.light) .hero-title {
    background: linear-gradient(to right, #111827, #4b5563);
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .hero-tagline {
    font-size: 1.25rem;
    color: var(--text-secondary);
    margin-bottom: 32px;
    font-weight: 500;
  }

  .hero-ctas {
    display: flex;
    gap: 16px;
    justify-content: center;
    flex-wrap: wrap;
  }

  .cta-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 24px;
    border-radius: var(--border-radius-lg);
    font-size: 1.1rem;
    font-weight: 600;
    cursor: pointer;
    transition: all var(--transition-fast);
    text-decoration: none;
  }

  .cta-btn.primary {
    background-color: var(--accent-primary);
    color: white;
    border: none;
    box-shadow: 0 4px 14px rgba(94, 106, 210, 0.4);
  }

  .cta-btn.primary:hover {
    background-color: var(--accent-hover);
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(94, 106, 210, 0.5);
  }

  .cta-btn.primary .muted {
    opacity: 0.8;
    font-size: 0.9em;
    font-weight: 400;
  }

  .cta-btn.secondary {
    background-color: transparent;
    color: var(--text-primary);
    border: 1px solid var(--border-color);
  }

  .cta-btn.secondary:hover {
    background-color: var(--bg-surface);
    border-color: var(--text-secondary);
  }

  .hero-playground {
    width: 100%;
    max-width: 1200px;
    margin: 0 auto;
    perspective: 1000px;
    min-height: 700px;
    position: relative;
    z-index: 10;
  }

  .playground-wrapper {
    background-color: var(--surface-0);
    border-radius: var(--border-radius-xl);
    border: 1px solid var(--border-color);
    box-shadow: var(--shadow-lg), 0 20px 40px rgba(0, 0, 0, 0.4);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    height: 700px;
    transform-origin: top center;
    animation: float 6s ease-in-out infinite;
  }

  @keyframes float {
    0% { transform: translateY(0px); }
    50% { transform: translateY(-10px); }
    100% { transform: translateY(0px); }
  }

  .playground-header {
    background-color: var(--surface-1);
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color);
    display: flex;
    align-items: center;
  }

  .window-controls {
    display: flex;
    gap: 8px;
  }

  .dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
  }

  .dot.close { background-color: #ff5f56; }
  .dot.minimize { background-color: #ffbd2e; }
  .dot.maximize { background-color: #27c93f; }

  .window-title {
    flex: 1;
    text-align: center;
    font-size: 0.85rem;
    color: var(--text-secondary);
    font-weight: 500;
    margin-right: 44px; /* Offset for window controls */
  }

  .playground-content {
    flex: 1;
    overflow: hidden;
    background-color: var(--bg-primary);
  }

  /* Override OperationsView styles to fit within the playground */
  :global(.playground-content .operations-view) {
    height: 100%;
    padding: 24px;
    box-sizing: border-box;
  }

  @media (max-width: 768px) {
    .hero-title {
      font-size: 2.5rem;
    }

    .playground-wrapper {
      height: 800px;
    }

    .hero-ctas {
      flex-direction: column;
    }

    .cta-btn {
      width: 100%;
      justify-content: center;
    }
  }
</style>
