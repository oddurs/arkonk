---
paths:
  - "src/render.rs"
  - "src/render/**"
  - "src/ui.rs"
  - "src/atlas.rs"
---

# Presentation

- Lay out in scene units of the fixed 960 × 900 scene. One scale factor maps the scene
  to the window. Snap hairlines and rims to physical pixels, at least 1 px.
- Spacing comes from the scale in `src/ui.rs` and type from
  `crates/ark-glyphs/src/spec.rs`. A new magic number needs a reason.
- The draw path reuses buffers. No `Vec::new`, `format!` or `String` per frame; write
  into the existing scratch buffers.
- Visual changes:
  - Check them in `--locale pseudo` (the longest strings) and at the Steam Deck size
    1280 × 800.
  - Include OpenGL captures in the PR.
