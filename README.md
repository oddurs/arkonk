# ARKONK

A compact Arkanoid-style game written in Rust. Five brick layouts, three lives,
angle-controlled paddle bounces, reinforced bricks, combo scoring, three power-ups,
particles, ball trails, synthesized sound, and a saved best score.

Late-80s CRT arcade art direction: a menu-only pixel marquee, amber score displays,
beveled spectrum bricks, a silver-and-red paddle, phosphor trails, and curved CRT
glass. Gameplay keeps a sparse score/lives display and thin playfield edges;
controls and settings live on the start and pause screens.
[See the start screen](docs/attract.png).

![ARKONK in play](docs/screenshot.png)

## Play

Install stable Rust (1.88 or newer), then:

```sh
git clone https://github.com/oddurs/arkonk.git
cd arkonk
cargo run --locked --release
```

No downloaded assets or working-directory-dependent resource files are needed.
The original 5×7 pixel alphabet is packed into an atlas at startup; artwork is
drawn with native geometry, and sounds are synthesized and decoded once.
Native macOS is verified locally. Windows and Linux have CI build coverage.
On Linux, install the native libraries first (Debian/Ubuntu):

```sh
sudo apt-get install libasound2-dev libx11-dev libxi-dev libgl1-mesa-dev
```

| Control | Action |
| --- | --- |
| Mouse / A, D / Left, Right | Move paddle |
| Space / Enter / Left click | Start or launch |
| P / Escape | Pause or resume |
| R | Restart |
| Enter after game over or victory | Restart |
| M | Mute sound |
| F | Toggle fullscreen |
| F3 | Performance overlay |
| C | Toggle CRT effects |
| Q / Close window | Quit |

Keyboard input takes control until the mouse moves again. Hit the ball with the
paddle's edges to steer it; a centered hit sends it straight up. Catch falling
capsules: **W** widens the paddle for 14 seconds, **S** slows all balls for 12
seconds, and **M** splits the ball up to a maximum of three. A life is lost only
when the last active ball drains. Clear all five sectors to win.

The playfield keeps its proportions when resized. Best scores are stored under
`~/Library/Application Support/arkonk` on macOS, `%LOCALAPPDATA%/arkonk` on Windows,
or `$XDG_DATA_HOME/arkonk` (falling back to `~/.local/share/arkonk`) on Linux.

## CRT presentation

The scene renders into one fixed 960×900 buffer. A single five-tap shader adds
restrained phosphor bloom, scanlines, an RGB grille, curved glass, edge shading,
and faint grain. Scanlines fade at smaller window sizes. Mouse coordinates use
the same curvature mapping as the screen, keeping paddle control aligned.

Brick-hit flashes, floating scores, paddle impact lights, and pickup rings use
fixed pools in the renderer. A restart reuses the scene buffer and shader.
Press **C** for the clean pixel-art view, or start without effects:

```sh
cargo run --locked --release -- --no-crt
```

## Performance

- Fixed 120 Hz simulation with interpolated rendering and VSync requested.
- At most eight catch-up ticks per frame; discarded ticks appear in the overlay.
- Continuous circle/rectangle collisions, including exact rounded corners.
- Moving-paddle collisions use relative motion, preserving interception timing.
- A compact 12 × 7 brick grid limits collision tests to cells on the swept path.
- Fixed arrays for three balls, 384 particles, and twelve power-up drops.
- Eight collision resolutions per ball per tick. If exhausted, the ball stays
  at its last safe position; it never advances unchecked through bricks.
- Single-threaded simulation and batched geometry through Macroquad.
- Reused text buffers; no heap allocation in simulation updates.

Run the headless benchmark:

```sh
cargo run --locked --release --bin benchmark
```

Measured on an **Apple M4 Pro, arm64, Rust 1.98.1**, release profile:

| Scenario | Mean / tick | Batch p95 / tick | Batch p99 / tick | Heap allocations |
| --- | ---: | ---: | ---: | ---: |
| Autopaddle gameplay through all five levels | 0.168 µs | 0.211 µs | 0.248 µs | 0 |
| Three balls at 12,000 px/s, full particle/drop pools | 0.371 µs | 0.444 µs | 0.499 µs | 0 |

Each scenario measures 262,144 ticks in batches of 64. Times include benchmark
control and pool replenishment, but exclude graphics and audio. Percentiles are
of batch averages, not individual ticks. The benchmark asserts zero simulation
allocations, no exhausted collision budgets, actual brick impacts, and coverage
of all five levels in the gameplay scenario. Results vary by hardware and load.

F3 shows recent frame-time p95/p99, simulation time, CPU draw preparation time,
discarded ticks, and collision-budget counts. Draw timing excludes GPU execution
and presentation; it is not a GPU benchmark. The locally verified windowed
smoke test completes 661 frames, checking CRT rendering, start/pause/game-over
screens, the stats overlay, and a resized window before reporting steady-state
statistics. The screenshot above was captured from the actual game.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo build --locked --release
cargo run --locked --release --bin arkonk -- --smoke-test
```

The smoke test opens a window, launches a ball, follows it with the paddle,
writes `target/smoke-test.png` and presentation captures, reports frame statistics,
and exits automatically. To compare the renderer without CRT effects, add
`--no-crt` (that capture is written to `target/smoke-plain.png`).
It needs a graphical session and does not change your saved score.

`src/physics.rs` contains context-free geometry; `src/game.rs` contains the pure
fixed-step simulation. `src/main.rs` handles input and timing; `src/render.rs`,
`src/audio.rs`, and `src/perf.rs` handle presentation. `src/crt.rs` contains the
post-processing pass, and `src/pixel_font.rs` contains the original bitmap type.
Collision tests cover high
speed tunneling, rounded corners, departing/parallel trajectories, moving
paddle interception, bounce steering, life loss, powers, and level transitions.
GitHub Actions runs formatting, Clippy, tests, release builds, and allocation
benchmarks on macOS, Linux, and Windows.

MIT licensed.
