---
name: perf-baseline
description: Use when a change could affect frame time (rendering, layout, text, effects, the simulation), to measure it against main in the same session and produce the before/after table for the PR.
---

# Frame time against main

Numbers from different sessions, power states or window sizes don't compare, so build
both sides now.

1. Build `main` in a throwaway worktree. It shares the cargo cache, so this is quick
   after the first time:
   ```sh
   git fetch origin
   git worktree add --detach ../.worktrees/arkonk/_baseline origin/main
   cargo build --locked --release --bin arkonk --manifest-path ../.worktrees/arkonk/_baseline/Cargo.toml
   ```
2. Run each side three times, alternating baseline and branch, on the same display,
   with no validation layers:
   ```sh
   <bin> --perf-test --effects-test >>target/perf-<side>.log 2>&1
   <bin> --perf-test --effects-test --opengl >>target/perf-<side>-gl.log 2>&1
   ```
   `<bin>` is `../.worktrees/arkonk/_baseline/target/release/arkonk` for the baseline
   and `target/release/arkonk` for the branch.
3. Read the `Whole-run pacing` lines (FPS, p95, p99, worst) and the `Foreground check`
   line. A run with unfocused frames is invalid; rerun it.
4. Report the median of the three per backend:

   | Backend | Side | p95 ms | p99 ms | worst ms |

   A regression beyond run-to-run spread blocks the PR.
5. If the simulation changed, also compare the `score checksum` line printed by
   `cargo bench --locked -p ark --bench tick`. It must be identical unless the PR
   changes behaviour on purpose.
6. Clean up: `git worktree remove --force ../.worktrees/arkonk/_baseline`.
