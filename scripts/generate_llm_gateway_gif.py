import os
import shutil
import subprocess
from PIL import Image, ImageDraw, ImageFont

WIDTH = 1000
HEIGHT = 620

FONT_REGULAR = "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf"
FONT_BOLD = "/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf"
FONT_MONO = "/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf"
FONT_MONO_BOLD = "/usr/share/fonts/truetype/liberation/LiberationMono-Bold.ttf"

font_title = ImageFont.truetype(FONT_BOLD, 14)
font_badge = ImageFont.truetype(FONT_BOLD, 11)
font_small = ImageFont.truetype(FONT_REGULAR, 12)
font_chat = ImageFont.truetype(FONT_REGULAR, 14)
font_chat_bold = ImageFont.truetype(FONT_BOLD, 14)
font_code = ImageFont.truetype(FONT_MONO, 12)
font_code_bold = ImageFont.truetype(FONT_MONO_BOLD, 12)
font_metric_num = ImageFont.truetype(FONT_BOLD, 22)
font_metric_lbl = ImageFont.truetype(FONT_REGULAR, 11)

BG_WINDOW = (16, 18, 24)
BG_HEADER = (12, 14, 18)
BORDER_COLOR = (38, 42, 54)
TEXT_WHITE = (245, 247, 250)
TEXT_MUTED = (145, 155, 175)
ACCENT_GREEN = (34, 197, 94)
ACCENT_BLUE = (59, 130, 246)
ACCENT_PURPLE = (168, 85, 247)
ACCENT_ORANGE = (249, 115, 22)
CARD_BG = (22, 26, 36)
BOX_BG = (10, 12, 16)

frames_dir = "/tmp/llm_gateway_frames"
if os.path.exists(frames_dir):
    shutil.rmtree(frames_dir)
os.makedirs(frames_dir)

def draw_base_window():
    img = Image.new("RGBA", (WIDTH, HEIGHT), BG_WINDOW)
    draw = ImageDraw.Draw(img)

    # Top title bar
    draw.rectangle([(0, 0), (WIDTH, 48)], fill=BG_HEADER)
    draw.line([(0, 48), (WIDTH, 48)], fill=BORDER_COLOR, width=1)

    # Window dots
    draw.ellipse([(18, 18), (28, 28)], fill=(239, 68, 68))
    draw.ellipse([(36, 18), (46, 28)], fill=(245, 158, 11))
    draw.ellipse([(54, 18), (64, 28)], fill=(34, 197, 94))

    # Window Title
    draw.text((82, 16), "PaperPilot Document Gateway ➔ LLM Pipeline", fill=TEXT_WHITE, font=font_title)

    # Status Badges
    p_x = WIDTH - 340
    # PaperPilot Badge
    draw.rounded_rectangle([(p_x, 12), (p_x + 155, 36)], radius=12, fill=(20, 35, 28), outline=(34, 197, 94))
    draw.ellipse([(p_x + 10, 20), (p_x + 18, 28)], fill=ACCENT_GREEN)
    draw.text((p_x + 24, 15), "Gateway: 3.4ms | <50MB", fill=(180, 245, 205), font=font_badge)

    # LLM Badge
    l_x = p_x + 165
    draw.rounded_rectangle([(l_x, 12), (l_x + 155, 36)], radius=12, fill=(28, 24, 45), outline=(168, 85, 247))
    draw.ellipse([(l_x + 10, 20), (l_x + 18, 28)], fill=ACCENT_PURPLE)
    draw.text((l_x + 24, 15), "Llama 3.3 70B (Fast)", fill=(230, 210, 255), font=font_badge)

    return img

def render_frame(idx, stage, stream_chars=0):
    img = draw_base_window()
    draw = ImageDraw.Draw(img)

    # Top Hero KPI Metric Cards
    # Card 1: Latency
    c1_x, c1_y, c1_w, c1_h = 24, 64, 220, 72
    draw.rounded_rectangle([(c1_x, c1_y), (c1_x + c1_w, c1_y + c1_h)], radius=10, fill=CARD_BG, outline=BORDER_COLOR)
    draw.text((c1_x + 16, c1_y + 12), "INGESTION LATENCY", fill=TEXT_MUTED, font=font_metric_lbl)
    lat_val = "3.42 ms" if stage >= 2 else ("Parsing..." if stage == 1 else "Ready")
    lat_col = ACCENT_GREEN if stage >= 2 else (ACCENT_ORANGE if stage == 1 else TEXT_MUTED)
    draw.text((c1_x + 16, c1_y + 32), lat_val, fill=lat_col, font=font_metric_num)

    # Card 2: Memory Footprint
    c2_x = c1_x + c1_w + 16
    draw.rounded_rectangle([(c2_x, c1_y), (c2_x + c1_w, c1_y + c1_h)], radius=10, fill=CARD_BG, outline=BORDER_COLOR)
    draw.text((c2_x + 16, c1_y + 12), "PROCESS MEMORY", fill=TEXT_MUTED, font=font_metric_lbl)
    draw.text((c2_x + 16, c1_y + 32), "< 48.5 MB", fill=ACCENT_BLUE, font=font_metric_num)

    # Card 3: Cloud Egress
    c3_x = c2_x + c1_w + 16
    draw.rounded_rectangle([(c3_x, c1_y), (c3_x + c1_w, c1_y + c1_h)], radius=10, fill=CARD_BG, outline=BORDER_COLOR)
    draw.text((c3_x + 16, c1_y + 12), "CLOUD EGRESS (INGESTION)", fill=TEXT_MUTED, font=font_metric_lbl)
    draw.text((c3_x + 16, c1_y + 32), "0 Bytes (Local)", fill=ACCENT_PURPLE, font=font_metric_num)

    # Card 4: Extraction Engine
    c4_x = c3_x + c1_w + 16
    c4_w = WIDTH - c4_x - 24
    draw.rounded_rectangle([(c4_x, c1_y), (c4_x + c4_w, c1_y + c1_h)], radius=10, fill=CARD_BG, outline=BORDER_COLOR)
    draw.text((c4_x + 16, c1_y + 12), "PROTOCOL COMPATIBILITY", fill=TEXT_MUTED, font=font_metric_lbl)
    draw.text((c4_x + 16, c1_y + 32), "Apache Tika Wire", fill=TEXT_WHITE, font=font_metric_num)

    # 2-Column Split: Left = Ingestion Terminal & Document, Right = LLM Intelligence
    col_y = 150
    col_h = HEIGHT - col_y - 20
    col_w = (WIDTH - 48 - 18) // 2
    left_x = 24
    right_x = left_x + col_w + 18

    # --- LEFT PANE: PaperPilot Ingestion Gateway ---
    draw.rounded_rectangle([(left_x, col_y), (left_x + col_w, col_y + col_h)], radius=10, fill=CARD_BG, outline=BORDER_COLOR)
    # Header
    draw.rectangle([(left_x, col_y), (left_x + col_w, col_y + 38)], fill=BG_HEADER)
    draw.line([(left_x, col_y + 38), (left_x + col_w, col_y + 38)], fill=BORDER_COLOR, width=1)
    draw.text((left_x + 16, col_y + 11), "PaperPilot Ingestion Gateway (Port 7823)", fill=TEXT_WHITE, font=font_chat_bold)

    # Terminal box inside left pane
    t_x, t_y, t_w, t_h = left_x + 14, col_y + 50, col_w - 28, 120
    draw.rounded_rectangle([(t_x, t_y), (t_x + t_w, t_y + t_h)], radius=6, fill=BOX_BG, outline=BORDER_COLOR)
    
    draw.text((t_x + 12, t_y + 10), "$ curl -X PUT http://127.0.0.1:7823/tika/text \\", fill=TEXT_MUTED, font=font_code)
    draw.text((t_x + 28, t_y + 28), "--data-binary @quarterly_financial_report.pdf", fill=TEXT_MUTED, font=font_code)
    
    if stage >= 1:
        draw.text((t_x + 12, t_y + 54), "[INFO] Received PUT /tika/text (17.8 KB PDF)", fill=ACCENT_BLUE, font=font_code)
    if stage >= 2:
        draw.text((t_x + 12, t_y + 72), "[INFO] Rust Engine: 4 pages extracted in 3.42ms", fill=ACCENT_GREEN, font=font_code_bold)
        draw.text((t_x + 12, t_y + 90), "[200 OK] Extracted 315 chars -> Tabular Markdown", fill=TEXT_WHITE, font=font_code)

    # Extracted Markdown Table Box
    m_y = t_y + t_h + 14
    m_h = col_y + col_h - m_y - 14
    draw.rounded_rectangle([(t_x, m_y), (t_x + t_w, m_y + m_h)], radius=6, fill=BOX_BG, outline=BORDER_COLOR)
    draw.text((t_x + 12, m_y + 10), "EXTRACTED DOCUMENT CONTEXT (Instant)", fill=ACCENT_GREEN, font=font_badge)

    if stage >= 2:
        lines = [
            "**Quarterly Financial Report**",
            "",
            "| Quarter | Revenue  | Expenses | Net Profit | Margin |",
            "|---------|----------|----------|------------|--------|",
            "| Q1 2024 | $120,000 | $80,000  | $40,000    | 33.3%  |",
            "| Q2 2024 | $150,000 | $90,000  | $60,000    | 40.0%  |",
            "| Q3 2024 | $175,000 | $105,000 | $70,000    | 40.0%  |",
            "| Q4 2024 | $210,000 | $120,000 | $90,000    | 42.9%  |",
        ]
        ly = m_y + 32
        for line in lines:
            col = ACCENT_PURPLE if line.startswith("| Quarter") else (TEXT_WHITE if line.startswith("| Q") else TEXT_MUTED)
            draw.text((t_x + 12, ly), line, fill=col, font=font_code)
            ly += 18

    # --- RIGHT PANE: LLM Financial Intelligence ---
    draw.rounded_rectangle([(right_x, col_y), (right_x + col_w, col_y + col_h)], radius=10, fill=CARD_BG, outline=BORDER_COLOR)
    draw.rectangle([(right_x, col_y), (right_x + col_w, col_y + 38)], fill=BG_HEADER)
    draw.line([(right_x, col_y + 38), (right_x + col_w, col_y + 38)], fill=BORDER_COLOR, width=1)
    draw.text((right_x + 16, col_y + 11), "LLM Inference: Meta Llama 3.3 70B Fast", fill=TEXT_WHITE, font=font_chat_bold)

    # Prompt box
    rp_x, rp_y, rp_w, rp_h = right_x + 14, col_y + 50, col_w - 28, 70
    draw.rounded_rectangle([(rp_x, rp_y), (rp_x + rp_w, rp_y + rp_h)], radius=6, fill=BOX_BG, outline=BORDER_COLOR)
    draw.text((rp_x + 12, rp_y + 10), "USER QUERY:", fill=ACCENT_ORANGE, font=font_badge)
    draw.text((rp_x + 12, rp_y + 26), "Extract quarterly performance into Markdown.", fill=TEXT_WHITE, font=font_chat)
    draw.text((rp_x + 12, rp_y + 44), "Which quarter had highest margin? YoY growth?", fill=TEXT_WHITE, font=font_chat)

    # Model Output Box
    ro_y = rp_y + rp_h + 14
    ro_h = col_y + col_h - ro_y - 14
    draw.rounded_rectangle([(rp_x, ro_y), (rp_x + rp_w, ro_y + ro_h)], radius=6, fill=BOX_BG, outline=BORDER_COLOR)
    draw.text((rp_x + 12, ro_y + 10), "LLM STREAMING RESPONSE:", fill=ACCENT_PURPLE, font=font_badge)

    full_output = (
        "Analysis Complete:\n\n"
        "1. Highest Profit Margin:\n"
        "   -> Q4 2024 achieved the peak margin at 42.9%\n"
        "      ($90,000 profit on $210,000 revenue).\n\n"
        "2. Year-over-Year Revenue Growth:\n"
        "   -> Q1: $120,000 | Q4: $210,000\n"
        "   -> Growth = (($210K - $120K) / $120K) * 100\n"
        "   -> Total Revenue Growth: +75.0%\n\n"
        "Zero-Egress Ingestion Verified (<50MB RAM)"
    )

    if stage >= 3:
        shown_text = full_output[:stream_chars]
        ry = ro_y + 32
        for o_line in shown_text.split("\n"):
            draw.text((rp_x + 12, ry), o_line, fill=TEXT_WHITE if "->" not in o_line else ACCENT_GREEN, font=font_code)
            ry += 18
            
        # Draw blinking cursor if still typing
        if stream_chars < len(full_output):
            draw.rectangle([(rp_x + 12, ry), (rp_x + 20, ry + 14)], fill=ACCENT_PURPLE)

    img.save(os.path.join(frames_dir, f"frame_{idx:04d}.png"))

def build_gif():
    frame_idx = 0

    # Stage 0: Initial clean state (4 frames)
    for _ in range(4):
        render_frame(frame_idx, stage=0)
        frame_idx += 1

    # Stage 1: Request sent to PaperPilot Gateway (4 frames)
    for _ in range(4):
        render_frame(frame_idx, stage=1)
        frame_idx += 1

    # Stage 2: Rust Gateway completes extraction in 3.4ms (6 frames)
    for _ in range(6):
        render_frame(frame_idx, stage=2)
        frame_idx += 1

    # Stage 3: Llama 3.3 70B streaming response (smooth typing)
    full_len = 360
    for c in range(15, full_len + 1, 15):
        render_frame(frame_idx, stage=3, stream_chars=c)
        frame_idx += 1

    # Stage 4: Hold final complete state (12 frames)
    for _ in range(12):
        render_frame(frame_idx, stage=3, stream_chars=9999)
        frame_idx += 1

    gif_out = "docs/assets/showcase/llm_gateway_financial_extraction.gif"
    os.makedirs(os.path.dirname(gif_out), exist_ok=True)
    palette_path = "/tmp/palette.png"

    print("Generating color palette...")
    subprocess.run([
        "ffmpeg", "-y", "-i", f"{frames_dir}/frame_%04d.png",
        "-vf", "palettegen=max_colors=128:stats_mode=diff",
        palette_path
    ], check=True)

    print(f"Compiling animated GIF to {gif_out}...")
    subprocess.run([
        "ffmpeg", "-y", "-framerate", "10",
        "-i", f"{frames_dir}/frame_%04d.png",
        "-i", palette_path,
        "-lavfi", "paletteuse=dither=bayer:bayer_scale=3",
        gif_out
    ], check=True)

    file_size_kb = os.path.getsize(gif_out) / 1024
    print(f"SUCCESS: Generated GIF at {gif_out} ({file_size_kb:.1f} KB, {frame_idx} frames)")

if __name__ == "__main__":
    build_gif()
