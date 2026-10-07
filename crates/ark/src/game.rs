use crate::{
    clock::{DT, TICK_HZ},
    field::{
        BALL_RADIUS as RADIUS, BOTTOM, CELLS, COLS, Cell, FIELD, LEFT, PADDLE_Y, RIGHT, TOP,
        cell_rect, swept_cells,
    },
    geom::{Rect, V2, sweep_circle_rect},
    sectors::SectorId,
};

pub const PADDLE_WIDTH: f32 = 118.0;
pub const MAX_BALLS: usize = 3;
pub const RELAY_TICKS: u8 = 10; // 42 ms per hop, independent of display refresh.

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
    pub held: bool,
    pub held_offset: f32,
    pub phase_hits: u8,
    pub phase_ignore: u128,
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
    Anchor,
    Phase,
}
impl Power {
    pub fn name(self) -> &'static str {
        match self {
            Self::Wide => "WIDE",
            Self::Slow => "SLOW",
            Self::Multi => "MULTIBALL",
            Self::Anchor => "ANCHOR",
            Self::Phase => "PHASE",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Wide => "W",
            Self::Slow => "S",
            Self::Multi => "M",
            Self::Anchor => "A",
            Self::Phase => "P",
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
    pub caught: bool,
    pub relay: bool,
    pub phase_hit: bool,
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
        self.caught |= other.caught;
        self.relay |= other.relay;
        self.phase_hit |= other.phase_hit;
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
    opening_collected: bool,
    direct_breaks: u32,
    last_brick_tick: u32,
    finish_assist_used: bool,
    pub anchor_charges: u8,
    pub notice: Option<Power>,
    pub notice_ticks: u32,
    pub cores: [bool; CELLS],
    pub relay_delay: [u8; CELLS],
    pub relay_flash: [u8; CELLS],
    pub balls: [Ball; MAX_BALLS],
    pub bricks: [u8; CELLS],
    pub particles: [Particle; 384],
    pub drops: [Drop; 12],
    pub paddle_x: f32,
    pub paddle_previous: f32,
    pub paddle_width: f32,
    pub score: u32,
    pub lives: u8,
    pub sector: SectorId,
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
            opening_collected: false,
            direct_breaks: 0,
            last_brick_tick: 0,
            finish_assist_used: false,
            anchor_charges: 0,
            notice: None,
            notice_ticks: 0,
            cores: [false; CELLS],
            relay_delay: [0; CELLS],
            relay_flash: [0; CELLS],
            balls: [Ball::default(); MAX_BALLS],
            bricks: [0; CELLS],
            particles: [Particle::default(); 384],
            drops: [Drop::default(); 12],
            paddle_x: FIELD.center().x,
            paddle_previous: FIELD.center().x,
            paddle_width: PADDLE_WIDTH,
            score: 0,
            lives: 3,
            sector: SectorId::FIRST,
            remaining: 0,
            wide_time: 0.0,
            slow_time: 0.0,
            events: Events::default(),
            collision_caps: 0,
            random: 0x51f15e77,
            particle_cursor: 0,
            combo: 0,
        };
        game.load_sector();
        game
    }

    fn random(&mut self) -> f32 {
        self.random ^= self.random << 13;
        self.random ^= self.random >> 17;
        self.random ^= self.random << 5;
        self.random as f32 / u32::MAX as f32
    }

    pub fn start(sector: SectorId, mode: Mode) -> Self {
        let mut game = Self::new();
        game.sector = sector;
        game.mode = mode;
        game.load_sector();
        game
    }

    fn load_sector(&mut self) {
        let layout = self.sector.sector().layout;
        self.bricks = layout.hp;
        for cell in Cell::all() {
            self.cores[cell.index()] = layout.cores.contains(cell);
        }
        self.relay_delay.fill(0);
        self.relay_flash.fill(0);
        self.opening_collected = false;
        self.direct_breaks = 0;
        self.last_brick_tick = 0;
        self.finish_assist_used = false;
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
        let swift = self.sector_ticks <= self.sector.sector().par_seconds * TICK_HZ;
        let bonus = 1000 + u32::from(clean) * 500 + u32::from(swift) * 500;
        let life_earned = self.mode == Mode::Journey
            && (self.sector.index() + 1).is_multiple_of(4)
            && self.lives < 5;
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
        self.phase = if self.sector.next().is_none() && self.mode == Mode::Journey {
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
        self.anchor_charges = 0;
        self.notice = None;
        self.notice_ticks = 0;
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
        (self.sector.sector().speed + self.rally_hits.min(20) as f32 * 4.0)
            * if self.slow_time > 0.0 { 0.74 } else { 1.0 }
    }

    pub fn launch_velocity(&self) -> V2 {
        let toward_center = if self.paddle_x > FIELD.center().x + 40.0 {
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
        self.notice_ticks = self.notice_ticks.saturating_sub(1);
        for flash in &mut self.relay_flash {
            *flash = flash.saturating_sub(1);
        }
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
                && let Some(next) = self.sector.next()
            {
                self.sector = next;
                self.load_sector();
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
        if self.remaining <= 2
            && self.remaining > 0
            && !self.finish_assist_used
            && self.sector_ticks.saturating_sub(self.last_brick_tick) >= TICK_HZ * 12
            && self.anchor_charges == 0
            && self.balls.iter().filter(|b| b.active).count() == 1
            && !self.balls.iter().any(|b| b.held)
        {
            self.anchor_charges = 1;
            self.finish_assist_used = true;
            self.notice = Some(Power::Anchor);
            self.notice_ticks = TICK_HZ * 2;
            self.events.pickup = true;
        }
        // A gentle, infrequent correction breaks exact vertical repeats after
        // several empty rallies, without changing normal player-directed shots.
        if self.quiet_ticks > TICK_HZ * 8 {
            for ball in &mut self.balls {
                if ball.active
                    && !ball.held
                    && ball.velocity.x.abs() < ball.velocity.length() * 0.08
                {
                    let speed = ball.velocity.length();
                    let sign = if ball.pos.x > FIELD.center().x {
                        -1.0
                    } else {
                        1.0
                    };
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
        self.sync_held_balls();
        for index in 0..MAX_BALLS {
            if !self.balls[index].active {
                continue;
            }
            if self.balls[index].held {
                if !input.launch {
                    continue;
                }
                self.balls[index].held = false;
                self.events.launch = true;
            }
            self.move_ball(index);
        }
        self.advance_relays();
        let pending_relay = self.relay_delay.iter().any(|&delay| delay > 0);
        if self.remaining == 0 && !pending_relay {
            self.finish_sector();
            return;
        }
        if !self.balls.iter().any(|b| b.active) && !pending_relay {
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
        self.sync_held_balls();
    }

    fn move_ball(&mut self, index: usize) {
        let mut ball = self.balls[index];
        let mut ignored = ball.phase_ignore;
        while ignored != 0 {
            let i = ignored.trailing_zeros() as usize;
            ignored &= ignored - 1;
            let Some(cell) = Cell::new(i) else { continue };
            let r = cell_rect(cell);
            if ball.pos.x < r.x - RADIUS
                || ball.pos.x > r.x + r.w + RADIUS
                || ball.pos.y < r.y - RADIUS
                || ball.pos.y > r.y + r.h + RADIUS
            {
                ball.phase_ignore &= !(1_u128 << i);
            }
        }
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
                if let Some(hit) = sweep_circle_rect(
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
            for cell in swept_cells(ball.pos, ball.pos + delta, RADIUS) {
                let i = cell.index();
                if self.bricks[i] > 0
                    && ball.phase_ignore & (1_u128 << i) == 0
                    && let Some(hit) = sweep_circle_rect(ball.pos, delta, RADIUS, cell_rect(cell))
                    && hit.t <= hit_t
                {
                    hit_t = hit.t;
                    normal = hit.normal;
                    kind = 3;
                    brick_index = i;
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
                if self.anchor_charges > 0 && !self.balls.iter().any(|b| b.active && b.held) {
                    self.anchor_charges -= 1;
                    ball.held = true;
                    ball.held_offset = offset.clamp(-0.85, 0.85);
                    let angle = ball.held_offset * 1.12;
                    ball.velocity = V2::new(angle.sin(), -angle.cos()) * self.speed();
                    ball.pos = V2::new(
                        self.paddle_x + ball.held_offset * self.paddle_width / 2.0,
                        PADDLE_Y - RADIUS - 2.0,
                    );
                    ball.previous = ball.pos;
                    ball.phase_ignore = 0;
                    self.events.caught = true;
                    remaining = 0.0;
                    break;
                }
            } else if kind == 3 && ball.phase_hits > 0 {
                ball.phase_hits -= 1;
                ball.phase_ignore |= 1_u128 << brick_index;
                self.hit_brick(brick_index, ball.pos);
                self.events.phase_hit = true;
                // Ignore this brick until fully outside it, including when the
                // third charge is spent inside reinforced material.
                continue;
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

    fn sync_held_balls(&mut self) {
        for ball in &mut self.balls {
            if ball.active && ball.held {
                ball.pos = V2::new(
                    self.paddle_x + ball.held_offset * self.paddle_width / 2.0,
                    PADDLE_Y - RADIUS - 2.0,
                );
                ball.previous = ball.pos;
            }
        }
    }

    fn hit_brick(&mut self, index: usize, pos: V2) {
        self.damage_brick(index, pos, true);
    }

    fn damage_brick(&mut self, index: usize, pos: V2, direct: bool) {
        if self.bricks[index] == 0 {
            return;
        }
        self.bricks[index] -= 1;
        self.events.brick = true;
        self.quiet_ticks = 0;
        self.last_brick_tick = self.sector_ticks;
        self.burst(pos, index / COLS, if direct { 10 } else { 4 });
        if self.bricks[index] == 0 {
            self.remaining -= 1;
            self.combo += 1;
            self.best_combo = self.best_combo.max(self.combo);
            self.events.combo = self.combo;
            self.score += 100 + 25 * self.combo.min(8);
            if self.cores[index] {
                self.relay_delay[index] = RELAY_TICKS;
            }
            // Relay damage rewards the shot without generating capsule storms.
            if direct {
                self.drop_from_hit(pos);
            }
        } else {
            self.score += 25;
        }
    }

    fn drop_from_hit(&mut self, pos: V2) {
        self.direct_breaks += 1;
        self.since_drop += 1;
        if self.direct_breaks == 2
            || self.since_drop >= 7
            || (self.since_drop >= 3 && self.random() < 0.18)
        {
            let power = if !self.opening_collected {
                self.sector.sector().opening
            } else {
                let count = match self.sector.index() {
                    0 => 2,
                    1..=3 => 3,
                    4..=7 => 4,
                    _ => 5,
                };
                [
                    Power::Wide,
                    Power::Slow,
                    Power::Anchor,
                    Power::Multi,
                    Power::Phase,
                ][((self.random() * count as f32) as usize).min(count - 1)]
            };
            if let Some(drop) = self.drops.iter_mut().find(|d| !d.active) {
                *drop = Drop {
                    pos,
                    power,
                    active: true,
                };
                self.since_drop = 0;
            }
        }
    }

    fn advance_relays(&mut self) {
        // Snapshot the due cells before propagation: traversal order cannot
        // change timing, and even a full board fits this fixed queue.
        let mut due = [false; CELLS];
        for (index, delay) in self.relay_delay.iter_mut().enumerate() {
            if *delay > 0 {
                *delay -= 1;
                due[index] = *delay == 0;
            }
        }
        for (index, ready) in due.into_iter().enumerate() {
            if !ready {
                continue;
            }
            self.events.relay = true;
            self.relay_flash[index] = 36;
            let Some(cell) = Cell::new(index) else {
                continue;
            };
            for neighbor in cell.neighbors().into_iter().flatten() {
                self.damage_brick(neighbor.index(), cell_rect(neighbor).center(), false);
            }
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

    pub fn apply_power(&mut self, power: Power) {
        self.events.pickup = true;
        self.notice = Some(power);
        self.notice_ticks = TICK_HZ * 2;
        if power == self.sector.sector().opening {
            self.opening_collected = true;
        }
        self.burst(V2::new(self.paddle_x, PADDLE_Y), 2, 24);
        match power {
            Power::Anchor => self.anchor_charges = 3,
            Power::Phase => {
                for ball in &mut self.balls {
                    if ball.active {
                        ball.phase_hits = 3;
                    }
                }
            }
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
                if let Some(source) = self
                    .balls
                    .iter()
                    .copied()
                    .find(|b| b.active && !b.held)
                    .or_else(|| self.balls.iter().copied().find(|b| b.active))
                {
                    let mut n = 0;
                    for b in &mut self.balls {
                        if !b.active {
                            let angle: f32 = if n == 0 { -0.38 } else { 0.38 };
                            let (s, c) = angle.sin_cos();
                            let v = source.velocity;
                            *b = Ball {
                                held: false,
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
    use crate::{
        field::{CELL_H, CELL_W, GRID_X, GRID_Y, ROWS},
        sectors::SECTOR_COUNT,
    };
    fn sector(index: usize) -> SectorId {
        SectorId::new(index).unwrap()
    }
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
        g.balls[0].pos = V2::new(FIELD.center().x, 600.0);
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
        assert_eq!(g.sector, SectorId::FIRST);
        assert_eq!(g.phase, Phase::Cleared);
        for _ in 0..TICK_HZ {
            g.step(&Input::default());
        }
        assert_eq!(g.phase, Phase::Cleared);
        g.step(&Input {
            launch: true,
            ..Input::default()
        });
        assert_eq!(g.sector, sector(1));
        assert_eq!(g.phase, Phase::Ready);
        g.sector = sector(SECTOR_COUNT - 1);
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
        let mut g = Game::start(sector(3), Mode::Journey);
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
        let mut g = Game::start(sector(0), Mode::Practice);
        g.phase = Phase::Playing;
        g.remaining = 0;
        g.bricks.fill(0);
        g.lost_in_sector = true;
        g.sector_ticks = SectorId::FIRST.sector().par_seconds * TICK_HZ;
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
        assert_eq!(g.sector, SectorId::FIRST);
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
    fn catch_at(g: &mut Game, offset: f32) {
        g.balls[0].pos = V2::new(g.paddle_x + offset, PADDLE_Y - RADIUS - 0.5);
        g.balls[0].velocity = V2::new(0.0, 500.0);
        g.step(&Input::default());
        assert!(g.events.caught);
        assert!(g.balls[0].held);
    }
    #[test]
    fn anchor_holds_without_timeout_and_releases_from_new_position() {
        let mut g = playing();
        g.apply_power(Power::Anchor);
        catch_at(&mut g, 20.0);
        assert_eq!(g.anchor_charges, 2);
        let direction = g.balls[0].velocity.normalized();
        for _ in 0..TICK_HZ * 15 {
            g.step(&Input {
                mouse_x: Some(650.0),
                ..Input::default()
            });
        }
        assert!(g.balls[0].held);
        assert_eq!(g.anchor_charges, 2);
        assert!((g.balls[0].pos.x - 670.0).abs() < 0.01);
        assert_eq!(g.balls[0].previous, g.balls[0].pos);
        assert!(g.sector_ticks >= TICK_HZ * 15); // Thinking time counts for Swift.
        g.step(&Input {
            launch: true,
            ..Input::default()
        });
        assert!(!g.balls[0].held);
        assert!(g.events.launch);
        assert!(g.balls[0].pos.y < PADDLE_Y - RADIUS - 2.0);
        assert!((g.balls[0].velocity.normalized().x - direction.x).abs() < 0.001);
    }
    #[test]
    fn power_combinations_preserve_the_held_ball_and_its_life() {
        let mut g = playing();
        g.apply_power(Power::Anchor);
        catch_at(&mut g, 30.0);
        g.apply_power(Power::Phase);
        g.apply_power(Power::Wide);
        g.apply_power(Power::Slow);
        g.apply_power(Power::Multi);
        g.apply_power(Power::Anchor);
        assert_eq!(g.anchor_charges, 3);
        assert_eq!(g.balls.iter().filter(|b| b.active).count(), 3);
        assert_eq!(g.balls.iter().filter(|b| b.held).count(), 1);
        for b in &g.balls {
            assert_eq!(b.phase_hits, 3);
            assert!(b.velocity.y < 0.0);
        }
        // Two draining balls must not consume a life while one is held.
        for ball in &mut g.balls[1..] {
            ball.pos = V2::new(LEFT + 20.0, BOTTOM + RADIUS - 1.0);
            ball.velocity = V2::new(0.0, 500.0);
        }
        g.wide_time = DT / 2.0;
        g.slow_time = DT / 2.0;
        g.step(&Input {
            mouse_x: Some(RIGHT),
            ..Input::default()
        });
        assert_eq!(g.lives, 3);
        assert!(g.balls[0].held);
        assert!(g.balls[0].pos.x <= RIGHT - RADIUS);
        assert_eq!(g.paddle_width, PADDLE_WIDTH);
        assert!((g.balls[0].velocity.length() - g.speed()).abs() < 0.001);
    }
    #[test]
    fn anchor_catches_only_one_ball_at_a_time() {
        let mut g = playing();
        g.apply_power(Power::Multi);
        g.apply_power(Power::Anchor);
        for ball in &mut g.balls {
            ball.pos = V2::new(g.paddle_x, PADDLE_Y - RADIUS - 0.5);
            ball.velocity = V2::new(0.0, 500.0);
        }
        g.step(&Input::default());
        assert_eq!(g.balls.iter().filter(|b| b.held).count(), 1);
        assert_eq!(g.anchor_charges, 2);
        assert!(
            g.balls
                .iter()
                .filter(|b| !b.held)
                .all(|b| b.velocity.y < 0.0)
        );
    }
    #[test]
    fn phase_hits_three_layers_once_each_then_normal_bounces_resume() {
        let mut g = playing();
        g.bricks.fill(0);
        g.cores.fill(false);
        for row in 0..4 {
            g.bricks[row * COLS + 5] = 3;
        }
        g.remaining = 4;
        g.apply_power(Power::Phase);
        g.balls[0].pos = V2::new(GRID_X + 5.0 * CELL_W + 29.0, GRID_Y + 4.0 * CELL_H + 15.0);
        g.balls[0].velocity = V2::new(0.0, -1000.0);
        for _ in 0..45 {
            g.step(&Input::default());
            if g.balls[0].velocity.y > 0.0 {
                break;
            }
        }
        assert_eq!(g.balls[0].phase_hits, 0);
        for row in 0..4 {
            assert_eq!(g.bricks[row * COLS + 5], 2, "row {row}");
        }
        assert!(g.balls[0].velocity.y > 0.0);
        assert_eq!(g.collision_caps, 0);
    }
    #[test]
    fn phase_can_trigger_a_core_without_reflecting() {
        let mut g = playing();
        g.bricks.fill(0);
        g.cores.fill(false);
        g.bricks[0] = 1;
        g.cores[0] = true;
        g.bricks[1] = 2;
        g.remaining = 2;
        g.apply_power(Power::Phase);
        g.balls[0].pos = V2::new(GRID_X + 29.0, GRID_Y + 24.0 + RADIUS + 1.0);
        g.balls[0].velocity = V2::new(0.0, -500.0);
        g.step(&Input::default());
        assert_eq!(g.bricks[0], 0);
        assert!(g.relay_delay[0] > 0);
        assert_eq!(g.balls[0].phase_hits, 2);
        assert!(g.balls[0].velocity.y < 0.0);
    }
    #[test]
    fn relays_damage_four_neighbors_with_staggered_propagation() {
        let mut g = playing();
        g.bricks.fill(0);
        g.cores.fill(false);
        let c = 3 * COLS + 5;
        for i in [c, c + 1, c - 1, c - COLS, c - COLS + 1] {
            g.bricks[i] = 1;
        }
        g.bricks[c - 1] = 2;
        g.remaining = 5;
        g.cores[c] = true;
        g.cores[c + 1] = true;
        g.hit_brick(c, V2::default());
        for _ in 1..RELAY_TICKS {
            g.advance_relays();
        }
        assert_eq!(g.bricks[c + 1], 1);
        g.advance_relays();
        assert_eq!(g.bricks[c - 1], 1);
        assert_eq!(g.bricks[c - COLS], 0);
        assert_eq!(g.bricks[c - COLS + 1], 1); // Diagonal survives the first core.
        assert_eq!(g.relay_delay[c + 1], RELAY_TICKS);
        for _ in 0..RELAY_TICKS {
            g.advance_relays();
        }
        assert_eq!(g.bricks[c - COLS + 1], 0);
        assert_eq!(g.remaining, 1);
        assert!(!g.drops.iter().any(|d| d.active));
    }
    #[test]
    fn relay_neighbors_do_not_wrap_between_rows() {
        let mut g = playing();
        g.bricks.fill(0);
        g.cores.fill(false);
        g.bricks[COLS - 1] = 1;
        g.cores[COLS - 1] = true;
        g.bricks[COLS] = 1;
        g.remaining = 2;
        g.hit_brick(COLS - 1, V2::default());
        for _ in 0..RELAY_TICKS {
            g.advance_relays();
        }
        assert_eq!(g.bricks[COLS], 1);
    }
    #[test]
    fn full_board_chain_finishes_even_after_last_ball_drains() {
        let mut g = playing();
        g.bricks.fill(1);
        g.cores.fill(true);
        g.remaining = ROWS * COLS;
        g.particles.fill(Particle {
            life: 1.0,
            ..Particle::default()
        });
        g.balls.fill(Ball::default());
        g.hit_brick(0, V2::default());
        for _ in 0..TICK_HZ {
            g.step(&Input::default());
        }
        assert_eq!(g.remaining, 0);
        assert_eq!(g.phase, Phase::Cleared);
        assert_eq!(g.lives, 3);
        assert_eq!(g.score, 26500);
        assert!(g.relay_delay.iter().all(|&d| d == 0));
        assert_eq!(g.collision_caps, 0);
        assert!(!g.drops.iter().any(|d| d.active));
    }
    #[test]
    fn ending_assist_is_a_single_charge_and_requires_a_stalled_rally() {
        let mut g = playing();
        g.remaining = 2;
        g.bricks.fill(0);
        g.bricks[0] = 1;
        g.bricks[COLS - 1] = 1;
        g.sector_ticks = TICK_HZ * 12 - 2;
        g.balls[0].pos = V2::new(480.0, 600.0);
        g.step(&Input::default());
        assert_eq!(g.anchor_charges, 0);
        g.step(&Input::default());
        assert_eq!(g.anchor_charges, 1);
        assert!(g.finish_assist_used);
        g.anchor_charges = 0;
        g.sector_ticks += TICK_HZ * 12;
        g.step(&Input::default());
        assert_eq!(g.anchor_charges, 0);
    }
    #[test]
    fn opening_drops_teach_the_sector_and_random_drops_follow_unlocks() {
        for id in SectorId::all() {
            let (level, definition) = (id.index(), id.sector());
            let mut g = Game::start(id, Mode::Practice);
            g.cores.fill(false);
            g.bricks.fill(1);
            g.remaining = ROWS * COLS;
            g.hit_brick(0, V2::default());
            assert!(!g.drops.iter().any(|d| d.active));
            g.hit_brick(1, V2::default());
            assert_eq!(g.drops[0].power, definition.opening);
            g.apply_power(definition.opening);
            for i in 2..ROWS * COLS {
                g.drops.fill(Drop::default());
                g.hit_brick(i, V2::default());
                for d in g.drops.iter().filter(|d| d.active) {
                    assert!(d.power != Power::Anchor || level >= 1);
                    assert!(d.power != Power::Multi || level >= 4);
                    assert!(d.power != Power::Phase || level >= 8);
                }
            }
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
                .map_or(FIELD.center().x, |b| b.pos.x)
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
