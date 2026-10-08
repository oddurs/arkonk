//! `ark` is the ARKONK simulation: the rules, the authored sectors and the
//! progress codec. It draws nothing, plays nothing, reads no clock and opens
//! no file. A front end feeds it one [`Input`] per tick and draws what it
//! reads back.
//!
//! # The fixed step
//!
//! The game advances in ticks of 1/240 s ([`clock::TICK_HZ`]), whatever the
//! display does. [`clock::FixedClock`] turns frame time into whole ticks,
//! runs at most [`clock::MAX_CATCH_UP`] per frame (a long stall drops the
//! rest instead of fast-forwarding into a lost life), and keeps the leftover
//! fraction as `alpha` for interpolating ball positions.
//!
//! ```
//! use ark::{Game, Input, clock::FixedClock};
//!
//! let mut game = Game::new();
//! let mut clock = FixedClock::default();
//! // One 60 Hz frame: four ticks.
//! let frame = clock.advance(1.0 / 60.0);
//! for _ in 0..frame.ticks {
//!     let events = game.step(Input { launch: true, ..Input::default() });
//!     // Play sounds and effects for `events` here.
//!     let _ = events;
//! }
//! assert_eq!(frame.ticks, 4);
//! ```
//!
//! # Continuous collision
//!
//! Balls never step through geometry. Each tick, a ball's circle is swept
//! along its path ([`geom::sweep_circle_rect`]) against the walls, the live
//! bricks in the grid cells the path can reach ([`field::swept_cells`]) and
//! the paddle. Brick corners are exactly rounded, not expanded squares. The
//! paddle is swept in its own moving frame, so a fast paddle meets the ball
//! where both actually are and cannot teleport under it. After each bounce
//! the ball is pushed [`tuning::CONTACT_SKIN`] off the surface.
//!
//! A ball resolves at most [`tuning::COLLISION_BUDGET`] contacts per tick. A
//! pathological tick that spends them all leaves the ball at its last safe
//! position and counts it in [`Diagnostics::budget_exhausted`]; authored play
//! never does this, and the benchmark asserts it.
//!
//! # Fixed pools, no allocation
//!
//! Three balls, twelve capsules, an 84-cell board and fixed effect pools:
//! [`Game`] is plain data, and stepping it never allocates. The benchmark
//! (`cargo bench -p ark`) counts heap allocations while simulating and fails
//! on any.
//!
//! # Determinism
//!
//! The same inputs from the same start give the same game. Random drops come
//! from a seeded xorshift generator inside the game, and nothing reads a
//! clock or iterates a hash map. The golden replay test hashes the gameplay
//! state after every tick of scripted sessions and compares the digests with
//! recorded ones. The simulation is still `f32` and calls the platform's
//! `sin` and `cos`, so those digests are exact only on the platform and
//! optimization level that recorded them.
//!
//! # Content and text
//!
//! [`sectors`] holds the twelve layouts, parsed at compile time. `ark`
//! carries no display text: sectors, chapters and powers have stable slugs
//! and ids, and the front end owns the words.
//!
//! # Encapsulation and the sandbox
//!
//! [`Game`]'s state is private and changes only through [`Game::step`], so
//! the rules' bookkeeping (the remaining-brick count, held balls, timers)
//! cannot drift. Tests, benchmarks and demos that need a particular
//! situation use [`Game::sandbox`]: every [`Sandbox`] operation keeps that
//! bookkeeping consistent, and [`Game::validate`] checks it after any tick.
//!
//! # Progress
//!
//! [`progress`] holds the progression rules (unlocks, medal merges,
//! checkpoints) and the save file's text format. It reads bytes and writes
//! into any [`core::fmt::Write`]; where the file lives is the front end's
//! business.
pub mod clock;
pub mod field;
pub mod geom;
pub mod progress;
pub mod sectors;
mod sim;
pub mod tuning;

pub use sim::*;
