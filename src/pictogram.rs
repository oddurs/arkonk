//! The capsules' pictograms and the pieces' glow, drawn once into the
//! atlas. A pictogram is the design's few strokes and dots, rasterized at
//! every pixel height a capsule can be drawn at, so it is always set 1:1
//! on whole pixels, as a glyph is.
use ark::Power;

/// The smallest and largest heights a pictogram is baked at, in pixels.
pub const MIN_H: u8 = 3;
pub const MAX_H: u8 = 32;
pub const HEIGHTS: usize = (MAX_H - MIN_H + 1) as usize;

/// The glow's side, in pixels. The atlas is sampled nearest, so the glow
/// is stretched over whole blocks of screen; at 64 its steps stay under a
/// few colour levels even under the paddle, where 32 showed bands.
pub const GLOW: usize = 64;

/// The glow's coverage: a soft radial falloff, `(1 − r²)²`, to nothing at
/// its edge. Stretched over a piece and tinted, it is the light the piece
/// casts, with no blur pass.
pub fn glow() -> Vec<u8> {
    let half = GLOW as f32 / 2.0;
    let mut out = vec![0; GLOW * GLOW];
    for y in 0..GLOW {
        for x in 0..GLOW {
            let dx = (x as f32 + 0.5) / half - 1.0;
            let dy = (y as f32 + 0.5) / half - 1.0;
            let a = (1.0 - (dx * dx + dy * dy)).max(0.0);
            out[y * GLOW + x] = (a * a * 255.0).round() as u8;
        }
    }
    out
}

#[derive(Clone, Copy)]
struct P(f32, f32);

/// One stroke or dot of a pictogram, in its view box's units.
enum Mark {
    /// A straight stroke with round caps: from, to, width.
    Line(P, P, f32),
    /// A cubic Bézier stroke with round caps.
    Curve([P; 4], f32),
    /// A filled dot: centre, radius.
    Dot(P, f32),
    /// A stroked circle: centre, radius, width, and its dash and gap,
    /// starting at three o'clock and running clockwise, as SVG draws one.
    Ring(P, f32, f32, Option<(f32, f32)>),
    /// The lower half of a circle, round-capped: centre, radius, width.
    Cradle(P, f32, f32),
}

/// A pictogram: its view box, how tall it stands in an 18-unit capsule,
/// and its marks. Values are the design's SVGs.
struct Icon {
    view: (f32, f32),
    height: f32,
    marks: &'static [Mark],
}

fn icon(power: Power) -> Icon {
    use Mark::*;
    match power {
        // A line with outward chevrons.
        Power::Wide => Icon {
            view: (24.0, 12.0),
            height: 20.0 / 3.0,
            marks: &[
                Line(P(3.0, 6.0), P(21.0, 6.0), 2.0),
                Line(P(7.0, 2.0), P(3.0, 6.0), 2.0),
                Line(P(3.0, 6.0), P(7.0, 10.0), 2.0),
                Line(P(17.0, 2.0), P(21.0, 6.0), 2.0),
                Line(P(21.0, 6.0), P(17.0, 10.0), 2.0),
            ],
        },
        // An hourglass.
        Power::Slow => Icon {
            view: (14.0, 16.0),
            height: 26.0 / 3.0,
            marks: &[
                Line(P(2.0, 1.5), P(12.0, 1.5), 1.8),
                Line(P(2.0, 14.5), P(12.0, 14.5), 1.8),
                Curve([P(3.0, 1.5), P(3.0, 5.5), P(11.0, 5.0), P(11.0, 8.0)], 1.8),
                Curve(
                    [P(11.0, 8.0), P(11.0, 11.0), P(3.0, 10.5), P(3.0, 14.5)],
                    1.8,
                ),
                Curve([P(11.0, 1.5), P(11.0, 5.5), P(3.0, 5.0), P(3.0, 8.0)], 1.8),
                Curve(
                    [P(3.0, 8.0), P(3.0, 11.0), P(11.0, 10.5), P(11.0, 14.5)],
                    1.8,
                ),
            ],
        },
        // Three balls: the outer two low, the middle one high.
        Power::Multi => Icon {
            view: (28.0, 12.0),
            height: 6.0,
            marks: &[
                Dot(P(5.0, 7.0), 2.6),
                Dot(P(14.0, 4.0), 2.6),
                Dot(P(23.0, 7.0), 2.6),
            ],
        },
        // A ball in a cradle.
        Power::Anchor => Icon {
            view: (18.0, 16.0),
            height: 26.0 / 3.0,
            marks: &[Dot(P(9.0, 4.5), 2.8), Cradle(P(9.0, 8.5), 6.5, 2.0)],
        },
        // A ring passing through a ring.
        Power::Phase => Icon {
            view: (22.0, 14.0),
            height: 8.0,
            marks: &[
                Ring(P(8.0, 7.0), 5.0, 1.9, None),
                Ring(P(14.0, 7.0), 5.0, 1.6, Some((2.2, 2.0))),
            ],
        },
    }
}

/// How tall `power`'s pictogram stands in a capsule 18 units tall.
pub fn height(power: Power) -> f32 {
    icon(power).height
}

/// `power`'s pictogram `h` pixels tall: its width, and white coverage
/// row by row. Strokes and dots never get thinner than a pixel, so the
/// smallest sizes stay solid.
pub fn raster(power: Power, h: u8) -> (usize, Vec<u8>) {
    const SUB: usize = 4;
    let icon = icon(power);
    let (vw, vh) = icon.view;
    let scale = f32::from(h) / vh;
    let w = (vw * scale).round().max(1.0) as usize;
    let h = usize::from(h);
    // Half a pixel, in view-box units: the thinnest a mark may be.
    let floor = 0.5 / scale;
    let mut out = vec![0; w * h];
    for y in 0..h {
        for x in 0..w {
            let mut hits = 0;
            for j in 0..SUB {
                for i in 0..SUB {
                    let p = P(
                        (x as f32 + (i as f32 + 0.5) / SUB as f32) / scale,
                        (y as f32 + (j as f32 + 0.5) / SUB as f32) / scale,
                    );
                    if icon.marks.iter().any(|m| covers(m, p, floor)) {
                        hits += 1;
                    }
                }
            }
            out[y * w + x] = (hits * 255 / (SUB * SUB)) as u8;
        }
    }
    (w, out)
}

fn covers(mark: &Mark, p: P, floor: f32) -> bool {
    match *mark {
        Mark::Line(a, b, w) => segment(p, a, b) <= (w / 2.0).max(floor),
        Mark::Curve(c, w) => {
            const PIECES: usize = 12;
            let half = (w / 2.0).max(floor);
            let mut from = c[0];
            (1..=PIECES).any(|k| {
                let to = bezier(c, k as f32 / PIECES as f32);
                let near = segment(p, from, to) <= half;
                from = to;
                near
            })
        }
        Mark::Dot(c, r) => distance(p, c) <= r.max(floor * 1.2),
        Mark::Ring(c, r, w, dash) => {
            if (distance(p, c) - r).abs() > (w / 2.0).max(floor) {
                return false;
            }
            dash.is_none_or(|(on, off)| {
                let turn = (p.1 - c.1)
                    .atan2(p.0 - c.0)
                    .rem_euclid(std::f32::consts::TAU);
                (turn * r).rem_euclid(on + off) < on
            })
        }
        Mark::Cradle(c, r, w) => {
            let half = (w / 2.0).max(floor);
            let ends = [P(c.0 - r, c.1), P(c.0 + r, c.1)];
            (p.1 >= c.1 && (distance(p, c) - r).abs() <= half)
                || ends.iter().any(|&e| distance(p, e) <= half)
        }
    }
}

fn distance(a: P, b: P) -> f32 {
    (a.0 - b.0).hypot(a.1 - b.1)
}

/// The distance from `p` to the segment from `a` to `b`.
fn segment(p: P, a: P, b: P) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let length = dx * dx + dy * dy;
    let t = if length > 0.0 {
        (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / length).clamp(0.0, 1.0)
    } else {
        0.0
    };
    distance(p, P(a.0 + t * dx, a.1 + t * dy))
}

fn bezier([a, b, c, d]: [P; 4], t: f32) -> P {
    let u = 1.0 - t;
    let (k0, k1, k2, k3) = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
    P(
        k0 * a.0 + k1 * b.0 + k2 * c.0 + k3 * d.0,
        k0 * a.1 + k1 * b.1 + k2 * c.1 + k3 * d.1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_pictogram_has_ink_at_every_height_and_keeps_its_aspect() {
        for power in Power::ALL {
            let (vw, vh) = icon(power).view;
            for h in MIN_H..=MAX_H {
                let (w, pixels) = raster(power, h);
                assert_eq!(pixels.len(), w * usize::from(h));
                assert!(
                    pixels.iter().any(|&a| a > 128),
                    "{power:?} at {h} px is empty"
                );
                let expected = vw / vh * f32::from(h);
                assert!((w as f32 - expected).abs() <= 0.5, "{power:?} at {h} px");
            }
        }
    }

    #[test]
    fn the_glow_fades_from_its_middle_to_nothing_at_its_edge() {
        let g = glow();
        let at = |x: usize, y: usize| g[y * GLOW + x];
        assert!(at(GLOW / 2, GLOW / 2) > 240);
        assert_eq!(at(0, 0), 0);
        assert!(at(0, GLOW / 2) <= 2);
        assert!(at(GLOW / 4, GLOW / 2) < at(GLOW / 2 - 2, GLOW / 2));
    }
}
