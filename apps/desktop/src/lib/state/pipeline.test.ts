import { describe, it, expect, beforeEach, vi } from 'vitest';
import { PipelineState, DEFAULT_PRESETS } from './pipeline.svelte';

describe('PipelineState Persistence & Presets', () => {
  let state: PipelineState;

  beforeEach(() => {
    // Reset DOM/localStorage environment
    vi.stubGlobal('localStorage', {
      getItem: vi.fn(),
      setItem: vi.fn(),
      clear: vi.fn()
    });

    // Stub crypto.randomUUID
    let uuidCounter = 0;
    vi.stubGlobal('crypto', {
      randomUUID: () => `mock-uuid-${++uuidCounter}`
    });

    state = new PipelineState();
  });

  it('DEFAULT_PRESETS contains the 3 specified industry workflows', () => {
    expect(DEFAULT_PRESETS).toHaveLength(3);
    const ids = DEFAULT_PRESETS.map(p => p.id);
    expect(ids).toContain('preset-legal-discovery');
    expect(ids).toContain('preset-clean-scan');
    expect(ids).toContain('preset-redact-protect');
  });

  it('initStorage loads defaults when empty', () => {
    vi.mocked(localStorage.getItem).mockReturnValue(null);
    state.initStorage();
    expect(state.savedFlows).toHaveLength(3);
    expect(state.savedFlows).toEqual([...DEFAULT_PRESETS]);
  });

  it('initStorage merges stored user flows with defaults', () => {
    const mockUserFlow = {
      id: 'mock-uuid',
      name: 'User Flow',
      steps: [],
      updatedAt: '2025-01-01',
      isPreset: false
    };
    vi.mocked(localStorage.getItem).mockReturnValue(JSON.stringify([mockUserFlow]));

    state.initStorage();
    // Default 3 + User 1
    expect(state.savedFlows).toHaveLength(4);
    const flowIds = state.savedFlows.map(f => f.id);
    expect(flowIds).toContain('mock-uuid');
    expect(flowIds).toContain('preset-legal-discovery');
  });

  it('loadFlow loads steps and sets activeFlowId', () => {
    state.initStorage();
    const success = state.loadFlow('preset-clean-scan');

    expect(success).toBe(true);
    expect(state.activeFlowId).toBe('preset-clean-scan');
    expect(state.steps).toHaveLength(3); // Clean scan has 3 steps
    expect(state.steps[0].operationId).toBe('pdf_remove_blank');
  });

  it('saveCurrentFlow creates new custom flow and stores it', () => {
    state.initStorage();

    // Manually add steps to state
    state.steps = [
      { id: 's1', operationId: 'pdf_compress', label: 'C', icon: 'i', params: {} }
    ];

    const newFlow = state.saveCurrentFlow('My Backup Flow', 'Does backup');

    expect(newFlow.name).toBe('My Backup Flow');
    expect(newFlow.isPreset).toBe(false);
    expect(state.savedFlows[0].id).toBe(newFlow.id);
    expect(state.activeFlowId).toBe(newFlow.id);

    // Verify it persisted
    expect(localStorage.setItem).toHaveBeenCalledWith('paperpilot_saved_flows', JSON.stringify(state.savedFlows));
  });

  it('deleteFlow prevents deleting presets but deletes user flows', () => {
    state.initStorage();

    // Attempt delete preset
    const deletedPreset = state.deleteFlow('preset-legal-discovery');
    expect(deletedPreset).toBe(false);
    expect(state.savedFlows.find(f => f.id === 'preset-legal-discovery')).toBeDefined();

    // Create and delete user flow
    const newFlow = state.saveCurrentFlow('To Delete');
    expect(state.savedFlows).toHaveLength(4);

    const deletedUser = state.deleteFlow(newFlow.id);
    expect(deletedUser).toBe(true);
    expect(state.savedFlows).toHaveLength(3);

    // Verify sync
    expect(localStorage.setItem).toHaveBeenCalled();
  });

  it('exportFlowJson and importFlowFromJson roundtrip correctly', () => {
    state.initStorage();

    // load preset, modify it conceptually by saving it as custom
    state.loadFlow('preset-clean-scan');
    const myFlow = state.saveCurrentFlow('Import Export Test');

    // Export
    const jsonStr = state.exportFlowJson(myFlow.id);
    const parsed = JSON.parse(jsonStr);
    expect(parsed.name).toBe('Import Export Test');

    // Import
    const importedFlow = state.importFlowFromJson(jsonStr);

    // It should generate a new ID on import
    expect(importedFlow.id).not.toBe(myFlow.id);
    expect(importedFlow.name).toBe('Import Export Test');
    expect(importedFlow.isPreset).toBe(false);

    // Verify it was loaded
    expect(state.activeFlowId).toBe(importedFlow.id);
  });

  it('setStepCondition updates the step condition', () => {
    state.initStorage();
    state.steps = [
      { id: 's1', operationId: 'pdf_compress', label: 'C', icon: 'i', params: {} }
    ];

    state.setStepCondition('s1', { if_scanned: true }, 'skip');
    expect(state.steps[0].condition).toEqual({ if_scanned: true });
    expect(state.steps[0].on_no_match).toBe('skip');

    state.removeStepCondition('s1');
    expect(state.steps[0].condition).toBeUndefined();
    expect(state.steps[0].on_no_match).toBeUndefined();
  });

  it('exportFlowJson includes condition block', () => {
    state.initStorage();
    state.steps = [
      { id: 's1', operationId: 'pdf_compress', label: 'C', icon: 'i', params: {}, condition: { if_scanned: true }, on_no_match: 'skip' }
    ];

    const myFlow = state.saveCurrentFlow('Condition Test');
    const jsonStr = state.exportFlowJson(myFlow.id);
    const parsed = JSON.parse(jsonStr);

    expect(parsed.steps[0].condition).toBeDefined();
    expect(parsed.steps[0].condition.if_scanned).toBe(true);
    expect(parsed.steps[0].on_no_match).toBe('skip');
  });

});
