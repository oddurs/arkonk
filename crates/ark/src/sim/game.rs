//! The orchestrator: one tick of play, dispatched by stage.
mod check;
mod sandbox;

pub use check::{Diagnostics, Violation};
pub use sandbox::Sandbox;

use super::{
    ball::{Ball, Contact, Sweep, steepen},
    board::{Board, Damage},
    capsules::{Capsule, DropDirector},
    effects::{Effects, RELAY_FLASH_TICKS},
    events::Events,
    paddle::Paddle,
    power::{Power, PowerState},
    rng::Rng,
    summary::{Medals, SectorSummary},
};
use crate::{
    clock::{DT, TICK_HZ},
    field::{BOTTOM, Cell, CellSet, FIELD, PADDLE_Y, cell_rect},
    geom::V2,
    profile::Checkpoint,
    sectors::SectorId,
    tuning::*,
};

/// Particles from a direct brick hit.
const BURST_DIRECT: usize = 10;
/// Particles from a relay blast.
const BURST_RELAY: usize = 4;
/// Particles from a capsule pickup.
const BURST_PICKUP: usize = 24;

/// How a run is played.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Mode {
    /// The twelve sectors in order, from a saved checkpoint.
    #[default]
    Journey,
    /// One unlocked sector, replayed for medals without touching the journey.
    Practice,
}

/// Where a sector run is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// A ball waits on the paddle to be served.
    Ready,
    /// In play.
    Playing,
    /// The sector is clear and its results are showing.
    Cleared,
    /// The last life is lost.
    GameOver,
    /// The journey's final sector is clear.
    Victory,
}

/// One tick's input. The application turns devices into this.
#[derive(Clone, Copy, Debug, Default)]
pub struct Input {
    /// Keyboard or stick deflection, -1 (left) to 1 (right).
    pub axis: f32,
    /// Where the pointer wants the paddle's centre, overriding `axis`.
    pub target_x: Option<f32>,
    /// Serve, release a held ball, or advance past the results.
    pub launch: bool,
}

/// A game in progress: one sector at a time, in Journey or Practice.
///
/// All state is private and changes only through [`Game::step`], so the
/// rules' invariants hold however the game is driven. Tests, benchmarks and
/// demos that need a particular situation set it up through
/// [`Game::sandbox`].
#[derive(Clone, Debug)]
pub struct Game {
    /// Where the sector run is.
    stage: Stage,
    /// Journey or Practice.
    mode: Mode,
    /// The sector being played.
    sector: SectorId,
    /// The last cleared sector's results.
    summary: SectorSummary,
    /// Points this run.
    score: u32,
    /// Lives left; zero only at game over.
    lives: u8,
    /// Ticks of active play in this sector.
    sector_ticks: u32,
    /// Ticks of active play in this run, from its checkpoint.
    run_ticks: u32,
    /// Ticks since the stage last changed.
    stage_ticks: u32,
    /// Breaks since the last paddle return.
    combo: u32,
    /// The longest chain in this sector.
    best_combo: u32,
    /// The bricks.
    board: Board,
    /// Ball slots.
    balls: [Ball; MAX_BALLS],
    /// Capsule slots.
    capsules: [Capsule; MAX_CAPSULES],
    /// The paddle.
    paddle: Paddle,
    /// Timers and charges.
    powers: PowerState,
    /// Cosmetic pools.
    effects: Effects,
    /// What has happened so far this tick.
    events: Events,
    /// Ticks in which a ball spent its whole collision budget.
    budget_exhausted: u64,
    /// Decides when capsules drop and which.
    director: DropDirector,
    /// Drives capsules and particle bursts.
    rng: Rng,
    /// Whether a life was lost in this sector, which forfeits Clean.
    lost_in_sector: bool,
    /// Paddle returns this serve; each one speeds the ball up a little.
    rally_hits: u32,
    /// Ticks since a brick was last damaged.
    quiet_ticks: u32,
    /// Whether launch was pressed on the results card.
    advance_requested: bool,
    /// `sector_ticks` when a brick was last damaged.
    last_brick_tick: u32,
    /// Whether this sector's finishing assist has been offered.
    finish_assist_used: bool,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    /// A new journey at the first sector.
    pub fn new() -> Self {
        let mut game = Self {
            stage: Stage::Ready,
            mode: Mode::Journey,
            sector: SectorId::FIRST,
            summary: SectorSummary::default(),
            score: 0,
            lives: START_LIVES,
            sector_ticks: 0,
            run_ticks: 0,
            stage_ticks: 0,
            combo: 0,
            best_combo: 0,
            board: Board::new(&SectorId::FIRST.sector().layout),
            balls: [Ball::default(); MAX_BALLS],
            capsules: [Capsule::default(); MAX_CAPSULES],
            paddle: Paddle::new(),
            powers: PowerState::default(),
            effects: Effects::new(),
            events: Events::default(),
            budget_exhausted: 0,
            director: DropDirector::new(),
            rng: Rng::new(),
            lost_in_sector: false,
            rally_hits: 0,
            quiet_ticks: 0,
            advance_requested: false,
            last_brick_tick: 0,
            finish_assist_used: false,
        };
        game.load_sector();
        game
    }

    /// A game starting at `sector`.
    pub fn start(sector: SectorId, mode: Mode) -> Self {
        let mut game = Self::new();
        game.sector = sector;
        game.mode = mode;
        game.load_sector();
        game
    }

    /// A journey resumed from a saved checkpoint: its sector, score, lives
    /// and run time.
    pub fn resume(checkpoint: Checkpoint) -> Self {
        let mut game = Self::start(checkpoint.sector, Mode::Journey);
        game.score = checkpoint.score;
        game.lives = checkpoint.lives;
        game.run_ticks = checkpoint.ticks;
        game
    }

    /// Where the sector run is.
    pub fn stage(&self) -> Stage {
        self.stage
    }
    /// Ticks since the stage last changed.
    pub fn stage_ticks(&self) -> u32 {
        self.stage_ticks
    }
    /// Journey or Practice.
    pub fn mode(&self) -> Mode {
        self.mode
    }
    /// The sector being played.
    pub fn sector(&self) -> SectorId {
        self.sector
    }
    /// Points this run.
    pub fn score(&self) -> u32 {
        self.score
    }
    /// Lives left; zero only at game over.
    pub fn lives(&self) -> u8 {
        self.lives
    }
    /// Ticks of active play in this sector; serving and results do not count.
    pub fn sector_ticks(&self) -> u32 {
        self.sector_ticks
    }
    /// Ticks of active play in this run, counted from its checkpoint.
    pub fn run_ticks(&self) -> u32 {
        self.run_ticks
    }
    /// Breaks since the last paddle return.
    pub fn combo(&self) -> u32 {
        self.combo
    }
    /// The longest chain in this sector.
    pub fn best_combo(&self) -> u32 {
        self.best_combo
    }
    /// The results of the most recent clear in this game; all zero before
    /// the first. The results card shows them while the stage is
    /// [`Stage::Cleared`] or [`Stage::Victory`].
    pub fn summary(&self) -> SectorSummary {
        self.summary
    }
    /// The bricks.
    pub fn board(&self) -> &Board {
        &self.board
    }
    /// Every ball slot; inactive slots are empty.
    pub fn balls(&self) -> &[Ball; MAX_BALLS] {
        &self.balls
    }
    /// Every capsule slot; inactive slots are empty.
    pub fn capsules(&self) -> &[Capsule; MAX_CAPSULES] {
        &self.capsules
    }
    /// The paddle.
    pub fn paddle(&self) -> Paddle {
        self.paddle
    }
    /// Timers and charges in effect.
    pub fn powers(&self) -> PowerState {
        self.powers
    }
    /// Cosmetic state for the renderer.
    pub fn effects(&self) -> &Effects {
        &self.effects
    }
    /// The run as a checkpoint: where a retry or a resumed journey starts.
    pub fn checkpoint(&self) -> Checkpoint {
        Checkpoint {
            sector: self.sector,
            score: self.score,
            lives: self.lives,
            ticks: self.run_ticks,
        }
    }
    /// Counters for performance overlays and tests.
    pub fn diagnostics(&self) -> Diagnostics {
        Diagnostics {
            budget_exhausted: self.budget_exhausted,
        }
    }
    /// Privileged access for setting up situations; see [`Sandbox`].
    pub fn sandbox(&mut self) -> Sandbox<'_> {
        Sandbox::new(self)
    }

    /// Current ball speed: the sector's serve speed, raised by the rally and
    /// lowered by Slow.
    pub fn speed(&self) -> f32 {
        (self.sector.sector().speed + self.rally_hits.min(RALLY_CAP) as f32 * RALLY_SPEEDUP)
            * if self.powers.slow() { SLOW_FACTOR } else { 1.0 }
    }

    /// The velocity a serve leaves with: up and toward the field's centre.
    pub fn launch_velocity(&self) -> V2 {
        let toward_center = if self.paddle.x > FIELD.center().x + LAUNCH_CENTER_BIAS {
            -1.0
        } else {
            1.0
        };
        V2::new(LAUNCH_DIR.x * toward_center, LAUNCH_DIR.y).normalized() * self.speed()
    }

    /// Advances the game by one tick and reports what happened.
    pub fn step(&mut self, input: Input) -> Events {
        self.events = Events::default();
        self.effects.tick();
        self.stage_ticks = self.stage_ticks.saturating_add(1);
        match self.stage {
            Stage::GameOver | Stage::Victory => {}
            Stage::Cleared => self.step_cleared(input.launch),
            Stage::Ready => {
                self.paddle.steer(input.target_x, input.axis);
                self.step_ready(input.launch);
            }
            Stage::Playing => {
                self.paddle.steer(input.target_x, input.axis);
                self.step_playing(input.launch);
            }
        }
        self.events
    }

    /// Grants `power` as if its capsule had been caught.
    fn apply_power(&mut self, power: Power) {
        self.events.pickup = true;
        self.effects.notice(power);
        self.director.collected(power, self.sector);
        self.effects.burst(
            &mut self.rng,
            V2::new(self.paddle.x, PADDLE_Y),
            2,
            BURST_PICKUP,
        );
        match power {
            Power::Anchor => self.powers.anchor_charges = ANCHOR_CHARGES,
            Power::Phase => {
                for ball in &mut self.balls {
                    if ball.active {
                        ball.phase_charges = PHASE_CONTACTS;
                    }
                }
            }
            Power::Wide => {
                self.powers.wide_seconds = WIDE_SECONDS;
                self.paddle.width = WIDE_PADDLE_WIDTH;
                self.paddle.keep_inside();
            }
            Power::Slow => {
                self.powers.slow_seconds = SLOW_SECONDS;
                self.set_ball_speeds();
            }
            Power::Multi => self.split_ball(),
        }
    }

    /// Results are up. A launch press in a journey moves on to the next
    /// sector once they have been shown long enough.
    fn step_cleared(&mut self, launch: bool) {
        self.advance_requested |= launch;
        if self.mode == Mode::Journey
            && self.advance_requested
            && self.stage_ticks >= ADVANCE_DELAY_TICKS
            && let Some(next) = self.sector.next()
        {
            self.sector = next;
            self.load_sector();
        }
    }

    /// The ball rides the paddle until it is served.
    fn step_ready(&mut self, launch: bool) {
        let pos = self.paddle.serve_point();
        self.balls[0].pos = pos;
        self.balls[0].previous = pos;
        if launch {
            self.stage = Stage::Playing;
            self.balls[0].velocity = self.launch_velocity();
            self.events.launch = true;
        }
    }

    fn step_playing(&mut self, launch: bool) {
        self.sector_ticks += 1;
        self.run_ticks += 1;
        self.quiet_ticks += 1;
        self.offer_finishing_assist();
        self.nudge_stalled_balls();
        if self.powers.tick() {
            self.set_ball_speeds();
        }
        self.paddle.width = if self.powers.wide() {
            WIDE_PADDLE_WIDTH
        } else {
            PADDLE_WIDTH
        };
        self.sync_held_balls();
        for index in 0..MAX_BALLS {
            if !self.balls[index].active {
                continue;
            }
            if self.balls[index].held {
                if !launch {
                    continue;
                }
                self.balls[index].held = false;
                self.events.launch = true;
            }
            self.move_ball(index);
        }
        self.ignite_relays();
        // A committed chain finishes even after the last ball drains, so a
        // final blast can still clear the sector.
        let pending = self.board.relays_pending();
        if self.board.remaining() == 0 && !pending {
            self.finish_sector();
        } else if !self.balls.iter().any(|b| b.active) && !pending {
            self.lose_life();
        } else {
            self.update_capsules();
            self.sync_held_balls();
        }
    }

    fn load_sector(&mut self) {
        self.board = Board::new(&self.sector.sector().layout);
        self.director = DropDirector::new();
        self.effects.relay_flash = [0; crate::field::CELLS];
        self.effects.particles.fill(Default::default());
        self.last_brick_tick = 0;
        self.finish_assist_used = false;
        self.sector_ticks = 0;
        self.lost_in_sector = false;
        self.best_combo = 0;
        self.reset_serve();
    }

    fn reset_serve(&mut self) {
        self.stage = Stage::Ready;
        self.stage_ticks = 0;
        self.rally_hits = 0;
        self.quiet_ticks = 0;
        self.balls = [Ball::default(); MAX_BALLS];
        self.capsules = [Capsule::default(); MAX_CAPSULES];
        self.powers = PowerState::default();
        self.effects.notice = None;
        self.effects.notice_ticks = 0;
        self.paddle.width = PADDLE_WIDTH;
        self.paddle.keep_inside();
        self.paddle.previous = self.paddle.x;
        self.combo = 0;
        let pos = self.paddle.serve_point();
        self.balls[0] = Ball {
            pos,
            previous: pos,
            active: true,
            ..Ball::default()
        };
    }

    fn finish_sector(&mut self) {
        let clean = !self.lost_in_sector;
        let swift = self.sector_ticks <= self.sector.sector().par_seconds * TICK_HZ;
        let mut medals = Medals::CLEAR;
        if clean {
            medals |= Medals::CLEAN;
        }
        if swift {
            medals |= Medals::SWIFT;
        }
        let bonus = CLEAR_BONUS + u32::from(clean) * MEDAL_BONUS + u32::from(swift) * MEDAL_BONUS;
        let life_earned = self.mode == Mode::Journey
            && (self.sector.index() + 1).is_multiple_of(4)
            && self.lives < MAX_LIVES;
        if life_earned {
            self.lives += 1;
        }
        self.score += bonus;
        self.summary = SectorSummary {
            ticks: self.sector_ticks,
            medals,
            bonus,
            best_combo: self.best_combo,
            life_earned,
        };
        self.stage = if self.sector.next().is_none() && self.mode == Mode::Journey {
            Stage::Victory
        } else {
            Stage::Cleared
        };
        self.stage_ticks = 0;
        self.advance_requested = false;
        self.events.clear = true;
        self.capsules.fill(Capsule::default());
    }

    fn lose_life(&mut self) {
        self.lives -= 1;
        self.lost_in_sector = true;
        self.events.lost = true;
        if self.lives == 0 {
            self.stage = Stage::GameOver;
        } else {
            self.reset_serve();
        }
    }

    /// When the last bricks have stalled a single-ball rally, one Anchor
    /// catch lets the player aim the finish. Offered once per sector.
    fn offer_finishing_assist(&mut self) {
        let remaining = self.board.remaining();
        if remaining <= ASSIST_BRICKS
            && remaining > 0
            && !self.finish_assist_used
            && self.sector_ticks.saturating_sub(self.last_brick_tick) >= ASSIST_STALL_TICKS
            && self.powers.anchor_charges == 0
            && self.balls.iter().filter(|b| b.active).count() == 1
            && !self.balls.iter().any(|b| b.held)
        {
            self.powers.anchor_charges = 1;
            self.finish_assist_used = true;
            self.effects.notice(Power::Anchor);
            self.events.pickup = true;
        }
    }

    /// A gentle, infrequent correction breaks exact vertical repeats after
    /// several empty rallies, without changing normal player-directed shots.
    fn nudge_stalled_balls(&mut self) {
        if self.quiet_ticks <= ANTI_STALL_TICKS {
            return;
        }
        for ball in &mut self.balls {
            if ball.active
                && !ball.held
                && ball.velocity.x.abs() < ball.velocity.length() * ANTI_STALL_NEAR_VERTICAL
            {
                let speed = ball.velocity.length();
                let sign = if ball.pos.x > FIELD.center().x {
                    -1.0
                } else {
                    1.0
                };
                ball.velocity =
                    V2::new(speed * ANTI_STALL_STEER * sign, ball.velocity.y).normalized() * speed;
            }
        }
        self.quiet_ticks = 0;
    }

    /// Sets every ball to the current speed, keeping its direction.
    fn set_ball_speeds(&mut self) {
        let speed = self.speed();
        for ball in &mut self.balls {
            ball.velocity = ball.velocity.normalized() * speed;
        }
    }

    /// Keeps held balls on the paddle as it moves.
    fn sync_held_balls(&mut self) {
        for ball in &mut self.balls {
            if ball.active && ball.held {
                ball.pos = self.paddle.hold_point(ball.held_offset);
                ball.previous = ball.pos;
            }
        }
    }

    /// Moves one ball through the tick, contact by contact.
    fn move_ball(&mut self, index: usize) {
        let mut ball = self.balls[index];
        ball.leave_phased();
        ball.previous = ball.pos;
        let mut sweep = Sweep::new();
        // A pathological tick that spends the whole budget leaves the ball at
        // its last safe position: it never advances unchecked through bricks.
        for _ in 0..COLLISION_BUDGET {
            if sweep.is_done() {
                break;
            }
            let normal = match sweep.advance(&mut ball, &self.paddle, &self.board) {
                Contact::None => {
                    sweep.stop();
                    break;
                }
                Contact::Drain => {
                    ball.active = false;
                    sweep.stop();
                    break;
                }
                Contact::Paddle(normal) => {
                    if self.return_ball(&mut ball, sweep.elapsed()) {
                        sweep.stop();
                        break;
                    }
                    normal
                }
                // Phase passes through and keeps ignoring the brick until the
                // ball is fully outside it, even when the last charge is
                // spent inside armour.
                Contact::Brick(cell, _) if ball.phase_charges > 0 => {
                    ball.phase_charges -= 1;
                    ball.phased.insert(cell);
                    self.damage(cell, ball.pos, true);
                    self.events.phase_hit = true;
                    continue;
                }
                Contact::Brick(cell, normal) => {
                    ball.bounce(normal);
                    self.damage(cell, ball.pos, true);
                    normal
                }
                Contact::Wall(normal) => {
                    ball.bounce(normal);
                    self.events.wall = true;
                    normal
                }
            };
            ball.pos += normal * CONTACT_SKIN;
        }
        if !sweep.is_done() {
            self.budget_exhausted += 1;
        }
        self.balls[index] = ball;
    }

    /// Sends a ball back up off the paddle, steered by where it hit, or
    /// catches it if Anchor has a charge and holds no other ball. True when
    /// caught.
    fn return_ball(&mut self, ball: &mut Ball, elapsed: f32) -> bool {
        let offset = self.paddle.hit_offset(ball.pos.x, elapsed);
        self.rally_hits += 1;
        ball.velocity = Paddle::bounce(offset) * self.speed();
        self.combo = 0;
        self.events.paddle = true;
        if self.powers.anchor_charges == 0 || self.balls.iter().any(|b| b.active && b.held) {
            return false;
        }
        self.powers.anchor_charges -= 1;
        ball.held = true;
        ball.held_offset = offset.clamp(-HOLD_OFFSET_LIMIT, HOLD_OFFSET_LIMIT);
        ball.velocity = Paddle::bounce(ball.held_offset) * self.speed();
        ball.pos = self.paddle.hold_point(ball.held_offset);
        ball.previous = ball.pos;
        ball.phased = CellSet::EMPTY;
        self.events.caught = true;
        true
    }

    /// One point of damage to the brick in `cell`, from a ball (`direct`)
    /// or a relay blast, with feedback at `pos`.
    fn damage(&mut self, cell: Cell, pos: V2, direct: bool) {
        let damage = self.board.damage(cell);
        if damage == Damage::Missed {
            return;
        }
        self.events.brick = true;
        self.quiet_ticks = 0;
        self.last_brick_tick = self.sector_ticks;
        let particles = if direct { BURST_DIRECT } else { BURST_RELAY };
        self.effects
            .burst(&mut self.rng, pos, cell.row(), particles);
        if damage == Damage::Chipped {
            self.score += SCORE_CHIP;
            return;
        }
        self.combo += 1;
        self.best_combo = self.best_combo.max(self.combo);
        self.events.combo = self.combo;
        self.score += SCORE_BREAK + SCORE_COMBO_STEP * self.combo.min(COMBO_CAP);
        // Relay damage rewards the shot without generating capsule storms.
        if direct
            && let Some(power) = self.director.on_break(&mut self.rng, self.sector)
            && let Some(slot) = self.capsules.iter_mut().find(|d| !d.active)
        {
            *slot = Capsule {
                pos,
                power,
                active: true,
            };
            self.director.dropped();
        }
    }

    /// Sets off the blasts due this tick; each damages its four neighbours.
    fn ignite_relays(&mut self) {
        for cell in self.board.tick_relays().iter() {
            self.events.relay = true;
            self.effects.relay_flash[cell.index()] = RELAY_FLASH_TICKS;
            for neighbor in cell.neighbors().into_iter().flatten() {
                self.damage(neighbor, cell_rect(neighbor).center(), false);
            }
        }
    }

    /// Capsules fall; the paddle catches them or they fall out.
    fn update_capsules(&mut self) {
        for i in 0..MAX_CAPSULES {
            if !self.capsules[i].active {
                continue;
            }
            self.capsules[i].pos.y += CAPSULE_SPEED * DT;
            let pos = self.capsules[i].pos;
            if self.paddle.catches(pos) {
                let power = self.capsules[i].power;
                self.capsules[i].active = false;
                self.apply_power(power);
            } else if pos.y > BOTTOM {
                self.capsules[i].active = false;
            }
        }
    }

    /// Multi: up to two new balls, split either side of a moving ball (or a
    /// held one, if none is moving). A held original stays held.
    fn split_ball(&mut self) {
        let Some(source) = self
            .balls
            .iter()
            .copied()
            .find(|b| b.active && !b.held)
            .or_else(|| self.balls.iter().copied().find(|b| b.active))
        else {
            return;
        };
        let mut n = 0;
        for b in &mut self.balls {
            if !b.active {
                let angle = if n == 0 {
                    -MULTI_SPLIT_ANGLE
                } else {
                    MULTI_SPLIT_ANGLE
                };
                let (s, c) = angle.sin_cos();
                let v = source.velocity;
                *b = Ball {
                    held: false,
                    velocity: steepen(V2::new(v.x * c - v.y * s, v.x * s + v.y * c)),
                    ..source
                };
                n += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        field::{
            BALL_RADIUS as RADIUS, CELL_H, CELL_W, CELLS, COLS, GRID_X, GRID_Y, LEFT, RIGHT, TOP,
        },
        sectors::SECTOR_COUNT,
        sim::effects::Particle,
    };
    fn sector(index: usize) -> SectorId {
        SectorId::new(index).unwrap()
    }
    fn cell(index: usize) -> Cell {
        Cell::new(index).unwrap()
    }
    /// Empties the board, then places `(cell, hp, core)` bricks.
    fn board(g: &mut Game, bricks: &[(usize, u8, bool)]) {
        g.board.reset([0; CELLS], CellSet::EMPTY);
        for &(i, hp, core) in bricks {
            g.board.set(cell(i), hp, core);
        }
    }
    fn hp(g: &Game, index: usize) -> u8 {
        g.board.hp(cell(index))
    }
    fn playing() -> Game {
        let mut g = Game::new();
        g.step(Input {
            launch: true,
            ..Input::default()
        });
        g
    }
    #[test]
    fn high_speed_brick_collision_and_damage() {
        let mut g = playing();
        board(&mut g, &[(0, 1, false), (COLS - 1, 1, false)]);
        g.balls[0].pos = V2::new(GRID_X + 29.0, GRID_Y + 80.0);
        g.balls[0].velocity = V2::new(0.0, -16000.0);
        g.step(Input::default());
        assert_eq!(hp(&g, 0), 0);
        assert!(g.score > 0);
        assert!(g.balls[0].velocity.y > 0.0);
        assert_eq!(g.budget_exhausted, 0);
    }
    #[test]
    fn paddle_edges_steer_and_center_is_vertical() {
        for offset in [-45.0, 0.0, 45.0] {
            let mut g = playing();
            g.balls[0].pos = V2::new(g.paddle.x + offset, PADDLE_Y - 8.0);
            g.balls[0].velocity = V2::new(0.0, 500.0);
            g.step(Input::default());
            assert!(g.balls[0].velocity.y < 0.0);
            assert!((g.balls[0].velocity.length() - g.speed()).abs() < 0.001);
            assert_eq!(g.balls[0].velocity.x.signum(), offset.signum());
        }
    }
    #[test]
    fn moving_paddle_cannot_teleport_to_intercept() {
        let mut g = playing();
        g.balls[0].pos = V2::new(g.paddle.x + 70.0, PADDLE_Y + 20.0);
        g.balls[0].velocity = V2::new(0.0, 600.0);
        let events = g.step(Input {
            target_x: Some(RIGHT),
            ..Input::default()
        });
        assert!(!events.paddle);
    }
    #[test]
    fn moving_paddle_catches_a_ball_in_its_actual_path() {
        let mut g = playing();
        g.balls[0].pos = V2::new(g.paddle.x + 70.0, PADDLE_Y - 7.5);
        g.balls[0].velocity = V2::new(0.0, 600.0);
        let events = g.step(Input {
            target_x: Some(RIGHT),
            ..Input::default()
        });
        assert!(events.paddle);
        assert!(g.balls[0].velocity.y < 0.0);
    }
    #[test]
    fn simultaneous_wall_corner_reflects_both_axes() {
        let mut g = playing();
        g.balls[0].pos = V2::new(LEFT + RADIUS + 1.0, TOP + RADIUS + 1.0);
        g.balls[0].velocity = V2::new(-500.0, -500.0);
        g.step(Input::default());
        assert!(g.balls[0].velocity.x > 0.0 && g.balls[0].velocity.y > 0.0);
        assert!(g.balls[0].pos.x >= LEFT + RADIUS && g.balls[0].pos.y >= TOP + RADIUS);
        assert_eq!(g.budget_exhausted, 0);
    }
    #[test]
    fn exhausted_budget_keeps_last_safe_position() {
        let mut g = playing();
        g.balls[0].pos = V2::new(FIELD.center().x, 600.0);
        g.balls[0].velocity = V2::new(1_000_000_000.0, 0.0);
        g.step(Input::default());
        assert_eq!(g.budget_exhausted, 1);
        assert!(g.balls[0].pos.x >= LEFT + RADIUS && g.balls[0].pos.x <= RIGHT - RADIUS);
        assert_eq!(g.balls[0].pos.y, 600.0);
    }
    #[test]
    fn life_lost_only_after_last_ball() {
        let mut g = playing();
        g.balls[1] = g.balls[0];
        g.balls[0].pos = V2::new(LEFT + 30.0, BOTTOM + RADIUS - 1.0);
        g.balls[0].velocity = V2::new(0.0, 500.0);
        g.step(Input::default());
        assert_eq!(g.lives, 3);
        g.balls[1].pos = g.balls[0].pos;
        g.balls[1].velocity = V2::new(0.0, 500.0);
        g.step(Input::default());
        assert_eq!(g.lives, 2);
        assert_eq!(g.stage, Stage::Ready);
    }
    #[test]
    fn next_level_and_victory() {
        let mut g = playing();
        board(&mut g, &[]);
        g.step(Input::default());
        assert_eq!(g.sector, SectorId::FIRST);
        assert_eq!(g.stage, Stage::Cleared);
        for _ in 0..TICK_HZ {
            g.step(Input::default());
        }
        assert_eq!(g.stage, Stage::Cleared);
        g.step(Input {
            launch: true,
            ..Input::default()
        });
        assert_eq!(g.sector, sector(1));
        assert_eq!(g.stage, Stage::Ready);
        g.sector = sector(SECTOR_COUNT - 1);
        g.stage = Stage::Playing;
        board(&mut g, &[]);
        g.step(Input::default());
        assert_eq!(g.stage, Stage::Victory);
    }
    #[test]
    fn powers_extend_expire_and_spawn() {
        let mut g = playing();
        g.apply_power(Power::Multi);
        assert_eq!(g.balls.iter().filter(|b| b.active).count(), 3);
        g.apply_power(Power::Wide);
        assert_eq!(g.paddle.width, WIDE_PADDLE_WIDTH);
        g.apply_power(Power::Slow);
        assert!((g.balls[0].velocity.length() - g.speed()).abs() < 0.001);
        g.powers.slow_seconds = DT / 2.0;
        g.powers.wide_seconds = DT / 2.0;
        g.step(Input::default());
        assert_eq!(g.paddle.width, PADDLE_WIDTH);
        assert!((g.balls[0].velocity.length() - g.speed()).abs() < 0.001);
    }
    #[test]
    fn mouse_tracks_in_one_tick_and_keyboard_has_consistent_speed() {
        let mut g = Game::new();
        g.step(Input {
            target_x: Some(700.0),
            ..Input::default()
        });
        assert_eq!(g.paddle.x, 700.0);
        assert_eq!(g.balls[0].pos.x, 700.0);
        g.step(Input {
            axis: -1.0,
            ..Input::default()
        });
        assert!((g.paddle.x - (700.0 - KEYBOARD_SPEED * DT)).abs() < 0.001);
    }
    #[test]
    fn chapter_rewards_and_medals_follow_actual_play() {
        let mut g = Game::start(sector(3), Mode::Journey);
        g.stage = Stage::Playing;
        board(&mut g, &[]);
        g.step(Input::default());
        assert_eq!(g.summary.medals, Medals::ALL);
        assert_eq!(g.summary.bonus, 2000);
        assert_eq!(g.lives, 4);
        assert!(g.summary.life_earned);
        let ticks = g.sector_ticks;
        for _ in 0..100 {
            g.step(Input::default());
        }
        assert_eq!(g.sector_ticks, ticks);
        assert_eq!(g.lives, 4);
        let mut g = Game::start(sector(0), Mode::Practice);
        g.stage = Stage::Playing;
        board(&mut g, &[]);
        g.lost_in_sector = true;
        g.sector_ticks = SectorId::FIRST.sector().par_seconds * TICK_HZ;
        g.step(Input::default());
        assert_eq!(g.summary.medals, Medals::CLEAR);
        assert_eq!(g.summary.bonus, 1000);
        for _ in 0..TICK_HZ {
            g.step(Input {
                launch: true,
                ..Input::default()
            });
        }
        assert_eq!(g.stage, Stage::Cleared);
        assert_eq!(g.sector, SectorId::FIRST);
    }
    #[test]
    fn vertical_rallies_are_nudged_after_a_quiet_spell() {
        let mut g = playing();
        g.balls[0].pos = V2::new(500.0, 600.0);
        g.balls[0].velocity = V2::new(0.0, -500.0);
        g.quiet_ticks = ANTI_STALL_TICKS;
        g.step(Input::default());
        assert!(g.balls[0].velocity.x < -50.0);
    }
    #[test]
    fn drop_cadence_has_a_bounded_dry_spell() {
        let mut g = playing();
        g.board.reset([1; CELLS], CellSet::EMPTY);
        let mut dry = 0;
        for c in Cell::all() {
            g.capsules.fill(Capsule::default());
            g.damage(c, V2::new(400.0, 300.0), true);
            dry += 1;
            if g.capsules.iter().any(|d| d.active) {
                assert!(dry <= 7);
                dry = 0;
            }
            assert!(dry < 7);
        }
    }
    fn catch_at(g: &mut Game, offset: f32) {
        g.balls[0].pos = V2::new(g.paddle.x + offset, PADDLE_Y - RADIUS - 0.5);
        g.balls[0].velocity = V2::new(0.0, 500.0);
        assert!(g.step(Input::default()).caught);
        assert!(g.balls[0].held);
    }
    #[test]
    fn anchor_holds_without_timeout_and_releases_from_new_position() {
        let mut g = playing();
        g.apply_power(Power::Anchor);
        catch_at(&mut g, 20.0);
        assert_eq!(g.powers.anchor_charges, 2);
        let direction = g.balls[0].velocity.normalized();
        for _ in 0..TICK_HZ * 15 {
            g.step(Input {
                target_x: Some(650.0),
                ..Input::default()
            });
        }
        assert!(g.balls[0].held);
        assert_eq!(g.powers.anchor_charges, 2);
        assert!((g.balls[0].pos.x - 670.0).abs() < 0.01);
        assert_eq!(g.balls[0].previous, g.balls[0].pos);
        assert!(g.sector_ticks >= TICK_HZ * 15); // Thinking time counts for Swift.
        let events = g.step(Input {
            launch: true,
            ..Input::default()
        });
        assert!(!g.balls[0].held);
        assert!(events.launch);
        assert!(g.balls[0].pos.y < PADDLE_Y - RADIUS - SERVE_GAP);
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
        assert_eq!(g.powers.anchor_charges, 3);
        assert_eq!(g.balls.iter().filter(|b| b.active).count(), 3);
        assert_eq!(g.balls.iter().filter(|b| b.held).count(), 1);
        for b in &g.balls {
            assert_eq!(b.phase_charges, 3);
            assert!(b.velocity.y < 0.0);
        }
        // Two draining balls must not consume a life while one is held.
        for ball in &mut g.balls[1..] {
            ball.pos = V2::new(LEFT + 20.0, BOTTOM + RADIUS - 1.0);
            ball.velocity = V2::new(0.0, 500.0);
        }
        g.powers.wide_seconds = DT / 2.0;
        g.powers.slow_seconds = DT / 2.0;
        g.step(Input {
            target_x: Some(RIGHT),
            ..Input::default()
        });
        assert_eq!(g.lives, 3);
        assert!(g.balls[0].held);
        assert!(g.balls[0].pos.x <= RIGHT - RADIUS);
        assert_eq!(g.paddle.width, PADDLE_WIDTH);
        assert!((g.balls[0].velocity.length() - g.speed()).abs() < 0.001);
    }
    #[test]
    fn anchor_catches_only_one_ball_at_a_time() {
        let mut g = playing();
        g.apply_power(Power::Multi);
        g.apply_power(Power::Anchor);
        for ball in &mut g.balls {
            ball.pos = V2::new(g.paddle.x, PADDLE_Y - RADIUS - 0.5);
            ball.velocity = V2::new(0.0, 500.0);
        }
        g.step(Input::default());
        assert_eq!(g.balls.iter().filter(|b| b.held).count(), 1);
        assert_eq!(g.powers.anchor_charges, 2);
        assert!(
            g.balls
                .iter()
                .filter(|b| !b.held)
                .all(|b| b.velocity.y < 0.0)
        );
    }
    #[test]
    fn phase_charges_three_layers_once_each_then_normal_bounces_resume() {
        let mut g = playing();
        board(
            &mut g,
            &[
                (5, 3, false),
                (COLS + 5, 3, false),
                (2 * COLS + 5, 3, false),
                (3 * COLS + 5, 3, false),
            ],
        );
        g.apply_power(Power::Phase);
        g.balls[0].pos = V2::new(GRID_X + 5.0 * CELL_W + 29.0, GRID_Y + 4.0 * CELL_H + 15.0);
        g.balls[0].velocity = V2::new(0.0, -1000.0);
        for _ in 0..45 {
            g.step(Input::default());
            if g.balls[0].velocity.y > 0.0 {
                break;
            }
        }
        assert_eq!(g.balls[0].phase_charges, 0);
        for row in 0..4 {
            assert_eq!(hp(&g, row * COLS + 5), 2, "row {row}");
        }
        assert!(g.balls[0].velocity.y > 0.0);
        assert_eq!(g.budget_exhausted, 0);
    }
    #[test]
    fn phase_can_trigger_a_core_without_reflecting() {
        let mut g = playing();
        board(&mut g, &[(0, 1, true), (1, 2, false)]);
        g.apply_power(Power::Phase);
        g.balls[0].pos = V2::new(GRID_X + 29.0, GRID_Y + 24.0 + RADIUS + 1.0);
        g.balls[0].velocity = V2::new(0.0, -500.0);
        g.step(Input::default());
        assert_eq!(hp(&g, 0), 0);
        assert!(g.board.relay_countdown(cell(0)) > 0);
        assert_eq!(g.balls[0].phase_charges, 2);
        assert!(g.balls[0].velocity.y < 0.0);
    }
    #[test]
    fn relays_damage_four_neighbors_with_staggered_propagation() {
        let mut g = playing();
        let c = 3 * COLS + 5;
        board(
            &mut g,
            &[
                (c, 1, true),
                (c + 1, 1, true),
                (c - 1, 2, false),
                (c - COLS, 1, false),
                (c - COLS + 1, 1, false),
            ],
        );
        g.damage(cell(c), V2::default(), true);
        for _ in 1..RELAY_TICKS {
            g.ignite_relays();
        }
        assert_eq!(hp(&g, c + 1), 1);
        g.ignite_relays();
        assert_eq!(hp(&g, c - 1), 1);
        assert_eq!(hp(&g, c - COLS), 0);
        assert_eq!(hp(&g, c - COLS + 1), 1); // Diagonal survives the first core.
        assert_eq!(g.board.relay_countdown(cell(c + 1)), RELAY_TICKS);
        for _ in 0..RELAY_TICKS {
            g.ignite_relays();
        }
        assert_eq!(hp(&g, c - COLS + 1), 0);
        assert_eq!(g.board.remaining(), 1);
        assert!(!g.capsules.iter().any(|d| d.active));
    }
    #[test]
    fn relay_neighbors_do_not_wrap_between_rows() {
        let mut g = playing();
        board(&mut g, &[(COLS - 1, 1, true), (COLS, 1, false)]);
        g.damage(cell(COLS - 1), V2::default(), true);
        for _ in 0..RELAY_TICKS {
            g.ignite_relays();
        }
        assert_eq!(hp(&g, COLS), 1);
    }
    #[test]
    fn full_board_chain_finishes_even_after_last_ball_drains() {
        let mut g = playing();
        g.board.reset([1; CELLS], CellSet::ALL);
        g.effects.particles.fill(Particle {
            life: 1.0,
            ..Particle::default()
        });
        g.balls.fill(Ball::default());
        g.damage(cell(0), V2::default(), true);
        for _ in 0..TICK_HZ {
            g.step(Input::default());
        }
        assert_eq!(g.board.remaining(), 0);
        assert_eq!(g.stage, Stage::Cleared);
        assert_eq!(g.lives, 3);
        assert_eq!(g.score, 26500);
        assert!(!g.board.relays_pending());
        assert_eq!(g.budget_exhausted, 0);
        assert!(!g.capsules.iter().any(|d| d.active));
    }
    #[test]
    fn ending_assist_is_a_single_charge_and_requires_a_stalled_rally() {
        let mut g = playing();
        board(&mut g, &[(0, 1, false), (COLS - 1, 1, false)]);
        g.sector_ticks = ASSIST_STALL_TICKS - 2;
        g.balls[0].pos = V2::new(480.0, 600.0);
        g.step(Input::default());
        assert_eq!(g.powers.anchor_charges, 0);
        g.step(Input::default());
        assert_eq!(g.powers.anchor_charges, 1);
        assert!(g.finish_assist_used);
        g.powers.anchor_charges = 0;
        g.sector_ticks += ASSIST_STALL_TICKS;
        g.step(Input::default());
        assert_eq!(g.powers.anchor_charges, 0);
    }
    #[test]
    fn opening_drops_teach_the_sector_and_random_drops_follow_unlocks() {
        for id in SectorId::all() {
            let (level, definition) = (id.index(), id.sector());
            let mut g = Game::start(id, Mode::Practice);
            g.board.reset([1; CELLS], CellSet::EMPTY);
            g.damage(cell(0), V2::default(), true);
            assert!(!g.capsules.iter().any(|d| d.active));
            g.damage(cell(1), V2::default(), true);
            assert_eq!(g.capsules[0].power, definition.opening);
            g.apply_power(definition.opening);
            for c in Cell::all().skip(2) {
                g.capsules.fill(Capsule::default());
                g.damage(c, V2::default(), true);
                for d in g.capsules.iter().filter(|d| d.active) {
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
            if matches!(a.stage, Stage::GameOver | Stage::Victory) {
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
                target_x: Some(x),
                launch: true,
                ..Input::default()
            };
            impacts += u32::from(a.step(input).brick);
            b.step(input);
            assert_eq!(a.validate(), Ok(()));
            assert_eq!(a.stage, b.stage);
            assert_eq!(a.score, b.score);
            assert_eq!(a.budget_exhausted, 0);
            for (ball, other) in a.balls.iter().zip(&b.balls) {
                assert!(ball.pos.x.is_finite() && ball.pos.y.is_finite());
                assert_eq!(ball.pos, other.pos);
                assert_eq!(ball.velocity, other.velocity);
            }
        }
        assert_eq!(a.score, b.score);
        assert_eq!(a.board, b.board);
        assert_eq!(a.budget_exhausted, 0);
        assert!(impacts > 100);
    }
}
