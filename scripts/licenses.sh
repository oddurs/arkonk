#!/bin/sh
# Writes the third-party license notice that ships in every package.
#   scripts/licenses.sh [output]   default: target/THIRD_PARTY_LICENSES.txt
# The notice is reproducible from Cargo.lock and this pinned cargo-about
# version; the release workflow installs the same version.
set -eu
cd "$(dirname "$0")/.."
about_version=0.9.2
out=${1:-target/THIRD_PARTY_LICENSES.txt}

if ! installed=$(cargo about --version 2>/dev/null); then
    echo "cargo-about is not installed. Install it with:" >&2
    echo "  cargo install cargo-about --locked --features cli --version $about_version" >&2
    exit 1
fi
if [ "$installed" != "cargo-about $about_version" ]; then
    echo "Found $installed; this notice is pinned to cargo-about $about_version." >&2
    echo "  cargo install cargo-about --locked --features cli --version $about_version" >&2
    exit 1
fi
mkdir -p "$(dirname "$out")"
# --all-features: the Steam build links extra crates, and one notice covers both.
cargo about generate --locked --all-features --fail \
    -c packaging/licenses/about.toml -o "$out" packaging/licenses/about.hbs
# The UI type is bitmaps rendered from these fonts (SIL OFL 1.1), linked into
# every build; cargo-about cannot see them, so their licenses follow here.
for font in packaging/licenses/fonts/*.txt; do
    printf '\n%s\n' "================================================================================" >>"$out"
    printf 'Font software, rendered into the glyph atlases in crates/ark-glyphs\n\n' >>"$out"
    cat "$font" >>"$out"
done
echo "Wrote $out" >&2
