---
title: CLI Guide
description: Headless CLI usage, stdin/stdout piping, and automated scripting.
---

# CLI Guide

The `paperpilot-cli` is a powerful tool for automating PDF workflows.

## Basic Usage

The CLI follows a standard command structure:

```bash
paperpilot-cli <command> [options]
```

## Piping

You can pipe data in and out of the CLI for seamless integration with other tools.

```bash
cat input.pdf | paperpilot-cli extract --pages 1 | paperpilot-cli hash
```

## Automated Scripting

Use the `--json` flag to receive structured JSON output, ideal for parsing in bash scripts or other automation pipelines.
