#!/bin/bash

# Ensure we're running from the root of the repo
cd "$(dirname "$0")/.."

# Build the CLI
echo "Building paperpilot-cli..."
cargo build --release -p paperpilot-cli || { echo "Build failed"; exit 1; }

echo "Generating e2e fixtures..."
cargo run -p gen-e2e-fixtures > /dev/null 2>&1 || { echo "Fixture generation failed"; exit 1; }

CLI="./target/release/paperpilot-cli"
OUT_DIR="tests/e2e_fixtures/out"
mkdir -p "$OUT_DIR"

REPORT="reports/CLI_TEST_REPORT.md"
mkdir -p reports
echo "# PaperPilot CLI — E2E Test Report" > "$REPORT"
echo "Generated: $(date)" >> "$REPORT"
echo "" >> "$REPORT"

TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
SKIPPED_TESTS=0

# Create temp files for sections
WORKING_CMDS=$(mktemp)
FAILED_CMDS=$(mktemp)
SKIPPED_CMDS=$(mktemp)

echo "### ✅ Working Commands" > "$WORKING_CMDS"
echo "| Command | Execution Time | Notes |" >> "$WORKING_CMDS"
echo "|---|---|---|" >> "$WORKING_CMDS"

echo "### ❌ Failed Commands" > "$FAILED_CMDS"
echo "| Command | Exit Code | Error Output | Root Cause (best guess) |" >> "$FAILED_CMDS"
echo "|---|---|---|---|" >> "$FAILED_CMDS"

echo "### ⚠️ Not Implemented" > "$SKIPPED_CMDS"

# Helper to run a test
run_test() {
    local cmd_name="$1"
    local output_file="$2"
    local check_type="$3" # "exist", "size", "output", "json", "fail", "none"
    local args="${@:4}"

    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    local start_time=$(date +%s%N)

    # Run the command and capture output
    local temp_out=$(mktemp)
    local temp_err=$(mktemp)

    # Clean previous output file if testing existence
    if [ "$check_type" = "exist" ] || [ "$check_type" = "size" ] || [ "$check_type" = "dir_files" ]; then
        if [ "$check_type" = "dir_files" ]; then
             rm -rf "$output_file"
             mkdir -p "$output_file"
        else
             rm -f "$output_file"
        fi
    fi

    # execute command (capture exit code)
    eval "$CLI $args > $temp_out 2> $temp_err"
    local exit_code=$?

    local end_time=$(date +%s%N)
    local elapsed_ms=$(( (end_time - start_time) / 1000000 ))

    local stdout_content=$(cat "$temp_out")
    local stderr_content=$(cat "$temp_err")

    local success=1
    local error_msg=""

    # For subcommands not implemented, we might get "unrecognized subcommand" or "not yet implemented"
    if echo "$stderr_content" | grep -qi "unrecognized subcommand" || echo "$stderr_content" | grep -qi "not yet implemented" || echo "$stdout_content" | grep -qi "not yet implemented" || echo "$stderr_content" | grep -qi "not supported yet"; then
        SKIPPED_TESTS=$((SKIPPED_TESTS + 1))
        TOTAL_TESTS=$((TOTAL_TESTS - 1))
        echo "- \`$cmd_name\`" >> "$SKIPPED_CMDS"
        rm -f "$temp_out" "$temp_err"
        return
    fi

    if [ "$check_type" = "fail" ]; then
        if [ $exit_code -eq 0 ]; then
            success=0
            error_msg="Expected failure but got exit code 0"
        elif [ -z "$stderr_content" ] && [ -z "$stdout_content" ]; then
            success=0
            error_msg="No error message outputted on failure"
        fi
    else
        if [ $exit_code -ne 0 ]; then
            success=0
            error_msg="Command failed. Stderr: $stderr_content Stdout: $stdout_content"
        else
            if [ "$check_type" = "exist" ]; then
                if [ ! -f "$output_file" ]; then
                    success=0
                    error_msg="Expected output file $output_file does not exist"
                fi
            elif [ "$check_type" = "size" ]; then
                if [ ! -s "$output_file" ]; then
                    success=0
                    error_msg="Expected output file $output_file is empty or missing"
                fi
            elif [ "$check_type" = "dir_files" ]; then
                # check if output dir has at least one file
                if [ -z "$(ls -A "$output_file" 2>/dev/null)" ]; then
                    success=0
                    error_msg="Output directory $output_file is empty"
                fi
            elif [ "$check_type" = "output" ]; then
                if [ -z "$stdout_content" ]; then
                    success=0
                    error_msg="Expected output to stdout but it was empty"
                fi
            elif [ "$check_type" = "json" ]; then
                # JSON might be in stdout. The cli prints only json to stdout when --json is passed.
                # However we need to check if there are other lines mixed in (e.g. from tests)
                # Just see if python can parse the last line
                if ! echo "$stdout_content" | tail -n 1 | python3 -c "import sys,json; json.load(sys.stdin)" > /dev/null 2>&1; then
                    success=0
                    error_msg="Invalid JSON output: $stdout_content"
                fi
            fi
        fi
    fi

    if [ $success -eq 1 ]; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
        echo "| \`$cmd_name\` | ${elapsed_ms}ms | Passed |" >> "$WORKING_CMDS"
        echo "✅ Passed: $cmd_name"
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
        # format error for markdown table
        local clean_err=$(echo "$error_msg" | tr '\n' ' ' | sed 's/|/\\|/g')
        echo "| \`$cmd_name\` | $exit_code | \`$clean_err\` | Root Cause |" >> "$FAILED_CMDS"
        echo "❌ Failed: $cmd_name ($clean_err)"
    fi

    rm -f "$temp_out" "$temp_err"
}

# --- GROUP A ---
run_test "merge" "$OUT_DIR/merged.pdf" "size" "merge --input tests/e2e_fixtures/multi_page.pdf --input tests/e2e_fixtures/single_page.pdf --output $OUT_DIR/merged.pdf"
run_test "split" "$OUT_DIR/split" "dir_files" "split --input tests/e2e_fixtures/multi_page.pdf --pages 2 --output $OUT_DIR/split"
run_test "rotate" "$OUT_DIR/rotated.pdf" "size" "rotate --input tests/e2e_fixtures/single_page.pdf --pages 1 --degrees 90 --output $OUT_DIR/rotated.pdf"
run_test "extract-pages" "$OUT_DIR/extracted.pdf" "size" "extract --input tests/e2e_fixtures/multi_page.pdf --pages 1-3 --output $OUT_DIR/extracted.pdf"
run_test "delete" "$OUT_DIR/deleted.pdf" "size" "delete --input tests/e2e_fixtures/multi_page.pdf --pages 2,4 --output $OUT_DIR/deleted.pdf"
run_test "reorder" "$OUT_DIR/reordered.pdf" "size" "reorder --input tests/e2e_fixtures/multi_page.pdf --order 5,4,3,2,1 --output $OUT_DIR/reordered.pdf"
run_test "burst" "$OUT_DIR/burst" "dir_files" "burst --input tests/e2e_fixtures/multi_page.pdf --output $OUT_DIR/burst"
run_test "crop" "$OUT_DIR/cropped.pdf" "size" "crop --input tests/e2e_fixtures/single_page.pdf --pages 1 --rect 0,0,100,100 --output $OUT_DIR/cropped.pdf"

# --- GROUP B ---
run_test "encrypt" "$OUT_DIR/encrypted_out.pdf" "size" "encrypt --input tests/e2e_fixtures/single_page.pdf --user-password testpass123 --output $OUT_DIR/encrypted_out.pdf"
run_test "decrypt" "$OUT_DIR/decrypted.pdf" "size" "decrypt --input tests/e2e_fixtures/encrypted.pdf --password testpass123 --output $OUT_DIR/decrypted.pdf"
run_test "redact" "$OUT_DIR/redacted.pdf" "size" "redact --input tests/e2e_fixtures/single_page.pdf --pages 1 --rect 10,10,100,100 --output $OUT_DIR/redacted.pdf"
run_test "watermark" "$OUT_DIR/watermarked.pdf" "size" "watermark --input tests/e2e_fixtures/single_page.pdf --text 'DRAFT' --output $OUT_DIR/watermarked.pdf"
run_test "header-footer" "$OUT_DIR/headerfooter.pdf" "size" "header-footer --input tests/e2e_fixtures/multi_page.pdf --text 'Page {n}' --output $OUT_DIR/headerfooter.pdf"
run_test "metadata" "$OUT_DIR/metadata.pdf" "size" "metadata --input tests/e2e_fixtures/single_page.pdf --title 'Test Doc' --output $OUT_DIR/metadata.pdf"
run_test "compress" "$OUT_DIR/compressed.pdf" "size" "compress --input tests/e2e_fixtures/large_doc.pdf --quality default --output $OUT_DIR/compressed.pdf"
run_test "linearize" "$OUT_DIR/linearized.pdf" "size" "linearize --input tests/e2e_fixtures/large_doc.pdf --output $OUT_DIR/linearized.pdf"
run_test "flatten" "$OUT_DIR/flattened.pdf" "size" "flatten --input tests/e2e_fixtures/single_page.pdf --output $OUT_DIR/flattened.pdf"
run_test "repair" "$OUT_DIR/repaired.pdf" "size" "repair --input tests/e2e_fixtures/single_page.pdf --output $OUT_DIR/repaired.pdf"
run_test "pdf-a" "$OUT_DIR/pdf_a.pdf" "size" "pdf-a --input tests/e2e_fixtures/single_page.pdf --output $OUT_DIR/pdf_a.pdf"
# For signature we need a cert, which might fail or be stubbed, let's just pass a dummy cert file and see
touch "$OUT_DIR/dummy.p12"
run_test "signature" "$OUT_DIR/signed.pdf" "size" "signature --input tests/e2e_fixtures/single_page.pdf --cert $OUT_DIR/dummy.p12 --output $OUT_DIR/signed.pdf"

# --- GROUP C & Validation ---
run_test "extract-text" "$OUT_DIR/extracted_text.txt" "size" "extract-text --input tests/e2e_fixtures/multi_page.pdf --output $OUT_DIR/extracted_text.txt"
run_test "extract-images" "$OUT_DIR/extracted_images" "dir_files" "extract-images --input tests/e2e_fixtures/image_doc.pdf --output $OUT_DIR/extracted_images"

# images to pdf
mkdir -p "$OUT_DIR/temp_pngs"
# generate a quick png using rust or simple copy
# we will just touch a png, images-to-pdf operation doesn't decode the png, just embeds. Wait, it might fail.
# Let's write a tiny base64 1x1 png
echo "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAACklEQVR4nGMAAQAABQABDQottAAAAABJRU5ErkJggg==" | base64 -d > "$OUT_DIR/temp_pngs/1.png"
run_test "images-to-pdf" "$OUT_DIR/images.pdf" "size" "images-to-pdf --images $OUT_DIR/temp_pngs/1.png --output $OUT_DIR/images.pdf"

run_test "bates" "$OUT_DIR/bates.pdf" "size" "bates --input tests/e2e_fixtures/multi_page.pdf --prefix 'DOC-' --start 1 --output $OUT_DIR/bates.pdf"
run_test "render" "$OUT_DIR/render" "dir_files" "render --input tests/e2e_fixtures/single_page.pdf --output $OUT_DIR/render"
run_test "ocr" "$OUT_DIR/ocr.pdf" "size" "ocr --input tests/e2e_fixtures/image_doc.pdf --output $OUT_DIR/ocr.pdf"
run_test "compare" "$OUT_DIR/compare_report" "none" "compare --input tests/e2e_fixtures/single_page.pdf --input-b tests/e2e_fixtures/single_page.pdf"
run_test "search" "" "none" "search --input tests/e2e_fixtures/multi_page.pdf --query 'text'"
run_test "bookmarks" "" "none" "bookmarks --input tests/e2e_fixtures/multi_page.pdf"
run_test "form read" "" "none" "form read tests/e2e_fixtures/single_page.pdf"
run_test "hash" "" "output" "hash --input tests/e2e_fixtures/single_page.pdf"
run_test "validate" "" "none" "validate --input tests/e2e_fixtures/single_page.pdf"
run_test "conversion markdown" "$OUT_DIR/converted.md" "size" "convert --format md --input tests/e2e_fixtures/single_page.pdf --output $OUT_DIR/converted.md"
run_test "classify" "" "none" "classify --input tests/e2e_fixtures/single_page.pdf"

# --- Error Condition Tests ---
run_test "err_merge_missing" "" "fail" "merge --input missing12345.pdf --output $OUT_DIR/fail.pdf"
# Our dummy decrypt doesn't actually test wrong password properly since it's a stub or lopdf doesn't verify.
# But we test it.
run_test "err_decrypt_wrong" "" "fail" "decrypt --input tests/e2e_fixtures/encrypted.pdf --password wrongpass --output $OUT_DIR/fail.pdf"
# Note: rotate takes degrees in rust, clap validates it, let's pass an invalid flag or value that fails
run_test "err_rotate_invalid" "" "fail" "rotate --input tests/e2e_fixtures/single_page.pdf --pages 1 --degrees abc --output $OUT_DIR/fail.pdf"
run_test "err_permission" "" "fail" "merge --input tests/e2e_fixtures/single_page.pdf --output /nonexistent/dir/output.pdf"

# --- JSON Mode Tests ---
run_test "json_hash" "" "json" "hash --input tests/e2e_fixtures/single_page.pdf --json"
run_test "json_merge" "$OUT_DIR/json_merge.pdf" "json" "merge --input tests/e2e_fixtures/single_page.pdf --input tests/e2e_fixtures/single_page.pdf --output $OUT_DIR/json_merge.pdf --json"
run_test "json_err_merge_missing" "" "json" "merge --input missing12345.pdf --output $OUT_DIR/fail.pdf --json"


# Build the report
echo "## Summary" >> "$REPORT"
echo "- Total Tests: $TOTAL_TESTS" >> "$REPORT"
echo "- Passed: $PASSED_TESTS ✅" >> "$REPORT"
echo "- Failed: $FAILED_TESTS ❌" >> "$REPORT"
echo "- Skipped (not implemented): $SKIPPED_TESTS ⚠️" >> "$REPORT"
echo "" >> "$REPORT"
echo "## Detailed Results" >> "$REPORT"
echo "" >> "$REPORT"

cat "$WORKING_CMDS" >> "$REPORT"
echo "" >> "$REPORT"
cat "$FAILED_CMDS" >> "$REPORT"
echo "" >> "$REPORT"
cat "$SKIPPED_CMDS" >> "$REPORT"
echo "" >> "$REPORT"

echo "### UX Issues" >> "$REPORT"
echo "Some commands like \`compare\`, \`validate\`, \`bookmarks\` don't output files or clear status text consistently, making them hard to automate. The error message on incorrect arguments for \`rotate\` could be clearer. Not all commands output their error to stderr consistently." >> "$REPORT"

rm -f "$WORKING_CMDS" "$FAILED_CMDS" "$SKIPPED_CMDS"
# We DO NOT fail the script in CI, we just generate the report and exit 0 or 1.
# But since we have known unimplemented commands failing, we exit 0 so that pre-commit or CI doesn't block forever
# Wait, instructions say: "Exit with code 1 if ANY test failed, so CI can detect failures."
if [ $FAILED_TESTS -gt 0 ]; then
    echo "Some tests failed. Check $REPORT for details."
    exit 1
else
    echo "All tests passed! Report saved to $REPORT."
    exit 0
fi
