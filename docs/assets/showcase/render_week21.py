#!/usr/bin/env python3
"""
Generate crisp, pixel-perfect animated frames and convert to palette-optimized GIF
for Week 21: Multi-Interface Showcase.
"""

import os
import sys

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '../../..'))
if REPO_ROOT not in sys.path:
    sys.path.insert(0, REPO_ROOT)

import showcase_configs
import multi_interface_renderer

def main():
    cfg = showcase_configs.CONFIGS[21]
    multi_interface_renderer.render_week_gif(21, cfg)

if __name__ == '__main__':
    main()
