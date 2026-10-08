//! Ukrainian. Formal *ви*, as Ukrainian game UI usually addresses players;
//! hints read "keys — action". The apostrophe is U+2019.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Розбий космос",
        ContinueJourney => "Продовжити подорож",
        NewJourney => "Нова подорож",
        SectorSelect => "Вибір сектора",
        ContinueDetail => "{0} · {1}",
        StatMedals => "МЕДАЛІ",
        StatBest => "РЕКОРД",
        ActionServe => "Подача",
        ActionRelease => "Відпустити",
        ActionSelect => "Вибрати",
        ActionResume => "Продовжити",
        ActionBack => "Назад",
        Fullscreen => "Повний екран",
        SectorsHeading => "Сектори",
        PracticeNote => "Тренування не впливають на вашу подорож",
        MedalClear => "ПРОЙДЕНО",
        MedalClean => "БЕЗ ВТРАТ",
        MedalSwift => "ШВИДКО",
        PlaySector => "Грати: сектор {0}",
        UnlockHint => "Пройдіть сектор {0}, щоб відкрити",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · СЕКТОР {1}",
        Paused => "Пауза",
        RetrySector => "Сектор знову",
        MainMenu => "Головне меню",
        JourneyComplete => "Подорож завершено",
        OneMoreOrbit => "Ще одна орбіта?",
        StatPoints => "ОЧКИ",
        SectorClear => "Сектор пройдено",
        StatTime => "ЧАС",
        StatBonus => "БОНУС",
        StatChain => "СЕРІЯ",
        ExtraLife => "Розділ завершено · +1 життя",
        NextSector => "Наступний сектор",
        BackToSectors => "До секторів",
        SaveFailed => "Не вдалося зберегти прогрес",
        PerfTitle => "Продуктивність / ЦП",
        HelpResume => "Продовжте з того ж місця",
        HelpRetry => "Почати знову з контрольної точки сектора",
        HelpMainMenu => "Вашу подорож збережено",
        HelpSectors => "Тренуйтеся в будь-якому відкритому секторі",
        HelpNewJourney => "Знову із сектора 01 · медалі залишаться",
        Settings => "Налаштування",
        HelpSettings => "Звук, екран, мова",
        SettingSound => "Звук",
        SettingVolume => "Гучність",
        SettingDisplay => "Екран",
        SettingLanguage => "Мова",
        DisplayWindow => "Вікно",
        LanguageSystem => "Системна",
        ActionAdjust => "Змінити",
        SettingEffects => "Ефекти",
        SettingContrast => "Контраст",
        LookStandard => "Стандарт",
        EffectsReduced => "Знижені",
        ContrastHigh => "Високий",
        KeySpace => "Пробіл",
        KeyEsc => "Esc",
        SectorsOf => "{0} з {1} секторів",
        TargetBest => "{0} · рекорд {1}",
        LifeGained => "+1 життя",
        PresenceMenus => "У меню",
        PresenceJourney => "Сектор {0} · {1}",
        PresencePractice => "Тренування: сектор {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "СВІТАНОК",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "СУТІНКИ",
            Chapter::BlueHour => "СИНЯ ГОДИНА",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "ШИРИНА",
            Power::Slow => "СПОВІЛЬНЕННЯ",
            Power::Multi => "МУЛЬТИМ’ЯЧ",
            Power::Anchor => "ЯКІР",
            Power::Phase => "ФАЗА",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Продовжити"),
        UnlockHint => Some("Закрито"),
        RetrySector => Some("Знову"),
        MainMenu => Some("Меню"),
        NewJourney => Some("Нова"),
        SectorSelect => Some("Сектори"),
        NextSector => Some("Далі"),
        BackToSectors => Some("Сектори"),
        Settings => Some("Опції"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Далі"),
        PlaySector => Some("Грати {0}"),
        ActionServe => Some("Подача"),
        ActionRelease => Some("Пуск"),
        ActionSelect => Some("Вибрати"),
        ActionBack => Some("Назад"),
        ActionAdjust => Some("Змінити"),
        SettingEffects => Some("Ефекти"),
        SettingContrast => Some("Контраст"),
        LookStandard => Some("Звичайно"),
        EffectsReduced => Some("Менше"),
        ContrastHigh => Some("Високий"),
        StatBest => Some("РЕКОРД"),
        StatMedals => Some("МЕДАЛІ"),
        SectorsHeading => Some("Сектори"),
        SaveFailed => Some("Не збережено"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Пробіл"),
        HelpResume => Some("Назад у гру"),
        HelpRetry => Some("З контрольної точки"),
        HelpMainMenu => Some("Збережено"),
        HelpSectors => Some("Тренування"),
        HelpNewJourney => Some("Із сектора 01"),
        HelpSettings => Some("Звук, екран"),
        SettingSound => Some("Звук"),
        SettingVolume => Some("Гучність"),
        SettingDisplay => Some("Екран"),
        SettingLanguage => Some("Мова"),
        DisplayWindow => Some("Вікно"),
        Fullscreen => Some("Повний"),
        LanguageSystem => Some("Система"),
        ExtraLife => Some("+1 життя"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("СЕКТОР {1}"),
        JourneyComplete => Some("Завершено"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "Перше світло",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Супутники",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Попутний потік",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Перехід",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Резонанс",
    "Призма",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Підводна течія",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Післясвітіння",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Паралакс",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Наднова",
    "Схід місяця",
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
    "Повернення",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "Перше світло",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Супутники",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Потік",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Перехід",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Резонанс",
    "Призма",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Течія",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Сяйво",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Паралакс",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Наднова",
    "Місяць",
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
    "Повернення",
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
    "{icon:anchor} Якір: спіймайте м’яч, прицільтеся й відпустіть",
    "Armored bricks take two hits; their rims count down", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall",       // awaiting translation
    "One door in: send the ball into the courtyard",       // awaiting translation
    "Knock out the keystone and the arch is open",         // awaiting translation
    "Read the angles: each shadow is a shot",              // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart",       // awaiting translation
    "Бурштинові ядра: кожен вибух зачіпає чотирьох сусідів",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Відкрийте шлях крізь дві лінії реле",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Сусідні ядра передають реакцію далі",
    "{icon:multi} Мультим’яч: три м’ячі, один прохід",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Пробийтеся в кишені за бронею",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Фаза: три удари по цеглі без відскоку",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Пробийте оболонку, потім підпаліть внутрішній шлях",
    "Through the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                        // awaiting translation
    "Three bells: ring each one from beneath",                       // awaiting translation
    "Спіймайте м’яч, поки інші летять",
    "Слідуйте за реле навколо відкритого центру",
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
    "Остання орбіта: хай кожен прохід важить",
];
