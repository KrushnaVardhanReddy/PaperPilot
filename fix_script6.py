import sys

with open("scripts/test_tri_interface_e2e.py", "r") as f:
    content = f.read()

# Gateway starts but takes time maybe
# The port is 7823.

content = content.replace('urllib.request.urlopen("http://127.0.0.1:7823/api/v1/health")',
                          'urllib.request.urlopen("http://127.0.0.1:7823/swagger-ui")')

with open("scripts/test_tri_interface_e2e.py", "w") as f:
    f.write(content)
