<script lang="ts">
  import { SvelteFlow, Background, Controls, MiniMap, type Node, type Edge, type Connection } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
  import { pipelineState } from '$lib/state/pipeline.svelte';
  import ActionNode from './nodes/ActionNode.svelte';
  import ConditionNode from './nodes/ConditionNode.svelte';
  import TriggerNode from './nodes/TriggerNode.svelte';

  const nodeTypes = {
    action: ActionNode,
    condition: ConditionNode,
    trigger: TriggerNode
  };

  // Create derived nodes and edges from pipelineState.steps
  let nodes = $derived.by(() => {
    const svelteNodes: Node[] = [];

    // Add Trigger Node if we have steps (or even if we don't)
    svelteNodes.push({
      id: 'trigger-node',
      type: 'trigger',
      position: { x: 50, y: 200 },
      data: { label: 'Input Document', icon: '📄' }
    });

    pipelineState.steps.forEach((step) => {
      const isCondition = !!step.condition;
      svelteNodes.push({
        id: step.id,
        type: isCondition ? 'condition' : 'action',
        position: step.position || { x: 0, y: 0 },
        data: step as unknown as Record<string, unknown>
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

  function onNodeDragStop(event: any, node: Node) {
    if (node && node.id !== 'trigger-node') {
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

  function onNodeClick(event: any, node: Node) {
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
    onnodedragstop={onNodeDragStop}
    onconnect={onConnect}
    onnodeclick={onNodeClick}
    onpaneclick={onPaneClick}
  >
    <Background variant={BackgroundVariant.Dots} gap={16} size={1} />
    <Controls />
    <MiniMap />
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
}

:global(.svelte-flow__node) {
  /* Ensure nodes don't have default unstyled background */
}
</style>
