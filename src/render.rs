use crate::{
    crt::{CURVATURE, Crt},
    perf::Perf,
    pixel_font::PixelFont,
    ui::{self, Screen, Ui},
};
use arkonk::{game::*, levels::CHAPTERS, physics::V2, profile::Profile};
use macroquad::prelude::*;
use std::fmt::Write;

const BG: Color = Color::new(0.012, 0.016, 0.035, 1.0);
const GLASS: Color = Color::new(0.017, 0.027, 0.052, 1.0);
const METAL: Color = Color::new(0.075, 0.094, 0.135, 1.0);
const INK: Color = Color::new(0.96, 0.96, 0.85, 1.0);
const DIM: Color = Color::new(0.45, 0.54, 0.64, 1.0);
const CYAN: Color = Color::new(0.27, 0.92, 0.95, 1.0);
const AMBER: Color = Color::new(1.0, 0.76, 0.30, 1.0);
const RED: Color = Color::new(1.0, 0.25, 0.33, 1.0);
const PALETTE: [Color; 7] = [
    RED,
    Color::new(1.0, 0.49, 0.22, 1.0),
    AMBER,
    Color::new(0.35, 0.94, 0.60, 1.0),
    CYAN,
    Color::new(0.49, 0.51, 1.0, 1.0),
    Color::new(0.95, 0.39, 0.78, 1.0),
];
const LOGO: [[u8; 7]; 6] = [
    [14, 17, 17, 31, 17, 17, 17], // A
    [30, 17, 17, 30, 20, 18, 17], // R
    [17, 18, 20, 24, 20, 18, 17], // K
    [14, 17, 17, 17, 17, 17, 14], // O
    [17, 25, 25, 21, 19, 19, 17], // N
    [17, 18, 20, 24, 20, 18, 17], // K
];

fn opacity(c: Color, alpha: f32) -> Color {
    Color::new(c.r, c.g, c.b, alpha)
}
fn shade(c: Color, value: f32) -> Color {
    Color::new(c.r * value, c.g * value, c.b * value, c.a)
}

pub struct View {
    pub scale: f32,
    pub x: f32,
    pub y: f32,
    font: Option<PixelFont>,
}
impl View {
    pub fn new() -> Self {
        let scale = (screen_width() / WIDTH).min(screen_height() / HEIGHT);
        Self {
            scale,
            x: (screen_width() - WIDTH * scale) / 2.0,
            y: (screen_height() - HEIGHT * scale) / 2.0,
            font: None,
        }
    }
    pub fn mouse(&self, crt: bool) -> V2 {
        let (x, y) = mouse_position();
        let mut x = (x - self.x) / self.scale / WIDTH * 2.0 - 1.0;
        let mut y = (y - self.y) / self.scale / HEIGHT * 2.0 - 1.0;
        if crt {
            // Same screen-to-scene mapping as the CRT shader, so the paddle
            // still tracks the cursor through the curved glass.
            let old_x = x;
            x *= 1.0 + CURVATURE * y * y;
            y *= 1.0 + CURVATURE * old_x * old_x;
        }
        V2::new((x * 0.5 + 0.5) * WIDTH, (y * 0.5 + 0.5) * HEIGHT)
    }
    fn scene(font: &PixelFont) -> Self {
        Self {
            scale: 1.0,
            x: 0.0,
            y: 0.0,
            font: Some(font.clone()),
        }
    }
    fn rect(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        draw_rectangle(x, y, w, h, color);
    }
    fn line(&self, a: V2, b: V2, thickness: f32, color: Color) {
        draw_line(a.x, a.y, b.x, b.y, thickness, color);
    }
    fn circle(&self, p: V2, r: f32, color: Color) {
        draw_circle(p.x, p.y, r, color);
    }
    fn frame(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        draw_rectangle_lines(x, y, w, h, 1.0, color);
    }
    fn text(&self, text: &str, x: f32, y: f32, size: f32, color: Color) {
        self.font
            .as_ref()
            .expect("scene font")
            .draw(text, x, y, size, color);
    }
    fn centered(&self, text: &str, y: f32, size: f32, color: Color) {
        let width = PixelFont::width(text, size);
        self.text(text, (WIDTH - width) / 2.0, y, size, color);
    }
    fn glow_rect(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        self.rect(x - 4.0, y - 4.0, w + 8.0, h + 8.0, opacity(color, 0.025));
        self.rect(x - 2.0, y - 2.0, w + 4.0, h + 4.0, opacity(color, 0.06));
    }
    fn logo(&self, x: f32, y: f32, cell: f32) {
        // A hand-drawn, slanted 5x7 marquee; all layers remain batched geometry.
        for (dx, dy, color) in [
            (2.0, 3.0, shade(RED, 0.35)),
            (-0.6, 0.4, opacity(CYAN, 0.45)),
            (0.0, 0.0, INK),
        ] {
            for (letter, glyph) in LOGO.iter().enumerate() {
                for (row, &bits) in glyph.iter().enumerate() {
                    for col in 0..5 {
                        if bits & (1 << (4 - col)) != 0 {
                            let skew = (6 - row) as f32 * 0.32;
                            self.rect(
                                x + (letter as f32 * 6.0 + col as f32 + skew + dx) * cell,
                                y + (row as f32 + dy) * cell,
                                cell - 0.4,
                                cell - 0.4,
                                color,
                            );
                        }
                    }
                }
            }
        }
    }
    fn digits(&self, value: u32, x: f32, y: f32, color: Color) {
        const MASKS: [u8; 10] = [63, 6, 91, 79, 102, 109, 125, 7, 127, 111];
        const SEGMENTS: [(f32, f32, f32, f32); 7] = [
            (2.0, 0.0, 10.0, 2.0),
            (12.0, 2.0, 2.0, 10.0),
            (12.0, 14.0, 2.0, 10.0),
            (2.0, 24.0, 10.0, 2.0),
            (0.0, 14.0, 2.0, 10.0),
            (0.0, 2.0, 2.0, 10.0),
            (2.0, 12.0, 10.0, 2.0),
        ];
        let mut divisor = 100_000;
        for digit in 0..6 {
            let mask = MASKS[((value / divisor) % 10) as usize];
            let x = x + digit as f32 * 22.0;
            for (i, &(sx, sy, w, h)) in SEGMENTS.iter().enumerate() {
                let lit = mask & (1 << i) != 0;
                if lit {
                    self.rect(
                        x + sx - 1.0,
                        y + sy - 1.0,
                        w + 2.0,
                        h + 2.0,
                        opacity(color, 0.08),
                    );
                }
                self.rect(
                    x + sx,
                    y + sy,
                    w,
                    h,
                    if lit { color } else { shade(color, 0.065) },
                );
            }
            divisor /= 10;
        }
    }
    fn ship(&self, x: f32, y: f32, active: bool) {
        let c = if active { INK } else { METAL };
        draw_triangle(
            vec2(x + 10.0, y),
            vec2(x + 3.0, y + 15.0),
            vec2(x + 17.0, y + 15.0),
            c,
        );
        self.rect(x, y + 10.0, 4.0, 8.0, if active { RED } else { METAL });
        self.rect(
            x + 16.0,
            y + 10.0,
            4.0,
            8.0,
            if active { RED } else { METAL },
        );
        self.rect(x + 8.0, y + 6.0, 4.0, 6.0, BG);
    }
}

#[derive(Clone, Copy, Default)]
struct Popup {
    pos: V2,
    life: f32,
    value: u32,
}

pub struct Renderer {
    crt: Crt,
    font: PixelFont,
    scratch: String,
    trails: [[V2; 12]; MAX_BALLS],
    trail_len: [usize; MAX_BALLS],
    cursor: usize,
    stars: [(V2, f32); 110],
    brick_flash: [f32; ROWS * COLS],
    previous_bricks: [u8; ROWS * COLS],
    popups: [Popup; 16],
    popup_cursor: usize,
    previous_score: u32,
    previous_level: usize,
    paddle_flash: f32,
    wall_flash: f32,
    pickup_flash: f32,
}
impl Renderer {
    pub fn new() -> Self {
        let mut seed = 0x1938_742a_u32;
        let stars = std::array::from_fn(|_| {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            let x = 80.0 + (seed % 800) as f32;
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            let y = 174.0 + (seed % 580) as f32;
            (V2::new(x, y), 0.12 + (seed % 7) as f32 * 0.026)
        });
        Self {
            crt: Crt::new(),
            font: PixelFont::new(),
            scratch: String::with_capacity(256),
            trails: [[V2::default(); 12]; MAX_BALLS],
            trail_len: [0; MAX_BALLS],
            cursor: 0,
            stars,
            brick_flash: [0.0; ROWS * COLS],
            previous_bricks: [0; ROWS * COLS],
            popups: [Popup::default(); 16],
            popup_cursor: 0,
            previous_score: 0,
            previous_level: usize::MAX,
            paddle_flash: 0.0,
            wall_flash: 0.0,
            pickup_flash: 0.0,
        }
    }
    pub fn capture(&self, path: &str) {
        self.crt.capture(path);
    }

    pub fn reset(&mut self) {
        self.trail_len.fill(0);
        self.brick_flash.fill(0.0);
        self.previous_bricks.fill(0);
        self.popups.fill(Popup::default());
        self.previous_score = 0;
        self.previous_level = usize::MAX;
        self.paddle_flash = 0.0;
        self.wall_flash = 0.0;
        self.pickup_flash = 0.0;
    }
    pub fn record(&mut self, game: &Game) {
        if self.previous_level != game.level {
            self.reset();
            self.previous_level = game.level;
            self.previous_bricks = game.bricks;
            self.previous_score = game.score;
        }
        for (i, ball) in game.balls.iter().enumerate() {
            if !ball.active || ball.held || game.phase != Phase::Playing {
                self.trail_len[i] = 0;
                continue;
            }
            self.trails[i][self.cursor] = ball.pos;
            self.trail_len[i] = (self.trail_len[i] + 1).min(12);
        }
        self.cursor = (self.cursor + 1) % 12;
        self.paddle_flash = (self.paddle_flash - DT).max(0.0);
        self.wall_flash = (self.wall_flash - DT).max(0.0);
        self.pickup_flash = (self.pickup_flash - DT).max(0.0);
        if game.events.paddle {
            self.paddle_flash = 0.16;
        }
        if game.events.wall {
            self.wall_flash = 0.12;
        }
        if game.events.pickup {
            self.pickup_flash = 0.65;
        }
        for popup in &mut self.popups {
            popup.life = (popup.life - DT).max(0.0);
            popup.pos.y -= 22.0 * DT;
        }
        let mut popup_spawned = false;
        for (i, flash) in self.brick_flash.iter_mut().enumerate() {
            *flash = (*flash - DT).max(0.0);
            if game.events.brick && game.bricks[i] < self.previous_bricks[i] {
                *flash = 0.18;
                if !popup_spawned && game.score > self.previous_score {
                    let r = Game::brick_rect(i);
                    self.popups[self.popup_cursor] = Popup {
                        pos: V2::new(r.x + r.w / 2.0, r.y),
                        life: 0.65,
                        value: (game.score - self.previous_score).saturating_sub(
                            if game.events.clear {
                                game.summary.bonus
                            } else {
                                0
                            },
                        ),
                    };
                    self.popup_cursor = (self.popup_cursor + 1) % self.popups.len();
                    popup_spawned = true;
                }
            }
        }
        self.previous_bricks = game.bricks;
        self.previous_score = game.score;
    }

    pub fn draw(
        &mut self,
        game: &Game,
        ui: &Ui,
        profile: &Profile,
        alpha: f32,
        perf: Option<&Perf>,
    ) {
        let crt = profile.crt;
        let screen = View::new();
        self.crt.begin();
        let v = View::scene(&self.font);
        clear_background(BG);
        if ui.screen != Screen::Play {
            self.background(&v);
            if ui.screen == Screen::Title {
                self.attract(&v, ui, profile);
            } else {
                self.sectors(&v, ui, profile);
            }
            if ui.save_error {
                v.centered("PROGRESS COULD NOT BE SAVED", 881.0, 11.0, AMBER);
            }
            self.crt.present(screen.x, screen.y, screen.scale, crt);
            return;
        }
        self.playfield(&v);
        self.background(&v);
        self.bricks(&v, game);
        self.effects(&v, game);
        self.paddle(&v, game);
        for drop in &game.drops {
            if drop.active {
                let c = power_color(drop.power);
                v.glow_rect(drop.pos.x - 13.0, drop.pos.y - 10.0, 26.0, 20.0, c);
                v.rect(
                    drop.pos.x - 13.0,
                    drop.pos.y - 8.0,
                    26.0,
                    16.0,
                    shade(c, 0.35),
                );
                v.rect(drop.pos.x - 10.0, drop.pos.y - 11.0, 20.0, 22.0, c);
                v.rect(
                    drop.pos.x - 9.0,
                    drop.pos.y - 9.0,
                    18.0,
                    2.0,
                    opacity(INK, 0.5),
                );
                v.rect(drop.pos.x - 7.0, drop.pos.y - 5.0, 14.0, 13.0, BG);
            }
        }
        self.balls(&v, game, alpha);
        // Keep font-atlas work together after the geometry batches.
        self.hud(&v, game, profile.best_score);
        for drop in &game.drops {
            if drop.active {
                v.text(
                    drop.power.label(),
                    drop.pos.x - 5.0,
                    drop.pos.y + 5.0,
                    15.0,
                    power_color(drop.power),
                );
            }
        }
        for popup in &self.popups {
            if popup.life > 0.0 {
                self.scratch.clear();
                let _ = write!(self.scratch, "+{}", popup.value);
                v.text(
                    &self.scratch,
                    popup.pos.x - PixelFont::width(&self.scratch, 12.0) / 2.0,
                    popup.pos.y,
                    12.0,
                    opacity(INK, (popup.life * 3.0).min(1.0)),
                );
            }
        }
        if !ui.paused && game.phase == Phase::Playing {
            if game.balls.iter().any(|b| b.active && b.held) {
                v.centered(
                    "ANCHOR / REPOSITION, THEN CLICK OR SPACE",
                    720.0,
                    11.0,
                    CYAN,
                );
            }
            if game.notice_ticks > 0
                && let Some(power) = game.notice
            {
                let fade = (game.notice_ticks as f32 / 60.0).min(1.0);
                v.centered(power.name(), 687.0, 13.0, opacity(power_color(power), fade));
            }
        }
        if ui.paused {
            v.rect(160.0, 344.0, 640.0, 385.0, opacity(BG, 0.98));
            v.centered("TAKE YOUR TIME", 403.0, 22.0, INK);
            self.menu(&v, ui.choice, ["RESUME", "RETRY SECTOR", "MAIN MENU"], None);
            v.centered("RETRY RETURNS TO THE SECTOR CHECKPOINT", 659.0, 11.0, DIM);
            self.options(&v, profile, 699.0);
        } else {
            match game.phase {
                Phase::Ready => {
                    self.scratch.clear();
                    let _ = write!(
                        self.scratch,
                        "{:02} / {}",
                        game.level + 1,
                        LEVELS[game.level].name
                    );
                    v.centered(&self.scratch, 560.0, 22.0, INK);
                    v.centered(CHAPTERS[LEVELS[game.level].chapter], 593.0, 11.0, AMBER);
                    v.centered("SPACE / CLICK TO SERVE", 646.0, 13.0, CYAN);
                    v.centered(LEVELS[game.level].tip, 680.0, 11.0, DIM);
                    let start = game.balls[0].pos;
                    let direction = game.launch_velocity().normalized();
                    for i in 1..=5 {
                        v.circle(
                            start + direction * (14.0 * i as f32),
                            1.0,
                            opacity(CYAN, 0.40 - i as f32 * 0.05),
                        );
                    }
                }
                Phase::Cleared => self.cleared(&v, game),
                Phase::GameOver | Phase::Victory => {
                    v.rect(160.0, 318.0, 640.0, 420.0, opacity(BG, 0.98));
                    v.centered(
                        if game.phase == Phase::Victory {
                            "JOURNEY COMPLETE"
                        } else {
                            "ONE MORE ORBIT?"
                        },
                        395.0,
                        22.0,
                        INK,
                    );
                    self.scratch.clear();
                    let _ = write!(
                        self.scratch,
                        "{:06} POINTS / {} MEDALS",
                        game.score,
                        profile.medals()
                    );
                    v.centered(&self.scratch, 432.0, 11.0, AMBER);
                    self.menu(
                        &v,
                        ui.choice,
                        [
                            "SECTOR SELECT",
                            if game.phase == Phase::Victory {
                                "NEW JOURNEY"
                            } else {
                                "RETRY SECTOR"
                            },
                            "MAIN MENU",
                        ],
                        None,
                    );
                    v.centered("YOUR PROGRESS IS SAVED", 699.0, 11.0, DIM);
                }
                Phase::Playing => {}
            }
        }
        self.scratch.clear();
        let _ = write!(
            self.scratch,
            "{:02} / {}",
            game.level + 1,
            if game.mode == Mode::Practice {
                "PRACTICE"
            } else {
                LEVELS[game.level].name
            }
        );
        v.text(&self.scratch, LEFT, BOTTOM + 34.0, 11.0, opacity(DIM, 0.7));
        for i in 0..LEVEL_COUNT {
            v.rect(
                421.0 + i as f32 * 10.0,
                BOTTOM + 27.0,
                5.0,
                3.0,
                if i == game.level {
                    CYAN
                } else if profile.records[i].medals > 0 {
                    shade(AMBER, 0.6)
                } else {
                    shade(DIM, 0.25)
                },
            );
        }
        v.text(
            "P PAUSE",
            RIGHT - PixelFont::width("P PAUSE", 11.0),
            BOTTOM + 34.0,
            11.0,
            opacity(DIM, 0.65),
        );
        if ui.save_error && (ui.paused || game.phase != Phase::Playing) {
            v.centered("PROGRESS COULD NOT BE SAVED", 881.0, 11.0, AMBER);
        }
        if let Some(perf) = perf {
            v.rect(80.0, 440.0, 416.0, 184.0, opacity(BG, 0.97));
            v.frame(80.0, 440.0, 416.0, 184.0, shade(CYAN, 0.4));
            v.text("PERFORMANCE / CPU", 96.0, 467.0, 14.0, CYAN);
            for (i, line) in perf.lines.iter().enumerate() {
                v.text(line, 96.0, 492.0 + i as f32 * 22.0, 11.0, INK);
            }
        }
        self.crt.present(screen.x, screen.y, screen.scale, crt);
    }

    fn playfield(&self, v: &View) {
        v.rect(LEFT, TOP, RIGHT - LEFT, BOTTOM - TOP, GLASS);
        let edge = opacity(CYAN, 0.15 + self.wall_flash * 2.0);
        v.line(V2::new(LEFT, TOP), V2::new(LEFT, BOTTOM), 1.0, edge);
        v.line(V2::new(RIGHT, TOP), V2::new(RIGHT, BOTTOM), 1.0, edge);
        v.line(V2::new(LEFT, TOP), V2::new(RIGHT, TOP), 1.0, edge);
        v.line(
            V2::new(LEFT, BOTTOM),
            V2::new(RIGHT, BOTTOM),
            1.0,
            opacity(RED, 0.15),
        );
    }
    fn background(&self, v: &View) {
        for &(pos, light) in &self.stars {
            v.rect(pos.x, pos.y, 1.0, 1.0, opacity(DIM, light));
        }
    }
    fn bricks(&self, v: &View, game: &Game) {
        // Quiet connections make the actual orthogonal blast routes readable.
        for (i, &core) in game.cores.iter().enumerate() {
            if !core || game.bricks[i] == 0 {
                continue;
            }
            let r = Game::brick_rect(i);
            for (valid, other) in [
                (i % COLS + 1 < COLS, i + 1),
                (i / COLS + 1 < ROWS, i + COLS),
            ] {
                if valid && game.cores[other] && game.bricks[other] > 0 {
                    let next = Game::brick_rect(other);
                    v.line(
                        V2::new(r.x + r.w / 2.0, r.y + r.h / 2.0),
                        V2::new(next.x + next.w / 2.0, next.y + next.h / 2.0),
                        1.0,
                        opacity(AMBER, 0.32),
                    );
                }
            }
        }
        let pulse = 0.7 + 0.15 * (game.phase_ticks as f32 * DT * 2.0).sin();
        for (i, &hp) in game.bricks.iter().enumerate() {
            let r = Game::brick_rect(i);
            let c = if game.cores[i] {
                AMBER
            } else {
                sector_color(i / COLS, LEVELS[game.level].chapter)
            };
            let flash = self.brick_flash[i] / 0.18;
            if hp == 0 {
                if flash > 0.0 {
                    v.frame(
                        r.x - (1.0 - flash) * 5.0,
                        r.y - (1.0 - flash) * 5.0,
                        r.w + (1.0 - flash) * 10.0,
                        r.h + (1.0 - flash) * 10.0,
                        opacity(c, flash * 0.75),
                    );
                }
                continue;
            }
            v.glow_rect(r.x, r.y, r.w, r.h, c);
            v.rect(r.x, r.y + 3.0, r.w, r.h, shade(c, 0.20));
            v.rect(r.x, r.y, r.w, r.h - 1.0, shade(c, 0.88));
            v.rect(
                r.x + 2.0,
                r.y + 4.0,
                r.w - 4.0,
                r.h - 9.0,
                shade(c, if hp > 1 { 0.48 } else { 0.70 }),
            );
            v.rect(r.x + 1.0, r.y + 1.0, r.w - 2.0, 2.0, c);
            v.rect(r.x + 1.0, r.y + 3.0, 1.0, r.h - 6.0, opacity(INK, 0.42));
            v.rect(r.x + r.w - 2.0, r.y + 3.0, 2.0, r.h - 3.0, shade(c, 0.35));
            v.rect(r.x + 2.0, r.y + r.h - 3.0, r.w - 4.0, 2.0, shade(c, 0.30));
            for x in [r.x + 5.0, r.x + r.w - 6.0] {
                v.rect(x, r.y + 9.0, 2.0, 2.0, opacity(INK, 0.5));
            }
            if game.cores[i] {
                v.rect(
                    r.x + 3.0,
                    r.y + 4.0,
                    r.w - 6.0,
                    r.h - 9.0,
                    shade(AMBER, 0.13),
                );
                let center = V2::new(r.x + r.w / 2.0, r.y + 11.0);
                for (from, to) in [
                    (V2::new(-6.0, 0.0), V2::new(0.0, -5.0)),
                    (V2::new(0.0, -5.0), V2::new(6.0, 0.0)),
                    (V2::new(6.0, 0.0), V2::new(0.0, 5.0)),
                    (V2::new(0.0, 5.0), V2::new(-6.0, 0.0)),
                ] {
                    v.line(center + from, center + to, 1.0, opacity(AMBER, pulse));
                }
                v.rect(center.x - 1.0, center.y - 1.0, 2.0, 2.0, INK);
            } else if hp > 1 {
                for j in 0..hp {
                    let x =
                        r.x + r.w / 2.0 - (f32::from(hp) * 12.0 - 6.0) / 2.0 + f32::from(j) * 12.0;
                    v.rect(x, r.y + 8.0, 6.0, 6.0, INK);
                }
            } else {
                v.rect(r.x + 22.0, r.y + 10.0, 14.0, 1.0, opacity(c, 0.5));
            }
            if flash > 0.0 {
                v.rect(r.x, r.y, r.w, r.h, opacity(INK, flash * 0.8));
            }
        }
    }
    fn effects(&self, v: &View, game: &Game) {
        for (index, &flash) in game.relay_flash.iter().enumerate() {
            if flash == 0 {
                continue;
            }
            let r = Game::brick_rect(index);
            let progress = 1.0 - f32::from(flash) / 36.0;
            let center = V2::new(r.x + r.w / 2.0, r.y + r.h / 2.0);
            let color = opacity(AMBER, (1.0 - progress) * 0.75);
            v.frame(
                r.x - progress * 8.0,
                r.y - progress * 5.0,
                r.w + progress * 16.0,
                r.h + progress * 10.0,
                color,
            );
            for direction in [
                V2::new(CELL_W, 0.0),
                V2::new(-CELL_W, 0.0),
                V2::new(0.0, CELL_H),
                V2::new(0.0, -CELL_H),
            ] {
                v.line(
                    center + direction * progress * 0.6,
                    center + direction * progress,
                    1.0,
                    color,
                );
            }
        }
        for p in &game.particles {
            if p.life <= 0.0 {
                continue;
            }
            let c = opacity(
                sector_color(p.hue % 7, LEVELS[game.level].chapter),
                (p.life * 3.0).min(1.0),
            );
            v.line(
                p.pos - p.velocity * 0.012,
                p.pos,
                1.0,
                opacity(c, c.a * 0.5),
            );
            v.rect(p.pos.x - 1.0, p.pos.y - 1.0, 2.0, 2.0, c);
        }
        if self.pickup_flash > 0.0 {
            let progress = 1.0 - self.pickup_flash / 0.65;
            draw_circle_lines(
                game.paddle_x,
                PADDLE_Y,
                20.0 + progress * 90.0,
                2.0,
                opacity(CYAN, (1.0 - progress) * 0.6),
            );
        }
    }
    fn paddle(&self, v: &View, game: &Game) {
        let paddle = game.paddle_x;
        let x = paddle - game.paddle_width / 2.0;
        let w = game.paddle_width;
        v.glow_rect(x - 2.0, PADDLE_Y, w + 4.0, 16.0, CYAN);
        v.rect(x + 4.0, PADDLE_Y + 4.0, w - 8.0, 14.0, shade(CYAN, 0.16));
        v.rect(x + 3.0, PADDLE_Y, w - 6.0, 14.0, INK);
        v.rect(
            x + 13.0,
            PADDLE_Y + 4.0,
            w - 26.0,
            6.0,
            Color::new(0.34, 0.52, 0.62, 1.0),
        );
        v.rect(x + 13.0, PADDLE_Y + 4.0, w - 26.0, 2.0, CYAN);
        v.rect(paddle - 9.0, PADDLE_Y + 4.0, 18.0, 6.0, GLASS);
        v.rect(paddle - 5.0, PADDLE_Y + 5.0, 10.0, 2.0, AMBER);
        for end in [x, x + w - 12.0] {
            v.rect(end, PADDLE_Y + 2.0, 12.0, 10.0, RED);
            v.rect(end + 3.0, PADDLE_Y, 6.0, 14.0, RED);
            v.rect(end + 3.0, PADDLE_Y + 2.0, 6.0, 2.0, opacity(INK, 0.5));
            v.rect(end + 4.0, PADDLE_Y + 15.0, 4.0, 4.0, opacity(CYAN, 0.35));
        }
        if game.anchor_charges > 0 || game.balls.iter().any(|b| b.active && b.held) {
            v.rect(x + 15.0, PADDLE_Y - 2.0, w - 30.0, 1.0, CYAN);
            for i in 0..3 {
                v.rect(
                    paddle - 8.0 + i as f32 * 7.0,
                    PADDLE_Y + 21.0,
                    3.0,
                    2.0,
                    if i < game.anchor_charges {
                        CYAN
                    } else {
                        shade(CYAN, 0.18)
                    },
                );
            }
        }
        if game.wide_time > 0.0 {
            v.rect(
                x,
                PADDLE_Y + 27.0,
                w * (game.wide_time / 14.0).min(1.0),
                1.0,
                power_color(Power::Wide),
            );
        }
        if game.slow_time > 0.0 {
            v.rect(
                x,
                PADDLE_Y + 30.0,
                w * (game.slow_time / 12.0).min(1.0),
                1.0,
                AMBER,
            );
        }
        if self.paddle_flash > 0.0 {
            v.rect(x, PADDLE_Y, w, 14.0, opacity(INK, self.paddle_flash * 4.0));
        }
    }
    fn balls(&self, v: &View, game: &Game, alpha: f32) {
        for (i, ball) in game.balls.iter().enumerate() {
            if !ball.active {
                continue;
            }
            if ball.held {
                let mut point = ball.pos;
                let mut direction = ball.velocity.normalized();
                for n in 1..=10 {
                    point += direction * 12.0;
                    if point.x < LEFT + RADIUS {
                        point.x = 2.0 * (LEFT + RADIUS) - point.x;
                        direction.x = -direction.x;
                    }
                    if point.x > RIGHT - RADIUS {
                        point.x = 2.0 * (RIGHT - RADIUS) - point.x;
                        direction.x = -direction.x;
                    }
                    v.circle(point, 1.0, opacity(CYAN, 0.55 - n as f32 * 0.035));
                }
            }
            let color = if ball.phase_hits > 0 {
                PALETTE[5]
            } else {
                CYAN
            };
            for n in (0..if ball.held { 0 } else { self.trail_len[i] }).rev() {
                let index = (self.cursor + 12 - 1 - n) % 12;
                let c = opacity(color, 0.28 * (1.0 - n as f32 / 12.0));
                v.circle(self.trails[i][index], RADIUS * (1.0 - n as f32 / 15.0), c);
            }
            let pos = if ball.held {
                ball.pos
            } else {
                ball.previous.lerp(ball.pos, alpha)
            };
            v.circle(pos, 15.0, opacity(color, 0.035));
            v.circle(pos, 10.0, opacity(color, 0.20));
            for n in 0..ball.phase_hits {
                v.rect(
                    pos.x - (f32::from(ball.phase_hits) * 4.0 - 2.0) / 2.0 + f32::from(n) * 4.0,
                    pos.y + 13.0,
                    2.0,
                    2.0,
                    color,
                );
            }
            v.circle(pos, RADIUS, INK);
            v.rect(pos.x - 3.0, pos.y - 3.0, 3.0, 3.0, WHITE);
        }
    }
    fn hud(&self, v: &View, game: &Game, high: u32) {
        // A sparse arcade scoreboard: gameplay information, aligned to the field.
        v.text("1UP", 96.0, 66.0, 11.0, RED);
        v.digits(game.score, 96.0, 80.0, AMBER);
        v.text("HI-SCORE", 416.0, 66.0, 11.0, DIM);
        v.digits(high.max(game.score), 416.0, 80.0, INK);
        for i in 0..game.lives.max(3) {
            v.ship(850.0 - i as f32 * 29.0, 85.0, i < game.lives);
        }
    }
    fn options(&mut self, v: &View, profile: &Profile, y: f32) {
        self.scratch.clear();
        let _ = write!(
            self.scratch,
            "M SOUND {}   C CRT {}   F FULLSCREEN",
            if profile.muted { "OFF" } else { "ON" },
            if profile.crt { "ON" } else { "OFF" }
        );
        v.centered(&self.scratch, y, 11.0, DIM);
        self.scratch.clear();
        let _ = write!(
            self.scratch,
            "VOLUME {} / 10   [ LOWER / ] HIGHER",
            profile.volume
        );
        v.centered(&self.scratch, y + 27.0, 11.0, shade(DIM, 0.7));
    }
    fn menu(&self, v: &View, selected: usize, labels: [&str; 3], disabled: Option<usize>) {
        for (i, label) in labels.iter().enumerate() {
            let rect = ui::menu_rect(i);
            let available = disabled != Some(i);
            let color = if !available {
                shade(DIM, 0.4)
            } else if i == selected {
                CYAN
            } else {
                INK
            };
            if i == selected && available {
                v.rect(rect.x, rect.y, rect.w, rect.h, opacity(CYAN, 0.035));
                v.rect(rect.x, rect.y + 15.0, 2.0, 13.0, CYAN);
            }
            v.centered(label, rect.y + 28.0, 14.0, color);
        }
    }
    fn attract(&mut self, v: &View, ui: &Ui, profile: &Profile) {
        v.logo(254.0, 188.0, 11.5);
        v.centered("BREAK THE COSMOS", 342.0, 13.0, AMBER);
        self.scratch.clear();
        let _ = write!(
            self.scratch,
            "{:02} / 12 SECTORS   {:02} / 36 MEDALS",
            profile.unlocked,
            profile.medals()
        );
        v.centered(&self.scratch, 406.0, 11.0, DIM);
        self.menu(
            v,
            ui.choice,
            ["CONTINUE JOURNEY", "NEW JOURNEY", "SECTOR SELECT"],
            profile.checkpoint.is_none().then_some(0),
        );
        if let Some(c) = profile.checkpoint {
            self.scratch.clear();
            let _ = write!(
                self.scratch,
                "SAVED AT {:02} / {}",
                c.level + 1,
                LEVELS[c.level].name
            );
            v.centered(&self.scratch, 671.0, 11.0, DIM);
        } else {
            v.centered("A SMALL JOURNEY THROUGH TWELVE SECTORS", 671.0, 11.0, DIM);
        }
        v.centered("MOUSE / ARROWS TO MOVE   SPACE TO SERVE", 714.0, 11.0, DIM);
        self.options(v, profile, 778.0);
        if profile.best_score > 0 {
            self.scratch.clear();
            let _ = write!(self.scratch, "PERSONAL BEST {:06}", profile.best_score);
            v.centered(&self.scratch, 859.0, 11.0, shade(AMBER, 0.75));
        }
    }
    fn sectors(&mut self, v: &View, ui: &Ui, profile: &Profile) {
        v.text("ESC / BACK", 105.0, 127.0, 11.0, DIM);
        v.centered("YOUR ORBIT", 130.0, 22.0, INK);
        v.centered(
            "RETURN TO A FAVORITE. MAKE IT A LITTLE BETTER.",
            169.0,
            11.0,
            DIM,
        );
        for (chapter, title) in CHAPTERS.iter().enumerate() {
            v.text(
                title,
                117.0 + chapter as f32 * 255.0,
                217.0,
                11.0,
                sector_color(0, chapter),
            );
        }
        for (i, level) in LEVELS.iter().enumerate() {
            let r = ui::sector_rect(i);
            let unlocked = i < profile.unlocked;
            let selected = i == ui.sector;
            v.rect(
                r.x,
                r.y,
                r.w,
                r.h,
                if selected {
                    opacity(CYAN, 0.065)
                } else {
                    opacity(DIM, 0.018)
                },
            );
            if selected {
                v.line(
                    V2::new(r.x, r.y),
                    V2::new(r.x, r.y + r.h),
                    2.0,
                    if unlocked { CYAN } else { DIM },
                );
            }
            self.scratch.clear();
            let _ = write!(self.scratch, "{:02} / {}", i + 1, level.name);
            v.text(
                &self.scratch,
                r.x + 12.0,
                r.y + 21.0,
                11.0,
                if unlocked { INK } else { shade(DIM, 0.55) },
            );
            for (row, pattern) in level.rows.iter().enumerate() {
                for (col, hp) in pattern.bytes().enumerate() {
                    if hp != b'.' {
                        v.rect(
                            r.x + 12.0 + col as f32 * 9.0,
                            r.y + 34.0 + row as f32 * 7.0,
                            7.0,
                            4.0,
                            if unlocked {
                                if hp == b'R' {
                                    AMBER
                                } else {
                                    shade(sector_color(row, level.chapter), 0.8)
                                }
                            } else {
                                shade(DIM, 0.16)
                            },
                        );
                    }
                }
            }
            if unlocked {
                for j in 0..3 {
                    v.circle(
                        V2::new(r.x + 163.0 + j as f32 * 19.0, r.y + 48.0),
                        3.0,
                        if profile.records[i].medals & (1 << j) != 0 {
                            AMBER
                        } else {
                            shade(DIM, 0.22)
                        },
                    );
                }
                v.text(
                    if profile.records[i].medals > 0 {
                        "CLEARED"
                    } else {
                        "OPEN"
                    },
                    r.x + 149.0,
                    r.y + 78.0,
                    11.0,
                    DIM,
                );
            } else {
                v.text("LOCKED", r.x + 149.0, r.y + 67.0, 11.0, shade(DIM, 0.45));
            }
        }
        let record = profile.records[ui.sector];
        self.scratch.clear();
        if record.best_ticks > 0 {
            let seconds = record.best_ticks / TICK_HZ;
            let _ = write!(
                self.scratch,
                "BEST {:02}:{:02}   /   SWIFT UNDER {} SECONDS",
                seconds / 60,
                seconds % 60,
                LEVELS[ui.sector].par_seconds
            );
        } else {
            let _ = write!(
                self.scratch,
                "SWIFT TARGET / {} SECONDS",
                LEVELS[ui.sector].par_seconds
            );
        }
        v.centered(&self.scratch, 771.0, 11.0, AMBER);
        v.centered(
            "CLEAR / FINISH   CLEAN / NO LIVES LOST   SWIFT / BEAT THE CLOCK",
            809.0,
            11.0,
            DIM,
        );
        v.centered(
            if ui.sector < profile.unlocked {
                "ENTER / CLICK TO PRACTICE"
            } else {
                "CLEAR THE PREVIOUS SECTOR TO UNLOCK"
            },
            854.0,
            13.0,
            if ui.sector < profile.unlocked {
                CYAN
            } else {
                DIM
            },
        );
    }
    fn cleared(&mut self, v: &View, game: &Game) {
        v.rect(160.0, 439.0, 640.0, 269.0, opacity(BG, 0.98));
        v.centered("SECTOR CLEAR", 485.0, 22.0, INK);
        let seconds = game.summary.ticks / TICK_HZ;
        self.scratch.clear();
        let _ = write!(
            self.scratch,
            "{:02}:{:02}   +{} POINTS   {} CHAIN",
            seconds / 60,
            seconds % 60,
            game.summary.bonus,
            game.summary.best_combo
        );
        v.centered(&self.scratch, 526.0, 11.0, AMBER);
        for (i, label) in ["CLEAR", "CLEAN", "SWIFT"].iter().enumerate() {
            let x = 334.0 + i as f32 * 126.0;
            let earned = game.summary.medals & (1 << i) != 0;
            v.circle(
                V2::new(x - 13.0, 566.0),
                3.0,
                if earned { AMBER } else { shade(DIM, 0.3) },
            );
            v.text(
                label,
                x,
                571.0,
                11.0,
                if earned { INK } else { shade(DIM, 0.5) },
            );
        }
        if game.summary.life_earned {
            v.centered("CHAPTER COMPLETE / EXTRA LIFE", 610.0, 11.0, CYAN);
        }
        v.rect(280.0, 622.0, 400.0, 44.0, opacity(CYAN, 0.045));
        v.centered(
            if game.mode == Mode::Practice {
                "RETURN TO SECTORS"
            } else {
                "CONTINUE JOURNEY"
            },
            651.0,
            14.0,
            CYAN,
        );
        v.centered("ENTER / CLICK", 690.0, 11.0, DIM);
    }
}

fn sector_color(row: usize, chapter: usize) -> Color {
    const BLUE: [usize; 7] = [4, 4, 5, 5, 6, 5, 4];
    const DUSK: [usize; 7] = [6, 0, 1, 2, 1, 0, 6];
    PALETTE[match chapter {
        1 => BLUE[row % 7],
        2 => DUSK[row % 7],
        _ => row % 7,
    }]
}
fn power_color(power: Power) -> Color {
    match power {
        Power::Wide => PALETTE[3],
        Power::Slow => AMBER,
        Power::Multi => PALETTE[6],
        Power::Anchor => CYAN,
        Power::Phase => PALETTE[5],
    }
}
