#!/usr/bin/env python3
"""
Generate crisp, pixel-perfect animated frames and convert to palette-optimized GIF
for Week 03: Multi-Interface Showcase - Structure-Aware RAG vs. Naive Text Chunking (pdf_to_markdown vs naive chunking).

4-Scene Progression:
  Scene 1: Desktop GUI (Visual PDF Viewer on left, Structure-Aware Markdown with table grid on right, 1-Click Export)
  Scene 2: AI Agent / MCP (Claude Desktop JSON tool call & arguments execution for vector ingestion)
  Scene 3: CLI & REST API (Sub-15ms terminal execution & sub-5ms curl/requests payload)
  Scene 4: Cryptographic Verification (Clean table retention vs. naive hallucination breakdown)
"""

import os
import shutil
import subprocess
from PIL import Image, ImageDraw, ImageFont

WIDTH = 1200
HEIGHT = 675
FPS = 10

FONT_REGULAR = "/usr/share/fonts/truetype/croscore/Arimo-Regular.ttf"
FONT_BOLD = "/usr/share/fonts/truetype/croscore/Arimo-Bold.ttf"
FONT_MONO = "/usr/share/fonts/truetype/croscore/Cousine-Regular.ttf"
FONT_MONO_BOLD = "/usr/share/fonts/truetype/croscore/Cousine-Bold.ttf"

def get_font(path, size):
    try:
        return ImageFont.truetype(path, size)
    except Exception:
        return ImageFont.load_default()

font_title = get_font(FONT_BOLD, 22)
font_subtitle = get_font(FONT_REGULAR, 14)
font_badge = get_font(FONT_BOLD, 12)
font_tab = get_font(FONT_BOLD, 13)
font_body = get_font(FONT_REGULAR, 14)
font_bold = get_font(FONT_BOLD, 14)
font_mono_lg = get_font(FONT_MONO_BOLD, 15)
font_mono_md = get_font(FONT_MONO, 13)
font_mono_sm = get_font(FONT_MONO, 12)
font_doc_title = get_font(FONT_BOLD, 16)
font_doc_body = get_font(FONT_REGULAR, 13)
font_doc_bold = get_font(FONT_BOLD, 13)

def draw_window_chrome(draw, x, y, w, h, title, active_tab_idx=0, tabs=None):
    draw.rounded_rectangle([x, y, x + w, y + h], radius=8, fill=(24, 28, 38), outline=(45, 52, 70), width=1)
    draw.rectangle([x, y, x + w, y + 40], fill=(28, 33, 46))
    draw.ellipse([x + 14, y + 14, x + 24, y + 24], fill=(255, 95, 87))
    draw.ellipse([x + 30, y + 14, x + 40, y + 24], fill=(254, 188, 46))
    draw.ellipse([x + 46, y + 14, x + 56, y + 24], fill=(40, 201, 64))
    
    if tabs:
        cur_tab_x = x + 70
        for i, tab in enumerate(tabs):
            tab_w = 175
            is_active = (i == active_tab_idx)
            if is_active:
                draw.rounded_rectangle([cur_tab_x, y + 8, cur_tab_x + tab_w, y + 40], radius=5, fill=(36, 44, 62))
                draw.rectangle([cur_tab_x, y + 34, cur_tab_x + tab_w, y + 40], fill=(36, 44, 62))
                draw.line([(cur_tab_x + 8, y + 38), (cur_tab_x + tab_w - 8, y + 38)], fill=(80, 160, 255), width=2)
                draw.text((cur_tab_x + 16, y + 14), tab, font=font_tab, fill=(255, 255, 255))
            else:
                draw.text((cur_tab_x + 16, y + 14), tab, font=font_tab, fill=(130, 145, 170))
            cur_tab_x += tab_w + 10
    else:
        draw.text((x + 70, y + 13), title, font=font_mono_sm, fill=(185, 195, 215))

def draw_top_banner(draw, week_badge, title_text, subtitle_text, active_surface):
    draw.rounded_rectangle([40, 16, 280, 42], radius=13, fill=(35, 45, 68), outline=(65, 95, 160), width=1)
    draw.text((55, 22), week_badge, font=font_badge, fill=(100, 185, 255))

    draw.text((300, 18), title_text, font=font_title, fill=(255, 255, 255))
    draw.text((40, 48), subtitle_text, font=font_subtitle, fill=(140, 155, 180))

    surfaces = [
        ("🖥️ Desktop GUI", active_surface == "gui"),
        ("🤖 MCP Agent", active_surface == "mcp"),
        ("💻 CLI / API", active_surface == "cli"),
        ("📊 RAG Vector DB", active_surface == "result")
    ]
    cur_pill_x = WIDTH - 40
    for label, active in reversed(surfaces):
        pill_w = 135
        cur_pill_x -= pill_w
        if active:
            draw.rounded_rectangle([cur_pill_x, 48, cur_pill_x + pill_w - 8, 72], radius=12, fill=(45, 90, 180), outline=(100, 160, 255), width=1)
            draw.text((cur_pill_x + 10, 53), label, font=font_badge, fill=(255, 255, 255))
        else:
            draw.rounded_rectangle([cur_pill_x, 48, cur_pill_x + pill_w - 8, 72], radius=12, fill=(22, 28, 40), outline=(45, 55, 75), width=1)
            draw.text((cur_pill_x + 10, 53), label, font=font_badge, fill=(120, 135, 160))
        cur_pill_x -= 6

def render_scene1_desktop_gui(state):
    """Scene 1: Desktop GUI visual PDF to Markdown converter with side-by-side preview."""
    img = Image.new("RGB", (WIDTH, HEIGHT), (14, 17, 24))
    draw = ImageDraw.Draw(img)

    draw_top_banner(
        draw,
        "WEEK 03 : MULTI-INTERFACE",
        "Structure-Aware RAG vs. Naive Text Chunking",
        "Surface 1/4: PaperPilot Desktop GUI — Dual-pane PDF vs GFM Markdown parser",
        "gui"
    )

    wx, wy, ww, wh = 40, 80, WIDTH - 80, HEIGHT - 105
    tabs = ["📄 Enterprise_SLA.pdf", "📝 Structured Markdown", "🧩 Semantic Chunks", "🔍 Embeddings"]
    draw_window_chrome(draw, wx, wy, ww, wh, "", active_tab_idx=1, tabs=tabs)

    half_w = (ww - 20) // 2

    # Left: PDF Canvas
    px, py, pw, ph = wx + 10, wy + 48, half_w - 5, wh - 60
    draw.rounded_rectangle([px, py, px + pw, py + ph], radius=6, fill=(248, 250, 254), outline=(50, 60, 80), width=1)
    draw.text((px + 20, py + 15), "PDF RENDER: Enterprise_SLA_Pricing.pdf (Page 4)", font=font_doc_bold, fill=(40, 50, 70))
    draw.line([(px + 20, py + 38), (px + pw - 20, py + 38)], fill=(215, 220, 230), width=1)

    # Table layout in PDF view
    ty = py + 55
    draw.text((px + 20, ty), "Tier Matrix & Service Level Agreements", font=font_doc_title, fill=(20, 30, 45))
    ty += 28
    
    # Table box
    tbl_w = pw - 40
    draw.rectangle([px + 20, ty, px + 20 + tbl_w, ty + 120], outline=(180, 190, 210), width=1)
    draw.rectangle([px + 20, ty, px + 20 + tbl_w, ty + 28], fill=(230, 238, 250))
    draw.text((px + 30, ty + 6), "Plan Tier", font=font_doc_bold, fill=(30, 50, 80))
    draw.text((px + 150, ty + 6), "Monthly", font=font_doc_bold, fill=(30, 50, 80))
    draw.text((px + 270, ty + 6), "Uptime SLA", font=font_doc_bold, fill=(30, 50, 80))
    draw.text((px + 390, ty + 6), "Support Level", font=font_doc_bold, fill=(30, 50, 80))

    # Row 1
    draw.text((px + 30, ty + 36), "Starter", font=font_doc_body, fill=(50, 60, 75))
    draw.text((px + 150, ty + 36), "$499/mo", font=font_doc_body, fill=(50, 60, 75))
    draw.text((px + 270, ty + 36), "99.5%", font=font_doc_body, fill=(50, 60, 75))
    draw.text((px + 390, ty + 36), "Email (24h)", font=font_doc_body, fill=(50, 60, 75))

    # Row 2 (Pro)
    draw.text((px + 30, ty + 64), "Pro Tier", font=font_doc_bold, fill=(20, 90, 180))
    draw.text((px + 150, ty + 64), "$1,499/mo", font=font_doc_bold, fill=(20, 90, 180))
    draw.text((px + 270, ty + 64), "99.9%", font=font_doc_bold, fill=(20, 90, 180))
    draw.text((px + 390, ty + 64), "Dedicated (4h)", font=font_doc_bold, fill=(20, 90, 180))

    # Row 3 (Enterprise)
    draw.text((px + 30, ty + 92), "Enterprise", font=font_doc_body, fill=(50, 60, 75))
    draw.text((px + 150, ty + 92), "Custom", font=font_doc_body, fill=(50, 60, 75))
    draw.text((px + 270, ty + 92), "99.99%", font=font_doc_body, fill=(50, 60, 75))
    draw.text((px + 390, ty + 92), "24/7 Phone SLA", font=font_doc_body, fill=(50, 60, 75))

    # Right: Structured Markdown Canvas
    mx, my, mw, mh = wx + half_w + 10, wy + 48, half_w - 5, wh - 60
    draw.rounded_rectangle([mx, my, mx + mw, my + mh], radius=6, fill=(18, 22, 32), outline=(40, 50, 70), width=1)
    draw.text((mx + 20, my + 15), "GFM MARKDOWN PARSER (GitHub Flavored Pipes)", font=font_mono_md, fill=(100, 190, 255))
    draw.line([(mx + 20, my + 38), (mx + mw - 20, my + 38)], fill=(40, 50, 70), width=1)

    m_lines = [
        "## Tier Matrix & Service Level Agreements",
        "",
        "| Plan Tier   | Monthly   | Uptime SLA | Support Level   |",
        "|:------------|:----------|:-----------|:----------------|",
        "| Starter     | $499/mo   | 99.5%      | Email (24h)     |",
        "| Pro Tier    | $1,499/mo | 99.9%      | Dedicated (4h)  |",
        "| Enterprise  | Custom    | 99.99%     | 24/7 Phone SLA  |",
        "",
        "> Preserves row/column coordinates for Qdrant / Pinecone."
    ]
    cur_m_y = my + 55
    for line in m_lines:
        color = (255, 215, 0) if "Pro Tier" in line else ((80, 220, 140) if "|" in line else (220, 230, 245))
        draw.text((mx + 20, cur_m_y), line, font=font_mono_sm, fill=color)
        cur_m_y += 24

    # Action / Status Bar at bottom
    btn_y = my + mh - 75
    btn_clicked = state.get("btn_clicked", False)
    btn_fill = (40, 180, 100) if btn_clicked else (45, 110, 235)
    draw.rounded_rectangle([mx + 20, btn_y, mx + mw - 20, btn_y + 44], radius=6, fill=btn_fill)
    b_txt = "✓ Parsed into Structured Markdown (11.2ms)" if btn_clicked else "⚡ Export Structure-Aware Markdown"
    draw.text((mx + 35, btn_y + 13), b_txt, font=font_bold, fill=(255, 255, 255))

    return img

def render_scene2_mcp_agent(state):
    """Scene 2: Claude Desktop executing pdf_to_markdown via MCP."""
    img = Image.new("RGB", (WIDTH, HEIGHT), (14, 17, 24))
    draw = ImageDraw.Draw(img)

    draw_top_banner(
        draw,
        "WEEK 03 : MULTI-INTERFACE",
        "Structure-Aware RAG vs. Naive Text Chunking",
        "Surface 2/4: AI Agent / MCP — Claude Desktop & RAG agents ingest tables deterministically",
        "mcp"
    )

    wx, wy, ww, wh = 40, 80, WIDTH - 80, HEIGHT - 105
    draw_window_chrome(draw, wx, wy, ww, wh, "Claude Desktop — Model Context Protocol (@usepaperpilot/mcp)")

    user_y = wy + 55
    draw.rounded_rectangle([wx + 30, user_y, wx + ww - 30, user_y + 60], radius=8, fill=(32, 38, 52), outline=(50, 60, 85), width=1)
    draw.rounded_rectangle([wx + 45, user_y + 14, wx + 85, user_y + 38], radius=4, fill=(70, 110, 210))
    draw.text((wx + 52, user_y + 18), "USER", font=font_badge, fill=(255, 255, 255))
    prompt = "Extract all SLA tiers and pricing from Enterprise_SLA.pdf into structured markdown tables for vector ingestion."
    draw.text((wx + 100, user_y + 20), prompt, font=font_bold, fill=(240, 245, 255))

    claude_y = user_y + 75
    draw.rounded_rectangle([wx + 30, claude_y, wx + 100, claude_y + 24], radius=4, fill=(217, 119, 87))
    draw.text((wx + 38, claude_y + 4), "CLAUDE", font=font_badge, fill=(255, 255, 255))
    draw.text((wx + 115, claude_y + 3), "I'll invoke PaperPilot's local pdf_to_markdown tool preserving table grid alignment.", font=font_body, fill=(180, 195, 215))

    tool_y = claude_y + 36
    draw.rounded_rectangle([wx + 30, tool_y, wx + ww - 30, tool_y + 200], radius=8, fill=(22, 28, 40), outline=(55, 75, 115), width=1)
    draw.text((wx + 45, tool_y + 12), "⚙️ MCP Tool Call: paperpilot.pdf_to_markdown", font=font_mono_lg, fill=(120, 200, 255))
    t_status = "✓ COMPLETED IN 11.2ms" if state.get("tool_done", False) else "EXECUTING..."
    t_color = (80, 220, 140) if state.get("tool_done", False) else (255, 180, 60)
    draw.text((wx + ww - 240, tool_y + 12), t_status, font=font_mono_sm, fill=t_color)

    args_lines = [
        '{',
        '  "input": "Enterprise_SLA_Pricing.pdf",',
        '  "table_mode": "github_pipes",',
        '  "preserve_hierarchy": true,',
        '  "chunk_boundary": "semantic_headers"',
        '}'
    ]
    cur_j_y = tool_y + 38
    for line in args_lines:
        draw.text((wx + 45, cur_j_y), line, font=font_mono_sm, fill=(210, 225, 245))
        cur_j_y += 18

    out_y = tool_y + 145
    draw.line([(wx + 30, out_y), (wx + ww - 30, out_y)], fill=(45, 55, 80), width=1)
    draw.text((wx + 45, out_y + 10), "Response:", font=font_mono_sm, fill=(130, 145, 175))
    draw.text((wx + 130, out_y + 10), '{"status": "success", "tables_found": 3, "chunks_produced": 8, "execution_ms": 11.2}', font=font_mono_sm, fill=(80, 220, 140))

    res_box_y = tool_y + 215
    draw.rounded_rectangle([wx + 30, res_box_y, wx + ww - 30, res_box_y + 70], radius=6, fill=(26, 34, 48), outline=(45, 60, 90), width=1)
    draw.text((wx + 45, res_box_y + 12), "Claude: Clean Markdown tables generated with headers intact across all 3 tiers.", font=font_body, fill=(240, 245, 255))
    draw.text((wx + 45, res_box_y + 36), "Pro tier confirmed: $1,499/mo at 99.9% uptime SLA. Zero vector chunk hallucinations.", font=font_body, fill=(120, 210, 160))

    return img

def render_scene3_cli_api(state):
    """Scene 3: Dual Terminal & REST API execution."""
    img = Image.new("RGB", (WIDTH, HEIGHT), (14, 17, 24))
    draw = ImageDraw.Draw(img)

    draw_top_banner(
        draw,
        "WEEK 03 : MULTI-INTERFACE",
        "Structure-Aware RAG vs. Naive Text Chunking",
        "Surface 3/4: Terminal CLI & REST API — Sub-15ms native command & sub-5ms microservice",
        "cli"
    )

    wx, wy, ww, wh = 40, 80, WIDTH - 80, HEIGHT - 105
    half_w = (ww - 20) // 2

    # Left: CLI Window
    draw_window_chrome(draw, wx, wy, half_w, wh, "zsh — paperpilot CLI (Rust Native)")
    cx = wx + 20
    cy = wy + 55
    prompt = "$ "
    draw.text((cx, cy), prompt, font=font_mono_md, fill=(80, 220, 120))
    cmd1 = "paperpilot pdf-to-markdown -i SLA_Pricing.pdf \\"
    draw.text((cx + 20, cy), cmd1, font=font_mono_sm, fill=(240, 245, 255))
    cy += 20
    cmd2 = "  -o sla.md --table-mode github_pipes"
    draw.text((cx + 20, cy), cmd2, font=font_mono_sm, fill=(240, 245, 255))
    cy += 26
    draw.text((cx + 20, cy), "✓ Structured markdown generated in 11.2ms", font=font_mono_sm, fill=(80, 220, 140))
    draw.text((cx + 20, cy + 18), "  Tables parsed: 3 (GFM pipe matrix preserved)", font=font_mono_sm, fill=(140, 160, 190))
    cy += 48

    draw.text((cx, cy), prompt, font=font_mono_md, fill=(80, 220, 120))
    cmd3 = "paperpilot chunk -i sla.md --strategy semantic"
    draw.text((cx + 20, cy), cmd3, font=font_mono_sm, fill=(240, 245, 255))
    cy += 24
    draw.text((cx + 20, cy), "✓ Produced 8 semantic chunks without table splitting", font=font_mono_sm, fill=(80, 220, 140))
    draw.text((cx + 20, cy + 18), "  Zero orphan data rows. Ready for Vector DB.", font=font_mono_sm, fill=(100, 200, 255))

    # Right: REST API Window
    rx = wx + half_w + 20
    draw_window_chrome(draw, rx, wy, half_w, wh, "HTTP REST API — http://localhost:8080")
    rcx = rx + 20
    rcy = wy + 55

    draw.text((rcx, rcy), "# Python Requests / Ingestion Microservice", font=font_mono_sm, fill=(120, 140, 170))
    rcy += 24
    api_code = [
        'import requests',
        '',
        'resp = requests.post(',
        '  "http://localhost:8080/api/v1/to-markdown",',
        '  json={',
        '    "input_path": "Enterprise_SLA.pdf",',
        '    "extract_tables": True,',
        '    "chunk_strategy": "semantic_headers"',
        '  }',
        ')'
    ]
    for line in api_code:
        draw.text((rcx, rcy), line, font=font_mono_sm, fill=(220, 230, 250))
        rcy += 18

    rcy += 15
    draw.rounded_rectangle([rcx, rcy, rx + half_w - 20, rcy + 105], radius=6, fill=(18, 22, 32), outline=(40, 50, 70), width=1)
    draw.text((rcx + 12, rcy + 10), "HTTP/1.1 200 OK  (Latency: 3.9ms)", font=font_mono_sm, fill=(80, 220, 140))
    draw.text((rcx + 12, rcy + 32), '{', font=font_mono_sm, fill=(180, 195, 215))
    draw.text((rcx + 12, rcy + 50), '  "status": "success", "tables_preserved": 3,', font=font_mono_sm, fill=(180, 195, 215))
    draw.text((rcx + 12, rcy + 68), '  "execution_ms": 3.91,', font=font_mono_sm, fill=(255, 215, 0))
    draw.text((rcx + 12, rcy + 86), '  "chunks_count": 8', font=font_mono_sm, fill=(100, 200, 255))
    draw.text((rcx + 12, rcy + 104), '}', font=font_mono_sm, fill=(180, 195, 215))

    return img

def render_scene4_verified_result(state):
    """Scene 4: Cryptographic Verification & Multi-Surface Summary."""
    img = Image.new("RGB", (WIDTH, HEIGHT), (14, 17, 24))
    draw = ImageDraw.Draw(img)

    draw_top_banner(
        draw,
        "WEEK 03 : MULTI-INTERFACE",
        "Structure-Aware RAG vs. Naive Text Chunking",
        "Surface 4/4: Vector Retrieval Accuracy — Deterministic table retention vs naive hallucination",
        "result"
    )

    wx, wy, ww, wh = 40, 80, WIDTH - 80, HEIGHT - 105
    draw_window_chrome(draw, wx, wy, ww, wh, "PaperPilot Engine — Multi-Interface Architecture Overview")

    card_y = wy + 60
    draw.rounded_rectangle([wx + 40, card_y, wx + ww - 40, card_y + 110], radius=8, fill=(22, 36, 30), outline=(45, 140, 80), width=2)
    draw.text((wx + 60, card_y + 18), "📊 RAG BENCHMARK: 100% RETRIEVAL GROUNDING", font=font_title, fill=(80, 230, 140))
    draw.text((wx + 60, card_y + 52), "Query: 'What is Pro tier uptime?' -> Exact Match: 99.9% SLA (0% Hallucination)", font=font_mono_md, fill=(180, 240, 200))
    draw.text((wx + 60, card_y + 76), "Preserved Markdown table pipes & H1/H2 document hierarchy across Qdrant & Pinecone", font=font_body, fill=(140, 200, 160))

    col_y = card_y + 130
    col_w = (ww - 80 - 45) // 4
    col_h = wh - 220

    surfaces = [
        ("🖥️ Desktop GUI", "Document Analysts", "Dual-Pane Preview", "Sub-20ms", (60, 110, 210)),
        ("🤖 MCP Agent", "Claude & Cursor", "Deterministic Tools", "Sub-15ms", (217, 119, 87)),
        ("💻 Terminal CLI", "Data Engineers", "High-Speed Pipelines", "Sub-12ms", (80, 180, 120)),
        ("🌐 REST API", "Vector Microservices", "Sub-5ms Webhook JSON", "Sub-5ms", (180, 90, 220)),
    ]

    for i, (s_title, s_target, s_mode, s_speed, s_accent) in enumerate(surfaces):
        cx = wx + 40 + i * (col_w + 15)
        draw.rounded_rectangle([cx, col_y, cx + col_w, col_y + col_h], radius=6, fill=(20, 25, 36), outline=(40, 50, 72), width=1)
        draw.rectangle([cx, col_y, cx + col_w, col_y + 6], fill=s_accent)
        draw.text((cx + 15, col_y + 18), s_title, font=font_bold, fill=(255, 255, 255))
        draw.text((cx + 15, col_y + 42), s_target, font=font_mono_sm, fill=(130, 145, 175))
        draw.line([(cx + 15, col_y + 65), (cx + col_w - 15, col_y + 65)], fill=(35, 45, 65), width=1)

        draw.text((cx + 15, col_y + 78), "Execution Flow:", font=font_mono_sm, fill=(120, 140, 170))
        draw.text((cx + 15, col_y + 98), s_mode, font=font_doc_bold, fill=(210, 225, 245))

        draw.text((cx + 15, col_y + 130), "Latency:", font=font_mono_sm, fill=(120, 140, 170))
        draw.text((cx + 15, col_y + 150), s_speed, font=font_mono_lg, fill=(80, 220, 140))

        draw.text((cx + 15, col_y + 182), "Egress:", font=font_mono_sm, fill=(120, 140, 170))
        draw.text((cx + 15, col_y + 202), "0 Bytes (Local)", font=font_doc_bold, fill=(100, 200, 255))

    foot_y = wy + wh - 45
    draw.text((wx + 40, foot_y), "Open-Source & Local-First in Pure Rust  •  https://github.com/usepaperpilot/paperpilot", font=font_mono_sm, fill=(130, 150, 180))

    return img

def main():
    frames_dir = "/tmp/week03_frames"
    if os.path.exists(frames_dir):
        shutil.rmtree(frames_dir)
    os.makedirs(frames_dir)

    print("Generating Week 03 Multi-Interface Showcase frames...")

    frame_count = 0

    # Scene 1: Desktop GUI (18 frames)
    for _ in range(8):
        img = render_scene1_desktop_gui({"btn_clicked": False})
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1
    for _ in range(10):
        img = render_scene1_desktop_gui({"btn_clicked": True})
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    # Scene 2: AI Agent / MCP (16 frames)
    for i in range(16):
        img = render_scene2_mcp_agent({"tool_done": (i >= 6)})
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    # Scene 3: CLI & REST API (16 frames)
    for _ in range(16):
        img = render_scene3_cli_api({})
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    # Scene 4: Cryptographic Verification & Multi-Surface Summary (18 frames)
    for _ in range(18):
        img = render_scene4_verified_result({})
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    print(f"Rendered {frame_count} frames. Encoding GIF via ffmpeg palette...")

    palette_path = "/tmp/week03_palette.png"
    out_gif = "docs/assets/showcase/week03-structure-aware-rag.gif"

    cmd1 = [
        "ffmpeg", "-y", "-framerate", str(FPS), "-i", f"{frames_dir}/frame_%04d.png",
        "-vf", "palettegen=max_colors=256:stats_mode=diff",
        palette_path
    ]
    subprocess.run(cmd1, check=True)

    cmd2 = [
        "ffmpeg", "-y", "-framerate", str(FPS), "-i", f"{frames_dir}/frame_%04d.png",
        "-i", palette_path,
        "-lavfi", "paletteuse=dither=bayer:bayer_scale=3",
        out_gif
    ]
    subprocess.run(cmd2, check=True)

    size_mb = os.path.getsize(out_gif) / (1024 * 1024)
    print(f"Done! Saved {out_gif} ({size_mb:.2f} MB)")

if __name__ == "__main__":
    main()
