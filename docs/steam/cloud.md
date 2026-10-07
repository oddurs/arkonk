# Steam Cloud

All progress and settings live in one small file, `progress.txt`, written through
a temporary file and renamed at menu and sector boundaries. Steam Auto-Cloud
syncs it before launch and after exit, so the game has no cloud code.

| Platform | Path the game uses |
| --- | --- |
| Windows | `%LOCALAPPDATA%\arkonk\progress.txt` |
| macOS | `~/Library/Application Support/arkonk/progress.txt` |
| Linux, Steam Deck | `$XDG_DATA_HOME/arkonk/progress.txt`, else `~/.local/share/arkonk/progress.txt` |

## Partner site configuration

**App Admin → Application → Steam Cloud:**

1. Set **Byte quota per user** to `1048576` (1 MB) and **Number of files allowed
   per user** to `4`. The file is under 1 KB; the headroom is for future files.
2. Under **Auto-Cloud Configuration**, add one root path:

   | Root | Subdirectory | Pattern | OS | Recursive |
   | --- | --- | --- | --- | --- |
   | `WinAppDataLocal` | `arkonk` | `progress.txt` | All OSes | No |

3. Add two **Root Overrides** so the same cloud file maps to each platform's
   location. Leave **Add/Replace Path** empty; the subdirectory stays `arkonk`.

   | Original Root | OS | New Root |
   | --- | --- | --- |
   | `WinAppDataLocal` | MacOS | `MacAppSupport` |
   | `WinAppDataLocal` | Linux + SteamOS | `LinuxXdgDataHome` |

4. Save, then publish from the **Publish** tab.

One root with overrides, rather than a root per OS, keeps a single cloud copy, so
progress follows a player between Windows, macOS and a Steam Deck. Steam resolves
`LinuxXdgDataHome` to `$XDG_DATA_HOME`, falling back to `~/.local/share`, the same
rule the game uses.

The pattern is the exact file name on purpose: `progress.tmp` exists only during a
save and must never sync, and the legacy `best.txt` is only read once to import a
score. The last-known-good `progress.bak`, any `unreadable-*` file set aside after
a failed read, and `logs/` are local recovery and diagnostics, not progress, and
stay out of the cloud. If a later change adds more save files, add them as further patterns rather
than widening this one to `*`.

## Checking it

With the real app id, play a sector, quit, and check **Steam → Library → ARKONK →
Properties → General → Keep game saves in the Steam Cloud**. The file appears at
`https://store.steampowered.com/account/remotestorage` under the app. Delete the
local file, launch again from Steam, and the sector should still be unlocked.
