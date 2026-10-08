//! Indonesian. Informal *-mu*, as Indonesian games address players; labels
//! in capitals.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Hancurkan kosmos",
        ContinueJourney => "Lanjutkan perjalanan",
        NewJourney => "Perjalanan baru",
        SectorSelect => "Pilih sektor",
        ContinueDetail => "{0} · {1}",
        StatMedals => "MEDALI",
        StatBest => "REKOR",
        ActionServe => "Servis",
        ActionRelease => "Lepas",
        ActionSelect => "Pilih",
        ActionResume => "Lanjutkan",
        ActionBack => "Kembali",
        Fullscreen => "Layar penuh",
        SectorsHeading => "Sektor",
        PracticeNote => "Latihan tidak pernah mengubah perjalananmu",
        MedalClear => "TUNTAS",
        MedalClean => "BERSIH",
        MedalSwift => "KILAT",
        PlaySector => "Mainkan sektor {0}",
        UnlockHint => "Selesaikan sektor {0} untuk membuka",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · SEKTOR {1}",
        Paused => "Dijeda",
        RetrySector => "Ulangi sektor",
        MainMenu => "Menu utama",
        JourneyComplete => "Perjalanan selesai",
        OneMoreOrbit => "Satu orbit lagi?",
        StatPoints => "POIN",
        SectorClear => "Sektor tuntas",
        StatTime => "WAKTU",
        StatBonus => "BONUS",
        StatChain => "RANTAI",
        ExtraLife => "Bab selesai · +1 nyawa",
        NextSector => "Sektor berikutnya",
        BackToSectors => "Kembali ke sektor",
        SaveFailed => "Kemajuan tidak dapat disimpan",
        PerfTitle => "Performa / CPU",
        HelpResume => "Lanjutkan dari posisi terakhir",
        HelpRetry => "Mulai lagi dari titik simpan sektor",
        HelpMainMenu => "Perjalananmu tersimpan",
        HelpSectors => "Latihan di sektor mana pun yang terbuka",
        HelpNewJourney => "Mulai lagi dari sektor 01 · medali tetap",
        Settings => "Pengaturan",
        HelpSettings => "Suara, layar, bahasa",
        SettingSound => "Suara",
        SettingVolume => "Volume",
        SettingDisplay => "Layar",
        SettingLanguage => "Bahasa",
        DisplayWindow => "Jendela",
        LanguageSystem => "Sistem",
        ActionAdjust => "Atur",
        SettingEffects => "Efek",
        SettingContrast => "Kontras",
        LookStandard => "Standar",
        EffectsReduced => "Dikurangi",
        ContrastHigh => "Tinggi",
        KeySpace => "Spasi",
        KeyEsc => "Esc",
        SectorsOf => "{0} dari {1} sektor",
        TargetBest => "{0} · rekor {1}",
        LifeGained => "+1 nyawa",
        PresenceMenus => "Di menu",
        PresenceJourney => "Sektor {0} · {1}",
        PresencePractice => "Berlatih sektor {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "FAJAR",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "SENJA",
            Chapter::BlueHour => "JAM BIRU",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "LEBAR",
            Power::Slow => "LAMBAT",
            Power::Multi => "MULTIBOLA",
            Power::Anchor => "JANGKAR",
            Power::Phase => "FASE",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Lanjutkan"),
        UnlockHint => Some("Terkunci"),
        RetrySector => Some("Ulangi"),
        MainMenu => Some("Menu"),
        NewJourney => Some("Baru"),
        SectorSelect => Some("Sektor"),
        NextSector => Some("Lanjut"),
        BackToSectors => Some("Sektor"),
        Settings => Some("Opsi"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Lanjut"),
        PlaySector => Some("Main {0}"),
        ActionServe => Some("Servis"),
        ActionRelease => Some("Lepas"),
        ActionSelect => Some("Pilih"),
        ActionBack => Some("Kembali"),
        ActionAdjust => Some("Atur"),
        SettingEffects => Some("Efek"),
        SettingContrast => Some("Kontras"),
        LookStandard => Some("Normal"),
        EffectsReduced => Some("Sedikit"),
        ContrastHigh => Some("Tinggi"),
        StatBest => Some("REKOR"),
        StatMedals => Some("MEDALI"),
        SectorsHeading => Some("Sektor"),
        SaveFailed => Some("Tidak tersimpan"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Spasi"),
        HelpResume => Some("Kembali bermain"),
        HelpRetry => Some("Dari titik simpan"),
        HelpMainMenu => Some("Tersimpan"),
        HelpSectors => Some("Latihan sektor"),
        HelpNewJourney => Some("Dari sektor 01"),
        HelpSettings => Some("Suara, layar"),
        SettingSound => Some("Suara"),
        SettingVolume => Some("Volume"),
        SettingDisplay => Some("Layar"),
        SettingLanguage => Some("Bahasa"),
        DisplayWindow => Some("Jendela"),
        Fullscreen => Some("Penuh"),
        LanguageSystem => Some("Sistem"),
        ExtraLife => Some("+1 nyawa"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("SEKTOR {1}"),
        JourneyComplete => Some("Selesai"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "Cahaya Pertama",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satelit",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Arus Seret",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Silang Pudar",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Resonansi",
    "Prisma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Arus Bawah",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Pendar Senja",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Paralaks",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Bulan Terbit",
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
    "Pulang",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "Cahaya",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Satelit",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Arus Seret",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Pudar",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Resonansi",
    "Prisma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Arus Bawah",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Pendar",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Paralaks",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Supernova",
    "Bulan Terbit",
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
    "Pulang",
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
    "{icon:anchor} Jangkar: tangkap bola, bidik, lalu lepaskan",
    "Armored bricks take two hits; their rims count down", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall",       // awaiting translation
    "One door in: send the ball into the courtyard",       // awaiting translation
    "Knock out the keystone and the arch is open",         // awaiting translation
    "Read the angles: each shadow is a shot",              // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart",       // awaiting translation
    "Inti amber: setiap ledakan mengenai empat tetangganya",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Buka jalan melewati dua jalur relai",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Inti yang bertetangga meneruskan reaksi",
    "{icon:multi} Multibola: tiga bola, satu celah",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Tembus ke kantong di balik lapisan baja",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Fase: tiga sentuhan bata tanpa memantul",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Tembus cangkangnya, lalu nyalakan rute dalam",
    "Through the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                        // awaiting translation
    "Three bells: ring each one from beneath",                       // awaiting translation
    "Tangkap bola yang kembali selagi bola lain terus melaju",
    "Ikuti relai mengitari bagian tengah yang terbuka",
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
    "Orbit terakhir: manfaatkan setiap celah",
];
