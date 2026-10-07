#!/bin/sh
# Builds target/ARKONK.app as a universal (Apple silicon + Intel) bundle.
#   scripts/build-macos.sh           DRM-free build
#   scripts/build-macos.sh --steam   `steam` feature, with libsteam_api.dylib bundled
#
# Signing and notarization follow the environment and are skipped, loudly,
# when it does not provide them:
#   MACOS_SIGN_IDENTITY       "Developer ID Application: Name (TEAMID)".
#                             Unset: an ad-hoc signature that runs only on this Mac.
#   NOTARY_KEYCHAIN_PROFILE   a profile saved by `xcrun notarytool store-credentials`, or
#   NOTARY_KEY_PATH, NOTARY_KEY_ID, NOTARY_ISSUER
#                             an App Store Connect API key (.p8 path, key ID, issuer ID).
set -eu
cd "$(dirname "$0")/.."

steam=false
case "${1-}" in
"") ;;
--steam) steam=true ;;
*)
    echo "usage: scripts/build-macos.sh [--steam]" >&2
    exit 2
    ;;
esac

if [ "$(uname -s)" != Darwin ]; then
    echo "Build this native app bundle on macOS." >&2
    exit 1
fi

pkgid=$(cargo pkgid)
version=${pkgid##*[#@]}
targets="aarch64-apple-darwin x86_64-apple-darwin"
# Both slices must honour the LSMinimumSystemVersion that Info.plist declares.
MACOSX_DEPLOYMENT_TARGET=$(plutil -extract LSMinimumSystemVersion raw packaging/macos/Info.plist)
export MACOSX_DEPLOYMENT_TARGET

for target in $targets; do
    if command -v rustup >/dev/null && ! rustup target list --installed | grep -qx "$target"; then
        echo "Missing Rust target $target. Install it with: rustup target add $target" >&2
        exit 1
    fi
    if $steam; then
        cargo build --locked --release --bin arkonk --target "$target" --features steam
    else
        cargo build --locked --release --bin arkonk --target "$target"
    fi
done

app=target/ARKONK.app
contents=$app/Contents
rm -rf "$app"
mkdir -p "$contents/MacOS" "$contents/Resources"
lipo -create -output "$contents/MacOS/arkonk" \
    target/aarch64-apple-darwin/release/arkonk target/x86_64-apple-darwin/release/arkonk
cp packaging/macos/Info.plist "$contents/Info.plist"
plutil -replace CFBundleShortVersionString -string "$version" "$contents/Info.plist"
plutil -replace CFBundleVersion -string "$version" "$contents/Info.plist"
cp packaging/icons/arkonk.icns "$contents/Resources/arkonk.icns"
if $steam; then
    # Its install name is @loader_path/libsteam_api.dylib: it must sit beside the executable.
    redist=$(scripts/steam-redist.sh macos)
    cp "$redist" "$contents/MacOS/libsteam_api.dylib"
fi

# Nested code is signed before the bundle that seals it.
sign() {
    for code in "$contents/MacOS/libsteam_api.dylib" "$app"; do
        [ -e "$code" ] || continue
        codesign --force "$@" "$code"
    done
}
if [ -n "${MACOS_SIGN_IDENTITY-}" ]; then
    # Notarization requires the hardened runtime and a secure timestamp.
    sign --options runtime --timestamp --sign "$MACOS_SIGN_IDENTITY"
    echo "Signed with $MACOS_SIGN_IDENTITY." >&2
else
    sign --sign -
    echo "Signed ad hoc: launchable on this Mac only. Set MACOS_SIGN_IDENTITY to distribute." >&2
fi
codesign --verify --strict "$app"

if [ -n "${NOTARY_KEYCHAIN_PROFILE-}" ]; then
    set -- --keychain-profile "$NOTARY_KEYCHAIN_PROFILE"
elif [ -n "${NOTARY_KEY_PATH-}" ]; then
    set -- --key "$NOTARY_KEY_PATH" --key-id "${NOTARY_KEY_ID:?NOTARY_KEY_ID is required with NOTARY_KEY_PATH}" \
        --issuer "${NOTARY_ISSUER:?NOTARY_ISSUER is required with NOTARY_KEY_PATH}"
else
    set --
fi
if [ "$#" -eq 0 ]; then
    echo "Notarization skipped: set NOTARY_KEYCHAIN_PROFILE or NOTARY_KEY_PATH, NOTARY_KEY_ID and NOTARY_ISSUER." >&2
elif [ -z "${MACOS_SIGN_IDENTITY-}" ]; then
    echo "Notarization needs a Developer ID signature; set MACOS_SIGN_IDENTITY." >&2
    exit 1
else
    submission=target/ARKONK-notarize.zip
    rm -f "$submission"
    ditto -c -k --keepParent "$app" "$submission"
    result=$(xcrun notarytool submit "$submission" "$@" --wait --output-format plist)
    status=$(printf '%s' "$result" | plutil -extract status raw -o - -)
    if [ "$status" != Accepted ]; then
        id=$(printf '%s' "$result" | plutil -extract id raw -o - -)
        xcrun notarytool log "$id" "$@" >&2
        echo "Notarization finished with status: $status" >&2
        exit 1
    fi
    xcrun stapler staple "$app"
    xcrun stapler validate "$app"
    echo "Notarized and stapled." >&2
fi

actual=$("$contents/MacOS/arkonk" --version)
if [ "$actual" != "arkonk $version" ]; then
    echo "Bundle reports '$actual', expected 'arkonk $version'." >&2
    exit 1
fi
echo "Built $app ($(lipo -archs "$contents/MacOS/arkonk")). Launch with: open $app"
