//! Headless simulation benchmark: `cargo bench -p ark`.
//!
//! Its assertions are gates, not just measurements: zero heap allocations
//! while simulating, no exhausted collision budget, real brick impacts and
//! every sector covered. Under `cargo test` (no `--bench` argument) it runs a
//! shorter workload with the same assertions.
use ark::{game::*, physics::V2};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    hint::black_box,
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
    time::Instant,
};

struct CountingAllocator;
static COUNTING: AtomicBool = AtomicBool::new(false);
static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);
// All allocation operations delegate unchanged to the system allocator. The
// counter is enabled only around simulation, excluding setup and reporting.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if COUNTING.load(Ordering::Relaxed) {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            System.dealloc(ptr, layout);
        }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if COUNTING.load(Ordering::Relaxed) {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.realloc(ptr, layout, size) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if COUNTING.load(Ordering::Relaxed) {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.alloc_zeroed(layout) }
    }
}
#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

fn setup(stress: bool, relays: bool, level: usize) -> Game {
    let mut g = Game::at(level, Mode::Journey);
    g.step(&Input {
        launch: true,
        ..Default::default()
    });
    if relays {
        g.bricks.fill(1);
        g.cores.fill(true);
        g.remaining = ROWS * COLS;
        g.initial_bricks = g.remaining;
        for power in [
            Power::Anchor,
            Power::Phase,
            Power::Wide,
            Power::Slow,
            Power::Multi,
        ] {
            g.apply_power(power);
        }
    }
    if stress {
        for (i, b) in g.balls.iter_mut().enumerate() {
            b.active = true;
            b.pos = V2::new(240.0 + i as f32 * 160.0, 460.0);
            b.previous = b.pos;
            b.velocity = V2::new(0.34 + i as f32 * 0.18, -0.8).normalized() * 12000.0;
        }
        if relays {
            // Exercise an actual catch/release before the upward balls ignite
            // the board; this fast scenario clears before capsules can fall.
            g.balls[0].pos = V2::new(WIDTH / 2.0, PADDLE_Y - RADIUS - 1.0);
            g.balls[0].previous = g.balls[0].pos;
            g.balls[0].velocity = V2::new(0.0, 12000.0);
        }
        g.particles.fill(Particle {
            pos: V2::new(400.0, 400.0),
            velocity: V2::new(100.0, 100.0),
            life: 10.0,
            hue: 0,
        });
        for (i, d) in g.drops.iter_mut().enumerate() {
            *d = Drop {
                pos: V2::new(120.0 + i as f32 * 60.0, 300.0),
                power: Power::Multi,
                active: true,
            };
        }
    }
    g
}

/// How much work one scenario does.
struct Load {
    batches: usize,
    /// Ticks spent in each sector before moving to the next.
    sector_ticks: usize,
}
const BENCH: Load = Load {
    batches: 4096,
    sector_ticks: 20000,
};
const TEST: Load = Load {
    batches: 1024,
    sector_ticks: 5000,
};
const BATCH: usize = 64;

fn run(load: &Load, stress: bool, relays: bool) {
    let mut game = setup(stress, relays, 0);
    let mut samples = Vec::with_capacity(load.batches);
    let mut score = 0_u64;
    let mut caps = 0_u64;
    let mut impacts = 0_u64;
    let mut chain_ticks = 0_u64;
    let mut phase_hits = 0_u64;
    let mut catches = 0_u64;
    let mut level_mask = 0_u16;
    let mut tick = 0;
    ALLOCATIONS.store(0, Ordering::Relaxed);
    COUNTING.store(true, Ordering::Relaxed);
    for _ in 0..load.batches {
        let start = Instant::now();
        for _ in 0..BATCH {
            if tick % load.sector_ticks == 0
                || matches!(game.phase, Phase::GameOver | Phase::Victory)
                || (stress && (tick % 240 == 0 || game.phase != Phase::Playing))
            {
                score += u64::from(game.score);
                caps += game.collision_caps;
                game = setup(stress, relays, (tick / load.sector_ticks) % LEVEL_COUNT);
            }
            level_mask |= 1 << game.level;
            if stress {
                // Replenish depleted pools to keep the measured workload full.
                for (i, ball) in game.balls.iter_mut().enumerate() {
                    if !ball.active {
                        let pos = V2::new(240.0 + i as f32 * 160.0, 460.0);
                        *ball = Ball {
                            pos,
                            previous: pos,
                            velocity: V2::new(0.4, -0.8),
                            active: true,
                            ..Ball::default()
                        };
                    }
                    ball.velocity = ball.velocity.normalized() * 12000.0;
                }
                for (i, drop) in game.drops.iter_mut().enumerate() {
                    if !drop.active {
                        *drop = Drop {
                            pos: V2::new(120.0 + i as f32 * 60.0, 300.0),
                            power: if relays {
                                [
                                    Power::Multi,
                                    Power::Anchor,
                                    Power::Phase,
                                    Power::Wide,
                                    Power::Slow,
                                ][i % 5]
                            } else {
                                Power::Multi
                            },
                            active: true,
                        };
                    }
                }
            }
            let x = game
                .balls
                .iter()
                .find(|b| b.active)
                .map_or(WIDTH / 2.0, |b| b.pos.x)
                + (tick as f32 * 0.003).sin() * 36.0;
            game.step(black_box(&Input {
                mouse_x: Some(x),
                launch: true,
                ..Default::default()
            }));
            impacts += u64::from(game.events.brick);
            chain_ticks += u64::from(game.events.relay);
            phase_hits += u64::from(game.events.phase_hit);
            catches += u64::from(game.events.caught);
            if stress {
                for p in &mut game.particles {
                    p.life = 1.0;
                }
            }
            tick += 1;
        }
        samples.push(start.elapsed().as_secs_f64() * 1_000_000.0 / BATCH as f64);
        black_box(&game);
    }
    COUNTING.store(false, Ordering::Relaxed);
    let allocations = ALLOCATIONS.load(Ordering::Relaxed);
    score += u64::from(game.score);
    caps += game.collision_caps;
    samples.sort_unstable_by(f64::total_cmp);
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    println!(
        "{}: {} ticks, mean {:.3} us/tick, batch p95 {:.3}, p99 {:.3} us/tick",
        if relays {
            "Relay stress (84 cores, 3 fast balls, all five powers, full pools)"
        } else if stress {
            "Stress (3 balls @ 12k px/s, 384 particles, 12 drops)"
        } else {
            "Gameplay (autopaddle, all levels)"
        },
        BATCH * load.batches,
        mean,
        samples[load.batches * 95 / 100],
        samples[load.batches * 99 / 100]
    );
    println!(
        "  allocations {allocations}, collision caps {caps}, brick impact ticks {impacts}, score checksum {score}, level mask {level_mask:012b}"
    );
    println!("  relay ticks {chain_ticks}, phase hits {phase_hits}, anchor catches {catches}");
    if relays {
        assert!(
            chain_ticks > 100 && phase_hits > 100 && catches > 0,
            "Exercise every new mechanic"
        );
    }
    assert_eq!(allocations, 0, "Simulation allocated on the heap");
    assert_eq!(caps, 0, "Collision budget exhausted");
    assert!(impacts > 100, "Benchmark must exercise real collisions");
    if !stress {
        assert_eq!(
            level_mask,
            (1 << LEVEL_COUNT) - 1,
            "Gameplay must exercise all twelve sectors"
        );
    }
}

fn main() {
    let load = if std::env::args().any(|a| a == "--bench") {
        &BENCH
    } else {
        &TEST
    };
    println!("ARKONK headless simulation benchmark (batch percentiles; excludes graphics/audio)");
    run(load, false, false);
    run(load, true, false);
    run(load, true, true);
}
