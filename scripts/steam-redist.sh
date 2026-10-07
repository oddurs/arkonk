#!/bin/sh
# Prints the path of the Steam API redistributable for a platform:
#   scripts/steam-redist.sh windows|macos|linux
# It is the copy the steamworks-sys crate links against, so the shipped
# library always matches the bindings. STEAM_SDK_LOCATION overrides it,
# exactly as it does for steamworks-sys.
set -eu
cd "$(dirname "$0")/.."

case "${1-}" in
windows) file=win64/steam_api64.dll ;;
macos) file=osx/libsteam_api.dylib ;;
linux) file=linux64/libsteam_api.so ;;
*)
    echo "usage: scripts/steam-redist.sh windows|macos|linux" >&2
    exit 2
    ;;
esac

if [ -n "${STEAM_SDK_LOCATION-}" ]; then
    sdk=$STEAM_SDK_LOCATION
else
    # cargo metadata is one JSON line; Windows paths arrive with escaped
    # backslashes, which forward slashes replace for the shell.
    metadata=$(cargo metadata --locked --format-version 1 --all-features)
    manifest=$(printf '%s' "$metadata" |
        grep -o '"manifest_path":"[^"]*steamworks-sys-[^"]*"' |
        head -n 1 |
        sed 's/^"manifest_path":"//; s/"$//; s/\\\\/\//g')
    if [ -z "$manifest" ]; then
        echo "steamworks-sys is not in the dependency tree; is the steam feature defined?" >&2
        exit 1
    fi
    sdk=$(dirname "$manifest")/lib/steam
fi

path=$sdk/redistributable_bin/$file
if [ ! -f "$path" ]; then
    echo "Steam API redistributable not found at $path" >&2
    exit 1
fi
echo "$path"
