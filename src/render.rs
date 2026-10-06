use crate::perf::Perf;
use arkonk::{game::*, physics::V2};
use macroquad::prelude::*;
use std::fmt::Write;

const BG: Color = Color::new(0.035, 0.047, 0.067, 1.0);
const PANEL: Color = Color::new(0.061, 0.082, 0.106, 1.0);
const INK: Color = Color::new(0.88, 0.94, 0.94, 1.0);
const DIM: Color = Color::new(0.39, 0.48, 0.53, 1.0);
const MINT: Color = Color::new(0.40, 0.94, 0.77, 1.0);
const ORANGE: Color = Color::new(1.0, 0.58, 0.36, 1.0);
const PALETTE: [Color; 7] = [
    ORANGE,
    Color::new(0.98, 0.72, 0.40, 1.0),
    Color::new(0.88, 0.84, 0.52, 1.0),
    MINT,
    Color::new(0.32, 0.75, 0.72, 1.0),
    Color::new(0.37, 0.61, 0.72, 1.0),
    Color::new(0.48, 0.53, 0.72, 1.0),
];

pub struct View {
    pub scale: f32,
    pub x: f32,
    pub y: f32,
}
impl View {
    pub fn new() -> Self {
        let scale = (screen_width() / WIDTH).min(screen_height() / HEIGHT);
        Self {
            scale,
            x: (screen_width() - WIDTH * scale) / 2.0,
            y: (screen_height() - HEIGHT * scale) / 2.0,
        }
    }
    pub fn mouse(&self) -> V2 {
        let (x, y) = mouse_position();
        V2::new((x - self.x) / self.scale, (y - self.y) / self.scale)
    }
    fn rect(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        draw_rectangle(
            self.x + x * self.scale,
            self.y + y * self.scale,
            w * self.scale,
            h * self.scale,
            color,
        );
    }
    fn line(&self, a: V2, b: V2, thickness: f32, color: Color) {
        draw_line(
            self.x + a.x * self.scale,
            self.y + a.y * self.scale,
            self.x + b.x * self.scale,
            self.y + b.y * self.scale,
            thickness * self.scale,
            color,
        );
    }
    fn circle(&self, p: V2, r: f32, color: Color) {
        draw_circle(
            self.x + p.x * self.scale,
            self.y + p.y * self.scale,
            r * self.scale,
            color,
        );
    }
    fn text(&self, text: &str, x: f32, y: f32, size: f32, color: Color) {
        draw_text_ex(
            text,
            self.x + x * self.scale,
            self.y + y * self.scale,
            TextParams {
                font_size: 40,
                font_scale: size / 40.0 * self.scale,
                color,
                ..Default::default()
            },
        );
    }
    fn centered(&self, text: &str, y: f32, size: f32, color: Color) {
        let width = measure_text(text, None, 40, size / 40.0).width;
        self.text(text, (WIDTH - width) / 2.0, y, size, color);
    }
}

pub struct Renderer {
    scratch: String,
    trails: [[V2; 12]; MAX_BALLS],
    trail_len: [usize; MAX_BALLS],
    cursor: usize,
}
impl Renderer {
    pub fn new() -> Self {
        Self {
            scratch: String::with_capacity(256),
            trails: [[V2::default(); 12]; MAX_BALLS],
            trail_len: [0; MAX_BALLS],
            cursor: 0,
        }
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
    ) {
        let v = View::new();
        clear_background(BG);
        v.text("A R K O N K", 64.0, 64.0, 31.0, INK);
        v.text("BRICK BY BRICK. SIGNAL BY SIGNAL.", 64.0, 88.0, 12.0, DIM);
        v.text("SCORE", 420.0, 48.0, 12.0, DIM);
        self.scratch.clear();
        let _ = write!(self.scratch, "{:06}", game.score);
        v.text(&self.scratch, 420.0, 80.0, 27.0, MINT);
        v.text("BEST", 608.0, 48.0, 12.0, DIM);
        self.scratch.clear();
        let _ = write!(self.scratch, "{:06}", high.max(game.score));
        v.text(&self.scratch, 608.0, 80.0, 27.0, INK);
        v.text("LIVES", 808.0, 48.0, 12.0, DIM);
        for i in 0..3 {
            v.rect(
                809.0 + i as f32 * 27.0,
                67.0,
                19.0,
                8.0,
                if i < game.lives { MINT } else { PANEL },
            );
        }
        v.rect(LEFT, TOP, RIGHT - LEFT, BOTTOM - TOP, PANEL);
        // Static geometry stays in the same texture/material batch.
        for x in (80..890).step_by(24) {
            for y in (150..804).step_by(24) {
                v.rect(
                    x as f32,
                    y as f32,
                    1.0,
                    1.0,
                    Color::new(0.11, 0.15, 0.18, 1.0),
                );
            }
        }
        v.line(
            V2::new(LEFT, BOTTOM),
            V2::new(LEFT, TOP),
            2.0,
            Color::new(0.17, 0.26, 0.29, 1.0),
        );
        v.line(V2::new(LEFT, TOP), V2::new(RIGHT, TOP), 2.0, MINT);
        v.line(
            V2::new(RIGHT, TOP),
            V2::new(RIGHT, BOTTOM),
            2.0,
            Color::new(0.17, 0.26, 0.29, 1.0),
        );
        v.line(
            V2::new(LEFT, BOTTOM),
            V2::new(RIGHT, BOTTOM),
            1.0,
            Color::new(0.38, 0.20, 0.18, 1.0),
        );
        for (i, &hp) in game.bricks.iter().enumerate() {
            if hp == 0 {
                continue;
            }
            let r = Game::brick_rect(i);
            let c = PALETTE[i / COLS];
            v.rect(
                r.x,
                r.y + 3.0,
                r.w,
                r.h,
                Color::new(c.r * 0.28, c.g * 0.28, c.b * 0.28, 1.0),
            );
            v.rect(r.x, r.y, r.w, r.h - 2.0, c);
            v.rect(
                r.x + 2.0,
                r.y + 2.0,
                r.w - 4.0,
                2.0,
                Color::new(1.0, 1.0, 1.0, 0.24),
            );
            if hp == 2 {
                v.rect(r.x + r.w / 2.0 - 9.0, r.y + 10.0, 18.0, 3.0, BG);
            }
        }
        for p in &game.particles {
            if p.life <= 0.0 {
                continue;
            }
            let c = PALETTE[p.hue % 7];
            v.rect(
                p.pos.x - 2.0,
                p.pos.y - 2.0,
                3.0,
                3.0,
                Color::new(c.r, c.g, c.b, (p.life * 2.5).min(1.0)),
            );
        }
        for (i, ball) in game.balls.iter().enumerate() {
            if !ball.active {
                continue;
            }
            for n in 0..self.trail_len[i] {
                let index = (self.cursor + 12 - 1 - n) % 12;
                v.circle(
                    self.trails[i][index],
                    RADIUS * (1.0 - n as f32 / 14.0),
                    Color::new(MINT.r, MINT.g, MINT.b, 0.20 * (1.0 - n as f32 / 12.0)),
                );
            }
            let pos = ball.previous.lerp(ball.pos, alpha);
            v.circle(pos, 12.0, Color::new(MINT.r, MINT.g, MINT.b, 0.07));
            v.circle(pos, RADIUS, INK);
            v.circle(pos + V2::new(-2.0, -2.0), 2.0, WHITE);
        }
        let paddle = game.paddle_previous + (game.paddle_x - game.paddle_previous) * alpha;
        v.rect(
            paddle - game.paddle_width / 2.0,
            PADDLE_Y + 4.0,
            game.paddle_width,
            16.0,
            Color::new(MINT.r, MINT.g, MINT.b, 0.10),
        );
        v.rect(
            paddle - game.paddle_width / 2.0,
            PADDLE_Y,
            game.paddle_width,
            14.0,
            MINT,
        );
        v.rect(
            paddle - game.paddle_width / 2.0 + 8.0,
            PADDLE_Y + 4.0,
            game.paddle_width - 16.0,
            6.0,
            PANEL,
        );
        v.rect(paddle - 9.0, PADDLE_Y + 4.0, 18.0, 6.0, INK);
        // Text after geometry reduces switches between the font atlas and shapes.
        for drop in &game.drops {
            if drop.active {
                let c = match drop.power {
                    Power::Wide => MINT,
                    Power::Slow => ORANGE,
                    Power::Multi => PALETTE[5],
                };
                v.rect(drop.pos.x - 12.0, drop.pos.y - 10.0, 24.0, 20.0, c);
                v.text(
                    drop.power.label(),
                    drop.pos.x - 6.0,
                    drop.pos.y + 6.0,
                    17.0,
                    BG,
                );
            }
        }
        self.scratch.clear();
        let _ = write!(self.scratch, "0{} / {}", game.level + 1, LEVELS[game.level]);
        v.text(&self.scratch, 80.0, 166.0, 12.0, DIM);
        self.scratch.clear();
        let _ = write!(self.scratch, "{} REMAINING", game.remaining);
        v.text(&self.scratch, 762.0, 166.0, 12.0, DIM);
        v.text(
            "MOVE  MOUSE / A D / ARROWS     LAUNCH  SPACE",
            64.0,
            845.0,
            12.0,
            INK,
        );
        v.text(
            "P PAUSE    R RESTART    F FULLSCREEN    F3 STATS    Q QUIT",
            64.0,
            869.0,
            11.0,
            DIM,
        );
        v.text(
            if muted { "M  SOUND OFF" } else { "M  SOUND ON" },
            797.0,
            845.0,
            12.0,
            DIM,
        );
        if game.wide_time > 0.0 || game.slow_time > 0.0 {
            self.scratch.clear();
            if game.wide_time > 0.0 {
                let _ = write!(self.scratch, "WIDE {:.0}s   ", game.wide_time.ceil());
            }
            if game.slow_time > 0.0 {
                let _ = write!(self.scratch, "SLOW {:.0}s", game.slow_time.ceil());
            }
            v.centered(&self.scratch, 741.0, 13.0, MINT);
        }
        if !started {
            v.rect(192.0, 466.0, 576.0, 231.0, BG);
            v.centered("THE SIGNAL IS YOURS", 509.0, 12.0, MINT);
            v.centered("BREAK THE ARRAY", 558.0, 38.0, INK);
            v.centered(
                "Five sectors. Three lives. Keep it alive.",
                594.0,
                17.0,
                DIM,
            );
            v.centered("SPACE / ENTER / CLICK TO START", 640.0, 15.0, MINT);
            v.centered("Catch W wide / S slow / M multiball", 675.0, 13.0, DIM);
        } else if paused {
            self.overlay(&v, "SIGNAL PAUSED", "P / ESC / SPACE TO RESUME");
        } else {
            match game.phase {
                Phase::Ready => {
                    v.centered("READY WHEN YOU ARE", 607.0, 25.0, INK);
                    v.centered("SPACE / CLICK TO LAUNCH", 639.0, 14.0, MINT);
                }
                Phase::GameOver => self.overlay(&v, "SIGNAL LOST", "R / ENTER TO PLAY AGAIN"),
                Phase::Victory => self.overlay(&v, "ARRAY COMPLETE", "R / ENTER TO PLAY AGAIN"),
                Phase::Playing => {}
            }
        }
        if let Some(perf) = perf {
            v.rect(
                80.0,
                430.0,
                416.0,
                184.0,
                Color::new(BG.r, BG.g, BG.b, 0.96),
            );
            v.text("PERFORMANCE / CPU", 96.0, 457.0, 14.0, MINT);
            for (i, line) in perf.lines.iter().enumerate() {
                v.text(line, 96.0, 482.0 + i as f32 * 22.0, 12.0, INK);
            }
        }
    }
    fn overlay(&self, v: &View, title: &str, subtitle: &str) {
        v.rect(200.0, 490.0, 560.0, 154.0, BG);
        v.centered(title, 554.0, 34.0, INK);
        v.centered(subtitle, 600.0, 14.0, MINT);
    }
}
