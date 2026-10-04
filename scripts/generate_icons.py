#!/usr/bin/env python3
import os
from PIL import Image

src_path = '/home/krushna/.gemini/antigravity-ide/brain/971d7225-0e96-4f5a-a40c-bdd0d2d57c3e/concept_rust_svelte_1.jpg'
dest_dir = os.path.join(os.path.dirname(__file__), '..', 'apps', 'desktop', 'src-tauri', 'icons')
static_dir = os.path.join(os.path.dirname(__file__), '..', 'apps', 'desktop', 'static')

im = Image.open(src_path).convert('RGBA')

sizes = {
    '32x32.png': (32, 32),
    '128x128.png': (128, 128),
    '128x128@2x.png': (256, 256),
    'icon.png': (512, 512),
    'Square30x30Logo.png': (30, 30),
    'Square44x44Logo.png': (44, 44),
    'Square71x71Logo.png': (71, 71),
    'Square89x89Logo.png': (89, 89),
    'Square107x107Logo.png': (107, 107),
    'Square142x142Logo.png': (142, 142),
    'Square150x150Logo.png': (150, 150),
    'Square284x284Logo.png': (284, 284),
    'Square310x310Logo.png': (310, 310),
    'StoreLogo.png': (50, 50),
}

for filename, size in sizes.items():
    resized = im.resize(size, Image.Resampling.LANCZOS)
    resized.save(os.path.join(dest_dir, filename), 'PNG')

# Also write to static/ for Vite/web view
im.resize((32, 32), Image.Resampling.LANCZOS).save(os.path.join(static_dir, 'favicon.png'), 'PNG')
im.resize((512, 512), Image.Resampling.LANCZOS).save(os.path.join(static_dir, 'logo.png'), 'PNG')

# Save multi-resolution Windows ICO
ico_sizes = [(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
im.save(os.path.join(dest_dir, 'icon.ico'), format='ICO', sizes=ico_sizes)

print('Successfully exported R1 Rust+Svelte icon suite.')
