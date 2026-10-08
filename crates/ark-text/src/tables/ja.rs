//! Japanese. Polite plain style for notes, terse imperatives for tips;
//! full-width punctuation (：・？) and a space between Japanese and Latin.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "宇宙を打ち砕け",
        ContinueJourney => "旅を続ける",
        NewJourney => "新しい旅",
        SectorSelect => "セクター選択",
        ContinueDetail => "{0}・{1}",
        StatMedals => "メダル",
        StatBest => "ベスト",
        ActionServe => "サーブ",
        ActionRelease => "リリース",
        ActionSelect => "決定",
        ActionResume => "再開",
        ActionBack => "戻る",
        Fullscreen => "フルスクリーン",
        SectorsHeading => "セクター",
        PracticeNote => "練習は旅の進行に影響しません",
        MedalClear => "クリア",
        MedalClean => "ノーミス",
        MedalSwift => "スピード",
        PlaySector => "セクター {0} をプレイ",
        UnlockHint => "セクター {0} をクリアで解放",
        Plus => "+{0}",
        ReadyEyebrow => "{0}・セクター {1}",
        Paused => "一時停止中",
        RetrySector => "セクターをリトライ",
        MainMenu => "メインメニュー",
        JourneyComplete => "旅の完了",
        OneMoreOrbit => "もう一周？",
        StatPoints => "ポイント",
        SectorClear => "セクタークリア",
        StatTime => "タイム",
        StatBonus => "ボーナス",
        StatChain => "チェイン",
        ExtraLife => "章クリア・ライフ +1",
        NextSector => "次のセクター",
        BackToSectors => "セクター選択へ",
        SaveFailed => "進行状況をセーブできませんでした",
        PerfTitle => "パフォーマンス / CPU",
        HelpResume => "中断したところから再開します",
        HelpRetry => "セクターのチェックポイントからやり直します",
        HelpMainMenu => "旅の進行はセーブされています",
        HelpSectors => "開放済みのセクターを練習できます",
        HelpNewJourney => "セクター 01 から再スタート・メダルは残ります",
        Settings => "設定",
        HelpSettings => "サウンド・画面・言語",
        SettingSound => "サウンド",
        SettingVolume => "音量",
        SettingDisplay => "画面",
        SettingLanguage => "言語",
        DisplayWindow => "ウインドウ",
        LanguageSystem => "システム",
        ActionAdjust => "調整",
        SettingEffects => "エフェクト",
        SettingContrast => "コントラスト",
        LookStandard => "標準",
        EffectsReduced => "控えめ",
        ContrastHigh => "高",
        KeySpace => "スペース",
        KeyEsc => "Esc",
        SectorsOf => "{0} / {1} セクター",
        TargetBest => "{0}・ベスト {1}",
        LifeGained => "ライフ +1",
        PresenceMenus => "メニュー画面",
        PresenceJourney => "セクター {0}・{1}",
        PresencePractice => "セクター {0}・{1} を練習中",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "暁",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "黄昏",
            Chapter::BlueHour => "ブルーアワー",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "ワイド",
            Power::Slow => "スロー",
            Power::Multi => "マルチボール",
            Power::Anchor => "アンカー",
            Power::Phase => "フェイズ",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("続ける"),
        UnlockHint => Some("ロック中"),
        RetrySector => Some("リトライ"),
        MainMenu => Some("メニュー"),
        NewJourney => Some("新規"),
        SectorSelect => Some("セクター"),
        NextSector => Some("次へ"),
        BackToSectors => Some("セクター"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("再開"),
        PlaySector => Some("プレイ {0}"),
        Settings => Some("設定"),
        ActionServe => Some("サーブ"),
        ActionRelease => Some("リリース"),
        ActionSelect => Some("決定"),
        ActionBack => Some("戻る"),
        ActionAdjust => Some("調整"),
        StatBest => Some("ベスト"),
        StatMedals => Some("メダル"),
        SectorsHeading => Some("セクター"),
        SaveFailed => Some("セーブ失敗"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("スペース"),
        HelpResume => Some("ゲームに戻ります"),
        HelpRetry => Some("チェックポイントから"),
        HelpMainMenu => Some("セーブ済み"),
        HelpSectors => Some("セクターを練習"),
        HelpNewJourney => Some("セクター 01 から"),
        HelpSettings => Some("サウンド・画面"),
        SettingSound => Some("サウンド"),
        SettingVolume => Some("音量"),
        SettingDisplay => Some("画面"),
        SettingLanguage => Some("言語"),
        DisplayWindow => Some("ウインドウ"),
        Fullscreen => Some("全画面"),
        LanguageSystem => Some("システム"),
        ExtraLife => Some("ライフ +1"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("セクター {1}"),
        SettingEffects => Some("効果"),
        SettingContrast => Some("コントラスト"),
        LookStandard => Some("標準"),
        EffectsReduced => Some("控えめ"),
        ContrastHigh => Some("高"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "夜明けの光",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "サテライト",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "スリップストリーム",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "クロスフェード",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "共鳴",
    "プリズム",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "引き潮",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "残光",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "視差",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "超新星",
    "月の出",
    "Gloaming",      // awaiting translation
    "Lamplight",     // awaiting translation
    "Fireflies",     // awaiting translation
    "Lighthouse",    // awaiting translation
    "Nocturne",      // awaiting translation
    "Deep Field",    // awaiting translation
    "Constellation", // awaiting translation
    "Penumbra",      // awaiting translation
    "Metronome",     // awaiting translation
    "Corona",        // awaiting translation
    "Syzygy",        // awaiting translation
    "Pulsar",        // awaiting translation
    "Shutter",       // awaiting translation
    "Umbra",         // awaiting translation
    "Totality",      // awaiting translation
    "Solar Wind",    // awaiting translation
    "Borealis",      // awaiting translation
    "Ribbons",       // awaiting translation
    "Polar Night",   // awaiting translation
    "Cathedral",     // awaiting translation
    "Shimmer",       // awaiting translation
    "Event Horizon", // awaiting translation
    "帰還",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "夜明け",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "サテライト",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "スリップ",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "クロス",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "共鳴",
    "プリズム",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "引き潮",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "残光",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "視差",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "超新星",
    "月の出",
    "Gloaming",      // awaiting translation
    "Lamplight",     // awaiting translation
    "Fireflies",     // awaiting translation
    "Lighthouse",    // awaiting translation
    "Nocturne",      // awaiting translation
    "Deep Field",    // awaiting translation
    "Stars",         // awaiting translation
    "Penumbra",      // awaiting translation
    "Metronome",     // awaiting translation
    "Corona",        // awaiting translation
    "Syzygy",        // awaiting translation
    "Pulsar",        // awaiting translation
    "Shutter",       // awaiting translation
    "Umbra",         // awaiting translation
    "Totality",      // awaiting translation
    "Solar Wind",    // awaiting translation
    "Borealis",      // awaiting translation
    "Ribbons",       // awaiting translation
    "Polar Night",   // awaiting translation
    "Cathedral",     // awaiting translation
    "Shimmer",       // awaiting translation
    "Event Horizon", // awaiting translation
    "帰還",
];

const TIPS: [&str; SECTOR_COUNT] = [
    "{icon:wide} Wide: catch the falling capsule", // awaiting translation
    "{icon:slow} Slow: catch it and the ball eases off", // awaiting translation
    "Steer: the paddle's edges send the ball wide", // awaiting translation
    "Lone sparks: chase each one down",            // awaiting translation
    "Send the ball up the open wings",             // awaiting translation
    "Clear each lantern from below",               // awaiting translation
    "Ride the swell: bank shots off the side walls", // awaiting translation
    "Chip the sun away from below, row by row",    // awaiting translation
    "{icon:anchor} アンカー：ボールを受け止め、狙って放つ",
    "Armored bricks take two hits; their rims count down", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall",       // awaiting translation
    "One door in: send the ball into the courtyard",       // awaiting translation
    "Knock out the keystone and the arch is open",         // awaiting translation
    "Read the angles: each shadow is a shot",              // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart",       // awaiting translation
    "琥珀のコア：爆発は隣接する4つに届く",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "2本のリレーラインの間に道を開こう",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "隣り合うコアが連鎖を伝える",
    "{icon:multi} マルチボール：3つのボールで1つの突破口を",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "装甲の奥にあるポケットへ切り込め",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} フェイズ：跳ね返らずにブロックへ3回接触",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "外殻を貫き、内側のルートに点火せよ",
    "Through the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                        // awaiting translation
    "Three bells: ring each one from beneath",                       // awaiting translation
    "他のボールが飛ぶ間に戻りをキャッチ",
    "空いた中心の周りをリレーに沿って進め",
    "Light is leaving: learn the field while you can", // awaiting translation
    "In the dark, your ball and keel carry the light", // awaiting translation
    "Fireflies: the cores glow even in the dark",      // awaiting translation
    "Sweep the keel's light across the coast",         // awaiting translation
    "More balls, more light: {icon:multi} Multiball shows the way", // awaiting translation
    "Deep field: every faint speck is a brick",        // awaiting translation
    "Join the stars: the bright ones are cores",       // awaiting translation
    "Gates fade on a beat: wait for the gap",          // awaiting translation
    "Tick, tock: the gates keep time",                 // awaiting translation
    "The corona opens on the beat: strike the core inside", // awaiting translation
    "A ghost gate shrugs off a blast: time the spark", // awaiting translation
    "A pulsar: quick beats, quick hands",              // awaiting translation
    "The shutter opens for a moment: be ready",        // awaiting translation
    "Half in shadow: the gated side waits for its beat", // awaiting translation
    "Ignite the sun while the moon is solid: the blast takes both", // awaiting translation
    "The wind runs one way: ride it to the cores",     // awaiting translation
    "Curtains of light: sweep them with {icon:multi} Multiball", // awaiting translation
    "Each ribbon is tied with a gate: cut both halves", // awaiting translation
    "The long night: aim by the light you carry",      // awaiting translation
    "Through the doors on the beat, up to the rose window", // awaiting translation
    "Everything at once: keep your eyes on the cores", // awaiting translation
    "Strike the ring as the gates turn solid: the blast takes them too", // awaiting translation
    "最後の一周：すべての突破口を生かせ",
];
