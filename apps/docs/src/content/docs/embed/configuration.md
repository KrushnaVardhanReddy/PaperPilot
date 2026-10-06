---
title: Configuration
description: Customize the Embed.js widget using data-* attributes.
---

# Configuration

You can customize the appearance and behavior of the embedded widget using `data-*` attributes on the target `<div>`.

## Data Attributes Table

| Attribute | Description | Example |
| :--- | :--- | :--- |
| `data-tools` | Comma-separated list of tools to enable. | `data-tools="merge,split"` |
| `data-theme` | The visual theme (`light` or `dark`). | `data-theme="dark"` |
| `data-brand-color` | Primary brand color hex code. | `data-brand-color="#4F46E5"` |
| `data-hide-badge` | Hide the "Powered by PaperPilot" badge. | `data-hide-badge="true"` |
| `data-logo-url` | URL to a custom logo image. | `data-logo-url="/my-logo.png"` |

## Example

```html
<div id="paperpilot-portal"
     data-theme="dark"
     data-brand-color="#3b82f6"
     data-tools="pdf_merge,pdf_split">
</div>
```
