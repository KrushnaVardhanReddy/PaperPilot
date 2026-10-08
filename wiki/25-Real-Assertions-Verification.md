# 25 Real Assertions Verification

The Real Assertions suite replaces shallow "file exists" verification with independent, 3rd-party semantic assertions across all 44 PDF operations.

## Architecture & Principles
Instead of trusting the engine output merely because a file size > 0, the testing suite (`scripts/test_tri_interface_e2e.py`) now reads the output PDF with independent libraries (`pypdf`, `python-docx`, `openpyxl`, `python-pptx`) to verify that the operation actually succeeded.

## The 6 Global Rules

1. **Output Opens Independently**: Every generated PDF or converted file must parse cleanly with an independent library without throwing syntax or structural errors.
2. **Output Differs from Input**: For modification tools, the SHA-256 hash of the output must differ from the input.
3. **Input File Immutability**: The input file must not be modified in place. Its SHA-256 hash must remain identical.
4. **Normalized Content Parity**: Results across CLI, MCP StdIO, and REST API must yield structurally identical files and outputs.
5. **Security Realism**: Tools like redaction and encryption must pass strict inspection. E.g., redacted text MUST NOT be extractable from the output. Encrypted files MUST NOT open without the correct password.
6. **No Engine Mocking**: If an engine tool currently fails these strict semantics, the test runner fails that test rather than artificially passing it or modifying the engine.

## Execution
Run the verification using:
```bash
python3 scripts/test_tri_interface_e2e.py
```
Outputs are strictly logged to `reports/REAL_ASSERTIONS_TRI_INTERFACE_REPORT.md` and measure Pass/Fail outcomes for all 44 tools across 3 interfaces (132 tests total).
