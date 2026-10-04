import sys

with open("scripts/test_tri_interface_e2e.py", "r") as f:
    content = f.read()

# Make sure start gateway is called correctly
content = content.replace("    gateway.start()\n", "    gateway.start()\n")
if "gateway.start()" not in content:
    print("WARNING gateway.start() missing")

with open("scripts/test_tri_interface_e2e.py", "w") as f:
    f.write(content)
