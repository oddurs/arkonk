//! Danish. Informal *du*; labels in capitals.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Knus kosmos",
        ContinueJourney => "Fortsæt rejsen",
        NewJourney => "Ny rejse",
        SectorSelect => "Vælg sektor",
        ContinueDetail => "{0} · {1}",
        StatMedals => "MEDALJER",
        StatBest => "REKORD",
        ActionServe => "Serv",
        ActionRelease => "Slip",
        ActionSelect => "Vælg",
        ActionResume => "Genoptag",
        ActionBack => "Tilbage",
        Fullscreen => "Fuld skærm",
        SectorsHeading => "Sektorer",
        PracticeNote => "Træning ændrer aldrig din rejse",
        MedalClear => "KLARET",
        MedalClean => "FEJLFRI",
        MedalSwift => "HURTIG",
        PlaySector => "Spil sektor {0}",
        UnlockHint => "Gennemfør sektor {0} for at låse op",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · SEKTOR {1}",
        Paused => "Pause",
        RetrySector => "Prøv sektoren igen",
        MainMenu => "Hovedmenu",
        JourneyComplete => "Rejsen er fuldført",
        OneMoreOrbit => "Én bane til?",
        StatPoints => "POINT",
        SectorClear => "Sektor klaret",
        StatTime => "TID",
        StatBonus => "BONUS",
        StatChain => "KÆDE",
        ExtraLife => "Kapitel fuldført · +1 liv",
        NextSector => "Næste sektor",
        BackToSectors => "Tilbage til sektorer",
        SaveFailed => "Fremskridt kunne ikke gemmes",
        PerfTitle => "Ydeevne / CPU",
        HelpResume => "Fortsæt, hvor du slap",
        HelpRetry => "Start forfra fra sektorens checkpoint",
        HelpMainMenu => "Din rejse er gemt",
        HelpSectors => "Træn i en åben sektor",
        HelpNewJourney => "Forfra fra sektor 01 · medaljer bevares",
        Settings => "Indstillinger",
        HelpSettings => "Lyd, skærm, sprog",
        SettingSound => "Lyd",
        SettingVolume => "Lydstyrke",
        SettingDisplay => "Skærm",
        SettingLanguage => "Sprog",
        DisplayWindow => "Vindue",
        LanguageSystem => "System",
        ActionAdjust => "Justér",
        SettingEffects => "Effekter",
        SettingContrast => "Kontrast",
        LookStandard => "Standard",
        EffectsReduced => "Reduceret",
        ContrastHigh => "Høj",
        KeySpace => "Mellemrum",
        KeyEsc => "Esc",
        SectorsOf => "{0} af {1} sektorer",
        TargetBest => "{0} · rekord {1}",
        LifeGained => "+1 liv",
        PresenceMenus => "I menuerne",
        PresenceJourney => "Sektor {0} · {1}",
        PresencePractice => "Træner sektor {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "DAGGRY",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "EFTERLYS",
            Chapter::BlueHour => "BLÅ TIME",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "BRED",
            Power::Slow => "LANGSOM",
            Power::Multi => "MULTIBOLD",
            Power::Anchor => "ANKER",
            Power::Phase => "FASE",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Fortsæt"),
        UnlockHint => Some("Låst"),
        RetrySector => Some("Prøv igen"),
        MainMenu => Some("Menu"),
        NewJourney => Some("Ny"),
        SectorSelect => Some("Sektorer"),
        NextSector => Some("Næste"),
        BackToSectors => Some("Sektorer"),
        Settings => Some("Valg"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Fortsæt"),
        PlaySector => Some("Spil {0}"),
        ActionServe => Some("Serv"),
        ActionRelease => Some("Slip"),
        ActionSelect => Some("Vælg"),
        ActionBack => Some("Tilbage"),
        ActionAdjust => Some("Justér"),
        SettingEffects => Some("Effekter"),
        SettingContrast => Some("Kontrast"),
        LookStandard => Some("Normal"),
        EffectsReduced => Some("Færre"),
        ContrastHigh => Some("Høj"),
        StatBest => Some("REKORD"),
        StatMedals => Some("MEDALJER"),
        SectorsHeading => Some("Sektorer"),
        SaveFailed => Some("Ikke gemt"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Mellemrum"),
        HelpResume => Some("Tilbage til spillet"),
        HelpRetry => Some("Fra checkpointet"),
        HelpMainMenu => Some("Rejsen er gemt"),
        HelpSectors => Some("Træn sektorer"),
        HelpNewJourney => Some("Fra sektor 01"),
        HelpSettings => Some("Lyd, skærm"),
        SettingSound => Some("Lyd"),
        SettingVolume => Some("Lydstyrke"),
        SettingDisplay => Some("Skærm"),
        SettingLanguage => Some("Sprog"),
        DisplayWindow => Some("Vindue"),
        Fullscreen => Some("Fuld"),
        LanguageSystem => Some("System"),
        ExtraLife => Some("+1 liv"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("SEKTOR {1}"),
        JourneyComplete => Some("Fuldført"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "Første lys",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satellitter",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Slipstrøm",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Overtoning",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Resonans",
    "Prisme",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Understrøm",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Efterglød",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Parallakse",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Måneopgang",
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
    "Hjemkomst",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "Første lys",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satellitter",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Slipstrøm",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Overtoning",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Resonans",
    "Prisme",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Understrøm",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Efterglød",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Parallakse",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Måne",
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
    "Hjemkomst",
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
    "{icon:anchor} Anker: grib bolden, sigt, og slip den",
    "Armored bricks take two hits: watch the rims", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall", // awaiting translation
    "One door in: send the ball into the courtyard", // awaiting translation
    "Knock out the keystone and the arch is open",  // awaiting translation
    "Read the angles: each shadow is a shot",       // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart", // awaiting translation
    "Ravkerner: hver eksplosion rammer de fire naboer",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Åbn en vej gennem de to relælinjer",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Nabokerner fører reaktionen videre",
    "{icon:multi} Multibold: tre bolde, én åbning",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Bryd ind i lommerne bag panseret",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Fase: tre klodsramninger uden at prelle af",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Gennembryd skallen, og antænd så den indre rute",
    "In by the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                      // awaiting translation
    "Three bells: ring each one from beneath",                     // awaiting translation
    "Grib en returbold, mens de andre fortsætter",
    "Følg relæet rundt om den åbne midte",
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
    "En sidste bane: få hver åbning til at tælle",
];
