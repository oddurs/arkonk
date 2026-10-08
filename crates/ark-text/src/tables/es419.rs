//! Latin American Spanish: the Spain table with the words that differ.
//! It falls back to `es` on purpose; every other string reads the same in
//! both, so a new id is a compile error there, not a gap here.
use super::es;
use crate::TextId::{self, *};

pub(super) fn text(id: TextId) -> &'static str {
    match id {
        KeysMove => "Mouse o flechas para moverte",
        KeysPlay => "Enter para jugar",
        KeysContinue => "Enter o clic para continuar",
        PadMove => "Stick o pad direccional para moverte",
        PadBrowse => "Pad direccional para recorrer",
        MedalSwiftHow => "Supera el tiempo objetivo",
        _ => es::text(id),
    }
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    es::short(id)
}
