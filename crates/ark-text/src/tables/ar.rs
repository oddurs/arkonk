//! Arabic. Right to left; Latin digits, as CLDR's default for `ar`. No short
//! vowels or shadda: the atlases bake no combining marks. Labels have no case.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "حطم الكون",
        ContinueJourney => "متابعة الرحلة",
        NewJourney => "رحلة جديدة",
        SectorSelect => "اختيار القطاع",
        ContinueDetail => "{0} · {1}",
        StatMedals => "الميداليات",
        StatBest => "الأفضل",
        ActionServe => "إطلاق",
        ActionRelease => "إفلات",
        ActionSelect => "اختيار",
        ActionResume => "استئناف",
        ActionBack => "رجوع",
        Fullscreen => "ملء الشاشة",
        SectorsHeading => "القطاعات",
        PracticeNote => "جولات التدريب لا تغير رحلتك",
        MedalClear => "اجتياز",
        MedalClean => "بلا خسارة",
        MedalSwift => "سريع",
        PlaySector => "العب القطاع {0}",
        UnlockHint => "أكمل القطاع {0} لفتحه",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · القطاع {1}",
        Paused => "إيقاف مؤقت",
        RetrySector => "إعادة القطاع",
        MainMenu => "القائمة الرئيسية",
        JourneyComplete => "اكتملت الرحلة",
        OneMoreOrbit => "مدار آخر؟",
        StatPoints => "النقاط",
        SectorClear => "اكتمل القطاع",
        StatTime => "الوقت",
        StatBonus => "مكافأة",
        StatChain => "السلسلة",
        ExtraLife => "اكتمل الفصل · +1 حياة",
        NextSector => "القطاع التالي",
        BackToSectors => "العودة إلى القطاعات",
        SaveFailed => "تعذر حفظ التقدم",
        PerfTitle => "الأداء / المعالج",
        HelpResume => "تابع من حيث توقفت",
        HelpRetry => "ابدأ من نقطة حفظ القطاع",
        HelpMainMenu => "رحلتك محفوظة",
        HelpSectors => "تدرب على أي قطاع مفتوح",
        HelpNewJourney => "ابدأ من القطاع 01 · الميداليات تبقى",
        Settings => "الإعدادات",
        HelpSettings => "الصوت، العرض، اللغة",
        SettingSound => "الصوت",
        SettingVolume => "مستوى الصوت",
        SettingDisplay => "العرض",
        SettingLanguage => "اللغة",
        DisplayWindow => "نافذة",
        LanguageSystem => "النظام",
        ActionAdjust => "ضبط",
        SettingEffects => "المؤثرات",
        SettingContrast => "التباين",
        LookStandard => "قياسي",
        EffectsReduced => "مخففة",
        ContrastHigh => "مرتفع",
        KeySpace => "مسافة",
        KeyEsc => "Esc",
        SectorsOf => "القطاعات: {0} من {1}",
        TargetBest => "{0} · الأفضل {1}",
        LifeGained => "+1 حياة",
        PresenceMenus => "في القوائم",
        PresenceJourney => "القطاع {0} · {1}",
        PresencePractice => "يتدرب في القطاع {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "الفجر",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "الشفق",
            Chapter::BlueHour => "الساعة الزرقاء",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "عريض",
            Power::Slow => "بطيء",
            Power::Multi => "كرات متعددة",
            Power::Anchor => "مرساة",
            Power::Phase => "طور",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("متابعة"),
        UnlockHint => Some("مقفل"),
        RetrySector => Some("إعادة"),
        MainMenu => Some("القائمة"),
        NewJourney => Some("جديدة"),
        SectorSelect => Some("القطاعات"),
        NextSector => Some("التالي"),
        BackToSectors => Some("القطاعات"),
        Settings => Some("خيارات"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("استئناف"),
        PlaySector => Some("العب {0}"),
        ActionServe => Some("إطلاق"),
        ActionRelease => Some("إفلات"),
        ActionSelect => Some("اختيار"),
        ActionBack => Some("رجوع"),
        ActionAdjust => Some("ضبط"),
        SettingEffects => Some("المؤثرات"),
        SettingContrast => Some("التباين"),
        LookStandard => Some("عادي"),
        EffectsReduced => Some("أقل"),
        ContrastHigh => Some("مرتفع"),
        StatBest => Some("الأفضل"),
        StatMedals => Some("ميداليات"),
        SectorsHeading => Some("القطاعات"),
        SaveFailed => Some("لم يحفظ"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("مسافة"),
        HelpResume => Some("عودة إلى اللعب"),
        HelpRetry => Some("من نقطة الحفظ"),
        HelpMainMenu => Some("محفوظة"),
        HelpSectors => Some("تدريب القطاعات"),
        HelpNewJourney => Some("من القطاع 01"),
        HelpSettings => Some("الصوت، العرض"),
        SettingSound => Some("الصوت"),
        SettingVolume => Some("الصوت"),
        SettingDisplay => Some("العرض"),
        SettingLanguage => Some("اللغة"),
        DisplayWindow => Some("نافذة"),
        Fullscreen => Some("كاملة"),
        LanguageSystem => Some("النظام"),
        ExtraLife => Some("+1 حياة"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("القطاع {1}"),
        JourneyComplete => Some("اكتملت"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "الضوء الأول",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "أقمار صناعية",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "انسياب",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "تداخل",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "رنين",
    "منشور",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "التيار الساحب",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "وهج الغروب",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "اختلاف المنظر",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "مستعر أعظم",
    "طلوع القمر",
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
    "العودة",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "الضوء الأول",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "أقمار",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "انسياب",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "تداخل",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "رنين",
    "منشور",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "التيار",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "الوهج",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "المنظر",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "مستعر أعظم",
    "القمر",
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
    "العودة",
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
    "{icon:anchor} مرساة: التقط الكرة، صوب، ثم أطلقها",
    "Armored bricks take two hits: watch the rims", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall", // awaiting translation
    "One door in: send the ball into the courtyard", // awaiting translation
    "Knock out the keystone and the arch is open",  // awaiting translation
    "Read the angles: each shadow is a shot",       // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart", // awaiting translation
    "النوى الكهرمانية: كل انفجار يصل إلى جيرانه الأربعة",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "افتح طريقا عبر خطي المرحلات",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "النوى المتجاورة تنقل التفاعل",
    "{icon:multi} كرات متعددة: ثلاث كرات، فتحة واحدة",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "اخترق الجيوب خلف الدروع",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} طور: ثلاث ملامسات للطوب بلا ارتداد",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "اخترق القشرة ثم أشعل المسار الداخلي",
    "In by the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                      // awaiting translation
    "Three bells: ring each one from beneath",                     // awaiting translation
    "التقط كرة عائدة بينما تواصل الكرات الأخرى",
    "اتبع المرحل حول المركز المفتوح",
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
    "مدار أخير: اجعل كل فتحة مهمة",
];
