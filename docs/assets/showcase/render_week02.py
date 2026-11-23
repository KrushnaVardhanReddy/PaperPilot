#!/usr/bin/env python3
"""
Generate crisp, pixel-perfect animated frames and convert to palette-optimized GIF
for Week 02: Multi-Interface Showcase - Autonomous Legal Clerk & Bates Stamping (pdf_bates + pdf_split).

4-Scene Progression:
  Scene 1: Desktop GUI (Legal Tools dock, Bates Stamping prefix & position config, 1-Click Process)
  Scene 2: AI Agent / MCP (Claude Desktop autonomous tool call & arguments execution)
  Scene 3: CLI & REST API (Sub-15ms terminal execution & sub-5ms curl/requests payload)
  Scene 4: Cryptographic Verification (Clean stamped exhibits, deterministic Bates numbering)
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
        ("⚖️ Legal Output", active_surface == "result")
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
    """Scene 1: Desktop GUI Legal Tools Dock, Bates Stamping configuration, 1-click execution."""
    img = Image.new("RGB", (WIDTH, HEIGHT), (14, 17, 24))
    draw = ImageDraw.Draw(img)

    draw_top_banner(
        draw,
        "WEEK 02 : MULTI-INTERFACE",
        "Autonomous Legal Clerk: Bates Stamping & Exhibit Splitting",
        "Surface 1/4: PaperPilot Desktop GUI — Visual discovery stamping & section splitting",
        "gui"
    )

    wx, wy, ww, wh = 40, 80, WIDTH - 80, HEIGHT - 105
    tabs = ["📁 exhibit_bundle.pdf", "⚖️ Legal Bates Dock", "✂️ Split & Reorder", "🔒 Security"]
    draw_window_chrome(draw, wx, wy, ww, wh, "", active_tab_idx=1, tabs=tabs)

    sb_w = 310
    draw.rounded_rectangle([wx + 10, wy + 48, wx + sb_w, wy + wh - 12], radius=6, fill=(20, 24, 34), outline=(40, 48, 65), width=1)

    draw.text((wx + 25, wy + 62), "LEGAL DISCOVERY TOOLS", font=font_bold, fill=(120, 170, 255))
    draw.line([(wx + 25, wy + 84), (wx + sb_w - 15, wy + 84)], fill=(40, 48, 65), width=1)

    y_opt = wy + 98
    fields = [
        ("Bates Prefix:", "CASE-2026-"),
        ("Start Index:", "000001"),
        ("Position:", "Bottom-Right (Corner)"),
        ("Font & Size:", "Helvetica-Bold 10pt"),
        ("Auto-Split Exhibits:", "True (3 Sub-files)")
    ]
    for lbl, val in fields:
        draw.text((wx + 25, y_opt), lbl, font=font_doc_body, fill=(140, 155, 175))
        draw.rounded_rectangle([wx + 25, y_opt + 18, wx + sb_w - 20, y_opt + 40], radius=4, fill=(28, 34, 48), outline=(50, 60, 80), width=1)
        draw.text((wx + 35, y_opt + 23), val, font=font_mono_sm, fill=(230, 240, 255))
        y_opt += 48

    btn_y = wy + wh - 110
    btn_clicked = state.get("btn_clicked", False)
    btn_fill = (40, 180, 100) if btn_clicked else (45, 110, 235)
    draw.rounded_rectangle([wx + 25, btn_y, wx + sb_w - 15, btn_y + 44], radius=6, fill=btn_fill)
    btn_text = "✓ Bundle Processed (14.2ms)" if btn_clicked else "⚡ Apply Bates & Split (1-Click)"
    draw.text((wx + 38, btn_y + 13), btn_text, font=font_bold, fill=(255, 255, 255))

    draw.rounded_rectangle([wx + 25, btn_y + 54, wx + sb_w - 15, btn_y + 80], radius=4, fill=(26, 32, 46))
    draw.text((wx + 35, btn_y + 60), "Local Engine: 100% Air-Gapped / Offline", font=font_mono_sm, fill=(90, 190, 130))

    # Right Canvas: Document Viewer
    doc_x = wx + sb_w + 15
    doc_w = ww - sb_w - 25
    doc_y = wy + 48
    doc_h = wh - 60
    draw.rounded_rectangle([doc_x, doc_y, doc_x + doc_w, doc_y + doc_h], radius=6, fill=(248, 250, 254), outline=(50, 60, 80), width=1)

    draw.text((doc_x + 30, doc_y + 20), "TRIAL EXHIBIT BUNDLE - UNITED STATES DISTRICT COURT", font=font_doc_title, fill=(20, 30, 45))
    draw.line([(doc_x + 30, doc_y + 45), (doc_x + doc_w - 30, doc_y + 45)], fill=(215, 220, 230), width=1)

    dy = doc_y + 60
    draw.text((doc_x + 30, dy), "In Re: Autonomous AI Intellectual Property Litigation", font=font_doc_bold, fill=(60, 70, 85))
    dy += 24
    draw.text((doc_x + 30, dy), "Document Type: Master Deposition Exhibits 1 - 3 (6 Total Pages)", font=font_doc_body, fill=(60, 70, 85))
    dy += 36

    draw.rounded_rectangle([doc_x + 30, dy, doc_x + doc_w - 30, dy + 160], radius=6, fill=(255, 255, 255), outline=(210, 220, 235), width=1)
    draw.text((doc_x + 45, dy + 15), "Exhibit A: Software Architecture Specification (Pages 1-2)", font=font_doc_bold, fill=(40, 50, 70))
    draw.text((doc_x + 45, dy + 38), "Exhibit B: Verified Cryptographic Timestamp Log (Pages 3-4)", font=font_doc_bold, fill=(40, 50, 70))
    draw.text((doc_x + 45, dy + 61), "Exhibit C: Production Deployment Records (Pages 5-6)", font=font_doc_bold, fill=(40, 50, 70))
    draw.line([(doc_x + 45, dy + 90), (doc_x + doc_w - 45, dy + 90)], fill=(230, 235, 245), width=1)
    draw.text((doc_x + 45, dy + 105), "Court Mandate: All trial exhibits must bear sequential Bates numbers in the", font=font_doc_body, fill=(80, 90, 105))
    draw.text((doc_x + 45, dy + 125), "bottom-right margin and be filed as segregated PDF attachments.", font=font_doc_body, fill=(80, 90, 105))

    is_stamped = state.get("stamped", False)
    if is_stamped:
        stamp_x = doc_x + doc_w - 220
        stamp_y = doc_y + doc_h - 130
        draw.rounded_rectangle([stamp_x, stamp_y, stamp_x + 190, stamp_y + 32], radius=4, fill=(230, 245, 255), outline=(40, 120, 220), width=1)
        draw.text((stamp_x + 15, stamp_y + 8), "BATES: CASE-2026-000001", font=font_mono_md, fill=(10, 60, 160))

    badge_y = doc_y + doc_h - 85
    if is_stamped:
        draw.rounded_rectangle([doc_x + 20, badge_y, doc_x + doc_w - 20, badge_y + 65], radius=6, fill=(235, 248, 240), outline=(50, 180, 100), width=1)
        draw.text((doc_x + 35, badge_y + 12), "✓ BATES STAMPING & EXHIBIT SPLIT SUCCESSFUL (14.2ms)", font=font_bold, fill=(20, 140, 60))
        draw.text((doc_x + 35, badge_y + 36), "Generated: Exhibit_A_CASE-000001.pdf, Exhibit_B_CASE-000003.pdf, Exhibit_C_CASE-000005.pdf", font=font_doc_body, fill=(30, 100, 50))
    else:
        draw.rounded_rectangle([doc_x + 20, badge_y, doc_x + doc_w - 20, badge_y + 65], radius=6, fill=(255, 248, 235), outline=(220, 150, 50), width=1)
        draw.text((doc_x + 35, badge_y + 12), "⚖ UNSTAMPED MASTER DISCOVERY FILE", font=font_bold, fill=(180, 110, 20))
        draw.text((doc_x + 35, badge_y + 36), "Click 'Apply Bates & Split' to stamp pages and split into court-ready exhibits.", font=font_doc_body, fill=(140, 90, 20))

    return img

def render_scene2_mcp_agent(state):
    """Scene 2: Claude Desktop orchestrating pdf_bates & pdf_split via MCP."""
    img = Image.new("RGB", (WIDTH, HEIGHT), (14, 17, 24))
    draw = ImageDraw.Draw(img)

    draw_top_banner(
        draw,
        "WEEK 02 : MULTI-INTERFACE",
        "Autonomous Legal Clerk: Bates Stamping & Exhibit Splitting",
        "Surface 2/4: AI Agent / MCP — Claude Desktop & Cursor execute discovery pipelines",
        "mcp"
    )

    wx, wy, ww, wh = 40, 80, WIDTH - 80, HEIGHT - 105
    draw_window_chrome(draw, wx, wy, ww, wh, "Claude Desktop — Model Context Protocol (@usepaperpilot/mcp)")

    user_y = wy + 55
    draw.rounded_rectangle([wx + 30, user_y, wx + ww - 30, user_y + 60], radius=8, fill=(32, 38, 52), outline=(50, 60, 85), width=1)
    draw.rounded_rectangle([wx + 45, user_y + 14, wx + 85, user_y + 38], radius=4, fill=(70, 110, 210))
    draw.text((wx + 52, user_y + 18), "USER", font=font_badge, fill=(255, 255, 255))
    prompt = "Bates stamp 'exhibit_bundle.pdf' with prefix 'CASE-2026-', bottom-right corner, then split each exhibit into a standalone PDF."
    draw.text((wx + 100, user_y + 20), prompt, font=font_bold, fill=(240, 245, 255))

    claude_y = user_y + 75
    draw.rounded_rectangle([wx + 30, claude_y, wx + 100, claude_y + 24], radius=4, fill=(217, 119, 87))
    draw.text((wx + 38, claude_y + 4), "CLAUDE", font=font_badge, fill=(255, 255, 255))
    draw.text((wx + 115, claude_y + 3), "I'll execute this using PaperPilot's native pdf_bates and pdf_split tools.", font=font_body, fill=(180, 195, 215))

    # MCP Tool 1 Card
    tool_y = claude_y + 36
    draw.rounded_rectangle([wx + 30, tool_y, wx + ww - 30, tool_y + 110], radius=8, fill=(22, 28, 40), outline=(55, 75, 115), width=1)
    draw.text((wx + 45, tool_y + 12), "⚙️ MCP Tool Call: paperpilot.pdf_bates", font=font_mono_lg, fill=(120, 200, 255))
    t1_status = "✓ COMPLETED IN 14ms" if state.get("tool1_done", False) else "EXECUTING..."
    t1_color = (80, 220, 140) if state.get("tool1_done", False) else (255, 180, 60)
    draw.text((wx + ww - 240, tool_y + 12), t1_status, font=font_mono_sm, fill=t1_color)

    draw.text((wx + 45, tool_y + 38), 'Arguments: {"input": "exhibit_bundle.pdf", "prefix": "CASE-2026-", "start": 1, "pos": "bottom-right"}', font=font_mono_sm, fill=(210, 225, 245))
    draw.text((wx + 45, tool_y + 64), 'Output:    ✓ Generated exhibit_bundle_stamped.pdf [Pages 1..6 stamped successfully]', font=font_mono_sm, fill=(80, 220, 140))

    # MCP Tool 2 Card
    tool2_y = tool_y + 125
    draw.rounded_rectangle([wx + 30, tool2_y, wx + ww - 30, tool2_y + 110], radius=8, fill=(22, 28, 40), outline=(55, 75, 115), width=1)
    draw.text((wx + 45, tool2_y + 12), "⚙️ MCP Tool Call: paperpilot.pdf_split", font=font_mono_lg, fill=(120, 200, 255))
    t2_status = "✓ COMPLETED IN 9ms" if state.get("tool2_done", False) else "EXECUTING..."
    t2_color = (80, 220, 140) if state.get("tool2_done", False) else (255, 180, 60)
    draw.text((wx + ww - 240, tool2_y + 12), t2_status, font=font_mono_sm, fill=t2_color)

    draw.text((wx + 45, tool2_y + 38), 'Arguments: {"input": "exhibit_bundle_stamped.pdf", "ranges": ["1-2", "3-4", "5-6"], "output_dir": "./exhibits/"}', font=font_mono_sm, fill=(210, 225, 245))
    draw.text((wx + 45, tool2_y + 64), 'Output:    ✓ Created Exhibit_A (pp 1-2), Exhibit_B (pp 3-4), Exhibit_C (pp 5-6)', font=font_mono_sm, fill=(80, 220, 140))

    return img

def render_scene3_cli_api(state):
    """Scene 3: Dual Terminal & REST API execution."""
    img = Image.new("RGB", (WIDTH, HEIGHT), (14, 17, 24))
    draw = ImageDraw.Draw(img)

    draw_top_banner(
        draw,
        "WEEK 02 : MULTI-INTERFACE",
        "Autonomous Legal Clerk: Bates Stamping & Exhibit Splitting",
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
    cmd1 = "paperpilot bates -i exhibit_bundle.pdf \\"
    draw.text((cx + 20, cy), cmd1, font=font_mono_sm, fill=(240, 245, 255))
    cy += 20
    cmd2 = "  -o bundle_stamped.pdf --prefix 'CASE-2026-'"
    draw.text((cx + 20, cy), cmd2, font=font_mono_sm, fill=(240, 245, 255))
    cy += 26
    draw.text((cx + 20, cy), "✓ Bates stamped 6 pages in 14.1ms", font=font_mono_sm, fill=(80, 220, 140))
    draw.text((cx + 20, cy + 18), "  Range: CASE-2026-000001 to CASE-2026-000006", font=font_mono_sm, fill=(140, 160, 190))
    cy += 48

    draw.text((cx, cy), prompt, font=font_mono_md, fill=(80, 220, 120))
    cmd3 = "paperpilot split -i bundle_stamped.pdf -r 1-2,3-4,5-6"
    draw.text((cx + 20, cy), cmd3, font=font_mono_sm, fill=(240, 245, 255))
    cy += 24
    draw.text((cx + 20, cy), "✓ Split into 3 legal exhibits in 8.6ms", font=font_mono_sm, fill=(80, 220, 140))
    draw.text((cx + 20, cy + 18), "  Exhibits ready for trial deposition filing.", font=font_mono_sm, fill=(100, 200, 255))

    # Right: REST API Window
    rx = wx + half_w + 20
    draw_window_chrome(draw, rx, wy, half_w, wh, "HTTP REST API — http://localhost:8080")
    rcx = rx + 20
    rcy = wy + 55

    draw.text((rcx, rcy), "# Python Requests / FastAPI Gateway", font=font_mono_sm, fill=(120, 140, 170))
    rcy += 24
    api_code = [
        'import requests',
        '',
        'resp = requests.post(',
        '  "http://localhost:8080/api/v1/bates",',
        '  json={',
        '    "input_path": "exhibit_bundle.pdf",',
        '    "prefix": "CASE-2026-",',
        '    "start": 1,',
        '    "position": "bottom-right"',
        '  }',
        ')'
    ]
    for line in api_code:
        draw.text((rcx, rcy), line, font=font_mono_sm, fill=(220, 230, 250))
        rcy += 18

    rcy += 15
    draw.rounded_rectangle([rcx, rcy, rx + half_w - 20, rcy + 105], radius=6, fill=(18, 22, 32), outline=(40, 50, 70), width=1)
    draw.text((rcx + 12, rcy + 10), "HTTP/1.1 200 OK  (Latency: 4.1ms)", font=font_mono_sm, fill=(80, 220, 140))
    draw.text((rcx + 12, rcy + 32), '{', font=font_mono_sm, fill=(180, 195, 215))
    draw.text((rcx + 12, rcy + 50), '  "status": "success", "pages_stamped": 6,', font=font_mono_sm, fill=(180, 195, 215))
    draw.text((rcx + 12, rcy + 68), '  "execution_ms": 4.12,', font=font_mono_sm, fill=(255, 215, 0))
    draw.text((rcx + 12, rcy + 86), '  "exhibits_split": 3', font=font_mono_sm, fill=(100, 200, 255))
    draw.text((rcx + 12, rcy + 104), '}', font=font_mono_sm, fill=(180, 195, 215))

    return img

def render_scene4_verified_result(state):
    """Scene 4: Cryptographic Verification & Multi-Surface Summary."""
    img = Image.new("RGB", (WIDTH, HEIGHT), (14, 17, 24))
    draw = ImageDraw.Draw(img)

    draw_top_banner(
        draw,
        "WEEK 02 : MULTI-INTERFACE",
        "Autonomous Legal Clerk: Bates Stamping & Exhibit Splitting",
        "Surface 4/4: Unified Discovery Delivery — Deterministic legal discovery across all surfaces",
        "result"
    )

    wx, wy, ww, wh = 40, 80, WIDTH - 80, HEIGHT - 105
    draw_window_chrome(draw, wx, wy, ww, wh, "PaperPilot Engine — Multi-Interface Architecture Overview")

    card_y = wy + 60
    draw.rounded_rectangle([wx + 40, card_y, wx + ww - 40, card_y + 110], radius=8, fill=(22, 36, 30), outline=(45, 140, 80), width=2)
    draw.text((wx + 60, card_y + 18), "⚖️ COURT-MANDATED DISCOVERY BUNDLE: PREPARED & AUDITED", font=font_title, fill=(80, 230, 140))
    draw.text((wx + 60, card_y + 52), "Sequential Range: CASE-2026-000001 through CASE-2026-000006  (3 Distinct Exhibits)", font=font_mono_md, fill=(180, 240, 200))
    draw.text((wx + 60, card_y + 76), "Federal Rules of Civil Procedure (FRCP Rule 34 & 37) & Air-Gapped Legal Compliance", font=font_body, fill=(140, 200, 160))

    col_y = card_y + 130
    col_w = (ww - 80 - 45) // 4
    col_h = wh - 220

    surfaces = [
        ("🖥️ Desktop GUI", "Litigators & Paralegals", "1-Click Visual Stamp", "Sub-20ms", (60, 110, 210)),
        ("🤖 MCP Agent", "Claude & Cursor", "Autonomous Legal Clerk", "Sub-15ms", (217, 119, 87)),
        ("💻 Terminal CLI", "Litigation Support", "Batch Discovery Scripts", "Sub-12ms", (80, 180, 120)),
        ("🌐 REST API", "E-Discovery Systems", "Sub-5ms HTTP Webhook", "Sub-5ms", (180, 90, 220)),
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
    frames_dir = "/tmp/week02_frames"
    if os.path.exists(frames_dir):
        shutil.rmtree(frames_dir)
    os.makedirs(frames_dir)

    print("Generating Week 02 Multi-Interface Showcase frames...")

    frame_count = 0

    # Scene 1: Desktop GUI (18 frames)
    for _ in range(8):
        img = render_scene1_desktop_gui({"stamped": False, "btn_clicked": False})
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1
    for _ in range(10):
        img = render_scene1_desktop_gui({"stamped": True, "btn_clicked": True})
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    # Scene 2: AI Agent / MCP (16 frames)
    for i in range(16):
        img = render_scene2_mcp_agent({"tool1_done": (i >= 4), "tool2_done": (i >= 10)})
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

    palette_path = "/tmp/week02_palette.png"
    out_gif = "docs/assets/showcase/week02-mcp-agent-bates-split.gif"

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
