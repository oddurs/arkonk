//! Italian. Informal *tu*; labels in capitals.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Spezza il cosmo",
        ContinueJourney => "Continua il viaggio",
        NewJourney => "Nuovo viaggio",
        SectorSelect => "Scegli settore",
        ContinueDetail => "{0} · {1}",
        StatMedals => "MEDAGLIE",
        StatBest => "RECORD",
        ActionServe => "Lancia",
        ActionRelease => "Rilascia",
        ActionSelect => "Seleziona",
        ActionResume => "Riprendi",
        ActionBack => "Indietro",
        Fullscreen => "Schermo intero",
        SectorsHeading => "Settori",
        PracticeNote => "L’allenamento non cambia mai il tuo viaggio",
        MedalClear => "SUPERATO",
        MedalClean => "PERFETTO",
        MedalSwift => "RAPIDO",
        PlaySector => "Gioca il settore {0}",
        UnlockHint => "Completa il settore {0} per sbloccarlo",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · SETTORE {1}",
        Paused => "In pausa",
        RetrySector => "Riprova il settore",
        MainMenu => "Menu principale",
        JourneyComplete => "Viaggio completato",
        OneMoreOrbit => "Un’altra orbita?",
        StatPoints => "PUNTI",
        SectorClear => "Settore completato",
        StatTime => "TEMPO",
        StatBonus => "BONUS",
        StatChain => "CATENA",
        ExtraLife => "Capitolo completato · +1 vita",
        NextSector => "Settore successivo",
        BackToSectors => "Torna ai settori",
        SaveFailed => "Impossibile salvare i progressi",
        PerfTitle => "Prestazioni / CPU",
        HelpResume => "Riprendi da dove eri rimasto",
        HelpRetry => "Riparti dal checkpoint del settore",
        HelpMainMenu => "Il tuo viaggio è salvato",
        HelpSectors => "Allenati in un settore aperto",
        HelpNewJourney => "Ricomincia dal settore 01 · le medaglie restano",
        Settings => "Impostazioni",
        HelpSettings => "Audio, schermo, lingua",
        SettingSound => "Audio",
        SettingVolume => "Volume",
        SettingDisplay => "Schermo",
        SettingLanguage => "Lingua",
        DisplayWindow => "Finestra",
        LanguageSystem => "Sistema",
        ActionAdjust => "Regola",
        SettingEffects => "Effetti",
        SettingContrast => "Contrasto",
        LookStandard => "Standard",
        EffectsReduced => "Ridotti",
        ContrastHigh => "Alto",
        KeySpace => "Spazio",
        KeyEsc => "Esc",
        SectorsOf => "{0} settori su {1}",
        TargetBest => "{0} · record {1}",
        LifeGained => "+1 vita",
        PresenceMenus => "Nei menu",
        PresenceJourney => "Settore {0} · {1}",
        PresencePractice => "In allenamento, settore {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "ALBA",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "CREPUSCOLO",
            Chapter::BlueHour => "ORA BLU",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "LARGO",
            Power::Slow => "LENTO",
            Power::Multi => "MULTIPALLA",
            Power::Anchor => "ANCORA",
            Power::Phase => "FASE",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Continua"),
        UnlockHint => Some("Bloccato"),
        RetrySector => Some("Riprova"),
        MainMenu => Some("Menu"),
        NewJourney => Some("Nuovo"),
        SectorSelect => Some("Settori"),
        NextSector => Some("Avanti"),
        BackToSectors => Some("Settori"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Riprendi"),
        PlaySector => Some("Gioca {0}"),
        Settings => Some("Opzioni"),
        ActionServe => Some("Lancia"),
        ActionRelease => Some("Rilascia"),
        ActionSelect => Some("Scegli"),
        ActionBack => Some("Indietro"),
        ActionAdjust => Some("Regola"),
        StatBest => Some("RECORD"),
        StatMedals => Some("MEDAGLIE"),
        SectorsHeading => Some("Settori"),
        SaveFailed => Some("Non salvato"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Spazio"),
        HelpResume => Some("Torna a giocare"),
        HelpRetry => Some("Dal checkpoint"),
        HelpMainMenu => Some("Viaggio salvato"),
        HelpSectors => Some("Allenati nei settori"),
        HelpNewJourney => Some("Dal settore 01"),
        HelpSettings => Some("Audio, schermo"),
        SettingSound => Some("Audio"),
        SettingVolume => Some("Volume"),
        SettingDisplay => Some("Schermo"),
        SettingLanguage => Some("Lingua"),
        DisplayWindow => Some("Finestra"),
        Fullscreen => Some("Intero"),
        LanguageSystem => Some("Sistema"),
        ExtraLife => Some("+1 vita"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("SETTORE {1}"),
        SettingEffects => Some("Effetti"),
        SettingContrast => Some("Contrasto"),
        LookStandard => Some("Normale"),
        EffectsReduced => Some("Meno"),
        ContrastHigh => Some("Alto"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "Prima luce",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satelliti",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Scia",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Dissolvenza",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Risonanza",
    "Prisma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Risacca",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Bagliore",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Parallasse",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Sorgere della luna",
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
    "Ritorno",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "Prima luce",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satelliti",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Scia",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Dissolvenza",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Risonanza",
    "Prisma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Risacca",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Bagliore",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Parallasse",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Luna",
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
    "Ritorno",
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
    "{icon:anchor} Ancora: prendi la palla, mira e poi rilasciala",
    "Armored bricks take two hits: watch the rims", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall", // awaiting translation
    "One door in: send the ball into the courtyard", // awaiting translation
    "Knock out the keystone and the arch is open",  // awaiting translation
    "Read the angles: each shadow is a shot",       // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart", // awaiting translation
    "Nuclei ambra: ogni esplosione raggiunge i suoi quattro vicini",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Apri un varco tra le due linee di relè",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "I nuclei vicini propagano la reazione",
    "{icon:multi} Multipalla: tre palle, un’apertura",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Entra nelle sacche dietro la corazza",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Fase: tre contatti con i mattoni senza rimbalzo",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Perfora il guscio, poi accendi il percorso interno",
    "In by the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                      // awaiting translation
    "Three bells: ring each one from beneath",                     // awaiting translation
    "Prendi una palla mentre le altre continuano",
    "Segui il relè intorno al centro aperto",
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
    "Un’ultima orbita: fai contare ogni apertura",
];
