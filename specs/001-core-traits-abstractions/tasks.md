# Implementation Tasks: Core Traits and Abstractions

**Feature**: `specs/001-core-traits-abstractions/spec.md`

## Phase 1: Setup & Foundational
- [ ] T001 Add `thiserror` and `serde` dependencies to `paperpilot-core/Cargo.toml`.
- [ ] T002 Initialize `error.rs`, `traits.rs`, and `events.rs` modules in `paperpilot-core/src/lib.rs`.

## Phase 2: Strongly Typed Error Handling [US3]
- [ ] T003 [US3] Implement `PdfError` enum and `OperationResult` type alias in `paperpilot-core/src/error.rs`.
- [ ] T004 [US3] Write unit tests for `PdfError` variants in `paperpilot-core/src/error.rs`.

## Phase 3: Asynchronous Job Progress [US4]
- [ ] T005 [P] [US4] Define `JobProgress` struct (with serialization) in `paperpilot-core/src/events.rs`.
- [ ] T006 [P] [US4] Write unit tests for `JobProgress` in `paperpilot-core/src/events.rs`.

## Phase 4: Core Interfaces [US1] & [US2]
- [ ] T007 [US1] Define `PdfDocument` trait in `paperpilot-core/src/traits.rs`.
- [ ] T008 [US2] Define `PdfOperation` trait in `paperpilot-core/src/traits.rs`.
- [ ] T009 [US1] [US2] Write unit tests verifying trait object safety and dummy implementations in `paperpilot-core/src/traits.rs`.
