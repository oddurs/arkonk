//! Thai. Polite neutral register. Thai writes no spaces between words, so
//! U+200B ZERO WIDTH SPACE marks where a line may break; spaces separate
//! phrases. Labels have no case.
use crate::TextId::{self, *};
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT},
};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        Tagline => "ทำลาย\u{200b}จักรวาล",
        ContinueJourney => "เดินทาง\u{200b}ต่อ",
        NewJourney => "การ\u{200b}เดินทาง\u{200b}ใหม่",
        SectorSelect => "เลือก\u{200b}เซกเตอร์",
        ContinueDetail => "{0} · {1}",
        StatMedals => "เหรียญ",
        StatBest => "สูงสุด",
        ActionServe => "เสิร์ฟ",
        ActionRelease => "ปล่อย",
        ActionSelect => "เลือก",
        ActionResume => "เล่น\u{200b}ต่อ",
        ActionBack => "กลับ",
        Fullscreen => "เต็ม\u{200b}จอ",
        SectorsHeading => "เซกเตอร์",
        PracticeNote => {
            "การ\u{200b}ฝึก\u{200b}ซ้อม\u{200b}ไม่\u{200b}เปลี่ยน\u{200b}การ\u{200b}เดินทาง\u{200b}ของ\u{200b}คุณ"
        }
        MedalClear => "ผ่าน",
        MedalClean => "ไร้\u{200b}ที่\u{200b}ติ",
        MedalSwift => "รวดเร็ว",
        PlaySector => "เล่น\u{200b}เซกเตอร์ {0}",
        UnlockHint => "ผ่าน\u{200b}เซกเตอร์ {0} เพื่อ\u{200b}ปลด\u{200b}ล็อก",
        Plus => "+{0}",
        ReadyEyebrow => "{0} · เซกเตอร์ {1}",
        Paused => "หยุด\u{200b}ชั่วคราว",
        RetrySector => "เล่น\u{200b}เซกเตอร์\u{200b}ใหม่",
        MainMenu => "เมนู\u{200b}หลัก",
        JourneyComplete => "การ\u{200b}เดินทาง\u{200b}สิ้นสุด",
        OneMoreOrbit => "อีก\u{200b}หนึ่ง\u{200b}รอบ?",
        StatPoints => "แต้ม",
        SectorClear => "ผ่าน\u{200b}เซกเตอร์\u{200b}แล้ว",
        StatTime => "เวลา",
        StatBonus => "โบนัส",
        StatChain => "คอมโบ",
        ExtraLife => "จบ\u{200b}บท · +1 ชีวิต",
        NextSector => "เซกเตอร์\u{200b}ถัดไป",
        BackToSectors => "กลับ\u{200b}ไป\u{200b}ที่\u{200b}เซกเตอร์",
        SaveFailed => "บันทึก\u{200b}ความ\u{200b}คืบหน้า\u{200b}ไม่\u{200b}ได้",
        PerfTitle => "ประสิทธิภาพ / CPU",
        HelpResume => "เล่น\u{200b}ต่อ\u{200b}จาก\u{200b}จุด\u{200b}ที่\u{200b}ค้าง\u{200b}ไว้",
        HelpRetry => {
            "เริ่ม\u{200b}ใหม่\u{200b}จาก\u{200b}จุด\u{200b}เช็ก\u{200b}พอยต์\u{200b}ของ\u{200b}เซกเตอร์"
        }
        HelpMainMenu => "บันทึก\u{200b}การ\u{200b}เดินทาง\u{200b}แล้ว",
        HelpSectors => "ฝึก\u{200b}เล่น\u{200b}เซกเตอร์\u{200b}ที่\u{200b}เปิด\u{200b}แล้ว",
        HelpNewJourney => "เริ่ม\u{200b}ใหม่\u{200b}ที่\u{200b}เซกเตอร์ 01 · เหรียญ\u{200b}ยัง\u{200b}อยู่",
        Settings => "การ\u{200b}ตั้ง\u{200b}ค่า",
        HelpSettings => "เสียง การ\u{200b}แสดง\u{200b}ผล ภาษา",
        SettingSound => "เสียง",
        SettingVolume => "ระดับ\u{200b}เสียง",
        SettingDisplay => "การ\u{200b}แสดง\u{200b}ผล",
        SettingLanguage => "ภาษา",
        DisplayWindow => "หน้า\u{200b}ต่าง",
        LanguageSystem => "ตาม\u{200b}ระบบ",
        ActionAdjust => "ปรับ",
        SettingEffects => "เอฟเฟกต์",
        SettingContrast => "คอนทราสต์",
        LookStandard => "มาตรฐาน",
        EffectsReduced => "ลด\u{200b}ลง",
        ContrastHigh => "สูง",
        KeySpace => "เว้น\u{200b}วรรค",
        KeyEsc => "Esc",
        SectorsOf => "{0} จาก {1} เซกเตอร์",
        TargetBest => "{0} · ดี\u{200b}ที่\u{200b}สุด {1}",
        LifeGained => "+1 ชีวิต",
        PresenceMenus => "อยู่\u{200b}ใน\u{200b}เมนู",
        PresenceJourney => "เซกเตอร์ {0} · {1}",
        PresencePractice => "กำลัง\u{200b}ฝึก\u{200b}ซ้อม\u{200b}เซกเตอร์ {0} · {1}",
        SectorName(s) => NAMES[s.index()],
        SectorTip(s) => TIPS[s.index()],
        ChapterName(c) => match c {
            Chapter::Daybreak => "รุ่ง\u{200b}อรุณ",
            Chapter::Morning => "MORNING", // awaiting translation
            Chapter::Zenith => "ZENITH",   // awaiting translation
            Chapter::GoldenHour => "GOLDEN HOUR", // awaiting translation
            Chapter::Afterlight => "สนธยา",
            Chapter::BlueHour => "บลู\u{200b}อาวร์",
            Chapter::Eclipse => "ECLIPSE", // awaiting translation
            Chapter::Aurora => "AURORA",   // awaiting translation
        },
        PowerName(p) => match p {
            Power::Wide => "ขยาย",
            Power::Slow => "ช้า",
            Power::Multi => "หลาย\u{200b}ลูก",
            Power::Anchor => "สมอ",
            Power::Phase => "เฟส",
        },
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    match id {
        ContinueDetail => Some("{0}"),
        ContinueJourney => Some("เล่น\u{200b}ต่อ"),
        UnlockHint => Some("ล็อก"),
        RetrySector => Some("ลอง\u{200b}ใหม่"),
        MainMenu => Some("เมนู"),
        NewJourney => Some("เริ่ม\u{200b}ใหม่"),
        SectorSelect => Some("เซกเตอร์"),
        NextSector => Some("ถัด\u{200b}ไป"),
        BackToSectors => Some("เซกเตอร์"),
        Settings => Some("ตั้ง\u{200b}ค่า"),
        SectorsOf => Some("{0}/{1}"),
        TargetBest => Some("{0}"),
        LifeGained => Some("+1"),
        ActionResume => Some("เล่น\u{200b}ต่อ"),
        PlaySector => Some("เล่น {0}"),
        ActionServe => Some("เสิร์ฟ"),
        ActionRelease => Some("ปล่อย"),
        ActionSelect => Some("เลือก"),
        ActionBack => Some("กลับ"),
        ActionAdjust => Some("ปรับ"),
        SettingEffects => Some("เอฟเฟกต์"),
        SettingContrast => Some("คอนทราสต์"),
        LookStandard => Some("ปกติ"),
        EffectsReduced => Some("น้อย"),
        ContrastHigh => Some("สูง"),
        StatBest => Some("สูงสุด"),
        StatMedals => Some("เหรียญ"),
        SectorsHeading => Some("เซกเตอร์"),
        SaveFailed => Some("ไม่\u{200b}ได้\u{200b}บันทึก"),
        KeyEsc => Some("Esc"),
        KeySpace => Some("เว้น\u{200b}วรรค"),
        HelpResume => Some("กลับ\u{200b}ไป\u{200b}เล่น"),
        HelpRetry => Some("จาก\u{200b}จุด\u{200b}เช็ก\u{200b}พอยต์"),
        HelpMainMenu => Some("บันทึก\u{200b}แล้ว"),
        HelpSectors => Some("ฝึก\u{200b}เล่น"),
        HelpNewJourney => Some("จาก\u{200b}เซกเตอร์ 01"),
        HelpSettings => Some("เสียง การ\u{200b}แสดง\u{200b}ผล"),
        SettingSound => Some("เสียง"),
        SettingVolume => Some("เสียง"),
        SettingDisplay => Some("จอ"),
        SettingLanguage => Some("ภาษา"),
        DisplayWindow => Some("หน้า\u{200b}ต่าง"),
        Fullscreen => Some("เต็ม\u{200b}จอ"),
        LanguageSystem => Some("ระบบ"),
        ExtraLife => Some("+1 ชีวิต"),
        SectorName(s) => Some(SHORT_NAMES[s.index()]),
        ReadyEyebrow => Some("เซกเตอร์ {1}"),
        JourneyComplete => Some("สิ้นสุด"),
        _ => None,
    }
}

const NAMES: [&str; SECTOR_COUNT] = [
    "แสง\u{200b}แรก",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "ดาวเทียม",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "กระแส\u{200b}ลม",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "ครอส\u{200b}เฟด",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "การ\u{200b}สั่น\u{200b}พ้อง",
    "ปริซึม",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "คลื่น\u{200b}ใต้\u{200b}น้ำ",
    "Harvest",      // awaiting translation
    "Kaleidoscope", // awaiting translation
    "Tapestry",     // awaiting translation
    "Long Shadows", // awaiting translation
    "แสง\u{200b}สนธยา",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "พารัล\u{200b}แลกซ์",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "ซูเปอร์\u{200b}โนวา",
    "จันทร์\u{200b}ขึ้น",
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
    "กลับ\u{200b}บ้าน",
];

/// Sector names for narrow places: the band and a Compact page.
const SHORT_NAMES: [&str; SECTOR_COUNT] = [
    "แสง\u{200b}แรก",
    "Drift",     // awaiting translation
    "Horizon",   // awaiting translation
    "Glimmer",   // awaiting translation
    "Skylark",   // awaiting translation
    "Lanterns",  // awaiting translation
    "Tidewater", // awaiting translation
    "Sunrise",   // awaiting translation
    "ดาวเทียม",
    "Dewpoint", // awaiting translation
    "Aperture", // awaiting translation
    "Cloister", // awaiting translation
    "Keystone", // awaiting translation
    "Sundial",  // awaiting translation
    "Pinhole",  // awaiting translation
    "Windrose", // awaiting translation
    "กระแส\u{200b}ลม",
    "Filament", // awaiting translation
    "Meridian", // awaiting translation
    "Cascade",  // awaiting translation
    "ครอส\u{200b}เฟด",
    "Switchback", // awaiting translation
    "Solstice",   // awaiting translation
    "สั่น\u{200b}พ้อง",
    "ปริซึม",
    "Honeycomb", // awaiting translation
    "Spindrift", // awaiting translation
    "คลื่น",
    "Harvest",  // awaiting translation
    "Kaleido",  // awaiting translation
    "Tapestry", // awaiting translation
    "Shadows",  // awaiting translation
    "แสง\u{200b}สนธยา",
    "Chrysalis", // awaiting translation
    "Geode",     // awaiting translation
    "พารัล\u{200b}แลกซ์",
    "Citadel",  // awaiting translation
    "Nautilus", // awaiting translation
    "Vespers",  // awaiting translation
    "ซูเปอร์\u{200b}โนวา",
    "จันทร์\u{200b}ขึ้น",
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
    "กลับ\u{200b}บ้าน",
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
    "{icon:anchor} สมอ: รับ\u{200b}ลูก\u{200b}บอล เล็ง แล้ว\u{200b}ปล่อย",
    "Armored bricks take two hits; their rims count down", // awaiting translation
    "{icon:anchor} Aim through the gap in the wall",       // awaiting translation
    "One door in: send the ball into the courtyard",       // awaiting translation
    "Knock out the keystone and the arch is open",         // awaiting translation
    "Read the angles: each shadow is a shot",              // awaiting translation
    "{icon:anchor} One narrow pinhole: hold, aim, release", // awaiting translation
    "Every point of the compass leads to the heart",       // awaiting translation
    "แกน\u{200b}อำพัน: ทุก\u{200b}การ\u{200b}ระเบิด\u{200b}ส่ง\u{200b}ผล\u{200b}ถึง\u{200b}สี่\u{200b}ช่อง\u{200b}ข้าง\u{200b}เคียง",
    "Touch the filament anywhere: it burns both ways", // awaiting translation
    "Split the field down the meridian",               // awaiting translation
    "Start the cascade at either end",                 // awaiting translation
    "เปิด\u{200b}ทาง\u{200b}ผ่าน\u{200b}สอง\u{200b}แนว\u{200b}รีเลย์",
    "Three relay lines: one clean shot each", // awaiting translation
    "The ring burns whole: find a way to its edge", // awaiting translation
    "แกน\u{200b}ที่\u{200b}อยู่\u{200b}ติด\u{200b}กัน\u{200b}ส่ง\u{200b}ต่อ\u{200b}ปฏิกิริยา",
    "{icon:multi} หลาย\u{200b}ลูก: สาม\u{200b}ลูก หนึ่ง\u{200b}ช่อง\u{200b}ว่าง",
    "A core in every cell: crack them open", // awaiting translation
    "Spray everywhere: let three balls loose", // awaiting translation
    "ทะลวง\u{200b}เข้า\u{200b}ช่อง\u{200b}หลัง\u{200b}เกราะ",
    "A full field: {icon:multi} Multiball reaps it fast", // awaiting translation
    "Mirrors everywhere: break one side, then its twin",  // awaiting translation
    "Pull one thread and the weave comes loose",          // awaiting translation
    "Break through the floor and light the long fuse",    // awaiting translation
    "{icon:phase} เฟส: ชน\u{200b}อิฐ\u{200b}สาม\u{200b}ครั้ง\u{200b}โดย\u{200b}ไม่\u{200b}เด้ง",
    "{icon:phase} Phase slips through the shell to the core", // awaiting translation
    "Crack a geode and its crystals light up",                // awaiting translation
    "เจาะ\u{200b}เปลือก แล้ว\u{200b}จุด\u{200b}ชนวน\u{200b}เส้น\u{200b}ทาง\u{200b}ด้าน\u{200b}ใน",
    "Through the gate, or through the wall with {icon:phase} Phase", // awaiting translation
    "Spiral in: the outer coil lights first",                        // awaiting translation
    "Three bells: ring each one from beneath",                       // awaiting translation
    "รับ\u{200b}ลูก\u{200b}ที่\u{200b}กลับ\u{200b}มา\u{200b}ขณะ\u{200b}ที่\u{200b}ลูก\u{200b}อื่น\u{200b}ยัง\u{200b}ลอย\u{200b}อยู่",
    "ตาม\u{200b}รีเลย์\u{200b}รอบ\u{200b}ใจ\u{200b}กลาง\u{200b}ที่\u{200b}เปิด\u{200b}โล่ง",
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
    "วง\u{200b}โคจร\u{200b}สุด\u{200b}ท้าย: ใช้\u{200b}ทุก\u{200b}ช่อง\u{200b}ว่าง\u{200b}ให้\u{200b}คุ้ม",
];
