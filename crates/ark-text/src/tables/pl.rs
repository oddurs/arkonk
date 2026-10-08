//! Polish. Hints put the action first ("Ruch: mysz lub strzałki"), which
//! keeps them short without an infinitive clause; labels in capitals.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Rozbij kosmos",
        ContinueJourney => "Kontynuuj podróż",
        NewJourney => "Nowa podróż",
        SectorSelect => "Wybór sektora",
        ContinueDetail => "{0} · {1}",
        StatMedals => "MEDALE",
        StatBest => "REKORD",
        ActionServe => "Start",
        ActionRelease => "Zwolnij",
        ActionSelect => "Wybierz",
        ActionResume => "Wznów",
        ActionBack => "Wstecz",
        Fullscreen => "Pełny ekran",
        SectorsHeading => "Sektory",
        PracticeNote => "Trening nigdy nie zmienia twojej podróży",
        MedalClear => "UKOŃCZONO",
        MedalClean => "BEZ STRAT",
        MedalSwift => "SZYBKO",
        PlaySector => "Graj: sektor {0}",
        UnlockHint => "Ukończ sektor {0}, aby odblokować",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · SEKTOR {1}",
        Paused => "Pauza",
        RetrySector => "Powtórz sektor",
        MainMenu => "Menu główne",
        JourneyComplete => "Podróż ukończona",
        OneMoreOrbit => "Jeszcze jedna orbita?",
        StatPoints => "PUNKTY",
        SectorClear => "Sektor ukończony",
        StatTime => "CZAS",
        StatBonus => "PREMIA",
        StatChain => "SERIA",
        ExtraLife => "Rozdział ukończony · +1 życie",
        NextSector => "Następny sektor",
        BackToSectors => "Powrót do sektorów",
        SaveFailed => "Nie udało się zapisać postępów",
        PerfTitle => "Wydajność / CPU",
        HelpResume => "Graj dalej od tego miejsca",
        HelpRetry => "Zacznij od punktu kontrolnego sektora",
        HelpMainMenu => "Twoja podróż jest zapisana",
        HelpSectors => "Ćwicz w dowolnym otwartym sektorze",
        HelpNewJourney => "Zacznij od sektora 01 · medale zostają",
        Settings => "Ustawienia",
        HelpSettings => "Dźwięk, ekran, język",
        SettingSound => "Dźwięk",
        SettingVolume => "Głośność",
        SettingDisplay => "Ekran",
        SettingLanguage => "Język",
        DisplayWindow => "Okno",
        LanguageSystem => "Systemowy",
        ActionAdjust => "Zmień",
        SettingEffects => "Efekty",
        SettingContrast => "Kontrast",
        LookStandard => "Standard",
        EffectsReduced => "Ograniczone",
        ContrastHigh => "Wysoki",
        KeySpace => "Spacja",
        KeyEsc => "Esc",
        SectorsOf => "{0} z {1} sektorów",
        TargetBest => "{0} · rekord {1}",
        LifeGained => "+1 życie",
        PresenceMenus => "W menu",
        PresenceJourney => "Sektor {0} · {1}",
        PresencePractice => "Trening: sektor {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "ŚWIT",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "ZMIERZCH",
            Chapter::BlueHour => "NIEBIESKA GODZINA",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "POSZERZENIE",
            Power::Slow => "SPOWOLNIENIE",
            Power::Multi => "MULTIPIŁKA",
            Power::Anchor => "KOTWICA",
            Power::Phase => "FAZA",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Kontynuuj"),
        UnlockHint => Some("Zablokowany"),
        RetrySector => Some("Powtórz"),
        MainMenu => Some("Menu"),
        NewJourney => Some("Nowa"),
        SectorSelect => Some("Sektory"),
        NextSector => Some("Dalej"),
        BackToSectors => Some("Sektory"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Wznów"),
        PlaySector => Some("Graj {0}"),
        Settings => Some("Opcje"),
        ActionServe => Some("Start"),
        ActionRelease => Some("Zwolnij"),
        ActionSelect => Some("Wybierz"),
        ActionBack => Some("Wstecz"),
        ActionAdjust => Some("Zmień"),
        StatBest => Some("REKORD"),
        StatMedals => Some("MEDALE"),
        SectorsHeading => Some("Sektory"),
        SaveFailed => Some("Nie zapisano"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Spacja"),
        HelpResume => Some("Wróć do gry"),
        HelpRetry => Some("Od punktu kontrolnego"),
        HelpMainMenu => Some("Podróż zapisana"),
        HelpSectors => Some("Ćwicz sektory"),
        HelpNewJourney => Some("Od sektora 01"),
        HelpSettings => Some("Dźwięk, ekran"),
        SettingSound => Some("Dźwięk"),
        SettingVolume => Some("Głośność"),
        SettingDisplay => Some("Ekran"),
        SettingLanguage => Some("Język"),
        DisplayWindow => Some("Okno"),
        Fullscreen => Some("Pełny"),
        LanguageSystem => Some("System"),
        ExtraLife => Some("+1 życie"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("SEKTOR {1}"),
        SettingEffects => Some("Efekty"),
        SettingContrast => Some("Kontrast"),
        LookStandard => Some("Norma"),
        EffectsReduced => Some("Mniej"),
        ContrastHigh => Some("Wysoki"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "Pierwsze światło",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satelity",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Strumień",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Przenikanie",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Rezonans",
    "Pryzmat",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Prąd wsteczny",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Poświata",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Paralaksa",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernowa",
    "Wschód księżyca",
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
    "Powrót",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "Pierwsze światło",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satelity",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Strumień",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Przenikanie",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Rezonans",
    "Pryzmat",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Prąd",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Poświata",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Paralaksa",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernowa",
    "Księżyc",
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
    "Powrót",
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
    "{icon:anchor} Kotwica: złap piłkę, wyceluj i ją wypuść",
    "Armored bricks take two hits: watch the rims", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall", // awaiting translation
    "One door in: send the ball into the courtyard", // awaiting translation
    "Knock out the keystone and the arch is open",  // awaiting translation
    "Read the angles: each shadow is a shot",       // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart", // awaiting translation
    "Bursztynowe rdzenie: każdy wybuch sięga czterech sąsiadów",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Otwórz drogę przez dwie linie przekaźników",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Sąsiednie rdzenie przenoszą reakcję",
    "{icon:multi} Multipiłka: trzy piłki, jedna luka",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Przebij się do kieszeni za pancerzem",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Faza: trzy trafienia w cegły bez odbicia",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Przebij pancerz, potem odpal wewnętrzną trasę",
    "In by the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                      // awaiting translation
    "Three bells: ring each one from beneath",                     // awaiting translation
    "Złap piłkę, gdy pozostałe wciąż lecą",
    "Podążaj za przekaźnikiem wokół pustego środka",
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
    "Ostatnia orbita: niech każda luka się liczy",
];
