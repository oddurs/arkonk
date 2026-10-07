//! Every rule constant, named. Distances are world pixels, speeds pixels per
//! second, and durations ticks unless the name says seconds. Changing any of
//! these changes the game, and the golden replays will say so.
use crate::{clock::TICK_HZ, game::Power, geom::V2, sectors::SECTOR_COUNT};

// The paddle.

/// Normal paddle width.
pub const PADDLE_WIDTH: f32 = 118.0;
/// Paddle width while Wide is active.
pub const WIDE_PADDLE_WIDTH: f32 = 174.0;
/// Paddle thickness; only its top edge can return a ball.
pub const PADDLE_HEIGHT: f32 = 14.0;
/// How fast keys or a full stick deflection move the paddle.
pub const KEYBOARD_SPEED: f32 = 980.0;
/// The bounce angle from vertical at the paddle's very edge, in radians.
/// The angle grows linearly from the centre, which sends the ball straight up.
pub const MAX_BOUNCE_ANGLE: f32 = 1.12;
/// Anchor holds a ball no farther out than this fraction of the half-width,
/// so a release never leaves at the steepest edge angle.
pub const HOLD_OFFSET_LIMIT: f32 = 0.85;

// Serving.

/// Gap between a served or held ball and the paddle's top edge.
pub const SERVE_GAP: f32 = 2.0;
/// Serve direction before normalizing, aimed right; mirrored to aim left
/// when the paddle is right of [`LAUNCH_CENTER_BIAS`].
pub const LAUNCH_DIR: V2 = V2::new(0.30, -0.954);
/// How far right of the field's centre the paddle may be and still serve
/// rightward; beyond it the serve aims back toward the centre.
pub const LAUNCH_CENTER_BIAS: f32 = 40.0;

// Ball speed and angle.

/// Speed added by each paddle return in a rally.
pub const RALLY_SPEEDUP: f32 = 4.0;
/// Paddle returns that still add speed.
pub const RALLY_CAP: u32 = 20;
/// Speed multiplier while Slow is active.
pub const SLOW_FACTOR: f32 = 0.74;
/// The least vertical component a bounce may leave with, as a fraction of
/// speed, so a ball never settles into a flat, endless rally.
pub const MIN_VERTICAL: f32 = 0.24;
/// Bounces faster than this keep their exact angle. Only stress tests reach
/// it; the steepening would otherwise distort their deliberate velocities.
pub const STEEPEN_BELOW_SPEED: f32 = 2000.0;
/// Ticks without a brick hit before near-vertical balls are nudged.
pub const ANTI_STALL_TICKS: u32 = TICK_HZ * 8;
/// A ball is near-vertical when its sideways speed is below this fraction of
/// its speed.
pub const ANTI_STALL_NEAR_VERTICAL: f32 = 0.08;
/// The sideways component, as a fraction of speed, that the nudge adds.
pub const ANTI_STALL_STEER: f32 = 0.16;

// Powers.

/// How long Wide lasts, in seconds.
pub const WIDE_SECONDS: f32 = 14.0;
/// How long Slow lasts, in seconds.
pub const SLOW_SECONDS: f32 = 12.0;
/// Multi's new balls leave at this angle either side of the original, in
/// radians.
pub const MULTI_SPLIT_ANGLE: f32 = 0.38;
/// Catches an Anchor capsule grants.
pub const ANCHOR_CHARGES: u8 = 3;
/// Brick contacts each ball passes through after a Phase capsule.
pub const PHASE_CONTACTS: u8 = 3;

// Capsule drops.

/// How fast a capsule falls.
pub const DROP_SPEED: f32 = 155.0;
/// How far above the paddle's top edge a capsule is caught.
pub const CATCH_ABOVE: f32 = 10.0;
/// How far below the paddle's top edge a capsule is still caught.
pub const CATCH_BELOW: f32 = 24.0;
/// Extra reach beyond each end of the paddle for catching a capsule.
pub const CATCH_MARGIN: f32 = 12.0;
/// The direct break that drops the sector's opening capsule.
pub const OPENING_BREAK: u32 = 2;
/// Direct breaks after which a capsule always drops.
pub const DROP_DRY_SPELL: u32 = 7;
/// Direct breaks since the last capsule before a random drop is possible.
pub const DROP_MIN_GAP: u32 = 3;
/// The chance of a random drop once [`DROP_MIN_GAP`] has passed.
pub const DROP_CHANCE: f32 = 0.18;
/// Random drops pick from this list, in unlock order.
pub const DROP_ORDER: [Power; 5] = [
    Power::Wide,
    Power::Slow,
    Power::Anchor,
    Power::Multi,
    Power::Phase,
];
/// How many of [`DROP_ORDER`] each sector's random drops can be.
pub const POWER_UNLOCKS: [usize; SECTOR_COUNT] = [2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5];

// Relays.

/// Ticks from a core's destruction to its blast, per hop of a chain
/// (about 42 ms, independent of the display's refresh rate).
pub const RELAY_TICKS: u8 = 10;

// Pacing and assists.

/// A cleared sector's results stay up at least this long before a launch
/// press advances.
pub const ADVANCE_DELAY_TICKS: u32 = TICK_HZ / 2;
/// The finishing assist offers one Anchor catch when at most this many
/// bricks remain and none has broken for [`ASSIST_STALL_TICKS`].
pub const ASSIST_BRICKS: usize = 2;
/// See [`ASSIST_BRICKS`].
pub const ASSIST_STALL_TICKS: u32 = TICK_HZ * 12;

// Scoring.

/// Points for breaking a brick, before the chain bonus.
pub const SCORE_BREAK: u32 = 100;
/// Chain bonus per consecutive break since the last paddle return.
pub const SCORE_COMBO_STEP: u32 = 25;
/// Breaks in a chain that still raise its bonus.
pub const COMBO_CAP: u32 = 8;
/// Points for damaging an armoured brick without breaking it.
pub const SCORE_CHIP: u32 = 25;
/// Points for clearing a sector.
pub const CLEAR_BONUS: u32 = 1000;
/// Points for each of the Clean and Swift medals.
pub const MEDAL_BONUS: u32 = 500;

// Lives.

/// Lives at the start of a journey.
pub const START_LIVES: u8 = 3;
/// Most lives a chapter's extra life can reach.
pub const MAX_LIVES: u8 = 5;

// Collision and pools.

/// Contacts one ball may resolve in a tick. A pathological tick that uses
/// them all leaves the ball at its last safe position rather than letting it
/// advance unchecked through geometry.
pub const COLLISION_BUDGET: usize = 8;
/// How far a ball is pushed off a surface after a bounce, so the next sweep
/// starts clear of it.
pub const CONTACT_SKIN: f32 = 0.01;
/// Balls in play at once.
pub const MAX_BALLS: usize = 3;
/// Capsules falling at once.
pub const MAX_DROPS: usize = 12;
/// The random generator's starting state; any nonzero value works.
pub const RNG_SEED: u32 = 0x51f1_5e77;
