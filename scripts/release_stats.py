#!/usr/bin/env python3
"""
Fetch PaperPilot ecosystem download statistics across GitHub Releases and npm.
"""

import sys
import json
import urllib.request
from datetime import datetime

GITHUB_URL = "https://api.github.com/repos/KrushnaVardhanReddy/PaperPilot/releases"
NPM_RANGE_URL = f"https://api.npmjs.org/downloads/range/2026-01-01:{datetime.now().strftime('%Y-%m-%d')}/@usepaperpilot/mcp"
NPM_LAST_7D_URL = "https://api.npmjs.org/downloads/point/last-week/@usepaperpilot/mcp"
NPM_LAST_30D_URL = "https://api.npmjs.org/downloads/point/last-month/@usepaperpilot/mcp"

headers = {"User-Agent": "PaperPilot-Stats"}

# 1. Fetch GitHub Releases
gh_total = 0
gh_breakdown = []
try:
    req = urllib.request.Request(GITHUB_URL, headers=headers)
    with urllib.request.urlopen(req, timeout=10) as resp:
        releases = json.loads(resp.read().decode())
        for r in releases:
            tag = r.get("tag_name", "unknown")
            r_total = sum(a.get("download_count", 0) for a in r.get("assets", []))
            if r_total > 0:
                gh_breakdown.append((tag, r_total))
        gh_total = sum(count for _, count in gh_breakdown)
except Exception as e:
    print(f"Warning: Failed to fetch GitHub stats: {e}", file=sys.stderr)

# 2. Fetch npm stats
npm_total = 0
npm_7d = 0
npm_30d = 0
try:
    req = urllib.request.Request(NPM_RANGE_URL, headers=headers)
    with urllib.request.urlopen(req, timeout=10) as resp:
        npm_data = json.loads(resp.read().decode())
        npm_total = sum(d.get("downloads", 0) for d in npm_data.get("downloads", []))

    req_7d = urllib.request.Request(NPM_LAST_7D_URL, headers=headers)
    with urllib.request.urlopen(req_7d, timeout=10) as resp:
        data_7d = json.loads(resp.read().decode())
        npm_7d = data_7d.get("downloads", 0)

    req_30d = urllib.request.Request(NPM_LAST_30D_URL, headers=headers)
    with urllib.request.urlopen(req_30d, timeout=10) as resp:
        data_30d = json.loads(resp.read().decode())
        npm_30d = data_30d.get("downloads", 0)
except Exception as e:
    print(f"Warning: Failed to fetch npm stats: {e}", file=sys.stderr)

grand_total = gh_total + npm_total

print(f"🚀 PaperPilot Ecosystem Downloads: {grand_total:,}")
print("═" * 44)
print(f"📦 npm (@usepaperpilot/mcp) : {npm_total:,}")
print(f"   • Last 7 days            : {npm_7d:,}")
print(f"   • Last 30 days           : {npm_30d:,}")
print("─" * 44)
print(f"💻 GitHub Releases (Desktop): {gh_total:,}")
for tag, count in gh_breakdown:
    print(f"   • {tag:23}: {count:,}")
