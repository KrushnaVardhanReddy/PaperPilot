# CI Pipeline

The PaperPilot project uses GitHub Actions for its Continuous Integration (CI) pipeline. This pipeline ensures code quality and prevents broken builds from being merged into the `main` branch.

## Workflow Triggers

The CI pipeline is triggered on:
- Pushes to the `main` branch.
- Pull Requests targeting the `main` branch.

## Checks Performed

The workflow consists of a single job that runs on an `ubuntu-latest` runner using the latest stable Rust toolchain. It performs the following checks for the Cargo workspace:

1.  **Formatting Check:**
    -   Command: `cargo fmt --all -- --check`
    -   Purpose: Ensures all Rust code adheres to the standard formatting guidelines. It will fail the build if any files are not formatted correctly.

2.  **Linting with Clippy:**
    -   Command: `cargo clippy --all-targets --all-features -- -D warnings`
    -   Purpose: Runs Clippy, the Rust linter, to catch common mistakes and improve code quality. The `-D warnings` flag treats all warnings as errors, causing the build to fail if any lints are triggered.

3.  **Unit Tests:**
    -   Command: `cargo test --all-targets --all-features`
    -   Purpose: Runs all tests across all targets and features in the workspace to verify the correctness of the code. The build fails if any tests fail.
