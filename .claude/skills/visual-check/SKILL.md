---
name: visual-check
description: Use after changing anything drawn, laid out, or driven by input. Runs the graphical smoke, flow and effects tests on OpenGL and Metal, and collects the screenshot captures for a PR.
---

# Visual check

On macOS these tests run hidden: the window is transparent and click-through, never
takes focus, never opens the audio device, and the game behaves as focused. Add `--show` to watch
and hear one in a normal window. Timings still skew when runs overlap, so only one agent runs them at a time.
Never take a full-screen screenshot; use the captures the game writes.

1. Build once:
   ```sh
   cargo build --locked --release --bin arkonk
   ```
2. Run OpenGL in the background, writing output to a file, then poll the file:
   ```sh
   target/release/arkonk --smoke-test --opengl >target/smoke.log 2>&1
   target/release/arkonk --flow-test --opengl >target/flow.log 2>&1
   ```
   - The smoke run writes `target/smoke-test.png`, `target/layout-*.png` (Deck,
     1080p, 1440p, ultrawide, 4:3), `target/*-pad.png` (gamepad prompts) and
     `target/locale-switch.png`.
   - Add `--locale de`, `ja` or `pseudo` for other languages. `pseudo` has the
     longest strings and is the one that breaks layouts.
3. Run Metal with validation. There are no captures; it passes when it exits 0 with no
   validation errors in the log:
   ```sh
   MTL_DEBUG_LAYER=1 MTL_SHADER_VALIDATION=1 target/release/arkonk --flow-test >target/metal.log 2>&1
   MTL_DEBUG_LAYER=1 MTL_SHADER_VALIDATION=1 target/release/arkonk --effects-test >target/metal-fx.log 2>&1
   ```
4. Read every log to the end. A test that "seems fine" without its final line has not
   passed.
5. For the PR: copy the relevant captures out of `target/` (it is cleaned) and attach
   them. Name them `<screen>-<locale>.png`.

Static screens are byte-stable between runs, hidden or shown: `attract`, `sectors`,
`sectors-pad` and the `layout-*-title` and `layout-*-sectors` captures. Live play
follows the wall clock and is not, so don't diff those, nor anything drawn over it
(`paused`, `clear`, `game-over`). `locale-switch` lands mid-way through an animated
resize, so its size can vary by a few pixels.
