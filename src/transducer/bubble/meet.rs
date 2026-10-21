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
//! The transport's fronts ride the same reading (78.37(1), `transport.rs`).
//! OWED by name and never printed as zero: a decoherence by a ruling (no
//! contemplation site cuts on the dialect chain today). [`MEET_OWED`] is the
//! one statement of it.

use super::*;

/// The fields TP1's reading names that no producer reads yet, each with what
/// owes it (law 12: one statement, read by every renderer).
pub const MEET_OWED: [(&str, &str); 2] = [
    ("decohered by a ruling", "no contemplation site cuts on the dialect chain"),
    ("docket demands taken", "CV4/SP2: a demand flows upward when minds speak to minds and the court hears an offer"),
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

/// 78.26(6) — THE DECOHERENCES, cumulative at the producer, each indexed by
/// what the reopen cut ([`cut_index`]: a ring filled at holonomy 1, or one
/// closed at its period, counted apart) and then by adjunct: the rings cut;
/// of those since read again, the ones that left a class between the two
/// thoughts, and the ones that closed again (rejoined). Never a deletion and
/// never a forced closure.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Decoherence {
    pub decohered: [[u64; 2]; 2],
    pub classed: [[u64; 2]; 2],
    pub rejoined: [[u64; 2]; 2],
}

/// What a reopen cut, as the decoherence counts' first index.
pub fn cut_index(cut: crate::transducer::cech::Cut) -> usize {
    match cut {
        crate::transducer::cech::Cut::Filled => 0,
        crate::transducer::cech::Cut::Congruent => 1,
    }
}

/// The cuts in [`cut_index`]'s order, by name.
pub const CUTS: [(&str, crate::transducer::cech::Cut); 2] =
    [("filled", crate::transducer::cech::Cut::Filled), ("congruent", crate::transducer::cech::Cut::Congruent)];

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
#[derive(Clone, Debug, Default, PartialEq, Eq)]
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
    /// 78.37(1) — the fronts: arrivals at the last step's `K` (edges advanced
    /// this k-step), the sources standing, and what the transport did since
    /// the contemplation began.
    pub advanced: u64,
    pub sources: u64,
    pub transport: super::transport::TransportTally,
    /// 78.37 — the docket: pending this orbit, open for the next, owed
    /// enclosures, unwritten entries.
    pub docket: super::docket::DocketReading,
    /// TC1 — the nerve's shape (free vertices and faces, the 2-core, its
    /// `Ȟ¹`), and what each shedding door removed that carried a loop.
    pub core: super::core::CoreReading,
    pub shed: super::core::ShedTally,
    /// TC1's fix — what the truncation removed, refused and deferred.
    pub truncation: super::core::Truncation,
    /// RM1 — the mind's remainders: `Congruent` fills by `ℓ`, their classes
    /// and those standing across charts, the re-fills.
    pub remainders: crate::transducer::cech::RemainderReading,
    /// STD1 — lane sums the contemplation refused past their carrier.
    pub overflowed: u64,
    /// N4 — perception across scenes: the circuits read, the cells they
    /// cross and how they cohere, and the thoughts and demands filed.
    pub resonance: super::resonance::ResonanceTally,
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
        let adj = CUTS
            .iter()
            .map(|(cut_name, cut)| {
                let c = cut_index(*cut);
                let per = Adjunct::ALL
                    .iter()
                    .map(|a| {
                        let i = a.index();
                        format!(
                            "{} {} (classed {} rejoined {})",
                            a.name(),
                            d.decohered[c][i],
                            d.classed[c][i],
                            d.rejoined[c][i]
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("{cut_name}: {per}")
            })
            .collect::<Vec<_>>()
            .join("; ");
        let owed = MEET_OWED.iter().map(|(f, _)| *f).collect::<Vec<_>>().join(", ");
        let t = &self.transport;
        let c = &self.core;
        let doors = super::core::Door::ALL
            .iter()
            .map(|d| {
                let i = d.index();
                format!(
                    "{} removed {} in-core {} rank lost {}",
                    d.name(),
                    self.shed.removed[i],
                    self.shed.in_core[i],
                    self.shed.rank_lost[i]
                )
            })
            .collect::<Vec<_>>()
            .join("; ");
        let tr = &self.truncation;
        let core = format!(
            "nerve: Ȟ¹ {}, 2-core {}, free vertices {} isolated {}, free faces edges {} triangles {}, unfaced {} malformed {} | shed: {doors} | truncated {} (refused: held by class {} glued {}; Ȟ¹ moved {} malformed {})",
            c.h1, c.core, c.free_vertices, c.isolated, c.free_faces[0], c.free_faces[1], c.unfaced, c.malformed,
            tr.truncated, tr.held_by_class, tr.glued, tr.rank_moved, tr.malformed
        );
        let core = format!(
            "{core} | {} | overflowed {} | {}",
            self.remainders.line(),
            self.overflowed,
            self.resonance.line()
        );
        format!(
            "meet graph: thoughts {} over {} vertices, components {} | edges {} | cycle rank {} | rings {} | ring orbits {} | decohered: {adj} | fronts: sources {} advanced {} this k-step; fixed by fill {} by class {}; rings collapsed {} (filled {}) held {}; candidates classified {}; unsectioned {} ungauged {} unclassed {} | docket: pending {} open {} (enclosure owed {}) unwritten {} answered {} taken {} lapsed {} | {core} | owed: {owed}",
            self.thoughts,
            self.vertices,
            self.components,
            by(&self.edges),
            self.cycle_rank,
            by(&self.rings),
            by(&self.orbits),
            self.sources,
            self.advanced,
            t.fixed[0],
            t.fixed[1],
            t.collapsed,
            t.collapsed_filled,
            t.held,
            t.classified,
            t.unsectioned,
            t.ungauged,
            t.unclassed,
            self.docket.pending,
            self.docket.open,
            self.docket.owed,
            self.docket.unwritten,
            self.docket.answered,
            self.docket.taken,
            self.docket.lapsed,
        )
    }
}

impl Contemplation {
    /// RM1 — THE MIND'S REMAINDERS: the contemplation's `Congruent` fills,
    /// each at the chart its thoughts share. A cover is the chain of an
    /// invariant's prefixes, so the intersection of the thoughts' covers is
    /// a chain, fixed by its deepest member — the chart the edge was made
    /// from; the chain itself, its keys written in order, is that chart's
    /// coordinate, and nothing is chosen from it. Unplaced where they share
    /// none.
    pub fn remainder_reading(&self) -> crate::transducer::cech::RemainderReading {
        let mut count = crate::transducer::cech::RemainderCount::default();
        for s in self.complex.congruent_standing() {
            let chain = self.shared_chain(&s.vertices).map(|ks| {
                let mut key = Vec::new();
                for k in &ks {
                    crate::transducer::address::put_key_bytes(&mut key, k);
                }
                key
            });
            count.add(&s, chain.as_deref());
        }
        count.refilled(self.complex.refilled());
        let r = &self.remainders;
        crate::transducer::cech::RemainderReading {
            refused: r.refused,
            past_carrier: r.past_carrier,
            misspoken: r.misspoken,
            unsited: r.unsited,
            unrecorded: r.unrecorded,
            unminted: r.unminted,
            unmerged: r.unmerged,
            ..count.read()
        }
    }

    /// THE CHART CHAIN THOUGHTS SHARE: the intersection of their covers, in
    /// key order — the chart a residue on their simplex is perceived on.
    /// `None` where they share none.
    pub(crate) fn shared_chain(&self, verts: &[Vec<u8>]) -> Option<Vec<Vec<u8>>> {
        let mut shared: Option<Vec<Vec<u8>>> = None;
        for v in verts {
            let cover: Vec<Vec<u8>> = match self.bubbles.get(v) {
                Some(b) => b.cover.scoped().map(|(k, _)| k).collect(),
                None => Vec::new(),
            };
            shared = Some(match shared {
                None => cover,
                Some(prev) => prev.into_iter().filter(|k| cover.contains(k)).collect(),
            });
        }
        shared.filter(|ks| !ks.is_empty())
    }

    /// 78.37(1), TP1 — THE MEET GRAPH AS IT STANDS, with the decoherences
    /// since the contemplation began.
    pub fn meet_graph(&self) -> MeetGraph {
        let (advanced, sources, transport) = self.transport_reading();
        let mut m = MeetGraph {
            decoherence: self.decoherence,
            advanced,
            sources,
            transport,
            docket: self.docket_reading(),
            core: self.core_reading(),
            shed: self.shed,
            truncation: self.truncation,
            remainders: self.remainder_reading(),
            overflowed: self.overflowed,
            resonance: self.resonance,
            ..MeetGraph::default()
        };
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
    pub(crate) fn note_decohered(&mut self, reopened: Vec<(Vec<u8>, crate::transducer::cech::Cut)>, adjunct: Adjunct) {
        for (key, cut) in reopened {
            self.decoherence.decohered[cut_index(cut)][adjunct.index()] += 1;
            self.decohering.insert(&key, (adjunct, cut));
        }
    }

    /// FILL A SIMPLEX OF THE MEET GRAPH, and settle a decoherence pending on
    /// it: a class read is the class the cut left between the two thoughts; a
    /// re-fill is the thought rejoined. Every other answer leaves it pending.
    /// A cell of degree two or more ASCENDS (MU-CLIENT §4.5), and so does a
    /// hole the cross left there — its read-back, the seal — and a class the
    /// vacuum holds there — its thaw; a tether is read by its own fill.
    pub(crate) fn fill_meet(&mut self, key: &[u8]) -> CoboundaryOutcome {
        let outcome = match self.complex.degree_of(key) {
            Some(d) if d >= 2 => self.ascend(key),
            None if self.complex.is_hole(key) || self.complex.is_frozen(key) => self.ascend(key),
            _ => self.complex.fill(key),
        };
        self.settle_decoherence(key, &outcome);
        outcome
    }

    /// A DECOHERENCE PENDING ON `key`, settled by whatever fill path read it
    /// again: a class is the class the cut left; a re-fill is the thought
    /// rejoined; every other answer leaves it pending.
    pub(crate) fn settle_decoherence(&mut self, key: &[u8], outcome: &CoboundaryOutcome) {
        // R1 — a simplex of a held ring re-filled: its docket entry answered.
        if matches!(outcome, CoboundaryOutcome::Attached(Fill::Filled(..))) {
            self.docket_ring_closed(key);
        }
        let Some(&(adjunct, cut)) = self.decohering.get(key) else { return };
        let (c, a) = (cut_index(cut), adjunct.index());
        // A class read is the class the cut left. The ring REJOINED where it
        // closes again the way it had closed: filled at holonomy 1, or at
        // its period for a ring cut from a congruence.
        let rejoined = matches!(
            (cut, outcome),
            (_, CoboundaryOutcome::Attached(Fill::Filled(..)))
                | (crate::transducer::cech::Cut::Congruent, CoboundaryOutcome::Attached(Fill::Congruent(..)))
        );
        match (outcome, rejoined) {
            (CoboundaryOutcome::Attached(Fill::Class(..)), _) => {
                self.decoherence.classed[c][a] += 1;
                self.decohering.remove(key);
            }
            (_, true) => {
                self.decoherence.rejoined[c][a] += 1;
                self.decohering.remove(key);
            }
            // Not yet a class nor a rejoining; it stays pending.
            _ => {}
        }
    }
}
