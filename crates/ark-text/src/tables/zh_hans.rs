//! Simplified Chinese. Full-width punctuation (：、？); a space separates
//! Chinese from Latin letters and numbers.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "击碎宇宙",
        ContinueJourney => "继续旅程",
        NewJourney => "新的旅程",
        SectorSelect => "选择星区",
        ContinueDetail => "{0} · {1}",
        StatMedals => "奖章",
        StatBest => "最高分",
        ActionServe => "发球",
        ActionRelease => "释放",
        ActionSelect => "选择",
        ActionResume => "继续",
        ActionBack => "返回",
        Fullscreen => "全屏",
        SectorsHeading => "星区",
        PracticeNote => "练习不会影响你的旅程",
        MedalClear => "通关",
        MedalClean => "无损",
        MedalSwift => "神速",
        PlaySector => "开始星区 {0}",
        UnlockHint => "完成星区 {0} 即可解锁",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · 星区 {1}",
        Paused => "已暂停",
        RetrySector => "重试本星区",
        MainMenu => "主菜单",
        JourneyComplete => "旅程完成",
        OneMoreOrbit => "再来一圈？",
        StatPoints => "得分",
        SectorClear => "星区完成",
        StatTime => "时间",
        StatBonus => "奖励",
        StatChain => "连击",
        ExtraLife => "篇章完成 · +1 生命",
        NextSector => "下一星区",
        BackToSectors => "返回星区",
        SaveFailed => "无法保存进度",
        PerfTitle => "性能 / CPU",
        HelpResume => "从暂停处继续",
        HelpRetry => "从本星区检查点重新开始",
        HelpMainMenu => "旅程已保存",
        HelpSectors => "练习任一已开放的星区",
        HelpNewJourney => "从星区 01 重新开始 · 奖章保留",
        Settings => "设置",
        HelpSettings => "声音、显示、语言",
        SettingSound => "声音",
        SettingVolume => "音量",
        SettingDisplay => "显示",
        SettingLanguage => "语言",
        DisplayWindow => "窗口",
        LanguageSystem => "跟随系统",
        ActionAdjust => "调整",
        SettingEffects => "特效",
        SettingContrast => "对比度",
        LookStandard => "标准",
        EffectsReduced => "减弱",
        ContrastHigh => "高",
        KeySpace => "空格",
        KeyEsc => "Esc",
        SectorsOf => "{0} / {1} 个星区",
        TargetBest => "{0} · 最佳 {1}",
        LifeGained => "+1 生命",
        PresenceMenus => "在菜单中",
        PresenceJourney => "星区 {0} · {1}",
        PresencePractice => "正在练习星区 {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "破晓",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "暮光",
            Chapter::BlueHour => "蓝调时刻",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "加宽",
            Power::Slow => "减速",
            Power::Multi => "多球",
            Power::Anchor => "锚定",
            Power::Phase => "相位",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("继续"),
        UnlockHint => Some("未解锁"),
        RetrySector => Some("重试"),
        MainMenu => Some("菜单"),
        NewJourney => Some("新旅程"),
        SectorSelect => Some("星区"),
        NextSector => Some("下一个"),
        BackToSectors => Some("星区"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("继续"),
        PlaySector => Some("开始 {0}"),
        Settings => Some("设置"),
        ActionServe => Some("发球"),
        ActionRelease => Some("释放"),
        ActionSelect => Some("选择"),
        ActionBack => Some("返回"),
        ActionAdjust => Some("调整"),
        StatBest => Some("最高"),
        StatMedals => Some("奖章"),
        SectorsHeading => Some("星区"),
        SaveFailed => Some("未保存"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("空格"),
        HelpResume => Some("返回游戏"),
        HelpRetry => Some("从检查点重来"),
        HelpMainMenu => Some("旅程已保存"),
        HelpSectors => Some("练习星区"),
        HelpNewJourney => Some("从星区 01 开始"),
        HelpSettings => Some("声音、显示"),
        SettingSound => Some("声音"),
        SettingVolume => Some("音量"),
        SettingDisplay => Some("显示"),
        SettingLanguage => Some("语言"),
        DisplayWindow => Some("窗口"),
        Fullscreen => Some("全屏"),
        LanguageSystem => Some("系统"),
        ExtraLife => Some("+1 生命"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("星区 {1}"),
        SettingEffects => Some("特效"),
        SettingContrast => Some("对比度"),
        LookStandard => Some("标准"),
        EffectsReduced => Some("减弱"),
        ContrastHigh => Some("高"),
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
    "卫星",
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
    "交错",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "共振",
    "棱镜",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "暗流",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "余晖",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "视差",
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
    "Singularity",   // awaiting translation
    "归途",
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
    "卫星",
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
    "交错",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "共振",
    "棱镜",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "暗流",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "余晖",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "视差",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "超新星",
    "月出",
    "Gloaming",    // awaiting translation
    "Lamplight",   // awaiting translation
    "Fireflies",   // awaiting translation
    "Lighthouse",  // awaiting translation
    "Nocturne",    // awaiting translation
    "Deep Field",  // awaiting translation
    "Stars",       // awaiting translation
    "Penumbra",    // awaiting translation
    "Metronome",   // awaiting translation
    "Corona",      // awaiting translation
    "Syzygy",      // awaiting translation
    "Pulsar",      // awaiting translation
    "Shutter",     // awaiting translation
    "Umbra",       // awaiting translation
    "Totality",    // awaiting translation
    "Solar Wind",  // awaiting translation
    "Borealis",    // awaiting translation
    "Ribbons",     // awaiting translation
    "Polar Night", // awaiting translation
    "Cathedral",   // awaiting translation
    "Shimmer",     // awaiting translation
    "Singularity", // awaiting translation
    "归途",
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
    "{icon:anchor} 锚定：接住球，瞄准，然后释放",
    "Armored bricks take two hits: watch the rims", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall", // awaiting translation
    "One door in: send the ball into the courtyard", // awaiting translation
    "Knock out the keystone and the arch is open",  // awaiting translation
    "Read the angles: each shadow is a shot",       // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart", // awaiting translation
    "琥珀核心：每次爆炸波及相邻四块",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "在两条中继线之间打开通路",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "相邻核心会传递连锁反应",
    "{icon:multi} 多球：三颗球，一个缺口",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "攻入装甲后方的空腔",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} 相位：连续三次穿透砖块，无需反弹",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "击穿外壳，再点燃内部通路",
    "In by the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                      // awaiting translation
    "Three bells: ring each one from beneath",                     // awaiting translation
    "其他球仍在飞行时，接住一颗回球",
    "沿着中继线绕过空旷的中心",
    "Light is leaving: learn the field while you can", // awaiting translation
    "In the dark, your ball and keel carry the light", // awaiting translation
    "Fireflies: the cores glow even in the dark",      // awaiting translation
    "Sweep the keel's light across the coast",         // awaiting translation
    "More balls, more light: {icon:multi} Multiball",  // awaiting translation
    "Deep field: every faint speck is a brick",        // awaiting translation
    "Join the stars: the bright ones are cores",       // awaiting translation
    "Gates fade on a beat: wait for the gap",          // awaiting translation
    "Tick, tock: the gates keep time",                 // awaiting translation
    "The corona opens on the beat: strike inside",     // awaiting translation
    "A ghost gate shrugs off a blast: time the spark", // awaiting translation
    "A pulsar: quick beats, quick hands",              // awaiting translation
    "The shutter opens for a moment: be ready",        // awaiting translation
    "Half in shadow: the gated side keeps time",       // awaiting translation
    "Ignite the sun while the moon is solid",          // awaiting translation
    "The wind runs one way: ride it to the cores",     // awaiting translation
    "Curtains of light: sweep them with {icon:multi} Multiball", // awaiting translation
    "Each ribbon is tied with a gate: cut both halves", // awaiting translation
    "The long night: aim by the light you carry",      // awaiting translation
    "In through the doors, up to the rose window",     // awaiting translation
    "Everything at once: watch the cores",             // awaiting translation
    "Strike the ring as the gates turn solid",         // awaiting translation
    "最后一圈：让每个缺口都有价值",
];
