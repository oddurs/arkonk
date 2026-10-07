# Releasing ARKONK

The version lives in one place: `version` in `Cargo.toml`. The binary reports it
(`arkonk --version`), the macOS bundle copies it into `Info.plist`, the Windows
executable embeds it, and every package is named after it.

## Cut a release

1. On a branch, set the new `version` in `Cargo.toml` and run `cargo build` to
   refresh `Cargo.lock`. In `CHANGELOG.md`, rename `## [Unreleased]` to
   `## [X.Y.Z] - YYYY-MM-DD` and open a new, empty `## [Unreleased]` above it.
   Merge the pull request.
2. Tag the merged commit and push the tag:

   ```sh
   git fetch origin
   git tag -a vX.Y.Z -m "ARKONK X.Y.Z" origin/main
   git push origin vX.Y.Z
   ```

3. The **Release** workflow then:
   - builds and packages Windows, macOS, and Linux (`package.yml`);
   - refuses to continue if the tag is not `v` + the `Cargo.toml` version;
   - publishes a GitHub Release with the three zips and `SHA256SUMS`, using the
     `CHANGELOG.md` section as the notes;
   - uploads the Steam builds to Steam when Steam is configured (below).

To rehearse, run the workflow by hand (Actions > Release > Run workflow, or
`gh workflow run release.yml --ref <branch>`). A manual run builds and checks
everything, signs and notarizes when the secrets exist, but publishes no
GitHub Release. It uploads to Steam only when you tick **steam**.

## What each package contains

| Package | Contents |
| --- | --- |
| `arkonk-X.Y.Z-windows.zip` | `arkonk.exe` (icon, version resource, no console window) |
| `arkonk-X.Y.Z-macos.zip` | `ARKONK.app`, universal arm64 + x86_64, macOS 11 or newer |
| `arkonk-X.Y.Z-linux.zip` | `arkonk`, built in the Steam Runtime 3 (sniper) SDK |

Each zip has one top-level folder that also holds `LICENSE.txt` and
`THIRD_PARTY_LICENSES.txt`. The `-steam` variants are built with
`--features steam` and add the Steam API redistributable; they go to Steam
only and are not attached to the GitHub Release.

Linux packaging fails if the binary needs a glibc newer than 2.31, the version
in Steam's runtime. The plain Linux zip runs outside Steam on any x86_64
distribution with glibc 2.31 or newer and ALSA; no launch wrapper is needed.

## Building packages locally

```sh
scripts/package.sh            # this platform's zip in dist/
scripts/package.sh --steam    # the Steam variant
scripts/build-macos.sh        # just target/ARKONK.app
scripts/licenses.sh           # just target/THIRD_PARTY_LICENSES.txt
```

The license notice is generated from `Cargo.lock` by cargo-about 0.9.2, pinned
in `scripts/licenses.sh` and in `package.yml`:
`cargo install cargo-about --locked --features cli --version 0.9.2`.
A dependency under a license that `packaging/licenses/about.toml` does not
accept fails packaging until someone reviews it.

Linux packages must be built in the sniper SDK image
(`registry.gitlab.steamos.cloud/steamrt/sniper/sdk`), with rustup and
cargo-about installed inside it. Use a separate checkout for that: the
container writes Linux binaries into the same `target/` as the host.

Icons in `packaging/icons/` are rendered from the in-game 5×7 glyphs by
`cargo run --example icons`. They are committed; rerun only when the logo changes.

## Secrets and variables to create

Set these under Settings > Secrets and variables > Actions. Every one is
optional: anything missing makes the matching step skip with a notice, and
nothing reports success that did not happen.

### macOS signing and notarization

Requires an Apple Developer Program membership.

| Secret | Value |
| --- | --- |
| `MACOS_CERTIFICATE` | Base64 of a **Developer ID Application** certificate and private key exported as `.p12` (`base64 -i cert.p12 \| pbcopy`) |
| `MACOS_CERTIFICATE_PASSWORD` | The `.p12` export password |
| `MACOS_SIGN_IDENTITY` | The identity name, e.g. `Developer ID Application: Your Name (TEAMID)` |
| `APPLE_NOTARY_KEY` | Contents of an App Store Connect API key `.p8` (Users and Access > Integrations > App Store Connect API; Developer role) |
| `APPLE_NOTARY_KEY_ID` | That key's ID |
| `APPLE_NOTARY_ISSUER` | The issuer ID shown above the key list |

Locally, sign and notarize with a keychain profile instead of an API key:

```sh
xcrun notarytool store-credentials arkonk --apple-id you@example.com --team-id TEAMID
MACOS_SIGN_IDENTITY="Developer ID Application: Your Name (TEAMID)" \
NOTARY_KEYCHAIN_PROFILE=arkonk scripts/package.sh
```

Windows executables are not Authenticode-signed. Steam does not require it;
SmartScreen warns about unsigned GitHub downloads until a certificate is added.

### Steam

Requires a Steamworks partner account and an app with three depots, one per OS,
each with its operating system set under SteamPipe > Depots. Set the launch
options to `arkonk.exe` (Windows), `ARKONK.app` (macOS), and `arkonk` (Linux).

Repository **variables** (not secret):

| Variable | Value |
| --- | --- |
| `STEAM_APP_ID` | The app ID |
| `STEAM_DEPOT_WINDOWS`, `STEAM_DEPOT_MACOS`, `STEAM_DEPOT_LINUX` | The depot IDs |
| `STEAM_BRANCH` | Optional. Branch that tag builds go live on, e.g. `beta`. Create it in Steamworks first. Unset: builds upload without going live |

Repository **secrets**: `STEAM_USERNAME`, a dedicated build account that has
only the Steamworks permissions to edit and publish this app, plus one way for
it to pass Steam Guard unattended:

- **Saved login (`STEAM_CONFIG_VDF`).** On Linux, run
  `steamcmd +login <user> +quit`, enter the password and Steam Guard code once,
  then store `base64 -w0 config/config.vdf` from steamcmd's directory as the
  secret. Simplest to set up; the token expires after a period without use,
  and an expired token shows up as a login failure in the upload job.
- **Authenticator secret (`STEAM_PASSWORD` + `STEAM_TOTP_SECRET`).** The build
  account uses the Steam mobile authenticator, and `STEAM_TOTP_SECRET` is its
  base64 `shared_secret` (for example from a steamguard-cli maFile). The upload
  script derives the current code itself, so it never expires, but anyone with
  the secret can pass Steam Guard for that account.

Email-based Steam Guard alone cannot work unattended.

## Push a build to a Steam branch

From CI: Actions > Release > Run workflow on the commit you want, tick
**steam**, and name the branch (or leave it empty for `STEAM_BRANCH`).

Locally, with steamcmd installed and the three `-steam` zips in `dist/`
(for example `gh run download <run-id> --pattern 'arkonk-*' --dir dist`):

```sh
export STEAM_APP_ID=... STEAM_DEPOT_WINDOWS=... STEAM_DEPOT_MACOS=... STEAM_DEPOT_LINUX=...
STEAM_USERNAME=builder scripts/steam-upload.sh --stage-only   # inspect target/steam
STEAM_USERNAME=builder scripts/steam-upload.sh --preview      # SteamPipe dry run
STEAM_USERNAME=builder scripts/steam-upload.sh --branch beta
```

steamcmd prompts for the password and Steam Guard code when it has no saved
login. `STEAMCMD` points the script at a steamcmd that is not on `PATH`. All
three platforms always upload together, so a branch never mixes versions. Steam
does not let build scripts set `default` live for a released app; promote a
build to the default branch from Steamworks > SteamPipe > Builds.

## The `steam` feature and the Steam API library

Steam builds use `cargo build --features steam`, a feature the Steamworks
integration defines. Until `Cargo.toml` has it, the release workflow skips the
Steam build and upload with a notice, and `scripts/package.sh --steam` exits
with an error.

The redistributable comes from the `steamworks-sys` crate the build links
against, or from `STEAM_SDK_LOCATION` when that is set
(`scripts/steam-redist.sh`). It is placed where the loader looks:

- Windows: `steam_api64.dll` beside `arkonk.exe`.
- Linux: `libsteam_api.so` beside `arkonk`; `build.rs` adds an `$ORIGIN` rpath
  to Steam builds, so no wrapper script sets `LD_LIBRARY_PATH`.
- macOS: `libsteam_api.dylib` in `ARKONK.app/Contents/MacOS` (its install name
  is `@loader_path`), re-signed with the app before notarization.

The Steam API library is covered by the Steamworks SDK agreement, not an
open-source license, so it is not listed in `THIRD_PARTY_LICENSES.txt`.

## Store and library art

These are artwork, not generated. Check sizes in Steamworks under Store Assets
and Library Assets when uploading; Valve revises them.

- Store: header capsule (920×430), small capsule (462×174), main capsule
  (1232×706), vertical capsule (748×896), optional page background (1438×810),
  and at least five 1920×1080 screenshots.
- Library: capsule (600×900), header (920×430), hero (3840×1240), and logo
  (transparent PNG, up to 1280×720).
- Community icon: 184×184 JPG.
- Client icon and Linux client icons: use `packaging/icons/arkonk.ico` and the
  PNGs in `packaging/icons/`.
