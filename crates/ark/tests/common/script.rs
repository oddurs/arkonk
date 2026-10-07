//! A seeded pilot that plays like a person who mostly watches the ball: it
//! tracks with the pointer, sometimes steers with keys, sometimes lets go,
//! and presses launch at random moments.
use super::splitmix::SplitMix64;
use ark::game::Input;

#[derive(Clone, Copy)]
enum Style {
    /// Pointer on the ball, offset to aim off the paddle's edges.
    Track(f32),
    /// Keyboard or stick deflection in `[-1, 1]`.
    Steer(f32),
    Idle,
}

pub struct Pilot {
    rng: SplitMix64,
    style: Style,
    left: u32,
}

impl Pilot {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: SplitMix64::new(seed),
            style: Style::Idle,
            left: 0,
        }
    }

    /// The next tick's input, given the first active ball's x, if any.
    pub fn input(&mut self, ball_x: Option<f32>) -> Input {
        if self.left == 0 {
            self.left = 30 + self.rng.below(240);
            self.style = match self.rng.below(16) {
                0 => Style::Steer(self.rng.signed_unit()),
                1 => Style::Idle,
                _ => Style::Track(self.rng.signed_unit() * 40.0),
            };
        }
        self.left -= 1;
        let launch = self.rng.below(24) == 0;
        match (self.style, ball_x) {
            (Style::Track(offset), Some(x)) => Input {
                mouse_x: Some(x + offset),
                launch,
                ..Input::default()
            },
            (Style::Steer(axis), _) => Input {
                axis,
                launch,
                ..Input::default()
            },
            _ => Input {
                launch,
                ..Input::default()
            },
        }
    }
}
