<script lang="ts">
  import { DOWNLOAD_URL } from '../constants';

  interface UseCase {
    id: string;
    icon: string;
    persona: string;
    personaColor: string;
    headline: string;
    problem: string;
    solution: string;
    toolBadge: string;
    actionType: 'tool' | 'scroll' | 'external';
    actionTarget: string;
    actionLabel: string;
  }

  const useCases: UseCase[] = [
    {
      id: 'legal',
      icon: '🏛️',
      persona: 'Legal & Compliance',
      personaColor: '#818cf8',
      headline: 'Court Filings & Redaction',
      problem: 'Risk of leaking confidential client details, SSNs, and unverified digital signatures in public docket filings.',
      solution: 'Sanitize hidden metadata, redact sensitive PII, and cryptographically validate electronic signatures offline.',
      toolBadge: 'validate_signatures + pii_scan',
      actionType: 'tool',
      actionTarget: 'pii_scan',
      actionLabel: 'Launch PII Scanner'
    },
    {
      id: 'healthcare',
      icon: '🏥',
      persona: 'Healthcare & HIPAA',
      personaColor: '#34d399',
      headline: 'Protected Health Information',
      problem: 'Sending patient records to third-party cloud converters triggers HIPAA violation fines and security reviews.',
      solution: '100% in-browser WASM processing ensures medical charts, labs, and MRIs never leave the workstation.',
      toolBadge: 'sanitize + encrypt',
      actionType: 'tool',
      actionTarget: 'sanitize',
      actionLabel: 'Sanitize Records'
    },
    {
      id: 'finance',
      icon: '📊',
      persona: 'Finance & Operations',
      personaColor: '#fbbf24',
      headline: 'Audit Reports & Statements',
      problem: 'Scattered financial statements, unnumbered audit exhibits, and password-protected tax filings stall reconciliations.',
      solution: 'Merge multi-quarter ledgers, add automated legal bates/page numbering, and inspect PDF structural hashes.',
      toolBadge: 'merge + page_numbers',
      actionType: 'tool',
      actionTarget: 'merge',
      actionLabel: 'Merge Statements'
    },
    {
      id: 'academic',
      icon: '🎓',
      persona: 'Academic & Research',
      personaColor: '#60a5fa',
      headline: 'Scanned Papers & Theses',
      problem: 'Large historical book scans and paper preprints are bloated, unsearchable, and misaligned.',
      solution: 'High-speed OCR recognition, page orientation correction, and lossless compression for manuscript submission.',
      toolBadge: 'ocr + compress',
      actionType: 'tool',
      actionTarget: 'compress',
      actionLabel: 'Compress & Prepare'
    },
    {
      id: 'print',
      icon: '🖨️',
      persona: 'Print & Prepress',
      personaColor: '#f472b6',
      headline: 'Imposition & Handouts',
      problem: 'Manually laying out slides 4-up for course booklets or print-shop brochures wastes hours and paper.',
      solution: 'Instant N-Up multi-page tiling, booklet saddle-stitch page ordering, and dynamic page dimension scaling.',
      toolBadge: 'pdf_nup + pdf_booklet',
      actionType: 'tool',
      actionTarget: 'pdf_nup',
      actionLabel: 'Try N-Up Tiling'
    },
    {
      id: 'developer',
      icon: '👩‍💻',
      persona: 'Developers & AI Agents',
      personaColor: '#a78bfa',
      headline: 'MCP & Headless Pipelines',
      problem: 'Expensive SaaS API subscriptions with rate limits and document transmission privacy liabilities.',
      solution: 'Deploy via Model Context Protocol (MCP) in Claude/Cursor, drop-in 2-line HTML widget, or run pure CLI.',
      toolBadge: 'MCP + CLI + Web SDK',
      actionType: 'scroll',
      actionTarget: 'api-explorer',
      actionLabel: 'View Dev Snippets'
    }
  ];

  function handleAction(useCase: UseCase) {
    if (useCase.actionType === 'tool') {
      // 1. Dispatch event to switch tool in OperationsView
      window.dispatchEvent(
        new CustomEvent('select-tool', { detail: { toolId: useCase.actionTarget } })
      );
      // 2. Smoothly scroll into the playground
      const el = document.getElementById('wasm-playground') || document.getElementById('hero-section');
      el?.scrollIntoView({ behavior: 'smooth' });
    } else if (useCase.actionType === 'scroll') {
      const el = document.getElementById(useCase.actionTarget);
      el?.scrollIntoView({ behavior: 'smooth' });
    } else if (useCase.actionType === 'external') {
      window.open(useCase.actionTarget, '_blank', 'noopener,noreferrer');
    }
  }
</script>

<section id="use-cases" class="usecases-section">
  <div class="usecases-container">
    <div class="usecases-header">
      <div class="header-pill">Target Workflows</div>
      <h2 class="section-title">Built For Your Critical Workflows</h2>
      <p class="section-subtitle">
        Whether you are filing court documents, safeguarding HIPAA clinical records, or feeding documents to AI agents, PaperPilot delivers zero-leak privacy with native speed.
      </p>
    </div>

    <div class="cards-grid">
      {#each useCases as uc}
        <div class="scenario-card" style="--persona-color: {uc.personaColor};">
          <div class="card-top">
            <div class="icon-avatar">{uc.icon}</div>
            <div class="persona-tag" style="color: {uc.personaColor}; border-color: {uc.personaColor}33; background: {uc.personaColor}12;">
              {uc.persona}
            </div>
          </div>

          <h3 class="card-headline">{uc.headline}</h3>

          <div class="problem-box">
            <span class="problem-label">Challenge</span>
            <p class="problem-text">{uc.problem}</p>
          </div>

          <div class="solution-box">
            <span class="solution-label">PaperPilot Solution</span>
            <p class="solution-text">{uc.solution}</p>
          </div>

          <div class="card-footer">
            <div class="tool-tag" title="Target Tools">
              <span class="bolt">⚡</span> {uc.toolBadge}
            </div>
            <button
              class="cta-action-btn"
              onclick={() => handleAction(uc)}
              aria-label="Use Case Action: {uc.actionLabel}"
            >
              <span>{uc.actionLabel}</span>
              <span class="arrow">&rarr;</span>
            </button>
          </div>
        </div>
      {/each}
    </div>
  </div>
</section>

<style>
  .usecases-section {
    padding: 72px 24px 80px;
    background: linear-gradient(180deg, rgba(15, 17, 23, 0.4) 0%, rgba(20, 22, 31, 0.9) 100%);
    position: relative;
    border-top: 1px solid rgba(255, 255, 255, 0.05);
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }

  .usecases-container {
    max-width: 1240px;
    margin: 0 auto;
  }

  .usecases-header {
    text-align: center;
    margin-bottom: 56px;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .header-pill {
    display: inline-block;
    font-size: 0.75rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    padding: 4px 14px;
    border-radius: 999px;
    background: rgba(94, 106, 210, 0.15);
    color: var(--accent-primary, #6366f1);
    border: 1px solid rgba(94, 106, 210, 0.3);
    margin-bottom: 16px;
  }

  .section-title {
    font-size: 2.25rem;
    font-weight: 800;
    letter-spacing: -0.02em;
    color: var(--text-primary, #f3f4f6);
    margin: 0 0 16px;
  }

  .section-subtitle {
    font-size: 1.05rem;
    line-height: 1.6;
    color: var(--text-secondary, #9ca3af);
    max-width: 720px;
    margin: 0;
  }

  .cards-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
    gap: 24px;
  }

  @media (max-width: 768px) {
    .cards-grid {
      grid-template-columns: 1fr;
    }
  }

  .scenario-card {
    background: rgba(24, 26, 37, 0.7);
    backdrop-filter: blur(12px);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 16px;
    padding: 28px;
    display: flex;
    flex-direction: column;
    transition: transform 0.25s ease, border-color 0.25s ease, box-shadow 0.25s ease;
    position: relative;
    overflow: hidden;
  }

  .scenario-card::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 3px;
    background: var(--persona-color);
    opacity: 0.7;
    transition: opacity 0.25s ease;
  }

  .scenario-card:hover {
    transform: translateY(-5px);
    border-color: rgba(255, 255, 255, 0.2);
    box-shadow: 0 16px 36px -12px rgba(0, 0, 0, 0.5), 0 0 24px -6px var(--persona-color);
  }

  .scenario-card:hover::before {
    opacity: 1;
  }

  .card-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 20px;
  }

  .icon-avatar {
    font-size: 2rem;
    line-height: 1;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    width: 48px;
    height: 48px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .persona-tag {
    font-size: 0.75rem;
    font-weight: 600;
    padding: 4px 10px;
    border-radius: 999px;
    border: 1px solid;
    letter-spacing: 0.02em;
  }

  .card-headline {
    font-size: 1.25rem;
    font-weight: 700;
    color: var(--text-primary, #f9fafb);
    margin: 0 0 14px;
    line-height: 1.35;
  }

  .problem-box {
    background: rgba(239, 68, 68, 0.06);
    border-left: 3px solid rgba(239, 68, 68, 0.5);
    border-radius: 0 8px 8px 0;
    padding: 10px 14px;
    margin-bottom: 12px;
  }

  .problem-label {
    display: block;
    font-size: 0.68rem;
    font-weight: 700;
    text-transform: uppercase;
    color: #f87171;
    letter-spacing: 0.06em;
    margin-bottom: 4px;
  }

  .problem-text {
    font-size: 0.85rem;
    color: #d1d5db;
    margin: 0;
    line-height: 1.45;
  }

  .solution-box {
    background: rgba(16, 185, 129, 0.06);
    border-left: 3px solid rgba(16, 185, 129, 0.5);
    border-radius: 0 8px 8px 0;
    padding: 10px 14px;
    margin-bottom: 24px;
    flex-grow: 1;
  }

  .solution-label {
    display: block;
    font-size: 0.68rem;
    font-weight: 700;
    text-transform: uppercase;
    color: #34d399;
    letter-spacing: 0.06em;
    margin-bottom: 4px;
  }

  .solution-text {
    font-size: 0.85rem;
    color: #e5e7eb;
    margin: 0;
    line-height: 1.45;
  }

  .card-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding-top: 16px;
    border-top: 1px solid rgba(255, 255, 255, 0.06);
  }

  .tool-tag {
    font-size: 0.72rem;
    color: var(--text-secondary, #9ca3af);
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    background: rgba(0, 0, 0, 0.3);
    padding: 4px 8px;
    border-radius: 6px;
    border: 1px solid rgba(255, 255, 255, 0.06);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 170px;
  }

  .tool-tag .bolt {
    color: #fbbf24;
  }

  .cta-action-btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 8px 14px;
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.06);
    color: var(--text-primary, #ffffff);
    border: 1px solid rgba(255, 255, 255, 0.12);
    font-size: 0.82rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
    white-space: nowrap;
  }

  .cta-action-btn:hover {
    background: var(--persona-color);
    color: #0f1117;
    border-color: var(--persona-color);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }

  .cta-action-btn .arrow {
    transition: transform 0.2s ease;
  }

  .cta-action-btn:hover .arrow {
    transform: translateX(3px);
  }
</style>
