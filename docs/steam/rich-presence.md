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
| `name` | The sector's title-cased name, e.g. `Slipstream` |

`steam_display` must name a token that Steam knows, or Steam shows nothing.
Upload [`rich_presence.vdf`](rich_presence.vdf) on the partner site under
**App Admin → Community → Rich Presence**, then publish. A unit test in
`src/presence.rs` checks that the file defines every token the game sends.

The sector names are passed as plain values rather than tokens; the game itself
is English only. To localize later, add a language block per file with the same
token names.

To check it, run a Steam build with the real app id while signed in, then open
<https://steamcommunity.com/dev/testrichpresence> in a browser signed in to the
same account. It lists the keys the client is currently publishing.
