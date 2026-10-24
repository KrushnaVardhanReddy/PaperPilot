# PDF N-Up (Page Imposition)

The `pdf_nup` engine enables compiling multiple pages of a PDF document into a single sheet, arranging them into grids such as 2-up, 4-up, 9-up, or 16-up.

## Architecture

The operation is built strictly following PaperPilot's 'Single Core Runner + 5 Thin Wrappers' convention:
- **Core Engine (`paperpilot-pdf`)**: Processes PDF byte streams directly using `lopdf`. Translates original pages into `Form XObjects` which are then embedded onto dynamically generated composite pages using `q/cm/Do/Q` affine transformation matrices.
- **REST Gateway, MCP Server, WASM Edge**: Expose the options transparently via `PdfNupOptions`.

## Options

`PdfNupOptions` accepts the following configuration parameters:
- `nup`: (u32) Number of pages per sheet: 2, 4, 9, or 16. Defaults to 2.
- `ordering`: (String) "horizontal" or "vertical" flow of the pages. Defaults to "horizontal".
- `margin_pt`: (f32) Canvas margin size around the edge of the sheet in points.
- `spacing_pt`: (f32) Padding margin size between the composited page cells in points.
- `draw_borders`: (bool) Option to draw stroke outlines around each embedded cell box.
