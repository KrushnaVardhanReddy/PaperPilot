import sys
with open("scripts/test_tri_interface_e2e.py", "r") as f:
    content = f.read()

# Make sure MCP client processes are correctly recreated each time and we don't have broken pipe error
# The original process was stopped and not restarted in call_tool
content = content.replace('def start(self):\n        self.process = subprocess.Popen([MCP_BIN], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)',
                          'def start(self):\n        if self.process:\n            try:\n                self.process.terminate()\n            except:\n                pass\n        self.process = subprocess.Popen([MCP_BIN], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)')
content = content.replace("    def call_tool(self, name, arguments, req_id=2):\n        self.start() # start fresh for each call",
                          "    def call_tool(self, name, arguments, req_id=2):\n        self.start() # start fresh for each call")

# If mcp_client.start() was removed from main we should add it back to be safe, but wait call_tool already does it.
content = content.replace("mcp_client.start()\n    \n    gateway", "gateway")

with open("scripts/test_tri_interface_e2e.py", "w") as f:
    f.write(content)
