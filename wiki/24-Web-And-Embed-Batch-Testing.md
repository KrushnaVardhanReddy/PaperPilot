# Web and Embed Batch Testing Documentation

## Overview
This wiki entry documents the Playwright unit tests implemented for testing the newly added batch multi-file processing and interactive parameter configuration features in PaperPilot's Web application and Embed Widget.

## Test Components

### Embed Widget Tests (`apps/embed/tests/widget_batch_and_config.spec.ts`)
Validates the embed widget's encapsulation and UI behaviors using a sample host page (`host_page.html`).
- **Interactive Config Panel**: Ensures the panel is displayed properly inside the widget's shadow DOM when selecting a tool.
- **Rotation Tool**: Verifies that specific controls (angle dropdown, page input) appear correctly upon rotation tool selection.
- **State Reactivity**: Confirms that updating input elements reflects correctly in the DOM.
- **Batch File Upload**: Tests simulated multi-file drag-and-drop on the widget and verifies accurate processing visualization.

### Web Application Tests (`apps/web/tests/operations_batch_processing.spec.ts`)
Ensures proper batch handling directly within the main Web application's interface.
- **Multi-File Queueing**: Simulates appending multiple PDF files to operations and ensuring correct state management including removing entries.
- **Dynamic Action State**: Verifies the final action button updates its label dynamically reflecting the quantity of files present in the batch queue.
