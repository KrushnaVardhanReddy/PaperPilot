<script lang="ts">
  import OperationsView from "../views/OperationsView.svelte";
  import QuickToolLaunchRow from "./QuickToolLaunchRow.svelte";
  import PrivacyTrustBanner from "./PrivacyTrustBanner.svelte";
  import { DOWNLOAD_URL } from "../constants";

  interface Props {
    onOpenAllTools?: () => void;
  }

  let { onOpenAllTools }: Props = $props();
  let isFullWidth = $state(false);
</script>

<div id="hero-section" class="hero-section">
  <div class="hero-content">
    <PrivacyTrustBanner />

    <h1 class="hero-title">The Pure-Rust, Zero-Cloud PDF Powerhouse.</h1>
    <p class="hero-tagline">
      Zero-cloud PDF toolbox executed directly in your browser's private memory. Instant WASM performance, no servers, zero tracking.
    </p>

    <!-- Quick Tool Launch Row with Popular Instant Tools -->
    <QuickToolLaunchRow {onOpenAllTools} />
  </div>

  <div id="wasm-playground" class="hero-playground" class:full-width={isFullWidth}>
    <div class="playground-wrapper">
      <div class="playground-header">
        <div class="window-controls">
          <span class="dot close"></span>
          <span class="dot minimize"></span>
          <span class="dot maximize"></span>
        </div>
        <div class="window-title">PaperPilot WASM Engine</div>
        <button
          type="button"
          class="expand-toggle-btn"
          onclick={() => isFullWidth = !isFullWidth}
          title={isFullWidth ? 'Exit Full Width' : 'Expand Full Width'}
          aria-label={isFullWidth ? 'Exit Full Width' : 'Expand Full Width'}
        >
          {isFullWidth ? '⤡ Exit Full Width' : '⤢ Full Width'}
        </button>
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
    padding: 64px 32px 120px;
    background: radial-gradient(
      circle at top center,
      rgba(94, 106, 210, 0.15) 0%,
      transparent 70%
    );
    height: auto;
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

  .hero-playground {
    width: 100%;
    max-width: 1400px;
    margin: 0 auto 64px auto;
    perspective: 1000px;
    min-height: 700px;
    position: relative;
    z-index: 10;
    transition: max-width 0.25s ease;
  }

  .hero-playground.full-width {
    max-width: none;
  }

  .playground-wrapper {
    background-color: var(--surface-0);
    border-radius: var(--border-radius-xl);
    border: 1px solid var(--border-color);
    box-shadow:
      var(--shadow-lg),
      0 20px 40px rgba(0, 0, 0, 0.4);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    height: 700px;
    transform-origin: top center;
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

  .dot.close {
    background-color: #ff5f56;
  }
  .dot.minimize {
    background-color: #ffbd2e;
  }
  .dot.maximize {
    background-color: #27c93f;
  }

  .window-title {
    flex: 1;
    text-align: center;
    font-size: 0.85rem;
    color: var(--text-secondary);
    font-weight: 500;
  }

  .expand-toggle-btn {
    background: transparent;
    border: 1px solid var(--border-color);
    color: var(--text-secondary);
    font-size: 0.75rem;
    font-weight: 500;
    padding: 4px 10px;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.15s ease;
    white-space: nowrap;
  }

  .expand-toggle-btn:hover {
    color: var(--text-primary);
    border-color: var(--text-secondary);
    background-color: var(--surface-2, rgba(255, 255, 255, 0.05));
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
    .hero-section {
      padding: 40px 16px 80px;
    }

    .hero-title {
      font-size: 2.2rem;
      line-height: 1.15;
    }

    .hero-tagline {
      font-size: 1rem;
      margin-bottom: 24px;
    }

    .hero-playground {
      width: 100%;
      padding: 0;
    }

    .playground-wrapper {
      height: auto;
      min-height: 700px;
      border-radius: var(--border-radius-md, 8px);
    }

    :global(.playground-content .operations-view) {
      padding: 12px;
    }
  }

  @media (max-width: 480px) {
    .hero-title {
      font-size: 1.85rem;
    }

    .playground-wrapper {
      min-height: 640px;
    }
  }
</style>
