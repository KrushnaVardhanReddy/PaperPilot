"""Training definitions loaded from a snapshot of the real MCP tools/list schemas.

Refresh with sync_tool_schemas.py after changing a server schema. Artifact types
and the deliberately supported training subset live in workflow.py.
"""

import json
from pathlib import Path
from typing import Any

from pydantic import BaseModel


class ToolParameter(BaseModel):
    type: str
    description: str = ""
    items: dict[str, Any] | None = None


class ToolDef(BaseModel):
    name: str
    properties: dict[str, ToolParameter]
    required: list[str]


TOOL_SCHEMAS = json.loads(Path(__file__).with_name("mcp_tool_schemas.json").read_text(encoding="utf-8"))
TOOLS_REGISTRY = {
    name: ToolDef(name=name, properties=schema["properties"], required=schema.get("required", []))
    for name, schema in TOOL_SCHEMAS.items()
}
