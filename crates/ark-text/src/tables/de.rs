//! German. Informal *du*, as games address players. Labels in capitals:
//! a label never contains ß, which has no capital form here; the table
//! tests check that every label is already upper case.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Zerbrich den Kosmos",
        ContinueJourney => "Reise fortsetzen",
        NewJourney => "Neue Reise",
        SectorSelect => "Sektorauswahl",
        ContinueDetail => "{0} · {1}",
        StatMedals => "MEDAILLEN",
        StatBest => "BESTWERT",
        ActionServe => "Starten",
        ActionRelease => "Lösen",
        ActionSelect => "Auswählen",
        ActionResume => "Fortsetzen",
        ActionBack => "Zurück",
        Fullscreen => "Vollbild",
        SectorsHeading => "Sektoren",
        PracticeNote => "Üben verändert deine Reise nie",
        MedalClear => "GESCHAFFT",
        MedalClean => "MAKELLOS",
        MedalSwift => "FLINK",
        PlaySector => "Sektor {0} spielen",
        UnlockHint => "Zum Freischalten Sektor {0} abschließen",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · SEKTOR {1}",
        Paused => "Pausiert",
        RetrySector => "Sektor wiederholen",
        MainMenu => "Hauptmenü",
        JourneyComplete => "Reise abgeschlossen",
        OneMoreOrbit => "Noch eine Runde?",
        StatPoints => "PUNKTE",
        SectorClear => "Sektor geschafft",
        StatTime => "ZEIT",
        StatBonus => "BONUS",
        StatChain => "KETTE",
        ExtraLife => "Kapitel abgeschlossen · +1 Leben",
        NextSector => "Nächster Sektor",
        BackToSectors => "Zurück zu den Sektoren",
        SaveFailed => "Fortschritt konnte nicht gespeichert werden",
        PerfTitle => "Leistung / CPU",
        HelpResume => "Mach da weiter, wo du warst",
        HelpRetry => "Ab dem Kontrollpunkt des Sektors neu starten",
        HelpMainMenu => "Deine Reise ist gespeichert",
        HelpSectors => "Übe jeden offenen Sektor",
        HelpNewJourney => "Neu ab Sektor 01 · Medaillen bleiben",
        Settings => "Einstellungen",
        HelpSettings => "Ton, Anzeige, Sprache",
        SettingSound => "Ton",
        SettingVolume => "Lautstärke",
        SettingDisplay => "Anzeige",
        SettingLanguage => "Sprache",
        DisplayWindow => "Fenster",
        LanguageSystem => "System",
        ActionAdjust => "Anpassen",
        SettingEffects => "Effekte",
        SettingContrast => "Kontrast",
        LookStandard => "Standard",
        EffectsReduced => "Reduziert",
        ContrastHigh => "Hoch",
        KeySpace => "Leertaste",
        KeyEsc => "Esc",
        SectorsOf => "{0} von {1} Sektoren",
        TargetBest => "{0} · Bestzeit {1}",
        LifeGained => "+1 Leben",
        PresenceMenus => "Im Menü",
        PresenceJourney => "Sektor {0} · {1}",
        PresencePractice => "Übt Sektor {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "MORGENGRAUEN",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "ABENDGLUT",
            Chapter::BlueHour => "BLAUE STUNDE",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "BREIT",
            Power::Slow => "LANGSAM",
            Power::Multi => "MULTIBALL",
            Power::Anchor => "ANKER",
            Power::Phase => "PHASE",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Weiter"),
        UnlockHint => Some("Gesperrt"),
        RetrySector => Some("Wiederholen"),
        MainMenu => Some("Menü"),
        NewJourney => Some("Neu"),
        SectorSelect => Some("Sektoren"),
        NextSector => Some("Weiter"),
        BackToSectors => Some("Sektoren"),
        Settings => Some("Optionen"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Weiter"),
        PlaySector => Some("Sektor {0}"),
        ActionServe => Some("Start"),
        ActionRelease => Some("Lösen"),
        ActionSelect => Some("Wählen"),
        ActionBack => Some("Zurück"),
        ActionAdjust => Some("Ändern"),
        StatBest => Some("BESTWERT"),
        StatMedals => Some("MEDAILLEN"),
        SectorsHeading => Some("Sektoren"),
        SaveFailed => Some("Nicht gespeichert"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Leer"),
        HelpResume => Some("Zurück ins Spiel"),
        HelpRetry => Some("Ab dem Kontrollpunkt"),
        HelpMainMenu => Some("Reise gespeichert"),
        HelpSectors => Some("Sektoren üben"),
        HelpNewJourney => Some("Ab Sektor 01"),
        HelpSettings => Some("Ton, Anzeige"),
        SettingSound => Some("Ton"),
        SettingVolume => Some("Lautstärke"),
        SettingDisplay => Some("Anzeige"),
        SettingLanguage => Some("Sprache"),
        DisplayWindow => Some("Fenster"),
        Fullscreen => Some("Voll"),
        LanguageSystem => Some("System"),
        ExtraLife => Some("+1 Leben"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("SEKTOR {1}"),
        SettingEffects => Some("Effekte"),
        SettingContrast => Some("Kontrast"),
        LookStandard => Some("Normal"),
        EffectsReduced => Some("Wenig"),
        ContrastHigh => Some("Hoch"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "Erstes Licht",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satelliten",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Windschatten",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Überblendung",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Resonanz",
    "Prisma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Sog",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Nachglühen",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Parallaxe",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Mondaufgang",
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
    "Heimkehr",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "Erstes Licht",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satelliten",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Windschatten",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Blende",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Resonanz",
    "Prisma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Sog",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Nachglühen",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Parallaxe",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Mond",
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
    "Heimkehr",
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
    "{icon:anchor} Anker: Ball fangen, zielen, dann loslassen",
    "Armored bricks take two hits; their rims count down", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall",       // awaiting translation
    "One door in: send the ball into the courtyard",       // awaiting translation
    "Knock out the keystone and the arch is open",         // awaiting translation
    "Read the angles: each shadow is a shot",              // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart",       // awaiting translation
    "Bernsteinkerne: Jede Explosion trifft ihre vier Nachbarn",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Öffne einen Weg durch die beiden Relaislinien",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Benachbarte Kerne tragen die Kettenreaktion weiter",
    "{icon:multi} Multiball: drei Bälle, eine Öffnung",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Brich in die Taschen hinter der Panzerung ein",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Phase: drei Steintreffer ohne Abprall",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Durchbrich die Hülle, dann zünde den inneren Weg",
    "Through the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                        // awaiting translation
    "Three bells: ring each one from beneath",                       // awaiting translation
    "Fang einen Ball, während die anderen weiterfliegen",
    "Folge dem Relais um die offene Mitte",
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
    "Ein letzter Orbit: Jede Öffnung zählt",
];
