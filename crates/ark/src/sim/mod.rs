//! The rules, one tick at a time. The submodules are private; the crate root
//! re-exports what callers need.
mod ball;
mod board;
mod capsules;
mod effects;
mod events;
mod game;
mod paddle;
mod power;
mod rng;
mod summary;

pub use ball::Ball;
pub use board::Board;
pub use capsules::Capsule;
pub use effects::{Effects, NOTICE_TICKS, PARTICLES, Particle, RELAY_FLASH_TICKS};
pub use events::Events;
pub use game::{Game, Input, Mode, Stage};
pub use paddle::Paddle;
pub use power::{Power, PowerState};
pub use summary::{Medals, SectorSummary};
