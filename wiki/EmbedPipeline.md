# Embed Pipeline Widget Architecture

The PaperPilot Embed Widget (\`apps/embed\`) brings the full, zero-server WASM document pipeline into any web application via Shadow DOM encapsulation.

## Features
- **Dropzone & File Loading**: Users can securely drag and drop multiple PDF files or use the integrated browse dialog. All files are kept 100% locally.
- **Interactive Step Picker**: Allows appending actions such as Rotate, Watermark, Compress, Extract, and Split directly into the execution flow.
- **Flow Management Toolbar**: Supports saving custom tool sequences (stored locally in \`localStorage\`), managing predefined presets, exporting recipes as JSON, and importing JSON recipes.
- **Execution & Processing**: Provides explicit control to trigger pipeline execution on demand, generating optimized PDF output via the \`pdf-lib\` core.

## Configuration & Usage
To mount the widget manually, use:
\`\`\`ts
window.PaperPilot.mount('#target-element', {
    tools: 'rotate,watermark,compress',
    allowCustomization: true,
    autoExecute: false,
    pipeline: [
        { tool: 'watermark', text: 'APPROVED' }
    ]
});
\`\`\`

Alternatively, use auto-initialization via \`data-\` attributes on any portal element:
\`\`\`html
<div
  data-paperpilot-widget
  data-allow-customization="true"
  data-auto-execute="false"
  data-pipeline='[{"tool":"compress"}]'
></div>
\`\`\`
