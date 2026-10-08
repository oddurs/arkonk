//! ARKONK's display text: the shipped [`Locale`]s, a typed [`TextId`] for
//! every visible string, one table per language, and formatting that puts
//! arguments, digit grouping and capsule icons where each language wants
//! them. It holds no font data and allocates nothing, so every port can
//! share it.
//!
//! ```
//! use ark_text::{Arg, Form, Locale, TextId, write};
//!
//! let mut s = String::new();
//! write(&mut s, Locale::De, Form::Full, TextId::Plus, &[Arg::Count(12500)]).unwrap();
//! assert_eq!(s, "+12.500");
//! ```
//!
//! Casing is authored, never computed: labels are written in capitals in
//! the tables, so German, Turkish or any later language gets its own
//! rules, and scripts without case are left alone.
#![no_std]

mod format;
mod id;
mod locale;
mod tables;

pub use format::{Arg, Form, capsule, grouped, short_template, template, write};
pub use id::{Role, TextId};
pub use locale::{Locale, Script};
