<script lang="ts">
  interface FaqItem {
    question: string;
    answer: string;
    code?: string;
    suffix?: string;
  }

  const faqs: FaqItem[] = [
    {
      question: 'Are my files ever uploaded to a server?',
      answer:
        "No — 100% of Web WASM tools execute locally in your browser's private memory with 0 bytes uploaded."
    },
    {
      question: 'What is the difference between Web WASM and Desktop?',
      answer:
        'Web WASM runs 50+ instant tools directly in your browser. PaperPilot Desktop & CLI unlocks all 101+ tools, native multi-threaded speed, offline AI/OCR models, folder watchers, and MCP server integration.'
    },
    {
      question: 'Is PaperPilot really free & open source?',
      answer:
        'Yes — free and open source under MIT/Apache-2.0 with no watermarks, no file limits, and no account required.'
    },
    {
      question: 'Why do Windows SmartScreen or macOS Gatekeeper show a warning on first launch?',
      answer:
        'Open-source builds use ad-hoc signing rather than paid EV certificates. On Windows, click "More info" → "Run anyway" or use the portable .zip. On macOS, run ',
      code: 'sudo xattr -rd com.apple.quarantine /Applications/PaperPilot.app',
      suffix: ' or allow via System Settings → Privacy & Security.'
    }
  ];

  let openIndex = $state<number | null>(0);

  function toggle(index: number) {
    openIndex = openIndex === index ? null : index;
  }
</script>

<section class="faq-section" aria-labelledby="faq-heading">
  <div class="faq-header">
    <span class="faq-eyebrow">FAQ</span>
    <h2 id="faq-heading">Frequently Asked Questions</h2>
    <p>Everything you need to know about privacy, local execution, and desktop builds.</p>
  </div>

  <div class="faq-list">
    {#each faqs as item, index}
      {@const isOpen = openIndex === index}
      <div class="faq-item" class:open={isOpen}>
        <button
          type="button"
          class="faq-trigger"
          aria-expanded={isOpen}
          aria-controls="faq-panel-{index}"
          onclick={() => toggle(index)}
        >
          <span class="faq-question">{item.question}</span>
          <span class="faq-icon" aria-hidden="true">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="6 9 12 15 18 9"></polyline>
            </svg>
          </span>
        </button>

        {#if isOpen}
          <div id="faq-panel-{index}" class="faq-panel" role="region">
            <p>
              {item.answer}{#if item.code}<code>{item.code}</code>{/if}{#if item.suffix}{item.suffix}{/if}
            </p>
          </div>
        {/if}
      </div>
    {/each}
  </div>
</section>

<style>
  .faq-section {
    max-width: 880px;
    margin: 4rem auto 2rem;
    padding: 0 1.25rem;
  }

  .faq-header {
    text-align: center;
    margin-bottom: 2rem;
  }

  .faq-eyebrow {
    display: inline-block;
    font-size: 0.75rem;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--accent-primary, #6366f1);
    margin-bottom: 0.5rem;
  }

  .faq-header h2 {
    margin: 0 0 0.5rem;
    font-size: clamp(1.5rem, 2.5vw, 2rem);
    font-weight: 700;
    color: var(--text-primary, #f8fafc);
  }

  .faq-header p {
    margin: 0;
    color: var(--text-secondary, #94a3b8);
    font-size: 0.95rem;
  }

  .faq-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .faq-item {
    background: var(--bg-surface, #111827);
    border: 1px solid var(--border-color, #1f2937);
    border-radius: 12px;
    overflow: hidden;
    transition: border-color 0.2s ease, box-shadow 0.2s ease;
  }

  .faq-item.open {
    border-color: var(--accent-primary, #6366f1);
    box-shadow: 0 8px 24px -12px rgba(99, 102, 241, 0.25);
  }

  .faq-trigger {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 1.125rem 1.25rem;
    background: transparent;
    border: none;
    color: var(--text-primary, #f8fafc);
    font-size: 1rem;
    font-weight: 600;
    text-align: left;
    cursor: pointer;
    transition: color 0.15s ease;
  }

  .faq-trigger:hover {
    color: var(--accent-primary, #6366f1);
  }

  .faq-trigger:focus-visible {
    outline: 2px solid var(--accent-primary, #6366f1);
    outline-offset: -2px;
  }

  .faq-icon {
    flex-shrink: 0;
    width: 20px;
    height: 20px;
    color: var(--text-secondary, #94a3b8);
    transition: transform 0.2s ease, color 0.2s ease;
  }

  .faq-item.open .faq-icon {
    transform: rotate(180deg);
    color: var(--accent-primary, #6366f1);
  }

  .faq-icon svg {
    width: 100%;
    height: 100%;
    display: block;
  }

  .faq-panel {
    padding: 0 1.25rem 1.125rem;
    border-top: 1px solid var(--border-color, #1f2937);
  }

  .faq-panel p {
    margin: 0.875rem 0 0;
    color: var(--text-secondary, #94a3b8);
    font-size: 0.9375rem;
    line-height: 1.65;
  }

  .faq-panel code {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 0.85em;
    padding: 0.15em 0.4em;
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid var(--border-color, #1f2937);
    color: var(--text-primary, #f8fafc);
    word-break: break-word;
  }
</style>
