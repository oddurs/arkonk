//! Traditional Chinese, as used in Taiwan (滑鼠, 螢幕, 儲存). Full-width
//! punctuation; a space separates Chinese from Latin letters and numbers.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "擊碎宇宙",
        ContinueJourney => "繼續旅程",
        NewJourney => "新的旅程",
        SectorSelect => "選擇星區",
        ContinueDetail => "{0} · {1}",
        StatMedals => "獎章",
        StatBest => "最高分",
        ActionServe => "發球",
        ActionRelease => "釋放",
        ActionSelect => "選擇",
        ActionResume => "繼續",
        ActionBack => "返回",
        Fullscreen => "全螢幕",
        SectorsHeading => "星區",
        PracticeNote => "練習不會影響你的旅程",
        MedalClear => "通關",
        MedalClean => "無損",
        MedalSwift => "神速",
        PlaySector => "開始星區 {0}",
        UnlockHint => "完成星區 {0} 即可解鎖",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · 星區 {1}",
        Paused => "已暫停",
        RetrySector => "重試本星區",
        MainMenu => "主選單",
        JourneyComplete => "旅程完成",
        OneMoreOrbit => "再來一圈？",
        StatPoints => "得分",
        SectorClear => "星區完成",
        StatTime => "時間",
        StatBonus => "獎勵",
        StatChain => "連擊",
        ExtraLife => "篇章完成 · +1 生命",
        NextSector => "下一個星區",
        BackToSectors => "返回星區",
        SaveFailed => "無法儲存進度",
        PerfTitle => "效能 / CPU",
        HelpResume => "從暫停處繼續",
        HelpRetry => "從本星區檢查點重新開始",
        HelpMainMenu => "旅程已儲存",
        HelpSectors => "練習任一已開放的星區",
        HelpNewJourney => "從星區 01 重新開始 · 獎章保留",
        Settings => "設定",
        HelpSettings => "音效、顯示、語言",
        SettingSound => "音效",
        SettingVolume => "音量",
        SettingDisplay => "顯示",
        SettingLanguage => "語言",
        DisplayWindow => "視窗",
        LanguageSystem => "跟隨系統",
        ActionAdjust => "調整",
        SettingEffects => "特效",
        SettingContrast => "對比度",
        LookStandard => "標準",
        EffectsReduced => "減弱",
        ContrastHigh => "高",
        KeySpace => "空白鍵",
        KeyEsc => "Esc",
        SectorsOf => "{0} / {1} 個星區",
        TargetBest => "{0} · 最佳 {1}",
        LifeGained => "+1 生命",
        PresenceMenus => "在選單中",
        PresenceJourney => "星區 {0} · {1}",
        PresencePractice => "正在練習星區 {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "破曉",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "暮光",
            Chapter::BlueHour => "藍調時刻",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "加寬",
            Power::Slow => "減速",
            Power::Multi => "多球",
            Power::Anchor => "錨定",
            Power::Phase => "相位",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("繼續"),
        UnlockHint => Some("未解鎖"),
        RetrySector => Some("重試"),
        MainMenu => Some("選單"),
        NewJourney => Some("新旅程"),
        SectorSelect => Some("星區"),
        NextSector => Some("下一個"),
        BackToSectors => Some("星區"),
        Settings => Some("設定"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("繼續"),
        PlaySector => Some("開始 {0}"),
        ActionServe => Some("發球"),
        ActionRelease => Some("釋放"),
        ActionSelect => Some("選擇"),
        ActionBack => Some("返回"),
        ActionAdjust => Some("調整"),
        SettingEffects => Some("特效"),
        SettingContrast => Some("對比度"),
        LookStandard => Some("標準"),
        EffectsReduced => Some("減弱"),
        ContrastHigh => Some("高"),
        StatBest => Some("最高"),
        StatMedals => Some("獎章"),
        SectorsHeading => Some("星區"),
        SaveFailed => Some("未儲存"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("空白"),
        HelpResume => Some("返回遊戲"),
        HelpRetry => Some("從檢查點重來"),
        HelpMainMenu => Some("旅程已儲存"),
        HelpSectors => Some("練習星區"),
        HelpNewJourney => Some("從星區 01 開始"),
        HelpSettings => Some("音效、顯示"),
        SettingSound => Some("音效"),
        SettingVolume => Some("音量"),
        SettingDisplay => Some("顯示"),
        SettingLanguage => Some("語言"),
        DisplayWindow => Some("視窗"),
        Fullscreen => Some("全螢幕"),
        LanguageSystem => Some("系統"),
        ExtraLife => Some("+1 生命"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("星區 {1}"),
        JourneyComplete => Some("完成"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "晨光",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "衛星",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "滑流",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "交錯",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "共振",
    "稜鏡",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "暗流",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "餘暉",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "視差",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "超新星",
    "月出",
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
    "歸途",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "晨光",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "衛星",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "滑流",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "交錯",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "共振",
    "稜鏡",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "暗流",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "餘暉",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "視差",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "超新星",
    "月出",
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
    "歸途",
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
    "{icon:anchor} 錨定：接住球，瞄準，然後釋放",
    "Armored bricks take two hits; their rims count down", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall",       // awaiting translation
    "One door in: send the ball into the courtyard",       // awaiting translation
    "Knock out the keystone and the arch is open",         // awaiting translation
    "Read the angles: each shadow is a shot",              // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart",       // awaiting translation
    "琥珀核心：每次爆炸會波及相鄰四塊",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "在兩條中繼線之間打開通路",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "相鄰核心會傳遞連鎖反應",
    "{icon:multi} 多球：三顆球，一個缺口",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "攻入裝甲後方的空腔",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} 相位：連續三次穿透磚塊，無需反彈",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "擊穿外殼，再點燃內部通路",
    "Through the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                        // awaiting translation
    "Three bells: ring each one from beneath",                       // awaiting translation
    "其他球仍在飛行時，接住一顆回球",
    "沿著中繼線繞過空曠的中心",
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
    "最後一圈：讓每個缺口都有價值",
];
