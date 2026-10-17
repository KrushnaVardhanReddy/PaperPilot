#!/usr/bin/env python3
import sys
import os

print("❌ ERROR: The Python `stress_test.py` harness is deprecated.", file=sys.stderr)
print("Please run the new pure-Rust soak-bench instead:", file=sys.stderr)
print("  cargo run --release -p soak-bench -- --duration 3600", file=sys.stderr)
sys.exit(1)
