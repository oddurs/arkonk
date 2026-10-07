//! The ARKONK simulation: fixed-step rules, continuous collision, the authored
//! sectors and saved progress. It draws nothing, plays nothing and reads no
//! clock; the application drives it one tick at a time.
pub mod clock;
pub mod field;
pub mod geom;
pub mod profile;
pub mod sectors;
mod sim;
pub mod tuning;

pub use sim::*;
