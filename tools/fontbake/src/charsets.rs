//! Which glyphs each atlas needs: every string in every table, in the
//! roles that set it, plus the figures and icons the game formats itself.
//! A glyph is usually one character; Thai bakes a consonant with its marks
//! as one, and Arabic bakes each letter's contextual form.
use crate::{
    script,
    spec::{self, Weight},
};
use ark_text::{Arg, Form, Locale, Role, TextId, write};
use std::collections::{BTreeMap, BTreeSet};
use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};

/// What a group's font draws, which decides how its text splits into glyphs.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Noto Sans: Latin, Greek, Cyrillic, figures; everything not local.
    Latin,
    Cjk,
    Thai,
    Arabic,
}

/// One data file: the locales whose text it draws, and its fonts.
pub struct Group {
    pub name: &'static str,
    pub locales: &'static [Locale],
    pub kind: Kind,
    /// Font file names for [`Weight::Regular`], [`Weight::Medium`] and
    /// [`Weight::Display`]. Only Latin has a Display cut; the other
    /// scripts' Medium stands in.
    pub fonts: [&'static str; 3],
}

pub const GROUPS: [Group; 7] = [
    Group {
        name: "latin",
        locales: &Locale::ALL,
        kind: Kind::Latin,
        fonts: [
            "NotoSans-Regular.ttf",
            "NotoSans-Medium.ttf",
            "NotoSansDisplay-Medium.ttf",
        ],
    },
    Group {
        name: "zh",
        locales: &[Locale::ZhHans],
        kind: Kind::Cjk,
        fonts: [
            "NotoSansSC-Regular.otf",
            "NotoSansSC-Medium.otf",
            "NotoSansSC-Medium.otf",
        ],
    },
    Group {
        name: "tw",
        locales: &[Locale::ZhHant],
        kind: Kind::Cjk,
        fonts: [
            "NotoSansTC-Regular.otf",
            "NotoSansTC-Medium.otf",
            "NotoSansTC-Medium.otf",
        ],
    },
    Group {
        name: "ja",
        locales: &[Locale::Ja],
        kind: Kind::Cjk,
        fonts: [
            "NotoSansJP-Regular.otf",
            "NotoSansJP-Medium.otf",
            "NotoSansJP-Medium.otf",
        ],
    },
    Group {
        name: "ko",
        locales: &[Locale::Ko],
        kind: Kind::Cjk,
        fonts: [
            "NotoSansKR-Regular.otf",
            "NotoSansKR-Medium.otf",
            "NotoSansKR-Medium.otf",
        ],
    },
    Group {
        name: "th",
        locales: &[Locale::Th],
        kind: Kind::Thai,
        fonts: [
            "NotoSansThai-Regular.ttf",
            "NotoSansThai-Medium.ttf",
            "NotoSansThai-Medium.ttf",
        ],
    },
    Group {
        name: "ar",
        locales: &[Locale::Ar],
        kind: Kind::Arabic,
        fonts: [
            "NotoSansArabicUI-Regular.ttf",
            "NotoSansArabicUI-Medium.ttf",
            "NotoSansArabicUI-Medium.ttf",
        ],
    },
];

/// A role in one of its cuts: body text has a strong one.
pub type Style = (Role, Weight);

/// The text a glyph draws: one character, or a Thai cluster.
pub type Unit = Vec<char>;

/// What one group needs baked.
pub struct Sets {
    /// Glyphs per style.
    pub units: BTreeMap<Style, BTreeSet<Unit>>,
    /// Adjacent single-character pairs per weight, for kerning.
    pub pairs: BTreeMap<Weight, BTreeSet<(char, char)>>,
}

/// Figures and punctuation the game formats into labels, values and
/// scores in any locale: digits, every locale's group separator, signs,
/// the times sign of a chain, and the ellipsis that ends the fit chain.
fn figures() -> BTreeSet<char> {
    let mut out: BTreeSet<char> = "0123456789+-/:·×…, ".chars().collect();
    for locale in Locale::ALL {
        let mut s = String::new();
        ark_text::grouped(&mut s, locale, 1_234_567).expect("a String accepts any text");
        out.extend(s.chars());
    }
    out
}

/// Every string as the game can show it: full and short forms, filled
/// with numbers wherever a slot takes one, with the styles it is set in.
fn strings(locale: Locale) -> Result<Vec<(Vec<Style>, String)>, String> {
    let mut out = Vec::new();
    for id in TextId::all() {
        let args = vec![Arg::Count(1_234_567); id.arity()];
        // Any string may also be set as body text.
        let mut styles: Vec<Style> = [id.role(), Role::Body]
            .iter()
            .chain(id.also())
            .map(|&r| (r, spec::style(r).1))
            .collect();
        if id.strong() {
            styles.push((Role::Body, spec::strong(Role::Body)));
        }
        for form in [Form::Full, Form::Short] {
            let mut s = String::new();
            write(&mut s, locale, form, id, &args).map_err(|e| e.to_string())?;
            if s.nfc().ne(s.chars()) {
                return Err(format!("{locale:?} {id:?} is not NFC: {s:?}"));
            }
            // Every string must arrive precomposed: only Thai marks are
            // baked onto their consonant; any other combining mark would
            // render detached.
            if let Some(mark) = s
                .chars()
                .find(|&c| is_combining_mark(c) && !script::is_thai_mark(c))
            {
                return Err(format!(
                    "{locale:?} {id:?} holds combining U+{:04X}",
                    mark as u32
                ));
            }
            out.push((styles.clone(), s));
        }
    }
    Ok(out)
}

/// The glyphs of `s` this group draws.
fn units(kind: Kind, s: &str) -> Vec<Unit> {
    let mut out = Vec::new();
    match kind {
        Kind::Latin => out.extend(
            s.chars()
                .filter(|&c| !spec::is_local(c) && c != '\u{200B}')
                .map(|c| vec![c]),
        ),
        Kind::Cjk => out.extend(s.chars().filter(|&c| spec::is_cjk(c)).map(|c| vec![c])),
        Kind::Thai => script::thai_clusters(s, |c| {
            if spec::is_local(c[0]) {
                out.push(c.to_vec());
            }
        }),
        Kind::Arabic => script::arabic_forms(s, |c| {
            if script::is_rtl(c) {
                out.push(vec![c]);
            }
        }),
    }
    out
}

pub fn collect(group: &Group) -> Result<Sets, String> {
    let mut units_by_style: BTreeMap<Style, BTreeSet<Unit>> = BTreeMap::new();
    let mut pairs: BTreeMap<Weight, BTreeSet<(char, char)>> = BTreeMap::new();
    let figures = figures();
    // Kerning needs both glyphs to be plain characters of the same font;
    // Thai clusters and Arabic forms are placed without it.
    let kerned = matches!(group.kind, Kind::Latin | Kind::Cjk);
    let owned = |c: char| units(group.kind, &c.to_string()) == [vec![c]];
    let mut add = |styles: &[Style], s: &str| {
        let mine = units(group.kind, s);
        let all: Vec<char> = s.chars().collect();
        for &style in styles {
            units_by_style
                .entry(style)
                .or_default()
                .extend(mine.iter().cloned());
            if kerned {
                pairs.entry(style.1).or_default().extend(
                    all.windows(2)
                        .filter(|w| owned(w[0]) && owned(w[1]))
                        .map(|w| (w[0], w[1])),
                );
            }
        }
    };
    for &locale in group.locales {
        for (styles, s) in strings(locale)? {
            add(&styles, &s);
        }
    }
    // The Settings sheet names the language being spoken in that language,
    // so each atlas holds its own locales' names.
    for &locale in group.locales.iter().filter(|&&l| l != Locale::Pseudo) {
        add(
            &[
                (Role::Caption, Weight::Regular),
                (Role::Body, Weight::Regular),
            ],
            locale.native_name(),
        );
    }
    if group.kind == Kind::Latin {
        // Every style, emphasised ones included, sets figures.
        let styles = spec::ROLES
            .into_iter()
            .flat_map(|r| [(r, spec::style(r).1), (r, spec::strong(r))]);
        for (role, weight) in styles {
            units_by_style
                .entry((role, weight))
                .or_default()
                .extend(figures.iter().map(|&c| vec![c]));
            let set = pairs.entry(weight).or_default();
            for &a in &figures {
                set.extend(figures.iter().map(|&b| (a, b)));
            }
        }
        // Button and key caps (A, B, X, M, F, [ ]) and capsule letters are
        // set as labels in any locale; the performance overlay is ASCII.
        let label = units_by_style
            .entry((Role::Label, Weight::Medium))
            .or_default();
        label.extend(('A'..='Z').chain("[]".chars()).map(|c| vec![c]));
        units_by_style
            .entry((Role::Body, Weight::Regular))
            .or_default()
            .extend((' '..='~').map(|c| vec![c]));
    }
    Ok(Sets {
        units: units_by_style,
        pairs,
    })
}
