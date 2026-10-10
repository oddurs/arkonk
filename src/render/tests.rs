use super::*;
use crate::{input::Pad, settings::Settings, ui::Preview};
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

/// The checked densities, plus, for every role and baked size, the
/// density just past where that size takes over from the ladder entry
/// below it: there its text is widest for its layout, so fitting there
/// means fitting at every density in between. Only Regular and Small
/// frames: a Compact one is laid out from its screen, in [`views`].
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
    }
    out.retain(|&d| Class::of(WIDTH * d) != Class::Compact);
    out.sort_by(f32::total_cmp);
    out.dedup();
    out
}

/// A screen the layout is checked on: its name, its size in pixels, and
/// the view of the scene it gets.
struct Screenful {
    name: String,
    size: (f32, f32),
    view: View,
}
fn screenful(name: String, (w, h): (f32, f32)) -> Screenful {
    let view = View::fit(w, h, 1.0).expect("a test screen has an area");
    Screenful {
        name,
        size: (w, h),
        view,
    }
}

/// The atlas a frame in `fonts` gets at `view`'s density.
fn atlas_for(fonts: &Fonts, view: &View) -> Atlas {
    Atlas::build(fonts, strikes_at(view.density)).unwrap()
}

/// Every screen size the layout is checked at: the frame widths where
/// the classes meet, the presets from 4K down to a 160 × 128 handheld,
/// and every density where a baked size changes.
fn views() -> Vec<Screenful> {
    let mut out = Vec::new();
    for (f, class) in [
        (399.0, Class::Compact),
        (400.0, Class::Small),
        (719.0, Class::Small),
        (720.0, Class::Regular),
    ] {
        // As tall as wide, so the frame's width is exactly `f`.
        let s = screenful(format!("F = {f}"), (f, f));
        assert_eq!(s.view.class, class, "{}", s.name);
        out.push(s);
    }
    for (w, h) in [
        (1920.0, 1080.0),
        (1280.0, 800.0),
        (3440.0, 1440.0),
        (3840.0, 2160.0),
        (1024.0, 768.0),
        (1080.0, 1920.0),
        (240.0, 240.0),
        (160.0, 128.0),
    ] {
        out.push(screenful(format!("{w}x{h}"), (w, h)));
    }
    for d in densities() {
        out.push(screenful(format!("@{d:.3}"), (WIDTH * d, HEIGHT * d)));
    }
    out
}

#[test]
fn every_class_is_reached_by_the_frame_width() {
    assert_eq!(
        View::fit(1920.0, 1080.0, 1.0).unwrap().class,
        Class::Regular
    );
    // The smallest desktop window is Small, at 1x and 2x alike.
    assert_eq!(View::fit(480.0, 450.0, 1.0).unwrap().class, Class::Small);
    assert_eq!(View::fit(240.0, 225.0, 2.0).unwrap().class, Class::Small);
    for (w, h) in [(160.0, 128.0), (240.0, 240.0)] {
        let v = View::fit(w, h, 1.0).unwrap();
        assert_eq!(v.class, Class::Compact);
        // The field fills the screen but for the rails and the strip.
        let (corner, far) = (v.to_scene(0.0, 0.0), v.to_scene(w, h));
        assert!(corner.x <= LEFT && far.x >= RIGHT, "{w}x{h}");
        assert!(far.y >= BOTTOM - 0.01 && corner.y <= TOP - view::STRIP / v.density);
    }
}

#[test]
fn overscan_keeps_the_band_and_sheets_in_the_safe_area() {
    let fonts = ark_glyphs::fonts(Locale::En).unwrap();
    let plain = View::fit(1920.0, 1080.0, 1.0).unwrap();
    // Desktop reports no overscan: the safe area is the whole screen.
    assert!(plain.safe.x <= 0.0 && plain.safe.y <= 0.0);
    let tv = View::fit_inset(1920.0, 1080.0, 1.0, 0.05).unwrap();
    assert!(tv.safe.y > plain.safe.y && tv.safe.x > plain.safe.x);
    let atlas = atlas_for(&fonts, &tv);
    let ui = Ui {
        screen: Screen::Play,
        paused: true,
        ..Ui::default()
    };
    let log = RefCell::new(Log::default());
    let v = Scene::new(
        None,
        (&atlas, &fonts, Locale::En),
        &tv,
        &ui,
        String::new(),
        Some(&log),
    );
    scene(
        &v,
        &Fx::default(),
        &Game::new(),
        &ui,
        &Profile::default(),
        1.0,
        None,
    );
    let safe = tv.safe;
    for p in &log.borrow().placed {
        let r = p.rect;
        let inside = r.x >= safe.x
            && r.y >= safe.y - 0.5
            && r.right() <= safe.right()
            && r.bottom() <= safe.bottom();
        assert!(inside, "{:?} at {r:?} outside {safe:?}", p.text);
    }
}
/// A profile far along: everything open, medals, best times, a huge best
/// score, and a checkpoint in `sector`.
fn veteran(sector: usize) -> Profile {
    let mut file = String::from("ARKONK 2\nbest 4294967295\nunlocked 64\n");
    for i in 0..SECTOR_COUNT {
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
            notice: if s.index() == 0 { 3.0 } else { 0.0 },
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
        let news = Ui {
            notice: 3.0,
            ..play.clone()
        };
        out.push((
            format!("play, {}", s.index()),
            practice.clone(),
            play,
            veteran(0),
        ));
        // The news line is the same in every sector: one per chapter.
        if s.ends_chapter() {
            out.push((
                format!("play, {}, life gained", s.index()),
                practice,
                news,
                veteran(0),
            ));
        }
    }
    let locked = Ui {
        screen: Screen::Sectors,
        sector: SectorId::clamped(5),
        save_error: true,
        notice: 3.0,
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
    // The last row: a Compact page too short for every row scrolls to it,
    // and the reduced and high looks name their values.
    let last = crate::settings::ROWS.len() - 1;
    for (row, paused, screen) in [
        (0, false, Screen::Title),
        (3, true, Screen::Play),
        (last, true, Screen::Play),
    ] {
        let ui = Ui {
            screen,
            paused,
            settings: Some(row),
            ..Ui::default()
        };
        let mut profile = veteran(3);
        profile.settings.fullscreen = paused;
        profile.settings.reduced_effects = row == last;
        profile.settings.high_contrast = row == last;
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

/// Draws every screen, in every locale this build can draw and the
/// pseudo locale, with the keyboard, Xbox and PlayStation glyphs, at
/// every screen in [`views`], and fails on text wider than its place,
/// wrapped past its lines, cut to an ellipsis or without a glyph; on
/// text that overlaps other text or leaves its box or the screen; and
/// on text, glyphs or pointer rows under their physical floors.
#[test]
fn every_screen_lays_out_in_every_locale_and_class() {
    let screens = screens();
    let views = views();
    // One thread per locale: each builds its own atlas and checks alone.
    let failures: Vec<String> = std::thread::scope(|threads| {
        let checks: Vec<_> = Locale::ALL
            .into_iter()
            .filter(|&l| ark_glyphs::supports(l))
            .map(|locale| {
                let (screens, views) = (&screens, &views);
                threads.spawn(move || check(locale, screens, views))
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

/// Whether `inner` lies inside `outer`, give or take `slack`.
fn contains(outer: Rect, inner: Rect, slack: f32) -> bool {
    inner.x >= outer.x - slack
        && inner.y >= outer.y - slack
        && inner.right() <= outer.right() + slack
        && inner.bottom() <= outer.bottom() + slack
}
/// Whether `a` and `b` share more than `slack` both ways.
fn overlap(a: Rect, b: Rect, slack: f32) -> bool {
    a.x + slack < b.right()
        && b.x + slack < a.right()
        && a.y + slack < b.bottom()
        && b.y + slack < a.bottom()
}

/// Draws every screen in `locale` with each glyph family on each of
/// `views`, and returns every layout failure.
fn check(
    locale: Locale,
    screens: &[(String, Game, Ui, Profile)],
    views: &[Screenful],
) -> Vec<String> {
    let fonts = ark_glyphs::fonts(locale).unwrap();
    let mut failures = Vec::new();
    // Packed again only where a view's density needs other strikes, as
    // the renderer does.
    let mut atlas = atlas_for(&fonts, &views[0].view);
    for at in views {
        let view = &at.view;
        if atlas.strikes() != strikes_at(view.density) {
            atlas = atlas_for(&fonts, view);
        }
        // One physical pixel, in scene units: what a layout may be off by.
        let px = 1.0 / view.density;
        let screen = Rect::new(
            -view.x / view.scale,
            -view.y / view.scale,
            at.size.0 / view.scale,
            at.size.1 / view.scale,
        );
        for device in [
            Device::KeyboardMouse,
            Device::Gamepad(Pad::Xbox),
            Device::Gamepad(Pad::PlayStation),
        ] {
            for (name, game, ui, profile) in screens {
                let ui = Ui {
                    device,
                    ..ui.clone()
                };
                let log = RefCell::new(Log::default());
                let v = Scene::new(
                    None,
                    (&atlas, &fonts, locale),
                    view,
                    &ui,
                    String::new(),
                    Some(&log),
                );
                // The Settings sheet names the language in use in itself.
                let mut profile = profile.clone();
                if ui.settings.is_some() && locale != Locale::Pseudo {
                    profile.settings.locale = Some(locale);
                }
                let fx = Fx {
                    life_gained: if name.ends_with("life gained") {
                        1.0
                    } else {
                        0.0
                    },
                    ..Fx::default()
                };
                scene(&v, &fx, game, &ui, &profile, 1.0, None);
                let hits = v.hits.into_inner();
                let mut found = Vec::new();
                let log = log.into_inner();
                found.extend(log.problems);
                for (i, p) in log.placed.iter().enumerate() {
                    if !contains(p.within, p.rect, px) {
                        found.push(format!(
                            "{:?} at {:?} leaves {:?}",
                            p.text, p.rect, p.within
                        ));
                    } else if !contains(screen, p.rect, px) {
                        found.push(format!("{:?} at {:?} is off the screen", p.text, p.rect));
                    }
                    for q in &log.placed[..i] {
                        if p.layer == q.layer && overlap(p.rect, q.rect, px) {
                            found.push(format!("{:?} overlaps {:?}", p.text, q.text));
                        }
                    }
                }
                let rows = hits.rows();
                for (i, r) in rows.iter().enumerate() {
                    // A Compact list is one row per line; its pointer is
                    // a preview's, not a player's.
                    if r.h > 0.0 && view.class != Class::Compact && r.h * view.density < 32.0 - 0.5
                    {
                        found.push(format!("row {i} is {:.1} px tall", r.h * view.density));
                    }
                    if rows[..i].iter().any(|q| overlap(*q, *r, px)) {
                        found.push(format!("row {i} overlaps another"));
                    }
                }
                for f in found {
                    let line = format!("{locale:?} {name} {device:?} {}: {f}", at.name);
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
        let ppem = spec::ppem(role, deck);
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

/// Each row is hit once, where it was drawn: a click on the second row
/// must choose the second action.
#[test]
fn every_drawn_row_is_one_hit_area() {
    let fonts = ark_glyphs::fonts(Locale::En).unwrap();
    for (ui, menu) in [
        (
            Ui {
                screen: Screen::Play,
                paused: true,
                ..Ui::default()
            },
            ui::pause_menu(),
        ),
        (Ui::default(), ui::title_menu(false)),
    ] {
        let view = View::fit(WIDTH, HEIGHT, 1.0).unwrap();
        let atlas = atlas_for(&fonts, &view);
        let v = Scene::new(
            None,
            (&atlas, &fonts, Locale::En),
            &view,
            &ui,
            String::new(),
            None,
        );
        scene(
            &v,
            &Fx::default(),
            &Game::new(),
            &ui,
            &Profile::default(),
            1.0,
            None,
        );
        let hits = v.hits.into_inner();
        assert_eq!(hits.rows().len(), menu.actions.len());
        for (i, r) in hits.rows().iter().enumerate() {
            assert_eq!(hits.row_at(menu, r.center()), Some(i));
        }
    }
}

/// A Small screen's pager glyphs keep their pixel floor, which grows them
/// past the design's size on the narrowest frames: the row must still sit
/// inside the field and a gap above the first card.
#[test]
fn the_small_pager_stays_in_the_field_above_the_cards() {
    let fonts = ark_glyphs::fonts(Locale::En).unwrap();
    let ui = Ui {
        screen: Screen::Sectors,
        ..Ui::default()
    };
    let small: Vec<_> = views()
        .into_iter()
        .filter(|s| s.view.class == Class::Small)
        .collect();
    assert!(!small.is_empty());
    for at in &small {
        let atlas = atlas_for(&fonts, &at.view);
        let v = Scene::new(
            None,
            (&atlas, &fonts, Locale::En),
            &at.view,
            &ui,
            String::new(),
            None,
        );
        let (mid, chip) = screens::pager(&v);
        let first = screens::card_rect(&v, SectorId::FIRST);
        assert!(
            mid - chip / 2.0 >= TOP,
            "{}: pager above the field",
            at.name
        );
        assert!(
            mid + chip / 2.0 + S8 <= first.y + 1e-3,
            "{}: pager crowds the first card",
            at.name
        );
    }
}

/// A locked card's padlock keeps a group's gap from every word around it,
/// in the longest strings and on every Regular and Small screen.
#[test]
fn padlocks_keep_clear_of_text() {
    for locale in [Locale::En, Locale::Pseudo] {
        let fonts = ark_glyphs::fonts(locale).unwrap();
        for sector in [4, 7] {
            let ui = Ui {
                screen: Screen::Sectors,
                sector: SectorId::clamped(sector),
                ..Ui::default()
            };
            for at in views().iter().filter(|s| s.view.class != Class::Compact) {
                let atlas = atlas_for(&fonts, &at.view);
                let log = RefCell::new(Log::default());
                let v = Scene::new(
                    None,
                    (&atlas, &fonts, locale),
                    &at.view,
                    &ui,
                    String::new(),
                    Some(&log),
                );
                let game = Game::new();
                scene(
                    &v,
                    &Fx::default(),
                    &game,
                    &ui,
                    &Profile::default(),
                    1.0,
                    None,
                );
                let placed = log.into_inner().placed;
                let locks: Vec<_> = placed.iter().filter(|p| p.text == PADLOCK).collect();
                assert!(!locks.is_empty(), "{}: no locked card drawn", at.name);
                for lock in locks {
                    let near = Rect::new(
                        lock.rect.x - S8,
                        lock.rect.y - S8,
                        lock.rect.w + 2.0 * S8,
                        lock.rect.h + 2.0 * S8,
                    );
                    for p in placed.iter().filter(|p| p.text != PADLOCK) {
                        assert!(
                            p.layer != lock.layer || !overlap(near, p.rect, 0.0),
                            "{locale:?} {}: {:?} crowds a padlock",
                            at.name,
                            p.text
                        );
                    }
                }
            }
        }
    }
}

/// In Arabic a sheet mirrors: its title and row labels end at the right,
/// the back glyph sits at the title's left, and the focused row's confirm
/// glyph at the row's left. English keeps them the other way round.
#[test]
fn arabic_sheets_mirror() {
    let ui = Ui {
        screen: Screen::Play,
        paused: true,
        device: Device::Gamepad(Pad::Xbox),
        ..Ui::default()
    };
    let view = View::fit(1920.0, 1080.0, 1.0).unwrap();
    for locale in [Locale::En, Locale::Ar] {
        if !ark_glyphs::supports(locale) {
            continue;
        }
        let rtl = locale == Locale::Ar;
        let fonts = ark_glyphs::fonts(locale).unwrap();
        let atlas = atlas_for(&fonts, &view);
        let log = RefCell::new(Log::default());
        let v = Scene::new(
            None,
            (&atlas, &fonts, locale),
            &view,
            &ui,
            String::new(),
            Some(&log),
        );
        scene(
            &v,
            &Fx::default(),
            &Game::new(),
            &ui,
            &Profile::default(),
            1.0,
            None,
        );
        let hits = v.hits.into_inner();
        let back = hits.back.expect("the pause sheet has a back glyph");
        let row = hits.rows()[0];
        let placed = log.into_inner().placed;
        let text = |id: TextId| {
            let mut s = String::new();
            ark_text::write(&mut s, locale, Form::Full, id, &[]).unwrap();
            let p = placed.iter().find(|p| p.text == s);
            p.unwrap_or_else(|| panic!("{locale:?}: {s:?} not drawn"))
                .rect
        };
        let title = text(TextId::Paused);
        let label = text(TextId::ActionResume);
        // The Xbox confirm glyph's letter, inside the first row.
        let glyph = placed
            .iter()
            .find(|p| p.text == "A" && contains(row, p.rect, 0.0))
            .expect("the focused row carries its confirm glyph")
            .rect;
        if rtl {
            assert!(back.right() < title.x, "{locale:?}: back glyph not left");
            assert!(
                glyph.right() < label.x,
                "{locale:?}: confirm glyph not left"
            );
            assert!(
                label.x > row.center().x,
                "{locale:?}: label not at the right"
            );
        } else {
            assert!(title.right() < back.x, "{locale:?}: back glyph not right");
            assert!(
                label.right() < glyph.x,
                "{locale:?}: confirm glyph not right"
            );
            assert!(
                label.right() < row.center().x,
                "{locale:?}: label not at the left"
            );
        }
        settings_rows_mirror(locale, &fonts);
    }
}

/// The Effects and Contrast rows of the Settings sheet, in a regular window
/// and on a Compact page: the name leads the row and the value ends it, so
/// in Arabic the name is at the right and the value at the left.
fn settings_rows_mirror(locale: Locale, fonts: &Fonts) {
    let rtl = locale == Locale::Ar;
    let last = crate::settings::ROWS.len() - 1;
    let ui = Ui {
        screen: Screen::Play,
        paused: true,
        settings: Some(last),
        ..Ui::default()
    };
    let mut profile = Profile::default();
    profile.settings.reduced_effects = true;
    profile.settings.high_contrast = true;
    let rows = [
        (TextId::SettingEffects, TextId::EffectsReduced),
        (TextId::SettingContrast, TextId::ContrastHigh),
    ];
    for (w, h) in [(1920.0, 1080.0), (240.0, 240.0)] {
        let view = View::fit(w, h, 1.0).unwrap();
        let atlas = &atlas_for(fonts, &view);
        let log = RefCell::new(Log::default());
        let v = Scene::new(
            None,
            (atlas, fonts, locale),
            &view,
            &ui,
            String::new(),
            Some(&log),
        );
        scene(&v, &Fx::default(), &Game::new(), &ui, &profile, 1.0, None);
        let hits = v.hits.into_inner();
        let placed = log.into_inner().placed;
        // The sheet sets each string full or short, whichever fits.
        let find = |id: TextId, row: Rect| {
            [Form::Full, Form::Short]
                .into_iter()
                .find_map(|form| {
                    let mut s = String::new();
                    ark_text::write(&mut s, locale, form, id, &[]).unwrap();
                    placed
                        .iter()
                        .find(|p| p.text == s && contains(row, p.rect, 0.5))
                })
                .unwrap_or_else(|| panic!("{locale:?} {w}: {id:?} not drawn in its row"))
                .rect
        };
        let mut checked = 0;
        for (i, (name, value)) in rows.into_iter().enumerate() {
            let row = hits.rows()[ROWS_BEFORE + i];
            // A Compact page too short for every row leaves earlier ones off.
            if row.w == 0.0 {
                continue;
            }
            let (name, value) = (find(name, row), find(value, row));
            if rtl {
                assert!(value.right() < name.x, "{locale:?} {w}: value not left");
                assert!(name.x > row.center().x, "{locale:?} {w}: name not right");
            } else {
                assert!(name.right() < value.x, "{locale:?} {w}: value not right");
                assert!(
                    name.right() < row.center().x,
                    "{locale:?} {w}: name not left"
                );
            }
            checked += 1;
        }
        assert!(checked > 0, "{locale:?} {w}: no look row drawn");
    }
}

/// The settings rows ahead of Effects and Contrast.
const ROWS_BEFORE: usize = 4;

/// A sector at its busiest: every brick live, three balls in flight, one
/// phased and one held, all twelve capsules falling, Wide, Slow and
/// Anchor running, and every particle slot alive.
fn busiest(sector: usize, fill: Option<u8>) -> Game {
    use ark::{Particle, field::CellSet, tuning::MAX_CAPSULES};
    let mut game = Game::start(SectorId::clamped(sector), Mode::Practice);
    game.step(Input {
        launch: true,
        ..Input::default()
    });
    let mut sandbox = game.sandbox();
    if let Some(hp) = fill {
        sandbox.fill_board(hp, CellSet::ALL);
    }
    for power in [Power::Wide, Power::Slow, Power::Anchor, Power::Phase] {
        sandbox.grant(power);
    }
    for i in 0..MAX_BALLS {
        let pos = V2::new(200.0 + i as f32 * 240.0, 600.0);
        sandbox.place_ball(i, pos, V2::new(80.0, -500.0));
    }
    for i in 0..MAX_CAPSULES {
        let pos = V2::new(125.0 + i as f32 * 62.0, 470.0);
        sandbox.spawn_capsule(i, pos, Power::ALL[i % Power::ALL.len()]);
    }
    for (i, p) in sandbox.effects().particles.iter_mut().enumerate() {
        *p = Particle {
            pos: V2::new(
                100.0 + (i % 60) as f32 * 12.0,
                200.0 + (i % 17) as f32 * 17.0,
            ),
            velocity: V2::new(20.0, 80.0),
            life: 0.5,
            hue: i % 7,
        };
    }
    sandbox.effects().relay_flash.fill(20);
    game
}

/// What one frame of `game` in play sends to the GPU at 1920 × 1800, with
/// every trail full and every brick just hit.
fn tally(game: &Game, profile: &Profile) -> Tally {
    let fonts = ark_glyphs::fonts(Locale::En).unwrap();
    let view = View::fit(1920.0, 1800.0, 1.0).unwrap();
    let atlas = atlas_for(&fonts, &view);
    let ui = Ui {
        screen: Screen::Play,
        ..Ui::default()
    };
    let v = Scene::new(
        None,
        (&atlas, &fonts, Locale::En),
        &view,
        &ui,
        String::new(),
        None,
    );
    let mut fx = Fx {
        trail_len: [12; MAX_BALLS],
        brick_age: [0.03; CELLS],
        brick_was: [3; CELLS],
        ..Fx::default()
    };
    for (i, ball) in game.balls().iter().enumerate() {
        fx.trails[i] = [ball.pos; 12];
    }
    scene(&v, &fx, game, &ui, profile, 1.0, None);
    v.tally.get()
}

/// The design's budget: at most 20,000 vertices a frame for the busiest
/// authored sector with three balls, twelve capsules and full particle
/// pools, which fits in two of Macroquad's batches.
#[test]
fn the_busiest_sector_stays_inside_the_vertex_budget() {
    let mut worst = (0, Tally::default());
    for sector in 0..SECTOR_COUNT {
        let game = busiest(sector, None);
        let mut ghosts = game.clone();
        // Ghost gates draw dashed outlines, their busiest look.
        if let Some(beat) = game.sector().sector().beat {
            ghosts.sandbox().elapse(beat.solid);
            assert!(ghosts.board().gates_are_ghosts());
        }
        for game in [game, ghosts] {
            let t = tally(&game, &Profile::default());
            if t.vertices > worst.1.vertices {
                worst = (sector, t);
            }
        }
    }
    let (sector, t) = worst;
    println!(
        "worst authored: sector {sector}, {} vertices, {} draw calls",
        t.vertices, t.calls
    );
    assert!(t.vertices <= 20_000, "sector {sector}: {t:?}");
    assert!(t.calls <= 2, "sector {sector}: {t:?}");
    // The effects test's board, denser than any sector: every cell a
    // core going off, in high contrast.
    let mut profile = Profile::default();
    profile.settings.high_contrast = true;
    let stress = tally(&busiest(10, Some(3)), &profile);
    println!(
        "stress board: {} vertices, {} draw calls",
        stress.vertices, stress.calls
    );
    assert!(stress.vertices <= 20_000, "{stress:?}");
}

/// High contrast keeps every hue it draws at 3:1 or better against the
/// field: bricks of every chapter, cores, and capsules.
#[test]
fn high_contrast_hues_read_at_three_to_one_on_the_field() {
    fn luminance(c: Color) -> f32 {
        let lin = |v: f32| {
            if v <= 0.04045 {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * lin(c.r) + 0.7152 * lin(c.g) + 0.0722 * lin(c.b)
    }
    let field = luminance(pieces::FIELD);
    for hue in PALETTE.iter().chain(&[AMBER]).copied() {
        let lifted = pieces::lifted(hue);
        let ratio = (luminance(lifted) + 0.05) / (field + 0.05);
        assert!(ratio >= 3.0, "{hue:?} at {ratio:.2}:1");
    }
    for power in Power::ALL {
        let ratio = (luminance(pieces::lifted(power_color(power))) + 0.05) / (field + 0.05);
        assert!(ratio >= 3.0, "{power:?} at {ratio:.2}:1");
    }
}

/// The simulation reports that a ball bounced off a wall, not where; the
/// renderer lights the wall beside the ball that did.
#[test]
fn a_wall_bounce_lights_the_wall_where_the_ball_struck() {
    let mut game = Game::start(SectorId::clamped(0), Mode::Practice);
    game.step(Input {
        launch: true,
        ..Input::default()
    });
    game.sandbox().place_ball(
        0,
        V2::new(LEFT + RADIUS + 1.0, 500.0),
        V2::new(-400.0, -100.0),
    );
    let mut fx = Fx::default();
    let mut lit = None;
    for _ in 0..10 {
        let events = game.step(Input::default());
        fx.record(&game, events);
        if events.wall {
            lit = fx.walls.iter().find(|f| f.age == 0.0).copied();
            break;
        }
    }
    let flash = lit.expect("the bounce lit a wall");
    assert_eq!(flash.wall, Wall::Left);
    assert!((flash.along - 500.0).abs() < 6.0, "{flash:?}");
}

/// A flat piece's one-pixel parts, such as the Compact seam's charge pips,
/// land on whole pixels at their full size wherever they start; rounding
/// each edge on its own once rounded them away.
#[test]
fn one_pixel_parts_of_flat_pieces_never_round_away() {
    let fonts = ark_glyphs::fonts(Locale::En).unwrap();
    let view = View::fit(240.0, 240.0, 1.0).unwrap();
    let atlas = atlas_for(&fonts, &view);
    assert_eq!(view.class, Class::Compact);
    let ui = Ui::default();
    let v = Scene::new(
        None,
        (&atlas, &fonts, Locale::En),
        &view,
        &ui,
        String::new(),
        None,
    );
    let px = 1.0 / view.density;
    for k in 0..40 {
        let at = 700.0 + k as f32 * px / 8.0;
        let r = pieces::place(&v, Rect::new(at, at, px, px), true);
        assert!((r.w - px).abs() < 1e-4 && (r.h - px).abs() < 1e-4, "{r:?}");
        let on_grid = |x: f32| ((x * view.density).round() - x * view.density).abs() < 1e-3;
        assert!(on_grid(r.x) && on_grid(r.y), "{r:?}");
    }
}

/// A relay chain breaks a run of neighbouring bricks a few ticks apart;
/// their points must not float up as a stack of overlapping figures, and
/// none of the points may go missing.
#[test]
fn a_relay_chains_popups_never_overlap() {
    let at = |row: usize, col: usize| FieldCell::new(row * ark::field::COLS + col).unwrap();
    let chain = [
        (1, 1, 750),
        (2, 1, 125),
        (2, 0, 825),
        (2, 2, 125),
        (3, 1, 125),
        (0, 1, 125),
        (1, 2, 125),
        (1, 0, 125),
    ];
    for gap in [0, 1, 4, 12, 40] {
        let mut fx = Fx::default();
        let mut total = 0;
        for &(row, col, points) in &chain {
            for _ in 0..gap {
                fx.age_popups();
            }
            fx.spawn_popup(at(row, col), points);
            total += points;
            let live: Vec<_> = fx.popups.iter().filter(|p| p.life > 0.0).collect();
            for (i, p) in live.iter().enumerate() {
                for q in &live[..i] {
                    assert!(!p.overlaps(q), "{gap} ticks apart: {p:?} over {q:?}");
                }
            }
        }
        if gap * chain.len() < (POPUP_LIFE / DT) as usize {
            let shown: u32 = fx
                .popups
                .iter()
                .filter(|p| p.life > 0.0)
                .map(|p| p.value)
                .sum();
            assert_eq!(shown, total, "{gap} ticks apart");
        }
    }
}
