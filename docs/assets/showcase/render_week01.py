#!/usr/bin/env python3
"""
Generate crisp, pixel-perfect animated frames and convert to palette-optimized GIF
for Week 01: Multi-Interface Showcase - Zero-Leak LLM Pipeline: Irreversible PII Redaction.

4-Scene Progression:
  Scene 1: Desktop GUI (Visual PDF Viewer, Redaction toolbar, 1-Click "Sanitize PII" button click)
  Scene 2: AI Agent / MCP (Claude Desktop JSON tool call & arguments execution)
  Scene 3: CLI & REST API (Sub-15ms terminal execution & sub-5ms curl/requests payload)
  Scene 4: Cryptographic Verification (Sanitized PDF output, 0 PII bytes, immutable SHA-256 seal)
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
    # Header bar
    draw.rounded_rectangle([x, y, x + w, y + h], radius=8, fill=(24, 28, 38), outline=(45, 52, 70), width=1)
    # Header top
    draw.rectangle([x, y, x + w, y + 40], fill=(28, 33, 46))
    # Window traffic light buttons
    draw.ellipse([x + 14, y + 14, x + 24, y + 24], fill=(255, 95, 87)) # red
    draw.ellipse([x + 30, y + 14, x + 40, y + 24], fill=(254, 188, 46)) # yellow
    draw.ellipse([x + 46, y + 14, x + 56, y + 24], fill=(40, 201, 64)) # green
    
    # Title or tabs
    if tabs:
        cur_tab_x = x + 70
        for i, tab in enumerate(tabs):
            tab_w = 175
            is_active = (i == active_tab_idx)
            if is_active:
                draw.rounded_rectangle([cur_tab_x, y + 8, cur_tab_x + tab_w, y + 40], radius=5, fill=(36, 44, 62))
                draw.rectangle([cur_tab_x, y + 34, cur_tab_x + tab_w, y + 40], fill=(36, 44, 62))
                # Active indicator bar
                draw.line([(cur_tab_x + 8, y + 38), (cur_tab_x + tab_w - 8, y + 38)], fill=(80, 160, 255), width=2)
                draw.text((cur_tab_x + 16, y + 14), tab, font=font_tab, fill=(255, 255, 255))
            else:
                draw.text((cur_tab_x + 16, y + 14), tab, font=font_tab, fill=(130, 145, 170))
            cur_tab_x += tab_w + 10
    else:
        draw.text((x + 70, y + 13), title, font=font_mono_sm, fill=(185, 195, 215))

def draw_top_banner(draw, week_badge, title_text, subtitle_text, active_surface):
    # Pill Badge
    draw.rounded_rectangle([40, 16, 260, 42], radius=13, fill=(35, 45, 68), outline=(65, 95, 160), width=1)
    draw.text((55, 22), week_badge, font=font_badge, fill=(100, 185, 255))

    # Title
    draw.text((280, 18), title_text, font=font_title, fill=(255, 255, 255))
    draw.text((40, 48), subtitle_text, font=font_subtitle, fill=(140, 155, 180))

    # Surface Selector Pills (Right aligned)
    surfaces = [
        ("🖥️ Desktop GUI", active_surface == "gui"),
        ("🤖 MCP Agent", active_surface == "mcp"),
        ("💻 CLI / API", active_surface == "cli"),
        ("🛡️ Verified Result", active_surface == "result")
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
    """Scene 1: Sleek Desktop GUI with visual viewer, toolbar & 1-click button click."""
    img = Image.new("RGB", (WIDTH, HEIGHT), (14, 17, 24))
    draw = ImageDraw.Draw(img)

    draw_top_banner(
        draw,
        "WEEK 01 : MULTI-INTERFACE",
        "Same Engine, Your Interface: Zero-Leak PII Redaction",
        "Surface 1/4: PaperPilot Desktop GUI — Drag-and-drop visual redaction with 1 click",
        "gui"
    )

    # Main Desktop Window
    wx, wy, ww, wh = 40, 80, WIDTH - 80, HEIGHT - 105
    tabs = ["📄 contract.pdf", "⚙️ Redaction Rules", "🔍 Forensic Diff", "📊 Metadata"]
    draw_window_chrome(draw, wx, wy, ww, wh, "", active_tab_idx=0, tabs=tabs)

    # Desktop Body Split: Left Sidebar Dock (280px), Right PDF Viewer Canvas
    sb_w = 290
    draw.rounded_rectangle([wx + 10, wy + 48, wx + sb_w, wy + wh - 12], radius=6, fill=(20, 24, 34), outline=(40, 48, 65), width=1)

    # Sidebar Header
    draw.text((wx + 25, wy + 62), "SECURITY & PRIVACY TOOLS", font=font_bold, fill=(120, 170, 255))
    draw.line([(wx + 25, wy + 84), (wx + sb_w - 15, wy + 84)], fill=(40, 48, 65), width=1)

    # Options in Sidebar
    y_opt = wy + 98
    opts = [
        ("Surgical Byte Incineration", True),
        ("Auto-Detect SSN / Tax IDs", True),
        ("Auto-Detect Credit Cards", True),
        ("Sanitize Direct Emails", True),
        ("Deterministic SHA-256 Seal", True),
    ]
    for opt_text, checked in opts:
        # Checkbox
        box_c = (50, 140, 240) if checked else (60, 70, 90)
        draw.rounded_rectangle([wx + 25, y_opt + 2, wx + 41, y_opt + 18], radius=3, fill=box_c)
        draw.text((wx + 28, y_opt + 1), "✓", font=font_badge, fill=(255, 255, 255))
        draw.text((wx + 50, y_opt + 2), opt_text, font=font_doc_body, fill=(210, 220, 235))
        y_opt += 32

    # Action Button in Sidebar
    btn_y = wy + wh - 110
    btn_clicked = state.get("btn_clicked", False)
    btn_fill = (40, 180, 100) if btn_clicked else (45, 110, 235)
    draw.rounded_rectangle([wx + 25, btn_y, wx + sb_w - 15, btn_y + 44], radius=6, fill=btn_fill)
    btn_text = "✓ Sanitize Applied (12.4ms)" if btn_clicked else "⚡ Sanitize Document (1-Click)"
    draw.text((wx + 42, btn_y + 13), btn_text, font=font_bold, fill=(255, 255, 255))

    # Engine status pill
    draw.rounded_rectangle([wx + 25, btn_y + 54, wx + sb_w - 15, btn_y + 80], radius=4, fill=(26, 32, 46))
    draw.text((wx + 35, btn_y + 60), "Local Engine: Rust 0.8.1 (0 Cloud Egress)", font=font_mono_sm, fill=(90, 190, 130))

    # Right Canvas: Document Viewer
    doc_x = wx + sb_w + 15
    doc_w = ww - sb_w - 25
    doc_y = wy + 48
    doc_h = wh - 60
    draw.rounded_rectangle([doc_x, doc_y, doc_x + doc_w, doc_y + doc_h], radius=6, fill=(248, 250, 254), outline=(50, 60, 80), width=1)

    # Document Header
    draw.text((doc_x + 30, doc_y + 20), "CONSULTING AGREEMENT & STATEMENT OF WORK", font=font_doc_title, fill=(20, 30, 45))
    draw.line([(doc_x + 30, doc_y + 45), (doc_x + doc_w - 30, doc_y + 45)], fill=(215, 220, 230), width=1)

    dy = doc_y + 60
    draw.text((doc_x + 30, dy), "Client: Apex Global Technologies LLC", font=font_doc_body, fill=(60, 70, 85))
    dy += 24
    draw.text((doc_x + 30, dy), "Consultant: Sarah M. Jenkins", font=font_doc_body, fill=(60, 70, 85))
    dy += 34

    is_redacted = state.get("redacted", False)

    # Item 1: SSN
    draw.text((doc_x + 30, dy), "Social Security Number:", font=font_doc_bold, fill=(30, 40, 55))
    val_x = doc_x + 210
    if is_redacted:
        draw.rectangle([val_x, dy - 2, val_x + 130, dy + 18], fill=(12, 14, 20))
        draw.text((val_x + 15, dy + 2), "[REDACTED]", font=font_badge, fill=(255, 255, 255))
    else:
        draw.text((val_x, dy), "123-45-6789", font=font_doc_bold, fill=(220, 40, 40))
        draw.rectangle([val_x - 4, dy - 2, val_x + 110, dy + 18], outline=(230, 80, 80), width=1)

    dy += 32
    # Item 2: Card
    draw.text((doc_x + 30, dy), "Corporate Card:", font=font_doc_bold, fill=(30, 40, 55))
    val_x = doc_x + 210
    if is_redacted:
        draw.rectangle([val_x, dy - 2, val_x + 180, dy + 18], fill=(12, 14, 20))
        draw.text((val_x + 35, dy + 2), "[REDACTED]", font=font_badge, fill=(255, 255, 255))
    else:
        draw.text((val_x, dy), "4532-8921-3847-9201", font=font_doc_bold, fill=(220, 40, 40))
        draw.rectangle([val_x - 4, dy - 2, val_x + 170, dy + 18], outline=(230, 80, 80), width=1)

    dy += 32
    # Item 3: Email
    draw.text((doc_x + 30, dy), "Direct Contact Email:", font=font_doc_bold, fill=(30, 40, 55))
    val_x = doc_x + 210
    if is_redacted:
        draw.rectangle([val_x, dy - 2, val_x + 230, dy + 18], fill=(12, 14, 20))
        draw.text((val_x + 55, dy + 2), "[REDACTED]", font=font_badge, fill=(255, 255, 255))
    else:
        draw.text((val_x, dy), "sarah.jenkins@apextech.internal", font=font_doc_body, fill=(60, 70, 85))

    # Bottom Document Notification Banner
    badge_y = doc_y + doc_h - 90
    if is_redacted:
        draw.rounded_rectangle([doc_x + 20, badge_y, doc_x + doc_w - 20, badge_y + 70], radius=6, fill=(235, 248, 240), outline=(50, 180, 100), width=1)
        draw.text((doc_x + 35, badge_y + 14), "✓ DESKTOP ENGINE: STREAM BYTE REMOVAL COMPLETE", font=font_bold, fill=(20, 140, 60))
        draw.text((doc_x + 35, badge_y + 38), "Underlying font glyphs permanently erased in 12.4ms. SHA-256 logged.", font=font_doc_body, fill=(30, 100, 50))
    else:
        draw.rounded_rectangle([doc_x + 20, badge_y, doc_x + doc_w - 20, badge_y + 70], radius=6, fill=(255, 242, 242), outline=(220, 60, 60), width=1)
        draw.text((doc_x + 35, badge_y + 14), "⚠ RAW PII EXPOSED: Click 'Sanitize Document' to burn bytes", font=font_bold, fill=(200, 30, 30))
        draw.text((doc_x + 35, badge_y + 38), "Plaintext SSN and credit cards detected in document stream.", font=font_doc_body, fill=(160, 30, 30))

    return img

def render_scene2_mcp_agent(state):
    """Scene 2: AI Agent / MCP execution in Claude Desktop / Cursor."""
    img = Image.new("RGB", (WIDTH, HEIGHT), (14, 17, 24))
    draw = ImageDraw.Draw(img)

    draw_top_banner(
        draw,
        "WEEK 01 : MULTI-INTERFACE",
        "Same Engine, Your Interface: Zero-Leak PII Redaction",
        "Surface 2/4: AI Agent / MCP — Claude Desktop & Cursor orchestrate local redaction",
        "mcp"
    )

    wx, wy, ww, wh = 40, 80, WIDTH - 80, HEIGHT - 105
    draw_window_chrome(draw, wx, wy, ww, wh, "Claude Desktop — Model Context Protocol (PaperPilot Local Server Active)")

    # User message
    user_y = wy + 55
    draw.rounded_rectangle([wx + 30, user_y, wx + ww - 30, user_y + 60], radius=8, fill=(32, 38, 52), outline=(50, 60, 85), width=1)
    draw.rounded_rectangle([wx + 45, user_y + 14, wx + 85, user_y + 38], radius=4, fill=(70, 110, 210))
    draw.text((wx + 52, user_y + 18), "USER", font=font_badge, fill=(255, 255, 255))
    prompt = "Sanitize all PII (SSN, credit card, internal email) from client_agreement.pdf before RAG ingestion."
    draw.text((wx + 100, user_y + 20), prompt, font=font_bold, fill=(240, 245, 255))

    # Claude response
    claude_y = user_y + 75
    draw.rounded_rectangle([wx + 30, claude_y, wx + 100, claude_y + 24], radius=4, fill=(217, 119, 87)) # Anthropic terracotta
    draw.text((wx + 38, claude_y + 4), "CLAUDE", font=font_badge, fill=(255, 255, 255))
    draw.text((wx + 115, claude_y + 3), "I'll invoke PaperPilot's local zero-leak redaction tool over MCP.", font=font_body, fill=(180, 195, 215))

    # MCP Tool Call Card
    tool_y = claude_y + 36
    draw.rounded_rectangle([wx + 30, tool_y, wx + ww - 30, tool_y + 200], radius=8, fill=(22, 28, 40), outline=(55, 75, 115), width=1)
    
    # Tool header
    draw.text((wx + 45, tool_y + 12), "⚙️ MCP Tool Call: paperpilot.pdf_redact", font=font_mono_lg, fill=(120, 200, 255))
    t_status = "✓ COMPLETED IN 12.4ms" if state.get("tool_done", False) else "EXECUTING..."
    t_color = (80, 220, 140) if state.get("tool_done", False) else (255, 180, 60)
    draw.text((wx + ww - 240, tool_y + 12), t_status, font=font_mono_sm, fill=t_color)

    # Tool JSON Arguments
    args_lines = [
        '{',
        '  "input": "client_agreement.pdf",',
        '  "output": "agreement_sanitized.pdf",',
        '  "patterns": ["ssn", "credit_card", "email"],',
        '  "mode": "stream_incineration",',
        '  "compute_hash": true',
        '}'
    ]
    cur_j_y = tool_y + 38
    for line in args_lines:
        draw.text((wx + 45, cur_j_y), line, font=font_mono_sm, fill=(210, 225, 245))
        cur_j_y += 18

    # MCP Tool Output
    out_y = tool_y + 158
    draw.line([(wx + 30, out_y), (wx + ww - 30, out_y)], fill=(45, 55, 80), width=1)
    draw.text((wx + 45, out_y + 10), "Response:", font=font_mono_sm, fill=(130, 145, 175))
    draw.text((wx + 130, out_y + 10), '{"status": "success", "redacted_count": 3, "sha256": "4f9a88...c1e3", "bytes_erased": 84}', font=font_mono_sm, fill=(80, 220, 140))

    # Claude follow-up
    res_box_y = tool_y + 215
    draw.rounded_rectangle([wx + 30, res_box_y, wx + ww - 30, res_box_y + 70], radius=6, fill=(26, 34, 48), outline=(45, 60, 90), width=1)
    draw.text((wx + 45, res_box_y + 12), "Claude: All sensitive PII bytes have been excised from the PDF content stream.", font=font_body, fill=(240, 245, 255))
    draw.text((wx + 45, res_box_y + 36), "The output file agreement_sanitized.pdf is cryptographically safe for Vector DB ingestion.", font=font_body, fill=(120, 210, 160))

    return img

def render_scene3_cli_api(state):
    """Scene 3: Dual Terminal & REST API execution."""
    img = Image.new("RGB", (WIDTH, HEIGHT), (14, 17, 24))
    draw = ImageDraw.Draw(img)

    draw_top_banner(
        draw,
        "WEEK 01 : MULTI-INTERFACE",
        "Same Engine, Your Interface: Zero-Leak PII Redaction",
        "Surface 3/4: Terminal CLI & REST API — Sub-15ms native command & sub-5ms microservice",
        "cli"
    )

    wx, wy, ww, wh = 40, 80, WIDTH - 80, HEIGHT - 105

    # Split screen: Left = Terminal CLI, Right = Python REST API
    half_w = (ww - 20) // 2

    # Left: CLI Window
    draw_window_chrome(draw, wx, wy, half_w, wh, "zsh — paperpilot CLI (Rust Native)")
    cx = wx + 20
    cy = wy + 55
    prompt = "$ "
    draw.text((cx, cy), prompt, font=font_mono_md, fill=(80, 220, 120))
    cmd1 = "paperpilot redact -i agreement.pdf \\"
    draw.text((cx + 20, cy), cmd1, font=font_mono_sm, fill=(240, 245, 255))
    cy += 20
    cmd2 = "  -o safe.pdf --regex '\\d{3}-\\d{2}-\\d{4}'"
    draw.text((cx + 20, cy), cmd2, font=font_mono_sm, fill=(240, 245, 255))
    cy += 26
    draw.text((cx + 20, cy), "✓ Stream redaction completed in 12.4ms", font=font_mono_sm, fill=(80, 220, 140))
    draw.text((cx + 20, cy + 18), "  Excised: 3 text objects, 0 raw glyphs remaining", font=font_mono_sm, fill=(140, 160, 190))
    cy += 48

    draw.text((cx, cy), prompt, font=font_mono_md, fill=(80, 220, 120))
    cmd3 = "paperpilot hash -i safe.pdf"
    draw.text((cx + 20, cy), cmd3, font=font_mono_sm, fill=(240, 245, 255))
    cy += 24
    draw.text((cx + 20, cy), "SHA-256: 4f9a88b14e2...c1e3 (VERIFIED)", font=font_mono_sm, fill=(100, 200, 255))
    cy += 40

    draw.text((cx, cy), prompt, font=font_mono_md, fill=(80, 220, 120))
    cmd4 = "paperpilot extract-text -i safe.pdf | grep SSN"
    draw.text((cx + 20, cy), cmd4, font=font_mono_sm, fill=(240, 245, 255))
    cy += 24
    draw.text((cx + 20, cy), "[0 matches found — byte stream 100% sanitized]", font=font_mono_sm, fill=(255, 120, 120))

    # Right: REST API Window
    rx = wx + half_w + 20
    draw_window_chrome(draw, rx, wy, half_w, wh, "HTTP REST API — http://localhost:8080")
    rcx = rx + 20
    rcy = wy + 55

    draw.text((rcx, rcy), "# Python Requests or curl payload", font=font_mono_sm, fill=(120, 140, 170))
    rcy += 24
    api_code = [
        'import requests',
        '',
        'resp = requests.post(',
        '  "http://localhost:8080/api/v1/redact",',
        '  json={',
        '    "input_path": "contract.pdf",',
        '    "output_path": "safe.pdf",',
        '    "regex": r"\\d{3}-\\d{2}-\\d{4}"',
        '  }',
        ')'
    ]
    for line in api_code:
        draw.text((rcx, rcy), line, font=font_mono_sm, fill=(220, 230, 250))
        rcy += 18

    rcy += 15
    draw.rounded_rectangle([rcx, rcy, rx + half_w - 20, rcy + 110], radius=6, fill=(18, 22, 32), outline=(40, 50, 70), width=1)
    draw.text((rcx + 12, rcy + 10), "HTTP/1.1 200 OK  (Latency: 3.8ms)", font=font_mono_sm, fill=(80, 220, 140))
    draw.text((rcx + 12, rcy + 32), '{', font=font_mono_sm, fill=(180, 195, 215))
    draw.text((rcx + 12, rcy + 50), '  "status": "success",', font=font_mono_sm, fill=(180, 195, 215))
    draw.text((rcx + 12, rcy + 68), '  "execution_ms": 3.82,', font=font_mono_sm, fill=(255, 215, 0))
    draw.text((rcx + 12, rcy + 86), '  "sha256": "4f9a88...c1e3"', font=font_mono_sm, fill=(100, 200, 255))
    draw.text((rcx + 12, rcy + 104), '}', font=font_mono_sm, fill=(180, 195, 215))

    return img

def render_scene4_verified_result(state):
    """Scene 4: Cryptographic Verification & Multi-Surface Summary."""
    img = Image.new("RGB", (WIDTH, HEIGHT), (14, 17, 24))
    draw = ImageDraw.Draw(img)

    draw_top_banner(
        draw,
        "WEEK 01 : MULTI-INTERFACE",
        "Same Engine, Your Interface: Zero-Leak PII Redaction",
        "Surface 4/4: Unified Verification — Immutable cryptographic proof across all surfaces",
        "result"
    )

    wx, wy, ww, wh = 40, 80, WIDTH - 80, HEIGHT - 105
    draw_window_chrome(draw, wx, wy, ww, wh, "PaperPilot Engine — Multi-Interface Architecture Overview")

    # Center Hero Card: Chain of Custody & Security Verification
    card_y = wy + 60
    draw.rounded_rectangle([wx + 40, card_y, wx + ww - 40, card_y + 110], radius=8, fill=(22, 36, 30), outline=(45, 140, 80), width=2)
    draw.text((wx + 60, card_y + 18), "🛡️ ZERO-LEAK SECURITY CERTIFICATE: VERIFIED", font=font_title, fill=(80, 230, 140))
    draw.text((wx + 60, card_y + 52), "SHA-256 Audit Seal:  4f9a88b14e21d609c13589cda01344ef6a43878b...c1e3", font=font_mono_md, fill=(180, 240, 200))
    draw.text((wx + 60, card_y + 76), "Compliance Status: SOC 2 Type II, HIPAA, GDPR Art. 17 & ISO 27001 Compliant", font=font_body, fill=(140, 200, 160))

    # 4 Surface Columns Comparison
    col_y = card_y + 130
    col_w = (ww - 80 - 45) // 4
    col_h = wh - 220

    surfaces = [
        ("🖥️ Desktop GUI", "Lawyers & Compliance", "1-Click Visual Redact", "Sub-20ms", (60, 110, 210)),
        ("🤖 MCP Agent", "Claude & Cursor", "Autonomous Tool Call", "Sub-15ms", (217, 119, 87)),
        ("💻 Terminal CLI", "DevOps & SREs", "High-Volume Scripts", "Sub-12ms", (80, 180, 120)),
        ("🌐 REST API", "Microservices & n8n", "Sub-5ms Webhook JSON", "Sub-5ms", (180, 90, 220)),
    ]

    for i, (s_title, s_target, s_mode, s_speed, s_accent) in enumerate(surfaces):
        cx = wx + 40 + i * (col_w + 15)
        draw.rounded_rectangle([cx, col_y, cx + col_w, col_y + col_h], radius=6, fill=(20, 25, 36), outline=(40, 50, 72), width=1)
        # Accent top bar
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

    # Bottom Footer Callout
    foot_y = wy + wh - 45
    draw.text((wx + 40, foot_y), "Open-Source & Local-First in Pure Rust  •  https://github.com/usepaperpilot/paperpilot", font=font_mono_sm, fill=(130, 150, 180))

    return img

def main():
    frames_dir = "/tmp/week01_frames"
    if os.path.exists(frames_dir):
        shutil.rmtree(frames_dir)
    os.makedirs(frames_dir)

    print("Generating Week 01 Multi-Interface Showcase frames...")

    frame_count = 0

    # Scene 1: Desktop GUI (Frames 0..17, ~1.8s)
    # Sub-phase A: Initial vulnerable document (8 frames)
    for _ in range(8):
        img = render_scene1_desktop_gui({"redacted": False, "btn_clicked": False})
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    # Sub-phase B: Button click and instant redaction (10 frames)
    for _ in range(10):
        img = render_scene1_desktop_gui({"redacted": True, "btn_clicked": True})
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    # Scene 2: AI Agent / MCP (Frames 18..33, ~1.6s)
    for i in range(16):
        is_done = (i >= 6)
        img = render_scene2_mcp_agent({"tool_done": is_done})
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    # Scene 3: CLI & REST API (Frames 34..49, ~1.6s)
    for _ in range(16):
        img = render_scene3_cli_api({})
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    # Scene 4: Cryptographic Verification & Multi-Surface Summary (Frames 50..67, ~1.8s)
    for _ in range(18):
        img = render_scene4_verified_result({})
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    print(f"Rendered {frame_count} frames. Encoding GIF via ffmpeg palette...")

    palette_path = "/tmp/week01_palette.png"
    out_gif = "docs/assets/showcase/week01-zero-leak-redaction.gif"

    # Step 1: generate palette
    cmd1 = [
        "ffmpeg", "-y", "-framerate", str(FPS), "-i", f"{frames_dir}/frame_%04d.png",
        "-vf", "palettegen=max_colors=256:stats_mode=diff",
        palette_path
    ]
    subprocess.run(cmd1, check=True)

    # Step 2: render palette-optimized gif
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
