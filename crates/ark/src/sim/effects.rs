//! Cosmetic state the renderer draws: particles, the name of a power just
//! picked up, and relay blast rings. None of it affects play, except that
//! particle bursts draw from the gameplay generator, so their counts decide
//! which capsules drop later.
use super::{power::Power, rng::Rng};
use crate::{clock::DT, field::CELLS, geom::V2};

/// Particles alive at once; the oldest is overwritten first.
pub const PARTICLES: usize = 384;
/// How long a picked-up power's name stays on screen, in ticks.
pub const NOTICE_TICKS: u32 = crate::clock::TICK_HZ * 2;
/// How long a relay blast's ring is drawn, in ticks.
pub const RELAY_FLASH_TICKS: u8 = 36;
/// Downward acceleration of particles, in pixels per second squared.
const GRAVITY: f32 = 180.0;

/// A spark from a hit or a pickup.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Particle {
    /// Position.
    pub pos: V2,
    /// Velocity in pixels per second.
    pub velocity: V2,
    /// Seconds left; dead at zero or below.
    pub life: f32,
    /// Palette row to colour it by.
    pub hue: usize,
}

/// The cosmetic pools.
#[derive(Clone, Copy, Debug)]
pub struct Effects {
    /// Every particle slot, alive or not.
    pub particles: [Particle; PARTICLES],
    /// The slot the next particle overwrites.
    cursor: usize,
    /// The power last picked up, while its name is shown.
    pub notice: Option<Power>,
    /// Ticks left to show `notice`.
    pub notice_ticks: u32,
    /// Ticks left of each cell's relay blast ring.
    pub relay_flash: [u8; CELLS],
}

impl Effects {
    /// Empty pools.
    pub(crate) const fn new() -> Self {
        Self {
            particles: [Particle {
                pos: V2::new(0.0, 0.0),
                velocity: V2::new(0.0, 0.0),
                life: 0.0,
                hue: 0,
            }; PARTICLES],
            cursor: 0,
            notice: None,
            notice_ticks: 0,
            relay_flash: [0; CELLS],
        }
    }

    /// Ages everything by one tick.
    pub(crate) fn tick(&mut self) {
        self.notice_ticks = self.notice_ticks.saturating_sub(1);
        for flash in &mut self.relay_flash {
            *flash = flash.saturating_sub(1);
        }
        for p in &mut self.particles {
            if p.life > 0.0 {
                p.life -= DT;
                p.pos += p.velocity * DT;
                p.velocity.y += GRAVITY * DT;
            }
        }
    }

    /// Sprays `count` particles from `pos`.
    pub(crate) fn burst(&mut self, rng: &mut Rng, pos: V2, hue: usize, count: usize) {
        for _ in 0..count {
            let angle = rng.next_f32() * core::f32::consts::TAU;
            let speed = 45.0 + rng.next_f32() * 160.0;
            let life = 0.3 + rng.next_f32() * 0.35;
            self.particles[self.cursor] = Particle {
                pos,
                velocity: V2::new(angle.cos(), angle.sin()) * speed,
                life,
                hue,
            };
            self.cursor = (self.cursor + 1) % PARTICLES;
        }
    }

    /// Shows `power`'s name for a moment.
    pub(crate) fn notice(&mut self, power: Power) {
        self.notice = Some(power);
        self.notice_ticks = NOTICE_TICKS;
    }
}
