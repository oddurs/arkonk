#!/bin/sh
# Stages the three Steam zips in dist/ as SteamPipe depot content and uploads
# them as one build with steamcmd.
#   scripts/steam-upload.sh [--branch NAME] [--preview] [--stage-only]
#     --branch NAME   set the build live on this branch. Steam refuses
#                     "default" for released apps; promote from Steamworks.
#     --preview       SteamPipe preview: scan and log, upload nothing
#     --stage-only    stage content and write build scripts; skip steamcmd
#
# Environment:
#   STEAM_APP_ID, STEAM_DEPOT_WINDOWS, STEAM_DEPOT_MACOS, STEAM_DEPOT_LINUX
#   STEAM_USERNAME     the build account
#   STEAM_PASSWORD     omit when steamcmd holds a saved login (config.vdf),
#                      or to have steamcmd prompt for it
#   STEAM_GUARD_CODE   a one-time Steam Guard code, or
#   STEAM_TOTP_SECRET  the base64 shared_secret of the account's mobile
#                      authenticator, to generate that code
#   STEAMCMD           steamcmd executable (default: steamcmd)
# docs/RELEASING.md explains the login options.
set -eu
cd "$(dirname "$0")/.."

usage() {
    echo "usage: scripts/steam-upload.sh [--branch NAME] [--preview] [--stage-only]" >&2
    exit 2
}
branch=
preview=0
stage_only=false
while [ "$#" -gt 0 ]; do
    case $1 in
    --branch)
        [ "$#" -ge 2 ] || usage
        branch=$2
        shift 2
        ;;
    --preview)
        preview=1
        shift
        ;;
    --stage-only)
        stage_only=true
        shift
        ;;
    *) usage ;;
    esac
done
: "${STEAM_APP_ID:?set STEAM_APP_ID}"
: "${STEAM_DEPOT_WINDOWS:?set STEAM_DEPOT_WINDOWS}"
: "${STEAM_DEPOT_MACOS:?set STEAM_DEPOT_MACOS}"
: "${STEAM_DEPOT_LINUX:?set STEAM_DEPOT_LINUX}"

pkgid=$(cargo pkgid)
version=${pkgid##*[#@]}
commit=$(git rev-parse --short HEAD)
root=$PWD/target/steam
content=$root/content
rm -rf "$root"
mkdir -p "$content" "$root/output" "$root/scripts" "$root/unpack"

# All three platforms go up together, so a branch never mixes versions.
for os in windows macos linux; do
    zip=dist/arkonk-$version-$os-steam.zip
    if [ ! -f "$zip" ]; then
        echo "Missing $zip. Build it with 'scripts/package.sh --steam' on $os," >&2
        echo "or download the steam artifacts of a release workflow run into dist/." >&2
        exit 1
    fi
    unzip -q "$zip" -d "$root/unpack"
    mv "$root/unpack/arkonk-$version-$os-steam" "$content/$os"
done
rmdir "$root/unpack"

for template in packaging/steam/*.vdf; do
    sed -e "s|@APP_ID@|$STEAM_APP_ID|g" \
        -e "s|@DEPOT_WINDOWS@|$STEAM_DEPOT_WINDOWS|g" \
        -e "s|@DEPOT_MACOS@|$STEAM_DEPOT_MACOS|g" \
        -e "s|@DEPOT_LINUX@|$STEAM_DEPOT_LINUX|g" \
        -e "s|@DESC@|arkonk $version ($commit)|g" \
        -e "s|@PREVIEW@|$preview|g" \
        -e "s|@BRANCH@|$branch|g" \
        -e "s|@CONTENT@|$content|g" \
        -e "s|@OUTPUT@|$root/output|g" \
        "$template" >"$root/scripts/$(basename "$template")"
done
script=$root/scripts/app_build.vdf
echo "Staged arkonk $version ($commit) for app $STEAM_APP_ID in $content" >&2

if $stage_only; then
    echo "Stage only: steamcmd not run. Build script: $script" >&2
    exit 0
fi
: "${STEAM_USERNAME:?set STEAM_USERNAME}"

code=${STEAM_GUARD_CODE-}
if [ -z "$code" ] && [ -n "${STEAM_TOTP_SECRET-}" ]; then
    # Steam Guard's TOTP variant: RFC 6238 over SHA-1 with 30 s steps,
    # rendered as five characters from Steam's own alphabet.
    code=$(python3 -c '
import base64, hashlib, hmac, os, struct, time
key = base64.b64decode(os.environ["STEAM_TOTP_SECRET"])
mac = hmac.new(key, struct.pack(">Q", int(time.time()) // 30), hashlib.sha1).digest()
start = mac[19] & 15
value = struct.unpack(">I", mac[start:start + 4])[0] & 0x7FFFFFFF
alphabet = "23456789BCDFGHJKMNPQRTVWXY"
code = ""
for _ in range(5):
    code += alphabet[value % 26]
    value //= 26
print(code)
')
fi

set -- +login "$STEAM_USERNAME"
if [ -n "${STEAM_PASSWORD-}" ]; then
    set -- "$@" "$STEAM_PASSWORD"
    if [ -n "$code" ]; then
        set -- "$@" "$code"
    fi
fi

# steamcmd's exit status alone is not trusted: success also needs its
# explicit confirmation line.
log=$root/steamcmd.log
{
    if "${STEAMCMD:-steamcmd}" "$@" +run_app_build "$script" +quit; then
        echo "steamcmd exited with 0"
    else
        echo "steamcmd exited with $?"
    fi
} 2>&1 | tee "$log"
if ! grep -q '^steamcmd exited with 0$' "$log" || ! grep -q 'Successfully finished AppID' "$log"; then
    echo "SteamPipe upload failed; see $log and $root/output." >&2
    exit 1
fi
if [ "$preview" = 1 ]; then
    echo "Preview of arkonk $version finished; nothing was uploaded." >&2
elif [ -n "$branch" ]; then
    echo "Uploaded arkonk $version and set it live on '$branch'." >&2
else
    echo "Uploaded arkonk $version. Set it live from Steamworks > SteamPipe > Builds." >&2
fi
