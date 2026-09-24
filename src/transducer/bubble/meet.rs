// 78.37(1), 78.26(6), TP1 — THE MEET GRAPH OF THE HELD THOUGHTS, READ: its
// thoughts, edges and rings by fill, its cycle rank, and what cut a coherent
// thought in two.
//
//! The contemplation's complex IS the meet graph: a vertex per held thought,
//! an edge where two thoughts share a chart — the edge's rotor the gauge
//! that carries one section to the other on the meet (13.12) — and a ring
//! where three thoughts share remainder volume. A THOUGHT is a component over
//! the FILLED edges; the meet graph's own components run over every edge.
//!
//! The state — thoughts, edges, rings, the cycle rank — is read at the
//! publish. What changed it — merges (`MergeTally`) and decoherences — is
//! counted at its producer since the contemplation began and never taken:
//! the delta between two publishes is the reader's (§6.5).
//!
//! OWED to TP1's fix, by name and never printed as zero: the fronts
//! transported (edges per k-step), the rings collapsed by transport, the
//! candidates classified by disagreement with the transported section, and
//! a decoherence by a ruling (no contemplation site cuts on the dialect
//! chain today). [`MEET_OWED`] is the one statement of them.

use super::*;

/// The fields TP1's reading names that no producer reads yet, each with what
/// owes it (law 12: one statement, read by every renderer).
pub const MEET_OWED: [(&str, &str); 4] = [
    ("fronts advanced", "TP1's fix: transport one edge per k-step"),
    ("rings collapsed by transport", "TP1's fix"),
    ("candidates classified by disagreement", "TP1's fix"),
    ("decohered by a ruling", "no contemplation site cuts on the dialect chain"),
];

/// WHAT CUT A COHERENT THOUGHT (78.26(6)): the gauge moved — a merged
/// thought re-linked with its re-gauged sections — or a new candidate was
/// held. A ruling demoting a citation is owed ([`MEET_OWED`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adjunct {
    Gauge,
    Candidate,
}

impl Adjunct {
    pub const ALL: [Adjunct; 2] = [Adjunct::Gauge, Adjunct::Candidate];

    pub fn index(self) -> usize {
        match self {
            Adjunct::Gauge => 0,
            Adjunct::Candidate => 1,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Adjunct::Gauge => "gauge",
            Adjunct::Candidate => "candidate",
        }
    }
}

/// 78.26(6) — THE DECOHERENCES, per adjunct, cumulative at the producer: a
/// FILLED ring reopened (the thought cut); of those since read again, the
/// ones that left a class between the two thoughts, and the ones that
/// re-filled (rejoined). Never a deletion and never a forced closure.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Decoherence {
    pub decohered: [u64; 2],
    pub classed: [u64; 2],
    pub rejoined: [u64; 2],
}

/// A fill's arm in the reading's own order: filled (holonomy 1), congruent,
/// held as a class, unspellable, unread, open (unasked).
fn arm(f: &Fill) -> usize {
    match f {
        Fill::Filled(..) => 0,
        Fill::Congruent(..) => 1,
        Fill::Class(..) => 2,
        Fill::Unspellable(_) => 3,
        Fill::Unread(_) => 4,
        Fill::Open => 5,
    }
}

/// The arms' names, in [`arm`]'s order.
pub const FILL_ARMS: [&str; 6] = ["filled", "congruent", "class", "unspellable", "unread", "open"];

/// 78.37(1), TP1 — THE MEET GRAPH, READ AT THE PUBLISH, and the decoherences
/// since the contemplation began.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MeetGraph {
    /// Held thoughts' vertices, `V`.
    pub vertices: u64,
    /// The THOUGHTS: components over the filled edges.
    pub thoughts: u64,
    /// The meet graph's own components over every edge, `C`.
    pub components: u64,
    /// Its edges by fill arm; `E` is their sum.
    pub edges: [u64; 6],
    /// `E − V + C`, the cycle space's rank, counted and never read off a tree.
    /// Signed and never clamped: negative only where an edge names a vertex
    /// that is no held thought, which is then a finding on the line.
    pub cycle_rank: i64,
    /// The rings — 2-simplices — by fill arm.
    pub rings: [u64; 6],
    /// The orbits filed at the rings' orbit keys, by fill arm.
    pub orbits: [u64; 6],
    pub decoherence: Decoherence,
}

impl MeetGraph {
    /// THE LINE: thoughts and components, edges and rings by arm, the cycle
    /// rank beside the rings (neither subtracted from the other), the
    /// decoherences per adjunct, and what is owed by name.
    pub fn line(&self) -> String {
        let by = |xs: &[u64; 6]| {
            FILL_ARMS.iter().zip(xs).map(|(n, x)| format!("{n} {x}")).collect::<Vec<_>>().join(" ")
        };
        let d = &self.decoherence;
        let adj = Adjunct::ALL
            .iter()
            .map(|a| {
                let i = a.index();
                format!("{} {} (classed {} rejoined {})", a.name(), d.decohered[i], d.classed[i], d.rejoined[i])
            })
            .collect::<Vec<_>>()
            .join(" ");
        let owed = MEET_OWED.iter().map(|(f, _)| *f).collect::<Vec<_>>().join(", ");
        format!(
            "meet graph: thoughts {} over {} vertices, components {} | edges {} | cycle rank {} | rings {} | ring orbits {} | decohered: {adj} | owed: {owed}",
            self.thoughts,
            self.vertices,
            self.components,
            by(&self.edges),
            self.cycle_rank,
            by(&self.rings),
            by(&self.orbits),
        )
    }
}

impl Contemplation {
    /// 78.37(1), TP1 — THE MEET GRAPH AS IT STANDS, with the decoherences
    /// since the contemplation began.
    pub fn meet_graph(&self) -> MeetGraph {
        let mut m = MeetGraph { decoherence: self.decoherence, ..MeetGraph::default() };
        m.vertices = self.bubbles.len() as u64;
        m.thoughts = self.component_count() as u64;
        // The meet graph's own components: every edge, whatever its fill.
        let mut seen: Keyed<()> = Keyed::new();
        for (k, _) in self.bubbles.scoped() {
            if seen.contains(&k) {
                continue;
            }
            m.components += 1;
            let mut stack = vec![k.clone()];
            seen.insert(&k, ());
            while let Some(v) = stack.pop() {
                for ek in self.complex.link(&v, 1) {
                    let Some(s) = self.complex.simplex(&ek) else { continue };
                    for u in &s.vertices {
                        if !seen.contains(u) {
                            seen.insert(u, ());
                            stack.push(u.clone());
                        }
                    }
                }
            }
        }
        if let Some(layer) = self.complex.degrees.get(1) {
            for (_, s) in layer.scoped() {
                m.edges[arm(&s.fill)] += 1;
            }
        }
        if let Some(layer) = self.complex.degrees.get(2) {
            for (_, s) in layer.scoped() {
                m.rings[arm(&s.fill)] += 1;
            }
        }
        for (_, f) in self.complex.orbits.scoped() {
            m.orbits[arm(f)] += 1;
        }
        // `E − V + C`: non-negative for a graph, since every component of `n`
        // vertices holds at least `n − 1` edges; carried signed, never clamped.
        let e: u64 = m.edges.iter().sum();
        m.cycle_rank = e as i64 + m.components as i64 - m.vertices as i64;
        m
    }

    /// 78.26(6) — THE RINGS A LINK CUT OPEN, by the adjunct that cut them: each
    /// counted, and held pending by its own key until it is read again.
    pub(crate) fn note_decohered(&mut self, reopened: Vec<Vec<u8>>, adjunct: Adjunct) {
        for key in reopened {
            self.decoherence.decohered[adjunct.index()] += 1;
            self.decohering.insert(&key, adjunct);
        }
    }

    /// FILL A SIMPLEX OF THE MEET GRAPH, and settle a decoherence pending on
    /// it: a class read is the class the cut left between the two thoughts; a
    /// re-fill is the thought rejoined. Every other answer leaves it pending.
    pub(crate) fn fill_meet(&mut self, key: &[u8]) -> CoboundaryOutcome {
        let outcome = self.complex.fill(key);
        self.settle_decoherence(key, &outcome);
        outcome
    }

    /// A DECOHERENCE PENDING ON `key`, settled by whatever fill path read it
    /// again: a class is the class the cut left; a re-fill is the thought
    /// rejoined; every other answer leaves it pending.
    pub(crate) fn settle_decoherence(&mut self, key: &[u8], outcome: &CoboundaryOutcome) {
        let Some(&adjunct) = self.decohering.get(key) else { return };
        match outcome {
            CoboundaryOutcome::Attached(Fill::Class(..)) => {
                self.decoherence.classed[adjunct.index()] += 1;
                self.decohering.remove(key);
            }
            CoboundaryOutcome::Attached(Fill::Filled(..)) => {
                self.decoherence.rejoined[adjunct.index()] += 1;
                self.decohering.remove(key);
            }
            // Congruent, unspellable, unread, open, already, absent: not yet
            // a class nor a rejoining; it stays pending.
            _ => {}
        }
    }
}
