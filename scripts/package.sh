#!/bin/sh
# Builds this host's release zip into dist/.
#   scripts/package.sh           DRM-free build, attached to GitHub Releases
#   scripts/package.sh --steam   `steam` feature plus the Steam API
#                                redistributable, for a SteamPipe depot
# Every zip holds one top-level folder with the game, LICENSE.txt and
# THIRD_PARTY_LICENSES.txt. macOS signing and notarization: see build-macos.sh.
set -eu
cd "$(dirname "$0")/.."

steam=false
suffix=
case "${1-}" in
"") ;;
--steam)
    steam=true
    suffix=-steam
    ;;
*)
    echo "usage: scripts/package.sh [--steam]" >&2
    exit 2
    ;;
esac

case "$(uname -s)" in
Darwin) os=macos ;;
Linux) os=linux ;;
MINGW* | MSYS* | CYGWIN*) os=windows ;;
*)
    echo "Unsupported host: $(uname -s)" >&2
    exit 1
    ;;
esac

if $steam && ! grep -q '^steam = \[' Cargo.toml; then
    echo "Cargo.toml defines no 'steam' feature yet, so there is no Steam build to package." >&2
    exit 1
fi

pkgid=$(cargo pkgid)
version=${pkgid##*[#@]}
name=arkonk-$version-$os$suffix
stage=target/package/$name
rm -rf "$stage"
mkdir -p "$stage" dist

scripts/licenses.sh "$stage/THIRD_PARTY_LICENSES.txt"
cp LICENSE "$stage/LICENSE.txt"

# The release binary must report the version the package is named after.
check_version() {
    actual=$("$1" --version)
    if [ "$actual" != "arkonk $version" ]; then
        echo "$1 reports '$actual', expected 'arkonk $version'." >&2
        exit 1
    fi
}

build() {
    if $steam; then
        cargo build --locked --release --bin arkonk --features steam
    else
        cargo build --locked --release --bin arkonk
    fi
}

case $os in
macos)
    if $steam; then
        scripts/build-macos.sh --steam
    else
        scripts/build-macos.sh
    fi
    cp -R target/ARKONK.app "$stage/"
    ;;
windows)
    build
    exe=target/release/arkonk.exe
    check_version "$exe"
    cp "$exe" "$stage/"
    if $steam; then
        redist=$(scripts/steam-redist.sh windows)
        cp "$redist" "$stage/"
    fi
    ;;
linux)
    build
    exe=target/release/arkonk
    check_version "$exe"
    # Steam runs Linux games in the Steam Runtime 3 "sniper" container, whose
    # glibc is 2.31. A newer symbol version would fail to load there.
    glibc=$(objdump -T "$exe" | grep -o 'GLIBC_[0-9.]*' | sed 's/^GLIBC_//' | sort -uV | tail -n 1)
    if [ "$(printf '%s\n2.31\n' "$glibc" | sort -V | tail -n 1)" != 2.31 ]; then
        echo "$exe needs glibc $glibc; the Steam Runtime provides 2.31. Build inside the sniper SDK." >&2
        exit 1
    fi
    echo "$exe needs glibc $glibc (Steam Runtime sniper provides 2.31)." >&2
    cp "$exe" "$stage/"
    if $steam; then
        redist=$(scripts/steam-redist.sh linux)
        cp "$redist" "$stage/"
    fi
    ;;
esac

zip=$PWD/dist/$name.zip
rm -f "$zip"
case $os in
# ditto keeps the bundle's signature intact; --norsrc keeps AppleDouble files
# out of an archive that Linux will unpack for the Steam upload.
macos) ditto -c -k --norsrc --keepParent "$stage" "$zip" ;;
# Windows' bundled bsdtar writes zips; Git Bash's own tar cannot.
windows) (cd target/package && "$(cygpath -u "$SYSTEMROOT")/System32/tar.exe" -a -c -f "$(cygpath -w "$zip")" "$name") ;;
linux) (cd target/package && zip -qr "$zip" "$name") ;;
esac
echo "Packaged dist/$name.zip"
