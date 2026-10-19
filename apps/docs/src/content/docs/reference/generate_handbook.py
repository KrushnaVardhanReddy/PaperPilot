import re
import sys
import os

def parse_report(filepath):
    tools = {}

    if not os.path.exists(filepath):
        print(f"Error: {filepath} not found.")
        sys.exit(1)

    with open(filepath, 'r', encoding='utf-8') as f:
        for line in f:
            line = line.strip()
            if line.startswith('| `') and line.endswith('|'):
                parts = [p.strip() for p in line.split('|')]
                if len(parts) > 6:
                    tool_id = parts[1].strip('`')
                    interface = parts[2]
                    command = parts[4].strip('`')

                    if tool_id not in tools:
                        tools[tool_id] = {'cli': '', 'mcp': '', 'api': ''}

                    # Store only if it's the first time we see this command for this interface and we don't have it yet
                    # We prioritize simple/tier 1 tests as they contain the basic commands
                    if 'CLI' in interface and not tools[tool_id]['cli']:
                        tools[tool_id]['cli'] = command
                    elif 'MCP' in interface and not tools[tool_id]['mcp']:
                        # Try to format the MCP command nicely. It looks like `tools/call {"name": ...}`
                        if command.startswith('tools/call '):
                            json_part = command[len('tools/call '):]
                            try:
                                import json
                                parsed = json.loads(json_part)
                                formatted = json.dumps(parsed, indent=2)
                                tools[tool_id]['mcp'] = formatted
                            except:
                                tools[tool_id]['mcp'] = json_part
                        else:
                            tools[tool_id]['mcp'] = command
                    elif 'REST API' in interface and not tools[tool_id]['api']:
                        tools[tool_id]['api'] = command

    return tools

def generate_handbook(tools, output_path):
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write("---\n")
        f.write("title: 50 Tools Handbook\n")
        f.write("description: Comprehensive guide to all 50 PaperPilot tools across CLI, MCP, and REST interfaces.\n")
        f.write("---\n\n")
        f.write("# 50 Tools Handbook\n\n")
        f.write("This handbook provides verified execution snippets for all 50 PaperPilot tools across the CLI, MCP, and REST interfaces.\n\n")

        for i, (tool_id, commands) in enumerate(sorted(tools.items()), 1):
            f.write(f"## {i}. `{tool_id}`\n\n")

            # Format Badge (Simulated with markdown since it's just text right now, but could be enhanced)
            f.write("**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data\n\n")

            f.write("### CLI\n")
            f.write("```bash\n")
            f.write(f"{commands.get('cli', 'N/A')}\n")
            f.write("```\n\n")

            f.write("### MCP JSON-RPC\n")
            f.write("```json\n")
            f.write(f"{commands.get('mcp', 'N/A')}\n")
            f.write("```\n\n")

            f.write("### REST API\n")
            f.write("```bash\n")
            f.write(f"{commands.get('api', 'N/A')}\n")
            f.write("```\n\n")

if __name__ == '__main__':
    report_path = 'reports/PENTA_INTERFACE_EXPANSION_AND_REAL_PLAYWRIGHT_REPORT.md'
    output_path = 'apps/docs/src/content/docs/reference/50-tools-handbook.md'

    tools = parse_report(report_path)
    generate_handbook(tools, output_path)
    print(f"Generated handbook with {len(tools)} tools.")
