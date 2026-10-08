import ast
import re

with open('scripts/test_tri_interface_e2e.py', 'r') as f:
    content = f.read()

# Fix the report structure as the previous regex replace might have missed the actual table formatting.
new_table_headers = '''        table_headers = ["Tool", "Interface", "Data Sent", "Specific Assertion Checked", "Expected Result", "Actual / Received Result", "Verdict"]
        f.write("| " + " | ".join(table_headers) + " |\\n")
        f.write("|" + "|".join(["---"] * len(table_headers)) + "|\\n")'''

content = re.sub(
    r'        table_headers = \["Tool", "Interface", "Command / Invocation", "Latency", "Expected Result", "Actual / Received Result", "Verdict"\]\n        f\.write\("\| " \+ " \| "\.join\(table_headers\) \+ " \|\\n"\)\n        f\.write\("\|" \+ "\|"\.join\(\["---"\] \* len\(table_headers\)\) \+ "\|\\n"\)',
    new_table_headers,
    content
)

new_write_cli = '''                f.write(f"| `{tool_name}` | {iface_map[iface_name]} | {get_data_sent(r)} | {get_specific_assertion(r, tool_name)} | {expected} | {get_actual(r)} | {get_verdict(r)} |\\n")'''
content = re.sub(
    r'                f\.write\(f"\| `\{tool_name\}` \| \{iface_map\[iface_name\]\} \| `\{format_cmd\(r\.get\(\'command\', \'\'\)\)\}` \| `\{r\.get\(\'latency\', 0\):\.2f\} ms` \| \{expected\} \| \{get_actual\(r\)\} \| \{get_verdict\(r\)\} \|\\n"\)',
    new_write_cli,
    content
)

new_write_wasm = '''            if wasm:
                f.write(f"| `{tool_name}` | {iface_map['WASM']} | {get_data_sent(wasm)} | {get_specific_assertion(wasm, tool_name)} | {expected} | {get_actual(wasm)} | {get_verdict(wasm)} |\\n")'''
content = re.sub(
    r'            if wasm:\n                f\.write\(f"\| `\{tool_name\}` \| \{iface_map\[\'WASM\'\]\} \| `\{format_cmd\(wasm\.get\(\'command\', \'\'\)\)\}` \| `\{wasm\.get\(\'latency\', 0\):\.2f\} ms` \| \{expected\} \| \{get_actual\(wasm\)\} \| \{get_verdict\(wasm\)\} \|\\n"\)',
    new_write_wasm,
    content
)

new_write_edge = '''            if edge:
                f.write(f"| `{tool_name}` | {iface_map['Edge']} | {get_data_sent(edge)} | {get_specific_assertion(edge, tool_name)} | {expected} | {get_actual(edge)} | {get_verdict(edge)} |\\n")'''
content = re.sub(
    r'            if edge:\n                f\.write\(f"\| `\{tool_name\}` \| \{iface_map\[\'Edge\'\]\} \| `\{format_cmd\(edge\.get\(\'command\', \'\'\)\)\}` \| `\{edge\.get\(\'latency\', 0\):\.2f\} ms` \| \{expected\} \| \{get_actual\(edge\)\} \| \{get_verdict\(edge\)\} \|\\n"\)',
    new_write_edge,
    content
)

with open('scripts/test_tri_interface_e2e.py', 'w') as f:
    f.write(content)
