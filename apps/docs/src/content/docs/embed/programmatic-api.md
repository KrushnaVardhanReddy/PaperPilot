---
title: Programmatic API
description: JavaScript lifecycle methods and event callbacks for Embed.js.
---

# Programmatic API

For advanced integrations, you can interact with the widget programmatically using JavaScript.

## Lifecycle Methods

The `PaperPilot` global object provides methods to control the widget:

*   `PaperPilot.mount(elementId, options)`: Manually mount the widget.
*   `PaperPilot.unmount()`: Remove the widget and clean up resources.

## Event Callbacks

You can listen for events dispatched by the widget to react to completed operations:

```javascript
window.addEventListener('paperpilot:operationCompleted', (event) => {
  console.log('Operation finished:', event.detail.toolName);
  console.log('Result data:', event.detail.result);
});
```
