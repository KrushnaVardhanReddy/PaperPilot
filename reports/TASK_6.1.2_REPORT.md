# Task 6.1.2: Spec 160 — System Diagnostics Engine (`paperpilot doctor`)

## Overview
Implemented a comprehensive environment and system diagnostics module that can be triggered via CLI, Gateway REST API, or Desktop UI.

## Implementation Highlights
1. **Core Diagnostics**: Created `paperpilot-core/src/diagnostics.rs` to run real probes on the system (Temp dir, Gateway port, OCR model weights, Disk space).
2. **CLI**: Added the `paperpilot doctor` and `paperpilot doctor --json` subcommands.
3. **Gateway Endpoints**:
   - Modified `GET /health` to return a standard payload.
   - Added `GET /api/v1/doctor` to execute the full diagnostic report.
4. **Desktop UI**: Created `SystemDoctorModal.svelte` with a visual representation of system health, integrated into `SettingsPanel.svelte`.

## Validation
- Workspace successfully compiles (`cargo check -j 4`).
- All unit tests pass (`cargo test`).
- Vitest UI test suite passes (`pnpm test`).
- CLI tool verified and exported JSON (`reports/task-6.1.2-validation/verification.json`).
