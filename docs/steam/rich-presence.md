# Rich presence

Friends see one of:

| State | Shown as |
| --- | --- |
| Title screen, sector select | In menus |
| Journey run | Sector 03 · Slipstream |
| Sector select practice | Practicing Sector 03 · Slipstream |

The game sets three keys, and only when the state changes:

| Key | Value |
| --- | --- |
| `steam_display` | `#Menus`, `#Journey` or `#Practice` |
| `sector` | Two-digit sector number, e.g. `03` |
| `name` | The sector's slug, e.g. `slipstream` |

`steam_display` must name a token that Steam knows, or Steam shows nothing.
Each friend sees the text in their own Steam language: the `#Journey` and
`#Practice` strings show the sector name through `{#Sector_%name%}`, a token
picked by the `name` key, so the game never sends display text.

Upload every file in [`rich_presence/`](rich_presence/), one per Steam
language, on the partner site under **App Admin → Community → Rich
Presence**, then publish. The files are written from the game's string
tables: after changing a translation, run
`ARKONK_WRITE_PRESENCE=1 cargo test presence`. A unit test in
`src/presence.rs` fails when a file is stale or misses a token.

To check it, run a Steam build with the real app id while signed in, then open
<https://steamcommunity.com/dev/testrichpresence> in a browser signed in to the
same account. It lists the keys the client is currently publishing.
