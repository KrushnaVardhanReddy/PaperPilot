import ast
import re

with open('scripts/test_tri_interface_e2e.py', 'r') as f:
    content = f.read()

# Change the report columns to match EXACTLY what's requested
new_report_format = '''        table_headers = ["Tool", "Interface", "Data Sent", "Specific Assertion Checked", "Expected Result", "Actual / Received Result", "Verdict"]
        f.write("| " + " | ".join(table_headers) + " |\\n")
        f.write("|" + "|".join(["---"] * len(table_headers)) + "|\\n")

        # Map interfaces for styling
        iface_map = {
            "CLI": "**💻 CLI**",
            "MCP": "**🤖 MCP**",
            "API": "**🌐 REST API**",
            "WASM": "**⚡ WASM (Browser)**",
            "Edge": "**☁️ Cloudflare Edge**"
        }

        for cli, mcp, api, wasm, edge in zip(cli_results, mcp_results, api_results, wasm_results, edge_results):
            def get_verdict(res):
                if not res.get('passed'):
                    return "❌ FAIL"
                return "✅ PASS"

            def get_actual(res):
                if not res.get('passed'):
                    return str(res.get('error', 'Error'))
                return str(res.get('actual_summary', 'Success'))

            def get_data_sent(res):
                cmd = res.get('command', '')
                if 'merge_a.pdf' in cmd: return '`merge_a.pdf` ("AAA") + `merge_b.pdf` ("BBB")'
                if 'multi_page.pdf' in cmd: return '`multi_page.pdf`'
                if 'invoice_text.pdf' in cmd: return '`invoice_text.pdf`'
                if 'with_blank.pdf' in cmd: return '`with_blank.pdf`'
                if 'redact_test.pdf' in cmd: return '`redact_test.pdf`'
                if 'form.pdf' in cmd: return '`form.pdf`'
                if 'encrypted.pdf' in cmd: return '`encrypted.pdf`'
                if 'broken_xref.pdf' in cmd: return '`broken_xref.pdf`'
                if 'img_sample.png' in cmd: return '`img_sample.png`'
                if 'test.csv' in cmd: return '`test.csv`'
                if 'test.html' in cmd: return '`test.html`'
                if 'test.md' in cmd: return '`test.md`'
                return "Default Input"

            def get_specific_assertion(res, tool_name):
                # Basic mapping from tool to specific assertion
                if tool_name == "pdf_merge": return "P1 text == 'AAA', P2 text == 'BBB'"
                if tool_name == "pdf_split": return "F1 text == 'P1', F2 text == 'P2'"
                if tool_name == "pdf_extract_pages": return "P1 == 'P1', P2 == 'P3'"
                if tool_name == "pdf_delete_pages": return "P2 and P4 absent"
                if tool_name == "pdf_reorder_pages": return "Order is P2, P1, P3, P4, P5"
                if tool_name == "pdf_rotate": return "/Rotate 90 and text intact"
                if tool_name == "pdf_crop": return "CropBox/MediaBox matches rect"
                if tool_name == "pdf_burst": return "5 files produced"
                if tool_name == "pdf_remove_blank": return "Blanks removed, 3-4 pages remain"
                if tool_name == "pdf_compress": return "Size reduced or bounded, text intact"
                if tool_name == "pdf_repair": return "Parses with pypdf without error"
                if tool_name == "pdf_linearize": return "Output contains /Linearized"
                if tool_name == "pdf_encrypt": return "Cannot read without password"
                if tool_name == "pdf_decrypt": return "Can read without password, text intact"
                if tool_name == "pdf_redact": return "Secret string absent in text and raw"
                if tool_name == "pdf_sign": return "Contains /Sig and /ByteRange"
                if tool_name == "pdf_hash": return "Hash matches expected SHA-256"
                if tool_name == "pdf_watermark": return "CONFIDENTIAL on every page"
                if tool_name == "pdf_header_footer": return "Header and footer text present"
                if tool_name == "pdf_bates": return "BATES-000001 and BATES-000005 present"
                if tool_name == "pdf_page_numbers": return "1/5 and 5/5 present"
                if tool_name == "pdf_annotate": return "/Annots and comment present"
                if tool_name == "pdf_read_form": return "Field first_name returned"
                if tool_name == "pdf_fill_form": return "Field first_name has expected value"
                if tool_name == "pdf_flatten": return "No fields returned, text visible"
                if tool_name == "pdf_create_form_field": return "New field exists"
                if tool_name == "pdf_extract_text": return "Expected strings present"
                if tool_name == "pdf_extract_images": return "Valid images extracted"
                if tool_name == "pdf_search": return "Hit coordinates and counts accurate"
                if tool_name == "pdf_render": return "Valid PNG image created"
                if tool_name == "pdf_compare": return "Diff string returned"
                if tool_name == "pdf_metadata": return "Metadata correct"
                if tool_name == "pdf_bookmarks": return "Bookmarks tree valid"
                if tool_name == "pdf_classify_type": return "Classified as text"
                if tool_name == "pdf_validate": return "Returns valid == true"
                if tool_name == "pdf_ocr": return "Output has text layer"
                if tool_name == "pdf_images_to_pdf": return "Valid PDF created"
                if tool_name == "pdf_to_pdf_a": return "Contains PDF/A XMP"
                if tool_name == "pdf_to_docx": return "Valid DOCX with text"
                if tool_name == "pdf_to_xlsx": return "Valid XLSX"
                if tool_name == "pdf_to_pptx": return "Valid PPTX"
                if tool_name == "pdf_convert_html": return "Valid PDF from HTML"
                if tool_name == "pdf_convert_markdown": return "Valid PDF from MD"
                if tool_name == "pdf_convert_excel": return "Valid PDF from CSV"
                return "Output valid"

            expected = cli.get('expected_desc', 'Success')

            # Rows for CLI, MCP, API
            for r, iface_name in [(cli, "CLI"), (mcp, "MCP"), (api, "API")]:
                f.write(f"| `{tool_name}` | {iface_map[iface_name]} | {get_data_sent(r)} | {get_specific_assertion(r, tool_name)} | {expected} | {get_actual(r)} | {get_verdict(r)} |\\n")

            if wasm:
                f.write(f"| `{tool_name}` | {iface_map['WASM']} | {get_data_sent(wasm)} | {get_specific_assertion(wasm, tool_name)} | {expected} | {get_actual(wasm)} | {get_verdict(wasm)} |\\n")
            if edge:
                f.write(f"| `{tool_name}` | {iface_map['Edge']} | {get_data_sent(edge)} | {get_specific_assertion(edge, tool_name)} | {expected} | {get_actual(edge)} | {get_verdict(edge)} |\\n")
'''

content = re.sub(
    r'        table_headers = \["Tool", "Interface", "Command / Invocation", "Latency", "Expected Result", "Actual / Received Result", "Verdict"\].*?            if edge:\n                f\.write\(f"\| `\{tool_name\}` \| \{iface_map\[\'Edge\'\]\} \| `\{format_cmd\(edge\.get\(\'command\', \'\'\)\)\}` \| `\{edge\.get\(\'latency\', 0\):\.2f\} ms` \| \{expected\} \| \{get_actual\(edge\)\} \| \{get_verdict\(edge\)\} \|\\n"\)',
    new_report_format,
    content,
    flags=re.DOTALL
)

with open('scripts/test_tri_interface_e2e.py', 'w') as f:
    f.write(content)
