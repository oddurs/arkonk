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
| Min / Max / Default | 0 / 192 / 0 |
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
| `CHAPTER_DAYBREAK` | Daybreak | Clear all eight sectors of Daybreak. | No |
| `CHAPTER_MORNING` | Morning | Clear all eight sectors of Morning. | No |
| `CHAPTER_ZENITH` | Zenith | Clear all eight sectors of Zenith. | No |
| `CHAPTER_GOLDEN_HOUR` | Golden Hour | Clear all eight sectors of Golden Hour. | No |
| `CHAPTER_AFTERLIGHT` | Afterlight | Clear all eight sectors of Afterlight. | No |
| `CHAPTER_BLUE_HOUR` | Blue Hour | Clear all eight sectors of Blue Hour. | No |
| `CHAPTER_ECLIPSE` | Eclipse | Clear all eight sectors of Eclipse. | No |
| `CHAPTER_AURORA` | Aurora | Clear all eight sectors of Aurora. | No |
| `MEDALS_DAYBREAK` | Daybreak Perfected | Earn all 24 medals in Daybreak. | No |
| `MEDALS_MORNING` | Morning Perfected | Earn all 24 medals in Morning. | No |
| `MEDALS_ZENITH` | Zenith Perfected | Earn all 24 medals in Zenith. | No |
| `MEDALS_GOLDEN_HOUR` | Golden Hour Perfected | Earn all 24 medals in Golden Hour. | No |
| `MEDALS_AFTERLIGHT` | Afterlight Perfected | Earn all 24 medals in Afterlight. | No |
| `MEDALS_BLUE_HOUR` | Blue Hour Perfected | Earn all 24 medals in Blue Hour. | No |
| `MEDALS_ECLIPSE` | Eclipse Perfected | Earn all 24 medals in Eclipse. | No |
| `MEDALS_AURORA` | Aurora Perfected | Earn all 24 medals in Aurora. | No |
| `ALL_MEDALS` | Full Spectrum | Earn all 192 medals. | No |
| `JOURNEY_COMPLETE` | Homecoming | Finish the journey by clearing Homecoming. | No |
| `CHAIN_REACTION` | Chain Reaction | Clear a sector with a best chain of 20 or more. | No |

On `ALL_MEDALS`, set **Progress Stat** to `MEDALS` with min 0 and max 192 so
Steam shows a progress bar.

## Changes for the 64-sector journey

The journey grew from 12 sectors in three chapters to 64 in eight
(`docs/journey.md`). On the partner site:

- **Add** `CHAPTER_MORNING`, `CHAPTER_ZENITH`, `CHAPTER_GOLDEN_HOUR`,
  `CHAPTER_ECLIPSE`, `CHAPTER_AURORA`, `MEDALS_MORNING`, `MEDALS_ZENITH`,
  `MEDALS_GOLDEN_HOUR`, `MEDALS_ECLIPSE` and `MEDALS_AURORA`, as in the table.
- **Keep the API names** of the other twelve. Edit the descriptions of
  `CHAPTER_DAYBREAK`, `CHAPTER_BLUE_HOUR` and `CHAPTER_AFTERLIGHT` (four sectors
  become eight), `MEDALS_DAYBREAK`, `MEDALS_BLUE_HOUR` and `MEDALS_AFTERLIGHT`
  (twelve medals become 24) and `ALL_MEDALS` (36 become 192).
- **Raise** the `MEDALS` stat's max, and `ALL_MEDALS`'s progress max, from 36
  to 192.
- `JOURNEY_COMPLETE` still means clearing Homecoming, which is still the last
  sector.

No build with these names has shipped, so there are no unlocks to migrate. Had
there been, a player's Blue Hour and Afterlight unlocks would stand: both
chapters still exist, with more sectors.

## When each one unlocks

The first twenty follow from the saved medals, which are earned in journey or sector
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
