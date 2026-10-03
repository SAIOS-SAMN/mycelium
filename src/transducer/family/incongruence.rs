// 78.79 — THE INTERPRETER FOR INCONGRUENCE (an instrument, deciding nothing):
// two bodies no motion makes the same are related by what they are made of —
// the constructors they share, those accreted, those extinguished — and a
// subject is defined by holding its readings together and contrasting them,
// never by dissolving them into one.
//
//! 1. THE CONSTRUCTOR DIFFERENCE (one change). An output body standing on
//!    exactly one input body (78.78's single carriage, 62% / 91% of what is
//!    left unsaid) overlaps it, so its core motion is the identity in place.
//!    Its constructors are its compartments' rounds, matched exactly by their
//!    coordinates (keyed, never searched): SHARED (the core), ACCRETED (the
//!    white hole's channel — growth, bridge), EXTINGUISHED (the black hole's —
//!    carving, hollowing), and the shared ones RECOLOURED (the hue fibre). The
//!    accretion is read as light (on a straight run out of the input body,
//!    78.77) and the extinction as carving (interior to the input body).
//! 2. THE CONTRAST PAIR. Two readings of one subject held together, each
//!    whole: what they share is DRAG (coherence), what differs LIFT (the
//!    contrast); neither absorbs the other.
//! 3. THE COHERENCE MANIFOLD. Per subject — the input bodies of one chart
//!    across a family's pairs — every reading held, written relative to its
//!    input body's exact barycentre. Its CORE is what every reading shares;
//!    its EDGES what some readings hold and others do not. Three readings of
//!    its sharpness, kept apart:
//!    - RESOLUTION: the quantum of its light — the greatest common length of
//!      the accretion's runs, refined as readings arrive (F4's law);
//!    - PHASE: luminal (open — its runs leave by the frame's edge) or
//!      holonomic (closed — they end at a body), per reading;
//!    - EDGES: the constructors that contrast.
//!
//!    Each new reading is counted by what it moved: refined, closed,
//!    sharpened an edge, or recognized.
//! 4. JUDGED BY THE PLANE: the core difference replayed on every reading's
//!    input body — reproduces its output, or does not.

use super::*;
use num_integer::Integer;
use num_rational::BigRational;
use num_traits::Zero;

type Q = BigRational;

/// One compartment: its site and hue.
pub type Cell = (i64, i64, u8);

/// ONE CHANGE, BY ITS CONSTRUCTORS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Difference {
    pub shared: u64,
    pub accreted: Vec<Cell>,
    pub extinguished: Vec<Cell>,
    /// Shared sites whose hue changed: (site, from, to).
    pub recoloured: Vec<(i64, i64, u8, u8)>,
    /// Accreted sites on a straight run out of the input body, and the runs'
    /// lengths with how each ended: at the frame's edge (open) or at a body.
    pub on_light: u64,
    pub runs: Vec<(u64, bool)>,
    /// Extinguished sites interior to the input body.
    pub carved: u64,
}

impl Difference {
    /// No constructor accreted or extinguished: the change is kinematic (here,
    /// in place: a recolour or nothing).
    pub fn kinematic(&self) -> bool {
        self.accreted.is_empty() && self.extinguished.is_empty()
    }
}

/// THE CONSTRUCTOR DIFFERENCE between an input body and the output body that
/// stands on it, in place (the core motion the identity).
pub fn difference(input: &[Cell], output: &[Cell], frame: &Grid) -> Difference {
    let at = |cells: &[Cell], r: i64, c: i64| cells.iter().find(|x| x.0 == r && x.1 == c).map(|x| x.2);
    let mut d = Difference { shared: 0, accreted: Vec::new(), extinguished: Vec::new(), recoloured: Vec::new(), on_light: 0, runs: Vec::new(), carved: 0 };
    for &(r, c, h) in output {
        match at(input, r, c) {
            Some(g) => {
                d.shared += 1;
                if g != h {
                    d.recoloured.push((r, c, g, h));
                }
            }
            None => d.accreted.push((r, c, h)),
        }
    }
    for &(r, c, h) in input {
        if at(output, r, c).is_none() {
            d.extinguished.push((r, c, h));
            let interior = [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().all(|(dr, dc)| at(input, r + dr, c + dc).is_some());
            d.carved += u64::from(interior);
        }
    }
    // The accretion as light: from each input cell along each of the eight
    // null directions, the run of accreted cells starting next to it.
    let mut lit: Vec<(i64, i64)> = Vec::new();
    for &(r, c, _) in input {
        for (dr, dc) in super::luminal::DIRECTIONS {
            let mut t = 1i64;
            while d.accreted.iter().any(|x| x.0 == r + t * dr && x.1 == c + t * dc) {
                t += 1;
            }
            if t > 1 {
                for k in 1..t {
                    let s = (r + k * dr, c + k * dc);
                    if !lit.contains(&s) {
                        lit.push(s);
                    }
                }
                let (er, ec) = (r + t * dr, c + t * dc);
                let open = usize::try_from(er).ok().zip(usize::try_from(ec).ok()).and_then(|(ur, uc)| frame.get(ur)?.get(uc)).is_none();
                d.runs.push(((t - 1) as u64, open));
            }
        }
    }
    d.on_light = lit.len() as u64;
    d
}

fn barycentre(cells: &[Cell]) -> Option<(Q, Q)> {
    let n = Q::from_integer(i64::try_from(cells.len()).ok()?.into());
    if n.is_zero() {
        return None;
    }
    let (r, c) = cells.iter().fold((Q::zero(), Q::zero()), |(a, b), x| (a + Q::from_integer(x.0.into()), b + Q::from_integer(x.1.into())));
    Some((r / &n, c / n))
}

/// A difference written relative to its input body's barycentre: each
/// accreted and extinguished constructor by its exact offset and hue, the
/// recolours by their hue pair — the reading a subject's readings compare by.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Relative {
    pub accreted: Vec<(Q, Q, u8)>,
    pub extinguished: Vec<(Q, Q, u8)>,
    pub recolours: Vec<(u8, u8)>,
}

pub fn relative(input: &[Cell], d: &Difference) -> Option<Relative> {
    let (br, bc) = barycentre(input)?;
    let off = |x: &Cell| (Q::from_integer(x.0.into()) - &br, Q::from_integer(x.1.into()) - &bc, x.2);
    let mut accreted: Vec<(Q, Q, u8)> = d.accreted.iter().map(off).collect();
    let mut extinguished: Vec<(Q, Q, u8)> = d.extinguished.iter().map(off).collect();
    let mut recolours: Vec<(u8, u8)> = d.recoloured.iter().map(|x| (x.2, x.3)).collect();
    accreted.sort();
    extinguished.sort();
    recolours.sort_unstable();
    recolours.dedup();
    Some(Relative { accreted, extinguished, recolours })
}

/// 2 — TWO READINGS HELD TOGETHER: what they share (drag, coherence) and what
/// each holds that the other does not (lift, contrast) — both kept whole.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Contrast {
    pub shared: u64,
    pub only_first: u64,
    pub only_second: u64,
}

pub fn contrast(a: &Relative, b: &Relative) -> Contrast {
    let all_a: Vec<&(Q, Q, u8)> = a.accreted.iter().chain(&a.extinguished).collect();
    let all_b: Vec<&(Q, Q, u8)> = b.accreted.iter().chain(&b.extinguished).collect();
    let shared = all_a.iter().filter(|x| all_b.contains(x)).count() as u64;
    Contrast { shared, only_first: all_a.len() as u64 - shared, only_second: all_b.len() as u64 - shared }
}

/// What one reading moved when it joined its subject's manifold.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Moved {
    pub refined: bool,
    pub closed: bool,
    pub sharpened: bool,
    pub recognized: bool,
}

/// 3 — ONE SUBJECT'S COHERENCE MANIFOLD: its readings held, its core and its
/// edges, its resolution and phase, and what each reading moved.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Manifold {
    pub readings: Vec<Relative>,
    /// The constructors every reading shares (accreted and extinguished), and
    /// the recolours every reading makes.
    pub core: Vec<(Q, Q, u8)>,
    pub core_recolours: Vec<(u8, u8)>,
    /// Constructors some readings hold and others do not.
    pub edges: u64,
    /// The quantum of its light: the gcd of every accretion run's length.
    pub quantum: Option<u64>,
    /// Readings whose runs all end at a body (holonomic), and those with a run
    /// leaving by the edge (luminal).
    pub holonomic: u64,
    pub luminal: u64,
    pub moved: Vec<Moved>,
}

/// BUILD A SUBJECT'S MANIFOLD from its readings in the order they came, each
/// with its runs.
pub fn manifold(readings: &[(Relative, Vec<(u64, bool)>)]) -> Manifold {
    let mut m = Manifold::default();
    for (rel, runs) in readings {
        let mut moved = Moved::default();
        let all = |r: &Relative| -> Vec<(Q, Q, u8)> { r.accreted.iter().chain(&r.extinguished).cloned().collect() };
        let mine = all(rel);
        moved.recognized = m.readings.iter().any(|h| h == rel);
        // The core narrows to what this reading shares; anything it drops or
        // brings beyond the core is an edge sharpened.
        let core_before = m.core.len();
        match m.readings.is_empty() {
            true => {
                m.core = mine.clone();
                m.core_recolours = rel.recolours.clone();
            }
            false => {
                m.core.retain(|x| mine.contains(x));
                m.core_recolours.retain(|x| rel.recolours.contains(x));
            }
        }
        moved.sharpened = !m.readings.is_empty() && (m.core.len() < core_before || mine.len() > m.core.len());
        // Resolution: the quantum refines where this reading's runs bring a
        // length the held quantum does not divide.
        let before = m.quantum;
        for (len, _) in runs {
            m.quantum = Some(match m.quantum {
                None => *len,
                Some(q) => q.gcd(len),
            });
        }
        moved.refined = matches!((before, m.quantum), (Some(b), Some(a)) if a < b);
        // Phase: every run of this reading ends at a body — the light held —
        // or one leaves by the edge.
        match (runs.is_empty(), runs.iter().all(|(_, open)| !open)) {
            (true, _) => {}
            (false, true) => {
                m.holonomic += 1;
                moved.closed = m.holonomic == 1;
            }
            (false, false) => m.luminal += 1,
        }
        m.readings.push(rel.clone());
        m.moved.push(moved);
    }
    let union: Vec<(Q, Q, u8)> = m.readings.iter().flat_map(|r| r.accreted.iter().chain(&r.extinguished).cloned()).collect();
    let mut distinct = union.clone();
    distinct.sort();
    distinct.dedup();
    m.edges = distinct.iter().filter(|x| !m.core.contains(x)).count() as u64;
    m
}

/// 4 — JUDGED BY THE PLANE: the manifold's core replayed on one reading's
/// input body — its core accretion laid down, its core extinction removed,
/// its core recolours applied — against the output body. Exact.
pub fn reproduces(m: &Manifold, input: &[Cell], output: &[Cell]) -> Option<bool> {
    let (br, bc) = barycentre(input)?;
    let mut spoken: Vec<Cell> = input.to_vec();
    for (dr, dc, h) in &m.core {
        let (r, c) = (&br + dr, &bc + dc);
        if !r.is_integer() || !c.is_integer() {
            return Some(false);
        }
        let (r, c) = (r.to_integer().try_into().ok()?, c.to_integer().try_into().ok()?);
        match spoken.iter().position(|x| x.0 == r && x.1 == c) {
            // An extinguished constructor is on the input: it leaves.
            Some(i) => {
                spoken.remove(i);
            }
            None => spoken.push((r, c, *h)),
        }
    }
    for x in spoken.iter_mut() {
        if let Some((_, to)) = m.core_recolours.iter().find(|(from, _)| *from == x.2) {
            x.2 = *to;
        }
    }
    let mut a = spoken;
    let mut b = output.to_vec();
    a.sort_unstable();
    b.sort_unstable();
    Some(a == b)
}
