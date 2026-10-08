//! Which characters each atlas needs: every string in every table, in the
//! roles that set it, plus the figures and icons the game formats itself.
use crate::spec::{self, Weight};
use ark_text::{Arg, Form, Locale, Role, TextId, write};
use std::collections::{BTreeMap, BTreeSet};
use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};

/// One data file: the locales whose text it draws, and its fonts.
pub struct Group {
    pub name: &'static str,
    pub locales: &'static [Locale],
    /// Whether this group holds the CJK characters of its locales, or
    /// everything else.
    pub cjk: bool,
    /// Font file names for [`Weight::Regular`], [`Weight::Medium`] and
    /// [`Weight::Display`]. CJK has no Display cut; its Medium stands in.
    pub fonts: [&'static str; 3],
}

pub const GROUPS: [Group; 4] = [
    Group {
        name: "latin",
        locales: &Locale::ALL,
        cjk: false,
        fonts: [
            "NotoSans-Regular.ttf",
            "NotoSans-Medium.ttf",
            "NotoSansDisplay-Medium.ttf",
        ],
    },
    Group {
        name: "zh",
        locales: &[Locale::ZhHans],
        cjk: true,
        fonts: [
            "NotoSansSC-Regular.otf",
            "NotoSansSC-Medium.otf",
            "NotoSansSC-Medium.otf",
        ],
    },
    Group {
        name: "ja",
        locales: &[Locale::Ja],
        cjk: true,
        fonts: [
            "NotoSansJP-Regular.otf",
            "NotoSansJP-Medium.otf",
            "NotoSansJP-Medium.otf",
        ],
    },
    Group {
        name: "ko",
        locales: &[Locale::Ko],
        cjk: true,
        fonts: [
            "NotoSansKR-Regular.otf",
            "NotoSansKR-Medium.otf",
            "NotoSansKR-Medium.otf",
        ],
    },
];

/// A role in one of its cuts: body text has a strong one.
pub type Style = (Role, Weight);

/// What one group needs baked.
pub struct Sets {
    /// Characters per style.
    pub chars: BTreeMap<Style, BTreeSet<char>>,
    /// Adjacent pairs per weight, for kerning.
    pub pairs: BTreeMap<Weight, BTreeSet<(char, char)>>,
}

/// Figures and punctuation the game formats into labels, values and
/// scores in any locale: digits, every locale's group separator, signs,
/// and the times sign of a chain.
fn figures() -> BTreeSet<char> {
    let mut out: BTreeSet<char> = "0123456789+-/:·×, ".chars().collect();
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
            // Every string must arrive precomposed: the atlas has no mark
            // positioning, so a combining accent would render detached.
            if s.nfc().ne(s.chars()) {
                return Err(format!("{locale:?} {id:?} is not NFC: {s:?}"));
            }
            if let Some(mark) = s.chars().find(|&c| is_combining_mark(c)) {
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

pub fn collect(group: &Group) -> Result<Sets, String> {
    let mut chars: BTreeMap<Style, BTreeSet<char>> = BTreeMap::new();
    let mut pairs: BTreeMap<Weight, BTreeSet<(char, char)>> = BTreeMap::new();
    let mine = |c: char| spec::is_cjk(c) == group.cjk;
    let figures = figures();
    for &locale in group.locales {
        for (styles, s) in strings(locale)? {
            let all: Vec<char> = s.chars().collect();
            for style in styles {
                chars
                    .entry(style)
                    .or_default()
                    .extend(all.iter().copied().filter(|&c| mine(c)));
                pairs.entry(style.1).or_default().extend(
                    all.windows(2)
                        .filter_map(|w| (mine(w[0]) && mine(w[1])).then_some((w[0], w[1]))),
                );
            }
        }
    }
    // The Settings sheet names the language being spoken in that language,
    // so each atlas holds its own locales' names.
    for &locale in group.locales.iter().filter(|&&l| l != Locale::Pseudo) {
        let name = locale.native_name();
        for style in [
            (Role::Caption, Weight::Regular),
            (Role::Body, Weight::Regular),
        ] {
            chars
                .entry(style)
                .or_default()
                .extend(name.chars().filter(|&c| mine(c)));
        }
    }
    if !group.cjk {
        // Every style, emphasised ones included, sets figures.
        let styles = spec::ROLES
            .into_iter()
            .flat_map(|r| [(r, spec::style(r).1), (r, spec::strong(r))]);
        for (role, weight) in styles {
            chars
                .entry((role, weight))
                .or_default()
                .extend(figures.iter().copied());
            let set = pairs.entry(weight).or_default();
            for &a in &figures {
                set.extend(figures.iter().map(|&b| (a, b)));
            }
        }
        // Button and key caps (A, B, X, M, F, [ ]) and capsule letters are
        // set as labels in any locale; the performance overlay is ASCII.
        let label = chars.entry((Role::Label, Weight::Medium)).or_default();
        label.extend(('A'..='Z').chain("[]".chars()));
        let body = chars.entry((Role::Body, Weight::Regular)).or_default();
        body.extend(' '..='~');
    }
    Ok(Sets { chars, pairs })
}
