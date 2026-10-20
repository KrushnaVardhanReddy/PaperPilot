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
