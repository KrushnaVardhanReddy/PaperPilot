#!/usr/bin/env bash
set -euo pipefail

# Deterministic corpus generation for Suite 1 (Tools 01–30)
CLI_BIN="${CLI_BIN:-./target/debug/paperpilot}"
OUT_DIR="tests/e2e_fixtures/real"
TEMPLATES_DIR="tests/fixtures_templates/suite1"

if [ ! -f "$CLI_BIN" ]; then
    echo "Error: CLI binary not found at $CLI_BIN. Run 'make build-cli' first." >&2
    exit 1
fi

mkdir -p "$OUT_DIR"

echo "==> Generating Suite 1 realistic enterprise test corpus..."

# 1. multi_page.pdf (8 pages)
echo "Generating multi_page.pdf from $TEMPLATES_DIR/multi_page_contract.html..."
"$CLI_BIN" convert --format pdf --input "$TEMPLATES_DIR/multi_page_contract.html" --output "$OUT_DIR/multi_page.pdf"

# 2. single_page.pdf (1 page)
echo "Generating single_page.pdf from $TEMPLATES_DIR/single_page_doc.html..."
"$CLI_BIN" convert --format pdf --input "$TEMPLATES_DIR/single_page_doc.html" --output "$OUT_DIR/single_page.pdf"

# 3. merge_a.pdf (4 pages)
echo "Generating merge_a.pdf from $TEMPLATES_DIR/merge_doc_a.html..."
"$CLI_BIN" convert --format pdf --input "$TEMPLATES_DIR/merge_doc_a.html" --output "$OUT_DIR/merge_a.pdf"

# 4. merge_b.pdf (3 pages)
echo "Generating merge_b.pdf from $TEMPLATES_DIR/merge_doc_b.html..."
"$CLI_BIN" convert --format pdf --input "$TEMPLATES_DIR/merge_doc_b.html" --output "$OUT_DIR/merge_b.pdf"

# 5. test.png (rendered from single_page.pdf)
echo "Generating test.png from $OUT_DIR/single_page.pdf..."
"$CLI_BIN" render --input "$OUT_DIR/single_page.pdf" --output "$OUT_DIR/test.png"

echo "==> Suite 1 corpus generated successfully:"
ls -lh "$OUT_DIR/multi_page.pdf" "$OUT_DIR/single_page.pdf" "$OUT_DIR/merge_a.pdf" "$OUT_DIR/merge_b.pdf" "$OUT_DIR/test.png"
