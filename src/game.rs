use crate::physics::{Rect, V2, sweep_circle};

pub const WIDTH: f32 = 960.0;
pub const HEIGHT: f32 = 900.0;
pub const LEFT: f32 = 64.0;
pub const RIGHT: f32 = 896.0;
pub const TOP: f32 = 136.0;
pub const BOTTOM: f32 = 814.0;
pub const PADDLE_Y: f32 = 770.0;
pub const RADIUS: f32 = 7.0;
pub const DT: f32 = 1.0 / 120.0;
pub const COLS: usize = 12;
pub const ROWS: usize = 7;
pub const GRID_X: f32 = 96.0;
pub const GRID_Y: f32 = 190.0;
pub const CELL_W: f32 = 64.0;
pub const CELL_H: f32 = 32.0;
pub const MAX_BALLS: usize = 3;
pub const LEVELS: [&str; 5] = [
    "FIRST CONTACT",
    "SIGNAL PATH",
    "DIAMOND ARRAY",
    "THE FORTRESS",
    "FINAL FREQUENCY",
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Phase {
    Ready,
    Playing,
    GameOver,
    Victory,
}

#[derive(Clone, Copy, Default)]
pub struct Ball {
    pub pos: V2,
    pub previous: V2,
    pub velocity: V2,
    pub active: bool,
}

#[derive(Clone, Copy, Default)]
pub struct Particle {
    pub pos: V2,
    pub velocity: V2,
    pub life: f32,
    pub hue: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Power {
    #[default]
    Wide,
    Slow,
    Multi,
}
impl Power {
    pub fn label(self) -> &'static str {
        match self {
            Self::Wide => "W",
            Self::Slow => "S",
            Self::Multi => "M",
        }
    }
}
#[derive(Clone, Copy, Default)]
pub struct Drop {
    pub pos: V2,
    pub power: Power,
    pub active: bool,
}

#[derive(Default)]
pub struct Input {
    pub axis: f32,
    pub mouse_x: Option<f32>,
    pub launch: bool,
}

#[derive(Default, Clone, Copy)]
pub struct Events {
    pub brick: bool,
    pub paddle: bool,
    pub wall: bool,
    pub lost: bool,
    pub clear: bool,
    pub pickup: bool,
    pub launch: bool,
}

pub struct Game {
    pub phase: Phase,
    pub balls: [Ball; MAX_BALLS],
    pub bricks: [u8; ROWS * COLS],
    pub particles: [Particle; 384],
    pub drops: [Drop; 12],
    pub paddle_x: f32,
    pub paddle_previous: f32,
    pub paddle_width: f32,
    pub score: u32,
    pub lives: u8,
    pub level: usize,
    pub remaining: usize,
    pub wide_time: f32,
    pub slow_time: f32,
    pub events: Events,
    pub collision_caps: u64,
    random: u32,
    particle_cursor: usize,
    combo: u32,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            phase: Phase::Ready,
            balls: [Ball::default(); MAX_BALLS],
            bricks: [0; ROWS * COLS],
            particles: [Particle::default(); 384],
            drops: [Drop::default(); 12],
            paddle_x: WIDTH / 2.0,
            paddle_previous: WIDTH / 2.0,
            paddle_width: 112.0,
            score: 0,
            lives: 3,
            level: 0,
            remaining: 0,
            wide_time: 0.0,
            slow_time: 0.0,
            events: Events::default(),
            collision_caps: 0,
            random: 0x51f15e77,
            particle_cursor: 0,
            combo: 0,
        };
        game.load_level();
        game
    }

    fn random(&mut self) -> f32 {
        self.random ^= self.random << 13;
        self.random ^= self.random >> 17;
        self.random ^= self.random << 5;
        self.random as f32 / u32::MAX as f32
    }

    pub fn brick_rect(index: usize) -> Rect {
        Rect {
            x: GRID_X + (index % COLS) as f32 * CELL_W,
            y: GRID_Y + (index / COLS) as f32 * CELL_H,
            w: 58.0,
            h: 24.0,
        }
    }

    fn load_level(&mut self) {
        for row in 0..ROWS {
            for col in 0..COLS {
                let filled = match self.level {
                    0 => true,
                    1 => (row + col) % 3 != 0,
                    2 => (col as i32 * 2 - 11).abs() + (row as i32 - 3).abs() * 2 < 13,
                    3 => row < 2 || !(2..COLS - 2).contains(&col) || row == 5,
                    _ => (row + col) % 2 == 0 || row == 0 || row == ROWS - 1,
                };
                self.bricks[row * COLS + col] = if !filled {
                    0
                } else if self.level > 0 && (row + col + self.level).is_multiple_of(4) {
                    2
                } else {
                    1
                };
            }
        }
        self.remaining = self.bricks.iter().filter(|&&hp| hp > 0).count();
        self.reset_serve();
    }

    fn reset_serve(&mut self) {
        self.phase = Phase::Ready;
        self.balls = [Ball::default(); MAX_BALLS];
        self.drops = [Drop::default(); 12];
        self.wide_time = 0.0;
        self.slow_time = 0.0;
        self.paddle_width = 112.0;
        self.paddle_x = self.paddle_x.clamp(LEFT + 56.0, RIGHT - 56.0);
        self.paddle_previous = self.paddle_x;
        self.combo = 0;
        let pos = V2::new(self.paddle_x, PADDLE_Y - RADIUS - 2.0);
        self.balls[0] = Ball {
            pos,
            previous: pos,
            active: true,
            ..Ball::default()
        };
    }

    pub fn speed(&self) -> f32 {
        (440.0 + self.level as f32 * 45.0) * if self.slow_time > 0.0 { 0.72 } else { 1.0 }
    }

    pub fn step(&mut self, input: &Input) {
        self.events = Events::default();
        for p in &mut self.particles {
            if p.life > 0.0 {
                p.life -= DT;
                p.pos += p.velocity * DT;
                p.velocity.y += 180.0 * DT;
            }
        }
        if matches!(self.phase, Phase::GameOver | Phase::Victory) {
            return;
        }
        self.paddle_previous = self.paddle_x;
        let target = input
            .mouse_x
            .unwrap_or(self.paddle_x + input.axis * 800.0 * DT);
        self.paddle_x += (target - self.paddle_x).clamp(-1400.0 * DT, 1400.0 * DT);
        self.paddle_x = self.paddle_x.clamp(
            LEFT + self.paddle_width / 2.0,
            RIGHT - self.paddle_width / 2.0,
        );
        if self.phase == Phase::Ready {
            let pos = V2::new(self.paddle_x, PADDLE_Y - RADIUS - 2.0);
            self.balls[0].pos = pos;
            self.balls[0].previous = pos;
            if input.launch {
                self.phase = Phase::Playing;
                self.balls[0].velocity = V2::new(0.38, -0.925).normalized() * self.speed();
                self.events.launch = true;
            }
            return;
        }
        self.wide_time = (self.wide_time - DT).max(0.0);
        let was_slow = self.slow_time > 0.0;
        self.slow_time = (self.slow_time - DT).max(0.0);
        self.paddle_width = if self.wide_time > 0.0 { 170.0 } else { 112.0 };
        if was_slow && self.slow_time == 0.0 {
            let speed = self.speed();
            for ball in &mut self.balls {
                ball.velocity = ball.velocity.normalized() * speed;
            }
        }
        for index in 0..MAX_BALLS {
            if self.balls[index].active {
                self.move_ball(index);
            }
        }
        if self.remaining == 0 {
            self.events.clear = true;
            if self.level + 1 == LEVELS.len() {
                self.phase = Phase::Victory;
            } else {
                self.level += 1;
                self.load_level();
            }
            return;
        }
        if !self.balls.iter().any(|b| b.active) {
            self.lives -= 1;
            self.events.lost = true;
            if self.lives == 0 {
                self.phase = Phase::GameOver;
            } else {
                self.reset_serve();
            }
            return;
        }
        self.update_drops();
    }

    fn move_ball(&mut self, index: usize) {
        let mut ball = self.balls[index];
        ball.previous = ball.pos;
        let mut remaining = DT;
        let mut elapsed = 0.0;
        // If a pathological tick uses the budget, keep the ball at its last safe
        // position. Never advance unchecked through geometry.
        for _ in 0..8 {
            if remaining < 0.000001 {
                break;
            }
            let delta = ball.velocity * remaining;
            let mut hit_t = 1.0;
            let mut normal = V2::default();
            let mut kind = 0; // 1 wall, 2 paddle, 3 brick, 4 drain
            let mut brick_index = 0;
            for (t, n, k) in [
                (
                    if delta.x < 0.0 {
                        (LEFT + RADIUS - ball.pos.x) / delta.x
                    } else {
                        2.0
                    },
                    V2::new(1.0, 0.0),
                    1,
                ),
                (
                    if delta.x > 0.0 {
                        (RIGHT - RADIUS - ball.pos.x) / delta.x
                    } else {
                        2.0
                    },
                    V2::new(-1.0, 0.0),
                    1,
                ),
                (
                    if delta.y < 0.0 {
                        (TOP + RADIUS - ball.pos.y) / delta.y
                    } else {
                        2.0
                    },
                    V2::new(0.0, 1.0),
                    1,
                ),
                (
                    if delta.y > 0.0 {
                        (BOTTOM + RADIUS - ball.pos.y) / delta.y
                    } else {
                        2.0
                    },
                    V2::default(),
                    4,
                ),
            ] {
                if t >= 0.0 && t <= hit_t {
                    hit_t = t;
                    normal = n;
                    kind = k;
                }
            }
            if ball.velocity.y > 0.0 {
                // Sweep against the moving paddle in its relative frame.
                let paddle_speed = (self.paddle_x - self.paddle_previous) / DT;
                let paddle = Rect {
                    x: self.paddle_previous + paddle_speed * elapsed - self.paddle_width / 2.0,
                    y: PADDLE_Y,
                    w: self.paddle_width,
                    h: 14.0,
                };
                if let Some(hit) = sweep_circle(
                    ball.pos,
                    delta - V2::new(paddle_speed * remaining, 0.0),
                    RADIUS,
                    paddle,
                ) && hit.t <= hit_t
                {
                    hit_t = hit.t;
                    normal = hit.normal;
                    kind = 2;
                }
            }
            let end = ball.pos + delta;
            let c0 = (((ball.pos.x.min(end.x) - RADIUS - GRID_X) / CELL_W).floor() as i32).max(0);
            let c1 = (((ball.pos.x.max(end.x) + RADIUS - GRID_X) / CELL_W).floor() as i32)
                .min(COLS as i32 - 1);
            let r0 = (((ball.pos.y.min(end.y) - RADIUS - GRID_Y) / CELL_H).floor() as i32).max(0);
            let r1 = (((ball.pos.y.max(end.y) + RADIUS - GRID_Y) / CELL_H).floor() as i32)
                .min(ROWS as i32 - 1);
            for row in r0..=r1 {
                for col in c0..=c1 {
                    let i = row as usize * COLS + col as usize;
                    if self.bricks[i] > 0
                        && let Some(hit) =
                            sweep_circle(ball.pos, delta, RADIUS, Self::brick_rect(i))
                        && hit.t <= hit_t
                    {
                        hit_t = hit.t;
                        normal = hit.normal;
                        kind = 3;
                        brick_index = i;
                    }
                }
            }
            ball.pos += delta * hit_t;
            elapsed += remaining * hit_t;
            remaining *= 1.0 - hit_t;
            if kind == 0 {
                remaining = 0.0;
                break;
            }
            if kind == 4 {
                ball.active = false;
                remaining = 0.0;
                break;
            }
            if kind == 2 {
                let paddle_at_hit =
                    self.paddle_previous + (self.paddle_x - self.paddle_previous) * (elapsed / DT);
                let offset =
                    ((ball.pos.x - paddle_at_hit) / (self.paddle_width / 2.0)).clamp(-1.0, 1.0);
                let angle = offset * 1.08;
                ball.velocity = V2::new(angle.sin(), -angle.cos()) * self.speed();
                self.combo = 0;
                self.events.paddle = true;
            } else {
                ball.velocity = ball.velocity - normal * (2.0 * ball.velocity.dot(normal));
                if kind == 3 {
                    self.hit_brick(brick_index, ball.pos);
                } else {
                    self.events.wall = true;
                }
            }
            ball.pos += normal * 0.01;
        }
        if remaining >= 0.000001 {
            self.collision_caps += 1;
        }
        self.balls[index] = ball;
    }

    fn hit_brick(&mut self, index: usize, pos: V2) {
        self.bricks[index] -= 1;
        self.events.brick = true;
        self.burst(pos, index / COLS, 10);
        if self.bricks[index] == 0 {
            self.remaining -= 1;
            self.combo += 1;
            self.score += 100 + 25 * self.combo.min(8);
            if self.random() < 0.14 {
                let power = match (self.random() * 3.0) as u32 {
                    0 => Power::Wide,
                    1 => Power::Slow,
                    _ => Power::Multi,
                };
                if let Some(drop) = self.drops.iter_mut().find(|d| !d.active) {
                    *drop = Drop {
                        pos,
                        power,
                        active: true,
                    };
                }
            }
        } else {
            self.score += 25;
        }
    }

    fn burst(&mut self, pos: V2, hue: usize, count: usize) {
        for _ in 0..count {
            let angle = self.random() * std::f32::consts::TAU;
            let speed = 45.0 + self.random() * 160.0;
            let life = 0.3 + self.random() * 0.35;
            self.particles[self.particle_cursor] = Particle {
                pos,
                velocity: V2::new(angle.cos(), angle.sin()) * speed,
                life,
                hue,
            };
            self.particle_cursor = (self.particle_cursor + 1) % self.particles.len();
        }
    }

    fn update_drops(&mut self) {
        for i in 0..self.drops.len() {
            if !self.drops[i].active {
                continue;
            }
            self.drops[i].pos.y += 155.0 * DT;
            let pos = self.drops[i].pos;
            if pos.y >= PADDLE_Y - 10.0
                && pos.y <= PADDLE_Y + 24.0
                && (pos.x - self.paddle_x).abs() < self.paddle_width / 2.0 + 12.0
            {
                let power = self.drops[i].power;
                self.drops[i].active = false;
                self.apply_power(power);
            } else if pos.y > BOTTOM {
                self.drops[i].active = false;
            }
        }
    }

    fn apply_power(&mut self, power: Power) {
        self.events.pickup = true;
        self.burst(V2::new(self.paddle_x, PADDLE_Y), 2, 24);
        match power {
            Power::Wide => {
                self.wide_time = 14.0;
                self.paddle_width = 170.0;
                self.paddle_x = self.paddle_x.clamp(LEFT + 85.0, RIGHT - 85.0);
            }
            Power::Slow => {
                self.slow_time = 12.0;
                let speed = self.speed();
                for b in &mut self.balls {
                    b.velocity = b.velocity.normalized() * speed;
                }
            }
            Power::Multi => {
                if let Some(source) = self.balls.iter().copied().find(|b| b.active) {
                    let mut n = 0;
                    for b in &mut self.balls {
                        if !b.active {
                            let angle: f32 = if n == 0 { -0.38 } else { 0.38 };
                            let (s, c) = angle.sin_cos();
                            let v = source.velocity;
                            *b = Ball {
                                velocity: V2::new(v.x * c - v.y * s, v.x * s + v.y * c),
                                ..source
                            };
                            n += 1;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn playing() -> Game {
        let mut g = Game::new();
        g.step(&Input {
            launch: true,
            ..Input::default()
        });
        g
    }
    #[test]
    fn high_speed_brick_collision_and_damage() {
        let mut g = playing();
        g.bricks.fill(0);
        g.bricks[0] = 1;
        g.bricks[COLS - 1] = 1;
        g.remaining = 2;
        g.balls[0].pos = V2::new(GRID_X + 29.0, GRID_Y + 80.0);
        g.balls[0].velocity = V2::new(0.0, -16000.0);
        g.step(&Input::default());
        assert_eq!(g.bricks[0], 0);
        assert!(g.score > 0);
        assert!(g.balls[0].velocity.y > 0.0);
        assert_eq!(g.collision_caps, 0);
    }
    #[test]
    fn paddle_edges_steer_and_center_is_vertical() {
        for offset in [-45.0, 0.0, 45.0] {
            let mut g = playing();
            g.balls[0].pos = V2::new(g.paddle_x + offset, PADDLE_Y - 8.0);
            g.balls[0].velocity = V2::new(0.0, 500.0);
            g.step(&Input::default());
            assert!(g.balls[0].velocity.y < 0.0);
            assert!((g.balls[0].velocity.length() - g.speed()).abs() < 0.001);
            assert_eq!(g.balls[0].velocity.x.signum(), offset.signum());
        }
    }
    #[test]
    fn moving_paddle_cannot_teleport_to_intercept() {
        let mut g = playing();
        g.balls[0].pos = V2::new(g.paddle_x + 70.0, PADDLE_Y + 20.0);
        g.balls[0].velocity = V2::new(0.0, 600.0);
        g.step(&Input {
            mouse_x: Some(RIGHT),
            ..Input::default()
        });
        assert!(!g.events.paddle);
    }
    #[test]
    fn moving_paddle_catches_a_ball_in_its_actual_path() {
        let mut g = playing();
        g.balls[0].pos = V2::new(g.paddle_x + 70.0, PADDLE_Y - 7.5);
        g.balls[0].velocity = V2::new(0.0, 600.0);
        g.step(&Input {
            mouse_x: Some(RIGHT),
            ..Input::default()
        });
        assert!(g.events.paddle);
        assert!(g.balls[0].velocity.y < 0.0);
    }
    #[test]
    fn simultaneous_wall_corner_reflects_both_axes() {
        let mut g = playing();
        g.balls[0].pos = V2::new(LEFT + RADIUS + 1.0, TOP + RADIUS + 1.0);
        g.balls[0].velocity = V2::new(-500.0, -500.0);
        g.step(&Input::default());
        assert!(g.balls[0].velocity.x > 0.0 && g.balls[0].velocity.y > 0.0);
        assert!(g.balls[0].pos.x >= LEFT + RADIUS && g.balls[0].pos.y >= TOP + RADIUS);
        assert_eq!(g.collision_caps, 0);
    }
    #[test]
    fn exhausted_budget_keeps_last_safe_position() {
        let mut g = playing();
        g.balls[0].pos = V2::new(WIDTH / 2.0, 600.0);
        g.balls[0].velocity = V2::new(1_000_000_000.0, 0.0);
        g.step(&Input::default());
        assert_eq!(g.collision_caps, 1);
        assert!(g.balls[0].pos.x >= LEFT + RADIUS && g.balls[0].pos.x <= RIGHT - RADIUS);
        assert_eq!(g.balls[0].pos.y, 600.0);
    }
    #[test]
    fn life_lost_only_after_last_ball() {
        let mut g = playing();
        g.balls[1] = g.balls[0];
        g.balls[0].pos = V2::new(LEFT + 30.0, BOTTOM + RADIUS - 1.0);
        g.balls[0].velocity = V2::new(0.0, 500.0);
        g.step(&Input::default());
        assert_eq!(g.lives, 3);
        g.balls[1].pos = g.balls[0].pos;
        g.balls[1].velocity = V2::new(0.0, 500.0);
        g.step(&Input::default());
        assert_eq!(g.lives, 2);
        assert_eq!(g.phase, Phase::Ready);
    }
    #[test]
    fn next_level_and_victory() {
        let mut g = playing();
        g.bricks.fill(0);
        g.remaining = 0;
        g.step(&Input::default());
        assert_eq!(g.level, 1);
        assert_eq!(g.phase, Phase::Ready);
        g.level = LEVELS.len() - 1;
        g.phase = Phase::Playing;
        g.bricks.fill(0);
        g.remaining = 0;
        g.step(&Input::default());
        assert_eq!(g.phase, Phase::Victory);
    }
    #[test]
    fn powers_extend_expire_and_spawn() {
        let mut g = playing();
        g.apply_power(Power::Multi);
        assert_eq!(g.balls.iter().filter(|b| b.active).count(), 3);
        g.apply_power(Power::Wide);
        assert_eq!(g.paddle_width, 170.0);
        g.apply_power(Power::Slow);
        assert!((g.balls[0].velocity.length() - g.speed()).abs() < 0.001);
        g.slow_time = DT / 2.0;
        g.wide_time = DT / 2.0;
        g.step(&Input::default());
        assert_eq!(g.paddle_width, 112.0);
        assert!((g.balls[0].velocity.length() - g.speed()).abs() < 0.001);
    }
    #[test]
    fn deterministic_long_run_stays_finite() {
        let mut a = Game::new();
        let mut b = Game::new();
        let mut impacts = 0;
        for tick in 0..40000 {
            if matches!(a.phase, Phase::GameOver | Phase::Victory) {
                a = Game::new();
                b = Game::new();
            }
            let x = a
                .balls
                .iter()
                .find(|b| b.active)
                .map_or(WIDTH / 2.0, |b| b.pos.x)
                + (tick as f32 * 0.003).sin() * 36.0;
            let input = Input {
                mouse_x: Some(x),
                launch: true,
                ..Input::default()
            };
            a.step(&input);
            b.step(&input);
            impacts += u32::from(a.events.brick);
            assert_eq!(a.phase, b.phase);
            assert_eq!(a.score, b.score);
            assert_eq!(a.collision_caps, 0);
            for (ball, other) in a.balls.iter().zip(&b.balls) {
                assert!(ball.pos.x.is_finite() && ball.pos.y.is_finite());
                assert_eq!(ball.pos, other.pos);
                assert_eq!(ball.velocity, other.velocity);
            }
        }
        assert_eq!(a.score, b.score);
        assert_eq!(a.bricks, b.bricks);
        assert_eq!(a.collision_caps, 0);
        assert!(impacts > 100);
    }
}
