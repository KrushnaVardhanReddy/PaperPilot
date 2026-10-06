---
title: Zero Cloud Privacy
description: Our commitment to zero telemetry and client-side processing.
---

# Zero Cloud Privacy

PaperPilot is designed with privacy as the foremost priority.

## Client-Side Processing

All PDF operations, OCR, and AI agent interactions happen **locally** on your device when using the Desktop app or Drop-In Widget. We use Rust and WebAssembly to ensure high performance (~50–100ms WASM Execution) without needing to send your sensitive documents to a server.

*Note: For the Cloudflare Workers API integration, documents are processed securely at the edge, so uploads to edge workers do occur in that specific mode.*

## Compliance & Security

Unlike Traditional Cloud SaaS, because your data never leaves your machine in desktop/widget modes, PaperPilot inherently supports your compliance requirements. There is no risk of data breaches in transit or on centralized storage servers.

## Zero Telemetry

We do not track your usage, collect analytics, or phone home. Your workflow is entirely your own.
