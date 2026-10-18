// 78.77 — LIGHT THROUGH THE SCENE, READ (L1–L6, an instrument, deciding
// nothing): the bodies stand congruent as perceived, and what the output
// shows beyond them is read as light — a waveform carried along null flows
// out of a body's boundary, met at interfaces, captured inside enclosures,
// resonating in cavities.
//
//! - L1 RAYS: from every site of an input body, along each of the scene's
//!   eight null directions (a translation's generator `d ∧ e_∞` is null), the
//!   ray's sites with their exact phase `t`, up to the frame's edge or the
//!   first site an input body holds.
//! - L2 WAVEFORM: the output's hues along the ray as a function of phase. A
//!   ray is LIT where the output holds hue from `t = 1` on; a lit prefix is a
//!   WAVEFORM where it is two or more sites and repeats its period at least
//!   twice — its period `p`, its hues over one period. Only a waveform
//!   explains an unsaid site on it.
//! - L3 INTERFACES: where a ray meets a body — beyond it, the light
//!   TRANSMITTED (a waveform past the body, its period's ratio to the
//!   incoming one the medium), REFLECTED (a waveform from the last free site
//!   along a mirrored direction), or ABSORBED.
//! - L4 CAPTURED: an enclosure is a component of the input's empty sites
//!   touching no edge of the frame; its output hues a STANDING WAVE, read by
//!   its periods along rows and columns.
//! - L5 CAVITY: a rectangular enclosure's walls are four planes `n + δ e_∞`
//!   between its cells and its wall; a diagonal ray's billiard round them
//!   is their product, an exact versor read by its modes; whether the
//!   standing wave's periods divide the cavity — the mode fits.
//! - L6 is read by the instrument off the cover's own tally.
//!
//! Exact throughout: phases and periods are integers on the scene's
//! compartments, ratios rational, the cavity's holonomy a versor of
//! reflectors. Nothing here speaks, mints or decides.

use super::*;
use crate::transducer::cga::{self, Mv};
use num_rational::BigRational;

/// The scene's null directions on its compartments: the axes and diagonals.
pub const DIRECTIONS: [(i64, i64); 8] = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];

/// What lies beyond the body a ray met.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Beyond {
    /// A waveform past the body: its period, against the incoming one's.
    Transmitted { period: u64 },
    /// A waveform from the last free site along a mirrored direction.
    Reflected { dir: (i64, i64), period: u64 },
    Absorbed,
}

/// Where a ray's free run ended.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stop {
    Edge,
    Body { hue: u8, beyond: Beyond },
}

/// ONE WAVEFORM RAY: its source body's hue, its direction, its free reach,
/// the lit prefix and its period and hues over one period, where it stopped,
/// and the unsaid sites it explains.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ray {
    pub source: u8,
    pub dir: (i64, i64),
    pub reach: u64,
    pub lit: u64,
    pub period: u64,
    pub wave: Vec<u8>,
    pub stop: Stop,
    pub explains: u64,
}

/// ONE ENCLOSURE OF THE INPUT: the hue of the walls holding it (one hue, or
/// none), its sites, its rectangle where it is one, the standing wave's
/// periods along rows and columns where the output fills it whole, whether
/// those periods divide the rectangle, the billiard's modes, and the unsaid
/// sites it captures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cavity {
    pub wall: Option<u8>,
    pub sites: u64,
    pub rect: Option<(u64, u64)>,
    pub periods: Option<(u64, u64)>,
    pub fits: Option<bool>,
    pub modes: Option<Vec<crate::network::nerve::coord::BivectorKind>>,
    pub explains: u64,
}

/// WHAT THE LIGHT READS ON ONE PAIR.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PairLight {
    /// The pair's output frame differs from its input's: no ray is read.
    pub unframed: bool,
    pub unsaid: u64,
    pub on_ray: u64,
    pub captured: u64,
    pub both: u64,
    pub neither: u64,
    pub rays: Vec<Ray>,
    pub cavities: Vec<Cavity>,
}

fn at(g: &Grid, r: i64, c: i64) -> Option<u8> {
    let (ur, uc) = (usize::try_from(r).ok()?, usize::try_from(c).ok()?);
    g.get(ur)?.get(uc).copied()
}

/// THE PERIOD OF A HUE SEQUENCE: the least `p` with `h[t + p] = h[t]` over
/// the whole sequence.
fn period_of(h: &[u8]) -> u64 {
    let n = h.len();
    (1..=n).find(|&p| (0..n - p).all(|t| h[t] == h[t + p])).map_or(n as u64, |p| p as u64)
}

/// A LIT PREFIX AS A WAVEFORM: two or more sites, repeating its period at
/// least twice. `Some((period, hues of one period))` where it is one.
fn waveform(h: &[u8]) -> Option<(u64, Vec<u8>)> {
    if h.len() < 2 {
        return None;
    }
    let p = period_of(h);
    let pu = usize::try_from(p).ok()?;
    (pu * 2 <= h.len()).then(|| (p, h[..pu].to_vec()))
}

/// The output's lit run along `dir` from `(r, c)` (exclusive), over sites the
/// input leaves free: its hues and its sites.
fn lit_run(x: &Grid, y: &Grid, (r, c): (i64, i64), dir: (i64, i64)) -> (Vec<u8>, Vec<(i64, i64)>) {
    let void = crate::arc::ground();
    let (mut hues, mut sites) = (Vec::new(), Vec::new());
    let mut t = 1i64;
    loop {
        let (rr, cc) = (r + t * dir.0, c + t * dir.1);
        match (at(x, rr, cc), at(y, rr, cc)) {
            (Some(free), Some(h)) if free == void && h != void => {
                hues.push(h);
                sites.push((rr, cc));
            }
            _ => break,
        }
        t += 1;
    }
    (hues, sites)
}

/// L1–L3 ON ONE PAIR: every waveform ray out of every input body's sites.
fn rays_of(x: &Grid, y: &Grid, unsaid: &[(i64, i64)], lit_by_ray: &mut Vec<(i64, i64)>) -> Vec<Ray> {
    let mut out = Vec::new();
    for (r, row) in x.iter().enumerate() {
        for (c, &src) in row.iter().enumerate() {
            if src == 0 {
                continue;
            }
            let (r, c) = (r as i64, c as i64);
            for dir in DIRECTIONS {
                // The free reach: sites the input leaves empty, to the edge or
                // the first body.
                let mut reach = 0i64;
                let stop_at = loop {
                    let (rr, cc) = (r + (reach + 1) * dir.0, c + (reach + 1) * dir.1);
                    match at(x, rr, cc) {
                        None => break None,
                        Some(0) => reach += 1,
                        Some(h) => break Some((h, (rr, cc))),
                    }
                };
                if reach == 0 {
                    continue;
                }
                let (hues, sites) = lit_run(x, y, (r, c), dir);
                let Some((period, wave)) = waveform(&hues) else { continue };
                let explains = sites.iter().filter(|s| unsaid.contains(s)).count() as u64;
                if explains == 0 {
                    continue;
                }
                lit_by_ray.extend(sites.iter().filter(|s| unsaid.contains(s)));
                let stop = match stop_at {
                    None => Stop::Edge,
                    Some((hue, (br, bc))) => {
                        // Through the body, then what the output shows past it.
                        let mut t = 0i64;
                        while at(x, br + t * dir.0, bc + t * dir.1).is_some_and(|h| h != crate::arc::ground()) {
                            t += 1;
                        }
                        let exit = (br + (t - 1) * dir.0, bc + (t - 1) * dir.1);
                        let (past, _) = lit_run(x, y, exit, dir);
                        let last = (r + reach * dir.0, c + reach * dir.1);
                        let beyond = match waveform(&past) {
                            Some((p, _)) => Beyond::Transmitted { period: p },
                            None => {
                                let mirrored = [(-dir.0, dir.1), (dir.0, -dir.1)]
                                    .into_iter()
                                    .filter(|m| *m != (-dir.0, -dir.1) && *m != dir)
                                    .find_map(|m| waveform(&lit_run(x, y, last, m).0).map(|(p, _)| (m, p)));
                                match mirrored {
                                    Some((m, p)) => Beyond::Reflected { dir: m, period: p },
                                    None => Beyond::Absorbed,
                                }
                            }
                        };
                        Stop::Body { hue, beyond }
                    }
                };
                out.push(Ray { source: src, dir, reach: reach as u64, lit: hues.len() as u64, period, wave, stop, explains });
            }
        }
    }
    out
}

/// A WALL OF A CAVITY: the plane `n + δ e_∞` normal to the base axis whose
/// mask is `axis`, at `δ`.
fn wall(axis: usize, delta: BigRational) -> Mv {
    Mv::blade(axis, BigRational::from_integer(1.into())).add(&cga::e_inf().scale(&delta))
}

/// L4, L5 ON ONE PAIR: the input's enclosures, each read as a standing wave
/// and, where rectangular, a billiard.
fn cavities_of(x: &Grid, y: &Grid, unsaid: &[(i64, i64)], captured: &mut Vec<(i64, i64)>, budget: &mut crate::physics::Budget) -> Vec<Cavity> {
    let rows = x.len() as i64;
    let mut seen: Vec<(i64, i64)> = Vec::new();
    let mut out = Vec::new();
    for (r0, row) in x.iter().enumerate() {
        for (c0, &h) in row.iter().enumerate() {
            let start = (r0 as i64, c0 as i64);
            if h != crate::arc::ground() || seen.contains(&start) {
                continue;
            }
            // The empty component, 4-connected.
            let mut comp = vec![start];
            seen.push(start);
            let mut i = 0;
            let mut walls: Vec<u8> = Vec::new();
            let mut open = false;
            while i < comp.len() {
                let (r, c) = comp[i];
                i += 1;
                for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let n = (r + dr, c + dc);
                    match at(x, n.0, n.1) {
                        None => open = true,
                        Some(0) => {
                            if !seen.contains(&n) {
                                seen.push(n);
                                comp.push(n);
                            }
                        }
                        Some(w) => {
                            if !walls.contains(&w) {
                                walls.push(w);
                            }
                        }
                    }
                }
            }
            if open || rows == 0 {
                continue;
            }
            let explains = comp.iter().filter(|s| unsaid.contains(s)).count() as u64;
            captured.extend(comp.iter().filter(|s| unsaid.contains(s)));
            let (rmin, rmax) = comp.iter().fold((i64::MAX, i64::MIN), |(a, b), s| (a.min(s.0), b.max(s.0)));
            let (cmin, cmax) = comp.iter().fold((i64::MAX, i64::MIN), |(a, b), s| (a.min(s.1), b.max(s.1)));
            let (hgt, wid) = ((rmax - rmin + 1) as u64, (cmax - cmin + 1) as u64);
            let rect = (comp.len() as u64 == hgt * wid).then_some((hgt, wid));
            // The standing wave: the output fills the cavity whole, read by
            // its periods along rows and columns.
            let filled = comp.iter().all(|s| at(y, s.0, s.1).is_some_and(|v| v != 0));
            let periods = match (rect, filled) {
                (Some(_), true) => {
                    let row_seq = |r: i64| (cmin..=cmax).filter_map(|c| at(y, r, c)).collect::<Vec<u8>>();
                    let col_seq = |c: i64| (rmin..=rmax).filter_map(|r| at(y, r, c)).collect::<Vec<u8>>();
                    let pc = (rmin..=rmax).map(|r| period_of(&row_seq(r))).fold(1u64, |a, b| a.max(b));
                    let pr = (cmin..=cmax).map(|c| period_of(&col_seq(c))).fold(1u64, |a, b| a.max(b));
                    Some((pr, pc))
                }
                _ => None,
            };
            let fits = match (rect, periods) {
                (Some((hh, ww)), Some((pr, pc))) => Some(hh % pr == 0 && ww % pc == 0),
                _ => None,
            };
            // The billiard round a rectangle: its four walls between cell and
            // wall, the product an exact versor, read by its modes.
            let modes = rect.and_then(|_| {
                let half = BigRational::new(1.into(), 2.into());
                let q = |n: i64| BigRational::from_integer(n.into());
                let w = [
                    wall(cga::E1, q(rmin) - &half),
                    wall(cga::E2, q(cmin) - &half),
                    wall(cga::E1, q(rmax) + &half),
                    wall(cga::E2, q(cmax) + &half),
                ];
                let omega = w[3].geo(&w[2]).geo(&w[1]).geo(&w[0]);
                cga::motion_modes(&omega, budget)?.ok().map(|ms| ms.iter().map(|m| m.kind).collect())
            });
            out.push(Cavity {
                wall: match walls.as_slice() {
                    [one] => Some(*one),
                    _ => None,
                },
                sites: comp.len() as u64,
                rect,
                periods,
                fits,
                modes,
                explains,
            });
        }
    }
    out
}

/// READ THE LIGHT ON ONE PAIR against what the form left unsaid there.
pub fn read_pair(x: &Grid, y: &Grid, remainder: &[(i64, i64, u8, u8)], budget: &mut crate::physics::Budget) -> PairLight {
    let unsaid: Vec<(i64, i64)> = remainder.iter().filter(|l| l.3 != 0).map(|l| (l.0, l.1)).collect();
    let mut read = PairLight { unsaid: unsaid.len() as u64, ..PairLight::default() };
    let same_frame = x.len() == y.len() && x.iter().zip(y).all(|(a, b)| a.len() == b.len());
    if !same_frame {
        read.unframed = true;
        read.neither = read.unsaid;
        return read;
    }
    let mut on_ray: Vec<(i64, i64)> = Vec::new();
    let mut captured: Vec<(i64, i64)> = Vec::new();
    read.rays = rays_of(x, y, &unsaid, &mut on_ray);
    read.cavities = cavities_of(x, y, &unsaid, &mut captured, budget);
    for s in &unsaid {
        match (on_ray.contains(s), captured.contains(s)) {
            (true, true) => read.both += 1,
            (true, false) => read.on_ray += 1,
            (false, true) => read.captured += 1,
            (false, false) => read.neither += 1,
        }
    }
    read
}
