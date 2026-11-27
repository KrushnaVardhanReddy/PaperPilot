# Task 175: Implement In-Memory Parser Self-Healing

## Overview
Implement Spec 175 by equipping `paperpilot-pdf` with automatic, in-memory recovery heuristics for corrupted, truncated, or malformed PDF documents.

---

## Subtasks & Checklist

- [x] **Subtask 1: In-Memory Repair Heuristics (`paperpilot-pdf/src/operations/repair.rs`)**
  - [x] Implement garbage prefix stripping and header normalization.
  - [x] Implement xref reconstruction by scanning object tokens.
  - [x] Implement synthetic trailer and `%%EOF` generation.

- [x] **Subtask 2: Transparent Loader Integration (`paperpilot-pdf/src/document.rs`)**
  - [x] Catch initial parser failures in document loaders.
  - [x] Fallback to in-memory self-healing pass before returning parse error.
  - [x] Emit structured tracing events during recovery.

- [x] **Subtask 3: Self-Healing Test Suite (`paperpilot-pdf/tests/self_healing_test.rs`)**
  - [x] Test truncated EOF recovery.
  - [x] Test corrupted xref recovery.
  - [x] Verify zero regression across standard valid test fixtures.
