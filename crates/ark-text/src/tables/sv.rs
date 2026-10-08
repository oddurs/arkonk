//! Swedish. Informal *du*; labels in capitals.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Krossa kosmos",
        ContinueJourney => "Fortsätt resan",
        NewJourney => "Ny resa",
        SectorSelect => "Välj sektor",
        ContinueDetail => "{0} · {1}",
        StatMedals => "MEDALJER",
        StatBest => "REKORD",
        ActionServe => "Serva",
        ActionRelease => "Släpp",
        ActionSelect => "Välj",
        ActionResume => "Återuppta",
        ActionBack => "Tillbaka",
        Fullscreen => "Helskärm",
        SectorsHeading => "Sektorer",
        PracticeNote => "Träning ändrar aldrig din resa",
        MedalClear => "KLARAD",
        MedalClean => "FELFRI",
        MedalSwift => "SNABB",
        PlaySector => "Spela sektor {0}",
        UnlockHint => "Klara sektor {0} för att låsa upp",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · SEKTOR {1}",
        Paused => "Pausat",
        RetrySector => "Försök sektorn igen",
        MainMenu => "Huvudmeny",
        JourneyComplete => "Resan är klar",
        OneMoreOrbit => "En bana till?",
        StatPoints => "POÄNG",
        SectorClear => "Sektorn klarad",
        StatTime => "TID",
        StatBonus => "BONUS",
        StatChain => "KEDJA",
        ExtraLife => "Kapitlet klart · +1 liv",
        NextSector => "Nästa sektor",
        BackToSectors => "Tillbaka till sektorer",
        SaveFailed => "Framstegen kunde inte sparas",
        PerfTitle => "Prestanda / CPU",
        HelpResume => "Fortsätt där du slutade",
        HelpRetry => "Börja om från sektorns kontrollpunkt",
        HelpMainMenu => "Din resa är sparad",
        HelpSectors => "Träna på valfri öppen sektor",
        HelpNewJourney => "Om från sektor 01 · medaljerna finns kvar",
        Settings => "Inställningar",
        HelpSettings => "Ljud, skärm, språk",
        SettingSound => "Ljud",
        SettingVolume => "Volym",
        SettingDisplay => "Skärm",
        SettingLanguage => "Språk",
        DisplayWindow => "Fönster",
        LanguageSystem => "System",
        ActionAdjust => "Justera",
        SettingEffects => "Effekter",
        SettingContrast => "Kontrast",
        LookStandard => "Standard",
        EffectsReduced => "Minskade",
        ContrastHigh => "Hög",
        KeySpace => "Mellanslag",
        KeyEsc => "Esc",
        SectorsOf => "{0} av {1} sektorer",
        TargetBest => "{0} · bäst {1}",
        LifeGained => "+1 liv",
        PresenceMenus => "I menyerna",
        PresenceJourney => "Sektor {0} · {1}",
        PresencePractice => "Tränar sektor {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "GRYNING",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "SKYMNING",
            Chapter::BlueHour => "BLÅ TIMMEN",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "BRED",
            Power::Slow => "LÅNGSAM",
            Power::Multi => "MULTIBOLL",
            Power::Anchor => "ANKARE",
            Power::Phase => "FAS",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Fortsätt"),
        UnlockHint => Some("Låst"),
        RetrySector => Some("Försök igen"),
        MainMenu => Some("Meny"),
        NewJourney => Some("Ny"),
        SectorSelect => Some("Sektorer"),
        NextSector => Some("Nästa"),
        BackToSectors => Some("Sektorer"),
        Settings => Some("Val"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Fortsätt"),
        PlaySector => Some("Spela {0}"),
        ActionServe => Some("Serva"),
        ActionRelease => Some("Släpp"),
        ActionSelect => Some("Välj"),
        ActionBack => Some("Tillbaka"),
        ActionAdjust => Some("Justera"),
        SettingEffects => Some("Effekter"),
        SettingContrast => Some("Kontrast"),
        LookStandard => Some("Normal"),
        EffectsReduced => Some("Färre"),
        ContrastHigh => Some("Hög"),
        StatBest => Some("REKORD"),
        StatMedals => Some("MEDALJER"),
        SectorsHeading => Some("Sektorer"),
        SaveFailed => Some("Inte sparat"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Mellanslag"),
        HelpResume => Some("Tillbaka till spelet"),
        HelpRetry => Some("Från kontrollpunkten"),
        HelpMainMenu => Some("Resan är sparad"),
        HelpSectors => Some("Träna sektorer"),
        HelpNewJourney => Some("Från sektor 01"),
        HelpSettings => Some("Ljud, skärm"),
        SettingSound => Some("Ljud"),
        SettingVolume => Some("Volym"),
        SettingDisplay => Some("Skärm"),
        SettingLanguage => Some("Språk"),
        DisplayWindow => Some("Fönster"),
        Fullscreen => Some("Hel"),
        LanguageSystem => Some("System"),
        ExtraLife => Some("+1 liv"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("SEKTOR {1}"),
        JourneyComplete => Some("Klar"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "Första ljuset",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satelliter",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Slipström",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Övertoning",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Resonans",
    "Prisma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Underström",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Efterglöd",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Parallax",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Månuppgång",
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
    "Hemkomst",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "Första ljuset",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satelliter",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Slipström",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Övertoning",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Resonans",
    "Prisma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Underström",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Efterglöd",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Parallax",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Månuppgång",
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
    "Hemkomst",
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
    "{icon:anchor} Ankare: fånga bollen, sikta och släpp den",
    "Armored bricks take two hits: watch the rims", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall", // awaiting translation
    "One door in: send the ball into the courtyard", // awaiting translation
    "Knock out the keystone and the arch is open",  // awaiting translation
    "Read the angles: each shadow is a shot",       // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart", // awaiting translation
    "Bärnstenskärnor: varje explosion når fyra grannar",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Öppna en väg genom de två relälinjerna",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Närliggande kärnor för reaktionen vidare",
    "{icon:multi} Multiboll: tre bollar, en öppning",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Bryt in i fickorna bakom pansaret",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Fas: tre klosskontakter utan studs",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Genomborra skalet och tänd sedan den inre rutten",
    "In by the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                      // awaiting translation
    "Three bells: ring each one from beneath",                     // awaiting translation
    "Fånga en returboll medan de andra fortsätter",
    "Följ reläet runt den öppna mitten",
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
    "En sista bana: låt varje öppning räknas",
];
