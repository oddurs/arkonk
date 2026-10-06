# ARKONK

A small, precise brick breaker in Rust. Twelve authored sectors across three
chapters, saved checkpoints, replayable sectors, and 36 medals to earn at your own
pace. Direct mouse control, 240 Hz collision simulation, and restrained feedback
keep the focus on the next bounce.

The CRT soul stays intact: a menu-only pixel marquee, amber scores, beveled bricks,
a silver-and-red paddle, phosphor trails, and softly curved glass. Each chapter
has its own palette. Gameplay keeps only the score, lives, and a quiet sector strip.
[Start screen](docs/attract.png) · [Sector map](docs/sectors.png)

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
| Space / Enter / Left click | Select a menu action or serve |
| P / Escape | Pause or resume |
| Arrows / W, S | Navigate menus |
| R in pause / results | Retry from the sector checkpoint |
| Escape in sector select / results | Main menu |
| M | Mute sound |
| [ / ] | Lower / raise volume |
| F | Toggle fullscreen |
| F3 | Performance overlay |
| C | Toggle CRT effects |
| Q / Close window | Quit |

Keyboard input takes control until the mouse moves again. Hit the ball with the
paddle's edges to steer it; a centered hit sends it straight up. Catch falling
capsules: **W** widens the paddle for 14 seconds, **S** slows all balls for 12
seconds, and **M** splits the ball up to a maximum of three. A life is lost only
when the last active ball drains. Power-up drops have a bounded dry spell, ball
speed rises gently through a rally, and rare angle corrections prevent long,
flat or perfectly vertical stalls. Leaving the window pauses play automatically.

## Your journey

**Daybreak**, **Blue Hour**, and **Afterlight** each contain four distinct layouts.
Clear a sector to unlock the next and stop at a results screen before continuing.
Each chapter awards an extra life, up to five. Three medals track each sector:

- **Clear:** finish the layout.
- **Clean:** finish without losing a life.
- **Swift:** finish within its target time; only active play counts.

A clear awards 1,000 points, with 500 each for Clean and Swift. Consecutive brick
breaks before a paddle bounce increase the chain bonus and subtly raise the hit
sound's pitch. Personal best times and medals persist across attempts.

**Continue Journey** restores the start of your saved sector, including score and
lives. Quitting mid-sector or choosing Retry returns to that checkpoint; partial
sector scores are not carried into retries. **Sector Select** lets you practice
unlocked layouts and improve medals without replacing your journey checkpoint or
journey high score. New Journey replaces the current checkpoint while keeping
unlocks, medals, times, and the personal best.

The playfield keeps its proportions when resized. Progress and settings live under
`~/Library/Application Support/arkonk` on macOS, `%LOCALAPPDATA%/arkonk` on Windows,
or `$XDG_DATA_HOME/arkonk` (falling back to `~/.local/share/arkonk`) on Linux.
The versioned `progress.txt` is written through a temporary file at
menu/sector boundaries; legacy `best.txt` scores are imported automatically.
No account, network connection, or asset download is used during play.

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

- Fixed 240 Hz simulation, interpolated balls, and VSync requested.
- Direct mouse tracking; the paddle renders at its latest position for lower latency.
- At most sixteen catch-up ticks per frame; discarded ticks appear in the overlay.
- Large stalls pause play instead of fast-forwarding into a lost life.
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
| Autopaddle gameplay across all twelve sectors | 0.141 µs | 0.174 µs | 0.196 µs | 0 |
| Three balls at 12,000 px/s, full particle/drop pools | 0.367 µs | 0.449 µs | 0.581 µs | 0 |

Each scenario measures 262,144 ticks in batches of 64. Times include benchmark
control and pool replenishment, but exclude graphics and audio. Percentiles are
of batch averages, not individual ticks. The benchmark asserts zero simulation
allocations, no exhausted collision budgets, actual brick impacts, and coverage
of all twelve sectors. Gameplay batches periodically start a new sector to ensure
coverage without depending on the autopaddle completing the journey. Results vary
by hardware and load.

F3 shows recent frame-time p95/p99, simulation time, CPU draw preparation time,
discarded ticks, and collision-budget counts. Draw timing excludes GPU execution
and presentation; it is not a GPU benchmark. The locally verified windowed
smoke test completes 661 frames, checking CRT rendering, title/pause/results/sector
screens, the stats overlay, and resizing. Statistics reset after frame 300 so PNG
captures and window resizing are excluded from the final steady-state sample.
Local repeat runs on a 120 Hz display delivered 116–118 FPS, about 0.15–0.23 ms
CPU draw preparation, and zero steady-state dropped ticks, with CRT on and off.
An earlier run had substantial display pacing spikes that did not reproduce in
the baseline comparison and repeat runs; these figures are observations, not a
universal frame-rate guarantee. The screenshots come from the actual game.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo build --locked --release
cargo run --locked --release --bin arkonk -- --smoke-test
cargo run --locked --release --bin arkonk -- --flow-test
```

The smoke test opens a window, launches a ball, follows it with the paddle,
writes `target/smoke-test.png` and presentation captures, reports frame statistics,
and exits automatically. To compare the renderer without CRT effects, add
`--no-crt` (that capture is written to `target/smoke-plain.png`).
Both graphical checks need a desktop session and leave your saved progress alone.
`--flow-test` drives the real menu input handlers through new journey, sector clear,
checkpoint restore, practice, focus pause, resume, and retry, with assertions at
transition boundaries.

`src/physics.rs` contains context-free geometry; `src/game.rs` contains the pure
fixed-step simulation. `src/levels.rs` authors the journey, `src/profile.rs` persists it, and
`src/timing.rs` schedules ticks. `src/main.rs` handles application transitions; `src/render.rs`,
`src/audio.rs`, and `src/perf.rs` handle presentation. `src/crt.rs` contains the
post-processing pass, and `src/pixel_font.rs` contains the original bitmap type.
Collision tests cover high
speed tunneling, rounded corners, departing/parallel trajectories, moving
paddle interception, bounce steering, life loss, powers, and level transitions.
Additional tests cover medals, checkpoint isolation, malformed saves, replacing
save files, drop cadence, anti-stall behavior, and timing at 30–360 Hz.
GitHub Actions runs formatting, Clippy, tests, release builds, and allocation
benchmarks on macOS, Linux, and Windows.

MIT licensed.
