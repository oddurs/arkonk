# The journey

ARKONK's journey is one day, from dawn to the next dawn: 64 sectors in 8
chapters of 8. Each chapter teaches one idea in its first sectors, develops
it, then remixes it with everything learned before. Its eighth sector is a
set piece, and clearing it in a journey earns the chapter's extra life.

This page is the design record: what each chapter teaches, the difficulty
curve, where the original twelve sectors went, and the rules for the gate
brick, Blue Hour's darkness and par times. The content itself lives in
`crates/ark/src/sectors.rs` (layouts and rules) and
`crates/ark-text/src/tables/en.rs` (names and tips).

## The chapters

| # | Chapter | Sectors | Teaches | Feel |
|---:|---|---|---|---|
| 1 | Daybreak | 1–8 | steering, Wide, Slow | open layouts at gentle speed; no armour, no cores |
| 2 | Morning | 9–16 | Anchor and aiming; first armour | pockets you aim into, two-hit bricks |
| 3 | Zenith | 17–24 | relay cores and chains | set pieces that pay off one clean shot |
| 4 | Golden Hour | 25–32 | Multiball; dense fields | busy, generous, fast scoring |
| 5 | Afterlight | 33–40 | Phase; armoured shells around cores | pierce the shell, ignite the core |
| 6 | Blue Hour | 41–48 | darkness | the board shows only near the light you carry |
| 7 | Eclipse | 49–56 | gate bricks on a beat | one small, deterministic rule |
| 8 | Aurora | 57–64 | everything, faster | the finale, earned |

Every power first appears as a sector's opening capsule, so it teaches
itself: Wide in sector 1, Slow in 2, Anchor in 9, Multiball in 25 and Phase
in 33. Random drops only ever pick powers that an earlier opening has taught
(`tuning::POWER_UNLOCKS` is derived from the openings, and a test holds it to
that). The one exception is the finishing assist, which can hand a single
Anchor catch to a stalled rally with one or two bricks left in any sector; it
is a lifeline, not a capsule, and its prompt explains itself.

## The difficulty curve

Difficulty climbs inside each chapter and steps back at each chapter's start,
so a new idea is learned in calm. Four measures carry it: the serve speed
(before the rally speed-up of up to 80 px/s), the brick count, the armour
share (bricks of two or three hit points) and the core share. Eclipse and
Aurora add the gate share, and Blue Hour the darkness.

Serve speed by sector, one bar per sector, 420 to 625 px/s:

```
Daybreak     ▁▁▁▁▁▁▁▂  420–460
Morning      ▁▁▁▂▂▂▂▃  440–485
Zenith       ▂▂▂▃▃▃▃▄  460–510
Golden Hour  ▃▃▃▃▄▄▄▅  480–535
Afterlight   ▃▄▄▄▄▅▅▆  500–560
Blue Hour    ▃▃▄▄▄▄▅▅  490–550
Eclipse      ▄▄▅▅▅▆▆▆  520–580
Aurora       ▅▆▆▆▇▇▇█  550–625
```

Blue Hour steps further back than the other chapters: darkness is the
difficulty there, so its speeds sit below Afterlight's.

Every sector, with its measures. A `*` marks one of the original twelve. Par
is the Swift medal's target, derived as described under [Par times](#par-times).

**1 · Daybreak** (steering, Wide and Slow)

| # | Sector | Opens with | Speed | Bricks | Armour | Cores | Gates | Dark | Par |
|---:|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1 | First Light * | Wide | 420 | 34 | – | – | – | – | – |
| 2 | Drift | Slow | 425 | 24 | – | – | – | – | – |
| 3 | Horizon | Wide | 430 | 30 | – | – | – | – | – |
| 4 | Glimmer | Slow | 435 | 25 | – | – | – | – | – |
| 5 | Skylark | Wide | 440 | 24 | – | – | – | – | – |
| 6 | Lanterns | Slow | 445 | 36 | – | – | – | – | – |
| 7 | Tidewater | Wide | 450 | 32 | – | – | – | – | – |
| 8 | Sunrise | Slow | 460 | 44 | – | – | – | – | – |

**2 · Morning** (Anchor, aiming, first armour)

| # | Sector | Opens with | Speed | Bricks | Armour | Cores | Gates | Dark | Par |
|---:|---|---|---:|---:|---:|---:|---:|---:|---:|
| 9 | Satellites * | Anchor | 440 | 36 | 11 % | – | – | – | – |
| 10 | Dewpoint | Wide | 445 | 28 | 29 % | – | – | – | – |
| 11 | Aperture | Anchor | 450 | 34 | 29 % | – | – | – | – |
| 12 | Cloister | Anchor | 455 | 36 | 11 % | – | – | – | – |
| 13 | Keystone | Slow | 460 | 26 | 23 % | – | – | – | – |
| 14 | Sundial | Wide | 470 | 32 | 19 % | – | – | – | – |
| 15 | Pinhole | Anchor | 475 | 41 | 27 % | – | – | – | – |
| 16 | Windrose | Anchor | 485 | 36 | 33 % | – | – | – | – |

**3 · Zenith** (relay cores and chains)

| # | Sector | Opens with | Speed | Bricks | Armour | Cores | Gates | Dark | Par |
|---:|---|---|---:|---:|---:|---:|---:|---:|---:|
| 17 | Slipstream * | Anchor | 460 | 36 | – | 11 % | – | – | – |
| 18 | Filament | Wide | 465 | 42 | – | 24 % | – | – | – |
| 19 | Meridian | Anchor | 470 | 36 | – | 33 % | – | – | – |
| 20 | Cascade | Wide | 480 | 35 | – | 40 % | – | – | – |
| 21 | Crossfade * | Anchor | 485 | 54 | 11 % | 30 % | – | – | – |
| 22 | Switchback | Anchor | 490 | 64 | – | 38 % | – | – | – |
| 23 | Solstice | Slow | 500 | 54 | – | 33 % | – | – | – |
| 24 | Resonance * | Anchor | 510 | 42 | 5 % | 24 % | – | – | – |

**4 · Golden Hour** (Multiball, dense fields)

| # | Sector | Opens with | Speed | Bricks | Armour | Cores | Gates | Dark | Par |
|---:|---|---|---:|---:|---:|---:|---:|---:|---:|
| 25 | Prism * | Multi | 480 | 52 | 19 % | 15 % | – | – | – |
| 26 | Honeycomb | Multi | 485 | 54 | – | 11 % | – | – | – |
| 27 | Spindrift | Multi | 495 | 42 | 10 % | – | – | – | – |
| 28 | Undertow * | Wide | 500 | 64 | 19 % | 19 % | – | – | – |
| 29 | Harvest | Multi | 510 | 72 | 17 % | 11 % | – | – | – |
| 30 | Kaleidoscope | Anchor | 515 | 60 | 10 % | 30 % | – | – | – |
| 31 | Tapestry | Multi | 525 | 66 | 27 % | 18 % | – | – | – |
| 32 | Long Shadows | Multi | 535 | 54 | 11 % | 33 % | – | – | – |

**5 · Afterlight** (Phase, armoured shells around cores)

| # | Sector | Opens with | Speed | Bricks | Armour | Cores | Gates | Dark | Par |
|---:|---|---|---:|---:|---:|---:|---:|---:|---:|
| 33 | Afterglow * | Phase | 500 | 58 | 17 % | 14 % | – | – | – |
| 34 | Chrysalis | Phase | 510 | 30 | 40 % | 27 % | – | – | – |
| 35 | Geode | Phase | 515 | 38 | 47 % | 21 % | – | – | – |
| 36 | Parallax * | Phase | 525 | 72 | 33 % | 22 % | – | – | – |
| 37 | Citadel | Phase | 530 | 58 | 59 % | 10 % | – | – | – |
| 38 | Nautilus | Phase | 540 | 65 | 65 % | 35 % | – | – | – |
| 39 | Vespers | Anchor | 550 | 48 | 50 % | 25 % | – | – | – |
| 40 | Supernova * | Multi | 560 | 48 | 17 % | 33 % | – | – | – |

**6 · Blue Hour** (darkness)

| # | Sector | Opens with | Speed | Bricks | Armour | Cores | Gates | Dark | Par |
|---:|---|---|---:|---:|---:|---:|---:|---:|---:|
| 41 | Moonrise * | Multi | 490 | 50 | 16 % | 24 % | – | – | – |
| 42 | Gloaming | Slow | 500 | 40 | 15 % | 10 % | – | 30 % | – |
| 43 | Lamplight | Wide | 505 | 42 | 10 % | 14 % | – | 50 % | – |
| 44 | Fireflies | Slow | 515 | 35 | – | 20 % | – | 65 % | – |
| 45 | Lighthouse | Anchor | 520 | 42 | 10 % | 5 % | – | 80 % | – |
| 46 | Nocturne | Multi | 530 | 41 | 15 % | 7 % | – | 90 % | – |
| 47 | Deep Field | Phase | 540 | 31 | 19 % | 19 % | – | 95 % | – |
| 48 | Constellation | Anchor | 550 | 30 | 13 % | 53 % | – | 100 % | – |

**7 · Eclipse** (gate bricks on a beat)

| # | Sector | Opens with | Speed | Bricks | Armour | Cores | Gates | Dark | Par |
|---:|---|---|---:|---:|---:|---:|---:|---:|---:|
| 49 | Penumbra | Slow | 520 | 36 | – | – | 22 % | – | – |
| 50 | Metronome | Wide | 530 | 52 | – | – | 15 % | – | – |
| 51 | Corona | Anchor | 535 | 40 | – | 30 % | 40 % | – | – |
| 52 | Syzygy | Slow | 545 | 39 | 31 % | 21 % | 8 % | – | – |
| 53 | Pulsar | Multi | 550 | 38 | – | 21 % | 53 % | – | – |
| 54 | Shutter | Phase | 560 | 48 | 25 % | – | 42 % | – | – |
| 55 | Umbra | Anchor | 570 | 47 | 11 % | 21 % | 26 % | – | – |
| 56 | Totality | Phase | 580 | 58 | 38 % | 24 % | 38 % | – | – |

**8 · Aurora** (everything, faster)

| # | Sector | Opens with | Speed | Bricks | Armour | Cores | Gates | Dark | Par |
|---:|---|---|---:|---:|---:|---:|---:|---:|---:|
| 57 | Solar Wind | Wide | 550 | 42 | 33 % | 17 % | 17 % | – | – |
| 58 | Borealis | Multi | 560 | 37 | 35 % | 16 % | 11 % | – | – |
| 59 | Ribbons | Slow | 570 | 46 | 9 % | 39 % | 13 % | – | – |
| 60 | Polar Night | Anchor | 580 | 42 | 33 % | 10 % | 10 % | 80 % | – |
| 61 | Cathedral | Phase | 590 | 38 | 58 % | 21 % | 11 % | – | – |
| 62 | Shimmer | Multi | 600 | 54 | 30 % | 15 % | 13 % | – | – |
| 63 | Event Horizon | Phase | 610 | 56 | 36 % | 25 % | 39 % | – | – |
| 64 | Homecoming * | Phase | 625 | 58 | 31 % | 28 % | – | – | – |

## Where the original twelve went

The original journey was Daybreak 1–4, Blue Hour 5–8 and Afterlight 9–12. Each
sector kept its slug and its layout, so medals and best times still mean the
same board; serve speeds and par times follow the new curve.

| Slug | Was | Now | Why |
|---|---|---|---|
| `first_light` | Daybreak 1 | Daybreak 1 | Open, no armour: still the first lesson |
| `satellites` | Daybreak 2 | Morning 9 | Opens with Anchor and has the first armour |
| `slipstream` | Daybreak 3 | Zenith 17 | Isolated cores: the calm introduction to relays |
| `resonance` | Daybreak 4 | Zenith 24 | A ring of cores that pays off one shot: Zenith's set piece |
| `prism` | Blue Hour 5 | Golden Hour 25 | Opens with Multiball, which Golden Hour teaches |
| `crossfade` | Blue Hour 6 | Zenith 21 | Two relay lines: a chain lesson |
| `undertow` | Blue Hour 7 | Golden Hour 28 | A dense field of cores behind armour |
| `moonrise` | Blue Hour 8 | Blue Hour 41 | Stays: the moon rises as the light goes. It opens the chapter fully lit |
| `afterglow` | Afterlight 9 | Afterlight 33 | Opens with Phase, which Afterlight teaches |
| `parallax` | Afterlight 10 | Afterlight 36 | Three-hit shells beside cores |
| `supernova` | Afterlight 11 | Afterlight 40 | Cores around an armoured heart: Afterlight's set piece |
| `homecoming` | Afterlight 12 | Aurora 64 | Still the last sector; the journey comes home |

Only `satellites`, `slipstream`, `resonance`, `prism`, `crossfade`,
`undertow` and `homecoming` change chapter.

## The gate brick

Eclipse's one rule, and the only rule the journey adds. A gate is written `G`
in a layout and is a one-hit brick that is solid for part of a beat and a ghost
for the rest.

- **The beat** is set per sector as solid ticks then ghost ticks
  (`Sector::beat`). Every gate in a sector keeps the same beat.
- **The clock** is the sector's play time (`Game::sector_ticks`): a gate is
  solid while `sector_ticks % (solid + ghost) < solid`. Gates start solid, and
  the beat holds still while a ball waits to be served or results show. It uses
  no random draw and no float, so replays stay exact.
- **A ghost gate** neither collides nor takes damage. Balls, Phase and relay
  blasts all pass it by, and it costs no Phase charge.
- **A solid gate** is an ordinary one-hit brick: it bounces balls, breaks,
  scores, counts toward a capsule drop and can be destroyed by a relay blast.
- **Turning solid around a ball** never traps it: a ball that overlaps a gate
  at the tick it turns solid passes out of it as if phased, without a bounce
  or damage.
- **Gates count** toward the bricks left, so a sector clears only when every
  gate has been broken. Gates are never cores, so a chain stops at a gate.

Its look: solid, it is glass in its row's hue with a thin beat line inside its
foot that drains toward the centre until it ghosts (the status grammar: a
duration is a drain). As a ghost it is a dotted outline with the same line
draining until it returns, and its glass fades back in over the last quarter
second. Armour is nested rims; a gate never has them, so the two never read
alike.

## Blue Hour's darkness

Darkness is render-only: the simulation neither knows nor cares. Each sector
has an authored `darkness` from 0 to 100 (`Sector::darkness`), which only the
front end reads.

| Sector | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 60 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Darkness | 0 | 30 | 50 | 65 | 80 | 90 | 95 | 100 | 80 |

Moonrise opens the chapter fully lit, and the dark deepens sector by sector.
Aurora's Polar Night brings it back once, for the finale's remix.

- **Light comes from** each ball (full within 72 units of its centre, none past
  200) and the paddle's keel light, which shines straight up: full above the
  paddle, gone 96 units beyond its ends, and never more than 65 %. Moving the
  paddle under a part of the board is a way to look at it.
- **Each brick's shade** is `darkness × (1 − light)`, from its nearest light,
  computed per brick per frame. Its colours mix toward the field by the shade:
  body, glow and inner marks fully, its rim by at most 80 %, so every rim stays
  faintly visible and the board is never invisible.
- **Relay cores carry their own light**: their shade is at most half, which is
  what Fireflies (44) is about.
- **Reduced effects and high contrast** halve the darkness, so the board stays
  readable for players who need it.
- It costs no new shader, pass, texture or allocation: it changes only the
  vertex colours bricks already have.

## Par times

The Swift medal's target, `par_seconds`, comes from measured clears rather than
guesses. The test `crates/ark/tests/clears.rs` runs the benchmark's autopilot
(the paddle follows the first live ball, offset by a slow 36-unit sine wave,
and serve is always pressed) on every sector, three times with the wave's
phase shifted by a third of its period each time. It asserts that each run
clears its sector within ten minutes of play without losing the game, so **no
sector is impossible**.

**Rule:** par is the median of the three clear times × 1.5, rounded up to the
next 5 seconds.

The autopilot never misses, but it never aims either, so its times sit near a
steady human's. The factor leaves room for lost lives and serves. The test
checks par against the rule on the platform the replay goldens are recorded
on (macOS arm64, unoptimized), because the clear times are exact only there;
everywhere else it checks only that every sector clears. To see the table:

```sh
cargo test -p ark --test clears -- --nocapture
```

## Saves

Version-1 saves (`ARKONK 1`) index twelve sectors in the old order. Version 2
(`ARKONK 2`) indexes the 64 in the new order and writes a `record` line only
for sectors with a medal or a time. Reading a version-1 save maps it by slug:

- each record (medals and best time) moves to its slug's new position;
- the checkpoint moves with its sector, keeping score, lives and run time;
- unlocks are a prefix of the journey, so the new count reaches the furthest
  new position of any sector the old save had open. Nothing that was playable
  becomes locked; a player who had finished the old journey finds every sector
  open.

The next save writes version 2. Unknown keys are still ignored and damaged
lines still skipped, so later versions can add lines the same way.
