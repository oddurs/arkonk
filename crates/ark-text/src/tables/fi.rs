//! Finnish. Hints read "action: keys", which keeps them short; labels in
//! capitals.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Murskaa kosmos",
        ContinueJourney => "Jatka matkaa",
        NewJourney => "Uusi matka",
        SectorSelect => "Valitse sektori",
        ContinueDetail => "{0} · {1}",
        StatMedals => "MITALIT",
        StatBest => "ENNÄTYS",
        ActionServe => "Syötä",
        ActionRelease => "Vapauta",
        ActionSelect => "Valitse",
        ActionResume => "Jatka",
        ActionBack => "Takaisin",
        Fullscreen => "Koko näyttö",
        SectorsHeading => "Sektorit",
        PracticeNote => "Harjoittelu ei koskaan muuta matkaasi",
        MedalClear => "LÄPI",
        MedalClean => "PUHDAS",
        MedalSwift => "NOPEA",
        PlaySector => "Pelaa sektori {0}",
        UnlockHint => "Avaa läpäisemällä sektori {0}",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · SEKTORI {1}",
        Paused => "Tauko",
        RetrySector => "Sektori uudelleen",
        MainMenu => "Päävalikko",
        JourneyComplete => "Matka päättyi",
        OneMoreOrbit => "Vielä yksi kierros?",
        StatPoints => "PISTEET",
        SectorClear => "Sektori läpäisty",
        StatTime => "AIKA",
        StatBonus => "BONUS",
        StatChain => "KETJU",
        ExtraLife => "Luku läpäisty · +1 elämä",
        NextSector => "Seuraava sektori",
        BackToSectors => "Takaisin sektoreihin",
        SaveFailed => "Edistymistä ei voitu tallentaa",
        PerfTitle => "Suorituskyky / CPU",
        HelpResume => "Jatka siitä, mihin jäit",
        HelpRetry => "Aloita sektorin tarkistuspisteestä",
        HelpMainMenu => "Matkasi on tallennettu",
        HelpSectors => "Harjoittele mitä tahansa avointa sektoria",
        HelpNewJourney => "Alusta sektorista 01 · mitalit säilyvät",
        Settings => "Asetukset",
        HelpSettings => "Ääni, näyttö, kieli",
        SettingSound => "Ääni",
        SettingVolume => "Äänenvoimakkuus",
        SettingDisplay => "Näyttö",
        SettingLanguage => "Kieli",
        DisplayWindow => "Ikkuna",
        LanguageSystem => "Järjestelmä",
        ActionAdjust => "Säädä",
        SettingEffects => "Tehosteet",
        SettingContrast => "Kontrasti",
        LookStandard => "Vakio",
        EffectsReduced => "Vähennetty",
        ContrastHigh => "Korkea",
        KeySpace => "Välilyönti",
        KeyEsc => "Esc",
        SectorsOf => "{0}/{1} sektoria",
        TargetBest => "{0} · paras {1}",
        LifeGained => "+1 elämä",
        PresenceMenus => "Valikoissa",
        PresenceJourney => "Sektori {0} · {1}",
        PresencePractice => "Harjoittelee sektoria {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "AAMUNKOITTO",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "ILTAHÄMÄRÄ",
            Chapter::BlueHour => "SININEN HETKI",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "LEVEÄ",
            Power::Slow => "HIDAS",
            Power::Multi => "MONIPALLO",
            Power::Anchor => "ANKKURI",
            Power::Phase => "VAIHE",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Jatka"),
        UnlockHint => Some("Lukittu"),
        RetrySector => Some("Uudelleen"),
        MainMenu => Some("Valikko"),
        NewJourney => Some("Uusi"),
        SectorSelect => Some("Sektorit"),
        NextSector => Some("Seuraava"),
        BackToSectors => Some("Sektorit"),
        Settings => Some("Asetukset"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Jatka"),
        PlaySector => Some("Pelaa {0}"),
        ActionServe => Some("Syötä"),
        ActionRelease => Some("Vapauta"),
        ActionSelect => Some("Valitse"),
        ActionBack => Some("Takaisin"),
        ActionAdjust => Some("Säädä"),
        SettingEffects => Some("Tehosteet"),
        SettingContrast => Some("Kontrasti"),
        LookStandard => Some("Vakio"),
        EffectsReduced => Some("Vähemmän"),
        ContrastHigh => Some("Korkea"),
        StatBest => Some("ENNÄTYS"),
        StatMedals => Some("MITALIT"),
        SectorsHeading => Some("Sektorit"),
        SaveFailed => Some("Ei tallennettu"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Väli"),
        HelpResume => Some("Takaisin peliin"),
        HelpRetry => Some("Tarkistuspisteestä"),
        HelpMainMenu => Some("Matka tallennettu"),
        HelpSectors => Some("Harjoittele"),
        HelpNewJourney => Some("Sektorista 01"),
        HelpSettings => Some("Ääni, näyttö"),
        SettingSound => Some("Ääni"),
        SettingVolume => Some("Voimakkuus"),
        SettingDisplay => Some("Näyttö"),
        SettingLanguage => Some("Kieli"),
        DisplayWindow => Some("Ikkuna"),
        Fullscreen => Some("Koko"),
        LanguageSystem => Some("Järjestelmä"),
        ExtraLife => Some("+1 elämä"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("SEKTORI {1}"),
        JourneyComplete => Some("Päättyi"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "Ensivalo",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satelliitit",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Imuvirta",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Ristihäivytys",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Resonanssi",
    "Prisma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Pohjavirta",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Jälkihehku",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Parallaksi",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Kuunnousu",
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
    "Kotiinpaluu",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "Ensivalo",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satelliitit",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Imuvirta",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Häivytys",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Resonanssi",
    "Prisma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Pohjavirta",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Jälkihehku",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Parallaksi",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Kuunnousu",
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
    "Kotiinpaluu",
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
    "{icon:anchor} Ankkuri: nappaa pallo, tähtää ja vapauta",
    "Armored bricks take two hits; their rims count down", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall",       // awaiting translation
    "One door in: send the ball into the courtyard",       // awaiting translation
    "Knock out the keystone and the arch is open",         // awaiting translation
    "Read the angles: each shadow is a shot",              // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart",       // awaiting translation
    "Meripihkaytimet: räjähdys osuu neljään naapuriin",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Avaa reitti kahden välityslinjan läpi",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Vierekkäiset ytimet välittävät reaktion",
    "{icon:multi} Monipallo: kolme palloa, yksi aukko",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Murtaudu panssarin takaisiin taskuihin",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Vaihe: kolme tiiliosumaa ilman kimpoamista",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Läpäise kuori ja sytytä sitten sisäreitti",
    "Through the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                        // awaiting translation
    "Three bells: ring each one from beneath",                       // awaiting translation
    "Nappaa palaava pallo muiden lentäessä",
    "Seuraa välitystä avoimen keskustan ympäri",
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
    "Viimeinen kierros: hyödynnä jokainen aukko",
];
