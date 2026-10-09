#!/bin/bash
set -e

# Colors for terminal output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================================================${NC}"
echo -e "${GREEN}  PaperPilot: Local AI Agent Legal Due Diligence & Discovery Demo ${NC}"
echo -e "${BLUE}========================================================================${NC}\n"

CLI="./target/debug/paperpilot"
if [ ! -f "$CLI" ]; then
    CLI="./target/debug/paperpilot-cli"
fi
FIXTURE_GEN="./target/debug/demo-legal-fixtures"

if [ ! -f "$CLI" ] || [ ! -f "$FIXTURE_GEN" ]; then
    echo -e "${YELLOW}>> [1/8] Compiling dependencies (PaperPilot CLI & Fixture Generator)...${NC}"
    cargo build -p paperpilot-cli -p demo-legal-fixtures
else
    echo -e "${YELLOW}>> [1/8] Skipping compilation (binaries already exist)...${NC}"
fi

INPUT_DIR="demo/legal_discovery/input"
TMP_DIR="demo/legal_discovery/tmp"
OUTPUT_DIR="demo/legal_discovery/output"

mkdir -p "$INPUT_DIR" "$TMP_DIR" "$OUTPUT_DIR"

echo -e "${YELLOW}>> [2/8] Generating mock legal document fixtures (if missing)...${NC}"
$FIXTURE_GEN "$INPUT_DIR"
echo -e "   - 01_asset_purchase_agreement.pdf"
echo -e "   - 02_disclosure_schedules_tilted.pdf"
echo -e "   - 03_unredacted_financial_exhibit.pdf\n"

echo -e "${BLUE}>> Starting AI Agent CLI/MCP pipeline...${NC}\n"

# Helper to run and measure time
run_step() {
    local step_msg="$1"
    local cmd="$2"

    echo -e "${YELLOW}>> $step_msg${NC}"
    echo -e "   $ ${cmd}"

    local start_ts=$(date +%s%3N)
    eval "$cmd" > /dev/null 2>&1
    local end_ts=$(date +%s%3N)

    local elapsed=$((end_ts - start_ts))
    echo -e "${GREEN}   ✓ Success (${elapsed} ms)${NC}\n"

    # Return time via global
    STEP_TIME=$elapsed
}

TOTAL_TIME=0

# Step 1: Rotate
run_step "🔄 [3/8] Step 1: Correcting page orientation (rotate 90 deg cw)..." "$CLI rotate --input $INPUT_DIR/02_disclosure_schedules_tilted.pdf --pages 2 --degrees 270 --output $TMP_DIR/02_rotated.pdf"
T1=$STEP_TIME
TOTAL_TIME=$((TOTAL_TIME + T1))

# Step 2: Redact
run_step "✂️ [4/8] Step 2: Redacting sensitive PII (SSN & Routing)..." "$CLI redact --input $INPUT_DIR/01_asset_purchase_agreement.pdf --pages 1 --rect \"100,500,300,520\" --output $TMP_DIR/01_redacted.pdf"
T2=$STEP_TIME
TOTAL_TIME=$((TOTAL_TIME + T2))

# Step 3: Bates Stamp
run_step "🏷️ [5/8] Step 3: Applying courtroom Bates identifiers..." "$CLI bates --input $TMP_DIR/01_redacted.pdf --prefix \"DISCOVERY-2026-\" --start 1 --output $TMP_DIR/01_bates.pdf && $CLI bates --input $TMP_DIR/02_rotated.pdf --prefix \"DISCOVERY-2026-\" --start 5 --output $TMP_DIR/02_bates.pdf && $CLI bates --input $INPUT_DIR/03_unredacted_financial_exhibit.pdf --prefix \"DISCOVERY-2026-\" --start 8 --output $TMP_DIR/03_bates.pdf"
T3=$STEP_TIME
TOTAL_TIME=$((TOTAL_TIME + T3))

# Step 4: Merge
run_step "📑 [6/8] Step 4: Assembling sanitized courtroom dossier..." "$CLI merge --input $TMP_DIR/01_bates.pdf $TMP_DIR/02_bates.pdf $TMP_DIR/03_bates.pdf --output $TMP_DIR/master_merged.pdf"
T4=$STEP_TIME
TOTAL_TIME=$((TOTAL_TIME + T4))

# Step 5: Watermark
run_step "🛡️ [7/8] Step 5: Stamping legal privilege protective order..." "$CLI watermark --input $TMP_DIR/master_merged.pdf --text \"CONFIDENTIAL - ATTORNEYS EYES ONLY\" --output $TMP_DIR/master_watermarked.pdf"
T5=$STEP_TIME
TOTAL_TIME=$((TOTAL_TIME + T5))

# Step 6: Linearize & Hash
run_step "⚡ [8/8] Step 6: Web-optimizing and hashing for audit trail..." "$CLI linearize --input $TMP_DIR/master_watermarked.pdf --output $OUTPUT_DIR/final_court_bundle.pdf"
T6=$STEP_TIME
TOTAL_TIME=$((TOTAL_TIME + T6))

echo -e "🔒 ${YELLOW}>> Extracting Audit Hash...${NC}"
AUDIT_HASH=$($CLI hash --input $OUTPUT_DIR/final_court_bundle.pdf)
echo -e "${GREEN}   $AUDIT_HASH${NC}\n"

echo -e "${BLUE}========================================================================${NC}"
echo -e "${GREEN}  PaperPilot Execution Scorecard ${NC}"
echo -e "${BLUE}========================================================================${NC}"
echo -e "  Total Inputs: 3 files (9 pages)"
echo -e "  Output File:  $OUTPUT_DIR/final_court_bundle.pdf"
echo -e "  Audit Hash:   $AUDIT_HASH"
echo -e ""
echo -e "  Performance Breakdown:"
printf "  - Rotate:      %4d ms\n" $T1
printf "  - Redact:      %4d ms\n" $T2
printf "  - Bates:       %4d ms\n" $T3
printf "  - Merge:       %4d ms\n" $T4
printf "  - Watermark:   %4d ms\n" $T5
printf "  - Linearize:   %4d ms\n" $T6
echo -e "  ------------------------"
printf "  ${GREEN}Total Time:    %4d ms${NC}\n" $TOTAL_TIME
echo -e "${BLUE}========================================================================${NC}\n"

echo -e "${YELLOW}The AI Agent tool pipeline ran 100% locally with zero cloud dependencies.${NC}"
echo -e "See ${GREEN}demo/legal_discovery/README.md${NC} for documentation on triggering this via MCP.\n"
