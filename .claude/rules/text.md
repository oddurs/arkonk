---
paths:
  - "crates/ark-text/**"
  - "crates/ark-glyphs/**"
  - "tools/fontbake/**"
  - "docs/localization.md"
---

# Strings and glyphs

- A new `TextId` needs an entry in every locale table. The `match` is exhaustive, so a
  missing one won't compile. English is authoritative; the rest are drafts, and
  `docs/localization.md` says so.
- Give every action and band string a short form. The layout's fit test fails when a
  string has to be ellipsized.
- Numbers go through the locale's formatter (grouping, tabular figures), never `format!`.
- Text is NFC. Line-break opportunities in scripts without spaces are authored in the
  table.
- Any new character, size or role changes the atlases. Rebake, commit the `.bin`
  files, and pass `fontbake --check` (skill: `rebake-glyphs`).
- Fonts are pinned by version and SHA-256 in `tools/fontbake/src/sources.rs`. A new font
  adds its OFL text under `packaging/licenses/fonts/`.
