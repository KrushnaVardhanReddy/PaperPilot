# Spec 034 — Automated PDF Structural & Conformance Verification (`qpdf` & `veraPDF`)

## 1. Overview & Objectives
PaperPilot performs structural manipulation, repair, compression, linearization, and archiving across 45+ PDF operations. To guarantee that PDFs generated or processed by PaperPilot meet strict industry standards, we integrate automated external validation tools into our CI and test suites:

1. **`qpdf` Structural Integrity Verification**:
   - Verify that PDFs produced by `paperpilot` commands (such as `repair`, `linearize`, `compress`, `merge`, and `split`) pass strict structural validation without xref corruption or syntax warnings (`qpdf --check`).
2. **`veraPDF` ISO Conformance Verification**:
   - Verify that output produced by `pdf_a` meets ISO 19005 (PDF/A-1b) baseline profile requirements without critical specification failures.
3. **CI Pipeline Integration**:
   - Provide a zero-overhead, scriptable test runner `scripts/verify_pdf_conformance.sh` and a Makefile target `make test-conformance`.
   - Incorporate `qpdf` checks into `.github/workflows/rust.yml` Linux runner.

---

## 2. Architecture & Design

### 2.1 Conformance Runner Script (`scripts/verify_pdf_conformance.sh`)
A shell script that:
1. Detects available validation binaries (`qpdf` and optionally `verapdf`).
2. Generates test output files across key PaperPilot operations:
   - `paperpilot merge`
   - `paperpilot split`
   - `paperpilot repair`
   - `paperpilot linearize`
   - `paperpilot compress`
   - `paperpilot convert --format pdfa`
3. Runs structural audits:
   ```bash
   qpdf --check --warning-exit-0 "$output_pdf"
   ```
4. If `verapdf` is installed:
   ```bash
   verapdf --flavour 1b --format text "$pdfa_output"
   ```
5. Returns exit code 0 if all audited PDFs pass structural checks, or exit code 1 with actionable failure logs.

### 2.2 Makefile Integration
Add targets to `Makefile`:
```makefile
.PHONY: test-conformance

test-conformance:
	./scripts/verify_pdf_conformance.sh
```

### 2.3 GitHub Actions Workflow Integration (`.github/workflows/rust.yml`)
In `build-and-test` Linux runner:
1. Ensure `qpdf` is installed via `sudo apt-get install -y qpdf`.
2. Add step `Run Structural Conformance Audits (qpdf)` invoking `make test-conformance`.

---

## 3. Scope & Constraints
- **Zero Regression**: Must not break any existing tests or build commands.
- **Graceful Fallback**: If `verapdf` is not installed in the local environment, the script must report a clear informational notice and proceed with `qpdf` audits without failing.
- **Portability**: Must execute reliably on Ubuntu/Debian runners in GitHub Actions.

---

## 4. Verification Deliverables
1. `scripts/verify_pdf_conformance.sh` executable script.
2. `Makefile` updated with `test-conformance`.
3. `.github/workflows/rust.yml` updated with `qpdf` installation and test step.
4. Comprehensive verification report at `reports/PDF_CONFORMANCE_VERIFICATION_REPORT.md`.
