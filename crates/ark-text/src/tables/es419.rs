//! Latin American Spanish: the Spain table. Every string the game shows
//! today reads the same in both; a word that differs becomes an arm here
//! before falling back to `es`.
use super::es;
use crate::TextId;

pub(super) fn text(id: TextId) -> &'static str {
    es::text(id)
}

pub(super) fn short(id: TextId) -> Option<&'static str> {
    es::short(id)
}
