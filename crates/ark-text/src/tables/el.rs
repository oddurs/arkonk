//! Greek. Informal second person; labels in capitals without accents, as
//! Greek sets all-caps text. The question mark is ";" (U+037E normalizes to it).
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "Σπάσε το σύμπαν",
        ContinueJourney => "Συνέχεια ταξιδιού",
        NewJourney => "Νέο ταξίδι",
        SectorSelect => "Επιλογή τομέα",
        ContinueDetail => "{0} · {1}",
        StatMedals => "ΜΕΤΑΛΛΙΑ",
        StatBest => "ΡΕΚΟΡ",
        ActionServe => "Σερβίς",
        ActionRelease => "Απελευθέρωση",
        ActionSelect => "Επιλογή",
        ActionResume => "Συνέχεια",
        ActionBack => "Πίσω",
        Fullscreen => "Πλήρης οθόνη",
        SectorsHeading => "Τομείς",
        PracticeNote => "Η εξάσκηση δεν αλλάζει ποτέ το ταξίδι σου",
        MedalClear => "ΟΛΟΚΛΗΡΩΣΗ",
        MedalClean => "ΑΨΟΓΑ",
        MedalSwift => "ΤΑΧΥΤΗΤΑ",
        PlaySector => "Παίξε τον τομέα {0}",
        UnlockHint => "Ολοκλήρωσε τον τομέα {0} για ξεκλείδωμα",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · ΤΟΜΕΑΣ {1}",
        Paused => "Σε παύση",
        RetrySector => "Τομέας ξανά",
        MainMenu => "Κύριο μενού",
        JourneyComplete => "Τέλος ταξιδιού",
        OneMoreOrbit => "Άλλη μία τροχιά;",
        StatPoints => "ΠΟΝΤΟΙ",
        SectorClear => "Τομέας ολοκληρώθηκε",
        StatTime => "ΧΡΟΝΟΣ",
        StatBonus => "ΜΠΟΝΟΥΣ",
        StatChain => "ΑΛΥΣΙΔΑ",
        ExtraLife => "Τέλος κεφαλαίου · +1 ζωή",
        NextSector => "Επόμενος τομέας",
        BackToSectors => "Πίσω στους τομείς",
        SaveFailed => "Η πρόοδος δεν αποθηκεύτηκε",
        PerfTitle => "Επιδόσεις / CPU",
        HelpResume => "Συνέχισε από εκεί που σταμάτησες",
        HelpRetry => "Από το σημείο ελέγχου του τομέα",
        HelpMainMenu => "Το ταξίδι σου αποθηκεύτηκε",
        HelpSectors => "Εξάσκηση σε ανοιχτούς τομείς",
        HelpNewJourney => "Από τον τομέα 01 · τα μετάλλια μένουν",
        Settings => "Ρυθμίσεις",
        HelpSettings => "Ήχος, οθόνη, γλώσσα",
        SettingSound => "Ήχος",
        SettingVolume => "Ένταση",
        SettingDisplay => "Οθόνη",
        SettingLanguage => "Γλώσσα",
        DisplayWindow => "Παράθυρο",
        LanguageSystem => "Συστήματος",
        ActionAdjust => "Ρύθμιση",
        SettingEffects => "Εφέ",
        SettingContrast => "Αντίθεση",
        LookStandard => "Τυπικό",
        EffectsReduced => "Μειωμένα",
        ContrastHigh => "Υψηλή",
        KeySpace => "Διάστημα",
        KeyEsc => "Esc",
        SectorsOf => "{0} από {1} τομείς",
        TargetBest => "{0} · ρεκόρ {1}",
        LifeGained => "+1 ζωή",
        PresenceMenus => "Στα μενού",
        PresenceJourney => "Τομέας {0} · {1}",
        PresencePractice => "Εξάσκηση στον τομέα {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "ΑΥΓΗ",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "ΛΥΚΟΦΩΣ",
            Chapter::BlueHour => "ΜΠΛΕ ΩΡΑ",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "ΦΑΡΔΥ",
            Power::Slow => "ΑΡΓΟ",
            Power::Multi => "ΠΟΛΛΕΣ ΜΠΑΛΕΣ",
            Power::Anchor => "ΑΓΚΥΡΑ",
            Power::Phase => "ΦΑΣΗ",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("Συνέχεια"),
        UnlockHint => Some("Κλειδωμένο"),
        RetrySector => Some("Ξανά"),
        MainMenu => Some("Μενού"),
        NewJourney => Some("Νέο"),
        SectorSelect => Some("Τομείς"),
        NextSector => Some("Επόμενος"),
        BackToSectors => Some("Τομείς"),
        Settings => Some("Επιλογές"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("Συνέχεια"),
        PlaySector => Some("Παίξε {0}"),
        ActionServe => Some("Σερβίς"),
        ActionRelease => Some("Άφησε"),
        ActionSelect => Some("Επιλογή"),
        ActionBack => Some("Πίσω"),
        ActionAdjust => Some("Ρύθμιση"),
        SettingEffects => Some("Εφέ"),
        SettingContrast => Some("Αντίθεση"),
        LookStandard => Some("Τυπικό"),
        EffectsReduced => Some("Λιγότερα"),
        ContrastHigh => Some("Υψηλή"),
        StatBest => Some("ΡΕΚΟΡ"),
        StatMedals => Some("ΜΕΤΑΛΛΙΑ"),
        SectorsHeading => Some("Τομείς"),
        SaveFailed => Some("Δεν αποθηκεύτηκε"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("Κενό"),
        HelpResume => Some("Πίσω στο παιχνίδι"),
        HelpRetry => Some("Από το σημείο ελέγχου"),
        HelpMainMenu => Some("Αποθηκεύτηκε"),
        HelpSectors => Some("Εξάσκηση"),
        HelpNewJourney => Some("Από τον τομέα 01"),
        HelpSettings => Some("Ήχος, οθόνη"),
        SettingSound => Some("Ήχος"),
        SettingVolume => Some("Ένταση"),
        SettingDisplay => Some("Οθόνη"),
        SettingLanguage => Some("Γλώσσα"),
        DisplayWindow => Some("Παράθυρο"),
        Fullscreen => Some("Πλήρης"),
        LanguageSystem => Some("Σύστημα"),
        ExtraLife => Some("+1 ζωή"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("ΤΟΜΕΑΣ {1}"),
        JourneyComplete => Some("Τέλος"),
        SectorClear => Some("Ολοκληρώθηκε"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "Πρώτο φως",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Δορυφόροι",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Ρεύμα έλξης",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Διασταύρωση",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Συντονισμός",
    "Πρίσμα",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Υπόρρευμα",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "Απόλαμψη",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Παράλλαξη",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Σουπερνόβα",
    "Ανατολή σελήνης",
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
    "Επιστροφή",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "Πρώτο φως",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "Δορυφόροι",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "Ρεύμα",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "Διασταύρωση",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "Συντονισμός",
    "Πρίσμα",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "Υπόρρευμα",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "Απόλαμψη",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "Παράλλαξη",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "Σουπερνόβα",
    "Σελήνη",
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
    "Επιστροφή",
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
    "{icon:anchor} Άγκυρα: πιάσε την μπάλα, σημάδεψε και άφησέ τη",
    "Armored bricks take two hits: watch the rims", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall", // awaiting translation
    "One door in: send the ball into the courtyard", // awaiting translation
    "Knock out the keystone and the arch is open",  // awaiting translation
    "Read the angles: each shadow is a shot",       // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart", // awaiting translation
    "Κεχριμπαρένιοι πυρήνες: κάθε έκρηξη φτάνει τους τέσσερις γείτονες",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "Άνοιξε δρόμο μέσα από τις δύο γραμμές αναμεταδοτών",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "Οι γειτονικοί πυρήνες μεταφέρουν την αντίδραση",
    "{icon:multi} Πολλαπλή μπάλα: τρεις μπάλες, ένα άνοιγμα",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "Μπες στις κοιλότητες πίσω από τη θωράκιση",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} Φάση: τρεις επαφές με τούβλα χωρίς αναπήδηση",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "Τρύπησε το κέλυφος και άναψε την εσωτερική διαδρομή",
    "In by the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                      // awaiting translation
    "Three bells: ring each one from beneath",                     // awaiting translation
    "Πιάσε μια μπάλα που επιστρέφει ενώ οι άλλες συνεχίζουν",
    "Ακολούθησε τον αναμεταδότη γύρω από το ανοιχτό κέντρο",
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
    "Μία τελευταία τροχιά: αξιοποίησε κάθε άνοιγμα",
];
