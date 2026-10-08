//! The pinned font files: where they come from and what they must hash to.
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path, process::Command};

/// Noto Sans 2.015, hinted TrueType, from the notofonts build repository.
macro_rules! noto_sans {
    ($file:literal) => {
        concat!(
            "https://raw.githubusercontent.com/notofonts/notofonts.github.io/",
            "b966e2e80feb783346a7d3b7c14c8d6e7e1a757b/fonts/NotoSans/hinted/ttf/",
            $file
        )
    };
}
/// Noto Sans Display 2.007, hinted TrueType, as last published by the
/// Noto project (the family has not moved to the notofonts build repository).
macro_rules! noto_display {
    ($file:literal) => {
        concat!(
            "https://raw.githubusercontent.com/notofonts/noto-fonts/",
            "503d300da2bbdec7d00e0ff9078876ec7d6dbb92/hinted/ttf/NotoSansDisplay/",
            $file
        )
    };
}
/// Noto Sans CJK 2.004 (the commit tagged `Sans2.004`), region subset
/// OpenType/CFF.
macro_rules! noto_cjk {
    ($file:literal) => {
        concat!(
            "https://raw.githubusercontent.com/notofonts/noto-cjk/",
            "523d033d6cb47f4a80c58a35753646f5c3608a78/Sans/SubsetOTF/",
            $file
        )
    };
}

/// Download URL and SHA-256 of every font the atlases come from.
pub const FONTS: [(&str, &str); 9] = [
    (
        noto_sans!("NotoSans-Regular.ttf"),
        "478c558ea716033cd60c03438f628dfa75694dcf6b5f6d505a2f05fd2b4f3823",
    ),
    (
        noto_sans!("NotoSans-Medium.ttf"),
        "635d93d1131d791f2576de90b3bb0f7cdf61929906e8420a61b5f7f8e76420bb",
    ),
    (
        noto_display!("NotoSansDisplay-Medium.ttf"),
        "96775693878d9436e30b6a9b355bb38b49cdd42b964c98f297706043e4b9f4f5",
    ),
    (
        noto_cjk!("SC/NotoSansSC-Regular.otf"),
        "faa6c9df652116dde789d351359f3d7e5d2285a2b2a1f04a2d7244df706d5ea9",
    ),
    (
        noto_cjk!("SC/NotoSansSC-Medium.otf"),
        "7633f5a016d4dd95e685a69633d818aabc4644c4b08e26bd35b1b30c45ed5dda",
    ),
    (
        noto_cjk!("JP/NotoSansJP-Regular.otf"),
        "dff723ba59d57d136764a04b9b2d03205544f7cd785a711442d6d2d085ac5073",
    ),
    (
        noto_cjk!("JP/NotoSansJP-Medium.otf"),
        "f396a3b57256e4515be9cb41f7aac54766d654890082a9f1b5c2451b5c093d8a",
    ),
    (
        noto_cjk!("KR/NotoSansKR-Regular.otf"),
        "69975a0ac8472717870aefeab0a4d52739308d90856b9955313b2ad5e0148d68",
    ),
    (
        noto_cjk!("KR/NotoSansKR-Medium.otf"),
        "b46988ef13e8bac08f3933af686eaf770972994f9b6d335be0184d60169b5431",
    ),
];

/// The file name a font is cached and looked up under.
pub fn file_name(url: &str) -> &str {
    url.rsplit('/').next().unwrap_or(url)
}

pub fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// Every pinned font, read from `cache` or downloaded into it, verified.
pub fn fetch_all(cache: &Path) -> Result<BTreeMap<&'static str, Vec<u8>>, String> {
    fs::create_dir_all(cache).map_err(|e| format!("{}: {e}", cache.display()))?;
    let mut fonts = BTreeMap::new();
    for (url, expected) in FONTS {
        let name = file_name(url);
        let path = cache.join(name);
        let cached = fs::read(&path).ok().filter(|b| sha256(b) == expected);
        let bytes = match cached {
            Some(bytes) => bytes,
            None => {
                eprintln!("fetching {url}");
                let partial = path.with_extension("part");
                let status = Command::new("curl")
                    .args(["--fail", "--location", "--silent", "--show-error"])
                    .args(["--retry", "3", "--max-time", "300", "--output"])
                    .arg(&partial)
                    .arg(url)
                    .status()
                    .map_err(|e| format!("could not run curl: {e}"))?;
                if !status.success() {
                    return Err(format!("curl failed for {url} ({status})"));
                }
                let bytes = fs::read(&partial).map_err(|e| e.to_string())?;
                let actual = sha256(&bytes);
                if actual != expected {
                    return Err(format!(
                        "{name}: SHA-256 {actual}, pinned {expected}; refusing to bake from it"
                    ));
                }
                fs::rename(&partial, &path).map_err(|e| e.to_string())?;
                bytes
            }
        };
        fonts.insert(name, bytes);
    }
    Ok(fonts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_matches_known_vectors() {
        assert_eq!(
            sha256(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
