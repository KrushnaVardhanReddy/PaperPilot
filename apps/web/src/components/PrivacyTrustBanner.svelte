<script lang="ts">
  interface Props {
    compact?: boolean;
  }

  let { compact = false }: Props = $props();
</script>

<div class="privacy-trust-banner" class:compact>
  <div class="trust-badge-pill">
    <span class="status-indicator"></span>
    <span class="trust-bold">100% Client-Side WASM</span>
    <span class="divider">•</span>
    <span class="trust-subtitle">Zero Cloud Uploads</span>
  </div>
</div>

<style>
  .privacy-trust-banner {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    margin: 0 auto 28px auto;
    max-width: 720px;
    text-align: center;
  }

  .privacy-trust-banner.compact {
    margin-bottom: 16px;
  }

  .trust-badge-pill {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    background: rgba(16, 185, 129, 0.1);
    border: 1px solid rgba(16, 185, 129, 0.3);
    padding: 6px 16px;
    border-radius: 24px;
    box-shadow: 0 4px 12px rgba(16, 185, 129, 0.15);
  }

  .status-indicator {
    width: 8px;
    height: 8px;
    background-color: #10b981;
    border-radius: 50%;
    box-shadow: 0 0 10px #10b981;
    animation: pulseGlow 2s infinite;
  }

  @keyframes pulseGlow {
    0% { transform: scale(0.9); opacity: 0.7; }
    50% { transform: scale(1.2); opacity: 1; }
    100% { transform: scale(0.9); opacity: 0.7; }
  }

  .trust-bold {
    font-size: 0.9rem;
    font-weight: 700;
    color: #10b981;
    letter-spacing: -0.01em;
  }

  .divider {
    color: rgba(16, 185, 129, 0.4);
    font-size: 0.8rem;
  }

  .trust-subtitle {
    font-size: 0.88rem;
    font-weight: 600;
    color: #34d399;
  }
</style>
