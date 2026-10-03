// 78.82 — RECONSTRUCTION BY HOLONOMY (HR1–HR4, the operator's word: "the
// question is not to get exactness — it's to recreate the reconstruction via
// holonomy"). The values stay exact; what must agree across a family is how
// they MOVE. A rule is the transport whose loops close across the pairs.
//
//! HR1 — THE FIELD. Every site of a scene by its coordinates on every axis:
//!   its hue; the sphere `C = ΣPₛ` of its neighbourhood (itself and the
//!   painted sites one null step away) read by its moments — local mass,
//!   spread, inertia form (direction), density; the body it stands in — its
//!   mass, its rank by mass among the scene's bodies, and its CENTRALITY, the
//!   site's weight against the body's sphere over the sphere's own square,
//!   `M·(P·C)/C²`, exact (½ at the barycentre, rising outward); and its ring,
//!   the hues one null step away.
//! HR2 — THE TRANSPORT. What each site of an input does in its pair's
//!   output: its hue fibre's move (`h → h'`, `0` the void: carving), and the
//!   light it emits — each null direction whose whole free reach the output
//!   fills with one hue. The body axis's own transport, the fascicle
//!   `C_out · C_in` of a single change, is read by its regime.
//! HR3 — THE HOLONOMY PER AXIS. On an axis, every site is keyed by its
//!   coordinates there; two sites under one key are a LOOP — carry one by its
//!   transport and return by the other's — and the loop closes where the two
//!   transports are one. An axis closes on a family where every loop does: it
//!   carries the rule.
//! HR4 — RECONSTRUCTION. Each pair left out, the axes closing on the others
//!   and covering its keys; the most compact (fewest keys) carries the
//!   pair's input field, and the output is read by where the transported
//!   hues and light land — judged by replay against the observed output.

use super::incongruence::{moments, Cell, Change, Moments};
use super::luminal::DIRECTIONS;
use super::*;
use crate::network::nerve::Keyed;
use crate::transducer::address::put_key_bytes;
use num_rational::BigRational;

type Q = BigRational;

/// HR1 — ONE SITE BY ITS COORDINATES ON EVERY AXIS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Site {
    pub r: i64,
    pub c: i64,
    pub hue: u8,
    /// The neighbourhood's sphere, by its moments.
    pub local: Moments,
    /// The body it stands in (none for the void): mass, rank (heavier,
    /// lighter bodies), centrality — refused on a body with no spread.
    pub body_mass: u64,
    pub rank: (u64, u64),
    pub centrality: Option<Q>,
    /// The hues one null step away, in `DIRECTIONS` order; 255 off the frame.
    pub ring: [u8; 8],
}

fn at(g: &Grid, r: i64, c: i64) -> Option<u8> {
    let (ur, uc) = (usize::try_from(r).ok()?, usize::try_from(c).ok()?);
    g.get(ur)?.get(uc).copied()
}

/// HR1 — THE FIELD OF A SCENE, site by site in row order.
pub fn field(x: &Grid) -> Vec<Site> {
    let bodies = perceive(x);
    let rows = x.len();
    let mut owner: Vec<Vec<Option<usize>>> = x.iter().map(|row| vec![None; row.len()]).collect();
    // Each body's sums: mass, Σr, Σc, Σ|x|² — its sphere's coordinates.
    let mut sums: Vec<(i128, i128, i128, i128)> = Vec::with_capacity(bodies.len());
    let mut masses: Vec<u64> = Vec::with_capacity(bodies.len());
    for (i, b) in bodies.iter().enumerate() {
        let mut s = (0i128, 0i128, 0i128, 0i128);
        let mut mass = 0u64;
        for l in &b.compartments {
            let (Some(r), Some(c)) = (l.site.axis(0), l.site.axis(1)) else { continue };
            if let Some(slot) = usize::try_from(r).ok().zip(usize::try_from(c).ok()).and_then(|(ur, uc)| owner.get_mut(ur)?.get_mut(uc)) {
                *slot = Some(i);
            }
            let (r, c) = (i128::from(r), i128::from(c));
            s = (s.0 + 1, s.1 + r, s.2 + c, s.3 + r * r + c * c);
            mass += 1;
        }
        sums.push(s);
        masses.push(mass);
    }
    let mut out = Vec::new();
    for r in 0..rows {
        for (c, &hue) in x[r].iter().enumerate() {
            let (ri, ci) = (r as i64, c as i64);
            let mut near: Vec<Cell> = Vec::new();
            if hue != 0 {
                near.push((ri, ci, hue));
            }
            let mut ring = [255u8; 8];
            for (k, (dr, dc)) in DIRECTIONS.iter().enumerate() {
                if let Some(h) = at(x, ri + dr, ci + dc) {
                    ring[k] = h;
                    if h != 0 {
                        near.push((ri + dr, ci + dc, h));
                    }
                }
            }
            let (body_mass, rank, centrality) = match owner[r][c].and_then(|i| Some((sums.get(i).copied()?, *masses.get(i)?))) {
                None => (0, (0, 0), None),
                Some(((m, sr, sc, q), mass)) => {
                    let rank = sums.iter().fold((0u64, 0u64), |(a, b), o| (a + u64::from(o.0 > m), b + u64::from(o.0 < m)));
                    // M·(P·C)/C² = (M²|p|² − 2M p·Σx + M·Σ|x|²) / (2·spread).
                    let spread = m * q - sr * sr - sc * sc;
                    let (pr, pc) = (i128::from(ri), i128::from(ci));
                    let num = m * m * (pr * pr + pc * pc) - 2 * m * (pr * sr + pc * sc) + m * q;
                    let centrality = (spread != 0).then(|| Q::new(num.into(), (2 * spread).into()));
                    (mass, rank, centrality)
                }
            };
            out.push(Site { r: ri, c: ci, hue, local: moments(&near), body_mass, rank, centrality, ring });
        }
    }
    out
}

/// HR2 — WHAT ONE SITE DOES IN ITS PAIR'S OUTPUT: its fibre's target hue,
/// and the light it emits as (direction index, hue).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transport {
    pub to: u8,
    pub rays: Vec<(u8, u8)>,
}

/// HR2 — THE TRANSPORT FIELD of one pair, in `field`'s order. `None` where
/// the frames differ: no site stands in both.
pub fn transports(x: &Grid, y: &Grid) -> Option<Vec<Transport>> {
    if x.len() != y.len() || x.iter().zip(y).any(|(a, b)| a.len() != b.len()) {
        return None;
    }
    let mut out = Vec::new();
    for (r, row) in x.iter().enumerate() {
        for (c, &h) in row.iter().enumerate() {
            let (ri, ci) = (r as i64, c as i64);
            let to = at(y, ri, ci)?;
            let mut rays = Vec::new();
            if h != 0 {
                for (k, (dr, dc)) in (0u8..).zip(DIRECTIONS.iter()) {
                    let mut t = 1i64;
                    let mut lit: Option<u8> = None;
                    let mut whole = true;
                    while at(x, ri + t * dr, ci + t * dc) == Some(0) {
                        let o = at(y, ri + t * dr, ci + t * dc)?;
                        match lit {
                            None if o != 0 => lit = Some(o),
                            Some(l) if l == o => {}
                            _ => whole = false,
                        }
                        t += 1;
                    }
                    if let (true, Some(l)) = (whole, lit) {
                        rays.push((k, l));
                    }
                }
            }
            out.push(Transport { to, rays });
        }
    }
    Some(out)
}

/// HR2 on the body axis — each single change's fascicle `C_out · C_in`, by
/// its regime: turn of finite order, rosette, kiss (light), apart (dilation),
/// null.
pub fn body_regimes(changes: &[Change]) -> [u64; 5] {
    let mut n = [0u64; 5];
    for ch in changes {
        let sites = |cells: &[Cell]| cells.iter().map(|x| (x.0, x.1)).collect::<Vec<_>>();
        let (cin, _) = super::confluence::gathered(&sites(&ch.input));
        let (cout, _) = super::confluence::gathered(&sites(&ch.output));
        let at = match crate::transducer::cga::pair_regime(&[cin, cout], 0, 1) {
            crate::transducer::cga::PairRegime::Finite(_) => 0,
            crate::transducer::cga::PairRegime::Rosette => 1,
            crate::transducer::cga::PairRegime::Parabolic => 2,
            crate::transducer::cga::PairRegime::Boost => 3,
            crate::transducer::cga::PairRegime::Null => 4,
        };
        n[at] += 1;
    }
    n
}

/// The axes a site is keyed on, in the order [`key`] reads them. Every key
/// opens with the site's own hue: the fibre it moves on.
pub const AXES: [&str; 9] = [
    "hue",
    "local mass",
    "local direction",
    "local density",
    "local moments",
    "body mass",
    "mass rank",
    "centrality",
    "neighbourhood",
];

fn put_q(k: &mut Vec<u8>, q: &Option<Q>) {
    match q {
        None => k.push(0),
        Some(q) => {
            k.push(1);
            put_key_bytes(k, &q.numer().to_signed_bytes_be());
            put_key_bytes(k, &q.denom().to_signed_bytes_be());
        }
    }
}

/// A SITE'S COORDINATES ON ONE AXIS, as its key.
pub fn key(s: &Site, axis: usize) -> Vec<u8> {
    let mut k = vec![s.hue];
    let int = |k: &mut Vec<u8>, v: i128| put_key_bytes(k, &v.to_be_bytes());
    match axis {
        1 => int(&mut k, i128::from(s.local.mass)),
        2 => s.local.direction().iter().for_each(|v| int(&mut k, *v)),
        3 => put_q(&mut k, &s.local.density()),
        4 => {
            int(&mut k, i128::from(s.local.mass));
            int(&mut k, s.local.spread);
            s.local.direction().iter().for_each(|v| int(&mut k, *v));
        }
        5 => int(&mut k, i128::from(s.body_mass)),
        6 => {
            int(&mut k, i128::from(s.rank.0));
            int(&mut k, i128::from(s.rank.1));
        }
        7 => put_q(&mut k, &s.centrality),
        8 => k.extend_from_slice(&s.ring),
        _ => {}
    }
    k
}

/// HR3 — ONE AXIS ON A SET OF PAIRS: its loops, those that closed, and its
/// distinct keys (the compaction: fewer keys, more of the field carried by
/// one transport).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Closure {
    pub loops: u64,
    pub closed: u64,
    pub keys: u64,
}

impl Closure {
    /// Every loop closed, and there was at least one.
    pub fn closes(&self) -> bool {
        self.loops > 0 && self.closed == self.loops
    }
}

/// The transport each key carries on one axis over a set of pairs, and the
/// axis's closure there.
fn table(pairs: &[(&[Site], &[Transport])], axis: usize) -> (Keyed<Transport>, Closure) {
    let mut t: Keyed<Transport> = Keyed::new();
    let mut cl = Closure::default();
    for (sites, moves) in pairs {
        for (s, m) in sites.iter().zip(moves.iter()) {
            let k = key(s, axis);
            match t.get(&k) {
                Some(held) => {
                    cl.loops += 1;
                    cl.closed += u64::from(held == m);
                }
                None => {
                    t.insert(&k, m.clone());
                    cl.keys += 1;
                }
            }
        }
    }
    (t, cl)
}

/// HR3 — THE HOLONOMY OF EVERY AXIS over a set of pairs.
pub fn holonomy(pairs: &[(&[Site], &[Transport])]) -> [Closure; 9] {
    std::array::from_fn(|axis| table(pairs, axis).1)
}

/// HR4 — WHAT THE RECONSTRUCTION DID ON ONE FAMILY.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Reconstruction {
    pub pairs: u64,
    /// Pairs whose frames agree (a field transports), and those reproduced.
    pub framed: u64,
    pub reproduced: u64,
    /// The axis that carried each reproduced pair.
    pub by_axis: [u64; 9],
    /// The axes that close on the whole family, and each axis's loops there.
    pub closing: [bool; 9],
    pub holonomy: [Closure; 9],
    /// Left-out pairs no axis closed on the others for, and pairs where an
    /// axis closed but the pair holds a key the others never showed.
    pub unclosed: u64,
    pub unseen: u64,
    /// Every pair reproduced.
    pub whole: bool,
}

/// Carry an input field by a table: every site to its key's target hue,
/// then the light each emits laid along its free reach.
fn carried(x: &Grid, sites: &[Site], t: &Keyed<Transport>, axis: usize) -> Option<Grid> {
    let mut out: Grid = x.iter().map(|row| vec![0u8; row.len()]).collect();
    let mut emitted: Vec<(i64, i64, u8)> = Vec::new();
    for s in sites {
        let m = t.get(&key(s, axis))?;
        let (ur, uc) = (usize::try_from(s.r).ok()?, usize::try_from(s.c).ok()?);
        *out.get_mut(ur)?.get_mut(uc)? = m.to;
        for &(k, h) in &m.rays {
            let (dr, dc) = *DIRECTIONS.get(usize::from(k))?;
            let mut step = 1i64;
            while at(x, s.r + step * dr, s.c + step * dc) == Some(0) {
                emitted.push((s.r + step * dr, s.c + step * dc, h));
                step += 1;
            }
        }
    }
    for (r, c, h) in emitted {
        let (ur, uc) = (usize::try_from(r).ok()?, usize::try_from(c).ok()?);
        *out.get_mut(ur)?.get_mut(uc)? = h;
    }
    Some(out)
}

/// HR4 — RECONSTRUCT A FAMILY BY ITS HOLONOMY, each pair left out in turn.
pub fn reconstruct(family: &[(Grid, Grid)]) -> Reconstruction {
    let mut read = Reconstruction { pairs: family.len() as u64, ..Reconstruction::default() };
    let fields: Vec<Option<(Vec<Site>, Vec<Transport>)>> =
        family.iter().map(|(x, y)| transports(x, y).map(|t| (field(x), t))).collect();
    let all: Vec<(&[Site], &[Transport])> = fields.iter().flatten().map(|(s, t)| (s.as_slice(), t.as_slice())).collect();
    let whole_family = holonomy(&all);
    read.closing = whole_family.map(|c| c.closes());
    read.holonomy = whole_family;
    for (i, f) in fields.iter().enumerate() {
        let Some((sites, _)) = f else { continue };
        read.framed += 1;
        let others: Vec<(&[Site], &[Transport])> =
            fields.iter().enumerate().filter(|(j, _)| *j != i).filter_map(|(_, g)| g.as_ref()).map(|(s, t)| (s.as_slice(), t.as_slice())).collect();
        // The axes that close on the others and hold every key this pair
        // asks for; the most compact carries it.
        let mut best: Option<(u64, usize, Keyed<Transport>)> = None;
        let mut any_closed = false;
        for axis in 0..AXES.len() {
            let (t, cl) = table(&others, axis);
            if !cl.closes() {
                continue;
            }
            any_closed = true;
            if sites.iter().any(|s| t.get(&key(s, axis)).is_none()) {
                continue;
            }
            if best.as_ref().is_none_or(|(keys, _, _)| cl.keys < *keys) {
                best = Some((cl.keys, axis, t));
            }
        }
        let Some((_, axis, t)) = best else {
            match any_closed {
                true => read.unseen += 1,
                false => read.unclosed += 1,
            }
            continue;
        };
        if carried(&family[i].0, sites, &t, axis).as_ref() == Some(&family[i].1) {
            read.reproduced += 1;
            read.by_axis[axis] += 1;
        }
    }
    read.whole = read.pairs > 0 && read.reproduced == read.pairs;
    read
}
