# Steam

Steamworks is compiled in only with the `steam` cargo feature. Default builds, the
tests and the benchmark never link the Steam API. With the feature on, the game
still runs without Steam: if the client is not running, it logs one line and plays
on. All Steam calls go through `src/steam.rs`; the achievement and presence rules
are pure functions in `src/achievements.rs` and `src/presence.rs`.

```sh
cargo run --locked --release --features steam
```

## What is integrated

- **Achievements** and a medal stat: [achievements.md](achievements.md).
- **Rich presence**: [rich-presence.md](rich-presence.md).
- **Overlay**: opening the Steam overlay pauses play, the same as leaving the window.
- **Steam Cloud**: Auto-Cloud, no code: [cloud.md](cloud.md).

Graphical test runs (`--smoke-test`, `--flow-test`, `--perf-test`,
`--effects-test`) never initialise Steam, so they cannot unlock achievements.

## App id

The id is `APP_ID` in `src/steam.rs`, currently Valve's test app **480**
(Spacewar). To ship, replace it with ARKONK's id from the partner site; nothing
else in the code changes.

While the id is 480 the game never relaunches itself through Steam: that would
start Spacewar rather than this build. With a real id, a release build started
outside Steam asks Steam to relaunch it and exits at once
(`SteamAPI_RestartAppIfNecessary`), which is what Valve requires of a shipped
game. Debug builds skip that so `cargo run --features steam` keeps working.

### `steam_appid.txt`

The game passes its id to Steam itself, so `steam_appid.txt` is not needed to
initialise. It matters only for running a **release** build with a **real** id
outside Steam: Steam does not relaunch a game whose working directory contains
that file. For such a run, create it in the directory you run from:

```sh
echo <appid> > steam_appid.txt
```

It is git-ignored. It must never be uploaded in a depot: in a shipped build it
would disable the relaunch check.

## Steam API library

A `steam` build loads the Steam API dynamically. The library ships with the
`steamworks-sys` crate (`lib/steam/redistributable_bin/` in its source) and the
build copies it into `target/<profile>/build/steamworks-sys-*/out/`. `cargo run`
finds it there; anything else needs it beside the executable, or the process fails
to start before the game's code runs.

| Platform | File | Where it goes |
| --- | --- | --- |
| Windows x64 | `steam_api64.dll` | Next to `arkonk.exe` |
| Linux x64, Steam Deck | `libsteam_api.so` (from `linux64/`) | Next to `arkonk` |
| macOS | `libsteam_api.dylib` (universal) | `ARKONK.app/Contents/MacOS/`, next to the executable |

The macOS library's install name is `@loader_path/libsteam_api.dylib`, so it
resolves from the executable's own directory. Sign it before signing the bundle
(`codesign --sign <identity> --options runtime` on the dylib, then the app).

Windows searches the executable's directory first. On Linux, `build.rs` gives
`steam` builds an `$ORIGIN` run path, so the library beside the executable is
found without `LD_LIBRARY_PATH`.

Default-feature builds must not ship the library, and no build should ship
`steam_appid.txt`.

`scripts/package.sh --steam` builds and stages all of this, and
`scripts/steam-upload.sh` uploads it; see [RELEASING.md](../RELEASING.md).

## Partner site checklist

1. Put the real app id in `src/steam.rs` (`APP_ID`).
2. Create the stat and achievements, and upload icons: [achievements.md](achievements.md).
3. Upload the rich presence tokens: [rich-presence.md](rich-presence.md).
4. Configure Auto-Cloud: [cloud.md](cloud.md).
5. Publish the changes from the **Publish** tab.
