<script lang="ts">
  import { generateQrSvg } from '../lib/qr';

  let isOpen = $state(false);
  let copied = $state(false);
  let copyTimeout: ReturnType<typeof setTimeout>;

  const currentUrl = typeof window !== 'undefined' ? window.location.href : 'https://paperpilot.app/';
  const qrSvg = generateQrSvg(currentUrl, 180);

  const canShare = typeof navigator !== 'undefined' && !!navigator.share;

  function toggleModal() {
    isOpen = !isOpen;
    if (isOpen) {
      copied = false;
    }
  }

  function closeModal() {
    isOpen = false;
  }

  async function copyToClipboard() {
    try {
      await navigator.clipboard.writeText(currentUrl);
      copied = true;
      clearTimeout(copyTimeout);
      copyTimeout = setTimeout(() => {
        copied = false;
      }, 2500);
    } catch {
      copied = false;
    }
  }

  async function triggerNativeShare() {
    if (canShare) {
      try {
        await navigator.share({
          title: 'PaperPilot — 100% Private Client-Side PDF Tools',
          text: 'Process PDFs locally in your browser with zero file uploads and zero telemetry.',
          url: currentUrl
        });
      } catch {
        // User cancelled share
      }
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape' && isOpen) {
      closeModal();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<button class="share-trigger-btn" onclick={toggleModal} aria-label="Share or open on mobile">
  <svg class="share-icon" viewBox="0 0 24 24" width="16" height="16" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">
    <circle cx="18" cy="5" r="3"></circle>
    <circle cx="6" cy="12" r="3"></circle>
    <circle cx="18" cy="19" r="3"></circle>
    <line x1="8.59" y1="13.51" x2="15.42" y2="17.49"></line>
    <line x1="15.41" y1="6.51" x2="8.59" y2="10.49"></line>
  </svg>
  <span>Share</span>
</button>

{#if isOpen}
  <div class="modal-backdrop" onclick={closeModal} role="presentation">
    <div
      class="modal-card"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="dialog"
      tabindex="-1"
      aria-modal="true"
      aria-labelledby="share-modal-title"
    >
      <div class="modal-header">
        <div class="title-wrap">
          <h3 id="share-modal-title">Mobile Handoff & Share</h3>
          <p class="subtitle">Scan to open on phone or share with colleagues</p>
        </div>
        <button class="close-btn" onclick={closeModal} aria-label="Close modal">✕</button>
      </div>

      <div class="qr-preview-box">
        <div class="qr-inner">
          {@html qrSvg}
        </div>
        <span class="qr-caption">📱 Point camera to open immediately</span>
      </div>

      <div class="copy-url-group">
        <input type="text" readonly value={currentUrl} class="url-input" />
        <button class="copy-btn" class:copied onclick={copyToClipboard}>
          {copied ? '✓ Copied' : 'Copy'}
        </button>
      </div>

      {#if canShare}
        <button class="native-share-btn" onclick={triggerNativeShare}>
          <svg viewBox="0 0 24 24" width="16" height="16" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">
            <path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8"></path>
            <polyline points="16 6 12 2 8 6"></polyline>
            <line x1="12" y1="2" x2="12" y2="15"></line>
          </svg>
          Share via Apps...
        </button>
      {/if}

      <div class="privacy-note">
        <span class="lock-icon">🔒</span>
        <span>100% Private: Documents never leave your browser or device.</span>
      </div>
    </div>
  </div>
{/if}

<style>
  .share-trigger-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.45rem;
    padding: 0.45rem 0.85rem;
    font-size: 0.85rem;
    font-weight: 600;
    color: #e2e8f0;
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .share-trigger-btn:hover {
    background: rgba(255, 255, 255, 0.15);
    color: #fff;
    border-color: rgba(99, 102, 241, 0.5);
  }

  .modal-backdrop {
    position: fixed;
    inset: 0;
    z-index: 9999;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.65);
    backdrop-filter: blur(8px);
    padding: 1rem;
    animation: fadeIn 0.15s ease-out;
  }

  .modal-card {
    width: 100%;
    max-width: 380px;
    background: #181825;
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 16px;
    padding: 1.5rem;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.5), 0 0 0 1px rgba(99, 102, 241, 0.2);
    display: flex;
    flex-col: column;
    flex-direction: column;
    gap: 1.15rem;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .title-wrap h3 {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 700;
    color: #f8fafc;
  }

  .subtitle {
    margin: 0.2rem 0 0;
    font-size: 0.8rem;
    color: #94a3b8;
  }

  .close-btn {
    background: transparent;
    border: none;
    color: #94a3b8;
    font-size: 1.1rem;
    cursor: pointer;
    padding: 0.2rem;
    line-height: 1;
    border-radius: 4px;
  }

  .close-btn:hover {
    color: #fff;
    background: rgba(255, 255, 255, 0.1);
  }

  .qr-preview-box {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.6rem;
    padding: 1rem;
    background: rgba(255, 255, 255, 0.03);
    border-radius: 12px;
    border: 1px solid rgba(255, 255, 255, 0.06);
  }

  .qr-inner {
    padding: 0.5rem;
    background: #fff;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.25);
  }

  .qr-caption {
    font-size: 0.75rem;
    color: #cbd5e1;
    font-weight: 500;
  }

  .copy-url-group {
    display: flex;
    gap: 0.5rem;
  }

  .url-input {
    flex: 1;
    background: #0f0f17;
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: #cbd5e1;
    font-size: 0.8rem;
    padding: 0.5rem 0.75rem;
    border-radius: 8px;
    outline: none;
    text-overflow: ellipsis;
  }

  .copy-btn {
    padding: 0.5rem 1rem;
    font-size: 0.8rem;
    font-weight: 600;
    background: #4f46e5;
    color: #fff;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s;
    min-width: 70px;
  }

  .copy-btn:hover {
    background: #4338ca;
  }

  .copy-btn.copied {
    background: #10b981;
  }

  .native-share-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.55rem 1rem;
    font-size: 0.85rem;
    font-weight: 600;
    background: rgba(255, 255, 255, 0.07);
    color: #f1f5f9;
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .native-share-btn:hover {
    background: rgba(255, 255, 255, 0.12);
  }

  .privacy-note {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    font-size: 0.72rem;
    color: #64748b;
    justify-content: center;
    text-align: center;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: scale(0.97);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }
</style>
