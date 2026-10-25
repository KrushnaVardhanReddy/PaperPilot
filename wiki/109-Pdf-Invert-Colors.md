# PDF Invert Colors Engine

The pure-Rust vector stream color inversion and dark mode engine applies color inversions at the vector level without rasterizing to images, ensuring 100% of selectable text and crisp embedded fonts are preserved.

## Engine
`paperpilot-pdf/src/operations/pdf_invert_colors.rs` contains the `invert_pdf_colors` function.
It alters color operands (like `rg`, `RG`, `g`, `G`, `k`, `K`) directly in the PDF's `/Contents` stream.

## Modes
- **DarkMode**: Inserts a dark background and softly maps colors to dark hues.
- **TrueInvert**: Completely maps 1.0 to 0.0 and vice-versa for full RGB values.
- **HighContrast**: Rounds to full black or full white.
- **Sepia**: Adjusts to a warm tone.
