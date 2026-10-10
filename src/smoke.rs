//! Graphical integration driver: inputs go through the real application handlers.
use crate::{
    display::MIN_PHYSICAL,
    input::{Dir, Presses, pad_controls},
    settings::Settings,
    storage::Profile,
    ui::{Controls, Screen, Ui},
};
use ark::{
    Events, Game, Input, Medals, Mode, Particle, Power, Stage,
    clock::TICK_HZ,
    field::{
        BALL_RADIUS as RADIUS, BOTTOM, COLS, Cell, CellSet, GRID_X, GRID_Y, PADDLE_Y, ROWS,
        cell_rect,
    },
    geom::V2,
    sectors::{Chapter, SectorId},
    tuning::{ADVANCE_DELAY_TICKS, MAX_BALLS, MAX_CAPSULES},
};
use macroquad::prelude::{request_new_screen_size, screen_dpi_scale, screen_height, screen_width};
/// The flow's practice sector, Afterglow: Phase meets a relay core there.
const PRACTICE: usize = 32;

pub fn flow(frame: u32, game: &mut Game, ui: &Ui, profile: &Profile) -> Controls {
    let mut keys = Controls::default();
    let none = Presses::default();
    let (south, east, west, start) = (
        Presses {
            south: true,
            ..none
        },
        Presses { east: true, ..none },
        Presses { west: true, ..none },
        Presses {
            start: true,
            ..none
        },
    );
    match frame {
        1 | 2 | 5 | 9 | 10 | 14 | 17 | 18 | 19 | 22 | 24 | 25 | 28 => keys.confirm = true,
        // Main menu is the pause sheet's last row: up wraps to it.
        7 | 12 => keys.up = true,
        15 | 16 => keys.down = true,
        6 | 11 | 23 | 30 => {
            keys.escape = true;
            keys.pause = true;
        }
        3 | 20 => {
            assert_eq!(game.stage(), Stage::Playing);
            assert!(
                game.mode()
                    == if frame == 3 {
                        Mode::Journey
                    } else {
                        Mode::Practice
                    }
            );
            game.sandbox().clear_board();
        }
        4 | 21 => {
            assert_eq!(game.stage(), Stage::Cleared);
            assert!(profile.progress.unlocked_count() >= 2);
            assert_eq!(profile.progress.checkpoint().unwrap().sector.index(), 1);
            game.sandbox().elapse(ADVANCE_DELAY_TICKS);
        }
        26 => keys.focus_lost = true,
        27 => assert!(ui.paused),
        29 => {
            assert!(!ui.paused);
            assert_eq!(game.stage(), Stage::Playing);
        }
        31 => keys.restart = true,
        32 => {
            assert!(ui.screen == Screen::Play && !ui.paused);
            assert_eq!(game.stage(), Stage::Ready);
            assert_eq!(game.sector().index(), 1);
            assert_eq!(game.score(), profile.progress.checkpoint().unwrap().score);
            for sector in [0, 1] {
                let record = profile.progress.record(SectorId::clamped(sector));
                assert_eq!(record.medals, Medals::ALL);
            }
            println!(
                "UI flow passed: new journey, clear, checkpoint, home, continue, practice, focus pause, resume, retry"
            );
        }
        33 | 39 => keys.confirm = true,
        34 => {
            let pos = V2::new(game.paddle().x + 25.0, PADDLE_Y - RADIUS - 0.5);
            let mut sandbox = game.sandbox();
            sandbox.grant(Power::Anchor);
            sandbox.grant(Power::Phase);
            sandbox.place_ball(0, pos, V2::new(0.0, 400.0));
        }
        35 => {
            assert!(game.balls()[0].held);
            assert_eq!(game.powers().anchor_charges, 2);
            let mut sandbox = game.sandbox();
            sandbox.grant(Power::Wide);
            sandbox.grant(Power::Slow);
            sandbox.grant(Power::Multi);
        }
        36 | 38 => keys.pause = true,
        37 => {
            assert!(ui.paused);
            assert!(game.balls()[0].held);
        }
        40 => {
            assert!(!game.balls()[0].held);
            assert_eq!(game.balls()[0].phase_charges, 3);
            *game = Game::start(SectorId::clamped(PRACTICE), Mode::Practice);
            game.step(Input {
                launch: true,
                ..Input::default()
            });
            let core = Cell::all().find(|&c| game.board().is_core(c)).unwrap();
            let r = cell_rect(core);
            let mut sandbox = game.sandbox();
            sandbox.grant(Power::Phase);
            sandbox.place_ball(
                0,
                V2::new(r.x - RADIUS - 1.0, r.y + r.h / 2.0),
                V2::new(500.0, 0.0),
            );
        }
        45 => {
            assert!(game.effects().relay_flash.iter().any(|&f| f > 0));
            assert_eq!(game.diagnostics().budget_exhausted, 0);
        }
        50 => println!(
            "Mechanics flow passed: Anchor hold, combined powers, pause/resume, release, Phase, relay ignition"
        ),
        // Gamepad buttons, mapped exactly as a connected controller's would be.
        51 => keys = pad_controls(start, None, false),
        52 => {
            assert!(ui.paused);
            keys = pad_controls(east, None, true);
        }
        53 => {
            assert!(!ui.paused);
            keys = pad_controls(start, None, false);
        }
        54 => {
            assert!(ui.paused);
            keys = pad_controls(none, Some(Dir::Down), true);
        }
        55 => {
            assert_eq!(ui.choice, 1);
            keys = pad_controls(south, None, true);
        }
        56 | 58 => {
            assert!(ui.screen == Screen::Play && !ui.paused);
            assert_eq!(game.stage(), Stage::Ready);
            assert!(game.sector().index() == PRACTICE && game.mode() == Mode::Practice);
            if frame == 56 {
                keys = pad_controls(start, None, false);
            } else {
                println!("Gamepad flow passed: Start pause, B resume, d-pad, A retry, X retry");
            }
        }
        57 => {
            assert!(ui.paused);
            keys = pad_controls(west, None, true);
        }
        // Serve and drain three times: the results menu opens on Retry.
        59..=67 => match (frame - 59) % 3 {
            0 => keys = pad_controls(south, None, false),
            1 => {
                assert_eq!(game.stage(), Stage::Playing);
                let below = V2::new(GRID_X, BOTTOM - RADIUS);
                game.sandbox().place_ball(0, below, V2::new(0.0, 2400.0));
            }
            _ => assert_eq!(game.lives(), 2 - ((frame - 59) / 3) as u8),
        },
        68 => {
            assert_eq!(game.stage(), Stage::GameOver);
            assert_eq!(ui.choice, 0, "the focus opens on the primary action");
            keys = pad_controls(south, None, true);
        }
        69 => {
            assert!(ui.screen == Screen::Play && !ui.paused);
            assert_eq!(game.stage(), Stage::Ready);
            assert!(game.sector().index() == PRACTICE && game.mode() == Mode::Practice);
            println!("Results flow passed: game over focuses Retry, A retries");
            keys.pause = true;
        }
        // Settings from the pause sheet: the volume changes live, and
        // leaving returns to the pause sheet on its Settings row.
        70 | 71 => {
            assert!(ui.paused);
            keys.down = true;
        }
        72 => {
            assert_eq!(ui.choice, 2);
            keys.confirm = true;
        }
        73 => {
            assert_eq!(ui.settings, Some(0));
            keys.down = true;
        }
        74 => keys.right = true,
        75 => {
            assert_eq!(ui.settings, Some(1));
            assert_eq!(profile.settings.volume, Settings::default().volume + 1);
            keys.escape = true;
        }
        76 => {
            assert!(ui.settings.is_none() && ui.paused);
            assert_eq!(ui.choice, 2);
            println!("Settings flow passed: open from pause, adjust volume live, back to pause");
        }
        _ => {}
    }
    keys
}

/// Fill every visual pool and ignite a full relay board repeatedly. This is a
/// rendering stress fixture, deliberately denser than any shipped sector.
pub fn effects(game: &mut Game, frame: u32) {
    if frame.is_multiple_of(90) {
        *game = Game::start(SectorId::clamped(10), Mode::Practice);
        game.sandbox().fill_board(1, CellSet::ALL);
        game.step(Input {
            launch: true,
            ..Input::default()
        });
        let mut sandbox = game.sandbox();
        for power in [
            Power::Multi,
            Power::Anchor,
            Power::Wide,
            Power::Slow,
            Power::Phase,
        ] {
            sandbox.grant(power);
        }
        for i in 0..MAX_BALLS {
            let pos = V2::new(200.0 + i as f32 * 240.0, 445.0);
            sandbox.place_ball(i, pos, V2::new(80.0, -900.0));
        }
    }
    let mut sandbox = game.sandbox();
    for (i, p) in sandbox.effects().particles.iter_mut().enumerate() {
        if p.life < 0.2 {
            *p = Particle {
                pos: V2::new(
                    GRID_X + (i % 60) as f32 * 12.0,
                    GRID_Y + (i % 17) as f32 * 17.0,
                ),
                velocity: V2::new(20.0, 80.0),
                life: 0.8,
                hue: i % 7,
            };
        }
    }
    for i in 0..MAX_CAPSULES {
        if !game.capsules()[i].active {
            let power = [
                Power::Wide,
                Power::Slow,
                Power::Multi,
                Power::Anchor,
                Power::Phase,
            ][i % 5];
            game.sandbox()
                .spawn_capsule(i, V2::new(125.0 + i as f32 * 62.0, 470.0), power);
        }
    }
}

/// A player partway through: Daybreak and Morning cleared, Zenith begun,
/// a mix of medals and best times, and a journey saved in sector 19. The
/// smoke run starts from an empty profile, so without this its captures
/// would never show a saved journey, earned medals or a locked selection.
pub fn showcase() -> Profile {
    use std::fmt::Write;
    let mut save = String::from("ARKONK 2\nbest 48200\nunlocked 19\n");
    for i in 0..18_u32 {
        let medals = [7, 7, 3, 7, 1, 7, 3, 7, 7, 3, 1, 3, 7, 1, 3, 7, 3, 1][i as usize];
        let _ = writeln!(save, "record {i} {medals} {}", 21840 + i * 977);
    }
    save.push_str("checkpoint 18 48200 4 0\n");
    Profile::decode(save.as_bytes()).expect("a fixed version-2 save decodes")
}

/// Smoke frames drawn with [`showcase`] progress, the screen and sector to
/// show, and where the capture goes.
pub const SHOWCASE: [(u32, Screen, usize, bool, &str); 5] = [
    (300, Screen::Title, 0, false, "target/attract-saved.png"),
    (310, Screen::Sectors, 9, false, "target/sectors-medals.png"),
    (320, Screen::Sectors, 21, false, "target/sectors-locked.png"),
    (330, Screen::Title, 0, true, "target/attract-saved-pad.png"),
    (
        340,
        Screen::Sectors,
        21,
        true,
        "target/sectors-locked-pad.png",
    ),
];

/// Sector select's pages for three chapters, and the title, drawn with
/// [`showcase`] progress in every layout class: a cleared chapter, the one
/// under way, and one not reached.
pub struct Page {
    pub class: &'static str,
    pub size: (u32, u32),
    pub screen: Screen,
    pub sector: usize,
    pub name: &'static str,
}
const fn page(class: &'static str, screen: Screen, sector: usize, name: &'static str) -> Page {
    Page {
        class,
        size: class_size(class),
        screen,
        sector,
        name,
    }
}
pub const PAGES: [Page; 12] = [
    page("regular", Screen::Sectors, 3, "sectors-daybreak"),
    page("regular", Screen::Sectors, 18, "sectors-zenith"),
    page("regular", Screen::Sectors, 60, "sectors-aurora"),
    page("regular", Screen::Title, 0, "title"),
    page("small", Screen::Sectors, 3, "sectors-daybreak"),
    page("small", Screen::Sectors, 18, "sectors-zenith"),
    page("small", Screen::Sectors, 60, "sectors-aurora"),
    page("small", Screen::Title, 0, "title"),
    page("compact", Screen::Sectors, 3, "sectors-daybreak"),
    page("compact", Screen::Sectors, 18, "sectors-zenith"),
    page("compact", Screen::Sectors, 60, "sectors-aurora"),
    page("compact", Screen::Title, 0, "title"),
];

/// The screen size each layout class is captured at.
const fn class_size(class: &str) -> (u32, u32) {
    match class.as_bytes()[0] {
        // A 960 × 900 window at 2x, as on a Mac.
        b'r' => (1920, 1800),
        b's' => (480, 450),
        _ => (240, 240),
    }
}

/// Physical sizes rendered offscreen after the main smoke run: Steam Deck,
/// 1080p, 1440p, ultrawide, 4:3, the smallest desktop window (Small), and
/// two handheld screens (Compact).
pub const LAYOUTS: [(&str, u32, u32); 8] = [
    ("deck", 1280, 800),
    ("1080p", 1920, 1080),
    ("1440p", 2560, 1440),
    ("ultrawide", 3440, 1440),
    ("4x3", 1024, 768),
    ("small", 480, 450),
    ("compact", 240, 240),
    ("compact-160", 160, 128),
];
/// Frames after the main run: one per layout and screen, then a too-small
/// window request that the minimum size must refuse.
pub const LAYOUT_FRAMES: u32 = LAYOUTS.len() as u32 * 3 + 40;

/// Shows play, title, and sector screens in turn; returns the layout and
/// capture path for this frame.
pub fn layout(frame: u32, ui: &mut Ui) -> Option<((u32, u32), String)> {
    let index = (frame / 3) as usize;
    let Some(&(name, w, h)) = LAYOUTS.get(index) else {
        let step = frame - LAYOUTS.len() as u32 * 3;
        if step == 0 {
            request_new_screen_size(10.0, 10.0);
        }
        if step == 39 {
            let dpi = screen_dpi_scale();
            let (pw, ph) = (screen_width() * dpi, screen_height() * dpi);
            println!("Minimum window: requested 10x10, got {pw}x{ph} physical");
            assert!(
                pw >= MIN_PHYSICAL.0 - 1.0 && ph >= MIN_PHYSICAL.1 - 1.0,
                "window shrank below its minimum: {pw}x{ph}"
            );
        }
        return None;
    };
    let (screen, suffix) = [
        (Screen::Play, ""),
        (Screen::Title, "-title"),
        (Screen::Sectors, "-sectors"),
    ][(frame % 3) as usize];
    ui.screen = screen;
    Some(((w, h), format!("target/layout-{name}{suffix}.png")))
}

/// One capture of the pieces after the layouts: the layout class it is
/// named for and the screen size that gets it, the look, and the staged
/// moment of play it shows.
pub struct Shot {
    pub class: &'static str,
    pub size: (u32, u32),
    pub variant: &'static str,
    pub screen: &'static str,
}
const fn shot(class: &'static str, variant: &'static str, screen: &'static str) -> Shot {
    Shot {
        class,
        size: class_size(class),
        variant,
        screen,
    }
}
/// Every kind of piece in every look: each chapter's glass, the busiest
/// sector with every capsule and both drains, Anchor holding, a relay
/// chain going off, hits, a break and a wall bounce; then a sector of each
/// chapter in play, Blue Hour's darkness, and Eclipse's gates mid-beat.
pub const SHOTS: [Shot; 34] = [
    shot("regular", "standard", "play"),
    shot("regular", "standard", "anchor"),
    shot("regular", "standard", "relay"),
    shot("regular", "standard", "hits"),
    shot("regular", "standard", "daybreak"),
    shot("regular", "standard", "morning"),
    shot("regular", "standard", "zenith"),
    shot("regular", "standard", "goldenhour"),
    shot("regular", "standard", "afterlight"),
    shot("regular", "standard", "bluehour"),
    shot("regular", "standard", "eclipse"),
    shot("regular", "standard", "aurora"),
    shot("regular", "reduced", "play"),
    shot("regular", "reduced", "relay"),
    shot("regular", "contrast", "play"),
    shot("regular", "contrast", "relay"),
    shot("regular", "contrast", "aurora"),
    shot("small", "standard", "play"),
    shot("small", "standard", "anchor"),
    shot("compact", "standard", "play"),
    shot("compact", "standard", "anchor"),
    shot("regular", "standard", "sector-08"),
    shot("regular", "standard", "sector-16"),
    shot("regular", "standard", "sector-23"),
    shot("regular", "standard", "sector-31"),
    shot("regular", "standard", "sector-38"),
    shot("regular", "standard", "sector-43"),
    shot("regular", "standard", "sector-51"),
    shot("regular", "standard", "sector-62"),
    shot("regular", "standard", "darkness"),
    shot("regular", "contrast", "darkness"),
    shot("regular", "standard", "gate-ghost"),
    shot("regular", "standard", "gate-solid"),
    shot("compact", "standard", "gate-ghost"),
];

/// Steps `game` `ticks` times without input, handing each tick to `record`.
fn run(game: &mut Game, ticks: u32, record: &mut impl FnMut(&Game, Events)) {
    for _ in 0..ticks {
        let events = game.step(Input::default());
        record(game, events);
    }
}
/// Steps until `done`, at most `ticks` times.
fn run_until(
    game: &mut Game,
    ticks: u32,
    record: &mut impl FnMut(&Game, Events),
    mut done: impl FnMut(&Game, Events) -> bool,
) {
    for _ in 0..ticks {
        let events = game.step(Input::default());
        record(game, events);
        if done(game, events) {
            return;
        }
    }
}
/// The sector with `slug`, wherever the journey puts it.
fn named(slug: &str) -> usize {
    SectorId::from_slug(slug).map_or(0, SectorId::index)
}
/// `sector` in play, its ball just served.
fn serve(sector: usize, record: &mut impl FnMut(&Game, Events)) -> Game {
    let mut game = Game::start(SectorId::clamped(sector), Mode::Practice);
    let events = game.step(Input {
        launch: true,
        ..Input::default()
    });
    record(&game, events);
    game
}
/// A live brick of `hp` with nothing under it, lowest first.
fn exposed(game: &Game, hp: u8) -> Option<Cell> {
    Cell::all().rev().find(|&c| {
        let below = Cell::new(c.index() + COLS).filter(|_| c.row() + 1 < ROWS);
        game.board().hp(c) == hp
            && !game.board().is_core(c)
            && below.is_none_or(|b| game.board().hp(b) == 0)
    })
}
/// Catches a ball on the paddle with Anchor.
fn hold(game: &mut Game, record: &mut impl FnMut(&Game, Events)) {
    let pos = V2::new(game.paddle().x + 25.0, PADDLE_Y - RADIUS - 0.5);
    let mut sandbox = game.sandbox();
    sandbox.grant(Power::Anchor);
    sandbox.place_ball(0, pos, V2::new(0.0, 400.0));
    run_until(game, 20, record, |g, _| g.balls()[0].held);
}

/// The moment of play `screen` shows, reached by stepping the simulation
/// so trails, flashes and timers are real; every tick goes to `record`.
pub fn stage(screen: &str, mut record: impl FnMut(&Game, Events)) -> Game {
    let record = &mut record;
    match screen {
        "anchor" => {
            let mut game = serve(named("satellites"), record);
            hold(&mut game, record);
            game.sandbox().grant(Power::Wide);
            run(&mut game, 30, record);
            game
        }
        "relay" => {
            let mut game = serve(named("crossfade"), record);
            let core = Cell::all().find(|&c| game.board().is_core(c));
            if let Some(core) = core {
                let r = cell_rect(core);
                let mut sandbox = game.sandbox();
                sandbox.grant(Power::Phase);
                sandbox.place_ball(
                    0,
                    V2::new(r.x - RADIUS - 1.0, r.y + r.h / 2.0),
                    V2::new(500.0, 0.0),
                );
            }
            run_until(&mut game, 120, record, |_, e| e.relay);
            run(&mut game, 12, record);
            game
        }
        "hits" => {
            let mut game = serve(named("undertow"), record);
            let below = |c: Cell| {
                let r = cell_rect(c);
                V2::new(r.x + r.w / 2.0, r.y + r.h + RADIUS + 2.0)
            };
            let up = V2::new(0.0, -500.0);
            let (armoured, plain) = (exposed(&game, 2), exposed(&game, 1));
            let mut sandbox = game.sandbox();
            if let Some(c) = armoured {
                sandbox.place_ball(0, below(c), up);
            }
            if let Some(c) = plain {
                sandbox.place_ball(1, below(c), up);
            }
            sandbox.place_ball(
                2,
                V2::new(ark::field::LEFT + RADIUS + 2.0, 600.0),
                V2::new(-400.0, -120.0),
            );
            run(&mut game, 14, record);
            game
        }
        "daybreak" | "morning" | "zenith" | "goldenhour" | "afterlight" | "bluehour"
        | "eclipse" | "aurora" => {
            let chapter = [
                "daybreak",
                "morning",
                "zenith",
                "goldenhour",
                "afterlight",
                "bluehour",
                "eclipse",
                "aurora",
            ]
            .iter()
            .position(|&c| c == screen)
            .and_then(Chapter::new)
            .unwrap_or(Chapter::Daybreak);
            // A lit sector of the chapter, so its hues read as they are.
            let lit = chapter
                .sectors()
                .find(|s| s.sector().darkness == 0)
                .unwrap_or(chapter.first_sector());
            let mut game = serve(lit.index(), record);
            let mut sandbox = game.sandbox();
            sandbox.fill_board(1, CellSet::EMPTY);
            // The bottom row shows armour and cores in the chapter's light.
            for col in 0..COLS {
                if let Some(cell) = Cell::new((ROWS - 1) * COLS + col) {
                    let (hp, core) = match col {
                        0..=2 => (2, false),
                        3..=5 => (3, false),
                        6..=8 => (1, true),
                        _ => (1, false),
                    };
                    sandbox.set_brick(cell, hp, core);
                }
            }
            run(&mut game, 10, record);
            game
        }
        // A sector as it is first played, its ball on the way up.
        sector if sector.starts_with("sector-") => {
            let number: usize = sector["sector-".len()..].parse().unwrap_or(1);
            let mut game = serve(number.saturating_sub(1), record);
            run(&mut game, TICK_HZ / 2, record);
            game
        }
        // Blue Hour at its darkest: one ball among the stars, the keel
        // lighting the column above the paddle.
        "darkness" => {
            let mut game = serve(named("constellation"), record);
            game.sandbox()
                .place_ball(0, V2::new(330.0, 420.0), V2::new(-260.0, -380.0));
            run(&mut game, 6, record);
            game
        }
        // Eclipse's set piece mid-beat: the gates halfway through their
        // ghost half or their solid half.
        "gate-ghost" | "gate-solid" => {
            let mut game = serve(named("totality"), record);
            let beat = game
                .sector()
                .sector()
                .beat
                .unwrap_or(ark::sectors::Beat { solid: 1, ghost: 1 });
            let at = if screen == "gate-ghost" {
                beat.solid + beat.ghost / 2
            } else {
                beat.solid / 2
            };
            game.sandbox()
                .place_ball(0, V2::new(250.0, 600.0), V2::new(-260.0, -380.0));
            game.sandbox().elapse(at.saturating_sub(4));
            run(&mut game, 4, record);
            game
        }
        // The busiest authored sector with three balls, one phased, every
        // capsule falling, both drains with Slow running out, and Anchor
        // charges in the seam.
        _ => {
            let mut game = serve(named("parallax"), record);
            hold(&mut game, record);
            game.sandbox().grant(Power::Slow);
            // Slow runs on while the ball is held, into its last 2 s.
            run(&mut game, TICK_HZ * 21 / 2, record);
            let mut sandbox = game.sandbox();
            sandbox.grant(Power::Wide);
            sandbox.place_ball(0, V2::new(380.0, 560.0), V2::new(260.0, -380.0));
            sandbox.place_ball(1, V2::new(600.0, 620.0), V2::new(-300.0, -330.0));
            sandbox.grant(Power::Phase);
            sandbox.place_ball(2, V2::new(250.0, 660.0), V2::new(-200.0, 420.0));
            for (i, power) in Power::ALL.into_iter().enumerate() {
                let pos = V2::new(170.0 + i as f32 * 150.0, 470.0 + (i % 2) as f32 * 70.0);
                sandbox.spawn_capsule(i, pos, power);
            }
            run(&mut game, 20, record);
            game
        }
    }
}
