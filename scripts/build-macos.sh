#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
if [ "$(uname -s)" != Darwin ]; then
    echo "Build this native app bundle on macOS." >&2
    exit 1
fi
cargo build --locked --release --bin arkonk
app="target/ARKONK.app"
mkdir -p "$app/Contents/MacOS"
cp target/release/arkonk "$app/Contents/MacOS/arkonk"
cp packaging/macos/Info.plist "$app/Contents/Info.plist"
# Local development signature. Distribution still needs Developer ID signing
# and notarization; this does not use an account or publish anything.
codesign --force --sign - "$app"
echo "Built $app. Launch with: open $app"
