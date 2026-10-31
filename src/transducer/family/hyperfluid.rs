// 78.83 — TIME'S OWN CONGRUENCE (HR5: time needs its own congruence for
// hyperfluidity, because its axes act one at a time). HR3
// froze time: two sites' transports closed a loop only where they were EQUAL,
// so the same flow observed at two times — a ray of three and a ray of five
// out of the same kind of source — read as two motions. A pair is a sample of
// a flow (78.75), and time is distance along it (78.43).
//
//! THE TRANSPORT AS FLOWS. A site's transport is one composite motion: its
//! hue fibre's move (the gauge, which carries no time) and the light it
//! emits, each ray a point on the NULL FLOW of its plane `d ∧ e∞` —
//! `FlowPoint::at(d ∧ e∞, L)`, its length `L` its time, its parameter adding
//! as a null flow's does. Each ray ends OBSTRUCTED (the frame's edge or a
//! body: the flow runs until the scene stops it) or FREE (it ends in open
//! space).
//!
//! CONGRUENT IN TIME. Two transports are one flow at two times where the
//! transports' gauges agree and the transports' rays pair off on the SAME PLANE (`against` reads one
//! on the other's reference), with the same hue and the same kind of end —
//! whatever the parameters. A loop that closes so has no lift between its
//! axes: it is the field sliding along one orbit (hyperfluid). What a loop
//! keeps open is read by where it lifts: the gauge (the colour differs) or
//! the light (a generator differs).
//!
//! THE TIME, READ, NEVER KEYED. Reconstruction carries a site by its key's
//! flow and reads each ray's time off the scene: an obstructed ray runs its
//! whole free reach; a free ray runs the one time every observation of it
//! agreed on, or its time is unread and the pair is refused by name.

use super::luminal::DIRECTIONS;
use super::reconstruct::{at, field, key, Site, AXES};
use super::*;
use crate::network::nerve::Keyed;
use crate::transducer::cga::{base_vector, e_inf, wedge, FlowPoint};
use num_rational::BigRational;

type Q = BigRational;

/// How a ray of light ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum End {
    /// At the frame's edge (`true`) or an input body (`false`): the scene
    /// stopped it, so its time is the scene's free reach.
    Obstructed(bool),
    /// In open space: its time is its own.
    Free,
}

impl End {
    fn kind(self) -> u8 {
        match self {
            End::Obstructed(_) => 0,
            End::Free => 1,
        }
    }
}

/// ONE RAY AS A POINT ON ITS NULL FLOW.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Light {
    pub dir: u8,
    pub hue: u8,
    pub end: End,
    /// `FlowPoint::at(d ∧ e∞, L)`: the plane its generator, `L` its time.
    pub flow: FlowPoint,
}

/// A SITE'S COMPOSITE TRANSPORT: the gauge and the light, together.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Flow {
    pub to: u8,
    pub light: Vec<Light>,
}

/// THE NULL PLANE OF A DIRECTION: `d ∧ e∞`, whose flow is the translation.
fn null_plane(dir: (i64, i64)) -> crate::transducer::cga::Mv {
    let d = base_vector(&[Q::from_integer(dir.0.into()), Q::from_integer(dir.1.into())]);
    wedge(&[d, e_inf()])
}

/// The free reach along `dir` from `(r, c)` and how it ends: the input's
/// empty sites up to the frame's edge (`true`) or the first body (`false`).
fn reach(x: &Grid, (r, c): (i64, i64), (dr, dc): (i64, i64)) -> (i64, bool) {
    let mut t = 1i64;
    loop {
        match at(x, r + t * dr, c + t * dc) {
            Some(0) => t += 1,
            Some(_) => return (t - 1, false),
            None => return (t - 1, true),
        }
    }
}

/// HR5 — THE FLOW FIELD of one pair, in `field`'s order. `None` where the
/// frames differ.
pub fn flows(x: &Grid, y: &Grid) -> Option<Vec<Flow>> {
    if x.len() != y.len() || x.iter().zip(y).any(|(a, b)| a.len() != b.len()) {
        return None;
    }
    let mut out = Vec::new();
    // The void sites the light reaches, gathered where each ray is read.
    let void = crate::arc::ground();
    let mut lit: Vec<(i64, i64)> = Vec::new();
    for (r, row) in x.iter().enumerate() {
        for (c, &h) in row.iter().enumerate() {
            let (ri, ci) = (r as i64, c as i64);
            let to = at(y, ri, ci)?;
            let mut light = Vec::new();
            if h != void {
                for (k, &(dr, dc)) in (0u8..).zip(DIRECTIONS.iter()) {
                    let (free, edge) = reach(x, (ri, ci), (dr, dc));
                    let Some(hue) = at(y, ri + dr, ci + dc).filter(|&o| o != void && free > 0) else { continue };
                    let mut len = 1i64;
                    while len < free && at(y, ri + (len + 1) * dr, ci + (len + 1) * dc) == Some(hue) {
                        len += 1;
                    }
                    let end = match len == free {
                        true => End::Obstructed(edge),
                        false => End::Free,
                    };
                    let flow = FlowPoint::at(&null_plane((dr, dc)), &Q::from_integer(len.into()))?;
                    lit.extend((1..=len).map(|s| (ri + s * dr, ci + s * dc)));
                    light.push(Light { dir: k, hue, end, flow });
                }
            }
            out.push(Flow { to, light });
        }
    }
    // A VOID SITE THE LIGHT REACHES MOVES NOTHING OF ITS OWN: its colour is
    // the arrival of a flow emitted elsewhere, carried by that ray, so its own
    // gauge is the void's (0 → 0). Counted as its own move it would read the
    // light twice, once at its source and once as a gauge lift where it lands.
    let sites = x.iter().enumerate().flat_map(|(r, row)| (0..row.len()).map(move |c| (r as i64, c as i64)));
    for (f, (r, c)) in out.iter_mut().zip(sites) {
        if at(x, r, c) == Some(0) && lit.contains(&(r, c)) {
            f.to = 0;
        }
    }
    Some(out)
}

/// HOW TWO TRANSPORTS STAND TO EACH OTHER.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Congruence {
    /// Equal, time included.
    Equal,
    /// One flow at two times: gauges agree, rays pair off on one plane with
    /// one hue and one kind of end, parameters free.
    InTime,
    /// The gauge lifts: the colour differs.
    Gauge,
    /// The light lifts: a ray's generator, hue or end differs.
    Light,
}

/// TWO TRANSPORTS COMPARED, time read apart from the flow.
pub fn congruence(a: &Flow, b: &Flow) -> Congruence {
    if a == b {
        return Congruence::Equal;
    }
    if a.to != b.to {
        return Congruence::Gauge;
    }
    let same_flow = |p: &Light, q: &Light| p.dir == q.dir && p.hue == q.hue && p.end.kind() == q.end.kind() && q.flow.against(p.flow.plane()).is_some();
    let paired = a.light.len() == b.light.len() && a.light.iter().all(|p| b.light.iter().any(|q| same_flow(p, q)));
    match paired {
        true => Congruence::InTime,
        false => Congruence::Light,
    }
}

/// HR5 — ONE AXIS's LOOPS, judged in time: closed equal, closed in time
/// only, open by the gauge, open by the light; and its keys.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TimeClosure {
    pub loops: u64,
    pub equal: u64,
    pub in_time: u64,
    pub gauge: u64,
    /// Of the gauge lifts, those on a void site (a creation in place that no
    /// ray carries); the rest recolour a painted site.
    pub gauge_void: u64,
    pub light: u64,
    pub keys: u64,
}

impl TimeClosure {
    /// Every loop closed, equal or in time, and there was one.
    pub fn closes(&self) -> bool {
        self.loops > 0 && self.equal + self.in_time == self.loops
    }
}

/// What a key carries on one axis: its flow, and per free ray (by direction
/// and hue) every time it was observed at.
struct Held {
    flow: Flow,
    free_times: Vec<(u8, u8, Q)>,
}

fn table(pairs: &[(&[Site], &[Flow])], axis: usize) -> (Keyed<Held>, TimeClosure) {
    let mut t: Keyed<Held> = Keyed::new();
    let mut cl = TimeClosure::default();
    for (sites, moves) in pairs {
        for (s, m) in sites.iter().zip(moves.iter()) {
            let k = key(s, axis);
            let free: Vec<(u8, u8, Q)> = m
                .light
                .iter()
                .filter(|l| l.end == End::Free)
                .filter_map(|l| Some((l.dir, l.hue, l.flow.parameter()?)))
                .collect();
            match t.get_mut(&k) {
                Some(held) => {
                    cl.loops += 1;
                    match congruence(&held.flow, m) {
                        Congruence::Equal => cl.equal += 1,
                        Congruence::InTime => cl.in_time += 1,
                        Congruence::Gauge => {
                            cl.gauge += 1;
                            cl.gauge_void += u64::from(s.hue == crate::arc::ground());
                        }
                        Congruence::Light => cl.light += 1,
                    }
                    held.free_times.extend(free);
                }
                None => {
                    t.insert(&k, Held { flow: m.clone(), free_times: free });
                    cl.keys += 1;
                }
            }
        }
    }
    (t, cl)
}

/// HR5 — THE HOLONOMY OF EVERY AXIS IN TIME over a set of pairs.
pub fn holonomy_in_time(pairs: &[(&[Site], &[Flow])]) -> [TimeClosure; 9] {
    std::array::from_fn(|axis| table(pairs, axis).1)
}

/// HR5 — WHAT RECONSTRUCTION IN TIME DID ON ONE FAMILY.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InTime {
    pub pairs: u64,
    pub framed: u64,
    pub reproduced: u64,
    pub by_axis: [u64; 9],
    pub holonomy: [TimeClosure; 9],
    /// Left-out pairs no axis closed for in time; an axis closed but a key
    /// was unseen; and an axis carried it but a free ray's time was unread
    /// (its observations disagree).
    pub unclosed: u64,
    pub unseen: u64,
    pub time_unread: u64,
    pub whole: bool,
}

/// Carry an input field by a table in time. `Err(())` where a free ray's
/// time is unread; `Ok(None)` where a key is unseen.
fn carried(x: &Grid, sites: &[Site], t: &Keyed<Held>, axis: usize) -> Result<Option<Grid>, ()> {
    let mut out: Grid = x.iter().map(|row| vec![crate::arc::ground(); row.len()]).collect();
    let mut emitted: Vec<(i64, i64, u8)> = Vec::new();
    for s in sites {
        let Some(held) = t.get(&key(s, axis)) else { return Ok(None) };
        let (Ok(ur), Ok(uc)) = (usize::try_from(s.r), usize::try_from(s.c)) else { return Ok(None) };
        if let Some(cell) = out.get_mut(ur).and_then(|row| row.get_mut(uc)) {
            *cell = held.flow.to;
        }
        for l in &held.flow.light {
            let Some(&(dr, dc)) = DIRECTIONS.get(usize::from(l.dir)) else { return Ok(None) };
            let (free, _) = reach(x, (s.r, s.c), (dr, dc));
            // THE TIME, READ: the scene's reach where the scene stopped the
            // flow; the one agreed time where it ran free.
            let time: i64 = match l.end {
                End::Obstructed(_) => free,
                End::Free => {
                    let seen: Vec<&Q> = held.free_times.iter().filter(|(d, h, _)| *d == l.dir && *h == l.hue).map(|(_, _, q)| q).collect();
                    match seen.split_first() {
                        Some((first, rest)) if rest.iter().all(|q| q == first) && first.is_integer() => {
                            i64::try_from(first.to_integer()).map_err(|_| ())?.min(free)
                        }
                        _ => return Err(()),
                    }
                }
            };
            for step in 1..=time {
                emitted.push((s.r + step * dr, s.c + step * dc, l.hue));
            }
        }
    }
    for (r, c, h) in emitted {
        if let Some(cell) = usize::try_from(r).ok().zip(usize::try_from(c).ok()).and_then(|(ur, uc)| out.get_mut(ur)?.get_mut(uc)) {
            *cell = h;
        }
    }
    Ok(Some(out))
}

/// HR5 — RECONSTRUCT A FAMILY IN TIME, each pair left out in turn: the axes
/// whose loops close in time on the others, the most compact carrying it.
pub fn reconstruct_in_time(family: &[(Grid, Grid)]) -> InTime {
    let mut read = InTime { pairs: family.len() as u64, ..InTime::default() };
    let fields: Vec<Option<(Vec<Site>, Vec<Flow>)>> = family.iter().map(|(x, y)| flows(x, y).map(|f| (field(x), f))).collect();
    let all: Vec<(&[Site], &[Flow])> = fields.iter().flatten().map(|(s, f)| (s.as_slice(), f.as_slice())).collect();
    read.holonomy = holonomy_in_time(&all);
    for (i, f) in fields.iter().enumerate() {
        let Some((sites, _)) = f else { continue };
        read.framed += 1;
        let others: Vec<(&[Site], &[Flow])> =
            fields.iter().enumerate().filter(|(j, _)| *j != i).filter_map(|(_, g)| g.as_ref()).map(|(s, f)| (s.as_slice(), f.as_slice())).collect();
        let mut best: Option<(u64, usize, Keyed<Held>)> = None;
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
        match carried(&family[i].0, sites, &t, axis) {
            Err(()) => read.time_unread += 1,
            Ok(None) => read.unseen += 1,
            Ok(Some(g)) => {
                if g == family[i].1 {
                    read.reproduced += 1;
                    read.by_axis[axis] += 1;
                }
            }
        }
    }
    read.whole = read.pairs > 0 && read.reproduced == read.pairs;
    read
}
