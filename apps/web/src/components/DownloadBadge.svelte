<script lang="ts">
  import { onMount } from 'svelte';
  import { ENABLE_DOWNLOAD_COUNT_BADGE, MIN_DOWNLOAD_COUNT_THRESHOLD } from '../constants';

  interface Props {
    repo?: string;
    forceShow?: boolean;
    minThreshold?: number;
  }

  let {
    repo = 'KrushnaVardhanReddy/PaperPilot',
    forceShow = false,
    minThreshold = MIN_DOWNLOAD_COUNT_THRESHOLD
  }: Props = $props();

  let downloadCount = $state<number | null>(null);
  let latestReleaseTag = $state<string | null>(null);
  let loading = $state(true);

  // If feature flag is off and not explicitly forced, do not query or display
  const isEnabled = $derived(forceShow || ENABLE_DOWNLOAD_COUNT_BADGE);

  onMount(async () => {
    if (!isEnabled) {
      loading = false;
      return;
    }

    try {
      const cached = sessionStorage.getItem(`gh_dls_${repo}`);
      if (cached) {
        const parsed = JSON.parse(cached);
        if (Date.now() - parsed.timestamp < 1000 * 60 * 15) { // 15 min cache
          downloadCount = parsed.count;
          latestReleaseTag = parsed.tag;
          loading = false;
          return;
        }
      }

      const res = await fetch(`https://api.github.com/repos/${repo}/releases`);
      if (res.ok) {
        const releases = await res.json();
        let total = 0;
        if (Array.isArray(releases) && releases.length > 0) {
          latestReleaseTag = releases[0].tag_name || releases[0].name;
          for (const rel of releases) {
            if (Array.isArray(rel.assets)) {
              for (const asset of rel.assets) {
                total += Number(asset.download_count) || 0;
              }
            }
          }
        }
        downloadCount = total;
        sessionStorage.setItem(
          `gh_dls_${repo}`,
          JSON.stringify({ count: total, tag: latestReleaseTag, timestamp: Date.now() })
        );
      }
    } catch {
      // Fallback silently if offline or rate limited
    } finally {
      loading = false;
    }
  });

  const shouldRender = $derived(
    isEnabled && !loading && downloadCount !== null && downloadCount >= minThreshold
  );
</script>

{#if shouldRender}
<a
  href={`https://github.com/${repo}/releases`}
  target="_blank"
  rel="noopener noreferrer"
  class="downloads-badge"
  title={latestReleaseTag ? `Latest Release: ${latestReleaseTag}` : 'GitHub Releases'}
>
  <svg class="dl-icon" viewBox="0 0 24 24" width="14" height="14" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">
    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
    <polyline points="7 10 12 15 17 10"></polyline>
    <line x1="12" y1="15" x2="12" y2="3"></line>
  </svg>
  <span class="label">Downloads</span>
  <span class="count-pill">
    {downloadCount?.toLocaleString()}
  </span>
</a>
{/if}

<style>
  .downloads-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 3px 8px 3px 8px;
    font-size: 0.75rem;
    font-weight: 500;
    color: #94a3b8;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 9999px;
    text-decoration: none;
    transition: all 0.2s ease;
  }

  .downloads-badge:hover {
    color: #f1f5f9;
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(99, 102, 241, 0.4);
  }

  .dl-icon {
    color: #818cf8;
  }

  .count-pill {
    background: rgba(99, 102, 241, 0.2);
    color: #c7d2fe;
    font-weight: 700;
    font-size: 0.72rem;
    padding: 1px 6px;
    border-radius: 9999px;
    border: 1px solid rgba(99, 102, 241, 0.35);
  }
</style>
