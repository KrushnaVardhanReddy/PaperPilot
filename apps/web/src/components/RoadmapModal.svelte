<script lang="ts">
    import { onMount } from 'svelte';
    import { fade, fly } from 'svelte/transition';

    let { show = $bindable(false) } = $props();

    type Status = 'shipped' | 'in_progress' | 'planned';

    interface Task {
        id: string;
        phase: string;
        title: string;
        summary: string;
        status: Status;
        pr: string | null;
    }

    let allTasks = $state<Task[]>([]);
    let searchQuery = $state('');
    let selectedCategory = $state('All');

    const categories = ['All', 'Core Engine', 'AI & Agents', 'Web & WASM', 'Desktop', 'Security'];

    onMount(async () => {
        try {
            const res = await fetch('/roadmap.json');
            allTasks = await res.json();
        } catch (e) {
            console.error("Failed to load roadmap:", e);
        }
    });

    let filteredTasks = $derived(allTasks.filter(task => {
        const matchesSearch = task.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
                              task.summary.toLowerCase().includes(searchQuery.toLowerCase());

        let matchesCategory = true;
        if (selectedCategory !== 'All') {
            const phaseLower = task.phase.toLowerCase();
            const catLower = selectedCategory.toLowerCase();
            if (catLower === 'core engine' && !phaseLower.includes('core')) matchesCategory = false;
            else if (catLower === 'ai & agents' && !phaseLower.includes('ai') && !phaseLower.includes('agent')) matchesCategory = false;
            else if (catLower === 'web & wasm' && !phaseLower.includes('web') && !phaseLower.includes('wasm')) matchesCategory = false;
            else if (catLower === 'desktop' && !phaseLower.includes('desktop')) matchesCategory = false;
            else if (catLower === 'security' && !phaseLower.includes('security')) matchesCategory = false;
        }

        return matchesSearch && matchesCategory;
    }));

    let shippedTasks = $derived(filteredTasks.filter(t => t.status === 'shipped'));
    let inProgressTasks = $derived(filteredTasks.filter(t => t.status === 'in_progress'));
    let plannedTasks = $derived(filteredTasks.filter(t => t.status === 'planned'));

    function close() {
        show = false;
    }

    function handleBackdropClick(e: MouseEvent) {
        if (e.target === e.currentTarget) {
            close();
        }
    }

    function createIssueUrl(title: string) {
        const url = new URL('https://github.com/KrushnaVardhanReddy/PaperPilot/issues/new');
        url.searchParams.set('template', 'feature_request.yml');
        url.searchParams.set('title', `[FEATURE]: ${title}`);
        return url.toString();
    }
</script>

{#if show}
<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="modal-backdrop" onclick={handleBackdropClick} transition:fade={{ duration: 200 }}>
    <div class="modal-content" transition:fly={{ y: 20, duration: 300 }}>
        <header class="modal-header">
            <div class="header-left">
                <h2>PaperPilot Live Roadmap</h2>
                <div class="metrics">
                    <span class="pill">78 Verified Tools</span>
                    <span class="pill">Zero Cloud / Air-Gapped</span>
                </div>
            </div>
            <button class="close-btn" onclick={close} aria-label="Close roadmap">✕</button>
        </header>

        <div class="controls">
            <input type="text" class="search-input" placeholder="Search features..." bind:value={searchQuery} />
            <div class="categories">
                {#each categories as category}
                    <button
                        class="category-btn"
                        class:active={selectedCategory === category}
                        onclick={() => selectedCategory = category}
                    >
                        {category}
                    </button>
                {/each}
            </div>
        </div>

        <div class="kanban-board">
            <div class="column column-shipped">
                <h3>🟢 Shipped & Verified <span class="count">{shippedTasks.length}</span></h3>
                <div class="cards">
                    {#each shippedTasks as task}
                        <div class="card shipped">
                            <div class="card-header">
                                <span class="phase">{task.phase.split('—')[0].trim()}</span>
                                <span class="id">{task.id}</span>
                            </div>
                            <h4 class="title">{task.title}</h4>
                            <p class="summary">{task.summary}</p>
                            {#if task.pr}
                                <div class="pr-tag">{task.pr}</div>
                            {/if}
                        </div>
                    {/each}
                </div>
            </div>

            <div class="column column-progress">
                <h3>🟡 In Active Progress <span class="count">{inProgressTasks.length}</span></h3>
                <div class="cards">
                    {#each inProgressTasks as task}
                        <div class="card progress">
                            <div class="card-header">
                                <span class="phase">{task.phase.split('—')[0].trim()}</span>
                                <span class="id">{task.id}</span>
                            </div>
                            <h4 class="title">{task.title}</h4>
                            <p class="summary">{task.summary}</p>
                            {#if task.pr}
                                <div class="pr-tag">{task.pr}</div>
                            {/if}
                        </div>
                    {/each}
                </div>
            </div>

            <div class="column column-planned">
                <h3>🔵 Planned / Community Backlog <span class="count">{plannedTasks.length}</span></h3>
                <div class="cards">
                    {#each plannedTasks as task}
                        <div class="card planned">
                            <div class="card-header">
                                <span class="phase">{task.phase.split('—')[0].trim()}</span>
                                <span class="id">{task.id}</span>
                            </div>
                            <h4 class="title">{task.title}</h4>
                            <p class="summary">{task.summary}</p>
                            <a href={createIssueUrl(task.title)} target="_blank" rel="noopener noreferrer" class="vote-btn">
                                👍 Vote / Request on GitHub
                            </a>
                        </div>
                    {/each}
                </div>
            </div>
        </div>
    </div>
</div>
{/if}

<style>
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background-color: rgba(0, 0, 0, 0.75);
        backdrop-filter: blur(8px);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
        padding: 24px;
        box-sizing: border-box;
    }

    .modal-content {
        background-color: var(--bg-surface, #1a1b1e);
        border: 1px solid var(--border-color, #2c2e33);
        border-radius: 12px;
        width: 100%;
        max-width: 1400px;
        height: 90vh;
        display: flex;
        flex-direction: column;
        box-shadow: 0 24px 48px rgba(0, 0, 0, 0.5);
        overflow: hidden;
    }

    .modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 20px 24px;
        border-bottom: 1px solid var(--border-color, #2c2e33);
        background: linear-gradient(to right, rgba(26, 27, 30, 1), rgba(26, 27, 30, 0.8));
    }

    .header-left h2 {
        margin: 0 0 8px 0;
        font-size: 1.5rem;
        color: #fff;
    }

    .metrics {
        display: flex;
        gap: 12px;
    }

    .pill {
        font-size: 0.75rem;
        padding: 4px 10px;
        border-radius: 9999px;
        background-color: rgba(255, 255, 255, 0.1);
        color: #e0e0e0;
        font-weight: 500;
        border: 1px solid rgba(255, 255, 255, 0.1);
    }

    .close-btn {
        background: none;
        border: none;
        color: #999;
        font-size: 1.5rem;
        cursor: pointer;
        padding: 4px;
        border-radius: 4px;
        transition: color 0.2s, background-color 0.2s;
    }

    .close-btn:hover {
        color: #fff;
        background-color: rgba(255, 255, 255, 0.1);
    }

    .controls {
        padding: 16px 24px;
        border-bottom: 1px solid var(--border-color, #2c2e33);
        display: flex;
        gap: 16px;
        align-items: center;
        flex-wrap: wrap;
        background-color: rgba(0, 0, 0, 0.2);
    }

    .search-input {
        background-color: rgba(0, 0, 0, 0.3);
        border: 1px solid var(--border-color, #2c2e33);
        color: #fff;
        padding: 8px 16px;
        border-radius: 6px;
        flex-grow: 1;
        max-width: 300px;
    }

    .search-input:focus {
        outline: none;
        border-color: #3b82f6;
    }

    .categories {
        display: flex;
        gap: 8px;
        flex-wrap: wrap;
    }

    .category-btn {
        background: transparent;
        border: 1px solid var(--border-color, #2c2e33);
        color: #999;
        padding: 6px 12px;
        border-radius: 6px;
        cursor: pointer;
        font-size: 0.85rem;
        transition: all 0.2s;
    }

    .category-btn:hover {
        border-color: #666;
        color: #ddd;
    }

    .category-btn.active {
        background-color: #3b82f6;
        border-color: #3b82f6;
        color: #fff;
    }

    .kanban-board {
        display: flex;
        flex: 1;
        overflow: hidden;
        gap: 16px;
        padding: 24px;
        background-color: var(--bg-primary, #0f1115);
    }

    .column {
        flex: 1;
        display: flex;
        flex-direction: column;
        background-color: rgba(255, 255, 255, 0.02);
        border-radius: 8px;
        border: 1px solid var(--border-color, #2c2e33);
        overflow: hidden;
    }

    .column h3 {
        margin: 0;
        padding: 16px;
        font-size: 1.1rem;
        border-bottom: 1px solid var(--border-color, #2c2e33);
        background-color: rgba(0, 0, 0, 0.2);
        display: flex;
        justify-content: space-between;
        align-items: center;
        color: #e0e0e0;
    }

    .column-shipped h3 { border-top: 3px solid #10b981; }
    .column-progress h3 { border-top: 3px solid #f59e0b; }
    .column-planned h3 { border-top: 3px solid #3b82f6; }

    .count {
        background-color: rgba(255, 255, 255, 0.1);
        padding: 2px 8px;
        border-radius: 12px;
        font-size: 0.8rem;
    }

    .cards {
        flex: 1;
        overflow-y: auto;
        padding: 12px;
        display: flex;
        flex-direction: column;
        gap: 12px;
    }

    .card {
        background-color: var(--bg-surface, #1a1b1e);
        border: 1px solid var(--border-color, #2c2e33);
        border-radius: 6px;
        padding: 16px;
        transition: transform 0.2s, box-shadow 0.2s;
    }

    .card:hover {
        transform: translateY(-2px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    }

    .card.shipped { border-left: 3px solid #10b981; }
    .card.progress { border-left: 3px solid #f59e0b; }
    .card.planned { border-left: 3px solid #3b82f6; }

    .card-header {
        display: flex;
        justify-content: space-between;
        margin-bottom: 8px;
        font-size: 0.75rem;
        color: #999;
    }

    .phase {
        background-color: rgba(255, 255, 255, 0.05);
        padding: 2px 6px;
        border-radius: 4px;
    }

    .title {
        margin: 0 0 8px 0;
        font-size: 1rem;
        color: #fff;
    }

    .summary {
        margin: 0;
        font-size: 0.85rem;
        color: #aaa;
        line-height: 1.4;
    }

    .pr-tag {
        display: inline-block;
        margin-top: 12px;
        background-color: rgba(16, 185, 129, 0.1);
        color: #10b981;
        border: 1px solid rgba(16, 185, 129, 0.2);
        padding: 2px 8px;
        border-radius: 4px;
        font-size: 0.75rem;
    }

    .vote-btn {
        display: block;
        margin-top: 12px;
        text-align: center;
        background-color: rgba(59, 130, 246, 0.1);
        color: #3b82f6;
        border: 1px solid rgba(59, 130, 246, 0.2);
        padding: 6px;
        border-radius: 4px;
        text-decoration: none;
        font-size: 0.85rem;
        font-weight: 500;
        transition: all 0.2s;
    }

    .vote-btn:hover {
        background-color: rgba(59, 130, 246, 0.2);
        color: #60a5fa;
    }

    /* Scrollbar Styling */
    .cards::-webkit-scrollbar {
        width: 6px;
    }
    .cards::-webkit-scrollbar-track {
        background: transparent;
    }
    .cards::-webkit-scrollbar-thumb {
        background-color: var(--border-color, #2c2e33);
        border-radius: 3px;
    }
</style>
