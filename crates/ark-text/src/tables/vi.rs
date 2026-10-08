//! Vietnamese. Neutral *bạn*; labels in capitals with their tone marks, all
//! precomposed (NFC).
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Phá tan vũ trụ",
        ContinueJourney => "Tiếp tục hành trình",
        NewJourney => "Hành trình mới",
        SectorSelect => "Chọn khu vực",
        ContinueDetail => "{0} · {1}",
        StatMedals => "HUY CHƯƠNG",
        StatBest => "KỶ LỤC",
        ActionServe => "Phát bóng",
        ActionRelease => "Thả",
        ActionSelect => "Chọn",
        ActionResume => "Tiếp tục",
        ActionBack => "Quay lại",
        Fullscreen => "Toàn màn hình",
        SectorsHeading => "Khu vực",
        PracticeNote => "Chơi luyện tập không ảnh hưởng hành trình của bạn",
        MedalClear => "HOÀN THÀNH",
        MedalClean => "HOÀN HẢO",
        MedalSwift => "THẦN TỐC",
        PlaySector => "Chơi khu vực {0}",
        UnlockHint => "Hoàn thành khu vực {0} để mở khóa",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · KHU VỰC {1}",
        Paused => "Tạm dừng",
        RetrySector => "Chơi lại khu vực",
        MainMenu => "Menu chính",
        JourneyComplete => "Hoàn thành hành trình",
        OneMoreOrbit => "Thêm một vòng nữa?",
        StatPoints => "ĐIỂM",
        SectorClear => "Đã hoàn thành khu vực",
        StatTime => "THỜI GIAN",
        StatBonus => "THƯỞNG",
        StatChain => "CHUỖI",
        ExtraLife => "Hoàn thành chương · +1 mạng",
        NextSector => "Khu vực tiếp theo",
        BackToSectors => "Về danh sách khu vực",
        SaveFailed => "Không thể lưu tiến trình",
        PerfTitle => "Hiệu năng / CPU",
        HelpResume => "Tiếp tục từ chỗ bạn dừng",
        HelpRetry => "Chơi lại từ điểm lưu của khu vực",
        HelpMainMenu => "Hành trình đã được lưu",
        HelpSectors => "Luyện tập ở khu vực đã mở",
        HelpNewJourney => "Lại từ khu vực 01 · giữ huy chương",
        Settings => "Cài đặt",
        HelpSettings => "Âm thanh, hiển thị, ngôn ngữ",
        SettingSound => "Âm thanh",
        SettingVolume => "Âm lượng",
        SettingDisplay => "Hiển thị",
        SettingLanguage => "Ngôn ngữ",
        DisplayWindow => "Cửa sổ",
        LanguageSystem => "Hệ thống",
        ActionAdjust => "Điều chỉnh",
        SettingEffects => "Hiệu ứng",
        SettingContrast => "Độ tương phản",
        LookStandard => "Tiêu chuẩn",
        EffectsReduced => "Giảm bớt",
        ContrastHigh => "Cao",
        KeySpace => "Cách",
        KeyEsc => "Esc",
        SectorsOf => "{0}/{1} khu vực",
        TargetBest => "{0} · kỷ lục {1}",
        LifeGained => "+1 mạng",
        PresenceMenus => "Đang ở menu",
        PresenceJourney => "Khu vực {0} · {1}",
        PresencePractice => "Đang luyện tập khu vực {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "BÌNH MINH",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "HOÀNG HÔN",
            Chapter::BlueHour => "GIỜ XANH",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "RỘNG",
            Power::Slow => "CHẬM",
            Power::Multi => "ĐA BÓNG",
            Power::Anchor => "NEO",
            Power::Phase => "PHA",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Tiếp tục"),
        UnlockHint => Some("Đã khóa"),
        RetrySector => Some("Chơi lại"),
        MainMenu => Some("Menu"),
        NewJourney => Some("Mới"),
        SectorSelect => Some("Khu vực"),
        NextSector => Some("Tiếp"),
        BackToSectors => Some("Khu vực"),
        Settings => Some("Cài đặt"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Tiếp tục"),
        PlaySector => Some("Chơi {0}"),
        ActionServe => Some("Phát"),
        ActionRelease => Some("Thả"),
        ActionSelect => Some("Chọn"),
        ActionBack => Some("Quay lại"),
        ActionAdjust => Some("Chỉnh"),
        SettingEffects => Some("Hiệu ứng"),
        SettingContrast => Some("Tương phản"),
        LookStandard => Some("Chuẩn"),
        EffectsReduced => Some("Ít"),
        ContrastHigh => Some("Cao"),
        StatBest => Some("KỶ LỤC"),
        StatMedals => Some("HUY HIỆU"),
        SectorsHeading => Some("Khu vực"),
        SaveFailed => Some("Chưa lưu"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Cách"),
        HelpResume => Some("Quay lại chơi"),
        HelpRetry => Some("Từ điểm lưu"),
        HelpMainMenu => Some("Đã lưu"),
        HelpSectors => Some("Luyện tập"),
        HelpNewJourney => Some("Từ khu vực 01"),
        HelpSettings => Some("Âm thanh, hiển thị"),
        SettingSound => Some("Âm thanh"),
        SettingVolume => Some("Âm lượng"),
        SettingDisplay => Some("Hiển thị"),
        SettingLanguage => Some("Ngôn ngữ"),
        DisplayWindow => Some("Cửa sổ"),
        Fullscreen => Some("Toàn màn"),
        LanguageSystem => Some("Hệ thống"),
        ExtraLife => Some("+1 mạng"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("KHU VỰC {1}"),
        JourneyComplete => Some("Hoàn thành"),
        SectorClear => Some("Hoàn thành"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "Tia sáng đầu",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Vệ tinh",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Luồng gió",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Chuyển cảnh",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Cộng hưởng",
    "Lăng kính",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Dòng ngầm",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Ánh tà",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Thị sai",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Siêu tân tinh",
    "Trăng lên",
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
    "Trở về",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "Tia sáng đầu",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Vệ tinh",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Luồng gió",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Chuyển cảnh",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Cộng hưởng",
    "Lăng kính",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Dòng ngầm",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Ánh tà",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Thị sai",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Siêu tân tinh",
    "Trăng lên",
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
    "Trở về",
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
    "{icon:anchor} Neo: bắt bóng, ngắm, rồi thả ra",
    "Armored bricks take two hits: watch the rims", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall", // awaiting translation
    "One door in: send the ball into the courtyard", // awaiting translation
    "Knock out the keystone and the arch is open",  // awaiting translation
    "Read the angles: each shadow is a shot",       // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart", // awaiting translation
    "Lõi hổ phách: mỗi vụ nổ lan tới bốn ô lân cận",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Mở đường xuyên qua hai tuyến tiếp sóng",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Các lõi kề nhau truyền tiếp phản ứng",
    "{icon:multi} Đa bóng: ba quả bóng, một lối mở",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Phá vào các hốc phía sau lớp giáp",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Pha: ba lần chạm gạch mà không nảy",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Xuyên thủng lớp vỏ, rồi kích nổ tuyến bên trong",
    "In by the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                      // awaiting translation
    "Three bells: ring each one from beneath",                     // awaiting translation
    "Bắt bóng quay về trong khi các bóng khác vẫn bay",
    "Đi theo tuyến tiếp sóng quanh vùng trung tâm trống",
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
    "Vòng quỹ đạo cuối: tận dụng mọi lối mở",
];
