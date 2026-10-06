use crate::{
    crt::{CURVATURE, Crt},
    perf::Perf,
    pixel_font::PixelFont,
};
use arkonk::{game::*, physics::V2};
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
            paddle_flash: 0.0,
            wall_flash: 0.0,
            pickup_flash: 0.0,
        }
    }
    pub fn reset(&mut self) {
        self.trail_len.fill(0);
        self.brick_flash.fill(0.0);
        self.previous_bricks.fill(0);
        self.popups.fill(Popup::default());
        self.previous_score = 0;
        self.paddle_flash = 0.0;
        self.wall_flash = 0.0;
        self.pickup_flash = 0.0;
    }
    pub fn record(&mut self, game: &Game) {
        for (i, ball) in game.balls.iter().enumerate() {
            if !ball.active || game.phase != Phase::Playing {
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
                        value: game.score - self.previous_score,
                    };
                    self.popup_cursor = (self.popup_cursor + 1) % self.popups.len();
                    popup_spawned = true;
                }
            }
        }
        self.previous_bricks = game.bricks;
        self.previous_score = game.score;
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &mut self,
        game: &Game,
        alpha: f32,
        started: bool,
        paused: bool,
        high: u32,
        muted: bool,
        perf: Option<&Perf>,
        crt: bool,
    ) {
        let screen = View::new();
        self.crt.begin();
        let v = View::scene(&self.font);
        clear_background(BG);
        if !started {
            self.background(&v);
            self.attract(&v, high, muted, crt);
            self.crt.present(screen.x, screen.y, screen.scale, crt);
            return;
        }
        self.playfield(&v);
        self.background(&v);
        self.bricks(&v, game);
        self.effects(&v, game);
        self.paddle(&v, game, alpha);
        self.balls(&v, game, alpha);
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
        // Keep font-atlas work together after the geometry batches.
        self.hud(&v, game, high);
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
        self.scratch.clear();
        if game.wide_time > 0.0 {
            let _ = write!(self.scratch, "WIDE {:02.0}   ", game.wide_time.ceil());
        }
        if game.slow_time > 0.0 {
            let _ = write!(self.scratch, "SLOW {:02.0}", game.slow_time.ceil());
        }
        v.centered(&self.scratch, 739.0, 12.0, CYAN);
        if paused {
            self.overlay(&v, "PAUSED", "SPACE / P / ESC TO RESUME", CYAN);
            self.options(&v, muted, crt, 667.0);
            v.centered("R RESTART / Q QUIT", 696.0, 11.0, DIM);
        } else {
            match game.phase {
                Phase::Ready => {
                    v.centered("P L A Y E R   O N E", 574.0, 22.0, INK);
                    self.scratch.clear();
                    let _ = write!(
                        self.scratch,
                        "SECTOR 0{} / {}",
                        game.level + 1,
                        LEVELS[game.level]
                    );
                    v.centered(&self.scratch, 603.0, 13.0, AMBER);
                    v.centered("SPACE / CLICK TO SERVE", 649.0, 15.0, CYAN);
                }
                Phase::GameOver => {
                    self.overlay(&v, "G A M E  O V E R", "R / ENTER FOR ONE MORE RUN", RED)
                }
                Phase::Victory => self.overlay(
                    &v,
                    "H I G H  O R B I T",
                    "ALL SECTORS CLEAR / R TO PLAY AGAIN",
                    AMBER,
                ),
                Phase::Playing => {}
            }
        }
        if !paused && game.phase == Phase::Playing {
            self.scratch.clear();
            let _ = write!(self.scratch, "SECTOR 0{}", game.level + 1);
            v.text(&self.scratch, LEFT, BOTTOM + 34.0, 11.0, opacity(DIM, 0.65));
            v.text(
                "P PAUSE",
                RIGHT - PixelFont::width("P PAUSE", 11.0),
                BOTTOM + 34.0,
                11.0,
                opacity(DIM, 0.65),
            );
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
        for (i, &hp) in game.bricks.iter().enumerate() {
            let r = Game::brick_rect(i);
            let c = PALETTE[i / COLS];
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
                shade(c, if hp == 2 { 0.48 } else { 0.70 }),
            );
            v.rect(r.x + 1.0, r.y + 1.0, r.w - 2.0, 2.0, c);
            v.rect(r.x + 1.0, r.y + 3.0, 1.0, r.h - 6.0, opacity(INK, 0.42));
            v.rect(r.x + r.w - 2.0, r.y + 3.0, 2.0, r.h - 3.0, shade(c, 0.35));
            v.rect(r.x + 2.0, r.y + r.h - 3.0, r.w - 4.0, 2.0, shade(c, 0.30));
            for x in [r.x + 5.0, r.x + r.w - 6.0] {
                v.rect(x, r.y + 9.0, 2.0, 2.0, opacity(INK, 0.5));
            }
            if hp == 2 {
                for x in [r.x + 20.0, r.x + 32.0] {
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
        for p in &game.particles {
            if p.life <= 0.0 {
                continue;
            }
            let c = opacity(PALETTE[p.hue % 7], (p.life * 3.0).min(1.0));
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
    fn paddle(&self, v: &View, game: &Game, alpha: f32) {
        let paddle = game.paddle_previous + (game.paddle_x - game.paddle_previous) * alpha;
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
        if self.paddle_flash > 0.0 {
            v.rect(x, PADDLE_Y, w, 14.0, opacity(INK, self.paddle_flash * 4.0));
        }
    }
    fn balls(&self, v: &View, game: &Game, alpha: f32) {
        for (i, ball) in game.balls.iter().enumerate() {
            if !ball.active {
                continue;
            }
            for n in (0..self.trail_len[i]).rev() {
                let index = (self.cursor + 12 - 1 - n) % 12;
                let c = opacity(CYAN, 0.28 * (1.0 - n as f32 / 12.0));
                v.circle(self.trails[i][index], RADIUS * (1.0 - n as f32 / 15.0), c);
            }
            let pos = ball.previous.lerp(ball.pos, alpha);
            v.circle(pos, 15.0, opacity(CYAN, 0.035));
            v.circle(pos, 10.0, opacity(CYAN, 0.12));
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
        for i in 0..3 {
            v.ship(790.0 + i as f32 * 29.0, 85.0, i < game.lives);
        }
    }
    fn options(&self, v: &View, muted: bool, crt: bool, y: f32) {
        v.text(
            if muted { "M SOUND OFF" } else { "M SOUND ON" },
            288.0,
            y,
            11.0,
            DIM,
        );
        v.text(
            if crt { "C CRT ON" } else { "C CRT OFF" },
            447.0,
            y,
            11.0,
            DIM,
        );
        v.text("F FULLSCREEN", 594.0, y, 11.0, DIM);
    }
    fn attract(&mut self, v: &View, high: u32, muted: bool, crt: bool) {
        // The marquee belongs to the title screen, not the in-game scoreboard.
        v.logo(254.0, 287.0, 11.5);
        v.centered("BREAK THE COSMOS", 443.0, 14.0, AMBER);
        if high > 0 {
            self.scratch.clear();
            let _ = write!(self.scratch, "HI-SCORE {high:06}");
            v.centered(&self.scratch, 488.0, 11.0, DIM);
        }
        let pulse = 0.75 + 0.25 * (get_time() as f32 * 3.0).sin();
        v.centered("PRESS START", 559.0, 22.0, opacity(CYAN, pulse));
        v.centered("SPACE / ENTER / CLICK", 591.0, 11.0, INK);
        v.centered("MOUSE / ARROWS TO MOVE", 647.0, 11.0, DIM);
        v.centered("W WIDE / S SLOW / M MULTIBALL", 676.0, 11.0, DIM);
        self.options(v, muted, crt, 749.0);
    }
    fn overlay(&self, v: &View, title: &str, subtitle: &str, color: Color) {
        v.rect(192.0, 494.0, 576.0, 155.0, opacity(BG, 0.97));
        v.centered(title, 568.0, 29.0, color);
        v.centered(subtitle, 611.0, 13.0, INK);
    }
}

fn power_color(power: Power) -> Color {
    match power {
        Power::Wide => PALETTE[3],
        Power::Slow => AMBER,
        Power::Multi => PALETTE[6],
    }
}
