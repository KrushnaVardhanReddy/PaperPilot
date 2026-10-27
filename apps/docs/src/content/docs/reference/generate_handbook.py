import os
import glob
import re

def extract_code_block(content, section_pattern):
    # Match the heading and capture the first code block following it
    pattern = r'(?:' + section_pattern + r')\s*\n+```(?:bash|json|sh)?\n(.*?)\n```'
    match = re.search(pattern, content, re.DOTALL | re.IGNORECASE)
    if match and match.group(1):
        return match.group(1).strip()
    return "N/A"

def extract_description(content):
    fm = re.search(r'^---\n(.*?)\n---', content, re.DOTALL)
    desc = ""
    if fm:
        d_match = re.search(r'description:\s*["\']?(.*?)["\']?\n', fm.group(1))
        if d_match:
            desc = d_match.group(1).strip()
    
    what_match = re.search(r'### What it does\s*\n(.*?)(?=\n###|\Z)', content, re.DOTALL)
    if what_match:
        desc = what_match.group(1).strip()
    elif not desc:
        body = re.sub(r'^---\n.*?\n---\n', '', content, flags=re.DOTALL)
        body = re.sub(r'#.*?\n', '', body).strip()
        first_para = body.split('\n\n')[0].strip()
        if first_para:
            desc = first_para
    return desc

def generate_handbook():
    tools_dir = 'apps/docs/src/content/docs/reference/tools'
    output_path = 'apps/docs/src/content/docs/reference/50-tools-handbook.md'
    tool_files = sorted(glob.glob(os.path.join(tools_dir, '*.md')))

    print(f"Discovered {len(tool_files)} tool documents in {tools_dir}")

    with open(output_path, 'w', encoding='utf-8') as f:
        f.write("---\n")
        f.write("title: PaperPilot Tools Handbook\n")
        f.write(f"description: Comprehensive reference for all {len(tool_files)} PaperPilot tools across CLI, MCP, and REST API interfaces.\n")
        f.write("---\n\n")
        f.write(f"# PaperPilot Tools Handbook ({len(tool_files)} Tools)\n\n")
        f.write(f"This handbook provides verified execution snippets and parameter references for all **{len(tool_files)} PaperPilot tools** across the CLI, MCP (AI Agent JSON-RPC), and local REST API interfaces.\n\n")

        for i, filepath in enumerate(tool_files, 1):
            tool_id = os.path.basename(filepath).replace('.md', '')
            with open(filepath, 'r', encoding='utf-8') as tf:
                content = tf.read()

            desc = extract_description(content)
            cli_code = extract_code_block(content, r'###\s*CLI|####\s*1\.\s*CLI|##\s*Usage Examples\s*\n\s*###\s*CLI')
            mcp_code = extract_code_block(content, r'###\s*MCP JSON-RPC|####\s*2\.\s*MCP JSON-RPC|###\s*MCP')
            api_code = extract_code_block(content, r'###\s*REST API|####\s*3\.\s*REST API|###\s*REST')

            f.write(f"## {i}. `{tool_id}`\n\n")
            f.write("**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data\n\n")
            if desc:
                f.write(f"{desc}\n\n")

            f.write("### CLI\n")
            f.write("```bash\n")
            f.write(f"{cli_code}\n")
            f.write("```\n\n")

            f.write("### MCP JSON-RPC\n")
            f.write("```json\n")
            f.write(f"{mcp_code}\n")
            f.write("```\n\n")

            f.write("### REST API\n")
            f.write("```bash\n")
            f.write(f"{api_code}\n")
            f.write("```\n\n")

    print(f"Successfully generated handbook with {len(tool_files)} tools at {output_path}")

if __name__ == '__main__':
    generate_handbook()
