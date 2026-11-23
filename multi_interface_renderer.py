import os
import sys
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
    draw.rounded_rectangle([40, 16, 260, 42], radius=13, fill=(35, 45, 68), outline=(65, 95, 160), width=1)
    draw.text((55, 22), week_badge, font=font_badge, fill=(100, 185, 255))
    draw.text((280, 18), title_text, font=font_title, fill=(255, 255, 255))
    draw.text((40, 48), subtitle_text, font=font_subtitle, fill=(140, 155, 180))

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

def render_scene1(draw, img, cfg, state):
    draw_top_banner(draw, cfg["week_badge"], cfg["title"], cfg["sub1"], "gui")
    wx, wy, ww, wh = 40, 80, WIDTH - 80, HEIGHT - 105
    tabs = [f"📄 {cfg['gui_tab']}", "⚙️ Tool Settings", "🔍 Inspector", "📊 Audit Log"]
    draw_window_chrome(draw, wx, wy, ww, wh, "", active_tab_idx=0, tabs=tabs)

    sb_w = 310
    draw.rounded_rectangle([wx + 10, wy + 48, wx + sb_w, wy + wh - 12], radius=6, fill=(20, 24, 34), outline=(40, 48, 65), width=1)
    draw.text((wx + 25, wy + 62), cfg["gui_left_title"], font=font_bold, fill=(120, 170, 255))
    draw.line([(wx + 25, wy + 84), (wx + sb_w - 15, wy + 84)], fill=(40, 48, 65), width=1)

    y_opt = wy + 98
    for lbl, val in cfg["gui_options"]:
        draw.text((wx + 25, y_opt), lbl, font=font_doc_body, fill=(140, 155, 175))
        draw.rounded_rectangle([wx + 25, y_opt + 18, wx + sb_w - 20, y_opt + 40], radius=4, fill=(28, 34, 48), outline=(50, 60, 80), width=1)
        draw.text((wx + 35, y_opt + 23), val, font=font_mono_sm, fill=(230, 240, 255))
        y_opt += 48

    btn_y = wy + wh - 110
    btn_clicked = state.get("btn_clicked", False)
    btn_fill = (40, 180, 100) if btn_clicked else (45, 110, 235)
    draw.rounded_rectangle([wx + 25, btn_y, wx + sb_w - 15, btn_y + 44], radius=6, fill=btn_fill)
    btn_text = cfg["gui_btn_done"] if btn_clicked else cfg["gui_btn"]
    draw.text((wx + 38, btn_y + 13), btn_text, font=font_bold, fill=(255, 255, 255))

    draw.rounded_rectangle([wx + 25, btn_y + 54, wx + sb_w - 15, btn_y + 80], radius=4, fill=(26, 32, 46))
    draw.text((wx + 35, btn_y + 60), "Local Engine: 100% Air-Gapped / Offline", font=font_mono_sm, fill=(90, 190, 130))

    # Right Canvas
    doc_x = wx + sb_w + 15
    doc_w = ww - sb_w - 25
    doc_y = wy + 48
    doc_h = wh - 60
    draw.rounded_rectangle([doc_x, doc_y, doc_x + doc_w, doc_y + doc_h], radius=6, fill=(248, 250, 254), outline=(50, 60, 80), width=1)
    draw.text((doc_x + 30, doc_y + 20), cfg["gui_doc_title"], font=font_doc_title, fill=(20, 30, 45))
    draw.line([(doc_x + 30, doc_y + 45), (doc_x + doc_w - 30, doc_y + 45)], fill=(215, 220, 230), width=1)

    dy = doc_y + 65
    draw.text((doc_x + 30, dy), cfg["gui_doc_p1"], font=font_doc_bold, fill=(50, 65, 85))
    dy += 28
    draw.text((doc_x + 30, dy), cfg["gui_doc_p2"], font=font_doc_body, fill=(70, 80, 95))
    dy += 36

    # Inner preview card
    draw.rounded_rectangle([doc_x + 30, dy, doc_x + doc_w - 30, dy + 160], radius=6, fill=(255, 255, 255), outline=(210, 220, 235), width=1)
    draw.text((doc_x + 45, dy + 20), "• Native Rust parsing runtime active", font=font_doc_bold, fill=(40, 50, 70))
    draw.text((doc_x + 45, dy + 50), "• Hardware-accelerated byte-stream operations", font=font_doc_body, fill=(60, 75, 95))
    draw.text((doc_x + 45, dy + 80), "• Zero external network or cloud SaaS telemetry", font=font_doc_body, fill=(60, 75, 95))
    draw.text((doc_x + 45, dy + 110), "• Deterministic SHA-256 state tracking", font=font_doc_bold, fill=(40, 100, 160))

    badge_y = doc_y + doc_h - 85
    b_title, b_sub, b_col = cfg["gui_doc_badge_post"] if btn_clicked else cfg["gui_doc_badge_pre"]
    bg_box = (235, 248, 240) if btn_clicked else (255, 248, 235)
    draw.rounded_rectangle([doc_x + 20, badge_y, doc_x + doc_w - 20, badge_y + 65], radius=6, fill=bg_box, outline=b_col, width=1)
    draw.text((doc_x + 35, badge_y + 12), b_title, font=font_bold, fill=b_col)
    draw.text((doc_x + 35, badge_y + 36), b_sub, font=font_doc_body, fill=(70, 85, 100))

def render_scene2(draw, img, cfg, state):
    draw_top_banner(draw, cfg["week_badge"], cfg["title"], cfg["sub2"], "mcp")
    wx, wy, ww, wh = 40, 80, WIDTH - 80, HEIGHT - 105
    draw_window_chrome(draw, wx, wy, ww, wh, "Claude Desktop — Model Context Protocol (@usepaperpilot/mcp)")

    user_y = wy + 55
    draw.rounded_rectangle([wx + 30, user_y, wx + ww - 30, user_y + 60], radius=8, fill=(32, 38, 52), outline=(50, 60, 85), width=1)
    draw.rounded_rectangle([wx + 45, user_y + 14, wx + 85, user_y + 38], radius=4, fill=(70, 110, 210))
    draw.text((wx + 52, user_y + 18), "USER", font=font_badge, fill=(255, 255, 255))
    draw.text((wx + 100, user_y + 20), cfg["mcp_user"], font=font_bold, fill=(240, 245, 255))

    claude_y = user_y + 75
    draw.rounded_rectangle([wx + 30, claude_y, wx + 100, claude_y + 24], radius=4, fill=(217, 119, 87))
    draw.text((wx + 38, claude_y + 4), "CLAUDE", font=font_badge, fill=(255, 255, 255))
    draw.text((wx + 115, claude_y + 3), cfg["mcp_claude"], font=font_body, fill=(180, 195, 215))

    tool_y = claude_y + 36
    draw.rounded_rectangle([wx + 30, tool_y, wx + ww - 30, tool_y + 190], radius=8, fill=(22, 28, 40), outline=(55, 75, 115), width=1)
    draw.text((wx + 45, tool_y + 12), f"⚙️ MCP Tool Call: {cfg['mcp_tool']}", font=font_mono_lg, fill=(120, 200, 255))
    t_status = "✓ COMPLETED IN 12ms" if state.get("tool_done", False) else "EXECUTING..."
    t_color = (80, 220, 140) if state.get("tool_done", False) else (255, 180, 60)
    draw.text((wx + ww - 240, tool_y + 12), t_status, font=font_mono_sm, fill=t_color)

    cur_j_y = tool_y + 36
    for line in cfg["mcp_args"]:
        draw.text((wx + 45, cur_j_y), line, font=font_mono_sm, fill=(210, 225, 245))
        cur_j_y += 17

    out_y = tool_y + 140
    draw.line([(wx + 30, out_y), (wx + ww - 30, out_y)], fill=(45, 55, 80), width=1)
    draw.text((wx + 45, out_y + 10), "Response:", font=font_mono_sm, fill=(130, 145, 175))
    draw.text((wx + 130, out_y + 10), cfg["mcp_res"], font=font_mono_sm, fill=(80, 220, 140))

    res_box_y = tool_y + 205
    draw.rounded_rectangle([wx + 30, res_box_y, wx + ww - 30, res_box_y + 65], radius=6, fill=(26, 34, 48), outline=(45, 60, 90), width=1)
    draw.text((wx + 45, res_box_y + 12), cfg["mcp_summary_1"], font=font_body, fill=(240, 245, 255))
    draw.text((wx + 45, res_box_y + 36), cfg["mcp_summary_2"], font=font_body, fill=(120, 210, 160))

def render_scene3(draw, img, cfg, state):
    draw_top_banner(draw, cfg["week_badge"], cfg["title"], cfg["sub3"], "cli")
    wx, wy, ww, wh = 40, 80, WIDTH - 80, HEIGHT - 105
    half_w = (ww - 20) // 2

    # Left: CLI
    draw_window_chrome(draw, wx, wy, half_w, wh, "zsh — paperpilot CLI (Rust Native)")
    cx = wx + 20
    cy = wy + 55
    draw.text((cx, cy), "$ ", font=font_mono_md, fill=(80, 220, 120))
    draw.text((cx + 20, cy), cfg["cli_cmd1"], font=font_mono_sm, fill=(240, 245, 255))
    cy += 24
    draw.text((cx + 20, cy), cfg["cli_res1"], font=font_mono_sm, fill=(80, 220, 140))
    cy += 48

    draw.text((cx, cy), "$ ", font=font_mono_md, fill=(80, 220, 120))
    draw.text((cx + 20, cy), cfg["cli_cmd2"], font=font_mono_sm, fill=(240, 245, 255))
    cy += 24
    draw.text((cx + 20, cy), cfg["cli_res2"], font=font_mono_sm, fill=(80, 220, 140))

    # Right: REST API
    rx = wx + half_w + 20
    draw_window_chrome(draw, rx, wy, half_w, wh, "HTTP REST API — http://localhost:8080")
    rcx = rx + 20
    rcy = wy + 55

    draw.text((rcx, rcy), "# Python Requests / Local Microservice", font=font_mono_sm, fill=(120, 140, 170))
    rcy += 24
    for line in cfg["api_code"]:
        draw.text((rcx, rcy), line, font=font_mono_sm, fill=(220, 230, 250))
        rcy += 18

    rcy += 15
    draw.rounded_rectangle([rcx, rcy, rx + half_w - 20, rcy + 105], radius=6, fill=(18, 22, 32), outline=(40, 50, 70), width=1)
    draw.text((rcx + 12, rcy + 10), "HTTP/1.1 200 OK  (Latency: 4.2ms)", font=font_mono_sm, fill=(80, 220, 140))
    draw.text((rcx + 12, rcy + 32), "{", font=font_mono_sm, fill=(180, 195, 215))
    draw.text((rcx + 12, rcy + 50), '  "status": "success",', font=font_mono_sm, fill=(180, 195, 215))
    draw.text((rcx + 12, rcy + 68), cfg["api_res"], font=font_mono_sm, fill=(255, 215, 0))
    draw.text((rcx + 12, rcy + 86), "}", font=font_mono_sm, fill=(180, 195, 215))

def render_scene4(draw, img, cfg, state):
    draw_top_banner(draw, cfg["week_badge"], cfg["title"], cfg["sub4"], "result")
    wx, wy, ww, wh = 40, 80, WIDTH - 80, HEIGHT - 105
    draw_window_chrome(draw, wx, wy, ww, wh, "PaperPilot Engine — Multi-Interface Architecture Overview")

    card_y = wy + 60
    draw.rounded_rectangle([wx + 40, card_y, wx + ww - 40, card_y + 110], radius=8, fill=(22, 36, 30), outline=(45, 140, 80), width=2)
    draw.text((wx + 60, card_y + 18), cfg["card_title"], font=font_title, fill=(80, 230, 140))
    draw.text((wx + 60, card_y + 52), cfg["card_sub"], font=font_mono_md, fill=(180, 240, 200))
    draw.text((wx + 60, card_y + 76), cfg["card_detail"], font=font_body, fill=(140, 200, 160))

    col_y = card_y + 130
    col_w = (ww - 80 - 45) // 4
    col_h = wh - 220

    surfaces = [
        ("🖥️ Desktop GUI", "Business & Legal Users", "1-Click Visual Flow", "Sub-20ms", (60, 110, 210)),
        ("🤖 MCP Agent", "Claude & Cursor", "Autonomous Tool Call", "Sub-15ms", (217, 119, 87)),
        ("💻 Terminal CLI", "DevOps & SREs", "High-Volume Pipelines", "Sub-12ms", (80, 180, 120)),
        ("🌐 REST API", "Microservices & n8n", "Sub-5ms Webhook JSON", "Sub-5ms", (180, 90, 220)),
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

def render_week_gif(week_num, cfg):
    frames_dir = f"/tmp/week{week_num:02d}_frames"
    if os.path.exists(frames_dir):
        shutil.rmtree(frames_dir)
    os.makedirs(frames_dir)

    print(f"Generating Week {week_num:02d} Multi-Interface Showcase frames...")

    frame_count = 0

    # Scene 1: Desktop GUI (18 frames)
    for _ in range(8):
        img = Image.new("RGB", (WIDTH, HEIGHT), (14, 17, 24))
        render_scene1(ImageDraw.Draw(img), img, cfg, {"btn_clicked": False})
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1
    for _ in range(10):
        img = Image.new("RGB", (WIDTH, HEIGHT), (14, 17, 24))
        render_scene1(ImageDraw.Draw(img), img, cfg, {"btn_clicked": True})
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    # Scene 2: AI Agent / MCP (16 frames)
    for i in range(16):
        img = Image.new("RGB", (WIDTH, HEIGHT), (14, 17, 24))
        render_scene2(ImageDraw.Draw(img), img, cfg, {"tool_done": (i >= 5)})
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    # Scene 3: CLI & REST API (16 frames)
    for _ in range(16):
        img = Image.new("RGB", (WIDTH, HEIGHT), (14, 17, 24))
        render_scene3(ImageDraw.Draw(img), img, cfg, {})
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    # Scene 4: Verification & Multi-Surface (18 frames)
    for _ in range(18):
        img = Image.new("RGB", (WIDTH, HEIGHT), (14, 17, 24))
        render_scene4(ImageDraw.Draw(img), img, cfg, {})
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    palette_path = f"/tmp/week{week_num:02d}_palette.png"
    out_gif = cfg["out_gif"]

    cmd1 = [
        "ffmpeg", "-y", "-framerate", str(FPS), "-i", f"{frames_dir}/frame_%04d.png",
        "-vf", "palettegen=max_colors=256:stats_mode=diff",
        palette_path
    ]
    subprocess.run(cmd1, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

    cmd2 = [
        "ffmpeg", "-y", "-framerate", str(FPS), "-i", f"{frames_dir}/frame_%04d.png",
        "-i", palette_path,
        "-lavfi", "paletteuse=dither=bayer:bayer_scale=3",
        out_gif
    ]
    subprocess.run(cmd2, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

    size_mb = os.path.getsize(out_gif) / (1024 * 1024)
    print(f"✓ Week {week_num:02d} done -> {out_gif} ({size_mb:.2f} MB, {frame_count} frames)")

if __name__ == "__main__":
    import showcase_configs
    if len(sys.argv) > 1:
        weeks = [int(w) for w in sys.argv[1:]]
    else:
        weeks = sorted(list(showcase_configs.CONFIGS.keys()))
    for w in weeks:
        if w in showcase_configs.CONFIGS:
            render_week_gif(w, showcase_configs.CONFIGS[w])
