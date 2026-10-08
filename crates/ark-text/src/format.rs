//! Turns a template and its arguments into text, in a locale's conventions.
use crate::{Locale, TextId, tables};
use ark::{Power, sectors::SectorId};
use core::fmt::{self, Write};

/// A value for a template's `{n}` slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arg {
    /// A count or score, digit-grouped in the locale's style.
    Count(u32),
    /// A sector's number, two digits: `03`.
    Sector(SectorId),
    /// A duration in whole seconds, as `mm:ss`.
    Clock(u32),
    /// Another string, such as a sector or chapter name.
    Text(TextId),
}

/// Which wording of an id to use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Form {
    Full,
    /// A shorter wording for narrow places; ids without one use the full text.
    Short,
}

/// The capsule's letter: fixed iconography, like a Tetris piece's letter,
/// tied to muscle memory and guides. No string table can change it.
pub const fn capsule(power: Power) -> char {
    match power {
        Power::Wide => 'W',
        Power::Slow => 'S',
        Power::Multi => 'M',
        Power::Anchor => 'A',
        Power::Phase => 'P',
    }
}

/// The character standing for a capsule in text from [`write_icons`]: a
/// private-use code point that no font has, so a renderer can draw the
/// capsule itself where the text puts it.
pub const fn icon(power: Power) -> char {
    match power {
        Power::Wide => '\u{E000}',
        Power::Slow => '\u{E001}',
        Power::Multi => '\u{E002}',
        Power::Anchor => '\u{E003}',
        Power::Phase => '\u{E004}',
    }
}

/// The capsule an [`icon`] character stands for.
pub fn icon_power(c: char) -> Option<Power> {
    Power::ALL.into_iter().find(|&p| icon(p) == c)
}

/// The raw template for `id`. The pseudo locale's templates are English;
/// [`write`] transforms them.
pub fn template(locale: Locale, id: TextId) -> &'static str {
    tables::text(table(locale), id)
}

/// The short template for `id`, if the locale has one.
pub fn short_template(locale: Locale, id: TextId) -> Option<&'static str> {
    tables::short(table(locale), id)
}

fn table(locale: Locale) -> Locale {
    match locale {
        Locale::Pseudo => Locale::En,
        real => real,
    }
}

/// Writes `id` in `locale`, filling its slots from `args`. A short form
/// falls back to the full text. A slot without an argument writes nothing;
/// the table tests make sure every template uses exactly its id's slots.
pub fn write(
    out: &mut impl Write,
    locale: Locale,
    form: Form,
    id: TextId,
    args: &[Arg],
) -> fmt::Result {
    write_as(out, locale, form, id, args, false)
}

/// Like [`write`], but capsules come out as [`icon`] characters instead of
/// their letters, for a renderer that draws them.
pub fn write_icons(
    out: &mut impl Write,
    locale: Locale,
    form: Form,
    id: TextId,
    args: &[Arg],
) -> fmt::Result {
    write_as(out, locale, form, id, args, true)
}

fn write_as(
    out: &mut impl Write,
    locale: Locale,
    form: Form,
    id: TextId,
    args: &[Arg],
    icons: bool,
) -> fmt::Result {
    let text = match form {
        Form::Short => short_template(locale, id).unwrap_or_else(|| template(locale, id)),
        Form::Full => template(locale, id),
    };
    if locale != Locale::Pseudo {
        return expand(out, locale, text, args, icons, &mut None);
    }
    out.write_char('[')?;
    let mut letters = Some(0);
    expand(out, locale, text, args, icons, &mut letters)?;
    // About 40 % longer, as long German or Russian text would be.
    for i in 0..(letters.unwrap_or(0) * 2).div_ceil(5) {
        out.write_char(if i % 4 == 3 { ' ' } else { '·' })?;
    }
    out.write_char(']')
}

/// Copies `text`, replacing `{n}` and `{icon:slug}` slots. `letters` is
/// `Some` in the pseudo locale: literal text is then accented and counted.
fn expand(
    out: &mut impl Write,
    locale: Locale,
    text: &str,
    args: &[Arg],
    icons: bool,
    letters: &mut Option<usize>,
) -> fmt::Result {
    let mut rest = text;
    loop {
        let (literal, slot) = match rest.find('{') {
            Some(open) => match rest[open..].find('}') {
                Some(close) => {
                    let slot = &rest[open + 1..open + close];
                    let literal = &rest[..open];
                    rest = &rest[open + close + 1..];
                    (literal, Some(slot))
                }
                None => (core::mem::take(&mut rest), None),
            },
            None => (core::mem::take(&mut rest), None),
        };
        match letters {
            Some(count) => {
                for c in literal.chars() {
                    *count += usize::from(c.is_alphabetic());
                    out.write_char(accent(c))?;
                }
            }
            None => out.write_str(literal)?,
        }
        let Some(slot) = slot else { return Ok(()) };
        if let Some(slug) = slot.strip_prefix("icon:") {
            // Icons and numbers are never accented: they read the same everywhere.
            if let Some(power) = Power::ALL.into_iter().find(|p| p.slug() == slug) {
                out.write_char(if icons { icon(power) } else { capsule(power) })?;
            }
        } else if let Some(arg) = slot.parse::<usize>().ok().and_then(|i| args.get(i)) {
            match *arg {
                Arg::Count(n) => grouped(out, locale, n)?,
                Arg::Sector(s) => write!(out, "{:02}", s.index() + 1)?,
                Arg::Clock(seconds) => write!(out, "{:02}:{:02}", seconds / 60, seconds % 60)?,
                Arg::Text(id) => {
                    expand(out, locale, template(locale, id), &[], icons, letters)?;
                }
            }
        }
    }
}

/// Writes `n` with the locale's thousands separator.
pub fn grouped(out: &mut impl Write, locale: Locale, n: u32) -> fmt::Result {
    let (separator, minimum) = locale.grouping();
    let mut digits = [0_u8; 10];
    let mut len = 0;
    let mut rest = n;
    loop {
        digits[len] = b'0' + (rest % 10) as u8;
        len += 1;
        rest /= 10;
        if rest == 0 {
            break;
        }
    }
    for i in (0..len).rev() {
        out.write_char(char::from(digits[i]))?;
        if i > 0 && i % 3 == 0 && len >= minimum {
            out.write_char(separator)?;
        }
    }
    Ok(())
}

/// Accents letters with characters the real tables already need, so the
/// pseudo locale costs few extra glyphs.
fn accent(c: char) -> char {
    match c {
        'a' => 'á',
        'e' => 'é',
        'i' => 'í',
        'o' => 'ö',
        'u' => 'ü',
        'c' => 'ç',
        'n' => 'ñ',
        's' => 'ś',
        'z' => 'ż',
        'l' => 'ł',
        'A' => 'Á',
        'E' => 'É',
        'I' => 'Í',
        'O' => 'Ö',
        'U' => 'Ü',
        'C' => 'Ç',
        'N' => 'Ñ',
        'S' => 'Ś',
        'Z' => 'Ż',
        'L' => 'Ł',
        other => other,
    }
}
