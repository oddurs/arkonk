//! One table per shipped language. Each `text` is an exhaustive `match`,
//! so a new [`TextId`] does not compile until every language has it, and
//! sector lists are fixed-length arrays for the same reason.
use crate::{Locale, TextId};

mod ar;
mod bg;
mod cs;
mod da;
mod de;
mod el;
mod en;
mod es;
mod es419;
mod fi;
mod fr;
mod hu;
mod id;
mod it;
mod ja;
mod ko;
mod nb;
mod nl;
mod pl;
mod pt_br;
mod pt_pt;
mod ro;
mod ru;
mod sv;
mod th;
mod tr;
mod uk;
mod vi;
mod zh_hans;
mod zh_hant;

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
        Locale::Ar => ar::text(id),
        Locale::Bg => bg::text(id),
        Locale::ZhHant => zh_hant::text(id),
        Locale::Cs => cs::text(id),
        Locale::Da => da::text(id),
        Locale::Nl => nl::text(id),
        Locale::Fi => fi::text(id),
        Locale::El => el::text(id),
        Locale::Hu => hu::text(id),
        Locale::Id => id::text(id),
        Locale::Nb => nb::text(id),
        Locale::PtPt => pt_pt::text(id),
        Locale::Ro => ro::text(id),
        Locale::Sv => sv::text(id),
        Locale::Th => th::text(id),
        Locale::Tr => tr::text(id),
        Locale::Uk => uk::text(id),
        Locale::Vi => vi::text(id),
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
        Locale::Ar => ar::short(id),
        Locale::Bg => bg::short(id),
        Locale::ZhHant => zh_hant::short(id),
        Locale::Cs => cs::short(id),
        Locale::Da => da::short(id),
        Locale::Nl => nl::short(id),
        Locale::Fi => fi::short(id),
        Locale::El => el::short(id),
        Locale::Hu => hu::short(id),
        Locale::Id => id::short(id),
        Locale::Nb => nb::short(id),
        Locale::PtPt => pt_pt::short(id),
        Locale::Ro => ro::short(id),
        Locale::Sv => sv::short(id),
        Locale::Th => th::short(id),
        Locale::Tr => tr::short(id),
        Locale::Uk => uk::short(id),
        Locale::Vi => vi::short(id),
    }
}
