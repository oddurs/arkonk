//! The committed atlases against the string tables: every character every
//! string needs is baked at every size its role is drawn at.
use ark_glyphs::{Fonts, ICON_EM, Source, fonts, spec, supports};
use ark_text::{Arg, Form, Locale, Role, Script, TextId, write};
use std::collections::{BTreeMap, BTreeSet};

fn locales() -> impl Iterator<Item = Locale> {
    Locale::ALL.into_iter().filter(|&l| supports(l))
}

/// Glyph indices baked per (source, weight, ppem).
fn baked(fonts: &Fonts) -> BTreeMap<(Source, spec::Weight, u8), BTreeSet<u16>> {
    let mut out = BTreeMap::new();
    for (source, font) in [(Source::Latin, Some(fonts.latin)), (Source::Cjk, fonts.cjk)] {
        for face in font.iter().flat_map(|f| f.faces()) {
            for strike in face.strikes() {
                let mut buffer = vec![0; strike.unpacked_len()];
                let images = strike
                    .unpack(&mut buffer)
                    .expect("committed strikes inflate");
                let glyphs = images
                    .inspect(|i| {
                        assert_eq!(i.pixels.len(), usize::from(i.width) * usize::from(i.height))
                    })
                    .map(|i| i.glyph)
                    .collect();
                out.insert((source, face.weight, strike.ppem), glyphs);
            }
        }
    }
    out
}

#[test]
fn every_string_has_its_glyphs_at_every_size_it_is_drawn() {
    for locale in locales() {
        let fonts = fonts(locale).expect("committed atlases parse");
        let strikes = baked(&fonts);
        for id in TextId::all() {
            let args = vec![Arg::Count(1_234_567); id.arity()];
            for form in [Form::Full, Form::Short] {
                let mut text = String::new();
                write(&mut text, locale, form, id, &args).unwrap();
                let mut styles: Vec<_> = [id.role(), Role::Body]
                    .iter()
                    .chain(id.also())
                    .map(|&r| (r, spec::style(r).1))
                    .collect();
                if id.strong() {
                    styles.push((Role::Body, spec::strong(Role::Body)));
                }
                for (role, weight) in styles {
                    for ppem in spec::rungs(role) {
                        fonts.layout(&text, weight, ppem, 0.0, |p| {
                            let glyph = p.glyph.unwrap_or_else(|| {
                                panic!("{locale:?} {id:?}: no glyph for {:?}", p.c)
                            });
                            let set = &strikes[&(p.source, weight, ppem)];
                            assert!(
                                set.contains(&glyph),
                                "{locale:?} {id:?} {role:?}: {:?} missing at {ppem} px",
                                p.c
                            );
                        });
                    }
                }
            }
        }
    }
}

#[test]
fn ascii_digits_and_separators_are_everywhere() {
    let fonts = fonts(Locale::En).unwrap();
    let strikes = baked(&fonts);
    for role in spec::ROLES {
        let (_, weight, _) = spec::style(role);
        let face = fonts.latin.face(weight).unwrap();
        for ppem in spec::rungs(role) {
            for c in "0123456789+-:/,.\u{A0}\u{202F}".chars() {
                let glyph = face.glyph(c).unwrap();
                assert!(
                    strikes[&(Source::Latin, weight, ppem)].contains(&glyph),
                    "{c:?} {ppem}"
                );
            }
        }
    }
}

#[test]
fn figures_are_tabular() {
    let fonts = fonts(Locale::En).unwrap();
    for face in fonts.latin.faces() {
        let advances: BTreeSet<u16> = ('0'..='9')
            .map(|d| face.advance(face.glyph(d).unwrap()))
            .collect();
        assert_eq!(advances.len(), 1, "{:?}: {advances:?}", face.weight);
    }
}

#[test]
fn gpos_pairs_are_kerned_and_measured() {
    let fonts = fonts(Locale::En).unwrap();
    let face = fonts.latin.face(spec::Weight::Regular).unwrap();
    // "Your progress is saved" and the French "Temps cible".
    for pair in ["Yo", "Te"] {
        let mut chars = pair.chars();
        let (a, b) = (chars.next().unwrap(), chars.next().unwrap());
        let kern = face.kern(face.glyph(a).unwrap(), face.glyph(b).unwrap());
        assert!(kern < 0, "{pair} kerns by {kern}");
        let apart = fonts.measure(&a.to_string(), spec::Weight::Regular, 36, 0.0)
            + fonts.measure(&b.to_string(), spec::Weight::Regular, 36, 0.0);
        let together = fonts.measure(pair, spec::Weight::Regular, 36, 0.0);
        let expected = apart + f32::from(kern) * 36.0 / f32::from(face.units_per_em);
        assert!(
            (together - expected).abs() < 1e-3,
            "{pair}: {together} vs {expected}"
        );
    }
}

#[test]
fn the_pen_accumulates_unrounded() {
    let fonts = fonts(Locale::En).unwrap();
    let mut xs = Vec::new();
    let width = fonts.layout("iiiiiiii", spec::Weight::Regular, 13, 0.0, |p| xs.push(p.x));
    let face = fonts.latin.face(spec::Weight::Regular).unwrap();
    let step =
        f32::from(face.advance(face.glyph('i').unwrap())) * 13.0 / f32::from(face.units_per_em);
    assert!(step.fract() != 0.0, "the test needs a fractional advance");
    for (n, x) in xs.iter().enumerate() {
        assert!((x - step * n as f32).abs() < 1e-4);
    }
    assert!((width - step * 8.0).abs() < 1e-4);
}

#[test]
fn lines_wrap_by_script() {
    let en = fonts(Locale::En).unwrap();
    let mut lines = Vec::new();
    let text = "Catch a return while the other balls keep going";
    let full = en.measure(text, spec::Weight::Regular, 18, 0.0);
    en.wrap(text, spec::Weight::Regular, 18, 0.0, full * 0.6, |l| {
        lines.push(l)
    });
    assert_eq!(lines.len(), 2, "{lines:?}");
    assert_eq!(lines.join(" "), text);
    for line in &lines {
        assert!(en.measure(line, spec::Weight::Regular, 18, 0.0) <= full * 0.6);
    }
    if supports(Locale::Ja) {
        let ja = fonts(Locale::Ja).unwrap();
        assert_eq!(ja.script, Script::Ideographic);
        let text = "リトライはチェックポイントから再開します。次へ";
        let mut lines = Vec::new();
        let full = ja.measure(text, spec::Weight::Regular, 18, 0.0);
        ja.wrap(text, spec::Weight::Regular, 18, 0.0, full * 0.55, |l| {
            lines.push(l)
        });
        assert!(lines.len() >= 2, "{lines:?}");
        assert_eq!(lines.concat(), text);
        // Never a line starting with closing punctuation.
        assert!(lines.iter().all(|l| !l.starts_with('。')), "{lines:?}");
    }
}

#[test]
fn every_role_snaps_to_a_baked_size() {
    for role in spec::ROLES {
        for d in spec::DENSITIES {
            let ppem = spec::ppem(role, d).expect("checked densities use Noto");
            assert!(spec::rungs(role).any(|r| r == ppem));
        }
        // A tiny window falls back to the pixel font.
        assert_eq!(spec::ppem(role, 0.5), None);
    }
    assert_eq!(spec::ppem(Role::Body, 2.0), Some(40));
    assert_eq!(spec::ppem(Role::Label, 800.0 / 900.0), Some(13));
    // Steam Deck body text sits on the 18 px strike, lowercase just over 9 px.
    assert_eq!(spec::ppem(Role::Body, 800.0 / 900.0), Some(18));
    assert_eq!(spec::ppem(Role::Caption, 800.0 / 900.0), Some(14));
}

#[test]
fn capsule_icons_take_their_width_and_wrap_like_words() {
    let fonts = fonts(Locale::En).unwrap();
    let regular = spec::Weight::Regular;
    let icon = '\u{E000}';
    assert!(ark_text::icon_power(icon).is_some());
    let alone = fonts.measure(&icon.to_string(), regular, 20, 0.0);
    assert!((alone - ICON_EM * 20.0).abs() < 1e-4);
    let text = format!("{icon} Wide");
    let word = fonts.measure(" Wide", regular, 20, 0.0);
    assert!((fonts.measure(&text, regular, 20, 0.0) - alone - word).abs() < 1e-3);
    let mut lines = Vec::new();
    fonts.wrap(&text, regular, 20, 0.0, alone + 1.0, |l| lines.push(l));
    assert_eq!(lines, [icon.to_string().as_str(), "Wide"]);
}
