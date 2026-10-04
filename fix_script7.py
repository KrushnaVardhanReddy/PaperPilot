import sys

with open("scripts/test_tri_interface_e2e.py", "r") as f:
    content = f.read()

# Replace stdio json-rpc initialization because rust MCP doesn't expect handshakes from stdin in direct mode.
# We fixed the broken pipe but we should make sure we read line carefully

content = content.replace('    def start(self):\n        print("Starting MCP server...")\n        self.process = subprocess.Popen([MCP_BIN], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)',
                          '    def start(self):\n        self.process = subprocess.Popen([MCP_BIN], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)')
content = content.replace("print(f\"  [MCP] {tool['name']}\")\n        mcp_res = mcp_client.call_tool(tool[\"name\"], mcp_args, req_id)",
                          "print(f\"  [MCP] {tool['name']}\")\n        mcp_res = mcp_client.call_tool(tool[\"name\"], mcp_args, req_id)\n")

with open("scripts/test_tri_interface_e2e.py", "w") as f:
    f.write(content)
