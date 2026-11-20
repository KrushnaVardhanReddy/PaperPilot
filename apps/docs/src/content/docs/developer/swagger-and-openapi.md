---
title: Swagger and OpenAPI
description: Accessing the local Swagger UI and OpenAPI specifications.
---

# Swagger and OpenAPI

PaperPilot includes an embedded Swagger UI for interactive exploration of the REST API.

## Accessing Swagger UI

When running the PaperPilot Gateway locally, navigate your browser to:

```text
http://127.0.0.1:7823/swagger-ui
```

This interface allows you to view all available endpoints, required parameters, and even test requests directly from the browser.

## Downloading the OpenAPI Spec

If you need the raw JSON specification (e.g., for generating client libraries or integrating with API gateways), you can download it from:

```text
http://127.0.0.1:7823/api-docs/openapi.json
```

## Route Namespacing & Aliases (v0.8.0+)

Starting in **v0.8.0**, PaperPilot separates endpoints into semantic categories:
- `/api/v1/pdf/*` — Pure PDF manipulation and rendering engines.
- `/api/v1/image/*` — Standalone image intelligence tools (`/deskew`, `/crop-to-content`, `/redact`, `/optimize-for-llm`, `/convert`, `/to-gif`).
- `/api/v1/office/*` — OpenXML document parsers and modernizers (`/word-to-markdown`, `/extract-text`).

> [!NOTE]
> **Backwards Compatibility**: For existing integrations, legacy `/api/v1/pdf/img-*` and `/api/v1/pdf/office-*` routes are maintained as active aliases. All new integrations should target the semantic `/image/*` and `/office/*` routes.

