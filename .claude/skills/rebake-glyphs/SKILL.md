---
name: rebake-glyphs
description: Use after adding or changing strings, characters, type roles, sizes or fonts, when fontbake --check fails, or when text renders as missing boxes. Rebakes the Noto glyph atlases and verifies them.
---

# Rebake the glyph atlases

1. Bake. The first run downloads the pinned Noto releases into `target/fontbake/` and
   checks their SHA-256:
   ```sh
   cargo run --locked --release -p fontbake >target/bake.log 2>&1
   ```
   Run it in the background and poll the log. A cold run takes minutes.
2. Verify, which must print no differences:
   ```sh
   cargo run --locked --release -p fontbake -- --check
   cargo test --locked -p ark-glyphs
   ```
3. Record the size change of each `crates/ark-glyphs/data/*.bin` in the PR, from
   `git diff --stat`. Size matters for the portable builds.
4. Commit the `.bin` files with the change that needed them, not separately.

- A character missing from every font means a new font. Pin it in
  `tools/fontbake/src/sources.rs` with its SHA-256, and add its licence under
  `packaging/licenses/fonts/`.
- Glyph coverage per script is set in `tools/fontbake/src/charsets.rs`. Atlases are
  split by script, and the CJK, Thai and Arabic ones sit behind the default
  `scripts` feature.
