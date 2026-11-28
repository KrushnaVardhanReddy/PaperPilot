<script lang="ts">
  import { onMount } from 'svelte';
  import { ENABLE_DOWNLOAD_COUNT_BADGE, MIN_DOWNLOAD_COUNT_THRESHOLD } from '../constants';

  interface Props {
    repo?: string;
    npmPackage?: string;
    forceShow?: boolean;
    minThreshold?: number;
  }

  let {
    repo = 'KrushnaVardhanReddy/PaperPilot',
    npmPackage = '@usepaperpilot/mcp',
    forceShow = false,
    minThreshold = MIN_DOWNLOAD_COUNT_THRESHOLD
  }: Props = $props();

  let downloadCount = $state<number | null>(null);
  let githubDownloads = $state<number>(0);
  let npmDownloads = $state<number>(0);
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
      const cacheKey = `eco_dls_${repo}_${npmPackage}`;
      const cached = sessionStorage.getItem(cacheKey);
      if (cached) {
        const parsed = JSON.parse(cached);
        if (Date.now() - parsed.timestamp < 1000 * 60 * 15) { // 15 min cache
          downloadCount = parsed.count;
          githubDownloads = parsed.ghCount || 0;
          npmDownloads = parsed.npmCount || 0;
          latestReleaseTag = parsed.tag;
          loading = false;
          return;
        }
      }

      // Fetch GitHub Releases, npm, and PyPI stats in parallel
      const [ghRes, npmRes, pypiRes] = await Promise.allSettled([
        fetch(`https://api.github.com/repos/${repo}/releases`),
        fetch(`https://api.npmjs.org/downloads/range/2026-01-01:${new Date().toISOString().slice(0, 10)}/${npmPackage}`),
        fetch('https://pypistats.org/api/packages/paperpilot/overall')
          .then((r) => (r.ok ? r.json() : null))
          .catch(() => null)
      ]);

      let ghTotal = 0;
      let npmTotal = 0;
      let pypiTotal = 0;

      if (ghRes.status === 'fulfilled' && ghRes.value.ok) {
        const releases = await ghRes.value.json();
        if (Array.isArray(releases) && releases.length > 0) {
          latestReleaseTag = releases[0].tag_name || releases[0].name;
          for (const rel of releases) {
            if (Array.isArray(rel.assets)) {
              for (const asset of rel.assets) {
                ghTotal += Number(asset.download_count) || 0;
              }
            }
          }
        }
      }

      if (npmRes.status === 'fulfilled' && npmRes.value.ok) {
        const npmData = await npmRes.value.json();
        if (Array.isArray(npmData?.downloads)) {
          for (const item of npmData.downloads) {
            npmTotal += Number(item.downloads) || 0;
          }
        }
      }

      if (pypiRes.status === 'fulfilled' && Array.isArray(pypiRes.value?.data)) {
        const rows = pypiRes.value.data;
        const withoutMirrors = rows.filter((d: { category?: string }) => d.category === 'without_mirrors');
        const targetRows = withoutMirrors.length > 0 ? withoutMirrors : rows;
        for (const item of targetRows) {
          pypiTotal += Number(item.downloads) || 0;
        }
      }

      const total = ghTotal + npmTotal + pypiTotal;
      if (total > 0) {
        downloadCount = total;
        githubDownloads = ghTotal;
        npmDownloads = npmTotal;
        sessionStorage.setItem(
          cacheKey,
          JSON.stringify({
            count: total,
            ghCount: ghTotal,
            npmCount: npmTotal,
            tag: latestReleaseTag,
            timestamp: Date.now()
          })
        );
      }
    } catch {
      // Fallback silently if offline or rate limited
    } finally {
      loading = false;
    }
  });

  function formatCount(count: number): string {
    if (count >= 1_000_000) {
      const millions = Math.floor(count / 100_000) / 10;
      return `${millions % 1 === 0 ? millions.toFixed(0) : millions.toFixed(1)}M+`;
    }
    if (count >= 1_000) {
      // Use floor so 1,061 stays true to "1K+" (doesn't falsely round up before reaching 1,100)
      const thousands = Math.floor(count / 100) / 10;
      return `${thousands % 1 === 0 ? thousands.toFixed(0) : thousands.toFixed(1)}K+`;
    }
    return count.toLocaleString();
  }

  const shouldRender = $derived(
    isEnabled && !loading && downloadCount !== null && downloadCount >= minThreshold
  );
  const tooltip = $derived(
    downloadCount !== null
      ? `${downloadCount.toLocaleString()} total ecosystem downloads (${githubDownloads.toLocaleString()} desktop releases + ${npmDownloads.toLocaleString()} npm installs)${latestReleaseTag ? ` · Latest: ${latestReleaseTag}` : ''}`
      : 'Ecosystem Downloads'
  );
</script>

{#if shouldRender}
<a
  href={`https://github.com/${repo}/releases`}
  target="_blank"
  rel="noopener noreferrer"
  class="downloads-badge"
  title={tooltip}
>
  <svg class="dl-icon" viewBox="0 0 24 24" width="14" height="14" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">
    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
    <polyline points="7 10 12 15 17 10"></polyline>
    <line x1="12" y1="15" x2="12" y2="3"></line>
  </svg>
  <span class="label">Downloads</span>
  <span class="count-pill">
    {formatCount(downloadCount!)}
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
