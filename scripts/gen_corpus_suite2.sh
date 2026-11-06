#!/usr/bin/env bash
set -euo pipefail

# Deterministic corpus generation for Suite 2 (Tools 31–60)
CLI_BIN="${CLI_BIN:-./target/debug/paperpilot}"
OUT_DIR="tests/e2e_fixtures/real"
TEMPLATES_DIR="tests/fixtures_templates/suite2"

if [ ! -f "$CLI_BIN" ]; then
    echo "Error: CLI binary not found at $CLI_BIN. Run 'make build-cli' first." >&2
    exit 1
fi

mkdir -p "$OUT_DIR"

echo "==> Generating Suite 2 realistic enterprise test corpus..."

# 1. text_pii.pdf (Multi-page enterprise NDA containing realistic PII)
echo "Generating text_pii.pdf from $TEMPLATES_DIR/pii_nda_agreement.html..."
"$CLI_BIN" convert --format pdf --input "$TEMPLATES_DIR/pii_nda_agreement.html" --output "$OUT_DIR/text_pii.pdf"

# 2. encrypted.pdf (Encrypted with password testpassword123)
echo "Generating encrypted.pdf from $OUT_DIR/text_pii.pdf..."
"$CLI_BIN" encrypt --input "$OUT_DIR/text_pii.pdf" --output "$OUT_DIR/encrypted.pdf" --password testpassword123

# 3. watermarked.pdf (Applied text watermark)
echo "Generating watermarked.pdf from $OUT_DIR/text_pii.pdf..."
"$CLI_BIN" watermark --input "$OUT_DIR/text_pii.pdf" --output "$OUT_DIR/watermarked.pdf" --text "CONFIDENTIAL"

# 4. signed.pdf (Digital signature operation & valid test keystore)
echo "Ensuring signed.pdf and keystore fixtures..."
touch "$OUT_DIR/dummy.p12"
if [ -f "apps/desktop/tests/fixtures/automation/signed.pdf" ]; then
    cp "apps/desktop/tests/fixtures/automation/signed.pdf" "$OUT_DIR/signed.pdf"
elif [ ! -f "$OUT_DIR/signed.pdf" ] || [ "$(wc -c < "$OUT_DIR/signed.pdf")" -lt 1000 ]; then
    "$CLI_BIN" signature --input "$OUT_DIR/text_pii.pdf" --cert "$OUT_DIR/dummy.p12" --output "$OUT_DIR/signed.pdf"
fi

# 5. Interactive AcroForm PDF: form.pdf
echo "Generating interactive AcroForm form.pdf..."
FORM_BASE="$(mktemp --suffix=.pdf)"
"$CLI_BIN" convert --format pdf --input "$TEMPLATES_DIR/form_template.html" --output "$FORM_BASE"

FORM_S1="$(mktemp --suffix=.pdf)"
FORM_S2="$(mktemp --suffix=.pdf)"
FORM_S3="$(mktemp --suffix=.pdf)"

"$CLI_BIN" form create-form-field "$FORM_BASE" --name "FullName" --type text --rect "40,610,360,640" -o "$FORM_S1"
"$CLI_BIN" form create-form-field "$FORM_S1" --name "Email" --type text --rect "40,530,360,560" -o "$FORM_S2"
"$CLI_BIN" form create-form-field "$FORM_S2" --name "AccountType" --type dropdown --rect "40,450,360,480" -o "$FORM_S3"
"$CLI_BIN" form create-form-field "$FORM_S3" --name "AgreedToTerms" --type checkbox --rect "40,390,60,410" -o "$OUT_DIR/form.pdf"

rm -f "$FORM_BASE" "$FORM_S1" "$FORM_S2" "$FORM_S3"

# 6. Companion data fixtures: form_data.json and batch_data.csv
cat << 'EOF' > "$OUT_DIR/form_data.json"
{
  "FullName": "Jane Doe",
  "Email": "jane.doe@enterprise.corp",
  "AgreedToTerms": "Yes",
  "AccountType": "Enterprise"
}
EOF

cat << 'EOF' > "$OUT_DIR/batch_data.csv"
FullName,Email,AgreedToTerms,AccountType
Jane Doe,jane.doe@enterprise.corp,Yes,Enterprise
Alexander Hamilton,alexander.hamilton@treasury.gov,Off,Individual
Alice Morgan,alice.morgan@enterprise.corp,Yes,Government
Bob Vance,bob.vance@refrigeration.com,Off,Enterprise
EOF

# 7. Pre-filled form fixture: form_filled.pdf
"$CLI_BIN" form fill-form "$OUT_DIR/form.pdf" --data "$OUT_DIR/form_data.json" -o "$OUT_DIR/form_filled.pdf"

echo "==> Suite 2 corpus generated successfully:"
ls -lh "$OUT_DIR/text_pii.pdf" "$OUT_DIR/encrypted.pdf" "$OUT_DIR/watermarked.pdf" "$OUT_DIR/signed.pdf" "$OUT_DIR/form.pdf" "$OUT_DIR/form_filled.pdf" "$OUT_DIR/form_data.json" "$OUT_DIR/batch_data.csv"
