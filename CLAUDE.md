# ARKONK

A brick breaker sold on Steam (Windows, macOS, Linux) that must also run on tiny,
low-resolution machines. MIT, public repository.

## Commands

- `scripts/task check` is the gate: format check, Clippy with warnings denied (every
  feature, `steam` included), tests and doc tests, release build, then the allocation
  benchmark. CI and the hooks call only `scripts/task`.
- One test: `cargo test -p ark --test replay`, or `cargo test --bin arkonk <name>`.
- `cargo run --locked --release -p fontbake -- --check` fails if the committed glyph
  atlases differ from a fresh bake. Any new character or type-size change means a
  rebake (skill: `rebake-glyphs`).
- Graphical checks open a real window, hidden on macOS unless `--show`: `target/release/arkonk --smoke-test --opengl`,
  `--flow-test`, `--effects-test`, `--perf-test`, plus `--locale <tag|pseudo>`
  (skill: `visual-check`).
- `./scripts/build-macos.sh` builds the app bundle. Packaging is
  `scripts/package.sh`; releasing is in `docs/RELEASING.md`.

## Decisions to keep

- `crates/ark` is the simulation. It has no dependencies, no I/O and no clock, and is
  deterministic at 240 ticks a second. A behaviour change fails the recorded digests
  in `crates/ark/tests/replay.rs` and moves the benchmark checksums. Only do that on
  purpose, and say so in the PR.
- Save files are pinned byte for byte by snapshot tests. A format change needs a
  migration that reads old saves, with a test.
- Never crash. `ark` sanitizes its input and never panics. The app degrades, logs and
  keeps running when audio, the GPU, saves, controllers or Steam fail. No
  `unwrap`/`expect` on runtime data.
- Display text lives only in `crates/ark-text`, one exhaustive `match` per locale.
  `ark` knows sectors and powers by id and slug.
- Glyphs are Noto atlases baked at build time (`crates/ark-glyphs/data/*.bin`). There
  are no font files at runtime.
- `vendor/miniquad` carries Metal fixes. Read `vendor/miniquad/ARKONK.md` before
  touching it.
- Only `src/steam.rs` calls Steamworks, behind the opt-in `steam` feature.
- Rendering is one batched pass. No new shaders, render targets or blurs, and no
  allocation on the draw path. Measure frame-time changes against a `main` baseline
  (skill: `perf-baseline`).

## Gotchas

- Metal has no texture readback, so screenshots come only from `--opengl` runs
  (`target/*.png`). Check Metal with `MTL_DEBUG_LAYER=1 MTL_SHADER_VALIDATION=1`, and
  turn validation off for timings.
- Never take full-screen screenshots; they capture the owner's desktop.
- Graphical tests on macOS run hidden: no focus, Dock icon, visible window or sound,
  and the game behaves as focused. `--show` opens a normal window to watch. Timings still
  skew under load, so only one agent runs them at a time.
- The owner's shell aliases `cp`, `mv` and `rm` to prompt. Use `command cp -f` and its
  siblings. Never run anything that waits for input: no `-i`, no editors, no pagers
  (`git --no-pager`, `GIT_EDITOR=true`).
- `git push` runs the full check in the pre-push hook, which takes several minutes.
  Run it in the background and poll its output.
- Smoke-test autoplay follows the wall clock, so live-play captures are not
  byte-reproducible. Compare only static screens.
- Non-English strings are drafts until a native speaker reviews them
  (`docs/localization.md`).

## Workflow

- Never commit to `main`. It advances only through a merged pull request.
- One unit of work, one worktree, one branch, one PR: `scripts/agent start feat/the-thing`.
  The worktree goes in `../.worktrees/arkonk/<branch>/`. Branch types: feat, fix, chore,
  docs, perf, refactor, test.
- `scripts/task check` must pass before a PR. The hooks enforce it; never `--no-verify`
  and never `|| true`.
- Conventional Commits: subject ≤ 72 characters, imperative, no trailing period. The
  body says why.
- A PR description has three parts: Problem, Approach, and Look at this sceptically.
  Visual changes include captures; changes that touch frame time include a before and
  after table.
- `scripts/agent sync` rebases onto `main`. `scripts/agent done` squash-merges a green
  PR, then removes the worktree and the branch.
- IMPORTANT: No AI or assistant attribution in commits, PRs, comments, or docs.
