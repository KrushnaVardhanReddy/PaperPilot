import ast
import re

with open('scripts/test_tri_interface_e2e.py', 'r') as f:
    content = f.read()

# Let's fix the report headers by replacing it with simple string search/replace
# First find the existing block that writes the headers. It looks like:
# f.write("| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |\\n")
# f.write("|---|---|---|---|---|---|---|\\n")

old_headers = '| Tool | Interface | Command / Invocation | Latency | Expected Result | Actual / Received Result | Verdict |'
new_headers = '| Tool | Interface | Data Sent | Specific Assertion Checked | Expected Result | Actual / Received Result | Verdict |'

content = content.replace(old_headers, new_headers)

with open('scripts/test_tri_interface_e2e.py', 'w') as f:
    f.write(content)
