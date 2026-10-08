//! French. Labels in capitals with accents kept (MÉDAILLES); a no-break
//! space before `:` and a narrow one before `?`, as French typography sets them.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Brisez le cosmos",
        ContinueJourney => "Poursuivre le voyage",
        NewJourney => "Nouveau voyage",
        SectorSelect => "Choix du secteur",
        ContinueDetail => "{0} · {1}",
        StatMedals => "MÉDAILLES",
        StatBest => "RECORD",
        ActionServe => "Servir",
        ActionRelease => "Relâcher",
        ActionSelect => "Choisir",
        ActionResume => "Reprendre",
        ActionBack => "Retour",
        Fullscreen => "Plein écran",
        SectorsHeading => "Secteurs",
        PracticeNote => "L’entraînement ne modifie jamais votre voyage",
        MedalClear => "RÉUSSI",
        MedalClean => "SANS PERTE",
        MedalSwift => "RAPIDE",
        PlaySector => "Jouer le secteur {0}",
        UnlockHint => "Terminez le secteur {0} pour le débloquer",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · SECTEUR {1}",
        Paused => "Pause",
        RetrySector => "Recommencer le secteur",
        MainMenu => "Menu principal",
        JourneyComplete => "Voyage terminé",
        OneMoreOrbit => "Encore une orbite\u{202f}?",
        StatPoints => "POINTS",
        SectorClear => "Secteur terminé",
        StatTime => "TEMPS",
        StatBonus => "BONUS",
        StatChain => "CHAÎNE",
        ExtraLife => "Chapitre terminé · +1 vie",
        NextSector => "Secteur suivant",
        BackToSectors => "Retour aux secteurs",
        SaveFailed => "Impossible de sauvegarder la progression",
        PerfTitle => "Performances / CPU",
        HelpResume => "Reprenez là où vous en étiez",
        HelpRetry => "Repartir du point de contrôle du secteur",
        HelpMainMenu => "Votre voyage est sauvegardé",
        HelpSectors => "Entraînez-vous sur un secteur ouvert",
        HelpNewJourney => "Recommencer au secteur 01 · les médailles restent",
        Settings => "Paramètres",
        HelpSettings => "Son, affichage, langue",
        SettingSound => "Son",
        SettingVolume => "Volume",
        SettingDisplay => "Affichage",
        SettingLanguage => "Langue",
        DisplayWindow => "Fenêtre",
        LanguageSystem => "Système",
        ActionAdjust => "Régler",
        SettingEffects => "Effets",
        SettingContrast => "Contraste",
        LookStandard => "Standard",
        EffectsReduced => "Réduits",
        ContrastHigh => "Élevé",
        KeySpace => "Espace",
        KeyEsc => "Échap",
        SectorsOf => "{0} secteurs sur {1}",
        TargetBest => "{0} · record {1}",
        LifeGained => "+1 vie",
        PresenceMenus => "Dans les menus",
        PresenceJourney => "Secteur {0} · {1}",
        PresencePractice => "Entraînement, secteur {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "AUBE",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "CRÉPUSCULE",
            Chapter::BlueHour => "HEURE BLEUE",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "LARGE",
            Power::Slow => "LENT",
            Power::Multi => "MULTIBALLE",
            Power::Anchor => "ANCRE",
            Power::Phase => "PHASE",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Continuer"),
        UnlockHint => Some("Verrouillé"),
        RetrySector => Some("Recommencer"),
        MainMenu => Some("Menu"),
        NewJourney => Some("Nouveau"),
        SectorSelect => Some("Secteurs"),
        NextSector => Some("Suivant"),
        BackToSectors => Some("Secteurs"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Reprendre"),
        PlaySector => Some("Jouer {0}"),
        Settings => Some("Options"),
        ActionServe => Some("Servir"),
        ActionRelease => Some("Lâcher"),
        ActionSelect => Some("Choisir"),
        ActionBack => Some("Retour"),
        ActionAdjust => Some("Régler"),
        StatBest => Some("RECORD"),
        StatMedals => Some("MÉDAILLES"),
        SectorsHeading => Some("Secteurs"),
        SaveFailed => Some("Non sauvegardé"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Espace"),
        HelpResume => Some("Retour au jeu"),
        HelpRetry => Some("Depuis le point de contrôle"),
        HelpMainMenu => Some("Voyage sauvegardé"),
        HelpSectors => Some("Entraînement"),
        HelpNewJourney => Some("Depuis le secteur 01"),
        HelpSettings => Some("Son, affichage"),
        SettingSound => Some("Son"),
        SettingVolume => Some("Volume"),
        SettingDisplay => Some("Affichage"),
        SettingLanguage => Some("Langue"),
        DisplayWindow => Some("Fenêtre"),
        Fullscreen => Some("Plein"),
        LanguageSystem => Some("Système"),
        ExtraLife => Some("+1 vie"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("SECTEUR {1}"),
        SettingEffects => Some("Effets"),
        SettingContrast => Some("Contraste"),
        LookStandard => Some("Normal"),
        EffectsReduced => Some("Moins"),
        ContrastHigh => Some("Élevé"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "Première lueur",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satellites",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Sillage",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Fondu enchaîné",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Résonance",
    "Prisme",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Contre-courant",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Rémanence",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Parallaxe",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Lever de lune",
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
    "Retour",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "Lueur",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satellites",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Sillage",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Fondu",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Résonance",
    "Prisme",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Ressac",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Rémanence",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Parallaxe",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Lune",
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
    "Retour",
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
    "{icon:anchor} Ancre\u{a0}: attrapez la balle, visez, puis relâchez-la",
    "Armored bricks take two hits: watch the rims", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall", // awaiting translation
    "One door in: send the ball into the courtyard", // awaiting translation
    "Knock out the keystone and the arch is open",  // awaiting translation
    "Read the angles: each shadow is a shot",       // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart", // awaiting translation
    "Noyaux ambrés\u{a0}: chaque explosion touche ses quatre voisins",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Ouvrez une voie à travers les deux lignes de relais",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Les noyaux voisins propagent la réaction",
    "{icon:multi} Multiballe\u{a0}: trois balles, une seule ouverture",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Percez jusqu’aux poches derrière le blindage",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Phase\u{a0}: trois contacts de brique sans rebond",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Percez la coque, puis allumez la voie intérieure",
    "In by the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                      // awaiting translation
    "Three bells: ring each one from beneath",                     // awaiting translation
    "Rattrapez une balle pendant que les autres continuent",
    "Suivez le relais autour du centre dégagé",
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
    "Une dernière orbite\u{a0}: chaque ouverture compte",
];
