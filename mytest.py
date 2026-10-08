import urllib.request
import time
start = time.time()
while time.time() - start < 15:
    try:
        with urllib.request.urlopen("http://127.0.0.1:7823/health", timeout=1) as resp:
            if resp.status == 200:
                print("OK")
                break
    except Exception as e:
        time.sleep(0.3)
