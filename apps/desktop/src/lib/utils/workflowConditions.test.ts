import { describe, expect, it } from 'vitest';
import { editableCondition, predicateOf, validateCondition, conditionSummary } from './workflowConditions';
import type { WorkflowCondition } from '$lib/state/pipeline.svelte';

describe('Saved workflow condition compatibility', () => {
  it('retains false booleans and zero legacy thresholds', () => {
    const original = { if_scanned: false, if_page_count_gt: 0, has_form_fields: false, is_signed: false, is_encrypted: false };
    expect(editableCondition(original)).toEqual({ all: [
      { if_scanned: false }, { if_page_count_gt: 0 }, { has_form_fields: false }, { is_signed: false }, { is_encrypted: false }
    ] });
    expect(predicateOf({ if_scanned: false })).toBe('if_scanned');
    expect(validateCondition(original)).toBeNull();
    expect(conditionSummary(original)).toContain('Not scanned');
  });
  it('preserves nested groups, NOT and empty group semantics without flattening', () => {
    const condition: WorkflowCondition = { any: [{ not: { if_scanned: false } }, { all: [] }, { any: [] }] };
    expect(editableCondition(condition)).toBe(condition);
    expect(validateCondition(condition)).toBeNull();
    expect(conditionSummary({ not: { is_signed: false } })).toBe('NOT (Not signed)');
  });
  it('preserves every size comparison and exact byte bounds', () => {
    expect(editableCondition({ file_size_bytes: { gt: 0, lte: 4294967297 } })).toEqual({ all: [
      { file_size_bytes: { gt: 0 } }, { file_size_bytes: { lte: 4294967297 } }
    ] });
    expect(validateCondition({ file_size_bytes: { between: [0, 123456789] } })).toBeNull();
  });
  it.each([
    { value: 3 }, { comparator: 'Greater' }, { if_page_count_gt: -1 }, { if_page_count_gt: 1.5 },
    { if_page_count_gt: NaN }, { text_contains: '   ' }, { file_size_bytes: { gt: Infinity } },
    { file_size_bytes: { between: [100, 50] } }, { file_size_bytes: {} }
  ] as WorkflowCondition[])('rejects incomplete or invalid edits: %j', condition => {
    expect(validateCondition(condition)).not.toBeNull();
  });
  it('preserves unknown rules instead of silently dropping them', () => {
    const rule = { future_predicate: false } as WorkflowCondition;
    expect(editableCondition(rule)).toEqual(rule);
    expect(validateCondition(rule)).toContain('preserved');
  });
  it('blocks unsupported fields on compound rules before saving', () => {
    const rule = { all: [{ if_scanned: false }], future_predicate: false } as WorkflowCondition;
    expect(editableCondition(rule)).toEqual(rule);
    expect(validateCondition(rule)).toContain('preserved');
  });
});
