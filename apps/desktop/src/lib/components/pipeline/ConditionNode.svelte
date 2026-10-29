<script lang="ts">
  import ConditionNode from './ConditionNode.svelte';
  import type { WorkflowCondition, WorkflowConditionLeaf } from '$lib/state/pipeline.svelte';
  import { editableCondition, predicates, predicateOf, defaultPredicate, pageComparators } from '$lib/utils/workflowConditions';
  import type { Predicate, NumericKey } from '$lib/utils/workflowConditions';
  let { node, onchange, depth = 0 }: { node: WorkflowCondition; onchange: (node: WorkflowCondition) => void; depth?: number } = $props();
  let view = $derived(editableCondition(node));
  let kind = $derived('all' in view ? 'all' : 'any' in view ? 'any' : 'not' in view ? 'not' : 'leaf');
  let children = $derived('all' in view ? view.all : 'any' in view ? view.any : 'not' in view ? [view.not] : []);
  let leaf = $derived(view as WorkflowConditionLeaf);
  let predicate = $derived(predicateOf(leaf));
  let size = $derived(leaf.file_size_bytes ?? { gte: 1024 });
  let sizeKey = $derived((Object.keys(size)[0] ?? 'gte') as NumericKey);
  let amount = $derived(Object.values(size)[0] ?? 0);
  let unit = $state(1);
  function groupNode(type: 'all' | 'any' | 'not', next: WorkflowCondition[]): WorkflowCondition {
    const preserved = { ...view } as Record<string, unknown>;
    delete preserved.all; delete preserved.any; delete preserved.not;
    return { ...preserved, [type]: type === 'not' ? (next.length === 1 ? next[0] : { all: next }) : next } as WorkflowCondition;
  }
  function updateChildren(next: WorkflowCondition[]) {
    onchange(groupNode(kind as 'all' | 'any' | 'not', next));
  }
  function numberValue(e: Event) { return (e.target as HTMLInputElement).valueAsNumber; }
  function sizeValue(e: Event, index?: number) {
    const value = numberValue(e) * unit;
    if (sizeKey === 'between') {
      const bounds = Array.isArray(amount) ? [...amount] as [number, number] : [0, 0] as [number, number];
      bounds[index ?? 0] = value; onchange({ file_size_bytes: { between: bounds } });
    } else onchange({ file_size_bytes: { [sizeKey]: value } });
  }
</script>

<div class="condition-node" class:group={kind !== 'leaf'}>
  {#if depth > 24}
    <p role="alert">This condition is too deeply nested to edit.</p>
  {:else if kind !== 'leaf'}
    <div class="group-toolbar">
      <select aria-label="Condition group" value={kind} onchange={e => {
        onchange(groupNode(e.currentTarget.value as 'all' | 'any' | 'not', children));
      }}>
        <option value="all">ALL rules (AND)</option><option value="any">ANY rule (OR)</option><option value="not">NOT</option>
      </select>
      {#if kind !== 'not'}<button type="button" onclick={() => updateChildren([...children, defaultPredicate('if_scanned')])}>+ Add rule</button>{/if}
    </div>
    {#if !children.length}<p>{kind === 'all' ? 'An empty ALL group always matches.' : 'An empty ANY group never matches.'}</p>{/if}
    {#each children as child, index}
      <div class="child">
        <ConditionNode node={child} depth={depth + 1} onchange={updated => updateChildren(children.map((c, i) => i === index ? updated : c))} />
        {#if kind !== 'not'}<button type="button" aria-label={`Remove rule ${index + 1}`} onclick={() => updateChildren(children.filter((_, i) => i !== index))}>×</button>{/if}
      </div>
    {/each}
  {:else}
    <div class="rule-fields">
      <select aria-label="Rule property" value={predicate} onchange={e => onchange(defaultPredicate(e.currentTarget.value as Predicate))}>
        {#if predicate === 'custom'}<option value="custom" disabled>Saved custom rule (preserved)</option>{/if}
        {#each predicates as p}<option value={p.value}>{p.label}</option>{/each}
      </select>
      {#if ['if_scanned', 'has_form_fields', 'is_signed', 'is_encrypted'].includes(predicate)}
        <select aria-label="Expected value" value={String(leaf[predicate as keyof WorkflowConditionLeaf])} onchange={e => onchange({ [predicate]: e.currentTarget.value === 'true' })}><option value="true">Yes</option><option value="false">No</option></select>
      {:else if predicate === 'page_count'}
        <select aria-label="Page comparator" value={leaf.comparator} onchange={e => onchange({ ...leaf, comparator: e.currentTarget.value as WorkflowConditionLeaf['comparator'] })}>{#each pageComparators as comparator}<option value={comparator}>{comparator}</option>{/each}</select>
        <input aria-label="Page count" type="number" min="0" step="1" value={leaf.value} oninput={e => onchange({ ...leaf, value: numberValue(e) })} />
      {:else if predicate === 'if_page_count_gt'}
        <input aria-label="Legacy page count threshold" type="number" min="0" step="1" value={leaf.if_page_count_gt} oninput={e => onchange({ if_page_count_gt: numberValue(e) })} />
      {:else if predicate === 'text_contains'}
        <input aria-label="Text to find" value={leaf.text_contains} oninput={e => onchange({ text_contains: e.currentTarget.value })} />
      {:else if predicate === 'file_size_bytes'}
        <select aria-label="File size comparator" value={sizeKey} onchange={e => onchange({ file_size_bytes: { [e.currentTarget.value]: e.currentTarget.value === 'between' ? [0, 1024] : 1024 } })}>
          {#each ['eq', 'gt', 'gte', 'lt', 'lte', 'between'] as op}<option value={op}>{op}</option>{/each}
        </select>
        {#if sizeKey === 'between'}
          <input aria-label="Minimum size" type="number" min="0" value={Array.isArray(amount) ? amount[0] / unit : 0} oninput={e => sizeValue(e, 0)} />
          <input aria-label="Maximum size" type="number" min="0" value={Array.isArray(amount) ? amount[1] / unit : 0} oninput={e => sizeValue(e, 1)} />
        {:else}<input aria-label="File size" type="number" min="0" value={Number(amount) / unit} oninput={e => sizeValue(e)} />{/if}
        <select aria-label="Size unit" bind:value={unit}><option value={1}>bytes</option><option value={1024}>KiB</option><option value={1048576}>MiB</option></select>
      {/if}
      <button type="button" onclick={() => onchange({ not: node })}>Invert (NOT)</button>
      <button type="button" onclick={() => onchange({ all: [node, defaultPredicate('if_scanned')] })}>+ Add rule</button>
    </div>
  {/if}
</div>

<style>
  .condition-node { min-width: 0; padding: 10px; border: 1px solid var(--border-color, #505465); border-radius: 8px; background: var(--bg-surface, #20232e); }
  .group { border-left: 3px solid var(--accent-primary, #9096ff); }
  .group-toolbar, .rule-fields { display: flex; flex-wrap: wrap; gap: 8px; align-items: center; }
  .child { display: flex; gap: 6px; align-items: start; margin-top: 8px; } .child > :global(.condition-node) { flex: 1; }
  select, input, button { max-width: 100%; min-width: 0; padding: 7px; color: var(--text-primary, #f6f7fb); background: var(--bg-surface, #20232e); border: 1px solid var(--border-color, #676b80); border-radius: 5px; }
  input { width: 115px; } button { cursor: pointer; } p { font-size: .85rem; }
  input:focus-visible, select:focus-visible, button:focus-visible { outline: 2px solid var(--accent-primary, #a0a8ff); outline-offset: 2px; }
</style>
