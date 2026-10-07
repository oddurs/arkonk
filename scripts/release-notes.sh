#!/bin/sh
# Prints one CHANGELOG.md section, for release notes:
#   scripts/release-notes.sh 0.2.0
#   scripts/release-notes.sh Unreleased
set -eu
cd "$(dirname "$0")/.."
if [ "$#" -ne 1 ]; then
    echo "usage: scripts/release-notes.sh VERSION" >&2
    exit 2
fi
notes=$(awk -v heading="## [$1]" '
    /^## \[/ { if (found) exit; found = index($0, heading) == 1; next }
    found
' CHANGELOG.md)
if [ -z "$notes" ]; then
    echo "CHANGELOG.md has no '## [$1]' section with entries." >&2
    exit 1
fi
printf '%s\n' "$notes"
