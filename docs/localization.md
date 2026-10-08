# Localization

ARKONK's text lives in two `no_std` crates that every port can share:

- **`crates/ark-text`**: the shipped `Locale`s, a typed `TextId` for every visible
  string, one table per language, and formatting (argument order, digit
  grouping, capsule icons, the pseudo locale). It holds no font data.
- **`crates/ark-glyphs`**: Noto Sans, Noto Sans CJK, Noto Sans Thai and Noto
  Sans Arabic UI, pre-rasterized into hinted strikes at fixed pixel sizes,
  with advances and GPOS pair kerning, plus the line layout (measure, place,
  wrap, right-to-left order) that the game draws with.

`ark` itself knows sectors, chapters and powers only by id and slug.

## Languages

ARKONK ships every language Steam offers. English is the source. Every other
table is a **draft that needs native review**; none has been checked by a
native speaker yet.

| Locale | Tag | Steam language | Status |
| --- | --- | --- | --- |
| English | `en` | english | Source |
| French | `fr` | french | Draft, needs native review |
| German | `de` | german | Draft, needs native review |
| Spanish (Spain) | `es-ES` | spanish | Draft, needs native review |
| Spanish (Latin America) | `es-419` | latam | Draft, needs native review. Inherits `es-ES` except a few words (Mouse, Enter, pad direccional). |
| Portuguese (Brazil) | `pt-BR` | brazilian | Draft, needs native review. Chosen for plain `pt`. |
| Portuguese (Portugal) | `pt-PT` | portuguese | Draft, needs native review. Also chosen for Portuguese-speaking Africa, Macau and Timor-Leste. |
| Italian | `it` | italian | Draft, needs native review |
| Polish | `pl` | polish | Draft, needs native review |
| Russian | `ru` | russian | Draft, needs native review |
| Ukrainian | `uk` | ukrainian | Draft, needs native review |
| Bulgarian | `bg` | bulgarian | Draft, needs native review |
| Czech | `cs` | czech | Draft, needs native review |
| Hungarian | `hu` | hungarian | Draft, needs native review |
| Romanian | `ro` | romanian | Draft, needs native review |
| Greek | `el` | greek | Draft, needs native review |
| Turkish | `tr` | turkish | Draft, needs native review |
| Dutch | `nl` | dutch | Draft, needs native review |
| Danish | `da` | danish | Draft, needs native review |
| Swedish | `sv` | swedish | Draft, needs native review |
| Norwegian (Bokmål) | `nb` | norwegian | Draft, needs native review. Also chosen for `no` and `nn`. |
| Finnish | `fi` | finnish | Draft, needs native review |
| Indonesian | `id` | indonesian | Draft, needs native review |
| Vietnamese | `vi` | vietnamese | Draft, needs native review |
| Thai | `th` | thai | Draft, needs native review. Line breaks marked in the table (see below). |
| Arabic | `ar` | arabic | Draft, needs native review. Right to left; see below. |
| Chinese (Simplified) | `zh-Hans` | schinese | Draft, needs native review |
| Chinese (Traditional) | `zh-Hant` | tchinese | Draft, needs native review. Taiwan's vocabulary; chosen for `zh-TW`, `zh-HK`, `zh-MO` and `zh-Hant`. |
| Japanese | `ja` | japanese | Draft, needs native review |
| Korean | `ko` | koreana | Draft, needs native review |
| Pseudo | `en-XA` | – | Test only: accented, bracketed, about 40 % longer |

### Which language plays

1. `--locale <tag>` on the command line (`--locale pseudo` works too). Not saved.
2. The player's choice, saved as a `locale <bcp47>` line in `progress.txt`.
   The line is written only when a choice exists. Older builds skip it: the
   progress codec reads it as a damaged line, because its value is not a number.
3. Steam's game language, in builds with the `steam` feature while Steam runs.
4. The operating system's preferred languages, in order (`sys-locale`).
5. English.

A language this build cannot draw (Chinese, Japanese, Korean, Thai or Arabic
without the `scripts` feature) is skipped.
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
- **Numbers.** `Arg::Count` groups digits the locale's way, from CLDR:
  `12,500`, `12.500`, `12 500` (French narrow no-break space), `12 500`
  (no-break space in the Slavic and Nordic languages, Hungarian and
  Portuguese). Where CLDR's minimum grouping digits is 2 (Spanish, Italian,
  Polish, Bulgarian, Hungarian, European Portuguese) four-digit numbers stay
  ungrouped. Arabic uses Latin digits, CLDR's default for `ar`. Times are
  `mm:ss` everywhere.
- **Short forms.** `short` in a table gives a narrower wording, used only
  when the full one does not fit (for example `Continue journey` → `Continue`).
  Every action, everything the band shows, the help lines, the settings and
  the news must have one in every table (a test checks); where the word is
  already as short as it gets (`Sound`), the short form repeats it. A short
  form is never longer than the full one.
  A short form may leave a slot out (the Continue button's caption drops the
  sector number and keeps the name); it never adds one.
- **Quoted strings.** A slot filled with another string (`Arg::Text`) sets
  that string in the quoting string's role. Sector names are quoted in a
  caption, so `TextId::also` tells the baker to bake them at caption sizes
  too; a new quotation in a new role needs an entry there.
- **Precomposed text only.** Strings must be NFC with no combining marks:
  the atlases hold whole glyphs and do no mark positioning. Vietnamese has a
  precomposed form for every letter it uses. The baker refuses anything else,
  except Thai vowel and tone marks (next).
- **Thai.** A consonant and its vowel and tone marks are baked together as one
  glyph, positioned by the font's own shaping, so marks stack correctly
  (ที่). Thai writes no spaces between words, so each table marks where a
  line may break with U+200B ZERO WIDTH SPACE; it takes no room. A test
  requires one in any Thai string long enough to wrap, and rejects it in
  every other language.
- **Arabic.** Letters join, so each one is baked in the contextual form it
  takes (isolated, initial, medial, final, and the lam-alef ligature),
  shaped by the font in context; `ark_glyphs::script` picks the form at run
  time. Lines run right to left with Latin words and numbers kept left to
  right inside them. No short vowels or shadda: they are combining marks
  the atlas cannot place.
- **Mirroring.** In Arabic, sheets, action rows, sector cards and help
  lines mirror: titles, labels and help end at the right, the confirm
  glyph sits at a button's left, the back glyph at the left of a sheet's
  title, settings values at a row's left (a switch is on with its knob at
  the left, a volume level fills from the right), results columns and
  medals read from the right, and a prompt's glyph leads from the right.
  Two things keep their places, on purpose. The band stays score · sector ·
  lives in every language: it is an instrument, read at a glance during
  play, and one layout means a player switching languages, or watching a
  stream in another one, finds the score where it always is. The sector
  grid keeps its chapters left to right, because the arrows move across it
  in that direction; a bricks miniature is a map of the field and never
  flips.
- **No tracking** for Chinese, Japanese, Thai or Arabic: spacing would break
  the joins, the clusters or the even character grid.

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
3. If it needs a script the atlases lack (Devanagari, Hebrew …), add a font
   and a group in `tools/fontbake`. Scripts whose letters change shape or
   reorder beyond what Thai clusters and Arabic forms cover need layout work
   in `ark-glyphs` first.
4. Rebake the atlases (below) and run `scripts/task check`. The layout test
   fails, naming the string, if anything is wider than its place; shorten it
   or add a `short` form.
5. Add the language to the table above as a draft.

## Type

The UI is set in **Noto Sans** (Latin, Greek and Cyrillic), **Noto Sans
CJK** (SC, TC, JP and KR subsets), **Noto Sans Thai** and **Noto Sans
Arabic UI**, each in Regular and Medium, all SIL Open Font License 1.1. The
Arabic UI cut keeps Arabic within the line height Latin text uses.
Headings are set in **Noto Sans Display** Medium; scripts it lacks take
their own font's Medium cut. The 5×7 pixel font draws the ARKONK logo and
the app icons, and sets the text of the Compact layout, for frames under
400 px wide.

| Role | Size (scene units) | Cut | Use |
| --- | ---: | --- | --- |
| Display | 40 | Display Medium | The one hero line on a screen: the sector being served |
| Title | 26 | Display Medium | Sheet titles and screen headers |
| Figure | 32 | Medium, tabular | Scores and results |
| Body | 20 | Regular; Medium when primary or focused | Actions, names, tips |
| Caption | 16 | Regular | Help lines, notes, settings values, the save warning |
| Label | 15 | Medium, tracked +0.10 em | Small capitals that name a value: `MEDALS`, `BEST`, medals, chapters, eyebrows |

Sizes, cuts and tracking come from `crates/ark-glyphs/src/spec.rs`. Chinese,
Japanese, Thai and Arabic are never tracked.

Figures are tabular (`tnum`) everywhere, so scores and timers never shift as
they change. Kerning is the fonts' GPOS pair kerning, extracted with a real
shaper for every pair the tables use; layout and the fit test apply the same
values. The pen moves in fractional pixels and each glyph is rounded once.

### Pixel sizes

The scene is 960 × 900 units scaled to the window. A style's physical size is
snapped to a **ladder** of pixel sizes (10–96 px) and glyphs are drawn 1:1,
never scaled. Only the sizes the checked displays need are baked:

| Display | Density | Label | Caption | Body | Figure | Title | Display |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Small, at its 400 px floor | 0.42 | 10 | 11 | 12 | 14 | 14 | 17 |
| Smallest desktop window (480 × 450) | 0.50 | 10 | 11 | 12 | 16 | 14 | 20 |
| Small | 0.60 | 10 | 11 | 12 | 20 | 16 | 24 |
| Small | 0.75 | 11 | 12 | 15 | 24 | 20 | 30 |
| 720p (1280 × 720) | 0.80 | 12 | 13 | 16 | 26 | 20 | 32 |
| Steam Deck 1280 × 800 | 0.89 | 13 | 14 | 18 | 28 | 24 | 36 |
| 960 × 900 window at 100 % | 1.00 | 15 | 16 | 20 | 32 | 26 | 40 |
| 1080p | 1.20 | 18 | 20 | 24 | 40 | 32 | 48 |
| 1440p | 1.60 | 24 | 26 | 32 | 48 | 40 | 64 |
| Retina window at 200 % | 2.00 | 30 | 32 | 40 | 64 | 56 | 80 |
| 4K | 2.40 | 36 | 40 | 48 | 80 | 64 | 96 |

Between the checked densities a style takes the nearest ladder size, or the
largest baked size below it. Above 2.4 text stays at the 4K sizes.

No role is ever set under its physical floor: body text 12 px, captions
11 px, labels 10 px (capitals about 7 px), headings and figures 14 px
(`spec::floor`). A floor raises the size and the layout reflows round it.
The fit chain may step a role down one baked size, never below its floor.

On a Compact frame (under 400 px) text the 5×7 font can spell is set in it
at whole pixels, headings doubled; a string with letters it lacks (accents,
Greek, Cyrillic, CJK, Thai, Arabic) is set in Noto at its floor, whole
paragraphs at once, so every script draws in its own font and none falls
back to missing glyphs. Compact pages mirror in Arabic as sheets do.

**Steam Deck legibility.** Valve asks for text at least 9 px tall at 1280 × 800.
The smallest text is a label at 13 px, whose capitals are 9.3 px tall;
captions are 14 px and body text 18 px (lowercase just over 9 px). A test
pins the cap-height check and the Deck strike for each role.

### Layout check

`every_screen_lays_out_in_every_locale_and_class` (`src/render/tests.rs`)
draws every screen and sheet in every locale, including pseudo, with the
keyboard, Xbox and PlayStation glyphs, at the frame widths where the layout
classes meet (399, 400, 719 and 720 px), at 1920×1080, 1280×800, 3440×1440,
3840×2160, 1024×768, 1080×1920, 240×240 and 160×128, and at the density just
past every size switch (where text is widest for its layout). It uses the
renderer's own measuring code and fails on any string wider than its slot,
wrapped past its lines or cut to an ellipsis; any glyph missing from the
atlas; text overlapping other text or leaving its box or the screen; and
text, glyph chips or pointer rows under their physical floors.

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

| Build | Bytes |
| --- | ---: |
| Before localization (`main` at #6, 5×7 font only) | 2,013,088 |
| Twelve languages, the UI hierarchy (#7, #8) | 3,178,832 |
| All thirty languages (default, `scripts` on) | 3,954,528 |
| All thirty languages without `scripts` (Latin, Greek, Cyrillic only) | 2,550,816 |

| Atlas file | Holds | Bytes |
| --- | --- | ---: |
| `latin.bin` | Latin, Cyrillic, figures, in the Regular, Medium and Display cuts | 363,750 |
| `zh.bin` | Simplified Chinese glyphs | 537,712 |
| `ja.bin` | Japanese glyphs | 498,762 |
| `ko.bin` | Korean glyphs | 335,605 |

The Small layout's strikes (densities 0.42 to 0.75) and the short forms
added 218,845 bytes (14 %) across the four files.

Deflated bytes per strike (each size carries only the glyphs of the roles
drawn at it; Label and Display sizes hold far fewer than Body):

| Size | Weight | Roles | Latin | zh | tw | ja | ko | th | ar |
| ---: | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 26 | Medium | Display | 10,406 | 5,333 | 5,702 | 6,303 | 4,286 | 3,058 | 3,722 |
| 28 | Medium | Display | 11,989 | 5,776 | 6,124 | 6,856 | 4,619 | 3,309 | 3,958 |
| 32 | Medium | Display | 13,998 | 6,463 | 6,914 | 7,657 | 5,111 | 3,718 | 4,708 |
| 40 | Medium | Display | 17,782 | 8,004 | 8,496 | 9,574 | 6,224 | 4,794 | 5,921 |
| 48 | Medium | Display | 22,417 | 9,568 | 10,173 | 11,442 | 7,369 | 5,954 | 7,465 |
| 64 | Medium | Display | 33,267 | 12,519 | 13,400 | 15,327 | 9,669 | 8,081 | 10,504 |
| 12 | Regular | Label | 2,786 | 1,857 | 1,951 | 1,598 | 1,362 | 957 | 1,201 |
| 13 | Regular | Label+Caption | 6,365 | 7,318 | 7,653 | 6,253 | 5,043 | 2,216 | 2,234 |
| 14 | Regular | Caption | 6,190 | 6,305 | 6,720 | 5,993 | 4,702 | 2,055 | 2,253 |
| 15 | Regular | Label | 3,446 | 2,448 | 2,594 | 2,103 | 1,722 | 1,283 | 1,681 |
| 16 | Regular | Body+Caption | 10,043 | 18,391 | 19,141 | 15,086 | 10,259 | 3,534 | 3,387 |
| 18 | Regular | Label+Body | 11,265 | 21,414 | 22,280 | 17,498 | 12,012 | 4,128 | 3,973 |
| 20 | Regular | Body+Caption | 12,793 | 24,213 | 25,331 | 19,733 | 13,351 | 4,762 | 4,326 |
| 24 | Regular | Label+Body | 15,916 | 29,282 | 31,177 | 24,059 | 15,884 | 5,583 | 5,157 |
| 26 | Regular | Caption | 11,493 | 13,178 | 14,295 | 12,360 | 9,439 | 4,039 | 4,841 |
| 30 | Regular | Label | 7,733 | 5,166 | 5,612 | 4,569 | 3,470 | 2,753 | 3,735 |
| 32 | Regular | Body+Caption | 23,454 | 38,828 | 41,232 | 32,086 | 20,953 | 7,603 | 7,767 |
| 36 | Regular | Label | 9,937 | 6,275 | 6,647 | 5,525 | 4,157 | 3,323 | 4,512 |
| 40 | Regular | Body+Caption | 30,790 | 49,091 | 51,699 | 40,728 | 26,662 | 9,689 | 10,323 |
| 48 | Regular | Body | 39,500 | 59,113 | 62,287 | 49,509 | 31,142 | 12,260 | 12,501 |

The 4K body size (48 px) is the largest single cost in each CJK file.

Every non-Latin script atlas sits behind the cargo feature `scripts`, on by
default, so every package ships all thirty languages. Building without it
saves 1,379,999 bytes of atlas; those locales then fall back to the next
preference. The desktop size budget is measured without `scripts` (as Steam
is measured separately): the Steam and desktop packages carry every
language, while portable and minimal builds hold the size line.

## Known gaps

- The band and the sector grid keep their left-to-right order in Arabic, by
  design (see Mirroring above); a reader of Arabic should confirm that it
  reads well.
- The bidi pass is simplified: no nested embeddings, and Arabic-Indic
  digits are not offered.
- Thai line breaks are placed by hand in the tables rather than found with a
  dictionary.
- The F3 performance overlay is a developer tool and stays in English.
- The pixel-font fallback below density 0.76 spells only ASCII.
