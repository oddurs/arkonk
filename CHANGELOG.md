# Changelog

All notable changes to ARKONK are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/). A release's section becomes its
GitHub Release notes, so each `## [x.y.z]` heading must exist before tagging.

## [Unreleased]

### Added

- Twelve authored sectors across three chapters (Daybreak, Blue Hour,
  Afterlight) with saved checkpoints, replayable sectors, and 36 medals.
- Five capsules (Wide, Slow, Multi, Anchor, Phase) and relay cores that chain
  explosions through adjacent bricks.
- Direct mouse, keyboard, and gamepad control; prompts follow the last device used.
- A fixed 240 Hz simulation with continuous collisions and no heap allocation
  per tick, verified by a headless benchmark.
- A flat, modern presentation with an original 5×7 pixel typeface,
  synthesized sound, native Metal on macOS, and OpenGL on Windows and Linux.
- Release packaging: per-platform zips with third-party license notices, a
  universal macOS app with Developer ID signing and notarization, a Windows
  executable with an icon and version resource and no console window, and a
  Linux build against the Steam Runtime.
- `arkonk --version`.
