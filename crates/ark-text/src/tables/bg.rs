//! Bulgarian. Informal *ти*; labels in capitals.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Разбий космоса",
        ContinueJourney => "Продължи пътешествието",
        NewJourney => "Ново пътешествие",
        SectorSelect => "Избор на сектор",
        ContinueDetail => "{0} · {1}",
        StatMedals => "МЕДАЛИ",
        StatBest => "РЕКОРД",
        ActionServe => "Старт",
        ActionRelease => "Пусни",
        ActionSelect => "Избери",
        ActionResume => "Продължи",
        ActionBack => "Назад",
        Fullscreen => "Цял екран",
        SectorsHeading => "Сектори",
        PracticeNote => "Тренировките не променят пътешествието ти",
        MedalClear => "ПРЕМИНАТ",
        MedalClean => "БЕЗУПРЕЧЕН",
        MedalSwift => "БЪРЗ",
        PlaySector => "Играй сектор {0}",
        UnlockHint => "Премини сектор {0}, за да отключиш",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · СЕКТОР {1}",
        Paused => "Пауза",
        RetrySector => "Сектора отново",
        MainMenu => "Главно меню",
        JourneyComplete => "Пътят завърши",
        OneMoreOrbit => "Още една орбита?",
        StatPoints => "ТОЧКИ",
        SectorClear => "Секторът е преминат",
        StatTime => "ВРЕМЕ",
        StatBonus => "БОНУС",
        StatChain => "ВЕРИГА",
        ExtraLife => "Главата завърши · +1 живот",
        NextSector => "Следващ сектор",
        BackToSectors => "Към секторите",
        SaveFailed => "Напредъкът не можа да се запази",
        PerfTitle => "Производителност / CPU",
        HelpResume => "Продължи оттам, където спря",
        HelpRetry => "Започни от контролната точка на сектора",
        HelpMainMenu => "Пътешествието ти е запазено",
        HelpSectors => "Тренирай във всеки отворен сектор",
        HelpNewJourney => "Отначало от сектор 01 · медалите остават",
        Settings => "Настройки",
        HelpSettings => "Звук, екран, език",
        SettingSound => "Звук",
        SettingVolume => "Сила на звука",
        SettingDisplay => "Екран",
        SettingLanguage => "Език",
        DisplayWindow => "Прозорец",
        LanguageSystem => "Системен",
        ActionAdjust => "Промени",
        SettingEffects => "Ефекти",
        SettingContrast => "Контраст",
        LookStandard => "Стандарт",
        EffectsReduced => "Намалени",
        ContrastHigh => "Висок",
        KeySpace => "Интервал",
        KeyEsc => "Esc",
        SectorsOf => "{0} от {1} сектора",
        TargetBest => "{0} · рекорд {1}",
        LifeGained => "+1 живот",
        PresenceMenus => "В менютата",
        PresenceJourney => "Сектор {0} · {1}",
        PresencePractice => "Тренира сектор {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "ЗОРА",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "ЗДРАЧ",
            Chapter::BlueHour => "СИН ЧАС",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "ШИРОКО",
            Power::Slow => "БАВНО",
            Power::Multi => "МУЛТИТОПКА",
            Power::Anchor => "КОТВА",
            Power::Phase => "ФАЗА",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Продължи"),
        UnlockHint => Some("Заключен"),
        RetrySector => Some("Отново"),
        MainMenu => Some("Меню"),
        NewJourney => Some("Ново"),
        SectorSelect => Some("Сектори"),
        NextSector => Some("Напред"),
        BackToSectors => Some("Сектори"),
        Settings => Some("Опции"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Продължи"),
        PlaySector => Some("Играй {0}"),
        ActionServe => Some("Старт"),
        ActionRelease => Some("Пусни"),
        ActionSelect => Some("Избери"),
        ActionBack => Some("Назад"),
        ActionAdjust => Some("Промени"),
        SettingEffects => Some("Ефекти"),
        SettingContrast => Some("Контраст"),
        LookStandard => Some("Нормално"),
        EffectsReduced => Some("По-малко"),
        ContrastHigh => Some("Висок"),
        StatBest => Some("РЕКОРД"),
        StatMedals => Some("МЕДАЛИ"),
        SectorsHeading => Some("Сектори"),
        SaveFailed => Some("Не е запазено"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Интервал"),
        HelpResume => Some("Обратно в играта"),
        HelpRetry => Some("От контролната точка"),
        HelpMainMenu => Some("Пътят е запазен"),
        HelpSectors => Some("Тренировка"),
        HelpNewJourney => Some("От сектор 01"),
        HelpSettings => Some("Звук, екран"),
        SettingSound => Some("Звук"),
        SettingVolume => Some("Сила"),
        SettingDisplay => Some("Екран"),
        SettingLanguage => Some("Език"),
        DisplayWindow => Some("Прозорец"),
        Fullscreen => Some("Цял"),
        LanguageSystem => Some("Система"),
        ExtraLife => Some("+1 живот"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("СЕКТОР {1}"),
        JourneyComplete => Some("Завърши"),
        SectorClear => Some("Преминат"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "Първа светлина",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Спътници",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Въздушна следа",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Преливане",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Резонанс",
    "Призма",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Подводно течение",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Отблясък",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Паралакс",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Свръхнова",
    "Изгрев на луната",
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
    "Завръщане",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "Първа светлина",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Спътници",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Следа",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Преливане",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Резонанс",
    "Призма",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Течение",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Отблясък",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Паралакс",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Свръхнова",
    "Луна",
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
    "Завръщане",
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
    "{icon:anchor} Котва: хвани топката, прицели се и я пусни",
    "Armored bricks take two hits; their rims count down", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall",       // awaiting translation
    "One door in: send the ball into the courtyard",       // awaiting translation
    "Knock out the keystone and the arch is open",         // awaiting translation
    "Read the angles: each shadow is a shot",              // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart",       // awaiting translation
    "Кехлибарени ядра: всеки взрив достига четирите съседни",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Отвори път през двете релейни линии",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Съседните ядра пренасят реакцията",
    "{icon:multi} Мултитопка: три топки, един отвор",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Пробий до джобовете зад бронята",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Фаза: три удара в тухли без отскок",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Пробий черупката, после запали вътрешния път",
    "Through the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                        // awaiting translation
    "Three bells: ring each one from beneath",                       // awaiting translation
    "Хвани връщаща се топка, докато другите летят",
    "Следвай релето около откритата среда",
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
    "Последна орбита: всеки отвор е важен",
];
