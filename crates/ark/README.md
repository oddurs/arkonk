# ark

The ARKONK simulation: a fixed 240 Hz step, continuous circle-versus-rectangle
collision, the 64 authored sectors, and the progress codec. No dependencies,
no rendering, no audio, no files, no clocks. The crate documentation
(`cargo doc -p ark --open`) explains the design: the fixed step, collision
budget, fixed pools, determinism, the `Sandbox`, and the save format.

| Module | Holds |
| --- | --- |
| `clock` | `TICK_HZ`, `FixedClock` |
| `geom` | `V2`, `Rect`, `sweep_circle_rect` |
| `field` | walls, the 12 × 7 grid, `Cell`, `CellSet` |
| `sectors` | `SectorId`, `Chapter`, `SECTORS` (parsed at compile time) |
| `tuning` | every rule constant, named |
| `progress` | `Progress`, `Checkpoint`, the save file's text format |
| (root) | `Game`, `Input`, `Events`, `Stage`, `Sandbox`, `Board`, `Ball`, … |

## Checks

```sh
cargo test -p ark          # unit tests, golden replays, doc tests
cargo bench -p ark         # timings; asserts zero allocations and full budgets
```

`tests/replay.rs` replays scripted sessions and compares a per-tick digest of
the gameplay state with `tests/snapshots/`. Any change in behaviour fails it.
The simulation is still `f32`, so the digests are exact on macOS arm64 at
opt-level 0 only; other platforms skip them.

Benchmark on an Apple M4 Pro (release profile, best of five runs):

| Scenario | Mean / tick |
| --- | ---: |
| Autopaddle gameplay across all twelve sectors | 0.18 µs |
| Three balls at 12,000 px/s, full pools | 0.41 µs |
| 84 connected cores, all five powers, full pools | 0.47 µs |
