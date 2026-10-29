<script lang="ts">
  import { SvelteFlow, Background, Controls, MiniMap, Panel, type Node, type Edge, type Connection } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
  import { pipelineState } from '$lib/state/pipeline.svelte';
  import { appState } from '$lib/state/app.svelte';
  import ActionNode from './nodes/ActionNode.svelte';
  import ConditionNode from './nodes/ConditionNode.svelte';
  import TriggerNode from './nodes/TriggerNode.svelte';
  import AutoLayoutButton from './AutoLayoutButton.svelte';

  const nodeTypes = {
    action: ActionNode,
    condition: ConditionNode,
    trigger: TriggerNode
  };

  // Create derived nodes and edges from pipelineState.steps
  let nodes = $derived.by(() => {
    const svelteNodes: Node[] = [];

    // Add Trigger Node reflecting loaded documents
    const docCount = appState.documents.length;
    svelteNodes.push({
      id: 'trigger-node',
      type: 'trigger',
      position: { x: 50, y: 200 },
      data: {
        label: docCount === 0
          ? 'No Documents'
          : docCount === 1
            ? appState.documents[0].name
            : `${docCount} Documents (Batch)`,
        count: docCount,
        icon: docCount > 1 ? '📚' : '📄'
      }
    });

    pipelineState.steps.forEach((step) => {
      // Keep action tools as 'action' nodes (with condition badge).
      // Only render as 'condition' if it's an explicit flow-control condition block.
      const isCondition = (step as any).type === 'condition';
      svelteNodes.push({
        id: step.id,
        type: isCondition ? 'condition' : 'action',
        position: step.position || { x: 0, y: 0 },
        data: step as unknown as Record<string, unknown>,
        selected: pipelineState.selectedStepId === step.id
      });
    });

    return svelteNodes;
  });

  let edges = $derived.by(() => {
    const svelteEdges: Edge[] = [];
    const steps = pipelineState.steps;

    steps.forEach((step, index) => {
      if (step.depends_on && step.depends_on.length > 0) {
        step.depends_on.forEach((depId) => {
          svelteEdges.push({
            id: `e-${depId}-${step.id}`,
            source: depId,
            target: step.id
          });
        });
      } else {
        // Fallback: connect to previous node, or trigger if it's the first step
        const sourceId = index === 0 ? 'trigger-node' : steps[index - 1].id;
        svelteEdges.push({
          id: `e-${sourceId}-${step.id}`,
          source: sourceId,
          target: step.id
        });
      }
    });

    return svelteEdges;
  });

  import { BackgroundVariant } from '@xyflow/svelte';

  function onNodeDragStop(arg: any, maybeNode?: Node) {
    const node = arg?.targetNode ?? arg?.node ?? maybeNode ?? (arg?.id ? arg : null);
    if (node && node.id !== 'trigger-node' && node.position) {
      pipelineState.updateStepPosition(node.id, { x: node.position.x, y: node.position.y });
    }
  }

  function onConnect(connection: Connection) {
    if (!connection.source || !connection.target) return;
    if (connection.source === 'trigger-node') return; // Don't track trigger in depends_on

    const targetStep = pipelineState.steps.find(s => s.id === connection.target);
    if (targetStep) {
      const currentDeps = targetStep.depends_on || [];
      if (!currentDeps.includes(connection.source)) {
        pipelineState.updateDependsOn(targetStep.id, [...currentDeps, connection.source]);
      }
    }
  }

  function onNodeClick(arg: any, maybeNode?: Node) {
    const node = arg?.node ?? maybeNode ?? (arg?.id ? arg : null);
    if (node && node.id !== 'trigger-node') {
      pipelineState.selectStep(node.id);
    } else {
      pipelineState.selectStep(null);
    }
  }

  function onPaneClick() {
    pipelineState.selectStep(null);
  }

</script>

<div class="pipeline-canvas-container" id="pipeline-canvas">
  <SvelteFlow
    {nodes}
    {edges}
    {nodeTypes}
    fitView
    colorMode="dark"
    onnodedragstop={onNodeDragStop}
    onconnect={onConnect}
    onnodeclick={onNodeClick}
    onpaneclick={onPaneClick}
  >
    <Background variant={BackgroundVariant.Dots} gap={16} size={1} color="#334155" />
    <Panel position="top-right">
      <AutoLayoutButton />
    </Panel>
    <Controls
      buttonBgColor="#1e293b"
      buttonBgColorHover="#334155"
      buttonColor="#f8fafc"
      buttonColorHover="#60a5fa"
      buttonBorderColor="#334155"
    />
    <MiniMap
      bgColor="#0f172a"
      nodeColor="#334155"
      nodeStrokeColor="#475569"
      maskColor="rgba(15, 23, 42, 0.75)"
      maskStrokeColor="#3b82f6"
    />
  </SvelteFlow>
</div>

<style>
.pipeline-canvas-container {
  width: 100%;
  height: 100%;
  min-height: 400px;
  background: var(--bg-secondary, #0f172a);
  border-bottom: 1px solid var(--border-color, #334155);
  flex-grow: 1;
  position: relative;
}

:global(.svelte-flow) {
  --xy-controls-button-background-color: var(--bg-surface, #1e293b);
  --xy-controls-button-background-color-hover: var(--bg-secondary, #334155);
  --xy-controls-button-color: var(--text-primary, #f8fafc);
  --xy-controls-button-color-hover: var(--accent-primary, #60a5fa);
  --xy-controls-button-border-color: var(--border-color, #334155);
  --xy-edge-stroke-default: #475569;
  --xy-edge-stroke-selected-default: #3b82f6;
  --xy-handle-background-color: #60a5fa;
  --xy-handle-border-color: #0f172a;
}

:global(.svelte-flow__controls) {
  background: var(--bg-surface, #1e293b) !important;
  border: 1px solid var(--border-color, #334155) !important;
  border-radius: var(--border-radius-md, 8px) !important;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.45) !important;
  overflow: hidden !important;
}

:global(.svelte-flow__controls-button) {
  background: var(--bg-surface, #1e293b) !important;
  border-bottom: 1px solid var(--border-color, #334155) !important;
  color: var(--text-primary, #f8fafc) !important;
  fill: var(--text-primary, #f8fafc) !important;
  display: flex !important;
  align-items: center !important;
  justify-content: center !important;
  width: 32px !important;
  height: 32px !important;
  padding: 6px !important;
  transition: all 0.15s ease !important;
}

:global(.svelte-flow__controls-button:last-child) {
  border-bottom: none !important;
}

:global(.svelte-flow__controls-button:hover) {
  background: var(--bg-secondary, #334155) !important;
  color: var(--accent-primary, #60a5fa) !important;
  fill: var(--accent-primary, #60a5fa) !important;
}

:global(.svelte-flow__controls-button svg) {
  width: 14px !important;
  height: 14px !important;
  fill: currentColor !important;
  stroke: currentColor !important;
}

:global(.svelte-flow__minimap) {
  background: var(--bg-surface, #0f172a) !important;
  border: 1px solid var(--border-color, #334155) !important;
  border-radius: var(--border-radius-md, 8px) !important;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.45) !important;
  overflow: hidden !important;
}

:global(.svelte-flow__minimap-mask) {
  fill: rgba(15, 23, 42, 0.75) !important;
  stroke: var(--accent-primary, #3b82f6) !important;
  stroke-width: 1.5px !important;
}

:global(.svelte-flow__minimap-node) {
  fill: var(--bg-surface-elevated, #334155) !important;
  stroke: var(--border-color, #475569) !important;
}
</style>
