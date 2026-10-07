//! The ARKONK simulation: fixed-step rules, continuous collision, the authored
//! sectors and saved progress. It draws nothing, plays nothing and reads no
//! clock; the application drives it one tick at a time.
pub mod game;
pub mod levels;
pub mod physics;
pub mod profile;
pub mod timing;
