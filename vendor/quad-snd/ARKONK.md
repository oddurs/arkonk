# ARKONK audio backend patch

Base: crates.io `quad-snd` **0.2.8**, the audio backend of macroquad 0.4. Upstream:
https://github.com/not-fl3/quad-snd . Its manifest declares MIT/Apache-2.0 but the
crate ships no license files, so `LICENSE-MIT` and `LICENSE-APACHE` here are the
same author's texts from `vendor/miniquad`. The examples, CI files and lockfile are
left out. Diff `src/` against the crates.io 0.2.8 source to see the whole patch.

## Why

macroquad builds its audio context inside `Context::new`, on the main thread,
while miniquad draws the first frame. Upstream, `AudioContext::new` opens the
device right there. On macOS it calls `AudioQueueStart` and `assert!`s on the
result. A wedged `coreaudiod` blocks that call for 15 seconds or more (reports
reach 17 minutes) before it returns an error. The assert then panics, the unwind
drops a half-built macroquad context whose destructor panics again, and the
process aborts with exit 134. No frame is ever drawn. On Windows and Linux the
device opens on a thread that panics instead, so those platforms already survived.

## What changed

- `AudioContext::new` no longer opens the device. Each desktop backend (CoreAudio,
  WASAPI, ALSA) parks its device start, and `quad_snd::start_output()` runs it on
  a new thread named `audio`. The returned channel reports `Ok` once the device
  plays, or why it failed. A start that panics drops the sender, which also
  reports failure. Sounds still load and decode on the calling thread whether
  or not the device ever starts.
- CoreAudio: the device setup moved into `start_queue`, unchanged except that
  each `assert!` returns an error naming the call and its `OSStatus`. A queue that
  fails to start is disposed. The mixer is never freed: it holds the receiving
  end of the control channel, and dropping it would make every later send print
  "Audio thread died".
- WASAPI and ALSA: the existing audio thread sends `Ok` once the device is set up.
  Their failure paths still panic on that thread, as before.

The game (`src/audio.rs`) never calls `start_output` in a silent test run. In
normal play it polls the channel once a frame and gives up after a few seconds,
so no frame waits on the device. Every `ARKONK:` comment marks a changed spot.

The Android and web backends are unchanged and still start in `new`, so
`start_output` reports an error there. ARKONK ships neither.

A later change replaces this audio path; until then keep the diff this small.
Check it with:

```sh
cargo check --locked -p quad-snd --target x86_64-pc-windows-msvc
cargo check --locked -p quad-snd --target x86_64-unknown-linux-gnu
target/release/arkonk --smoke-test --opengl            # never opens the device
target/release/arkonk --perf-test --audio              # opens it, hidden
```
