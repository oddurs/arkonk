# Localization

ARKONK's text lives in two `no_std` crates that every port can share:

- **`crates/ark-text`**: the shipped `Locale`s, a typed `TextId` for every visible
  string, one table per language, and formatting (argument order, digit
  grouping, capsule icons, the pseudo locale). It holds no font data.
- **`crates/ark-glyphs`**: Noto Sans and Noto Sans CJK, pre-rasterized into
  hinted strikes at fixed pixel sizes, with advances and GPOS pair kerning,
  plus the line layout (measure, place, wrap) that the game draws with.

`ark` itself knows sectors, chapters and powers only by id and slug.

## Languages

English is the source. Every other table is a **draft that needs native
review**; none has been checked by a native speaker yet.

| Locale | Tag | Steam language | Status |
| --- | --- | --- | --- |
| English | `en` | english | Source |
| French | `fr` | french | Draft, needs native review |
| German | `de` | german | Draft, needs native review |
| Spanish (Spain) | `es-ES` | spanish | Draft, needs native review |
| Spanish (Latin America) | `es-419` | latam | Draft, needs native review. Inherits `es-ES` except a few words (Mouse, Enter, pad direccional). |
| Portuguese (Brazil) | `pt-BR` | brazilian | Draft, needs native review. Also chosen for other Portuguese. |
| Italian | `it` | italian | Draft, needs native review |
| Polish | `pl` | polish | Draft, needs native review |
| Russian | `ru` | russian | Draft, needs native review |
| Chinese (Simplified) | `zh-Hans` | schinese | Draft, needs native review |
| Japanese | `ja` | japanese | Draft, needs native review |
| Korean | `ko` | koreana | Draft, needs native review |
| Pseudo | `en-XA` | – | Test only: accented, bracketed, about 40 % longer |

Traditional Chinese is not shipped; `zh-TW`, `zh-HK` and `zh-Hant` fall back
to the next preference, then English.

### Which language plays

1. `--locale <tag>` on the command line (`--locale pseudo` works too). Not saved.
2. The player's choice, saved as a `locale <bcp47>` line in `progress.txt`.
   The line is written only when a choice exists. Older builds skip it: the
   progress codec reads it as a damaged line, because its value is not a number.
3. Steam's game language, in builds with the `steam` feature while Steam runs.
4. The operating system's preferred languages, in order (`sys-locale`).
5. English.

A language this build cannot draw (CJK without the `cjk` feature) is skipped.
`logs/arkonk.log` records the choice and where it came from.

For a language picker: `Locale::ALL` minus `Locale::Pseudo`,
`Locale::native_name()` for the labels (never translated), and
`ark_glyphs::supports(locale)` to hide what the build cannot draw. To apply a
choice, set `profile.settings.locale = Some(locale)`, mark the profile dirty,
and call `Renderer::set_locale(locale)`, which rebuilds the glyph atlas in a
few milliseconds. `None` returns to following Steam or the system.

## Writing and reviewing strings

Each table is a Rust `match` over `TextId` with no wildcard, so a new id does
not compile until every language has it. Sector names and tips are
fixed-length arrays for the same reason. `es-419` deliberately falls back to
`es` for strings that read the same.

- **Slots.** `{0}`, `{1}` … are arguments; move them to wherever the language
  puts them. Each id's doc comment says what each slot holds. A test checks that
  every translation uses exactly the source's slots.
- **Capsule icons.** `{icon:wide}`, `{icon:slow}`, `{icon:multi}`, `{icon:anchor}`
  and `{icon:phase}` stand for the capsules W, S, M, A and P. The game draws
  each as the capsule itself, a coloured pill with its letter, 1.5 em wide,
  so a tip teaches the icon the player will see falling; plain text output
  (`ark_text::write`) prints the letter. They are gameplay iconography and the
  same in every language; keep the placeholder, never write the letter, and
  leave a space on each side.
- **Casing is authored, never computed.** Labels (`SCORE`, `LIVES`, medal and
  chapter names) are written in capitals where the script has them; headings,
  buttons and hints in sentence case; sector names as the language writes
  titles. The code never changes case, so German ß, Turkish dotted i and any
  later language follow their own rules. A test rejects a label that is not
  already upper case (it would catch an ß, which upper-cases to SS).
- **No plurals.** Counts appear as label–value pairs (`MEDALS 7`), never
  inside a sentence, so no language needs plural forms.
- **Numbers.** `Arg::Count` groups digits the locale's way: `12,500`,
  `12.500`, `12 500` (French narrow no-break space), `12 500` (Russian, Polish
  no-break space); Spanish and Polish leave four-digit numbers ungrouped (CLDR
  minimum grouping digits of 2). Times are `mm:ss` everywhere.
- **Short forms.** `short` in a table gives a narrower wording, used only
  when the full one does not fit (for example `Continue journey` → `Continue`).
- **Precomposed text only.** Strings must be NFC with no combining marks: the
  atlases hold whole glyphs and do no mark positioning. The baker refuses
  anything else.

### Native review checklist

For each language, a native speaker who plays games should check:

- [ ] Every string in `crates/ark-text/src/tables/<lang>.rs` reads naturally
      as game UI: terse, consistent terminology, the register games in that
      language use (`du`/`vous`/`вы` …, as noted at the top of each table).
- [ ] Sector and chapter names work as names and are not awkward literal
      translations; power names match what the tips call them.
- [ ] Tips explain the mechanic correctly (play the sector).
- [ ] Labels are in capitals where the script has them, with the right
      accents or ß/ẞ choice; headings and buttons use sentence case.
- [ ] Punctuation and spacing follow local typography (French spaces before
      `:` and `?`, Spanish `¿`, full-width CJK punctuation).
- [ ] Run `cargo run --release -- --locale <tag>` and look at every screen with
      keyboard and with a gamepad: nothing truncated, wrapped badly, or
      falling back to a short form where the full one would read better.
- [ ] Then change the status in the table above, with the reviewer's name.

## Adding a language

1. Add a variant to `Locale` (`crates/ark-text/src/locale.rs`) with its tag,
   native name, script, Steam API name, and digit grouping from CLDR.
2. Copy `tables/en.rs` to `tables/<lang>.rs`, translate it, and route it in
   `tables/mod.rs`.
3. If it needs a script the atlases lack (Arabic, Thai, Devanagari …), add
   a font and a group in `tools/fontbake`; right-to-left text and complex
   shaping also need layout work in `ark-glyphs` first.
4. Rebake the atlases (below) and run `scripts/task check`. The layout test
   fails, naming the string, if anything is wider than its place; shorten it
   or add a `short` form.
5. Add the language to the table above as a draft.

## Type

The UI is set in **Noto Sans** (Regular and Medium) and **Noto Sans CJK**
(SC, JP and KR subsets, Regular and Medium), both SIL Open Font License 1.1.
The 5×7 pixel font draws the ARKONK logo and the app icons, and takes over
for text when a window is too small for Noto to stay legible.

| Role | Size (scene units) | Weight | Use |
| --- | ---: | --- | --- |
| Label | 15 | Regular, tracked +0.06 em | Small capitals: `SCORE`, `LIVES`, medals, chapters |
| Caption | 16 | Regular | Control hints, settings keys, the save warning |
| Body | 20 | Regular | Buttons, tips, sector names, values, gameplay prompts |
| Display | 32 | Medium | Headings, the score, the sector being served |

Hints are captions so the screen's actions and content lead and the
control reminders recede; gameplay prompts ("Click or space to serve")
stay body text because they are the one thing to do next.

Figures are tabular (`tnum`) everywhere, so scores and timers never shift as
they change. Kerning is the fonts' GPOS pair kerning, extracted with a real
shaper for every pair the tables use; layout and the fit test apply the same
values. The pen moves in fractional pixels and each glyph is rounded once.

### Pixel sizes

The scene is 960 × 900 units scaled to the window. A style's physical size is
snapped to a **ladder** of pixel sizes (10–64 px) and glyphs are drawn 1:1,
never scaled. Only the sizes the checked displays need are baked:

| Display | Density | Label | Caption | Body | Display |
| --- | ---: | ---: | ---: | ---: | ---: |
| 720p (1280 × 720) | 0.80 | 12 | 13 | 16 | 26 |
| Steam Deck 1280 × 800 | 0.89 | 13 | 14 | 18 | 28 |
| 960 × 900 window at 100 % | 1.00 | 15 | 16 | 20 | 32 |
| 1080p | 1.20 | 18 | 20 | 24 | 40 |
| 1440p | 1.60 | 24 | 26 | 32 | 48 |
| Retina window at 200 % | 2.00 | 30 | 32 | 40 | 64 |
| 4K | 2.40 | 36 | 40 | 48 | 64 |

Between those densities a style takes the nearest ladder size, or the largest
baked size below it. Above 2.4 text stays at the 4K sizes. Below about 0.76,
where even the smallest strike would be far larger than the layout planned,
Latin text switches to the pixel font; scripts it cannot spell keep the
smallest strike. The Small and Compact layouts will handle tiny displays.

**Steam Deck legibility.** Valve asks for text at least 9 px tall at 1280 × 800.
The smallest text is a label at 13 px, whose capitals are 9.3 px tall;
captions are 14 px and body text 18 px (lowercase just over 9 px). A test
pins the cap-height check and the Deck strike for each role.

### Layout check

`every_string_fits_its_place_in_every_locale` (`src/render/tests.rs`) draws
every screen in every locale, including pseudo, with keyboard and gamepad
prompts, at each density above and at the density just past every size
switch (where text is widest for its layout). It uses the renderer's own
measuring code, so it fails on any string wider than its slot, any wrapped
text past its lines, and any glyph missing from the atlas.

## Baking the atlases

The atlases in `crates/ark-glyphs/data/` are committed: normal builds need no
font files and no network.

```sh
cargo run --release -p fontbake            # rebake after changing strings
cargo run --release -p fontbake -- --check # fail if a fresh bake differs
```

The baker downloads the pinned font files with `curl` into
`target/fontbake/`, verifies each SHA-256 (pinned in
`tools/fontbake/src/sources.rs`), shapes every character with tabular
figures, extracts kerning, renders hinted glyphs with `swash`, quantizes
coverage to 16 levels and deflates each strike. Only characters the tables
use are baked, and each size gets only the characters of the roles drawn at
it. It bakes twice in one run and fails if the two differ; CI runs `--check`
on macOS and Linux, so the committed bytes must match a fresh bake.

To update a font, change its URL and hash in `sources.rs`, rebake, and
update `packaging/licenses/fonts/`. Those license texts are appended to
`THIRD_PARTY_LICENSES.txt` by `scripts/licenses.sh`, so every package ships them.

### Size

macOS arm64 release binary, symbols kept:

| Build | Bytes | Change |
| --- | ---: | ---: |
| Before (`main`, 5×7 font only) | 2,013,088 | |
| Without CJK (`--no-default-features`) | 2,335,216 | +322,128 |
| With CJK (default) | 3,144,672 | +1,131,584 |

| Atlas file | Holds | Bytes |
| --- | --- | ---: |
| `latin.bin` | Latin, Cyrillic, figures; 203 Regular + 126 Medium glyphs | 175,509 |
| `zh.bin` | 219 + 36 Simplified Chinese glyphs | 324,282 |
| `ja.bin` | 205 + 52 Japanese glyphs | 286,940 |
| `ko.bin` | 204 + 45 Korean glyphs | 192,553 |

The rest of the change, about 134 KB, is code and string tables: the new
renderer text path, the twelve tables (with their CJK and Cyrillic UTF-8),
and the inflate routine (already linked through Macroquad's PNG decoder
today, so it costs nothing until that goes).

Deflated bytes per strike (each size carries only the characters of the
roles drawn at it; Label and Display sizes hold far fewer glyphs than Body):

| Size | Weight | Roles | Latin | zh | ja | ko |
| ---: | --- | --- | ---: | ---: | ---: | ---: |
| 26 | Medium | Display | 6,026 | 5,333 | 6,303 | 4,286 |
| 28 | Medium | Display | 6,908 | 5,776 | 6,856 | 4,619 |
| 32 | Medium | Display | 7,848 | 6,463 | 7,657 | 5,111 |
| 40 | Medium | Display | 10,032 | 8,004 | 9,574 | 6,224 |
| 48 | Medium | Display | 12,425 | 9,568 | 11,442 | 7,369 |
| 64 | Medium | Display | 18,955 | 12,519 | 15,327 | 9,669 |
| 12 | Regular | Label | 1,852 | 1,861 | 1,598 | 1,361 |
| 13 | Regular | Label+Caption | 3,421 | 5,030 | 4,229 | 3,715 |
| 14 | Regular | Caption | 3,356 | 3,433 | 3,347 | 2,973 |
| 15 | Regular | Label | 2,300 | 2,454 | 2,103 | 1,720 |
| 16 | Regular | Body+Caption | 6,041 | 18,638 | 15,167 | 10,254 |
| 18 | Regular | Label+Body | 6,840 | 21,666 | 17,602 | 12,003 |
| 20 | Regular | Body+Caption | 7,646 | 24,511 | 19,828 | 13,321 |
| 24 | Regular | Label+Body | 9,064 | 29,677 | 24,156 | 15,871 |
| 26 | Regular | Caption | 6,228 | 7,258 | 6,811 | 5,867 |
| 30 | Regular | Label | 5,200 | 5,169 | 4,569 | 3,467 |
| 32 | Regular | Body+Caption | 13,211 | 39,373 | 32,276 | 20,999 |
| 36 | Regular | Label | 6,260 | 6,281 | 5,525 | 4,156 |
| 40 | Regular | Body+Caption | 16,717 | 49,655 | 40,914 | 26,659 |
| 48 | Regular | Body | 21,931 | 59,865 | 49,710 | 31,197 |

The 4K body size (48 px) is the largest single cost in each CJK file.

CJK is a cargo feature, `cjk`, on by default, so every package ships all
twelve languages; building without it saves 803,775 bytes of atlas and the
CJK locales fall back to the next preference. The desktop size budget is
measured without `cjk` (as Steam is measured separately): Chinese, Japanese
and Korean are part of the Steam and desktop packages, while the portable
and minimal builds hold the size line.

## Known gaps

- Steam rich presence (`docs/steam/rich_presence.vdf`) is English only, and
  sends English sector names.
- The F3 performance overlay is a developer tool and stays in English.
- The pixel-font fallback below density 0.76 spells only ASCII.
