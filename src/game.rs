use crate::physics::{Rect, V2, sweep_circle};

pub const WIDTH: f32 = 960.0;
pub const HEIGHT: f32 = 900.0;
pub const LEFT: f32 = 64.0;
pub const RIGHT: f32 = 896.0;
pub const TOP: f32 = 136.0;
pub const BOTTOM: f32 = 814.0;
pub const PADDLE_Y: f32 = 770.0;
pub const RADIUS: f32 = 7.0;
pub const TICK_HZ: u32 = 240;
pub const DT: f32 = 1.0 / TICK_HZ as f32;
pub const PADDLE_WIDTH: f32 = 118.0;
pub const COLS: usize = 12;
pub const ROWS: usize = 7;
pub const GRID_X: f32 = 96.0;
pub const GRID_Y: f32 = 190.0;
pub const CELL_W: f32 = 64.0;
pub const CELL_H: f32 = 32.0;
pub const MAX_BALLS: usize = 3;
pub use crate::levels::{LEVEL_COUNT, LEVELS};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Mode {
    #[default]
    Journey,
    Practice,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RoundSummary {
    pub ticks: u32,
    pub medals: u8,
    pub bonus: u32,
    pub best_combo: u32,
    pub life_earned: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Phase {
    Ready,
    Playing,
    Cleared,
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
    pub combo: u32,
}

impl Events {
    pub fn merge(&mut self, other: Self) {
        self.brick |= other.brick;
        self.paddle |= other.paddle;
        self.wall |= other.wall;
        self.lost |= other.lost;
        self.clear |= other.clear;
        self.pickup |= other.pickup;
        self.launch |= other.launch;
        self.combo = self.combo.max(other.combo);
    }
}

pub struct Game {
    pub phase: Phase,
    pub mode: Mode,
    pub summary: RoundSummary,
    pub sector_ticks: u32,
    pub run_ticks: u32,
    pub phase_ticks: u32,
    pub initial_bricks: usize,
    pub combo: u32,
    pub best_combo: u32,
    lost_in_sector: bool,
    rally_hits: u32,
    quiet_ticks: u32,
    since_drop: u32,
    advance_requested: bool,
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
            mode: Mode::Journey,
            summary: RoundSummary::default(),
            sector_ticks: 0,
            run_ticks: 0,
            phase_ticks: 0,
            initial_bricks: 0,
            best_combo: 0,
            lost_in_sector: false,
            rally_hits: 0,
            quiet_ticks: 0,
            since_drop: 0,
            advance_requested: false,
            balls: [Ball::default(); MAX_BALLS],
            bricks: [0; ROWS * COLS],
            particles: [Particle::default(); 384],
            drops: [Drop::default(); 12],
            paddle_x: WIDTH / 2.0,
            paddle_previous: WIDTH / 2.0,
            paddle_width: PADDLE_WIDTH,
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

    pub fn at(level: usize, mode: Mode) -> Self {
        let mut game = Self::new();
        game.level = level.min(LEVEL_COUNT - 1);
        game.mode = mode;
        game.load_level();
        game
    }

    fn load_level(&mut self) {
        self.bricks = crate::levels::layout(self.level);
        self.remaining = self.bricks.iter().filter(|&&hp| hp > 0).count();
        self.initial_bricks = self.remaining;
        self.sector_ticks = 0;
        self.lost_in_sector = false;
        self.best_combo = 0;
        self.since_drop = 0;
        self.particles.fill(Particle::default());
        self.reset_serve();
    }

    fn finish_sector(&mut self) {
        let clean = !self.lost_in_sector;
        let swift = self.sector_ticks <= LEVELS[self.level].par_seconds * TICK_HZ;
        let bonus = 1000 + u32::from(clean) * 500 + u32::from(swift) * 500;
        let life_earned =
            self.mode == Mode::Journey && (self.level + 1).is_multiple_of(4) && self.lives < 5;
        if life_earned {
            self.lives += 1;
        }
        self.score += bonus;
        self.summary = RoundSummary {
            ticks: self.sector_ticks,
            medals: 1 | (u8::from(clean) << 1) | (u8::from(swift) << 2),
            bonus,
            best_combo: self.best_combo,
            life_earned,
        };
        self.phase = if self.level + 1 == LEVEL_COUNT && self.mode == Mode::Journey {
            Phase::Victory
        } else {
            Phase::Cleared
        };
        self.phase_ticks = 0;
        self.advance_requested = false;
        self.events.clear = true;
        self.drops.fill(Drop::default());
    }

    fn reset_serve(&mut self) {
        self.phase = Phase::Ready;
        self.phase_ticks = 0;
        self.rally_hits = 0;
        self.quiet_ticks = 0;
        self.balls = [Ball::default(); MAX_BALLS];
        self.drops = [Drop::default(); 12];
        self.wide_time = 0.0;
        self.slow_time = 0.0;
        self.paddle_width = PADDLE_WIDTH;
        self.paddle_x = self
            .paddle_x
            .clamp(LEFT + PADDLE_WIDTH / 2.0, RIGHT - PADDLE_WIDTH / 2.0);
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
        (LEVELS[self.level].speed + self.rally_hits.min(20) as f32 * 4.0)
            * if self.slow_time > 0.0 { 0.74 } else { 1.0 }
    }

    pub fn launch_velocity(&self) -> V2 {
        let toward_center = if self.paddle_x > WIDTH / 2.0 + 40.0 {
            -1.0
        } else {
            1.0
        };
        V2::new(0.30 * toward_center, -0.954).normalized() * self.speed()
    }

    fn keep_ball_moving(velocity: V2) -> V2 {
        let speed = velocity.length();
        if speed < 1.0 {
            return velocity;
        }
        let mut direction = velocity * (1.0 / speed);
        if direction.y.abs() < 0.24 {
            direction.y = 0.24 * if direction.y < 0.0 { -1.0 } else { 1.0 };
            direction.x = (1.0 - direction.y * direction.y).sqrt() * direction.x.signum();
        }
        direction * speed
    }

    pub fn step(&mut self, input: &Input) {
        self.events = Events::default();
        self.phase_ticks = self.phase_ticks.saturating_add(1);
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
        if self.phase == Phase::Cleared {
            self.advance_requested |= input.launch;
            if self.mode == Mode::Journey
                && self.advance_requested
                && self.phase_ticks >= TICK_HZ / 2
            {
                self.level += 1;
                self.load_level();
            }
            return;
        }
        self.paddle_previous = self.paddle_x;
        let target = input
            .mouse_x
            .unwrap_or(self.paddle_x + input.axis * 980.0 * DT);
        self.paddle_x = target;
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
                self.balls[0].velocity = self.launch_velocity();
                self.events.launch = true;
            }
            return;
        }
        self.sector_ticks += 1;
        self.run_ticks += 1;
        self.quiet_ticks += 1;
        // A gentle, infrequent correction breaks exact vertical repeats after
        // several empty rallies, without changing normal player-directed shots.
        if self.quiet_ticks > TICK_HZ * 8 {
            for ball in &mut self.balls {
                if ball.active && ball.velocity.x.abs() < ball.velocity.length() * 0.08 {
                    let speed = ball.velocity.length();
                    let sign = if ball.pos.x > WIDTH / 2.0 { -1.0 } else { 1.0 };
                    ball.velocity =
                        V2::new(speed * 0.16 * sign, ball.velocity.y).normalized() * speed;
                }
            }
            self.quiet_ticks = 0;
        }
        self.wide_time = (self.wide_time - DT).max(0.0);
        let was_slow = self.slow_time > 0.0;
        self.slow_time = (self.slow_time - DT).max(0.0);
        self.paddle_width = if self.wide_time > 0.0 {
            174.0
        } else {
            PADDLE_WIDTH
        };
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
            self.finish_sector();
            return;
        }
        if !self.balls.iter().any(|b| b.active) {
            self.lives -= 1;
            self.lost_in_sector = true;
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
            if ball.velocity.y > 0.0 && ball.pos.y <= PADDLE_Y {
                // Only the upper half can save a ball; never scoop one from below.
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
                let angle = offset * 1.12;
                self.rally_hits += 1;
                ball.velocity = V2::new(angle.sin(), -angle.cos()) * self.speed();
                self.combo = 0;
                self.events.paddle = true;
            } else {
                ball.velocity = ball.velocity - normal * (2.0 * ball.velocity.dot(normal));
                // Preserve deliberately extreme velocities used by stress tests.
                if ball.velocity.length() < 2000.0 {
                    ball.velocity = Self::keep_ball_moving(ball.velocity);
                }
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
        self.quiet_ticks = 0;
        self.burst(pos, index / COLS, 10);
        if self.bricks[index] == 0 {
            self.remaining -= 1;
            self.combo += 1;
            self.best_combo = self.best_combo.max(self.combo);
            self.events.combo = self.combo;
            self.score += 100 + 25 * self.combo.min(8);
            self.since_drop += 1;
            if self.since_drop >= 7 || (self.since_drop >= 3 && self.random() < 0.18) {
                self.since_drop = 0;
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
                self.paddle_width = 174.0;
                self.paddle_x = self.paddle_x.clamp(LEFT + 87.0, RIGHT - 87.0);
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
                                velocity: Self::keep_ball_moving(V2::new(
                                    v.x * c - v.y * s,
                                    v.x * s + v.y * c,
                                )),
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
        assert_eq!(g.level, 0);
        assert_eq!(g.phase, Phase::Cleared);
        for _ in 0..TICK_HZ {
            g.step(&Input::default());
        }
        assert_eq!(g.phase, Phase::Cleared);
        g.step(&Input {
            launch: true,
            ..Input::default()
        });
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
        assert_eq!(g.paddle_width, 174.0);
        g.apply_power(Power::Slow);
        assert!((g.balls[0].velocity.length() - g.speed()).abs() < 0.001);
        g.slow_time = DT / 2.0;
        g.wide_time = DT / 2.0;
        g.step(&Input::default());
        assert_eq!(g.paddle_width, PADDLE_WIDTH);
        assert!((g.balls[0].velocity.length() - g.speed()).abs() < 0.001);
    }
    #[test]
    fn mouse_tracks_in_one_tick_and_keyboard_has_consistent_speed() {
        let mut g = Game::new();
        g.step(&Input {
            mouse_x: Some(700.0),
            ..Input::default()
        });
        assert_eq!(g.paddle_x, 700.0);
        assert_eq!(g.balls[0].pos.x, 700.0);
        g.step(&Input {
            axis: -1.0,
            ..Input::default()
        });
        assert!((g.paddle_x - (700.0 - 980.0 * DT)).abs() < 0.001);
    }
    #[test]
    fn chapter_rewards_and_medals_follow_actual_play() {
        let mut g = Game::at(3, Mode::Journey);
        g.phase = Phase::Playing;
        g.remaining = 0;
        g.bricks.fill(0);
        g.step(&Input::default());
        assert_eq!(g.summary.medals, 7);
        assert_eq!(g.summary.bonus, 2000);
        assert_eq!(g.lives, 4);
        assert!(g.summary.life_earned);
        let ticks = g.sector_ticks;
        for _ in 0..100 {
            g.step(&Input::default());
        }
        assert_eq!(g.sector_ticks, ticks);
        assert_eq!(g.lives, 4);
        let mut g = Game::at(0, Mode::Practice);
        g.phase = Phase::Playing;
        g.remaining = 0;
        g.bricks.fill(0);
        g.lost_in_sector = true;
        g.sector_ticks = LEVELS[0].par_seconds * TICK_HZ;
        g.step(&Input::default());
        assert_eq!(g.summary.medals, 1);
        assert_eq!(g.summary.bonus, 1000);
        for _ in 0..TICK_HZ {
            g.step(&Input {
                launch: true,
                ..Input::default()
            });
        }
        assert_eq!(g.phase, Phase::Cleared);
        assert_eq!(g.level, 0);
    }
    #[test]
    fn flat_rallies_are_corrected_without_changing_speed() {
        let v = Game::keep_ball_moving(V2::new(500.0, -1.0));
        assert!(v.y < -100.0);
        assert!((v.length() - 500.001).abs() < 0.01);
        let mut g = playing();
        g.balls[0].pos = V2::new(500.0, 600.0);
        g.balls[0].velocity = V2::new(0.0, -500.0);
        g.quiet_ticks = TICK_HZ * 8;
        g.step(&Input::default());
        assert!(g.balls[0].velocity.x < -50.0);
    }
    #[test]
    fn drop_cadence_has_a_bounded_dry_spell() {
        let mut g = playing();
        g.bricks.fill(1);
        g.remaining = ROWS * COLS;
        let mut dry = 0;
        for i in 0..ROWS * COLS {
            g.drops.fill(Drop::default());
            g.hit_brick(i, V2::new(400.0, 300.0));
            dry += 1;
            if g.drops.iter().any(|d| d.active) {
                assert!(dry <= 7);
                dry = 0;
            }
            assert!(dry < 7);
        }
    }
    #[test]
    fn deterministic_long_run_stays_finite() {
        let mut a = Game::new();
        let mut b = Game::new();
        let mut impacts = 0;
        for tick in 0..80000 {
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
