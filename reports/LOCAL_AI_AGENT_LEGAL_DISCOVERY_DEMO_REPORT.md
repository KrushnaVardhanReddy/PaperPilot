./scripts/run_ai_legal_demo.sh
[0;34m========================================================================[0m
[0;32m  PaperPilot: Local AI Agent Legal Due Diligence & Discovery Demo [0m
[0;34m========================================================================[0m

[1;33m>> [1/8] Skipping compilation (binaries already exist)...[0m
[1;33m>> [2/8] Generating mock legal document fixtures (if missing)...[0m
Fixtures generated successfully at demo/legal_discovery/input
   - 01_asset_purchase_agreement.pdf
   - 02_disclosure_schedules_tilted.pdf
   - 03_unredacted_financial_exhibit.pdf

[0;34m>> Starting AI Agent CLI/MCP pipeline...[0m

[1;33m>> 🔄 [3/8] Step 1: Correcting page orientation (rotate 90 deg cw)...[0m
   $ ./target/debug/paperpilot rotate --input demo/legal_discovery/input/02_disclosure_schedules_tilted.pdf --pages 2 --degrees 270 --output demo/legal_discovery/tmp/02_rotated.pdf
[0;32m   ✓ Success (106 ms)[0m

[1;33m>> ✂️ [4/8] Step 2: Redacting sensitive PII (SSN & Routing)...[0m
   $ ./target/debug/paperpilot redact --input demo/legal_discovery/input/01_asset_purchase_agreement.pdf --pages 1 --rect "100,500,300,520" --output demo/legal_discovery/tmp/01_redacted.pdf
[0;32m   ✓ Success (27 ms)[0m

[1;33m>> 🏷️ [5/8] Step 3: Applying courtroom Bates identifiers...[0m
   $ ./target/debug/paperpilot bates --input demo/legal_discovery/tmp/01_redacted.pdf --prefix "DISCOVERY-2026-" --start 1 --output demo/legal_discovery/tmp/01_bates.pdf && ./target/debug/paperpilot bates --input demo/legal_discovery/tmp/02_rotated.pdf --prefix "DISCOVERY-2026-" --start 5 --output demo/legal_discovery/tmp/02_bates.pdf && ./target/debug/paperpilot bates --input demo/legal_discovery/input/03_unredacted_financial_exhibit.pdf --prefix "DISCOVERY-2026-" --start 8 --output demo/legal_discovery/tmp/03_bates.pdf
[0;32m   ✓ Success (62 ms)[0m

[1;33m>> 📑 [6/8] Step 4: Assembling sanitized courtroom dossier...[0m
   $ ./target/debug/paperpilot merge --input demo/legal_discovery/tmp/01_bates.pdf demo/legal_discovery/tmp/02_bates.pdf demo/legal_discovery/tmp/03_bates.pdf --output demo/legal_discovery/tmp/master_merged.pdf
[0;32m   ✓ Success (29 ms)[0m

[1;33m>> 🛡️ [7/8] Step 5: Stamping legal privilege protective order...[0m
   $ ./target/debug/paperpilot watermark --input demo/legal_discovery/tmp/master_merged.pdf --text "CONFIDENTIAL - ATTORNEYS EYES ONLY" --output demo/legal_discovery/tmp/master_watermarked.pdf
[0;32m   ✓ Success (24 ms)[0m

[1;33m>> ⚡ [8/8] Step 6: Web-optimizing and hashing for audit trail...[0m
   $ ./target/debug/paperpilot linearize --input demo/legal_discovery/tmp/master_watermarked.pdf --output demo/legal_discovery/output/final_court_bundle.pdf
[0;32m   ✓ Success (37 ms)[0m

🔒 [1;33m>> Extracting Audit Hash...[0m
[0;32m   1582dcbceadf55b68cbc1dd10e4444cc18117feec270e984adae089a07101fb9[0m

[0;34m========================================================================[0m
[0;32m  PaperPilot Execution Scorecard [0m
[0;34m========================================================================[0m
  Total Inputs: 3 files (9 pages)
  Output File:  demo/legal_discovery/output/final_court_bundle.pdf
  Audit Hash:   1582dcbceadf55b68cbc1dd10e4444cc18117feec270e984adae089a07101fb9

  Performance Breakdown:
  - Rotate:       106 ms
  - Redact:        27 ms
  - Bates:         62 ms
  - Merge:         29 ms
  - Watermark:     24 ms
  - Linearize:     37 ms
  ------------------------
  [0;32mTotal Time:     285 ms[0m
[0;34m========================================================================[0m

[1;33mThe AI Agent tool pipeline ran 100% locally with zero cloud dependencies.[0m
See [0;32mdemo/legal_discovery/README.md[0m for documentation on triggering this via MCP.
