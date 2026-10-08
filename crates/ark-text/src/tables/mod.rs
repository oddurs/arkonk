//! One table per shipped language. Each `text` is an exhaustive `match`,
//! so a new [`TextId`] does not compile until every language has it, and
//! sector lists are fixed-length arrays for the same reason.
use crate::{Locale, TextId};

mod de;
mod en;
mod es;
mod es419;
mod fr;
mod it;
mod ja;
mod ko;
mod pl;
mod pt_br;
mod ru;
mod zh_hans;

pub(crate) fn text(locale: Locale, id: TextId) -> &'static str {
    match locale {
        Locale::En | Locale::Pseudo => en::text(id),
        Locale::Fr => fr::text(id),
        Locale::De => de::text(id),
        Locale::EsEs => es::text(id),
        Locale::Es419 => es419::text(id),
        Locale::PtBr => pt_br::text(id),
        Locale::It => it::text(id),
        Locale::Pl => pl::text(id),
        Locale::Ru => ru::text(id),
        Locale::ZhHans => zh_hans::text(id),
        Locale::Ja => ja::text(id),
        Locale::Ko => ko::text(id),
    }
}

pub(crate) fn short(locale: Locale, id: TextId) -> Option<&'static str> {
    match locale {
        Locale::En | Locale::Pseudo => en::short(id),
        Locale::Fr => fr::short(id),
        Locale::De => de::short(id),
        Locale::EsEs => es::short(id),
        Locale::Es419 => es419::short(id),
        Locale::PtBr => pt_br::short(id),
        Locale::It => it::short(id),
        Locale::Pl => pl::short(id),
        Locale::Ru => ru::short(id),
        Locale::ZhHans => zh_hans::short(id),
        Locale::Ja => ja::short(id),
        Locale::Ko => ko::short(id),
    }
}
