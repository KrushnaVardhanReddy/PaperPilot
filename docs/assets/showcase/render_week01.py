#!/usr/bin/env python3
"""
Generate crisp, pixel-perfect animated frames and convert to palette-optimized GIF
for Week 01: Zero-Leak LLM Pipeline: Irreversible PII Redaction (pdf_redact + pdf_hash).
"""

import os
import shutil
import subprocess
from PIL import Image, ImageDraw, ImageFont

WIDTH = 1200
HEIGHT = 675
FPS = 12

# Fonts
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
font_mono_lg = get_font(FONT_MONO_BOLD, 17)
font_mono_md = get_font(FONT_MONO, 15)
font_mono_sm = get_font(FONT_MONO, 13)
font_doc_title = get_font(FONT_BOLD, 16)
font_doc_body = get_font(FONT_REGULAR, 13)
font_doc_bold = get_font(FONT_BOLD, 13)
font_badge = get_font(FONT_BOLD, 12)

def draw_window_header(draw, x, y, w, h, title, icon_color=(79, 140, 255)):
    # Header bar
    draw.rounded_rectangle([x, y, x + w, y + h], radius=8, fill=(28, 33, 44))
    # Fill bottom straight to attach body
    draw.rectangle([x, y + h - 8, x + w, y + h], fill=(28, 33, 44))
    # Window dots
    draw.ellipse([x + 14, y + 14, x + 24, y + 24], fill=(255, 95, 87)) # red
    draw.ellipse([x + 30, y + 14, x + 40, y + 24], fill=(254, 188, 46)) # yellow
    draw.ellipse([x + 46, y + 14, x + 56, y + 24], fill=(40, 201, 64)) # green
    # Title
    draw.text((x + 70, y + 12), title, font=font_mono_sm, fill=(180, 195, 215))

def render_frame(step_index, total_steps, state):
    """
    state dict:
      cmd1_progress: 0.0 to 1.0
      cmd1_done: bool
      cmd2_progress: 0.0 to 1.0
      cmd2_done: bool
      redacted: bool
      attempt_copy: bool
      verification_done: bool
    """
    img = Image.new("RGB", (WIDTH, HEIGHT), (15, 18, 25)) # Sleek dark background
    draw = ImageDraw.Draw(img)

    # Top Banner / Header
    # Pill Badge
    draw.rounded_rectangle([40, 20, 250, 46], radius=13, fill=(35, 45, 65), outline=(60, 90, 150), width=1)
    draw.text((55, 25), "WEEK 01 : SECURITY & PRIVACY", font=font_badge, fill=(100, 185, 255))

    # Main Title
    draw.text((270, 22), "Zero-Leak LLM Pipeline: Irreversible PII Redaction", font=font_title, fill=(255, 255, 255))
    draw.text((40, 52), "PaperPilot burns underlying PDF byte streams before piping contracts to AI models & Vector DBs", font=font_subtitle, fill=(140, 155, 175))

    # Left Panel: Terminal Window
    term_x, term_y, term_w, term_h = 40, 85, 640, 560
    draw_window_header(draw, term_x, term_y, term_w, 38, "zsh — paperpilot v0.8.1 (Rust / Local-First)")
    # Terminal Body
    draw.rounded_rectangle([term_x, term_y + 38, term_x + term_w, term_y + term_h], radius=8, fill=(18, 22, 30), outline=(40, 48, 65), width=1)
    draw.rectangle([term_x, term_y + 38, term_x + term_w, term_y + 46], fill=(18, 22, 30))

    cur_y = term_y + 55
    prompt_str = "$ "
    draw.text((term_x + 18, cur_y), prompt_str, font=font_mono_lg, fill=(80, 200, 120))
    p_len = 22

    # Command 1 text
    full_cmd1 = "paperpilot redact --input contract.pdf -o safe.pdf --query 'PII'"
    chars1 = int(len(full_cmd1) * state["cmd1_progress"])
    drawn_cmd1 = full_cmd1[:chars1]
    draw.text((term_x + 18 + p_len, cur_y), drawn_cmd1, font=font_mono_sm, fill=(240, 240, 245))

    # Blinking cursor for cmd1
    if not state["cmd1_done"]:
        cursor_x = term_x + 18 + p_len + int(chars1 * 7.8)
        draw.rectangle([cursor_x, cur_y, cursor_x + 8, cur_y + 15], fill=(255, 255, 255))

    cur_y += 28

    if state["cmd1_done"]:
        # Output of redaction
        draw.text((term_x + 22, cur_y), " [Scanning text streams...]", font=font_mono_sm, fill=(130, 145, 170))
        cur_y += 20
        draw.text((term_x + 22, cur_y), " Found: SSN, CreditCard, InternalEmail", font=font_mono_sm, fill=(255, 170, 60))
        cur_y += 20
        draw.text((term_x + 22, cur_y), " Incinerating underlying glyph objects in ContentStream...", font=font_mono_sm, fill=(130, 145, 170))
        cur_y += 20
        draw.text((term_x + 22, cur_y), "✓ Operation completed successfully in 12.4ms", font=font_mono_sm, fill=(80, 220, 140))
        cur_y += 32

        # Prompt 2
        draw.text((term_x + 18, cur_y), prompt_str, font=font_mono_sm, fill=(80, 200, 120))
        full_cmd2 = "paperpilot hash --input safe.pdf"
        chars2 = int(len(full_cmd2) * state["cmd2_progress"])
        drawn_cmd2 = full_cmd2[:chars2]
        draw.text((term_x + 18 + p_len, cur_y), drawn_cmd2, font=font_mono_sm, fill=(240, 240, 245))

        if not state["cmd2_done"]:
            cursor_x = term_x + 18 + p_len + int(chars2 * 7.8)
            draw.rectangle([cursor_x, cur_y, cursor_x + 8, cur_y + 15], fill=(255, 255, 255))
        cur_y += 28

    if state["cmd2_done"]:
        # Hash output
        draw.text((term_x + 22, cur_y), "SHA-256 Digest:", font=font_mono_sm, fill=(130, 145, 170))
        cur_y += 20
        draw.text((term_x + 22, cur_y), "ef715f48e06a1094d8e409681121cda39becdf658c...", font=font_mono_sm, fill=(100, 200, 255))
        cur_y += 20
        draw.text((term_x + 22, cur_y), "✓ Immutable audit fingerprint verified.", font=font_mono_sm, fill=(80, 220, 140))
        cur_y += 32

    if state["attempt_copy"]:
        # Attempt copy test
        draw.text((term_x + 18, cur_y), prompt_str, font=font_mono_sm, fill=(80, 200, 120))
        draw.text((term_x + 18 + p_len, cur_y), "paperpilot extract-text -i safe.pdf | grep SSN", font=font_mono_sm, fill=(240, 240, 245))
        cur_y += 24
        draw.text((term_x + 22, cur_y), "-> [0 matches found. Byte stream completely sanitized]", font=font_mono_sm, fill=(255, 100, 100))
        cur_y += 28

    # Right Panel: Document Viewer (Paper / PDF visual)
    doc_x, doc_y, doc_w, doc_h = 705, 85, 455, 560
    status_label = "PDF Document: safe.pdf (100% Sanitized)" if state["redacted"] else "PDF Document: contract.pdf (UNPROTECTED PII)"
    draw_window_header(draw, doc_x, doc_y, doc_w, 38, status_label)
    
    # Paper Body (White/cream document sheet)
    draw.rounded_rectangle([doc_x, doc_y + 38, doc_x + doc_w, doc_y + doc_h], radius=8, fill=(248, 249, 252), outline=(50, 60, 80), width=1)
    draw.rectangle([doc_x, doc_y + 38, doc_x + doc_w, doc_y + 46], fill=(248, 249, 252))

    # Document Header Badge
    paper_x = doc_x + 25
    paper_y = doc_y + 55

    # Document Watermark / Header
    draw.text((paper_x, paper_y), "CONFIDENTIAL CONSULTING AGREEMENT", font=font_doc_title, fill=(20, 30, 45))
    draw.line([(paper_x, paper_y + 26), (doc_x + doc_w - 25, paper_y + 26)], fill=(210, 215, 225), width=1)
    
    dy = paper_y + 38
    draw.text((paper_x, dy), "Client: Apex Global Technologies LLC", font=font_doc_body, fill=(50, 60, 75))
    dy += 26
    draw.text((paper_x, dy), "Consultant: Sarah M. Jenkins", font=font_doc_body, fill=(50, 60, 75))
    dy += 32

    # Sensitive Item 1: SSN
    draw.text((paper_x, dy), "Social Security Number:", font=font_doc_bold, fill=(30, 40, 55))
    val_x = paper_x + 175
    if state["redacted"]:
        # Permanent solid black box
        draw.rectangle([val_x, dy - 2, val_x + 130, dy + 18], fill=(10, 10, 15))
        draw.text((val_x + 15, dy + 2), "[REDACTED]", font=font_badge, fill=(255, 255, 255))
    else:
        # Exposed with subtle warning border
        draw.text((val_x, dy), "123-45-6789", font=font_doc_bold, fill=(220, 40, 40))
        draw.rectangle([val_x - 4, dy - 2, val_x + 110, dy + 18], outline=(230, 80, 80), width=1)

    dy += 32
    # Sensitive Item 2: Corporate Card
    draw.text((paper_x, dy), "Corporate Card:", font=font_doc_bold, fill=(30, 40, 55))
    val_x = paper_x + 175
    if state["redacted"]:
        draw.rectangle([val_x, dy - 2, val_x + 180, dy + 18], fill=(10, 10, 15))
        draw.text((val_x + 35, dy + 2), "[REDACTED]", font=font_badge, fill=(255, 255, 255))
    else:
        draw.text((val_x, dy), "4532-8921-3847-9201", font=font_doc_bold, fill=(220, 40, 40))
        draw.rectangle([val_x - 4, dy - 2, val_x + 170, dy + 18], outline=(230, 80, 80), width=1)

    dy += 32
    # Sensitive Item 3: Email
    draw.text((paper_x, dy), "Direct Contact Email:", font=font_doc_bold, fill=(30, 40, 55))
    val_x = paper_x + 175
    if state["redacted"]:
        draw.rectangle([val_x, dy - 2, val_x + 220, dy + 18], fill=(10, 10, 15))
        draw.text((val_x + 55, dy + 2), "[REDACTED]", font=font_badge, fill=(255, 255, 255))
    else:
        draw.text((val_x, dy), "sarah.jenkins@apextech.internal", font=font_doc_body, fill=(50, 60, 75))

    dy += 36
    draw.text((paper_x, dy), "Terms of Engagement:", font=font_doc_bold, fill=(30, 40, 55))
    dy += 22
    draw.text((paper_x, dy), "The consultant shall maintain strict confidentiality over trade", font=font_doc_body, fill=(80, 90, 105))
    dy += 18
    draw.text((paper_x, dy), "secrets, architecture blueprints, and financial projections.", font=font_doc_body, fill=(80, 90, 105))

    dy += 36
    draw.text((paper_x, dy), "Authorized Signature:", font=font_doc_body, fill=(50, 60, 75))
    draw.line([(paper_x + 145, dy + 14), (paper_x + 320, dy + 14)], fill=(120, 130, 145), width=1)
    draw.text((paper_x + 160, dy - 4), "Sarah M. Jenkins", font=font_doc_title, fill=(30, 60, 120))
    dy += 24
    draw.text((paper_x, dy), "Date Signed: Nov 18, 2026", font=font_doc_body, fill=(100, 110, 125))

    # Bottom Document Security Card / Callout
    card_y = doc_y + doc_h - 130
    if state["redacted"]:
        draw.rounded_rectangle([paper_x, card_y, doc_x + doc_w - 25, card_y + 110], radius=6, fill=(235, 248, 240), outline=(50, 180, 100), width=1)
        draw.text((paper_x + 15, card_y + 12), "✓ STREAM-LEVEL BYTE REMOVAL (Zero-Leak)", font=font_doc_bold, fill=(20, 140, 60))
        draw.text((paper_x + 15, card_y + 36), "• Underlying font glyphs removed from PDF objects.", font=font_doc_body, fill=(30, 100, 50))
        draw.text((paper_x + 15, card_y + 56), "• Safe for OpenAI, Anthropic, Claude Desktop & Vector DBs.", font=font_doc_body, fill=(30, 100, 50))
        draw.text((paper_x + 15, card_y + 76), "• Immutable SHA-256 chain-of-custody logged.", font=font_doc_body, fill=(30, 100, 50))
    else:
        draw.rounded_rectangle([paper_x, card_y, doc_x + doc_w - 25, card_y + 110], radius=6, fill=(255, 242, 242), outline=(220, 60, 60), width=1)
        draw.text((paper_x + 15, card_y + 12), "⚠ UNPROTECTED PII DETECTED", font=font_doc_bold, fill=(200, 30, 30))
        draw.text((paper_x + 15, card_y + 36), "• Social Security Number & Card details exposed.", font=font_doc_body, fill=(160, 30, 30))
        draw.text((paper_x + 15, card_y + 56), "• Ingesting this into RAG/LLM violates GDPR & HIPAA.", font=font_doc_body, fill=(160, 30, 30))
        draw.text((paper_x + 15, card_y + 76), "• Standard visual masks DO NOT delete underlying text.", font=font_doc_body, fill=(160, 30, 30))

    return img

def main():
    frames_dir = "/tmp/week01_frames"
    if os.path.exists(frames_dir):
        shutil.rmtree(frames_dir)
    os.makedirs(frames_dir)

    print("Generating Week 01 frames...")

    frame_count = 0

    # Phase 1: Initial state, showing vulnerable document (1.2 seconds = 14 frames)
    for _ in range(14):
        state = {
            "cmd1_progress": 0.0,
            "cmd1_done": False,
            "cmd2_progress": 0.0,
            "cmd2_done": False,
            "redacted": False,
            "attempt_copy": False,
            "verification_done": False
        }
        img = render_frame(frame_count, 100, state)
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    # Phase 2: Typing command 1 (paperpilot redact...) (1.5 seconds = 18 frames)
    for i in range(18):
        progress = (i + 1) / 18.0
        state = {
            "cmd1_progress": progress,
            "cmd1_done": False,
            "cmd2_progress": 0.0,
            "cmd2_done": False,
            "redacted": False,
            "attempt_copy": False,
            "verification_done": False
        }
        img = render_frame(frame_count, 100, state)
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    # Phase 3: Redaction execution -> Document instantly switches to REDACTED (2.0 seconds = 24 frames)
    for _ in range(24):
        state = {
            "cmd1_progress": 1.0,
            "cmd1_done": True,
            "cmd2_progress": 0.0,
            "cmd2_done": False,
            "redacted": True,
            "attempt_copy": False,
            "verification_done": False
        }
        img = render_frame(frame_count, 100, state)
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    # Phase 4: Typing command 2 (paperpilot hash...) (1.2 seconds = 14 frames)
    for i in range(14):
        progress = (i + 1) / 14.0
        state = {
            "cmd1_progress": 1.0,
            "cmd1_done": True,
            "cmd2_progress": progress,
            "cmd2_done": False,
            "redacted": True,
            "attempt_copy": False,
            "verification_done": False
        }
        img = render_frame(frame_count, 100, state)
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    # Phase 5: Hash displayed + Verification attempt copy (2.5 seconds = 30 frames)
    for _ in range(30):
        state = {
            "cmd1_progress": 1.0,
            "cmd1_done": True,
            "cmd2_progress": 1.0,
            "cmd2_done": True,
            "redacted": True,
            "attempt_copy": True,
            "verification_done": True
        }
        img = render_frame(frame_count, 100, state)
        img.save(f"{frames_dir}/frame_{frame_count:04d}.png")
        frame_count += 1

    print(f"Rendered {frame_count} frames. Encoding GIF via ffmpeg palette...")

    palette_path = "/tmp/week01_palette.png"
    out_gif = "docs/assets/showcase/week01-zero-leak-redaction.gif"

    # Step 1: palettegen
    cmd_pal = [
        "ffmpeg", "-y", "-framerate", str(FPS), "-i", f"{frames_dir}/frame_%04d.png",
        "-vf", "scale=1200:-1:flags=lanczos,palettegen=stats_mode=diff", palette_path
    ]
    subprocess.run(cmd_pal, check=True)

    # Step 2: paletteuse
    cmd_gif = [
        "ffmpeg", "-y", "-framerate", str(FPS), "-i", f"{frames_dir}/frame_%04d.png",
        "-i", palette_path,
        "-filter_complex", "scale=1200:-1:flags=lanczos[x];[x][1:v]paletteuse=dither=bayer:bayer_scale=3",
        out_gif
    ]
    subprocess.run(cmd_gif, check=True)

    size_mb = os.path.getsize(out_gif) / (1024 * 1024)
    print(f"Done! Saved {out_gif} ({size_mb:.2f} MB)")

if __name__ == "__main__":
    main()
