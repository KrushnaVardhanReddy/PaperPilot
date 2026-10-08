import ast
import re

with open('scripts/test_tri_interface_e2e.py', 'r') as f:
    content = f.read()

# Fix the report structure - replace "Command / Invocation" and "Latency" with the correct headers in the actual file.
# Looks like we previously replaced the table headers, but maybe the markdown headers for the table content are still wrong in the file.
new_table_headers = '''        table_headers = ["Tool", "Interface", "Data Sent", "Specific Assertion Checked", "Expected Result", "Actual / Received Result", "Verdict"]
        f.write("| " + " | ".join(table_headers) + " |\\n")
        f.write("|" + "|".join(["---"] * len(table_headers)) + "|\\n")'''

content = re.sub(
    r'        table_headers = \["Tool", "Interface", "Command / Invocation", "Latency", "Expected Result", "Actual / Received Result", "Verdict"\].*?f\.write\("\|" \+ "\|"\.join\(\["---"\] \* len\(table_headers\)\) \+ "\|\\n"\)',
    new_table_headers,
    content,
    flags=re.DOTALL
)

with open('scripts/test_tri_interface_e2e.py', 'w') as f:
    f.write(content)
