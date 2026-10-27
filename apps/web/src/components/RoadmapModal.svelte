<script lang="ts">
  import { onMount } from 'svelte';
  import DownloadBadge from './DownloadBadge.svelte';
  import { GITHUB_REPO_URL } from '../constants';
  import {
    featureRequestUrl, isRoadmapDocument, matchesRoadmapItem,
    roadmapCategories, roadmapSourceUrl, type RoadmapDocument, type RoadmapStatus,
  } from '../lib/roadmap';

  let { onclose }: { onclose: () => void } = $props();
  let dialog: HTMLDialogElement;
  let searchInput: HTMLInputElement;
  let roadmap = $state<RoadmapDocument | null>(null);
  let loading = $state(true);
  let error = $state('');
  let query = $state('');
  let category = $state('all');
  let controller: AbortController;
  const columns: { status: RoadmapStatus; title: string; subtitle: string }[] = [
    { status: 'shipped', title: 'Shipped', subtitle: 'Completed milestones' },
    { status: 'in_progress', title: 'In active development', subtitle: 'Taking shape right now' },
    { status: 'planned', title: 'Planned / Up next', subtitle: 'Help shape what comes next' },
  ];
  const filtered = $derived(roadmap?.items.filter(item => matchesRoadmapItem(item, query, category)) ?? []);

  async function loadRoadmap() {
    loading = true;
    error = '';
    try {
      const response = await fetch(`${import.meta.env.BASE_URL}roadmap.json`, { signal: controller.signal });
      if (!response.ok) throw new Error('Roadmap unavailable');
      const value: unknown = await response.json();
      if (!isRoadmapDocument(value)) throw new Error('Invalid roadmap');
      roadmap = value;
    } catch {
      if (!controller.signal.aborted) error = 'The roadmap could not be loaded. Please try again.';
    } finally {
      if (!controller.signal.aborted) loading = false;
    }
  }

  onMount(() => {
    controller = new AbortController();
    dialog.showModal();
    searchInput.focus();
    void loadRoadmap();
    return () => controller.abort();
  });

  function dismissBackdrop(event: MouseEvent) {
    if (event.target !== dialog) return;
    const box = dialog.getBoundingClientRect();
    if (event.clientX < box.left || event.clientX > box.right || event.clientY < box.top || event.clientY > box.bottom) dialog.close();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      event.preventDefault();
      dialog.close();
    } else if (event.key === 'Tab') {
      const focusable = [...dialog.querySelectorAll<HTMLElement>(
        'a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])'
      )].filter(element => element.tabIndex >= 0 && element.getClientRects().length > 0);
      const first = focusable[0];
      const last = focusable[focusable.length - 1];
      if (event.shiftKey && document.activeElement === first) {
        event.preventDefault();
        last?.focus();
      } else if (!event.shiftKey && document.activeElement === last) {
        event.preventDefault();
        first?.focus();
      }
    }
  }

  function itemsForColumn(status: RoadmapStatus) {
    const items = filtered.filter(item => item.status === status);
    if (status === 'shipped') {
      const prNumber = (url?: string) => Number(url?.split('/').pop() ?? 0);
      items.sort((left, right) => prNumber(right.pr_url) - prNumber(left.pr_url));
    }
    return items;
  }
</script>

<dialog bind:this={dialog} class="roadmap-modal" aria-labelledby="roadmap-title" onclose={onclose} onclick={dismissBackdrop} onkeydown={handleKeydown}>
  <div class="roadmap-shell">
    <header class="roadmap-header">
      <div class="heading-copy">
        <span class="eyebrow"><span class="live-dot"></span> OPEN DEVELOPMENT</span>
        <h2 id="roadmap-title">PaperPilot Live Roadmap</h2>
        <p>See what we’ve built. Help decide what comes next.</p>
      </div>
      <button class="close-button" aria-label="Close roadmap" onclick={() => dialog.close()}>✕</button>
    </header>

    <div class="roadmap-metrics" aria-label="Project highlights">
      {#if roadmap?.tool_count}<span class="metric"><strong>{roadmap.tool_count}+</strong> verified tools</span>{/if}
      <span class="metric privacy"><span aria-hidden="true">◈</span> Zero Cloud / Air-Gapped</span>
      <DownloadBadge forceShow minThreshold={0} />
    </div>

    <div class="roadmap-controls">
      <label class="search-box">
        <svg aria-hidden="true" viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="1.6"><circle cx="10.5" cy="10.5" r="6.5"/><path d="m16 16 4 4"/></svg>
        <input bind:this={searchInput} bind:value={query} type="search" aria-label="Search roadmap" placeholder="Search milestones, tools, or task IDs…" />
        <span class="search-hint" aria-hidden="true">SEARCH</span>
      </label>
      <div class="category-filters" aria-label="Roadmap categories" role="group">
        {#each roadmapCategories as filter}
          <button class:active={category === filter.id} aria-pressed={category === filter.id} onclick={() => category = filter.id}>{filter.label}</button>
        {/each}
      </div>
    </div>

    <div class="roadmap-note">
      <span class="freeze-indicator" aria-hidden="true">✦</span>
      <p>Feature freeze: community requests guide our next priorities.</p>
      {#if roadmap && !loading}<span class="result-count" role="status">{filtered.length} of {roadmap.items.length} milestones</span>{/if}
    </div>

    {#if loading}
      <div class="roadmap-message" role="status"><span class="loading-ring" aria-hidden="true"></span><p>Loading roadmap…</p></div>
    {:else if error}
      <div class="roadmap-message" role="alert"><h3>Roadmap unavailable</h3><p>{error}</p><button class="retry-button" onclick={loadRoadmap}>Try again</button></div>
    {:else}
      <div class="roadmap-board" aria-label="Roadmap milestones">
        {#each columns as column}
          {@const items = itemsForColumn(column.status)}
          <section class="roadmap-column" data-status={column.status} aria-labelledby={`roadmap-${column.status}`}>
            <div class="column-heading">
              <span class="status-dot" aria-hidden="true"></span>
              <div><h3 id={`roadmap-${column.status}`}>{column.title}</h3><p>{column.subtitle}</p></div>
              <span class="column-count">{items.length}</span>
            </div>
            <div class="column-scroll" role="region" aria-label={`${column.title} milestones`}>
              {#each items as item (item.key)}
                <article class="milestone-card" data-task-id={item.id}>
                  <div class="card-meta"><span class="task-id">{item.id}</span><span>PHASE {String(item.phase).padStart(2, '0')}</span></div>
                  <h4>{item.title}</h4>
                  {#if item.summary}<p class="card-summary">{item.summary}</p>{/if}
                  <div class="card-links">
                    <a href={roadmapSourceUrl(item)} target="_blank" rel="noopener noreferrer" aria-label={`View source for ${item.title}`}>View milestone <span aria-hidden="true">↗</span></a>
                    {#if item.pr_url}<a class="pr-tag" href={item.pr_url} target="_blank" rel="noopener noreferrer">PR #{item.pr_url.split('/').pop()}</a>{/if}
                    {#if item.spec_path}<a href={`${GITHUB_REPO_URL}/blob/feature/dev/${item.spec_path}`} target="_blank" rel="noopener noreferrer">Spec ↗</a>{/if}
                  </div>
                  {#if item.status === 'planned'}<a class="vote-link" href={featureRequestUrl(item)} target="_blank" rel="noopener noreferrer" aria-label={`Vote or request ${item.title} on GitHub`}><span aria-hidden="true">👍</span> Vote / Request on GitHub <span aria-hidden="true">↗</span></a>{/if}
                </article>
              {:else}
                <div class="empty-column"><span aria-hidden="true">—</span><p>No matching milestones</p>{#if query || category !== 'all'}<button onclick={() => { query = ''; category = 'all'; }}>Clear filters</button>{/if}</div>
              {/each}
            </div>
          </section>
        {/each}
      </div>
    {/if}

    <footer class="roadmap-footer"><span>Progress is open. Your documents stay private.</span><a href={`${GITHUB_REPO_URL}/issues`} target="_blank" rel="noopener noreferrer">Join the conversation ↗</a></footer>
  </div>
</dialog>

<style>
  .roadmap-modal { color: var(--text-primary); width: min(1392px, calc(100vw - 48px)); max-width: none; max-height: min(920px, calc(100dvh - 48px)); margin: auto; padding: 0; background: #11151eeF; border: 1px solid #46516d80; border-radius: 22px; box-shadow: 0 40px 120px #000a, 0 0 80px #5e6ad21a; overflow: hidden; }
  .roadmap-modal::backdrop { background: #070910bc; backdrop-filter: blur(12px); }
  .roadmap-shell { height: min(890px, calc(100dvh - 52px)); display: flex; flex-direction: column; background: radial-gradient(ellipse at 20% 0%, #5e6ad217, transparent 55%); }
  .roadmap-header { display: flex; justify-content: space-between; gap: 16px; padding: 30px 32px 18px; }
  .eyebrow { display: flex; align-items: center; gap: 8px; color: #a7b2d6; font-size: 10px; letter-spacing: .16em; font-weight: 700; margin-bottom: 9px; }
  .live-dot { width: 6px; height: 6px; background: #8fa1ff; border-radius: 50%; box-shadow: 0 0 12px #8fa1ff80; }
  h2 { font-size: clamp(23px, 2.4vw, 30px); letter-spacing: -.035em; line-height: 1.2; }
  .heading-copy p { color: var(--text-secondary); font-size: 13px; margin-top: 7px; }
  button, a, input { -webkit-tap-highlight-color: transparent; }
  button { font: inherit; cursor: pointer; }
  .close-button { flex-shrink: 0; align-self: flex-start; width: 40px; height: 40px; border-radius: 12px; border: 1px solid var(--border-color); background: #ffffff05; color: #b2bbcf; font-size: 16px; }
  .close-button:hover { color: #fff; background: #ffffff0e; }
  .roadmap-metrics { padding: 0 32px 22px; display: flex; gap: 10px; flex-wrap: wrap; align-items: center; }
  .metric { border: 1px solid #ffffff12; background: #ffffff04; border-radius: 99px; padding: 5px 10px; color: #acb5cb; font-size: 11px; white-space: nowrap; }
  .metric strong { color: #e0e5f7; margin-right: 3px; }
  .privacy { color: #86cbb3; border-color: #86cbb322; background: #86cbb305; }
  .roadmap-controls { display: flex; gap: 18px; padding: 18px 32px; border-top: 1px solid #ffffff09; border-bottom: 1px solid #ffffff09; align-items: center; }
  .search-box { min-width: 190px; flex: 1; display: flex; align-items: center; gap: 10px; padding: 0 12px; background: #080b1270; border: 1px solid #46516d70; border-radius: 9px; color: #77829e; height: 42px; }
  .search-box:focus-within { border-color: #8d9bf1; box-shadow: 0 0 0 2px #8d9bf114; }
  input { min-width: 0; width: 100%; border: 0; outline: 0; background: transparent; color: #e4e8f6; font: inherit; font-size: 12px; }
  input::placeholder { color: #8490ac; }
  .search-hint { font-size: 8px; letter-spacing: .1em; color: #818ba2; }
  .category-filters { display: flex; gap: 4px; flex-wrap: wrap; }
  .category-filters button { border: 1px solid transparent; border-radius: 7px; color: #a4aec4; background: transparent; padding: 8px 9px; font-size: 11px; white-space: nowrap; }
  .category-filters button:hover { background: #ffffff08; color: #e6eaff; }
  .category-filters .active { color: #c5ceff; background: #7c8af31c; border-color: #7c8af336; }
  .roadmap-note { display: flex; align-items: center; gap: 7px; padding: 13px 32px; font-size: 11px; color: #919bb4; }
  .freeze-indicator { color: #9aa8f2; }
  .result-count { margin-left: auto; font-size: 10px; color: #a9b3cb; white-space: nowrap; }
  .roadmap-board { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 16px; padding: 0 24px 18px; min-height: 0; flex: 1; }
  .roadmap-column { --status: #7cddb4; --status-bg: #7cddb40c; background: #090c133f; border: 1px solid #ffffff0a; border-radius: 13px; display: flex; flex-direction: column; min-width: 0; min-height: 0; }
  .roadmap-column[data-status='in_progress'] { --status: #edc17b; --status-bg: #edc17b0c; }
  .roadmap-column[data-status='planned'] { --status: #9aaeff; --status-bg: #9aaeff0c; }
  .column-heading { display: flex; gap: 9px; align-items: center; padding: 17px 16px 14px; background: var(--status-bg); border-bottom: 1px solid #ffffff08; border-radius: 13px 13px 0 0; }
  .status-dot { width: 7px; height: 7px; background: var(--status); border-radius: 50%; box-shadow: 0 0 12px color-mix(in srgb, var(--status) 20%, transparent); flex-shrink: 0; }
  .column-heading h3 { color: #e2e7f5; font-size: 12px; font-weight: 600; }
  .column-heading p { color: #8994ad; font-size: 10px; margin-top: 3px; }
  .column-count { margin-left: auto; color: var(--status); border: 1px solid color-mix(in srgb, var(--status) 20%, transparent); background: var(--status-bg); border-radius: 6px; min-width: 27px; text-align: center; font: 10px ui-monospace, monospace; padding: 4px; }
  .column-scroll { min-height: 0; overflow-y: auto; padding: 10px; scrollbar-width: thin; scrollbar-color: #45506c transparent; }
  .milestone-card { padding: 14px 13px 12px; background: linear-gradient(145deg, #ffffff05, #ffffff02); border: 1px solid #ffffff0e; border-radius: 9px; margin-bottom: 9px; }
  .milestone-card:hover { border-color: #7180a255; background: #ffffff06; }
  .card-meta { display: flex; justify-content: space-between; align-items: center; color: #8290ab; font-size: 8px; letter-spacing: .08em; margin-bottom: 9px; }
  .task-id { color: var(--status); letter-spacing: .03em; font: 10px ui-monospace, monospace; }
  h4 { color: #e1e7f5; font-size: 12px; font-weight: 600; line-height: 1.55; overflow-wrap: anywhere; }
  .card-summary { font-size: 11px; color: #a0abc2; line-height: 1.7; margin-top: 7px; display: -webkit-box; -webkit-line-clamp: 3; line-clamp: 3; -webkit-box-orient: vertical; overflow: hidden; overflow-wrap: anywhere; }
  .card-links { margin-top: 10px; display: flex; flex-wrap: wrap; gap: 8px; align-items: center; }
  .card-links a { color: #9caaca; font-size: 10px; display: inline-flex; align-items: center; gap: 5px; padding-block: 5px; }
  .card-links a:hover { color: #d9e1ff; }
  .card-links .pr-tag { color: var(--status); padding: 2px 6px; border: 1px solid color-mix(in srgb, var(--status) 18%, transparent); border-radius: 5px; background: var(--status-bg); }
  .vote-link { margin-top: 11px; min-height: 35px; display: flex; justify-content: center; align-items: center; gap: 7px; font-size: 10px; color: #b5c3ff; border: 1px solid #8b9ce52b; border-radius: 6px; background: #8b9ce508; text-decoration: none; }
  .vote-link:hover { color: #e4eaff; border-color: #8b9ce570; background: #8b9ce514; }
  .empty-column { padding: 30px 10px; text-align: center; color: #909bb4; font-size: 12px; }
  .empty-column span { font-size: 24px; color: #59657f; }
  .empty-column button { margin-top: 10px; color: #c5ceff; background: none; border: 0; text-decoration: underline; }
  .roadmap-message { min-height: 200px; flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 14px; text-align: center; padding: 24px; color: #acb6cc; font-size: 13px; }
  .roadmap-message h3 { font-size: 18px; }
  .retry-button { background: #5e6ad2; border: 1px solid #8c96e5; color: #fff; padding: 9px 18px; border-radius: 7px; }
  .loading-ring { width: 24px; height: 24px; border: 2px solid #8d9bf133; border-top-color: #a5b1fc; border-radius: 50%; animation: turn .8s linear infinite; }
  .roadmap-footer { padding: 14px 32px; border-top: 1px solid #ffffff0c; font-size: 10px; color: #929eb9; display: flex; justify-content: space-between; gap: 12px; }
  .roadmap-footer a { color: #b1bfed; }
  :is(button, a):focus-visible, .column-scroll:focus-visible { outline: 2px solid #a6b2ff; outline-offset: 3px; }
  @keyframes turn { to { transform: rotate(360deg); } }
  @media (max-width: 1080px) { .roadmap-controls { flex-wrap: wrap; } .search-box { flex-basis: 100%; } }
  @media (max-width: 700px) {
    .roadmap-modal { width: calc(100vw - 16px); max-height: calc(100dvh - 16px); border-radius: 16px; }
    .roadmap-shell { height: calc(100dvh - 20px); }
    .roadmap-header { padding: 22px 18px 14px; }
    .roadmap-metrics { padding: 0 18px 16px; gap: 7px; }
    .roadmap-controls { padding: 12px 16px; gap: 9px; }
    .category-filters { flex-wrap: nowrap; overflow-x: auto; width: 100%; padding-block: 2px; }
    .category-filters button, .close-button { min-height: 44px; }
    .search-hint { display: none; }
    .roadmap-note { padding: 10px 18px; flex-wrap: wrap; }
    .result-count { margin-left: 0; flex-basis: 100%; padding-left: 17px; }
    .roadmap-board { display: flex; flex-direction: column; overflow-y: auto; padding: 0 12px 14px; }
    .roadmap-column { flex-shrink: 0; min-height: 140px; max-height: 48dvh; }
    .card-links a, .vote-link { min-height: 44px; }
    .roadmap-footer { padding: 12px 18px; flex-wrap: wrap; }
    .search-box { height: 44px; }
  }
  @media (prefers-reduced-motion: reduce) { .loading-ring { animation: none; } }
</style>
