#!/usr/bin/env python3
"""
PaperPilot Live Monitor: Downloads (npm + Desktop), GitHub Stars,
and open external PRs & Issues.
"""

import sys
import json
import urllib.request
from datetime import datetime

OWNER = "KrushnaVardhanReddy"
REPO = "PaperPilot"
REPO_URL = f"https://api.github.com/repos/{OWNER}/{REPO}"
RELEASES_URL = f"{REPO_URL}/releases"
ISSUES_URL = f"{REPO_URL}/issues?state=open&per_page=30"
NPM_LAST_7D_URL = "https://api.npmjs.org/downloads/point/last-week/@usepaperpilot/mcp"
PYPI_OVERALL_URL = "https://pypistats.org/api/packages/usepaperpilot/overall"
PYPI_RECENT_URL = "https://pypistats.org/api/packages/usepaperpilot/recent"
HEADERS = {"User-Agent": "PaperPilot-Monitor"}


def fetch_json(url: str):
    req = urllib.request.Request(url, headers=HEADERS)
    with urllib.request.urlopen(req, timeout=10) as resp:
        return json.loads(resp.read().decode())


def main() -> None:
    npm_range_url = (
        f"https://api.npmjs.org/downloads/range/2026-01-01:"
        f"{datetime.now().strftime('%Y-%m-%d')}/@usepaperpilot/mcp"
    )

    # 1. GitHub Repo (Stars)
    stars = 0
    try:
        repo_data = fetch_json(REPO_URL)
        stars = repo_data.get("stargazers_count", 0)
    except Exception as e:
        print(f"Warning: Failed to fetch repo info: {e}", file=sys.stderr)

    # 2. GitHub Releases (Desktop total)
    gh_total = 0
    try:
        releases = fetch_json(RELEASES_URL)
        gh_total = sum(
            a.get("download_count", 0)
            for r in releases
            for a in r.get("assets", [])
        )
    except Exception as e:
        print(f"Warning: Failed to fetch GitHub releases: {e}", file=sys.stderr)

    # 3. npm stats (total + 7d)
    npm_total = 0
    npm_7d = 0
    try:
        npm_data = fetch_json(npm_range_url)
        npm_total = sum(d.get("downloads", 0) for d in npm_data.get("downloads", []))
        npm_7d = fetch_json(NPM_LAST_7D_URL).get("downloads", 0)
    except Exception as e:
        print(f"Warning: Failed to fetch npm stats: {e}", file=sys.stderr)

    # 4. PyPI stats (total + 7d), silent 0 fallback on error
    pypi_total = 0
    pypi_7d = 0
    try:
        pypi_overall = fetch_json(PYPI_OVERALL_URL)
        rows = pypi_overall.get("data", [])
        without_mirrors = [d.get("downloads", 0) for d in rows if d.get("category") == "without_mirrors"]
        pypi_total = sum(without_mirrors) if without_mirrors else sum(d.get("downloads", 0) for d in rows)
    except Exception:
        pypi_total = 0
    try:
        pypi_recent = fetch_json(PYPI_RECENT_URL)
        pypi_7d = pypi_recent.get("data", {}).get("last_week", 0) or 0
    except Exception:
        pypi_7d = 0

    # 5. External Open PRs & Issues
    ext_prs = []
    ext_issues = []
    try:
        items = fetch_json(ISSUES_URL)
        for item in items:
            author = item.get("user", {}).get("login", "")
            if author.lower() == OWNER.lower() or author.endswith("[bot]"):
                continue
            entry = (item.get("number"), item.get("title", ""), author)
            if "pull_request" in item:
                ext_prs.append(entry)
            else:
                ext_issues.append(entry)
    except Exception as e:
        print(f"Warning: Failed to fetch issues/PRs: {e}", file=sys.stderr)

    grand_total = gh_total + npm_total + pypi_total

    print(f"🚀 PaperPilot Monitor  |  ⭐ {stars:,} Stars")
    print("═" * 48)
    print(f"📥 Total Downloads : {grand_total:,}  (npm: {npm_total:,} [7d: {npm_7d:,}] | PyPI: {pypi_total:,} | Desktop: {gh_total:,})")
    print("─" * 48)
    print(f"🔀 External Open PRs    : {len(ext_prs)}")
    for num, title, author in ext_prs[:5]:
        print(f"   • #{num} {title[:36]} (@{author})")
    print(f"🐛 External Open Issues : {len(ext_issues)}")
    for num, title, author in ext_issues[:5]:
        print(f"   • #{num} {title[:36]} (@{author})")


if __name__ == "__main__":
    main()
