//! Turkish. Informal *sen*; hints read "action: keys". Labels in capitals
//! with the dotted capital İ, as Turkish writes them.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Kozmosu parçala",
        ContinueJourney => "Yolculuğa devam et",
        NewJourney => "Yeni yolculuk",
        SectorSelect => "Sektör seçimi",
        ContinueDetail => "{0} · {1}",
        StatMedals => "MADALYALAR",
        StatBest => "REKOR",
        ActionServe => "Servis",
        ActionRelease => "Bırak",
        ActionSelect => "Seç",
        ActionResume => "Devam et",
        ActionBack => "Geri",
        Fullscreen => "Tam ekran",
        SectorsHeading => "Sektörler",
        PracticeNote => "Antrenmanlar yolculuğunu asla değiştirmez",
        MedalClear => "TAMAM",
        MedalClean => "KUSURSUZ",
        MedalSwift => "HIZLI",
        PlaySector => "{0}. sektörü oyna",
        UnlockHint => "Açmak için {0}. sektörü tamamla",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · SEKTÖR {1}",
        Paused => "Duraklatıldı",
        RetrySector => "Sektörü tekrarla",
        MainMenu => "Ana menü",
        JourneyComplete => "Yolculuk tamamlandı",
        OneMoreOrbit => "Bir tur daha?",
        StatPoints => "PUAN",
        SectorClear => "Sektör tamamlandı",
        StatTime => "SÜRE",
        StatBonus => "BONUS",
        StatChain => "ZİNCİR",
        ExtraLife => "Bölüm tamamlandı · +1 can",
        NextSector => "Sonraki sektör",
        BackToSectors => "Sektörlere dön",
        SaveFailed => "İlerleme kaydedilemedi",
        PerfTitle => "Performans / CPU",
        HelpResume => "Kaldığın yerden devam et",
        HelpRetry => "Sektörün kontrol noktasından yeniden başla",
        HelpMainMenu => "Yolculuğun kaydedildi",
        HelpSectors => "Açık herhangi bir sektörde antrenman yap",
        HelpNewJourney => "Sektör 01’den yeniden · madalyalar kalır",
        Settings => "Ayarlar",
        HelpSettings => "Ses, ekran, dil",
        SettingSound => "Ses",
        SettingVolume => "Ses düzeyi",
        SettingDisplay => "Ekran",
        SettingLanguage => "Dil",
        DisplayWindow => "Pencere",
        LanguageSystem => "Sistem",
        ActionAdjust => "Ayarla",
        SettingEffects => "Efektler",
        SettingContrast => "Kontrast",
        LookStandard => "Standart",
        EffectsReduced => "Azaltılmış",
        ContrastHigh => "Yüksek",
        KeySpace => "Boşluk",
        KeyEsc => "Esc",
        SectorsOf => "{0}/{1} sektör",
        TargetBest => "{0} · rekor {1}",
        LifeGained => "+1 can",
        PresenceMenus => "Menülerde",
        PresenceJourney => "Sektör {0} · {1}",
        PresencePractice => "Antrenman: {0}. sektör · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "ŞAFAK",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "ALACAKARANLIK",
            Chapter::BlueHour => "MAVİ SAAT",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "GENİŞ",
            Power::Slow => "YAVAŞ",
            Power::Multi => "ÇOKLU TOP",
            Power::Anchor => "ÇAPA",
            Power::Phase => "FAZ",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Devam et"),
        UnlockHint => Some("Kilitli"),
        RetrySector => Some("Tekrar"),
        MainMenu => Some("Menü"),
        NewJourney => Some("Yeni"),
        SectorSelect => Some("Sektörler"),
        NextSector => Some("Sonraki"),
        BackToSectors => Some("Sektörler"),
        Settings => Some("Ayarlar"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Devam"),
        PlaySector => Some("{0}. sektör"),
        ActionServe => Some("Servis"),
        ActionRelease => Some("Bırak"),
        ActionSelect => Some("Seç"),
        ActionBack => Some("Geri"),
        ActionAdjust => Some("Ayarla"),
        SettingEffects => Some("Efektler"),
        SettingContrast => Some("Kontrast"),
        LookStandard => Some("Normal"),
        EffectsReduced => Some("Az"),
        ContrastHigh => Some("Yüksek"),
        StatBest => Some("REKOR"),
        StatMedals => Some("MADALYALAR"),
        SectorsHeading => Some("Sektörler"),
        SaveFailed => Some("Kaydedilmedi"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Boşluk"),
        HelpResume => Some("Oyuna dön"),
        HelpRetry => Some("Kontrol noktasından"),
        HelpMainMenu => Some("Kaydedildi"),
        HelpSectors => Some("Antrenman"),
        HelpNewJourney => Some("Sektör 01’den"),
        HelpSettings => Some("Ses, ekran"),
        SettingSound => Some("Ses"),
        SettingVolume => Some("Ses düzeyi"),
        SettingDisplay => Some("Ekran"),
        SettingLanguage => Some("Dil"),
        DisplayWindow => Some("Pencere"),
        Fullscreen => Some("Tam"),
        LanguageSystem => Some("Sistem"),
        ExtraLife => Some("+1 can"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("SEKTÖR {1}"),
        JourneyComplete => Some("Tamamlandı"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "İlk Işık",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Uydular",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Hava Akımı",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Geçiş",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Rezonans",
    "Prizma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Dip Akıntısı",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Son Işıltı",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Paralaks",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Süpernova",
    "Ay Doğuşu",
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
    "Eve Dönüş",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "İlk Işık",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Uydular",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Hava Akımı",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Geçiş",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Rezonans",
    "Prizma",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Akıntı",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Son Işıltı",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Paralaks",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Süpernova",
    "Ay Doğuşu",
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
    "Eve Dönüş",
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
    "{icon:anchor} Çapa: topu yakala, nişan al, sonra bırak",
    "Armored bricks take two hits; their rims count down", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall",       // awaiting translation
    "One door in: send the ball into the courtyard",       // awaiting translation
    "Knock out the keystone and the arch is open",         // awaiting translation
    "Read the angles: each shadow is a shot",              // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart",       // awaiting translation
    "Kehribar çekirdekler: her patlama dört komşusuna ulaşır",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "İki röle hattı arasından bir yol aç",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Komşu çekirdekler tepkimeyi taşır",
    "{icon:multi} Çoklu top: üç top, tek açıklık",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Zırhın ardındaki ceplere gir",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Faz: sekmeden üç tuğla teması",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Kabuğu del, sonra iç rotayı ateşle",
    "Through the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                        // awaiting translation
    "Three bells: ring each one from beneath",                       // awaiting translation
    "Diğer toplar uçarken dönen bir topu yakala",
    "Açık merkezin çevresinde röleyi izle",
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
    "Son bir yörünge: her açıklığı değerlendir",
];
