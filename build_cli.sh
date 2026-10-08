#!/bin/bash
RUSTFLAGS="-C debuginfo=1" cargo build -p paperpilot-cli --jobs 2
