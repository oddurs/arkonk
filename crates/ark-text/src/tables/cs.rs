//! Czech. Informal *ty*; hints read "action: keys", which keeps them short;
//! labels in capitals.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Rozbij vesmír",
        ContinueJourney => "Pokračovat v cestě",
        NewJourney => "Nová cesta",
        SectorSelect => "Výběr sektoru",
        ContinueDetail => "{0} · {1}",
        StatMedals => "MEDAILE",
        StatBest => "REKORD",
        ActionServe => "Podat",
        ActionRelease => "Uvolnit",
        ActionSelect => "Vybrat",
        ActionResume => "Pokračovat",
        ActionBack => "Zpět",
        Fullscreen => "Celá obrazovka",
        SectorsHeading => "Sektory",
        PracticeNote => "Tréninky tvou cestu nikdy nezmění",
        MedalClear => "SPLNĚNO",
        MedalClean => "BEZ ZTRÁT",
        MedalSwift => "RYCHLE",
        PlaySector => "Hrát sektor {0}",
        UnlockHint => "Odemkneš dokončením sektoru {0}",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · SEKTOR {1}",
        Paused => "Pozastaveno",
        RetrySector => "Opakovat sektor",
        MainMenu => "Hlavní nabídka",
        JourneyComplete => "Cesta dokončena",
        OneMoreOrbit => "Ještě jeden oběh?",
        StatPoints => "BODY",
        SectorClear => "Sektor dokončen",
        StatTime => "ČAS",
        StatBonus => "BONUS",
        StatChain => "ŘETĚZ",
        ExtraLife => "Kapitola dokončena · +1 život",
        NextSector => "Další sektor",
        BackToSectors => "Zpět k sektorům",
        SaveFailed => "Postup se nepodařilo uložit",
        PerfTitle => "Výkon / CPU",
        HelpResume => "Pokračuj od místa přerušení",
        HelpRetry => "Znovu od kontrolního bodu sektoru",
        HelpMainMenu => "Tvá cesta je uložena",
        HelpSectors => "Trénuj kterýkoli otevřený sektor",
        HelpNewJourney => "Znovu od sektoru 01 · medaile zůstanou",
        Settings => "Nastavení",
        HelpSettings => "Zvuk, obraz, jazyk",
        SettingSound => "Zvuk",
        SettingVolume => "Hlasitost",
        SettingDisplay => "Obraz",
        SettingLanguage => "Jazyk",
        DisplayWindow => "Okno",
        LanguageSystem => "Systémový",
        ActionAdjust => "Upravit",
        SettingEffects => "Efekty",
        SettingContrast => "Kontrast",
        LookStandard => "Standardní",
        EffectsReduced => "Omezené",
        ContrastHigh => "Vysoký",
        KeySpace => "Mezerník",
        KeyEsc => "Esc",
        SectorsOf => "{0} z {1} sektorů",
        TargetBest => "{0} · rekord {1}",
        LifeGained => "+1 život",
        PresenceMenus => "V nabídkách",
        PresenceJourney => "Sektor {0} · {1}",
        PresencePractice => "Trénuje sektor {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "ÚSVIT",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "SOUMRAK",
            Chapter::BlueHour => "MODRÁ HODINKA",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "ŠIROKÁ",
            Power::Slow => "POMALÁ",
            Power::Multi => "MULTIMÍČEK",
            Power::Anchor => "KOTVA",
            Power::Phase => "FÁZE",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Pokračovat"),
        UnlockHint => Some("Zamčeno"),
        RetrySector => Some("Znovu"),
        MainMenu => Some("Nabídka"),
        NewJourney => Some("Nová"),
        SectorSelect => Some("Sektory"),
        NextSector => Some("Další"),
        BackToSectors => Some("Sektory"),
        Settings => Some("Volby"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Dál"),
        PlaySector => Some("Hrát {0}"),
        ActionServe => Some("Podat"),
        ActionRelease => Some("Pustit"),
        ActionSelect => Some("Vybrat"),
        ActionBack => Some("Zpět"),
        ActionAdjust => Some("Změnit"),
        SettingEffects => Some("Efekty"),
        SettingContrast => Some("Kontrast"),
        LookStandard => Some("Běžné"),
        EffectsReduced => Some("Méně"),
        ContrastHigh => Some("Vysoký"),
        StatBest => Some("REKORD"),
        StatMedals => Some("MEDAILE"),
        SectorsHeading => Some("Sektory"),
        SaveFailed => Some("Neuloženo"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Mezera"),
        HelpResume => Some("Zpět do hry"),
        HelpRetry => Some("Od kontrolního bodu"),
        HelpMainMenu => Some("Cesta uložena"),
        HelpSectors => Some("Trénink sektorů"),
        HelpNewJourney => Some("Od sektoru 01"),
        HelpSettings => Some("Zvuk, obraz"),
        SettingSound => Some("Zvuk"),
        SettingVolume => Some("Hlasitost"),
        SettingDisplay => Some("Obraz"),
        SettingLanguage => Some("Jazyk"),
        DisplayWindow => Some("Okno"),
        Fullscreen => Some("Celá"),
        LanguageSystem => Some("Systém"),
        ExtraLife => Some("+1 život"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("SEKTOR {1}"),
        JourneyComplete => Some("Dokončeno"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "První světlo",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satelity",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Úplav",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Prolínání",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Rezonance",
    "Hranol",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Spodní proud",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Dosvit",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Paralaxa",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Východ měsíce",
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
    "Návrat domů",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "První světlo",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satelity",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Úplav",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Prolínání",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Rezonance",
    "Hranol",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Proud",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Dosvit",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Paralaxa",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Měsíc",
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
    "Návrat",
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
    "{icon:anchor} Kotva: chyť míček, zamiř a pusť ho",
    "Armored bricks take two hits; their rims count down", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall",       // awaiting translation
    "One door in: send the ball into the courtyard",       // awaiting translation
    "Knock out the keystone and the arch is open",         // awaiting translation
    "Read the angles: each shadow is a shot",              // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart",       // awaiting translation
    "Jantarová jádra: každý výbuch zasáhne čtyři sousedy",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Otevři cestu skrz obě řady relé",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Sousední jádra šíří reakci dál",
    "{icon:multi} Multimíček: tři míčky, jeden průchod",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Proraž se do kapes za pancířem",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Fáze: tři zásahy cihel bez odrazu",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Proraž skořápku, pak zapal vnitřní trasu",
    "Through the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                        // awaiting translation
    "Three bells: ring each one from beneath",                       // awaiting translation
    "Chyť vracející se míček, zatímco ostatní letí",
    "Sleduj relé kolem volného středu",
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
    "Poslední oběh: využij každý průchod",
];
