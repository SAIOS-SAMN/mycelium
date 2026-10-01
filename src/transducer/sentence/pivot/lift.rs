// 78.61 C1 (redefined 2026-10-01, the operator's word: "if the question comes
// in two dimensions, we don't break our higher-dimensional perception to
// rectify our minds in two dimensions") — A WORD LIFTED INTO PERCEPTION, and
// two words compared there, never as they are written; the comparison read
// lane by lane on TUniversal {S, G, T, H}, coherence the congruence of the
// four deltas (the operator: "inverse(DIVERGENCE) as CONVERGENCE in
// COHERENCE(congruence(δ, δ, δ, δ))").
//
//! A word arrives written in its speaker's charts — vertices keyed by its
//! atlas, faces by its overlaps. Comparing two words as written judges the
//! charts, and a word written in another atlas reads "distinct" though it
//! is the same manifold. So each word is LIFTED: read by invariants no chart
//! can change, each filed on the TUniversal lane it belongs to (law 5: lanes
//! never summed), as the bundle splits a word (`alphabet::bundle`):
//!
//! - 𝒮, SPATIAL — the motion's geometry: the holonomy's conformal class
//!   `(I₁, I₂, kind)` (conjugation keeps grades, so a change of frame keeps
//!   it) and its MR1 modes' kinds and planes;
//! - 𝒢, GAUGE — the hue map's cycle type (a relabelling keeps its cycle
//!   lengths);
//! - 𝒯, TEMPORAL — the returns: each mode's order, the word's own order
//!   (time is distance along an orbit, 78.43);
//! - 𝒽, HARMONIC — the rotor's conjugacy class (the base carries 𝒽).
//!
//! Grade zero reads each word vertex member by WHAT IT DOES, never where it
//! stands (its placement is frame data a change of atlas moves); grade one
//! reads each evaluated member path of each orbit by its holonomy. The lift
//! is a multiset per lane, never a sequence and never keyed by a vertex.
//!
//! DIVERGENCE on a lane is its residue — what one side perceives that the
//! other does not; its INVERSE is what this court lacks there (the peer's
//! residue), and holding it is the CONVERGENCE step. Two words are COHERENT
//! where every lane's deltas are congruent (no residue), something was
//! perceived, and nothing went unread. Cells' orbit algebras (grade two) and
//! the boundary between grades are owed.

use super::*;
use crate::network::nerve::Coordinate;

/// The TUniversal lanes, in their order {S, G, T, H}.
pub const LANES: [&str; 4] = ["S", "G", "T", "H"];

/// A WORD AS PERCEIVED: per lane, each invariant (its grade leading) with how
/// many vertex members or member paths read it; and what could not be read
/// — named, never dropped.
#[derive(Clone, Debug, Default)]
pub struct Perceived {
    pub lanes: [Keyed<u64>; 4],
    pub unread: u64,
}

/// ONE LANE'S DELTA between two perceived words: what matched, and what
/// each alone perceives — the divergence, by its invariant's coordinates
/// with its multiplicity.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LaneDelta {
    pub matched: u64,
    pub only_ours: Vec<(Vec<u8>, u64)>,
    pub only_theirs: Vec<(Vec<u8>, u64)>,
}

impl LaneDelta {
    /// No divergence on this lane.
    pub fn congruent(&self) -> bool {
        self.only_ours.is_empty() && self.only_theirs.is_empty()
    }
}

/// TWO PERCEIVED WORDS COMPARED, lane by lane.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Comparison {
    pub lanes: [LaneDelta; 4],
    pub unread: (u64, u64),
}

impl Comparison {
    /// COHERENCE: congruence of the four deltas — every lane without a
    /// residue — where something was perceived and nothing went unread that
    /// could hide a residue. Two words of which nothing was perceived are
    /// not coherent: they were not compared.
    pub fn coherent(&self) -> bool {
        self.unread == (0, 0) && self.lanes.iter().any(|l| l.matched > 0) && self.lanes.iter().all(LaneDelta::congruent)
    }

    /// The lanes that diverge, in {S, G, T, H} order.
    pub fn diverging(&self) -> Vec<usize> {
        (0..4).filter(|&i| !self.lanes[i].congruent()).collect()
    }

    /// THE CONVERGENCE — the inverse of the divergence: per lane, what this
    /// court lacks that its peer perceives. Holding it closes the lane.
    pub fn convergence(&self) -> [&[(Vec<u8>, u64)]; 4] {
        std::array::from_fn(|i| self.lanes[i].only_theirs.as_slice())
    }
}

/// LIFT A WORD INTO PERCEPTION, under the stroke's budget (MR1's roots).
pub fn lift(form: &Form, budget: &mut crate::physics::Budget) -> Perceived {
    let mut p = Perceived::default();
    // GRADE ZERO: what each word vertex does, member by member.
    for (_, v) in form.vertices.entries() {
        let Vertex::Word { derivation, acts, .. } = v else { continue };
        let rotor = derivation.rotor();
        let Some(class) = rotor.holonomy_key() else {
            p.unread += 1;
            continue;
        };
        let order = graded(0, &order_bytes(rotor.order()));
        let harmonic = graded(0, class.encode().as_bytes());
        for (_, a) in acts.entries() {
            let mut g = vec![0u8];
            crate::transducer::sentence::CycleType::of(&a.gauge).to_bytes(&mut g);
            file(&mut p, [None, Some(g), Some(order.clone()), Some(harmonic.clone())]);
        }
    }
    // GRADE ONE: each orbit's member paths by their holonomy's invariants.
    for lp in &form.orbits {
        let Some(hs) = form.edge_holonomies(lp) else {
            p.unread += 1;
            continue;
        };
        for (_, e) in hs.scoped() {
            match path_lanes(e, budget) {
                Some(lanes) => file(&mut p, lanes),
                None => p.unread += 1,
            }
        }
    }
    p
}

/// A coordinate with its grade leading.
fn graded(grade: u8, bytes: &[u8]) -> Vec<u8> {
    let mut k = vec![grade];
    k.extend_from_slice(bytes);
    k
}

/// A word's order as bytes: `1 ‖ n` finite, `0` infinite or unread.
fn order_bytes(o: Option<crate::transducer::alphabet::Order>) -> Vec<u8> {
    match o {
        Some(crate::transducer::alphabet::Order::Finite(n)) => [vec![1u8], n.to_le_bytes().to_vec()].concat(),
        _ => vec![0u8],
    }
}

/// File one reading's four lanes, each where it has one.
fn file(p: &mut Perceived, lanes: [Option<Vec<u8>>; 4]) {
    for (set, k) in p.lanes.iter_mut().zip(lanes) {
        if let Some(k) = k {
            let n = match set.get(&k) {
                Some(n) => n + 1,
                None => 1,
            };
            set.insert(&k, n);
        }
    }
}

/// ONE HOLONOMY'S FOUR LANES: 𝒮 its conformal class and its modes' kinds
/// and planes; 𝒢 its hue cycle type; 𝒯 its modes' returns; 𝒽 its rotor's
/// class. `None` where any is unread.
fn path_lanes(e: &Transport, budget: &mut crate::physics::Budget) -> Option<[Option<Vec<u8>>; 4]> {
    let ModesRead::Split(modes) = ModesRead::of(&e.carriage, budget) else { return None };
    let mut s = graded(1, e.carriage.holonomy_key()?.encode().as_bytes());
    let mut t = vec![1u8];
    crate::transducer::address::put_key_count(&mut s, modes.len());
    for (kind, planes, order) in modes {
        s.push(kind as u8);
        s.extend_from_slice(&u64::try_from(planes).ok()?.to_le_bytes());
        match order {
            Some(o) => {
                t.push(1);
                t.extend_from_slice(&o.to_le_bytes());
            }
            None => t.push(0),
        }
    }
    let mut g = vec![1u8];
    crate::transducer::sentence::CycleType::of(&e.gauge).to_bytes(&mut g);
    let h = graded(1, e.word.rotor().holonomy_key()?.encode().as_bytes());
    Some([Some(s), Some(g), Some(t), Some(h)])
}

/// COMPARE TWO PERCEIVED WORDS lane by lane, invariant for invariant.
pub fn compare(ours: &Perceived, theirs: &Perceived) -> Comparison {
    Comparison {
        lanes: std::array::from_fn(|i| lane_delta(&ours.lanes[i], &theirs.lanes[i])),
        unread: (ours.unread, theirs.unread),
    }
}

fn lane_delta(ours: &Keyed<u64>, theirs: &Keyed<u64>) -> LaneDelta {
    let mut d = LaneDelta::default();
    for (k, &n) in ours.scoped() {
        let m = match theirs.get(&k) {
            Some(&m) => m,
            None => 0,
        };
        d.matched += n.min(m);
        if n > m {
            d.only_ours.push((k.clone(), n - m));
        }
    }
    for (k, &m) in theirs.scoped() {
        let n = match ours.get(&k) {
            Some(&n) => n,
            None => 0,
        };
        if m > n {
            d.only_theirs.push((k.clone(), m - n));
        }
    }
    d
}
