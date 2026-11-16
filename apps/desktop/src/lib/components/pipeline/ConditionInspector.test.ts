import { beforeEach, afterEach, describe, expect, it } from 'vitest';
import { render, screen, fireEvent, cleanup } from '@testing-library/svelte';
import { tick } from 'svelte';
import ConditionInspector from './ConditionInspector.svelte';
import { pipelineState } from '$lib/state/pipeline.svelte';
import type { WorkflowCondition } from '$lib/state/pipeline.svelte';

function select(condition: WorkflowCondition) {
  pipelineState.addStep('pdf_rotate');
  const id = pipelineState.selectedStepId!;
  pipelineState.setStepCondition(id, condition, 'error');
  render(ConditionInspector); return id;
}
describe('Condition editor uses actual pipeline state', () => {
  beforeEach(() => { pipelineState.clearPipeline(); localStorage.clear(); });
  afterEach(cleanup);
  it('viewing false and legacy predicates does not rewrite the recipe', async () => {
    const original = { if_scanned: false, if_page_count_gt: 0 };
    select(original); await tick();
    expect(screen.getByLabelText('Expected value')).toHaveValue('false');
    expect(screen.getByLabelText('Legacy page count threshold')).toHaveValue(0);
    expect(pipelineState.steps[0].condition).toEqual(original);
    expect(pipelineState.steps[0].on_no_match).toBe('error');
  });
  it('applies nested NOT/ANY changes while retaining untouched size bounds', async () => {
    const original: WorkflowCondition = { all: [{ not: { if_scanned: false } }, { file_size_bytes: { between: [10, 987654] } }] };
    select(original); await tick();
    await fireEvent.change(screen.getByLabelText('Expected value'), { target: { value: 'true' } });
    expect(pipelineState.steps[0].condition).toEqual(original);
    await fireEvent.click(screen.getByText('Apply conditions'));
    expect(pipelineState.steps[0].condition).toEqual({ all: [{ not: { if_scanned: true } }, { file_size_bytes: { between: [10, 987654] } }] });
    expect(pipelineState.steps[0].on_no_match).toBe('error');
  });
  it('supports ALL/ANY and removes individual rules explicitly', async () => {
    select({ all: [{ if_scanned: false }, { is_signed: false }] }); await tick();
    await fireEvent.change(screen.getByLabelText('Condition group'), { target: { value: 'any' } });
    await fireEvent.click(screen.getByText('Apply conditions'));
    expect(pipelineState.steps[0].condition).toEqual({ any: [{ if_scanned: false }, { is_signed: false }] });
    await fireEvent.click(screen.getByLabelText('Remove rule 2'));
    await fireEvent.click(screen.getByText('Apply conditions'));
    expect(pipelineState.steps[0].condition).toEqual({ any: [{ if_scanned: false }] });
  });
  it('invalid draft input never destroys the last saved condition', async () => {
    const original = { comparator: 'GreaterOrEqual' as const, value: 0 };
    select(original); await tick();
    await fireEvent.input(screen.getByLabelText('Page count'), { target: { value: '-1' } });
    expect(screen.getByText('Apply conditions')).toBeDisabled();
    expect(pipelineState.steps[0].condition).toEqual(original);
    expect(screen.getByRole('alert')).toHaveTextContent('Page counts');
  });
  it('editing a child or changing a group never drops unknown saved group fields', async () => {
    const original = { all: [{ if_scanned: false }], future_predicate: false } as WorkflowCondition;
    select(original); await tick();
    await fireEvent.change(screen.getByLabelText('Expected value'), { target: { value: 'true' } });
    await fireEvent.change(screen.getByLabelText('Condition group'), { target: { value: 'any' } });
    expect(screen.getByText('Apply conditions')).toBeDisabled();
    expect(screen.getByRole('alert')).toHaveTextContent('preserved');
    expect(pipelineState.steps[0].condition).toEqual(original);
  });
});
