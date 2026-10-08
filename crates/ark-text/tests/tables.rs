//! Checks every table against the source: the same slots, authored casing,
//! no combining marks, and short forms that are actually shorter.
use ark::Power;
use ark_text::{
    Arg, Form, Locale, Role, Script, TextId, capsule, grouped, icon, icon_power, short_template,
    template, write, write_icons,
};

/// The `{…}` slots in a template, sorted.
fn slots(template: &str) -> Vec<&str> {
    let mut found = Vec::new();
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        let close = rest[open..].find('}').expect("an unclosed slot") + open;
        found.push(&rest[open + 1..close]);
        rest = &rest[close + 1..];
    }
    found.sort_unstable();
    found
}

fn real_locales() -> impl Iterator<Item = Locale> {
    Locale::ALL.into_iter().filter(|&l| l != Locale::Pseudo)
}

#[test]
fn every_translation_has_the_sources_slots() {
    for id in TextId::all() {
        let source = slots(template(Locale::En, id));
        let numbered = source.iter().filter(|s| s.parse::<usize>().is_ok()).count();
        assert_eq!(numbered, id.arity(), "{id:?} in English");
        for locale in real_locales() {
            let full = template(locale, id);
            assert!(!full.trim().is_empty(), "{locale:?} {id:?} is blank");
            assert_eq!(slots(full), source, "{locale:?} {id:?}");
            // A short form may leave a slot out to save room; it never adds one.
            if let Some(short) = short_template(locale, id) {
                assert!(!short.trim().is_empty(), "{locale:?} {id:?} short is blank");
                let mut left = source.clone();
                for slot in slots(short) {
                    let at = left.iter().position(|&s| s == slot);
                    let at = at.unwrap_or_else(|| panic!("{locale:?} {id:?} short adds {slot}"));
                    left.remove(at);
                }
            }
        }
    }
}

/// A short form is never longer than the full one. Where the full one is
/// already as short as the word can be ("Sound"), the short form repeats
/// it, so the fit chain finds one for every action and band item.
#[test]
fn short_forms_are_never_longer() {
    for locale in real_locales() {
        for id in TextId::all() {
            if let Some(short) = short_template(locale, id) {
                let full = template(locale, id);
                assert!(
                    short.chars().count() <= full.chars().count(),
                    "{locale:?} {id:?}: {short:?} is longer than {full:?}"
                );
            }
        }
    }
}

/// The fit chain's second step needs a short form for every action, for
/// everything the band shows, and for the help lines, settings and news.
#[test]
fn every_action_and_band_item_has_a_short_form() {
    use TextId::*;
    let ids = [
        ContinueJourney,
        NewJourney,
        SectorSelect,
        ActionResume,
        RetrySector,
        MainMenu,
        NextSector,
        BackToSectors,
        PlaySector,
        Settings,
        ActionServe,
        ActionRelease,
        ActionSelect,
        ActionBack,
        ActionAdjust,
        StatBest,
        StatMedals,
        SectorsOf,
        SectorsHeading,
        SaveFailed,
        LifeGained,
        KeyEsc,
        KeySpace,
        HelpResume,
        HelpRetry,
        HelpMainMenu,
        HelpSectors,
        HelpNewJourney,
        HelpSettings,
        SettingSound,
        SettingVolume,
        SettingDisplay,
        SettingLanguage,
        DisplayWindow,
        Fullscreen,
        LanguageSystem,
        ExtraLife,
        ReadyEyebrow,
    ]
    .into_iter()
    .chain(ark::sectors::SectorId::all().map(SectorName));
    for id in ids {
        for locale in real_locales() {
            assert!(
                short_template(locale, id).is_some(),
                "{locale:?} {id:?} has no short form"
            );
        }
    }
}

#[test]
fn labels_are_authored_in_capitals() {
    for locale in real_locales().filter(|l| l.script() == Script::Alphabetic) {
        for id in TextId::all().filter(|id| id.role() == Role::Label) {
            let text = template(locale, id);
            // `to_uppercase` turns ß into SS, so a label holding ß fails here:
            // capitals are the translator's call, not the code's.
            assert_eq!(text.to_uppercase(), text, "{locale:?} {id:?}");
        }
    }
}

#[test]
fn text_is_precomposed() {
    for locale in real_locales() {
        for id in TextId::all() {
            let text = template(locale, id);
            let combining = text
                .chars()
                .find(|c| matches!(u32::from(*c), 0x300..=0x36F | 0x1AB0..=0x1AFF | 0x1DC0..=0x1DFF | 0x20D0..=0x20FF | 0xFE20..=0xFE2F | 0x3099..=0x309A
                    // Arabic short vowels, shadda and Quranic marks.
                    | 0x0610..=0x061A | 0x064B..=0x065F | 0x0670 | 0x06D6..=0x06ED));
            assert_eq!(combining, None, "{locale:?} {id:?}: {text:?}");
        }
    }
}

fn format(locale: Locale, id: TextId, args: &[Arg]) -> String {
    let mut s = String::new();
    write(&mut s, locale, Form::Full, id, args).unwrap();
    s
}

#[test]
fn numbers_group_in_each_locales_style() {
    for (locale, n, expected) in [
        (Locale::En, 1_234_567, "1,234,567"),
        (Locale::En, 999, "999"),
        (Locale::En, 1000, "1,000"),
        (Locale::De, 1_234_567, "1.234.567"),
        (Locale::Fr, 12_500, "12\u{202F}500"),
        (Locale::Ru, 1000, "1\u{A0}000"),
        // CLDR minimum grouping digits of 2: four digits stay together.
        (Locale::EsEs, 1000, "1000"),
        (Locale::EsEs, 10_000, "10.000"),
        (Locale::Es419, 1000, "1,000"),
        // Italian also has CLDR minimum grouping digits of 2; this was once
        // written "1.000".
        (Locale::It, 1000, "1000"),
        (Locale::It, 12_000, "12.000"),
        (Locale::PtPt, 1000, "1000"),
        (Locale::PtPt, 12_000, "12\u{A0}000"),
        (Locale::Hu, 9999, "9999"),
        (Locale::Bg, 10_000, "10\u{A0}000"),
        (Locale::Sv, 1000, "1\u{A0}000"),
        (Locale::Tr, 1000, "1.000"),
        (Locale::Ar, 1000, "1,000"),
        (Locale::Pl, 9999, "9999"),
        (Locale::Pl, 12_000, "12\u{A0}000"),
        (Locale::Ja, 0, "0"),
        (Locale::En, u32::MAX, "4,294,967,295"),
    ] {
        let mut s = String::new();
        grouped(&mut s, locale, n).unwrap();
        assert_eq!(s, expected, "{locale:?} {n}");
    }
}

#[test]
fn arguments_follow_each_languages_word_order() {
    let sector = ark::sectors::SectorId::new(2).unwrap();
    assert_eq!(
        format(Locale::En, TextId::PlaySector, &[Arg::Sector(sector)]),
        "Play sector 03"
    );
    assert_eq!(
        format(Locale::De, TextId::PlaySector, &[Arg::Sector(sector)]),
        "Sektor 03 spielen"
    );
    assert_eq!(
        format(Locale::Ja, TextId::PlaySector, &[Arg::Sector(sector)]),
        "セクター 03 をプレイ"
    );
    let slipstream = ark::sectors::SectorId::from_slug("slipstream").unwrap();
    let saved = [Arg::Text(TextId::SectorName(slipstream)), Arg::Count(2450)];
    assert_eq!(
        format(Locale::En, TextId::ContinueDetail, &saved),
        "Slipstream · 2,450"
    );
    assert_eq!(
        format(Locale::Ja, TextId::ContinueDetail, &saved),
        "スリップストリーム・2,450"
    );
    let open = [Arg::Count(4), Arg::Count(12)];
    assert_eq!(
        format(Locale::Ko, TextId::SectorsOf, &open),
        "12개 섹터 중 4개"
    );
    assert_eq!(
        format(
            Locale::De,
            TextId::TargetBest,
            &[Arg::Clock(110), Arg::Clock(129)]
        ),
        "01:50 · Bestzeit 02:09"
    );
}

#[test]
fn capsule_letters_are_the_same_in_every_language() {
    for locale in Locale::ALL {
        for (sector, power) in [
            ("first_light", Power::Wide),
            ("satellites", Power::Anchor),
            ("afterglow", Power::Phase),
        ] {
            let id = TextId::SectorTip(ark::sectors::SectorId::from_slug(sector).unwrap());
            let tip = format(locale, id, &[]);
            assert!(
                tip.starts_with(capsule(power)) || tip.starts_with(&format!("[{}", capsule(power))),
                "{locale:?}: {tip}"
            );
        }
    }
}

#[test]
fn pseudo_locale_expands_accents_and_brackets() {
    let english = format(Locale::En, TextId::ContinueJourney, &[]);
    let pseudo = format(Locale::Pseudo, TextId::ContinueJourney, &[]);
    assert!(pseudo.starts_with('[') && pseudo.ends_with(']'), "{pseudo}");
    assert!(pseudo.contains("Çöñtíñüé"), "{pseudo}");
    let ratio = pseudo.chars().count() as f32 / english.chars().count() as f32;
    assert!((1.3..=1.6).contains(&ratio), "{pseudo}: {ratio}");
    // Numbers and icons are never accented.
    let score = format(Locale::Pseudo, TextId::Plus, &[Arg::Count(1500)]);
    assert!(score.starts_with("[+1,500"), "{score}");
    let tip = format(
        Locale::Pseudo,
        TextId::SectorTip(ark::sectors::SectorId::FIRST),
        &[],
    );
    assert!(tip.starts_with("[W ") && tip.contains(" Wídé"), "{tip}");
}

#[test]
fn icons_can_come_out_as_marks_for_the_renderer() {
    let tip = TextId::SectorTip(ark::sectors::SectorId::FIRST);
    for locale in Locale::ALL {
        let mut marked = String::new();
        write_icons(&mut marked, locale, Form::Full, tip, &[]).unwrap();
        let marks: Vec<_> = marked.chars().filter_map(icon_power).collect();
        assert_eq!(marks, [Power::Wide], "{locale:?}: {marked}");
        // Everything else reads as the lettered text does.
        let lettered: String = marked
            .chars()
            .map(|c| icon_power(c).map_or(c, capsule))
            .collect();
        assert_eq!(lettered, format(locale, tip, &[]));
    }
    for power in Power::ALL {
        assert_eq!(icon_power(icon(power)), Some(power));
    }
    assert_eq!(icon_power('W'), None);
}

#[test]
fn thai_marks_where_lines_may_break_and_nothing_else_does() {
    for id in TextId::all() {
        for locale in real_locales() {
            let text = template(locale, id);
            let has_break = text.contains('\u{200B}');
            assert!(
                !has_break || locale == Locale::Th,
                "{locale:?} {id:?} holds a zero width space"
            );
        }
        // Long Thai text must offer somewhere to wrap. An English
        // placeholder awaiting translation has its own spaces to break at.
        let thai = template(Locale::Th, id);
        let script = thai.chars().any(|c| ('\u{0E00}'..='\u{0E7F}').contains(&c));
        if script && thai.chars().count() > 12 {
            assert!(
                thai.contains('\u{200B}') || thai.contains(' '),
                "{id:?} cannot wrap: {thai}"
            );
        }
    }
}
