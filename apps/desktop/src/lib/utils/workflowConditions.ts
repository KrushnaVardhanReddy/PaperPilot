import type { WorkflowCondition, WorkflowConditionLeaf } from '$lib/state/pipeline.svelte';

export type Predicate = 'if_scanned' | 'if_page_count_gt' | 'page_count' | 'text_contains' | 'has_form_fields' | 'is_signed' | 'is_encrypted' | 'file_size_bytes';
export const predicates: { value: Predicate; label: string }[] = [
  { value: 'if_scanned', label: 'Scanned document' }, { value: 'page_count', label: 'Page count' },
  { value: 'text_contains', label: 'Text contains' }, { value: 'has_form_fields', label: 'Has form fields' },
  { value: 'is_signed', label: 'Contains a digital signature' }, { value: 'is_encrypted', label: 'Encrypted document' },
  { value: 'file_size_bytes', label: 'File size' }, { value: 'if_page_count_gt', label: 'Page count greater than (legacy)' }
];
export const pageComparators = ['Greater', 'GreaterOrEqual', 'Equal', 'Less', 'LessOrEqual'] as const;
export type NumericKey = 'eq' | 'gt' | 'gte' | 'lt' | 'lte' | 'between';
export function predicateOf(leaf: WorkflowConditionLeaf): Predicate | 'custom' {
  if (leaf.comparator !== undefined || leaf.value !== undefined) return 'page_count';
  return predicates.find(p => p.value !== 'page_count' && p.value in leaf)?.value ?? 'custom';
}
export function defaultPredicate(type: Predicate): WorkflowConditionLeaf {
  if (type === 'page_count') return { comparator: 'Greater', value: 1 };
  if (type === 'file_size_bytes') return { file_size_bytes: { gte: 1024 } };
  if (type === 'text_contains') return { text_contains: '' };
  if (type === 'if_page_count_gt') return { if_page_count_gt: 1 };
  return { [type]: true };
}

// A legacy leaf combines its fields with AND. Present separate editable rows
// without changing the saved object until the user explicitly applies edits.
export function editableCondition(node: WorkflowCondition): WorkflowCondition {
  if ('all' in node || 'any' in node || 'not' in node) return node;
  const parts: WorkflowCondition[] = [];
  for (const [key, value] of Object.entries(node)) {
    if (key === 'value') continue;
    if (key === 'comparator') parts.push({ comparator: node.comparator, value: node.value });
    else if (key === 'file_size_bytes' && value && Object.keys(value).length > 1) {
      for (const [op, amount] of Object.entries(value)) parts.push({ file_size_bytes: { [op]: amount } });
    } else parts.push({ [key]: value });
  }
  return parts.length > 1 ? { all: parts } : node;
}

export function validateCondition(node: WorkflowCondition, depth = 0): string | null {
  if (depth > 24) return 'Conditions may nest at most 24 levels.';
  const group = 'all' in node ? 'all' : 'any' in node ? 'any' : 'not' in node ? 'not' : null;
  if (group && Object.keys(node).some(key => key !== group)) return 'This saved rule contains unsupported fields and has been preserved. Remove or clear it explicitly to replace it.';
  if ('all' in node || 'any' in node) {
    const children = 'all' in node ? node.all : node.any;
    if (children.length > 64) return 'A condition group may contain at most 64 rules.';
    return children.map(child => validateCondition(child, depth + 1)).find(Boolean) ?? null;
  }
  if ('not' in node) return validateCondition(node.not, depth + 1);
  const integer = (v: unknown) => typeof v === 'number' && Number.isSafeInteger(v) && v >= 0;
  if (Object.keys(node).length === 0) return 'Choose a condition.';
  for (const [key, value] of Object.entries(node)) {
    if (['if_scanned', 'has_form_fields', 'is_signed', 'is_encrypted'].includes(key) && typeof value !== 'boolean') return 'Choose Yes or No.';
    if (['value', 'if_page_count_gt'].includes(key) && (!integer(value) || Number(value) > 4294967295)) return 'Page counts must be whole numbers between 0 and 4294967295.';
    if (key === 'comparator' && (!pageComparators.includes(value) || node.value === undefined)) return 'Choose a page comparator and value.';
    if (key === 'text_contains' && (typeof value !== 'string' || !value.trim())) return 'Enter text to find.';
    if (key === 'file_size_bytes') {
      const entries = Object.entries(value ?? {});
      if (!entries.length) return 'Choose a file-size comparison.';
      for (const [op, amount] of entries) {
        if (op === 'between') {
          if (!Array.isArray(amount) || amount.length !== 2 || !amount.every(integer) || amount[0] > amount[1]) return 'Enter an ordered size range in whole bytes.';
        } else if (!['eq', 'gt', 'gte', 'lt', 'lte'].includes(op) || !integer(amount)) return 'File sizes must be nonnegative whole bytes within the safe integer range.';
      }
    }
    if (!['if_scanned', 'if_page_count_gt', 'comparator', 'value', 'text_contains', 'has_form_fields', 'is_signed', 'is_encrypted', 'file_size_bytes'].includes(key)) return 'This saved rule is unsupported by this editor and has been preserved. Replace it explicitly to edit it.';
  }
  if (node.value !== undefined && !node.comparator) return 'Choose a page comparator.';
  return null;
}

export function conditionSummary(node: WorkflowCondition): string {
  if ('all' in node) return `ALL (${node.all.length} rules)`;
  if ('any' in node) return `ANY (${node.any.length} rules)`;
  if ('not' in node) return `NOT (${conditionSummary(node.not)})`;
  const parts: string[] = [];
  for (const [key, label] of [['if_scanned', 'Scanned'], ['has_form_fields', 'Has forms'], ['is_signed', 'Signed'], ['is_encrypted', 'Encrypted']] as const) {
    if (node[key] !== undefined) parts.push(node[key] ? label : `Not ${label.toLowerCase()}`);
  }
  if (node.if_page_count_gt !== undefined) parts.push(`Pages > ${node.if_page_count_gt}`);
  if (node.comparator !== undefined) parts.push(`Pages ${node.comparator} ${node.value}`);
  if (node.text_contains !== undefined) parts.push(`Contains “${node.text_contains}”`);
  if (node.file_size_bytes !== undefined) parts.push('File size');
  return parts.join(', ') || 'Saved custom rule';
}
