---
title: Semantic Search Tool
description: Searches document chunks using semantic similarity (Cosine similarity).
---

# Semantic Search Tool

The semantic search tool queries the `paperpilot-storage` vector store database using cosine similarity to rank chunks.

## CLI Usage

Currently, semantic search is implemented natively in `paperpilot-storage` and is accessible internally via the `StorageEngine` API. E2E CLI exposure is handled in respective workflow tasks.

## MCP JSON-RPC

Provides internal endpoints via MCP for tool calling (integration pending in dependent layers).

## REST API

Internal `StorageEngine::search_chunks` serves as the backbone for HTTP endpoints.
