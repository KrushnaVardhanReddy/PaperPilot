import urllib.request
with urllib.request.urlopen("http://127.0.0.1:7823/health", timeout=1) as resp:
    print(resp.status)
