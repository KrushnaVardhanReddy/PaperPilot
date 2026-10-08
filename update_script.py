import ast
import re

with open('scripts/test_tri_interface_e2e.py', 'r') as f:
    content = f.read()

# Fix gateway server start
new_start = '''    def start(self):
        print("Starting paperpilot-gateway...")
        if not os.path.exists(CLI_BIN):
            subprocess.run(["cargo", "build", "-p", "paperpilot-cli"], check=True)

        self.process = subprocess.Popen(
            [CLI_BIN, "serve", "--port", "7823"],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL
        )

        start_wait = time.time()
        while time.time() - start_wait < 15:
            try:
                with urllib.request.urlopen("http://127.0.0.1:7823/health", timeout=1) as resp:
                    if resp.status == 200:
                        print("Gateway server is healthy and responding on :7823.")
                        return
            except Exception:
                time.sleep(0.3)
        raise RuntimeError("Gateway server failed to start on port 7823 within 15 seconds")'''

content = re.sub(
    r'    def start\(self\):\n        print\("Starting paperpilot-gateway..."\)\n        self\.process = subprocess\.Popen\(\n            \[CLI_BIN, "serve", "--port", "7823"\],\n            stdout=subprocess\.DEVNULL,\n            stderr=subprocess\.DEVNULL\n        \)\n        time\.sleep\(2\)',
    new_start,
    content
)

# Update OUT_DIR
content = re.sub(
    r'OUT_DIR = os\.path\.join\(FIXTURE_DIR, "out", "tri_e2e"\)',
    'OUT_DIR = os.path.abspath(os.path.join(FIXTURE_DIR, "out", "tri_e2e"))\nos.makedirs(OUT_DIR, exist_ok=True)',
    content
)

with open('scripts/test_tri_interface_e2e.py', 'w') as f:
    f.write(content)
