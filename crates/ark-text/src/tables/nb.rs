//! Norwegian Bokmål. Informal *du*; labels in capitals.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Knus kosmos",
        ContinueJourney => "Fortsett reisen",
        NewJourney => "Ny reise",
        SectorSelect => "Velg sektor",
        ContinueDetail => "{0} · {1}",
        StatMedals => "MEDALJER",
        StatBest => "REKORD",
        ActionServe => "Serve",
        ActionRelease => "Slipp",
        ActionSelect => "Velg",
        ActionResume => "Gjenoppta",
        ActionBack => "Tilbake",
        Fullscreen => "Fullskjerm",
        SectorsHeading => "Sektorer",
        PracticeNote => "Trening endrer aldri reisen din",
        MedalClear => "KLART",
        MedalClean => "FEILFRI",
        MedalSwift => "RASK",
        PlaySector => "Spill sektor {0}",
        UnlockHint => "Fullfør sektor {0} for å låse opp",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · SEKTOR {1}",
        Paused => "Pause",
        RetrySector => "Prøv sektoren igjen",
        MainMenu => "Hovedmeny",
        JourneyComplete => "Reisen er fullført",
        OneMoreOrbit => "Én bane til?",
        StatPoints => "POENG",
        SectorClear => "Sektor klart",
        StatTime => "TID",
        StatBonus => "BONUS",
        StatChain => "KJEDE",
        ExtraLife => "Kapittel fullført · +1 liv",
        NextSector => "Neste sektor",
        BackToSectors => "Tilbake til sektorer",
        SaveFailed => "Fremgangen kunne ikke lagres",
        PerfTitle => "Ytelse / CPU",
        HelpResume => "Fortsett der du slapp",
        HelpRetry => "Start på nytt fra sektorens sjekkpunkt",
        HelpMainMenu => "Reisen din er lagret",
        HelpSectors => "Tren i en åpen sektor",
        HelpNewJourney => "På nytt fra sektor 01 · medaljene beholdes",
        Settings => "Innstillinger",
        HelpSettings => "Lyd, skjerm, språk",
        SettingSound => "Lyd",
        SettingVolume => "Volum",
        SettingDisplay => "Skjerm",
        SettingLanguage => "Språk",
        DisplayWindow => "Vindu",
        LanguageSystem => "System",
        ActionAdjust => "Juster",
        SettingEffects => "Effekter",
        SettingContrast => "Kontrast",
        LookStandard => "Standard",
        EffectsReduced => "Redusert",
        ContrastHigh => "Høy",
        KeySpace => "Mellomrom",
        KeyEsc => "Esc",
        SectorsOf => "{0} av {1} sektorer",
        TargetBest => "{0} · rekord {1}",
        LifeGained => "+1 liv",
        PresenceMenus => "I menyene",
        PresenceJourney => "Sektor {0} · {1}",
        PresencePractice => "Trener sektor {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "DAGGRY",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "ETTERLYS",
            Chapter::BlueHour => "BLÅTIMEN",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "BRED",
            Power::Slow => "SAKTE",
            Power::Multi => "MULTIBALL",
            Power::Anchor => "ANKER",
            Power::Phase => "FASE",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Fortsett"),
        UnlockHint => Some("Låst"),
        RetrySector => Some("Prøv igjen"),
        MainMenu => Some("Meny"),
        NewJourney => Some("Ny"),
        SectorSelect => Some("Sektorer"),
        NextSector => Some("Neste"),
        BackToSectors => Some("Sektorer"),
        Settings => Some("Valg"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Fortsett"),
        PlaySector => Some("Spill {0}"),
        ActionServe => Some("Serve"),
        ActionRelease => Some("Slipp"),
        ActionSelect => Some("Velg"),
        ActionBack => Some("Tilbake"),
        ActionAdjust => Some("Juster"),
        SettingEffects => Some("Effekter"),
        SettingContrast => Some("Kontrast"),
        LookStandard => Some("Normal"),
        EffectsReduced => Some("Færre"),
        ContrastHigh => Some("Høy"),
        StatBest => Some("REKORD"),
        StatMedals => Some("MEDALJER"),
        SectorsHeading => Some("Sektorer"),
        SaveFailed => Some("Ikke lagret"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Mellomrom"),
        HelpResume => Some("Tilbake til spillet"),
        HelpRetry => Some("Fra sjekkpunktet"),
        HelpMainMenu => Some("Reisen er lagret"),
        HelpSectors => Some("Tren sektorer"),
        HelpNewJourney => Some("Fra sektor 01"),
        HelpSettings => Some("Lyd, skjerm"),
        SettingSound => Some("Lyd"),
        SettingVolume => Some("Volum"),
        SettingDisplay => Some("Skjerm"),
        SettingLanguage => Some("Språk"),
        DisplayWindow => Some("Vindu"),
        Fullscreen => Some("Full"),
        LanguageSystem => Some("System"),
        ExtraLife => Some("+1 liv"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("SEKTOR {1}"),
        JourneyComplete => Some("Fullført"),
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
    "Etterglød",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Parallakse",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Måneoppgang",
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
    "Etterglød",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Parallakse",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Måne",
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
    "{icon:anchor} Anker: fang ballen, sikt og slipp den",
    "Armored bricks take two hits; their rims count down", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall",       // awaiting translation
    "One door in: send the ball into the courtyard",       // awaiting translation
    "Knock out the keystone and the arch is open",         // awaiting translation
    "Read the angles: each shadow is a shot",              // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart",       // awaiting translation
    "Ravkjerner: hver eksplosjon når de fire naboene",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Åpne en vei gjennom de to relélinjene",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Nabokjerner fører reaksjonen videre",
    "{icon:multi} Multiball: tre baller, én åpning",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Bryt inn i lommene bak panseret",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Fase: tre klosstreff uten å sprette",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Gjennombryt skallet, og tenn så den indre ruten",
    "Through the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                        // awaiting translation
    "Three bells: ring each one from beneath",                       // awaiting translation
    "Fang en returball mens de andre fortsetter",
    "Følg reléet rundt den åpne midten",
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
    "En siste bane: få hver åpning til å telle",
];
