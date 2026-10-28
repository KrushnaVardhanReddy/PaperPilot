"""Refresh/check the training schema snapshot using the actual local MCP server."""

import argparse
import json
from pathlib import Path

from mcp_client import McpClient
from workflow import CONTRACTS

SNAPSHOT = Path(__file__).with_name("mcp_tool_schemas.json")


def read_live_schemas(client):
    tools = client.list_tools()
    missing = CONTRACTS.keys() - tools.keys()
    if missing:
        raise ValueError(f"Training tools absent from MCP tools/list: {sorted(missing)}")
    return {name: tools[name]["inputSchema"] for name in sorted(CONTRACTS)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mcp-bin", type=Path, required=True)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    with McpClient(args.mcp_bin) as client:
        schemas = read_live_schemas(client)
    if args.check:
        if schemas != json.loads(SNAPSHOT.read_text(encoding="utf-8")):
            raise SystemExit("Training schemas differ from the MCP server; refresh the snapshot")
        print(f"Verified {len(schemas)} training tool schemas against live MCP tools/list")
    else:
        SNAPSHOT.write_text(json.dumps(schemas, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        print(f"Saved {len(schemas)} MCP schemas to {SNAPSHOT}")


if __name__ == "__main__":
    main()
