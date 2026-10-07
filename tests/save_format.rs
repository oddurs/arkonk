//! The version-1 save file, byte for byte. Players' existing files must keep
//! loading and must be rewritten exactly as before.
use arkonk::profile::{Checkpoint, Profile, Record};

#[test]
fn encoded_layout() {
    let mut p = Profile {
        best_score: 24600,
        unlocked: 7,
        muted: true,
        volume: 3,
        fullscreen: true,
        ..Profile::default()
    };
    p.records[0] = Record {
        medals: 7,
        best_ticks: 15400,
    };
    p.records[5] = Record {
        medals: 3,
        best_ticks: 30001,
    };
    p.checkpoint = Some(Checkpoint {
        level: 6,
        score: 31250,
        lives: 4,
        ticks: 99000,
    });
    insta::assert_snapshot!(visible(&p.encode()));
}

#[test]
fn default_layout() {
    insta::assert_snapshot!(visible(&Profile::default().encode()));
}

/// Damaged, duplicated, out-of-range and unknown lines, decoded and written
/// back: the result pins which lines survive and how values are clamped.
#[test]
fn damaged_file_rewritten() {
    let file: &[u8] = b"ARKONK 1\r\n\
        best 100\r\n\
        best\xc2\xa05\n\
        best 24600\n\
        unlocked 999\n\
        settings 1 0 4\n\
        display 1\n\
        record 3 255 7000\n\
        record 12 7 1\n\
        record 0 7 1 2\n\
        record 1 \xff 9\n\
        record 2 3 +40\n\
        checkpoint 5 31250 4 99000\n\
        future 1 2 3 4 5\n\
        \tunlocked\t11 \n\
        volume 3\n";
    let p = Profile::decode(&String::from_utf8_lossy(file)).unwrap();
    insta::assert_snapshot!(visible(&p.encode()));
}

/// Marks every line end, so the snapshot also pins the final newline and
/// shows that no line ends in `\r`.
fn visible(file: &str) -> String {
    file.replace('\n', "\\n\n")
}
