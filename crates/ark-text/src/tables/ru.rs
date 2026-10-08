//! Russian. Formal *вы*, as Russian game UI usually addresses players;
//! hints read "key — action" with an em dash, the idiomatic short form.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Разбей космос",
        ContinueJourney => "Продолжить путь",
        NewJourney => "Новый путь",
        SectorSelect => "Выбор сектора",
        ContinueDetail => "{0} · {1}",
        StatMedals => "МЕДАЛИ",
        StatBest => "РЕКОРД",
        ActionServe => "Запуск",
        ActionRelease => "Отпустить",
        ActionSelect => "Выбрать",
        ActionResume => "Продолжить",
        ActionBack => "Назад",
        Fullscreen => "Полный экран",
        SectorsHeading => "Секторы",
        PracticeNote => "Тренировка не влияет на ваш путь",
        MedalClear => "ПРОЙДЕН",
        MedalClean => "БЕЗ ПОТЕРЬ",
        MedalSwift => "БЫСТРО",
        PlaySector => "Играть: сектор {0}",
        UnlockHint => "Пройдите сектор {0}, чтобы открыть",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · СЕКТОР {1}",
        Paused => "Пауза",
        RetrySector => "Сектор заново",
        MainMenu => "Главное меню",
        JourneyComplete => "Путь пройден",
        OneMoreOrbit => "Ещё одна орбита?",
        StatPoints => "ОЧКИ",
        SectorClear => "Сектор пройден",
        StatTime => "ВРЕМЯ",
        StatBonus => "БОНУС",
        StatChain => "СЕРИЯ",
        ExtraLife => "Глава пройдена · +1 жизнь",
        NextSector => "Следующий сектор",
        BackToSectors => "К секторам",
        SaveFailed => "Не удалось сохранить прогресс",
        PerfTitle => "Производительность / ЦП",
        HelpResume => "Продолжить с того же места",
        HelpRetry => "Начать заново с контрольной точки сектора",
        HelpMainMenu => "Ваш путь сохранён",
        HelpSectors => "Тренируйтесь в любом открытом секторе",
        HelpNewJourney => "Заново с сектора 01 · медали сохранятся",
        Settings => "Настройки",
        HelpSettings => "Звук, экран, язык",
        SettingSound => "Звук",
        SettingVolume => "Громкость",
        SettingDisplay => "Экран",
        SettingLanguage => "Язык",
        DisplayWindow => "Окно",
        LanguageSystem => "Системный",
        ActionAdjust => "Изменить",
        SettingEffects => "Эффекты",
        SettingContrast => "Контраст",
        LookStandard => "Стандарт",
        EffectsReduced => "Снижены",
        ContrastHigh => "Высокий",
        KeySpace => "Пробел",
        KeyEsc => "Esc",
        SectorsOf => "{0} из {1} секторов",
        TargetBest => "{0} · рекорд {1}",
        LifeGained => "+1 жизнь",
        PresenceMenus => "В меню",
        PresenceJourney => "Сектор {0} · {1}",
        PresencePractice => "Тренировка: сектор {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "РАССВЕТ",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "СУМЕРКИ",
            Chapter::BlueHour => "СИНИЙ ЧАС",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "ШИРИНА",
            Power::Slow => "ЗАМЕДЛЕНИЕ",
            Power::Multi => "МУЛЬТИМЯЧ",
            Power::Anchor => "ЯКОРЬ",
            Power::Phase => "ФАЗА",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Продолжить"),
        UnlockHint => Some("Закрыт"),
        RetrySector => Some("Заново"),
        MainMenu => Some("Меню"),
        NewJourney => Some("Заново"),
        SectorSelect => Some("Секторы"),
        NextSector => Some("Дальше"),
        BackToSectors => Some("Секторы"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Дальше"),
        PlaySector => Some("Играть {0}"),
        Settings => Some("Опции"),
        ActionServe => Some("Запуск"),
        ActionRelease => Some("Пуск"),
        ActionSelect => Some("Выбрать"),
        ActionBack => Some("Назад"),
        ActionAdjust => Some("Изменить"),
        StatBest => Some("РЕКОРД"),
        StatMedals => Some("МЕДАЛИ"),
        SectorsHeading => Some("Секторы"),
        SaveFailed => Some("Не сохранено"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Пробел"),
        HelpResume => Some("Назад в игру"),
        HelpRetry => Some("С контрольной точки"),
        HelpMainMenu => Some("Путь сохранён"),
        HelpSectors => Some("Тренировка"),
        HelpNewJourney => Some("С сектора 01"),
        HelpSettings => Some("Звук, экран"),
        SettingSound => Some("Звук"),
        SettingVolume => Some("Громкость"),
        SettingDisplay => Some("Экран"),
        SettingLanguage => Some("Язык"),
        DisplayWindow => Some("Окно"),
        Fullscreen => Some("Полный"),
        LanguageSystem => Some("Система"),
        ExtraLife => Some("+1 жизнь"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("СЕКТОР {1}"),
        SettingEffects => Some("Эффекты"),
        SettingContrast => Some("Контраст"),
        LookStandard => Some("Обычно"),
        EffectsReduced => Some("Меньше"),
        ContrastHigh => Some("Высокий"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "Первый свет",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Спутники",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Попутный поток",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Наплыв",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Резонанс",
    "Призма",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Течение",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Послесвечение",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Параллакс",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Сверхновая",
    "Восход луны",
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
    "Возвращение",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "Первый свет",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Спутники",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Поток",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Наплыв",
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
    "Свечение",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Параллакс",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Сверхновая",
    "Луна",
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
    "Возврат",
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
    "{icon:anchor} Якорь: поймайте мяч, прицельтесь и отпустите",
    "Armored bricks take two hits: watch the rims", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall", // awaiting translation
    "One door in: send the ball into the courtyard", // awaiting translation
    "Knock out the keystone and the arch is open",  // awaiting translation
    "Read the angles: each shadow is a shot",       // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart", // awaiting translation
    "Янтарные ядра: каждый взрыв задевает четырёх соседей",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Откройте путь через две линии реле",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Соседние ядра передают реакцию дальше",
    "{icon:multi} Мультимяч: три мяча, один проход",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Пробейтесь в карманы за бронёй",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Фаза: три касания кирпичей без отскока",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Пробейте оболочку, затем подожгите внутренний путь",
    "In by the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                      // awaiting translation
    "Three bells: ring each one from beneath",                     // awaiting translation
    "Поймайте мяч, пока остальные летят",
    "Следуйте за реле вокруг открытого центра",
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
    "Последняя орбита: пусть каждый проход считается",
];
