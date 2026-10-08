//! Korean. Polite *-세요* imperatives; ASCII colons and question marks,
//! as Korean sets them.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "우주를 부숴라",
        ContinueJourney => "여정 계속하기",
        NewJourney => "새 여정",
        SectorSelect => "섹터 선택",
        ContinueDetail => "{0} · {1}",
        StatMedals => "메달",
        StatBest => "최고 점수",
        ActionServe => "서브",
        ActionRelease => "놓기",
        ActionSelect => "선택",
        ActionResume => "재개",
        ActionBack => "뒤로",
        Fullscreen => "전체 화면",
        SectorsHeading => "섹터",
        PracticeNote => "연습은 여정에 영향을 주지 않습니다",
        MedalClear => "클리어",
        MedalClean => "무결점",
        MedalSwift => "신속",
        PlaySector => "섹터 {0} 플레이",
        UnlockHint => "섹터 {0} 완료 시 잠금 해제",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · 섹터 {1}",
        Paused => "일시정지",
        RetrySector => "섹터 재시도",
        MainMenu => "메인 메뉴",
        JourneyComplete => "여정 완료",
        OneMoreOrbit => "한 바퀴 더?",
        StatPoints => "점수",
        SectorClear => "섹터 클리어",
        StatTime => "시간",
        StatBonus => "보너스",
        StatChain => "연쇄",
        ExtraLife => "챕터 완료 · 생명 +1",
        NextSector => "다음 섹터",
        BackToSectors => "섹터로 돌아가기",
        SaveFailed => "진행 상황을 저장하지 못했습니다",
        PerfTitle => "성능 / CPU",
        HelpResume => "멈춘 곳부터 이어서 합니다",
        HelpRetry => "섹터 체크포인트부터 다시 시작합니다",
        HelpMainMenu => "여정이 저장되어 있습니다",
        HelpSectors => "열린 섹터를 연습합니다",
        HelpNewJourney => "섹터 01부터 다시 시작 · 메달은 유지됩니다",
        Settings => "설정",
        HelpSettings => "사운드, 화면, 언어",
        SettingSound => "사운드",
        SettingVolume => "볼륨",
        SettingDisplay => "화면",
        SettingLanguage => "언어",
        DisplayWindow => "창 모드",
        LanguageSystem => "시스템",
        ActionAdjust => "조정",
        SettingEffects => "효과",
        SettingContrast => "대비",
        LookStandard => "표준",
        EffectsReduced => "줄임",
        ContrastHigh => "높음",
        KeySpace => "스페이스",
        KeyEsc => "Esc",
        SectorsOf => "{1}개 섹터 중 {0}개",
        TargetBest => "{0} · 최고 {1}",
        LifeGained => "생명 +1",
        PresenceMenus => "메뉴 화면",
        PresenceJourney => "섹터 {0} · {1}",
        PresencePractice => "섹터 {0} · {1} 연습 중",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "새벽",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "황혼",
            Chapter::BlueHour => "블루 아워",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "와이드",
            Power::Slow => "슬로우",
            Power::Multi => "멀티볼",
            Power::Anchor => "앵커",
            Power::Phase => "페이즈",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("계속"),
        UnlockHint => Some("잠김"),
        RetrySector => Some("재시도"),
        MainMenu => Some("메뉴"),
        NewJourney => Some("새로"),
        SectorSelect => Some("섹터"),
        NextSector => Some("다음"),
        BackToSectors => Some("섹터"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("재개"),
        PlaySector => Some("플레이 {0}"),
        Settings => Some("설정"),
        ActionServe => Some("서브"),
        ActionRelease => Some("놓기"),
        ActionSelect => Some("선택"),
        ActionBack => Some("뒤로"),
        ActionAdjust => Some("조정"),
        StatBest => Some("최고"),
        StatMedals => Some("메달"),
        SectorsHeading => Some("섹터"),
        SaveFailed => Some("저장 실패"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("스페이스"),
        HelpResume => Some("게임으로 돌아갑니다"),
        HelpRetry => Some("체크포인트부터"),
        HelpMainMenu => Some("여정 저장됨"),
        HelpSectors => Some("섹터 연습"),
        HelpNewJourney => Some("섹터 01부터"),
        HelpSettings => Some("사운드, 화면"),
        SettingSound => Some("사운드"),
        SettingVolume => Some("볼륨"),
        SettingDisplay => Some("화면"),
        SettingLanguage => Some("언어"),
        DisplayWindow => Some("창"),
        Fullscreen => Some("전체"),
        LanguageSystem => Some("시스템"),
        ExtraLife => Some("생명 +1"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("섹터 {1}"),
        SettingEffects => Some("효과"),
        SettingContrast => Some("대비"),
        LookStandard => Some("표준"),
        EffectsReduced => Some("줄임"),
        ContrastHigh => Some("높음"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "첫 빛",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "위성",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "슬립스트림",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "크로스페이드",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "공명",
    "프리즘",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "역류",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "잔광",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "시차",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "초신성",
    "월출",
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
    "귀환",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "첫 빛",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "위성",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "슬립",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "크로스",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "공명",
    "프리즘",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "역류",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "잔광",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "시차",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "초신성",
    "월출",
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
    "귀환",
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
    "{icon:anchor} 앵커: 공을 잡고, 조준한 뒤 놓으세요",
    "Armored bricks take two hits: watch the rims", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall", // awaiting translation
    "One door in: send the ball into the courtyard", // awaiting translation
    "Knock out the keystone and the arch is open",  // awaiting translation
    "Read the angles: each shadow is a shot",       // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart", // awaiting translation
    "호박색 코어: 폭발이 인접한 네 칸에 닿습니다",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "두 릴레이 라인 사이로 길을 여세요",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "이웃한 코어가 연쇄 반응을 이어 갑니다",
    "{icon:multi} 멀티볼: 세 개의 공, 하나의 틈",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "장갑 뒤의 빈 공간으로 파고드세요",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} 페이즈: 튕기지 않고 벽돌에 세 번 접촉",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "외피를 뚫고 안쪽 경로에 불을 붙이세요",
    "In by the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                      // awaiting translation
    "Three bells: ring each one from beneath",                     // awaiting translation
    "다른 공이 날아가는 동안 돌아오는 공을 잡으세요",
    "빈 중앙을 따라 릴레이를 쫓으세요",
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
    "마지막 한 바퀴: 모든 틈을 살리세요",
];
