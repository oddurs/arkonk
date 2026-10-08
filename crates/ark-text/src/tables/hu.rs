//! Hungarian. Informal *te*; hints read "action: keys". Ordinal sector
//! numbers take a period (`03. szektor`); labels in capitals.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Zúzd szét a kozmoszt",
        ContinueJourney => "Utazás folytatása",
        NewJourney => "Új utazás",
        SectorSelect => "Szektorválasztás",
        ContinueDetail => "{0} · {1}",
        StatMedals => "ÉRMEK",
        StatBest => "REKORD",
        ActionServe => "Indítás",
        ActionRelease => "Elengedés",
        ActionSelect => "Kiválasztás",
        ActionResume => "Folytatás",
        ActionBack => "Vissza",
        Fullscreen => "Teljes képernyő",
        SectorsHeading => "Szektorok",
        PracticeNote => "A gyakorlás sosem változtat az utazásodon",
        MedalClear => "TELJESÍTVE",
        MedalClean => "HIBÁTLAN",
        MedalSwift => "VILLÁM",
        PlaySector => "{0}. szektor indítása",
        UnlockHint => "Nyitás: teljesítsd a(z) {0}. szektort",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · {1}. SZEKTOR",
        Paused => "Szünet",
        RetrySector => "Szektor újra",
        MainMenu => "Főmenü",
        JourneyComplete => "Utazás teljesítve",
        OneMoreOrbit => "Még egy kör?",
        StatPoints => "PONTOK",
        SectorClear => "Szektor teljesítve",
        StatTime => "IDŐ",
        StatBonus => "BÓNUSZ",
        StatChain => "LÁNC",
        ExtraLife => "Fejezet teljesítve · +1 élet",
        NextSector => "Következő szektor",
        BackToSectors => "Vissza a szektorokhoz",
        SaveFailed => "Az előrehaladást nem sikerült menteni",
        PerfTitle => "Teljesítmény / CPU",
        HelpResume => "Folytasd ott, ahol abbahagytad",
        HelpRetry => "Újra a szektor ellenőrzőpontjától",
        HelpMainMenu => "Az utazásod mentve",
        HelpSectors => "Gyakorolj bármely nyitott szektorban",
        HelpNewJourney => "Újra a 01. szektortól · az érmek maradnak",
        Settings => "Beállítások",
        HelpSettings => "Hang, kijelző, nyelv",
        SettingSound => "Hang",
        SettingVolume => "Hangerő",
        SettingDisplay => "Kijelző",
        SettingLanguage => "Nyelv",
        DisplayWindow => "Ablak",
        LanguageSystem => "Rendszer",
        ActionAdjust => "Állítás",
        SettingEffects => "Effektek",
        SettingContrast => "Kontraszt",
        LookStandard => "Normál",
        EffectsReduced => "Csökkentett",
        ContrastHigh => "Magas",
        KeySpace => "Szóköz",
        KeyEsc => "Esc",
        SectorsOf => "{0}/{1} szektor",
        TargetBest => "{0} · rekord {1}",
        LifeGained => "+1 élet",
        PresenceMenus => "A menükben",
        PresenceJourney => "{0}. szektor · {1}",
        PresencePractice => "Gyakorol: {0}. szektor · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "HAJNAL",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "ALKONY",
            Chapter::BlueHour => "KÉK ÓRA",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "SZÉLES",
            Power::Slow => "LASSÚ",
            Power::Multi => "TÖBBLABDA",
            Power::Anchor => "HORGONY",
            Power::Phase => "FÁZIS",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Folytatás"),
        UnlockHint => Some("Zárolva"),
        RetrySector => Some("Újra"),
        MainMenu => Some("Menü"),
        NewJourney => Some("Új"),
        SectorSelect => Some("Szektorok"),
        NextSector => Some("Tovább"),
        BackToSectors => Some("Szektorok"),
        Settings => Some("Opciók"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Folytatás"),
        PlaySector => Some("{0}. szektor"),
        ActionServe => Some("Indítás"),
        ActionRelease => Some("Elenged"),
        ActionSelect => Some("Választ"),
        ActionBack => Some("Vissza"),
        ActionAdjust => Some("Állítás"),
        SettingEffects => Some("Effektek"),
        SettingContrast => Some("Kontraszt"),
        LookStandard => Some("Normál"),
        EffectsReduced => Some("Kevesebb"),
        ContrastHigh => Some("Magas"),
        StatBest => Some("REKORD"),
        StatMedals => Some("ÉRMEK"),
        SectorsHeading => Some("Szektorok"),
        SaveFailed => Some("Nincs mentve"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Szóköz"),
        HelpResume => Some("Vissza a játékba"),
        HelpRetry => Some("Ellenőrzőponttól"),
        HelpMainMenu => Some("Utazás mentve"),
        HelpSectors => Some("Gyakorlás"),
        HelpNewJourney => Some("A 01. szektortól"),
        HelpSettings => Some("Hang, kijelző"),
        SettingSound => Some("Hang"),
        SettingVolume => Some("Hangerő"),
        SettingDisplay => Some("Kijelző"),
        SettingLanguage => Some("Nyelv"),
        DisplayWindow => Some("Ablak"),
        Fullscreen => Some("Teljes"),
        LanguageSystem => Some("Rendszer"),
        ExtraLife => Some("+1 élet"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("{1}. SZEKTOR"),
        JourneyComplete => Some("Teljesítve"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "Első fény",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Műholdak",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Szélárnyék",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Áttűnés",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Rezonancia",
    "Prizma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Örvény",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Utófény",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Parallaxis",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Szupernóva",
    "Holdkelte",
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
    "Hazatérés",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "Első fény",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Műholdak",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Szélárnyék",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Áttűnés",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Rezonancia",
    "Prizma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Örvény",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Utófény",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Parallaxis",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Szupernóva",
    "Holdkelte",
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
    "Hazatérés",
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
    "{icon:anchor} Horgony: kapd el a labdát, célozz, majd engedd el",
    "Armored bricks take two hits; their rims count down", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall",       // awaiting translation
    "One door in: send the ball into the courtyard",       // awaiting translation
    "Knock out the keystone and the arch is open",         // awaiting translation
    "Read the angles: each shadow is a shot",              // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart",       // awaiting translation
    "Borostyánmagok: minden robbanás eléri a négy szomszédot",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Nyiss utat a két reléláncon át",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "A szomszédos magok továbbviszik a reakciót",
    "{icon:multi} Többlabda: három labda, egy rés",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Törj be a páncél mögötti zsebekbe",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Fázis: három téglaérintés visszapattanás nélkül",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Törd át a héjat, aztán gyújtsd be a belső utat",
    "Through the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                        // awaiting translation
    "Three bells: ring each one from beneath",                       // awaiting translation
    "Kapj el egy visszatérő labdát, amíg a többi repül",
    "Kövesd a relét a nyitott közép körül",
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
    "Egy utolsó kör: használj ki minden rést",
];
