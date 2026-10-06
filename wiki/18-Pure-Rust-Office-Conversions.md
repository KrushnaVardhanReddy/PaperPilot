# 18. Pure-Rust Office Conversions

PaperPilot relies on native, zero-dependency Rust pipelines for universal document formats to maintain extreme speed without invoking external browsers.

## 1. Excel to PDF Pipeline
Uses `calamine` for zero-allocation `.xlsx` parsing and `genpdf` for document construction.
- Discards intermediate HTML formats.
- Renders cells as bounded frames.

## 2. Markdown to PDF Pipeline
Uses the `typst` compiler as a high-performance markup engine.
- Direct-to-PDF export via `typst-pdf`.
- Eliminates Chromium cold start times.

## 3. HTML to PDF Pipeline
Uses `fulgur` for pure-Rust parsing of HTML DOMs and inline CSS styling to generate vector PDF pages.
