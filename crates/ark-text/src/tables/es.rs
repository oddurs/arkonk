//! Spanish as written in Spain. Informal *tú*; labels in capitals with
//! accents kept (RÉCORD).
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Rompe el cosmos",
        ContinueJourney => "Continuar viaje",
        NewJourney => "Nuevo viaje",
        SectorSelect => "Elegir sector",
        ContinueDetail => "{0} · {1}",
        StatMedals => "MEDALLAS",
        StatBest => "RÉCORD",
        ActionServe => "Sacar",
        ActionRelease => "Soltar",
        ActionSelect => "Elegir",
        ActionResume => "Reanudar",
        ActionBack => "Volver",
        Fullscreen => "Pantalla completa",
        SectorsHeading => "Sectores",
        PracticeNote => "Practicar nunca cambia tu viaje",
        MedalClear => "SUPERADO",
        MedalClean => "IMPECABLE",
        MedalSwift => "VELOZ",
        PlaySector => "Jugar sector {0}",
        UnlockHint => "Supera el sector {0} para desbloquearlo",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · SECTOR {1}",
        Paused => "En pausa",
        RetrySector => "Reintentar sector",
        MainMenu => "Menú principal",
        JourneyComplete => "Viaje completado",
        OneMoreOrbit => "¿Otra órbita?",
        StatPoints => "PUNTOS",
        SectorClear => "Sector superado",
        StatTime => "TIEMPO",
        StatBonus => "BONIFICACIÓN",
        StatChain => "CADENA",
        ExtraLife => "Capítulo completado · +1 vida",
        NextSector => "Siguiente sector",
        BackToSectors => "Volver a los sectores",
        SaveFailed => "No se pudo guardar el progreso",
        PerfTitle => "Rendimiento / CPU",
        HelpResume => "Sigue donde lo dejaste",
        HelpRetry => "Vuelve al punto de control del sector",
        HelpMainMenu => "Tu viaje está guardado",
        HelpSectors => "Practica cualquier sector abierto",
        HelpNewJourney => "Empieza de nuevo en el sector 01 · las medallas se quedan",
        Settings => "Ajustes",
        HelpSettings => "Sonido, pantalla, idioma",
        SettingSound => "Sonido",
        SettingVolume => "Volumen",
        SettingDisplay => "Pantalla",
        SettingLanguage => "Idioma",
        DisplayWindow => "Ventana",
        LanguageSystem => "Sistema",
        ActionAdjust => "Ajustar",
        SettingEffects => "Efectos",
        SettingContrast => "Contraste",
        LookStandard => "Estándar",
        EffectsReduced => "Reducidos",
        ContrastHigh => "Alto",
        KeySpace => "Espacio",
        KeyEsc => "Esc",
        SectorsOf => "{0} de {1} sectores",
        TargetBest => "{0} · récord {1}",
        LifeGained => "+1 vida",
        PresenceMenus => "En los menús",
        PresenceJourney => "Sector {0} · {1}",
        PresencePractice => "Practicando el sector {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "AMANECER",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "OCASO",
            Chapter::BlueHour => "HORA AZUL",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "ANCHO",
            Power::Slow => "LENTO",
            Power::Multi => "MULTIBOLA",
            Power::Anchor => "ANCLA",
            Power::Phase => "FASE",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Continuar"),
        UnlockHint => Some("Bloqueado"),
        RetrySector => Some("Reintentar"),
        MainMenu => Some("Menú"),
        NewJourney => Some("Nuevo"),
        SectorSelect => Some("Sectores"),
        NextSector => Some("Siguiente"),
        BackToSectors => Some("Sectores"),
        StatBonus => Some("BONO"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Seguir"),
        PlaySector => Some("Jugar {0}"),
        Settings => Some("Ajustes"),
        ActionServe => Some("Sacar"),
        ActionRelease => Some("Soltar"),
        ActionSelect => Some("Elegir"),
        ActionBack => Some("Volver"),
        ActionAdjust => Some("Ajustar"),
        StatBest => Some("RÉCORD"),
        StatMedals => Some("MEDALLAS"),
        SectorsHeading => Some("Sectores"),
        SaveFailed => Some("Sin guardar"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Espacio"),
        HelpResume => Some("Volver al juego"),
        HelpRetry => Some("Desde el punto de control"),
        HelpMainMenu => Some("Viaje guardado"),
        HelpSectors => Some("Practicar sectores"),
        HelpNewJourney => Some("Desde el sector 01"),
        HelpSettings => Some("Sonido, pantalla"),
        SettingSound => Some("Sonido"),
        SettingVolume => Some("Volumen"),
        SettingDisplay => Some("Pantalla"),
        SettingLanguage => Some("Idioma"),
        DisplayWindow => Some("Ventana"),
        Fullscreen => Some("Completa"),
        LanguageSystem => Some("Sistema"),
        ExtraLife => Some("+1 vida"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("SECTOR {1}"),
        SettingEffects => Some("Efectos"),
        SettingContrast => Some("Contraste"),
        LookStandard => Some("Normal"),
        EffectsReduced => Some("Pocos"),
        ContrastHigh => Some("Alto"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "Primera luz",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satélites",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Estela",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Fundido",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Resonancia",
    "Prisma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Resaca",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Resplandor",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Paralaje",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Salida de la luna",
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
    "Regreso",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "Primera luz",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satélites",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Estela",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Fundido",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Resonancia",
    "Prisma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Resaca",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Resplandor",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Paralaje",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Luna",
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
    "Regreso",
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
    "{icon:anchor} Ancla: atrapa la bola, apunta y suéltala",
    "Armored bricks take two hits; their rims count down", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall",       // awaiting translation
    "One door in: send the ball into the courtyard",       // awaiting translation
    "Knock out the keystone and the arch is open",         // awaiting translation
    "Read the angles: each shadow is a shot",              // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart",       // awaiting translation
    "Núcleos ámbar: cada explosión alcanza a sus cuatro vecinos",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Abre una ruta entre las dos líneas de relés",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Los núcleos vecinos propagan la reacción",
    "{icon:multi} Multibola: tres bolas, una abertura",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Entra en los huecos tras el blindaje",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Fase: tres contactos con ladrillos sin rebotar",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Perfora la coraza y luego enciende la ruta interior",
    "Through the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                        // awaiting translation
    "Three bells: ring each one from beneath",                       // awaiting translation
    "Atrapa una bola mientras las demás siguen en juego",
    "Sigue el relé alrededor del centro abierto",
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
    "Una última órbita: que cada abertura cuente",
];
