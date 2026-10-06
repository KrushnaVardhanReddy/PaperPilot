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
