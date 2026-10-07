# ARKONK macOS backend patch

Base: crates.io `miniquad` **0.4.11**, unchanged rendering API.
One macOS focus-query helper initializes diagnostics accurately after startup. Upstream:
https://github.com/not-fl3/miniquad . Both original licenses are included.
Only the Apple graphics backend and one unused-function annotation differ;
Windows/Linux retain the released backend implementation.

ARKONK uses one color attachment, single sampling, and no depth/stencil testing.
The macOS Metal view and pipelines therefore omit depth/stencil attachments.
Offscreen color targets match the view's BGRA format. Pipelines declare only
attachment zero. Without these fixes the released backend fails Metal validation
and offscreen render targets appear black.

Scissor coordinates use the active render attachment's dimensions, including
Retina offscreen targets and drawable/window size differences during resizing.
These fixes follow the corresponding upstream work inspected at revision
`39c928fb7e09fd44e45f37fe9a98503968bca412`; the full unreleased branch is not required.
Uniform buffer flushes cover the actual written offset.

Foreground rendering explicitly requests interactive thread QoS; focus loss
returns the thread to utility QoS. A development-shell launch was observed
inheriting utility priority, inappropriate for a frame-sensitive foreground loop.

GPU submission uses two frame slots. Each has its own uniform buffer and a
disjoint half of each existing vertex/index buffer rotation array. A retained
command buffer guards each slot; its completion is awaited only before reuse.
The frame index advances once per submission, not once per uniform upload.
This removes the unconditional GPU completion wait at the end of every frame
without racing CPU writes against GPU reads or adding an unbounded frame queue.
CVDisplayLink provides the CPU cadence. CAMetalLayer retains its default display
synchronization: disabling that property can introduce tearing even with a paced
CPU loop ([Apple documentation](https://developer.apple.com/documentation/quartzcore/cametallayer/displaysyncenabled)).
There is no free-running render loop.

The total vertex/index buffer allocation is unchanged; uniform storage is reduced
from three buffers to two. A guard catches exhausting a slot's rotation budget.

This is an application-specific patch, not a general replacement for Miniquad.
Adding depth rendering, MSAA, or multiple render targets requires revisiting it.
Keep the diff small when upgrading the dependency. Validate changes with:

```sh
cargo build --locked --release
MTL_DEBUG_LAYER=1 MTL_SHADER_VALIDATION=1 target/release/arkonk --effects-test
MTL_DEBUG_LAYER=1 MTL_SHADER_VALIDATION=1 target/release/arkonk --flow-test
```

Measure frame pacing separately with `--perf-test --effects-test`, without Metal
validation, screenshots, resizing, or concurrent compiler/graphics workloads.
