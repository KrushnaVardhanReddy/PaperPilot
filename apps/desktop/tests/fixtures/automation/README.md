# Desktop automation fixtures

These are complete synthetic PDFs with no user documents or private keys. `paperpilot-pdf/tests/desktop_automation.rs` produces `text.pdf`, `scan.pdf`, `multi.pdf`, `form.pdf`, and `unsigned.pdf`, then records real inspection results in `document-facts.json` and actual filesystem events in `watch-events.json`.

`signed.pdf` contains a detached CMS/PKCS#7 SHA-256 RSA signature over its actual PDF byte ranges, created using an ephemeral QA key and a self-signed public certificate (PaperPilot synthetic QA certificate). The private key was discarded. It exercises signature presence detection, not certificate trust or cryptographic validation in the smart filters. `unsigned.pdf` has signature flags but no filled signature.

The TypeScript view/state tests read these PDFs and the native runner's recorded facts/events. They do not replace PDF operation responses or native IPC with success stubs. Historical event paths point to a temporary test directory and are not expected to exist.

To refresh the native evidence (retain the signed fixture):

```powershell
$env:PAPERPILOT_DESKTOP_EVIDENCE_DIR = (Resolve-Path apps/desktop/tests/fixtures/automation).Path
cargo test -p paperpilot-pdf --test desktop_automation -j 2 -- --test-threads=2
```
