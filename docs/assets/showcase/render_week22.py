#!/usr/bin/env python3
"""
Generate crisp animated frames and palette-optimized GIF (<100KB) for Week 22:
CLI to Desktop Sync: One-Command Install (Terminal Hacker to Native GUI App).
Tools: Flash Installer (install.sh, install.ps1), Desktop AppImage/Windows installer, CLI & MCP instant launcher
Target: Developers across Linux, macOS, and Windows
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
font_subtitle = get_font(FONT_REGULAR, 15)
font_mono_lg = get_font(FONT_MONO_BOLD, 14)
font_mono_md = get_font(FONT_MONO, 12)
font_mono_sm = get_font(FONT_MONO, 11)
font_badge = get_font(FONT_BOLD, 12)

def draw_window_header(draw, x, y, w, h, title):
    draw.rounded_rectangle([x, y, x + w, y + h], radius=8, fill=(30, 34, 46))
    draw.rectangle([x, y + h - 8, x + w, y + h], fill=(30, 34, 46))
    draw.ellipse([x + 14, y + 14, x + 24, y + 24], fill=(255, 95, 87))
    draw.ellipse([x + 30, y + 14, x + 40, y + 24], fill=(254, 188, 46))
    draw.ellipse([x + 46, y + 14, x + 56, y + 24], fill=(40, 201, 64))
    draw.text((x + 70, y + 12), title, font=font_mono_sm, fill=(185, 195, 215))

def render_frame(step_index, state):
    img = Image.new("RGB", (WIDTH, HEIGHT), (16, 19, 24))
    draw = ImageDraw.Draw(img)

    step = state["step"]

    # Top Badge / Header
    draw.rounded_rectangle([40, 20, 280, 46], radius=13, fill=(20, 35, 60), outline=(59, 130, 246), width=1)
    draw.text((55, 25), "WEEK 22 : ONE-COMMAND FLASH INSTALL", font=font_badge, fill=(96, 165, 250))

    draw.text((300, 22), "CLI to Desktop Sync: Terminal Hacker to Native GUI App", font=font_title, fill=(255, 255, 255))
    draw.text((40, 52), "Zero dependencies -> Auto-detects Linux/macOS/Windows -> Installs CLI, Desktop GUI & MCP in <4 seconds", font=font_subtitle, fill=(140, 160, 185))

    # Left Column: Terminal / CLI Flash Installer
    left_x, left_y, left_w, left_h = 40, 85, 560, 550
    draw_window_header(draw, left_x, left_y, left_w, 34, "bash — curl -fsSL install.sh | bash --all")
    draw.rounded_rectangle([left_x, left_y + 34, left_x + left_w, left_y + left_h], radius=8, fill=(22, 26, 36), outline=(42, 50, 68), width=1)
    draw.rectangle([left_x, left_y + 34, left_x + left_w, left_y + 42], fill=(22, 26, 36))

    # Terminal Content
    ty = left_y + 50
    draw.text((left_x + 20, ty), "$ curl -fsSL https://paperpilot.dev/install.sh | bash -s -- --all", font=font_mono_md, fill=(240, 240, 240))
    ty += 28

    if step >= 1:
        draw.text((left_x + 20, ty), "[+] Detected OS: Linux (x86_64) | Shell: bash", font=font_mono_sm, fill=(96, 165, 250))
        ty += 22
        draw.text((left_x + 20, ty), "[+] Fetching release metadata: PaperPilot v0.8.1...", font=font_mono_sm, fill=(156, 163, 175))
        ty += 22
        draw.text((left_x + 20, ty), "[*] Downloading CLI binary: paperpilot-linux-amd64 (14MB)", font=font_mono_sm, fill=(156, 163, 175))
        ty += 22
        draw.text((left_x + 20, ty), "    ################################ 100% [0.8s]", font=font_mono_sm, fill=(34, 197, 94))
        ty += 26

    if step >= 2:
        draw.text((left_x + 20, ty), "[*] Downloading Desktop GUI: PaperPilot-0.7.0.AppImage (42MB)", font=font_mono_sm, fill=(156, 163, 175))
        ty += 22
        draw.text((left_x + 20, ty), "    ################################ 100% [1.9s]", font=font_mono_sm, fill=(34, 197, 94))
        ty += 26
        draw.text((left_x + 20, ty), "[+] Linking CLI -> ~/.local/bin/paperpilot", font=font_mono_sm, fill=(96, 165, 250))
        ty += 20
        draw.text((left_x + 20, ty), "[+] Installing desktop launcher -> ~/.local/share/applications", font=font_mono_sm, fill=(96, 165, 250))
        ty += 20
        draw.text((left_x + 20, ty), "[+] Registering MCP server -> Claude Desktop & Cursor config", font=font_mono_sm, fill=(168, 85, 247))
        ty += 26

    if step >= 3:
        draw.rounded_rectangle([left_x + 20, ty, left_x + left_w - 20, ty + 70], radius=6, fill=(16, 44, 28), outline=(34, 197, 94), width=1)
        draw.text((left_x + 35, ty + 12), "✓ INSTALLATION COMPLETE (Total Elapsed: 3.2s)", font=font_mono_md, fill=(74, 222, 128))
        draw.text((left_x + 35, ty + 32), "CLI:       /home/user/.local/bin/paperpilot (92 tools active)", font=font_mono_sm, fill=(187, 247, 208))
        draw.text((left_x + 35, ty + 48), "Desktop:   Native AppImage ready (Launch: paperpilot-desktop)", font=font_mono_sm, fill=(187, 247, 208))
        ty += 84

        draw.text((left_x + 20, ty), "$ paperpilot doctor --check-mcp", font=font_mono_md, fill=(240, 240, 240))
        ty += 24
        draw.text((left_x + 20, ty), "[PASS] CLI Core Runtime  : v0.8.1 (SIMD AVX2 enabled)", font=font_mono_sm, fill=(34, 197, 94))
        ty += 20
        draw.text((left_x + 20, ty), "[PASS] MCP Stdio Gateway : Ready (92 tools registered)", font=font_mono_sm, fill=(34, 197, 94))
        ty += 20
        draw.text((left_x + 20, ty), "[PASS] Claude Integration: Config synced seamlessly", font=font_mono_sm, fill=(34, 197, 94))

    # Right Column: Desktop GUI App Sync Preview
    right_x, right_y, right_w, right_h = 620, 85, 540, 550
    draw_window_header(draw, right_x, right_y, right_w, 34, "PaperPilot Desktop — Native GUI App (v0.8.1)")
    draw.rounded_rectangle([right_x, right_y + 34, right_x + right_w, right_y + right_h], radius=8, fill=(24, 28, 40), outline=(42, 50, 68), width=1)
    draw.rectangle([right_x, right_y + 34, right_x + right_w, right_y + 42], fill=(24, 28, 40))

    # App Sidebar & Main Content
    sb_w = 140
    draw.rectangle([right_x, right_y + 34, right_x + sb_w, right_y + right_h], fill=(19, 23, 33))
    draw.line([right_x + sb_w, right_y + 34, right_x + sb_w, right_y + right_h], fill=(42, 50, 68), width=1)

    # Sidebar Navigation items
    sy = right_y + 50
    nav_items = ["All 92 Tools", "Redaction", "OCR & Clean", "Forms & Acro", "Sign & Verify", "MCP Bridge", "Settings"]
    for i, item in enumerate(nav_items):
        is_active = (i == 0) if step < 3 else (i == 5)
        bg_col = (40, 55, 85) if is_active else (19, 23, 33)
        txt_col = (96, 165, 250) if is_active else (156, 163, 175)
        draw.rounded_rectangle([right_x + 8, sy, right_x + sb_w - 8, sy + 28], radius=4, fill=bg_col)
        draw.text((right_x + 18, sy + 7), item, font=font_mono_sm, fill=txt_col)
        sy += 34

    # Main Panel inside Desktop
    main_x = right_x + sb_w + 15
    main_y = right_y + 50

    draw.text((main_x, main_y), "Local-First Desktop & MCP Studio", font=font_mono_lg, fill=(255, 255, 255))
    draw.text((main_x, main_y + 22), "Instant synchronization between CLI binary and Native GUI", font=get_font(FONT_REGULAR, 12), fill=(156, 163, 175))

    # Cards in Desktop App
    cy = main_y + 52
    if step >= 1:
        # Card 1: Single Binary Engine
        draw.rounded_rectangle([main_x, cy, right_x + right_w - 15, cy + 90], radius=6, fill=(30, 36, 52), outline=(50, 60, 85))
        draw.text((main_x + 15, cy + 12), "1. Unified Local Engine", font=font_mono_md, fill=(96, 165, 250))
        draw.text((main_x + 15, cy + 32), "Terminal commands and desktop clicks call identical", font=get_font(FONT_REGULAR, 11), fill=(200, 210, 225))
        draw.text((main_x + 15, cy + 48), "high-performance Rust & C++ core libraries.", font=get_font(FONT_REGULAR, 11), fill=(200, 210, 225))
        draw.text((main_x + 15, cy + 68), "Status: ACTIVE (v0.8.1 Native)", font=font_mono_sm, fill=(34, 197, 94))

    cy += 105
    if step >= 2:
        # Card 2: Cross-Platform Support
        draw.rounded_rectangle([main_x, cy, right_x + right_w - 15, cy + 90], radius=6, fill=(30, 36, 52), outline=(50, 60, 85))
        draw.text((main_x + 15, cy + 12), "2. Cross-Platform Parity", font=font_mono_md, fill=(245, 158, 11))
        draw.text((main_x + 15, cy + 32), "Linux: AppImage / Tarball | macOS: DMG / Homebrew", font=get_font(FONT_REGULAR, 11), fill=(200, 210, 225))
        draw.text((main_x + 15, cy + 48), "Windows: PowerShell One-Liner (install.ps1) + MSI", font=get_font(FONT_REGULAR, 11), fill=(200, 210, 225))
        draw.text((main_x + 15, cy + 68), "Zero admin / sudo required by default", font=font_mono_sm, fill=(245, 158, 11))

    cy += 105
    if step >= 3:
        # Card 3: Auto MCP Bridge
        draw.rounded_rectangle([main_x, cy, right_x + right_w - 15, cy + 120], radius=6, fill=(25, 20, 45), outline=(147, 51, 234))
        draw.text((main_x + 15, cy + 12), "3. Automatic MCP Agent Bridge", font=font_mono_md, fill=(192, 132, 252))
        draw.text((main_x + 15, cy + 32), "Claude Desktop & Cursor auto-detected on install.", font=get_font(FONT_REGULAR, 11), fill=(220, 205, 245))
        draw.text((main_x + 15, cy + 48), "All 92 tools instantly accessible in AI prompt chats.", font=get_font(FONT_REGULAR, 11), fill=(220, 205, 245))

        draw.rounded_rectangle([main_x + 15, cy + 74, right_x + right_w - 30, cy + 105], radius=4, fill=(40, 25, 65))
        draw.text((main_x + 25, cy + 84), "CONNECTED: Claude Desktop (Port stdio active)", font=font_mono_sm, fill=(74, 222, 128))

    # Bottom Highlight Bar
    b_y = right_y + right_h - 65
    draw.rounded_rectangle([right_x + 15, b_y, right_x + right_w - 15, b_y + 50], radius=6, fill=(18, 38, 28), outline=(34, 197, 94))
    draw.text((right_x + 30, b_y + 11), "ONE COMMAND TO FULL PRIVATE DOCUMENT STACK", font=get_font(FONT_BOLD, 12), fill=(74, 222, 128))
    draw.text((right_x + 30, b_y + 28), "Terminal CLI, native Desktop UI & AI MCP server ready in seconds.", font=get_font(FONT_REGULAR, 11), fill=(187, 247, 208))

    return img

def main():
    tmp_dir = "/tmp/week22_frames"
    if os.path.exists(tmp_dir):
        shutil.rmtree(tmp_dir)
    os.makedirs(tmp_dir, exist_ok=True)

    states = [
        {"step": 0},
        {"step": 1},
        {"step": 2},
        {"step": 3},
    ]
    durations = [8, 10, 12, 18]

    frame_idx = 0
    for s_idx, state in enumerate(states):
        n_frames = durations[s_idx]
        frame_img = render_frame(s_idx, state)
        for _ in range(n_frames):
            frame_img.save(f"{tmp_dir}/frame_{frame_idx:04d}.png")
            frame_idx += 1

    palette_path = "/tmp/palette_w22.png"
    out_gif = "docs/assets/showcase/week22-flash-installation-sync.gif"

    subprocess.run([
        "ffmpeg", "-y", "-framerate", str(FPS), "-i", f"{tmp_dir}/frame_%04d.png",
        "-vf", "palettegen=max_colors=64:reserve_transparent=0", palette_path
    ], check=True)

    subprocess.run([
        "ffmpeg", "-y", "-framerate", str(FPS), "-i", f"{tmp_dir}/frame_%04d.png",
        "-i", palette_path, "-filter_complex", "paletteuse=dither=bayer:bayer_scale=4",
        out_gif
    ], check=True)

    size = os.path.getsize(out_gif)
    print(f"Week 22 GIF Generated: {out_gif} ({size} bytes, {size/1024:.2f} KB)")
    shutil.rmtree(tmp_dir)

if __name__ == "__main__":
    main()
