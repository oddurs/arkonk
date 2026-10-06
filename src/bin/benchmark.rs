//! Headless benchmarks. Run with `cargo run --release --bin benchmark`.
use arkonk::{game::*, physics::V2};
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

fn setup(stress: bool) -> Game {
    let mut g = Game::new();
    g.step(&Input {
        launch: true,
        ..Default::default()
    });
    if stress {
        for (i, b) in g.balls.iter_mut().enumerate() {
            b.active = true;
            b.pos = V2::new(240.0 + i as f32 * 160.0, 460.0);
            b.previous = b.pos;
            b.velocity = V2::new(0.34 + i as f32 * 0.18, -0.8).normalized() * 12000.0;
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

fn run(stress: bool) {
    const BATCH: usize = 64;
    const BATCHES: usize = 4096;
    let mut game = setup(stress);
    let mut samples = Vec::with_capacity(BATCHES);
    let mut score = 0_u64;
    let mut caps = 0_u64;
    let mut impacts = 0_u64;
    let mut level_mask = 0_u8;
    let mut tick = 0;
    ALLOCATIONS.store(0, Ordering::Relaxed);
    COUNTING.store(true, Ordering::Relaxed);
    for _ in 0..BATCHES {
        let start = Instant::now();
        for _ in 0..BATCH {
            if matches!(game.phase, Phase::GameOver | Phase::Victory)
                || (stress && (tick % 240 == 0 || game.phase != Phase::Playing))
            {
                score += u64::from(game.score);
                caps += game.collision_caps;
                game = setup(stress);
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
                        };
                    }
                    ball.velocity = ball.velocity.normalized() * 12000.0;
                }
                for (i, drop) in game.drops.iter_mut().enumerate() {
                    if !drop.active {
                        *drop = Drop {
                            pos: V2::new(120.0 + i as f32 * 60.0, 300.0),
                            power: Power::Multi,
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
        if stress {
            "Stress (3 balls @ 12k px/s, 384 particles, 12 drops)"
        } else {
            "Gameplay (autopaddle, all levels)"
        },
        BATCH * BATCHES,
        mean,
        samples[BATCHES * 95 / 100],
        samples[BATCHES * 99 / 100]
    );
    println!(
        "  allocations {allocations}, collision caps {caps}, brick impact ticks {impacts}, score checksum {score}, level mask {level_mask:05b}"
    );
    assert_eq!(allocations, 0, "Simulation allocated on the heap");
    assert_eq!(caps, 0, "Collision budget exhausted");
    assert!(impacts > 100, "Benchmark must exercise real collisions");
    if !stress {
        assert_eq!(
            level_mask, 0b11111,
            "Gameplay must exercise all five levels"
        );
    }
}

fn main() {
    println!("ARKONK headless simulation benchmark (batch percentiles; excludes graphics/audio)");
    run(false);
    run(true);
}
