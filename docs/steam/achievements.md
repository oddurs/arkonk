# Achievements and stats

Enter these on the Steamworks partner site under **App Admin → Stats &
Achievements**, then publish the change from the **Publish** tab. The API names
must match exactly; `src/achievements.rs` uses them and a unit test checks that
this table lists every one.

Icons are the owner's job: each achievement needs a 256 × 256 achieved icon and a
greyed-out unachieved icon. The code does not reference icons.

## Stat

Create the stat before the achievements so `ALL_MEDALS` can link to it.

| Field | Value |
| --- | --- |
| API Name | `MEDALS` |
| Type | INT |
| Set By | Client |
| Increment Only | Yes |
| Min / Max / Default | 0 / 36 / 0 |
| Display Name | Medals |

The game sets it to the number of medals in the saved profile and never lowers
it, so a fresh install cannot reduce progress recorded on another machine.

## Achievements

All are **Set By: Client**. None are hidden: the chapters, medals and journey are
visible from the first screen, so there is nothing to spoil.

| API Name | Display Name | Description | Hidden |
| --- | --- | --- | --- |
| `FIRST_LIGHT` | First Light | Clear your first sector. | No |
| `CLEAN` | Spotless | Earn a Clean medal: clear a sector without losing a life. | No |
| `SWIFT` | Ahead of Time | Earn a Swift medal: clear a sector within its target time. | No |
| `CHAPTER_DAYBREAK` | Daybreak | Clear all four sectors of Daybreak. | No |
| `CHAPTER_BLUE_HOUR` | Blue Hour | Clear all four sectors of Blue Hour. | No |
| `CHAPTER_AFTERLIGHT` | Afterlight | Clear all four sectors of Afterlight. | No |
| `MEDALS_DAYBREAK` | Daybreak Perfected | Earn all twelve medals in Daybreak. | No |
| `MEDALS_BLUE_HOUR` | Blue Hour Perfected | Earn all twelve medals in Blue Hour. | No |
| `MEDALS_AFTERLIGHT` | Afterlight Perfected | Earn all twelve medals in Afterlight. | No |
| `ALL_MEDALS` | Full Spectrum | Earn all 36 medals. | No |
| `JOURNEY_COMPLETE` | Homecoming | Finish the journey by clearing Homecoming. | No |
| `CHAIN_REACTION` | Chain Reaction | Clear a sector with a best chain of 20 or more. | No |

On `ALL_MEDALS`, set **Progress Stat** to `MEDALS` with min 0 and max 36 so
Steam shows a progress bar.

## When each one unlocks

The first ten follow from the saved medals, which are earned in journey or sector
select alike. They are checked at every sector clear and again at startup, so
medals earned before Steam was present (or while it was offline) unlock on the
next launch with Steam running.

`JOURNEY_COMPLETE` and `CHAIN_REACTION` are moments rather than saved state: they
unlock at the clear that earns them (a journey run reaching the end of Homecoming;
a results card showing a best chain of at least 20). If Steam is not running at
that moment they are not recorded, and the profile cannot recover them later.
The chain counts consecutive breaks before a paddle bounce, relay damage
included, so connected cores in later sectors are the natural route to 20.

## Checking them

Valve's test app 480 does not define these names. A Steam build running as 480
logs one line listing the missing names and carries on. Test against the real app
id with a Steam account that owns it; reset a test account's progress from the
partner site's **Stats & Achievements** page, or with
`steam://open/console` then `reset_all_stats <appid>` in the Steam console.
