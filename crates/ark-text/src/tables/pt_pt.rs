//! European Portuguese. Informal *tu*, as Portuguese games address
//! players; Portugal's spelling and words (ecrã, rato, guardar). Labels in
//! capitals.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Destrói o cosmos",
        ContinueJourney => "Continuar viagem",
        NewJourney => "Nova viagem",
        SectorSelect => "Escolher setor",
        ContinueDetail => "{0} · {1}",
        StatMedals => "MEDALHAS",
        StatBest => "RECORDE",
        ActionServe => "Lançar",
        ActionRelease => "Soltar",
        ActionSelect => "Selecionar",
        ActionResume => "Retomar",
        ActionBack => "Voltar",
        Fullscreen => "Ecrã inteiro",
        SectorsHeading => "Setores",
        PracticeNote => "Os treinos nunca alteram a tua viagem",
        MedalClear => "CONCLUÍDO",
        MedalClean => "IMPECÁVEL",
        MedalSwift => "VELOZ",
        PlaySector => "Jogar setor {0}",
        UnlockHint => "Conclui o setor {0} para desbloquear",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · SETOR {1}",
        Paused => "Em pausa",
        RetrySector => "Repetir setor",
        MainMenu => "Menu principal",
        JourneyComplete => "Viagem concluída",
        OneMoreOrbit => "Mais uma órbita?",
        StatPoints => "PONTOS",
        SectorClear => "Setor concluído",
        StatTime => "TEMPO",
        StatBonus => "BÓNUS",
        StatChain => "COMBO",
        ExtraLife => "Capítulo concluído · +1 vida",
        NextSector => "Setor seguinte",
        BackToSectors => "Voltar aos setores",
        SaveFailed => "Não foi possível guardar o progresso",
        PerfTitle => "Desempenho / CPU",
        HelpResume => "Continua onde paraste",
        HelpRetry => "Recomeça no ponto de controlo do setor",
        HelpMainMenu => "A tua viagem está guardada",
        HelpSectors => "Treina em qualquer setor aberto",
        HelpNewJourney => "Recomeça no setor 01 · as medalhas ficam",
        Settings => "Definições",
        HelpSettings => "Som, ecrã, idioma",
        SettingSound => "Som",
        SettingVolume => "Volume",
        SettingDisplay => "Ecrã",
        SettingLanguage => "Idioma",
        DisplayWindow => "Janela",
        LanguageSystem => "Sistema",
        ActionAdjust => "Ajustar",
        SettingEffects => "Efeitos",
        SettingContrast => "Contraste",
        LookStandard => "Padrão",
        EffectsReduced => "Reduzidos",
        ContrastHigh => "Alto",
        KeySpace => "Espaço",
        KeyEsc => "Esc",
        SectorsOf => "{0} de {1} setores",
        TargetBest => "{0} · recorde {1}",
        LifeGained => "+1 vida",
        PresenceMenus => "Nos menus",
        PresenceJourney => "Setor {0} · {1}",
        PresencePractice => "A treinar o setor {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "ALVORADA",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "CREPÚSCULO",
            Chapter::BlueHour => "HORA AZUL",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "LARGO",
            Power::Slow => "LENTO",
            Power::Multi => "MULTIBOLA",
            Power::Anchor => "ÂNCORA",
            Power::Phase => "FASE",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Continuar"),
        UnlockHint => Some("Bloqueado"),
        RetrySector => Some("Repetir"),
        MainMenu => Some("Menu"),
        NewJourney => Some("Nova"),
        SectorSelect => Some("Setores"),
        NextSector => Some("Seguinte"),
        BackToSectors => Some("Setores"),
        Settings => Some("Opções"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Retomar"),
        PlaySector => Some("Jogar {0}"),
        ActionServe => Some("Lançar"),
        ActionRelease => Some("Soltar"),
        ActionSelect => Some("Escolher"),
        ActionBack => Some("Voltar"),
        ActionAdjust => Some("Ajustar"),
        SettingEffects => Some("Efeitos"),
        SettingContrast => Some("Contraste"),
        LookStandard => Some("Padrão"),
        EffectsReduced => Some("Menos"),
        ContrastHigh => Some("Alto"),
        StatBest => Some("RECORDE"),
        StatMedals => Some("MEDALHAS"),
        SectorsHeading => Some("Setores"),
        SaveFailed => Some("Não guardado"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Espaço"),
        HelpResume => Some("Voltar ao jogo"),
        HelpRetry => Some("Do ponto de controlo"),
        HelpMainMenu => Some("Viagem guardada"),
        HelpSectors => Some("Treinar setores"),
        HelpNewJourney => Some("Do setor 01"),
        HelpSettings => Some("Som, ecrã"),
        SettingSound => Some("Som"),
        SettingVolume => Some("Volume"),
        SettingDisplay => Some("Ecrã"),
        SettingLanguage => Some("Idioma"),
        DisplayWindow => Some("Janela"),
        Fullscreen => Some("Inteiro"),
        LanguageSystem => Some("Sistema"),
        ExtraLife => Some("+1 vida"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("SETOR {1}"),
        JourneyComplete => Some("Concluída"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "Primeira luz",
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
    "Esteira",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Transição",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Ressonância",
    "Prisma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Ressaca",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Arrebol",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Paralaxe",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Nascer da lua",
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
    "Regresso",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "Primeira luz",
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
    "Esteira",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Transição",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Ressonância",
    "Prisma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Ressaca",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Arrebol",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Paralaxe",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Lua",
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
    "Regresso",
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
    "{icon:anchor} Âncora: apanha a bola, aponta e depois solta-a",
    "Armored bricks take two hits: watch the rims", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall", // awaiting translation
    "One door in: send the ball into the courtyard", // awaiting translation
    "Knock out the keystone and the arch is open",  // awaiting translation
    "Read the angles: each shadow is a shot",       // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart", // awaiting translation
    "Núcleos âmbar: cada explosão atinge os quatro vizinhos",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Abre caminho pelas duas linhas de relés",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Núcleos vizinhos propagam a reação",
    "{icon:multi} Multibola: três bolas, uma abertura",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Entra nas bolsas atrás da blindagem",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Fase: três contactos com tijolos sem ressaltar",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Perfura a carapaça e depois acende a rota interior",
    "In by the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                      // awaiting translation
    "Three bells: ring each one from beneath",                     // awaiting translation
    "Apanha uma bola enquanto as outras continuam",
    "Segue o relé à volta do centro aberto",
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
    "Uma última órbita: faz cada abertura valer",
];
