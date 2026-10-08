use super::*;
use crate::{settings::Settings, ui::Preview};
use ark::{Input, progress::Checkpoint};

#[test]
fn empty_or_invalid_windows_have_no_view() {
    for (w, h, dpi) in [
        (0.0, 0.0, 2.0),
        (1280.0, 0.0, 1.0),
        (f32::NAN, 800.0, 1.0),
        (1280.0, 800.0, 0.0),
        (f32::INFINITY, f32::INFINITY, 1.0),
    ] {
        assert_eq!(View::fit(w, h, dpi), None, "{w}x{h} @{dpi}");
    }
}
#[test]
fn scene_fits_every_target_display_and_maps_the_pointer_back() {
    // Steam Deck, 1080p, 1440p, ultrawide, 4:3, and the minimum window.
    for (w, h) in [
        (1280.0, 800.0),
        (1920.0, 1080.0),
        (2560.0, 1440.0),
        (3440.0, 1440.0),
        (1024.0, 768.0),
        (480.0, 450.0),
    ] {
        for dpi in [1.0, 2.0] {
            let (w, h) = (w / dpi, h / dpi);
            let v = View::fit(w, h, dpi).unwrap();
            let (right, bottom) = (v.x + WIDTH * v.scale, v.y + HEIGHT * v.scale);
            let pixel = 1.0 / dpi;
            assert!(v.x >= 0.0 && v.y >= 0.0, "{w}x{h}: scene clipped");
            assert!(right <= w + pixel && bottom <= h + pixel, "{w}x{h}");
            // One axis fills the window; letterbox bars are even.
            assert!(v.x.min(v.y) <= pixel / 2.0);
            assert!((v.x - (w - right)).abs() <= pixel && (v.y - (h - bottom)).abs() <= pixel);
            let top_left = v.to_scene(v.x, v.y);
            let far = v.to_scene(right, bottom);
            assert!(top_left.x.abs() < 1e-3 && top_left.y.abs() < 1e-3);
            assert!((far.x - WIDTH).abs() < 1e-2 && (far.y - HEIGHT).abs() < 1e-2);
        }
    }
}

/// The checked displays, plus, for every role and baked size, the density
/// just past where that size takes over from the ladder entry below it:
/// there its text is widest for its layout, so fitting there means fitting
/// at every density in between.
fn densities() -> Vec<f32> {
    let mut out: Vec<f32> = spec::DENSITIES.to_vec();
    for role in spec::ROLES {
        let (size, _, _) = spec::style(role);
        for rung in spec::rungs(role) {
            if let Some(&below) = spec::LADDER.iter().rev().find(|&&l| l < rung) {
                let switch = (f32::from(below) * f32::from(rung)).sqrt() / size;
                out.push(switch * 1.0005);
            }
        }
        // The smallest density that still uses Noto.
        let smallest = spec::rungs(role).min().unwrap();
        out.push(f32::from(smallest) * 0.87 / size * 1.0005);
    }
    out.sort_by(f32::total_cmp);
    out.dedup();
    out
}

/// A profile far along: everything open, medals, best times, a huge best
/// score, and a checkpoint in `sector`.
fn veteran(sector: usize) -> Profile {
    let mut file = String::from("ARKONK 1\nbest 4294967295\nunlocked 12\n");
    for i in 0..12 {
        let _ = writeln!(file, "record {i} 7 {}", 599 * 240 + i);
    }
    let _ = writeln!(file, "checkpoint {sector} 4000000000 5 0");
    Profile::decode(file.as_bytes()).unwrap()
}

/// The widest results a sector can produce: an hour's clock, every bonus,
/// and a chain longer than any board can hold.
fn summary(life_earned: bool) -> SectorSummary {
    use ark::tuning::{CLEAR_BONUS, MEDAL_BONUS};
    SectorSummary {
        ticks: 59 * 60 * 240,
        medals: Medals::ALL,
        bonus: CLEAR_BONUS + 2 * MEDAL_BONUS,
        best_combo: 999,
        life_earned,
    }
}

/// Every screen state with distinct text, by name.
fn screens() -> Vec<(String, Game, Ui, Profile)> {
    let mut out = Vec::new();
    let title = Ui::default();
    out.push((
        "title, new player".into(),
        Game::new(),
        title.clone(),
        Profile::default(),
    ));
    for s in SectorId::all() {
        let ui = Ui {
            save_error: s.index() == 0,
            ..Ui::default()
        };
        out.push((
            format!("title, saved at {}", s.index()),
            Game::new(),
            ui,
            veteran(s.index()),
        ));
        let sectors = Ui {
            screen: Screen::Sectors,
            sector: s,
            ..Ui::default()
        };
        out.push((
            format!("sectors, {}", s.index()),
            Game::new(),
            sectors.clone(),
            veteran(0),
        ));
        // Journey readies show each tip; practice play shows each HUD.
        let play = Ui {
            screen: Screen::Play,
            ..Ui::default()
        };
        let journey = Game::resume(Checkpoint {
            sector: s,
            score: 4_000_000_000,
            lives: 5,
            ticks: 0,
        });
        out.push((
            format!("ready, {}", s.index()),
            journey,
            play.clone(),
            veteran(0),
        ));
        let mut practice = Game::start(s, Mode::Practice);
        practice.step(Input {
            launch: true,
            ..Input::default()
        });
        out.push((format!("play, {}", s.index()), practice, play, veteran(0)));
    }
    let locked = Ui {
        screen: Screen::Sectors,
        sector: SectorId::clamped(5),
        save_error: true,
        ..Ui::default()
    };
    out.push((
        "sectors, locked".into(),
        Game::new(),
        locked,
        Profile::default(),
    ));

    let play = Ui {
        screen: Screen::Play,
        ..Ui::default()
    };
    let mut held = Game::start(SectorId::FIRST, Mode::Journey);
    held.step(Input {
        launch: true,
        ..Input::default()
    });
    let pos = V2::new(held.paddle().x + 25.0, PADDLE_Y - RADIUS - 0.5);
    held.sandbox().grant(Power::Anchor);
    held.sandbox().place_ball(0, pos, V2::new(0.0, 400.0));
    held.step(Input::default());
    assert!(held.balls()[0].held, "the fixture must hold a ball");
    for power in Power::ALL {
        let mut game = held.clone();
        let mut sandbox = game.sandbox();
        sandbox.effects().notice = Some(power);
        sandbox.effects().notice_ticks = 60;
        sandbox.spawn_capsule(0, V2::new(400.0, 400.0), power);
        out.push((
            format!("held, {power:?} notice"),
            game,
            play.clone(),
            veteran(0),
        ));
    }
    for (row, paused, screen) in [(0, false, Screen::Title), (3, true, Screen::Play)] {
        let ui = Ui {
            screen,
            paused,
            settings: Some(row),
            ..Ui::default()
        };
        let mut profile = veteran(3);
        profile.settings.fullscreen = paused;
        out.push((format!("settings {row}"), held.clone(), ui, profile));
    }
    let profiles = [Profile::default(), veteran(3)];
    for (i, profile) in profiles.iter().enumerate() {
        let mut muted = profile.clone();
        muted.settings = Settings {
            muted: i == 1,
            volume: 10,
            ..muted.settings
        };
        let paused = Ui {
            screen: Screen::Play,
            paused: true,
            save_error: i == 1,
            ..Ui::default()
        };
        out.push((format!("paused {i}"), held.clone(), paused, muted.clone()));
        for (stage, life) in [
            (Stage::Cleared, true),
            (Stage::Cleared, false),
            (Stage::GameOver, false),
            (Stage::Victory, false),
        ] {
            let ui = Ui {
                screen: Screen::Play,
                preview: Some(Preview {
                    stage,
                    summary: summary(life),
                }),
                save_error: i == 1,
                ..Ui::default()
            };
            let game = if i == 0 {
                Game::start(SectorId::FIRST, Mode::Practice)
            } else {
                held.clone()
            };
            out.push((format!("{stage:?} {i}"), game, ui, muted.clone()));
        }
    }
    out
}

/// Draws every screen, in every locale this build can draw, with both
/// prompt styles, at every density that matters, and fails on any text
/// wider than its slot, any wrapped text past its lines, and any glyph
/// missing from the atlas.
#[test]
fn every_string_fits_its_place_in_every_locale() {
    let screens = screens();
    let densities = densities();
    // One thread per locale: each builds its own atlas and checks alone.
    let failures: Vec<String> = std::thread::scope(|threads| {
        let checks: Vec<_> = Locale::ALL
            .into_iter()
            .filter(|&l| ark_glyphs::supports(l))
            .map(|locale| {
                let (screens, densities) = (&screens, &densities);
                threads.spawn(move || check(locale, screens, densities))
            })
            .collect();
        checks
            .into_iter()
            .flat_map(|c| c.join().expect("a layout check panicked"))
            .collect()
    });
    assert!(
        failures.is_empty(),
        "{} layout failures:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

/// Draws every screen in `locale` with both prompt styles at every density
/// that matters, and returns any text wider than its slot, any wrapped text
/// past its lines, and any glyph missing from the atlas.
fn check(
    locale: Locale,
    screens: &[(String, Game, Ui, Profile)],
    densities: &[f32],
) -> Vec<String> {
    let fonts = ark_glyphs::fonts(locale).unwrap();
    let atlas = Atlas::build(&fonts).unwrap();
    let mut failures = Vec::new();
    for &density in densities {
        for device in [Device::KeyboardMouse, Device::Gamepad] {
            for (name, game, ui, profile) in screens {
                let log = RefCell::new(Vec::new());
                let v = Scene {
                    texture: None,
                    atlas: &atlas,
                    fonts: &fonts,
                    locale,
                    density,
                    device,
                    buffer: RefCell::new(String::new()),
                    misfits: Some(&log),
                    hits: RefCell::default(),
                    motion: Shared::new((1.0, 0.0)),
                };
                let ui = Ui {
                    device,
                    ..ui.clone()
                };
                // The Settings sheet names the language in use in itself.
                let mut profile = profile.clone();
                if ui.settings.is_some() && locale != Locale::Pseudo {
                    profile.settings.locale = Some(locale);
                }
                let profile = &profile;
                scene(&v, &Fx::default(), game, &ui, profile, 1.0, None);
                for m in log.into_inner() {
                    let line = match m.missing {
                        Some(c) => format!("{locale:?} {name}: no glyph for {c:?} in {:?}", m.text),
                        None => format!(
                            "{locale:?} {name} {device:?} @{density:.3}: {:?} needs {:.1}, has {:.1}",
                            m.text, m.need, m.room
                        ),
                    };
                    if !failures.contains(&line) {
                        failures.push(line);
                    }
                }
            }
        }
    }
    failures
}

/// Valve asks for text at least 9 px tall at 1280 × 800. The smallest
/// text is a label, set in capitals, so its cap height is what counts.
#[test]
fn the_smallest_text_is_nine_pixels_tall_on_steam_deck() {
    let fonts = ark_glyphs::fonts(Locale::En).unwrap();
    let deck = 800.0 / 900.0;
    for role in spec::ROLES {
        let ppem = spec::ppem(role, deck).unwrap();
        let (_, weight, _) = spec::style(role);
        let face = fonts.latin.face(weight).unwrap();
        let cap = f32::from(face.cap_height) * f32::from(ppem) / f32::from(face.units_per_em);
        assert!(cap >= 9.0, "{role:?} at {ppem} px: capitals {cap:.1} px");
    }
}

#[test]
fn figures_group_on_the_stack() {
    let f = Figures::count(Locale::Fr, u32::MAX);
    assert_eq!(f.as_str(), "4\u{202F}294\u{202F}967\u{202F}295");
    let t = Figures::of(|f| write!(f, "{}", Clock(83 * TICK_HZ)));
    assert_eq!(t.as_str(), "01:23");
}
