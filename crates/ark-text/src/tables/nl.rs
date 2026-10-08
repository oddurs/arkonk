//! Dutch. Informal *je*; labels in capitals.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Breek de kosmos",
        ContinueJourney => "Reis voortzetten",
        NewJourney => "Nieuwe reis",
        SectorSelect => "Sector kiezen",
        ContinueDetail => "{0} · {1}",
        StatMedals => "MEDAILLES",
        StatBest => "RECORD",
        ActionServe => "Serveren",
        ActionRelease => "Loslaten",
        ActionSelect => "Kiezen",
        ActionResume => "Hervatten",
        ActionBack => "Terug",
        Fullscreen => "Volledig scherm",
        SectorsHeading => "Sectoren",
        PracticeNote => "Oefenrondes veranderen je reis nooit",
        MedalClear => "GEHAALD",
        MedalClean => "FOUTLOOS",
        MedalSwift => "SNEL",
        PlaySector => "Sector {0} spelen",
        UnlockHint => "Voltooi sector {0} om te ontgrendelen",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · SECTOR {1}",
        Paused => "Gepauzeerd",
        RetrySector => "Sector opnieuw",
        MainMenu => "Hoofdmenu",
        JourneyComplete => "Reis voltooid",
        OneMoreOrbit => "Nog één baan?",
        StatPoints => "PUNTEN",
        SectorClear => "Sector gehaald",
        StatTime => "TIJD",
        StatBonus => "BONUS",
        StatChain => "KETTING",
        ExtraLife => "Hoofdstuk voltooid · +1 leven",
        NextSector => "Volgende sector",
        BackToSectors => "Terug naar sectoren",
        SaveFailed => "Voortgang kon niet worden opgeslagen",
        PerfTitle => "Prestaties / CPU",
        HelpResume => "Ga verder waar je was",
        HelpRetry => "Opnieuw vanaf het checkpoint van de sector",
        HelpMainMenu => "Je reis is opgeslagen",
        HelpSectors => "Oefen in elke open sector",
        HelpNewJourney => "Opnieuw vanaf sector 01 · medailles blijven",
        Settings => "Instellingen",
        HelpSettings => "Geluid, scherm, taal",
        SettingSound => "Geluid",
        SettingVolume => "Volume",
        SettingDisplay => "Scherm",
        SettingLanguage => "Taal",
        DisplayWindow => "Venster",
        LanguageSystem => "Systeem",
        ActionAdjust => "Aanpassen",
        SettingEffects => "Effecten",
        SettingContrast => "Contrast",
        LookStandard => "Standaard",
        EffectsReduced => "Beperkt",
        ContrastHigh => "Hoog",
        KeySpace => "Spatie",
        KeyEsc => "Esc",
        SectorsOf => "{0} van {1} sectoren",
        TargetBest => "{0} · record {1}",
        LifeGained => "+1 leven",
        PresenceMenus => "In de menu’s",
        PresenceJourney => "Sector {0} · {1}",
        PresencePractice => "Oefent sector {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "DAGERAAD",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "SCHEMERING",
            Chapter::BlueHour => "BLAUWE UUR",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "BREED",
            Power::Slow => "TRAAG",
            Power::Multi => "MULTIBAL",
            Power::Anchor => "ANKER",
            Power::Phase => "FASE",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Verder"),
        UnlockHint => Some("Op slot"),
        RetrySector => Some("Opnieuw"),
        MainMenu => Some("Menu"),
        NewJourney => Some("Nieuw"),
        SectorSelect => Some("Sectoren"),
        NextSector => Some("Volgende"),
        BackToSectors => Some("Sectoren"),
        Settings => Some("Opties"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Verder"),
        PlaySector => Some("Speel {0}"),
        ActionServe => Some("Serveren"),
        ActionRelease => Some("Los"),
        ActionSelect => Some("Kiezen"),
        ActionBack => Some("Terug"),
        ActionAdjust => Some("Wijzig"),
        SettingEffects => Some("Effecten"),
        SettingContrast => Some("Contrast"),
        LookStandard => Some("Normaal"),
        EffectsReduced => Some("Minder"),
        ContrastHigh => Some("Hoog"),
        StatBest => Some("RECORD"),
        StatMedals => Some("MEDAILLES"),
        SectorsHeading => Some("Sectoren"),
        SaveFailed => Some("Niet opgeslagen"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Spatie"),
        HelpResume => Some("Terug naar het spel"),
        HelpRetry => Some("Vanaf het checkpoint"),
        HelpMainMenu => Some("Reis opgeslagen"),
        HelpSectors => Some("Oefen sectoren"),
        HelpNewJourney => Some("Vanaf sector 01"),
        HelpSettings => Some("Geluid, scherm"),
        SettingSound => Some("Geluid"),
        SettingVolume => Some("Volume"),
        SettingDisplay => Some("Scherm"),
        SettingLanguage => Some("Taal"),
        DisplayWindow => Some("Venster"),
        Fullscreen => Some("Volledig"),
        LanguageSystem => Some("Systeem"),
        ExtraLife => Some("+1 leven"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("SECTOR {1}"),
        JourneyComplete => Some("Voltooid"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "Eerste licht",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satellieten",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Zuigstroom",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Overvloeier",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Resonantie",
    "Prisma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Onderstroom",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Avondgloed",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Parallax",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Maansopkomst",
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
    "Thuiskomst",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "Eerste licht",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satellieten",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Zuigstroom",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Overvloeier",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Resonantie",
    "Prisma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Onderstroom",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Avondgloed",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Parallax",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Maan",
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
    "Thuiskomst",
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
    "{icon:anchor} Anker: vang de bal, richt en laat hem los",
    "Armored bricks take two hits; their rims count down", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall",       // awaiting translation
    "One door in: send the ball into the courtyard",       // awaiting translation
    "Knock out the keystone and the arch is open",         // awaiting translation
    "Read the angles: each shadow is a shot",              // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart",       // awaiting translation
    "Amberkernen: elke explosie raakt de vier buren",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Open een route door de twee relaislijnen",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Naburige kernen dragen de reactie verder",
    "{icon:multi} Multibal: drie ballen, één opening",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Breek door naar de holtes achter het pantser",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Fase: drie steentreffers zonder te stuiteren",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Doorboor de schil en ontsteek dan de binnenste route",
    "Through the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                        // awaiting translation
    "Three bells: ring each one from beneath",                       // awaiting translation
    "Vang een terugkerende bal terwijl de andere doorgaan",
    "Volg het relais rond het open midden",
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
    "Een laatste baan: laat elke opening tellen",
];
