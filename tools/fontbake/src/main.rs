//! Bakes the glyph atlases in `crates/ark-glyphs/data/` from pinned Noto
//! releases: only the characters the string tables use, only at the sizes
//! `spec.rs` asks for, hinted, as deflated 16-level alpha with advances and
//! GPOS pair kerning. Thai clusters and Arabic letter forms are shaped
//! by the fonts themselves (`script.rs` says which). The game links the
//! output, never this tool.
//!
//! ```text
//! cargo run -p fontbake --release            # rewrite the data files
//! cargo run -p fontbake --release -- --check # fail if they would change
//! ```
//!
//! Fonts are fetched with `curl` into `target/fontbake/` and verified
//! against the SHA-256 pinned in `sources.rs` before use.
#[path = "../../../crates/ark-glyphs/src/script.rs"]
#[allow(dead_code)]
mod script;
#[path = "../../../crates/ark-glyphs/src/spec.rs"]
#[allow(dead_code)]
mod spec;

mod bake;
mod charsets;
mod sources;

use std::{
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

/// Where the data files live, relative to the workspace root.
const DATA: &str = "crates/ark-glyphs/data";

fn main() -> ExitCode {
    let check = std::env::args().skip(1).any(|a| a == "--check");
    match run(check) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("fontbake: {e}");
            ExitCode::FAILURE
        }
    }
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Bakes every group; with `check`, compares instead of writing. Returns
/// whether the committed files match (always true when writing).
fn run(check: bool) -> Result<bool, String> {
    let root = root();
    let cache = root.join("target/fontbake");
    let fonts = sources::fetch_all(&cache)?;
    let mut matches = true;
    let mut total = 0;
    for group in &charsets::GROUPS {
        let sets = charsets::collect(group)?;
        let first = bake::group(group, &sets, &fonts)?;
        // A second bake in the same process catches any nondeterminism
        // (hash order, uninitialised scratch) before it reaches a commit.
        let second = bake::group(group, &sets, &fonts)?;
        if first.bytes != second.bytes {
            return Err(format!("{}: two bakes differ", group.name));
        }
        print!("{}", first.report);
        total += first.bytes.len();
        let path = root.join(DATA).join(format!("{}.bin", group.name));
        if check {
            let committed = fs::read(&path).unwrap_or_default();
            if committed != first.bytes {
                eprintln!(
                    "{} differs from a fresh bake ({} bytes committed, {} baked)",
                    path.display(),
                    committed.len(),
                    first.bytes.len()
                );
                matches = false;
            }
        } else {
            fs::create_dir_all(path.parent().ok_or("no data directory")?)
                .map_err(|e| e.to_string())?;
            fs::write(&path, &first.bytes).map_err(|e| format!("{}: {e}", path.display()))?;
        }
    }
    println!("total {total} bytes");
    if check && matches {
        println!("committed atlases match a fresh bake");
    }
    Ok(matches)
}
