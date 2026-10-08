//! Romanian. Informal *tu*; ș and ț with comma below; labels in capitals.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Sparge cosmosul",
        ContinueJourney => "Continuă călătoria",
        NewJourney => "Călătorie nouă",
        SectorSelect => "Alege sectorul",
        ContinueDetail => "{0} · {1}",
        StatMedals => "MEDALII",
        StatBest => "RECORD",
        ActionServe => "Lansează",
        ActionRelease => "Eliberează",
        ActionSelect => "Alege",
        ActionResume => "Reia",
        ActionBack => "Înapoi",
        Fullscreen => "Ecran complet",
        SectorsHeading => "Sectoare",
        PracticeNote => "Antrenamentul nu îți schimbă niciodată călătoria",
        MedalClear => "TRECUT",
        MedalClean => "IMPECABIL",
        MedalSwift => "RAPID",
        PlaySector => "Joacă sectorul {0}",
        UnlockHint => "Termină sectorul {0} pentru deblocare",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · SECTORUL {1}",
        Paused => "Pauză",
        RetrySector => "Reia sectorul",
        MainMenu => "Meniu principal",
        JourneyComplete => "Călătorie încheiată",
        OneMoreOrbit => "Încă o orbită?",
        StatPoints => "PUNCTE",
        SectorClear => "Sector trecut",
        StatTime => "TIMP",
        StatBonus => "BONUS",
        StatChain => "LANȚ",
        ExtraLife => "Capitol încheiat · +1 viață",
        NextSector => "Sectorul următor",
        BackToSectors => "Înapoi la sectoare",
        SaveFailed => "Progresul nu a putut fi salvat",
        PerfTitle => "Performanță / CPU",
        HelpResume => "Continuă de unde ai rămas",
        HelpRetry => "Reia de la punctul de control al sectorului",
        HelpMainMenu => "Călătoria ta e salvată",
        HelpSectors => "Exersează în orice sector deschis",
        HelpNewJourney => "Din nou de la sectorul 01 · medaliile rămân",
        Settings => "Setări",
        HelpSettings => "Sunet, ecran, limbă",
        SettingSound => "Sunet",
        SettingVolume => "Volum",
        SettingDisplay => "Ecran",
        SettingLanguage => "Limbă",
        DisplayWindow => "Fereastră",
        LanguageSystem => "Sistem",
        ActionAdjust => "Ajustează",
        SettingEffects => "Efecte",
        SettingContrast => "Contrast",
        LookStandard => "Standard",
        EffectsReduced => "Reduse",
        ContrastHigh => "Ridicat",
        KeySpace => "Spațiu",
        KeyEsc => "Esc",
        SectorsOf => "{0} din {1} sectoare",
        TargetBest => "{0} · record {1}",
        LifeGained => "+1 viață",
        PresenceMenus => "În meniuri",
        PresenceJourney => "Sectorul {0} · {1}",
        PresencePractice => "Se antrenează în sectorul {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "ZORI",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "AMURG",
            Chapter::BlueHour => "ORA ALBASTRĂ",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "LAT",
            Power::Slow => "LENT",
            Power::Multi => "MULTIMINGE",
            Power::Anchor => "ANCORĂ",
            Power::Phase => "FAZĂ",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Continuă"),
        UnlockHint => Some("Blocat"),
        RetrySector => Some("Reia"),
        MainMenu => Some("Meniu"),
        NewJourney => Some("Nouă"),
        SectorSelect => Some("Sectoare"),
        NextSector => Some("Următorul"),
        BackToSectors => Some("Sectoare"),
        Settings => Some("Setări"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Reia"),
        PlaySector => Some("Joacă {0}"),
        ActionServe => Some("Lansează"),
        ActionRelease => Some("Lasă"),
        ActionSelect => Some("Alege"),
        ActionBack => Some("Înapoi"),
        ActionAdjust => Some("Ajustează"),
        SettingEffects => Some("Efecte"),
        SettingContrast => Some("Contrast"),
        LookStandard => Some("Normal"),
        EffectsReduced => Some("Puține"),
        ContrastHigh => Some("Ridicat"),
        StatBest => Some("RECORD"),
        StatMedals => Some("MEDALII"),
        SectorsHeading => Some("Sectoare"),
        SaveFailed => Some("Nesalvat"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Spațiu"),
        HelpResume => Some("Înapoi în joc"),
        HelpRetry => Some("De la punctul de control"),
        HelpMainMenu => Some("Călătorie salvată"),
        HelpSectors => Some("Exersează"),
        HelpNewJourney => Some("De la sectorul 01"),
        HelpSettings => Some("Sunet, ecran"),
        SettingSound => Some("Sunet"),
        SettingVolume => Some("Volum"),
        SettingDisplay => Some("Ecran"),
        SettingLanguage => Some("Limbă"),
        DisplayWindow => Some("Fereastră"),
        Fullscreen => Some("Complet"),
        LanguageSystem => Some("Sistem"),
        ExtraLife => Some("+1 viață"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("SECTORUL {1}"),
        JourneyComplete => Some("Încheiată"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "Prima lumină",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Sateliți",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Siaj",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Fondu",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Rezonanță",
    "Prismă",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Curent de fund",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Licăr de apus",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Paralaxă",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernovă",
    "Răsărit de lună",
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
    "Întoarcere acasă",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "Prima lumină",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Sateliți",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Siaj",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Fondu",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Rezonanță",
    "Prismă",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Curent",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Licăr",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Paralaxă",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernovă",
    "Lună",
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
    "Întoarcere",
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
    "{icon:anchor} Ancoră: prinde mingea, țintește, apoi elibereaz-o",
    "Armored bricks take two hits: watch the rims", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall", // awaiting translation
    "One door in: send the ball into the courtyard", // awaiting translation
    "Knock out the keystone and the arch is open",  // awaiting translation
    "Read the angles: each shadow is a shot",       // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart", // awaiting translation
    "Nuclee de chihlimbar: fiecare explozie atinge cei patru vecini",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Deschide un drum prin cele două linii de relee",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Nucleele vecine duc reacția mai departe",
    "{icon:multi} Multiminge: trei mingi, o singură deschidere",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Pătrunde în buzunarele din spatele blindajului",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Fază: trei atingeri de cărămizi fără ricoșeu",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Străpunge carcasa, apoi aprinde traseul interior",
    "In by the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                      // awaiting translation
    "Three bells: ring each one from beneath",                     // awaiting translation
    "Prinde o minge care revine în timp ce celelalte zboară",
    "Urmează releul în jurul centrului deschis",
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
    "O ultimă orbită: fă ca fiecare deschidere să conteze",
];
