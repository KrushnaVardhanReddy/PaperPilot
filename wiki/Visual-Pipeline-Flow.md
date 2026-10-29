# Visual Pipeline Flow Management

The PaperPilot Visual Pipeline Builder now supports comprehensive flow management capabilities across the Desktop app and Embed Widget.

## Component Architecture

1. **`PipelineState`**: The central Svelte 5 state class (`apps/desktop/src/lib/state/pipeline.svelte.ts`) managing the step sequence.
   * `initStorage()`: Reads `localStorage` (`paperpilot_saved_flows`) to merge custom flows with predefined `DEFAULT_PRESETS`.
   * `saveCurrentFlow(name, desc)`: Clones the active `steps` and generates a persistent `SavedPipeline` UUID record.
   * `exportFlowJson()` / `importFlowFromJson()`: Serializes and deserializes the entire step definition array for sharing.

2. **`FlowSelector.svelte`**: A glassmorphic toolbar injected directly under the pipeline header.
   * Parses the active preset or determines if the user has drifted into a "Custom / Unsaved" flow via an `$effect` block that tracks `steps.length`.
   * Manages browser file APIs for JSON downloads.

3. **`SaveFlowModal.svelte`**: Follows the `AiSettingsModal.svelte` design language using Svelte transitions (`fade`, `slide`) and matching color tokens (`#0f172a` backdrop, blue focus rings).

4. **Widget Integration**: `apps/embed/src/Widget.svelte` intercepts `pipeline` properties. If the string exactly matches a preset ID (e.g. `preset-legal-discovery`), it resolves the preset's step operations internally, bypassing the need for duplicate JSON hardcoding on the host page.

## Data Structure

```typescript
export interface SavedPipeline {
  id: string;
  name: string;
  description?: string;
  isPreset: boolean;
  updatedAt: string;
  steps: PipelineStep[];
}
```

## Conditional Branching UI

PaperPilot supports multi-conditional workflow nodes via the visual desktop interface. Users can open the 'Condition Inspector' when an operation step is selected, allowing them to visually build rules to evaluate the document prior to executing that step.

### Supported Conditions
- **Only if Scanned Document:** Checks if the document is purely a scanned image PDF via OCR heuristic.
- **Page Count:** Comparator (`Greater`, `Less`, `Equal`) and numeric value.
- **Text Contains:** Simple sub-string matching logic.
- **Has Form Fields:** Checks the AcroForm dictionary for presence of fields.
- **Is Digitally Signed:** Evaluates the presence of an active signature block.

### Branching Behavior (On No Match)
When a condition evaluates to false, the step can route the document into three possible behaviors:
1. **Skip:** The default. Bypasses the operation and proceeds to the next step.
2. **Archive Original & Stop:** Preserves the incoming version of the file in the workspace history and terminates the flow.
3. **Fail Workflow:** Throws a validation error and aborts the entire execution batch.

This visual mapping automatically synchronizes with the `WorkflowConditionLeaf` schema representation, storing the definitions directly within the saved `.json` or `.toml` pipeline profiles.
