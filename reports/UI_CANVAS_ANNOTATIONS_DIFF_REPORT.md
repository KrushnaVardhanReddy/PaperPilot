# UI Canvas Annotations & Diff Report

This report summarizes the E2E verification of visual interactive canvas features in PaperPilot.

## 1. Sticky Notes & Annotations
- **Adding Notes:** A sticky note (`.annot-note`) can be correctly added to the canvas at the specified mouse coordinates by clicking with the note tool.
- **Attributes:** The note successfully binds to the `New Note` title and custom attributes. It supports content editing and attribute updates.
- **Repositioning:** The note supports being dragged and repositioned to updated canvas coordinates.
- **Annotation Panel:** The side panel (`#right-panel-tab-annotations`) updates immediately with the new annotation list.
- **Deletion:** Hovering the item in the list shows the removal button, which correctly removes the item from both the DOM canvas layer and the list.

## 2. Pointer, Highlighting & Markup Tools
- **Draw Tool & Highlight:** Simulating `pointerdown`, `pointermove`, `pointerup` generates correct stroke paths (svg `polyline`) and highlight bounded boxes (`.annot-highlight`).
- **Annotation Tracking:** The annotations list updates accurately, categorizing items by `type` (e.g. `highlight`, `pen`).

## 3. Visual Pixel Diff Slider
- **Initialization:** Triggering the comparison (e.g. `Control+D`) correctly opens the comparison view.
- **Split Slider:** The split handle (`#diff-split-handle`) is functional; it defaults to 50% split width.
- **Interactivity:** A pointer drag operation on the slider handle successfully changes the `left` CSS property to reveal differences.

## 4. Zoom Toolbar & Thumbnail Navigation
- **Zoom Controls:** Functions like `zoomOut`, `zoomIn`, `fitWidth`, and preset resets to `100%` update the viewer correctly.
- **Thumbnails:** Clicking `#thumbnail-page-X` dynamically updates the global document state, which syncs with `#toolbar-page-input` seamlessly.
