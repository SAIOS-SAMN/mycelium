// 78.37(1), TP1 — COLLAPSE BY TRANSPORT, STOPPED BY HOLONOMY: a fixed
// thought's section carried along the meet graph's gauges one edge per
// k-step, compared at every ring.
//
//! A thought is FIXED by fill — a component closes into a spoken thought — or
//! by class — a court-ruled word arrives on a chart the thought stands on.
//! Its section is then TRANSPORTED: at each of the voice's k-steps (its `K`,
//! never TUniversal, 45.4) every arrival from an earlier `K` walks every edge
//! it stands on except the one it came by, the edge's gauge carrying the
//! section to the far thought. One edge per `K`, never instantaneous (78.21).
//!
//! A far thought this front has not reached takes the arrival — and where
//! its own section disagrees with it, the disagreement is CLASSIFIED into
//! its horizon: a class, never garbage, never removed. A far thought the
//! front has already reached by another path closes a RING, and the two
//! arrivals are compared on both halves. Holonomy 1: the ring COLLAPSES — its
//! 2-simplex fills through the one fill path and 13.12's merge runs where the
//! component closes, as `decide_neighbors` runs it. Holonomy ≠ 1: the ring is
//! HELD, its class — the ray of the reverse product `B·Ã`, never an inverse
//! (78.32(1)) — written into both thoughts' horizons, and the front STOPS
//! there. No spanning tree: every edge is walked and every second arrival
//! compared. Nothing is deleted.

use super::*;

/// ONE ARRIVAL OF A FRONT: the section carried to `at`, the edge it came by
/// (empty at the source), the `K` it arrived at, whether it has walked on,
/// whether the front stops here (a held ring), and — for a thought fixed as
/// it leaves the meet graph — the edges it departs by, read before it left.
#[derive(Clone, Debug)]
pub(crate) struct Arrival {
    pub(crate) source: Vec<u8>,
    pub(crate) at: Vec<u8>,
    pub(crate) rotor: Rotor,
    pub(crate) carriage: element::Versor,
    pub(crate) by: Vec<u8>,
    /// The thought it came from (empty at the source): a ring's thoughts are
    /// read back along it.
    pub(crate) prev: Vec<u8>,
    pub(crate) k: u64,
    pub(crate) walked: bool,
    pub(crate) stopped: bool,
    pub(crate) departs: Option<Vec<Departure>>,
    /// The source's charts, read at the fix before it may leave the meet
    /// graph; `None` on every other arrival, whose thought is read live.
    pub(crate) cover: Option<Vec<Vec<u8>>>,
}

/// An edge a fixed thought departs by: the far thought, the edge's gauge
/// oriented from the thought, and the edge's own key.
pub(crate) type Departure = (Vec<u8>, Rotor, element::Versor, Vec<u8>);

/// 78.37(1) — WHAT THE TRANSPORT DID, cumulative at its producer since the
/// contemplation began: thoughts fixed (by fill, by class), rings collapsed
/// at holonomy 1 (and of those, the ones a 2-simplex spans and filled),
/// rings held with their class, candidates classified by disagreement; and
/// fixes that could not start, by why — the thought held no section, or an
/// edge of it has no oriented gauge.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TransportTally {
    pub fixed: [u64; 2],
    pub collapsed: u64,
    pub collapsed_filled: u64,
    pub held: u64,
    pub classified: u64,
    pub unsectioned: u64,
    pub ungauged: u64,
    /// A disagreement or a held ring whose class did not read (of_rotor
    /// refused: no conjugacy invariants).
    pub unclassed: u64,
}

/// The key of one arrival: the source, then the thought reached, each
/// framed — MU-CLIENT §4.13, compacted to its 32-byte hash (the
/// ruling of 2026-10-16: a hash is valid where nothing is lost): the arrival
/// holds its source and the thought reached itself, and every reader derives
/// the key from the two it holds.
fn arrival_key(source: &[u8], at: &[u8]) -> Vec<u8> {
    let mut k = Vec::new();
    crate::transducer::address::put_key_bytes(&mut k, source);
    crate::transducer::address::put_key_bytes(&mut k, at);
    crate::chronometry::payload_hash(&k).to_vec()
}

/// THE CLASS OF A DISAGREEMENT between two sections of one thought: the ray
/// of the reverse product `B·Ã` on both halves (78.32(1)), and its rotor.
/// Sections carry no hue, so the fibre's holonomy is the identity, measured.
fn disagreement(
    b: (&Rotor, &element::Versor),
    a: (&Rotor, &element::Versor),
) -> Option<(crate::transducer::sentence::ClassVector, Rotor)> {
    let r = b.0.geo(&a.0.reverse());
    let c = b.1.geo(&a.1.reverse());
    let kappa = crate::transducer::sentence::ClassVector::of_rotor(&r, c, crate::transducer::sentence::CycleType::Identity)?;
    Some((kappa, r))
}

impl Contemplation {
    /// THE VOICE'S K, stamped after its orbit closes (45.4): the one clock the
    /// transport steps on.
    pub fn set_k(&mut self, k: u64) {
        self.voice_k = k;
    }

    /// FIX A THOUGHT: its section starts a front at this `K`. Where the thought
    /// leaves the meet graph as it is fixed — a spoken component — its edges
    /// are read now, before it leaves, and the front departs by them.
    pub(crate) fn fix(&mut self, thought: &[u8], by_class: bool) {
        let Some(sec) = self.complex.sections.get(thought).cloned() else {
            self.transport.unsectioned += 1;
            return;
        };
        let mut departs: Vec<Departure> = Vec::new();
        for ek in self.complex.link(thought, 1) {
            let Some(s) = self.complex.simplex(&ek) else { continue };
            let Some(far) = s.vertices.iter().find(|v| v.as_slice() != thought).cloned() else { continue };
            match self.complex.edge(thought, &far) {
                Some((r, c)) => departs.push((far, r, c, ek)),
                None => self.transport.ungauged += 1,
            }
        }
        self.transport.fixed[usize::from(by_class)] += 1;
        let cover = self.bubbles.get(thought).map(|b| b.cover.scoped().map(|(k, _)| k).collect());
        let key = arrival_key(thought, thought);
        self.fronts.insert(
            &key,
            Arrival {
                source: thought.to_vec(),
                at: thought.to_vec(),
                rotor: sec.gamma,
                carriage: sec.carriage,
                by: Vec::new(),
                prev: Vec::new(),
                k: self.voice_k,
                walked: false,
                stopped: false,
                departs: Some(departs),
                cover,
            },
        );
    }

    /// A RING'S COVER: the charts its thoughts stand on, ascending, read back
    /// from the two arrivals that met along each one's path to the source —
    /// the source's as read at its fix. `None` where a thought's charts do
    /// not read (it left the meet graph and was not the source): the
    /// enclosure is owed on that entry.
    fn ring_cover(&self, source: &[u8], ends: [&[u8]; 2]) -> Option<Vec<Vec<u8>>> {
        let mut charts: Keyed<()> = Keyed::new();
        for end in ends {
            let mut at = end.to_vec();
            loop {
                let a = self.fronts.get(&arrival_key(source, &at))?;
                let read: Vec<Vec<u8>> = match (&a.cover, self.bubbles.get(&at)) {
                    (Some(c), _) => c.clone(),
                    (None, Some(b)) => b.cover.scoped().map(|(k, _)| k).collect(),
                    (None, None) => return None,
                };
                for c in read {
                    charts.insert(&c, ());
                }
                if a.prev.is_empty() {
                    break;
                }
                at = a.prev.clone();
            }
        }
        Some(charts.scoped().map(|(k, _)| k).collect())
    }

    /// FIX BY CLASS: a court-ruled word arrived; every thought standing on its
    /// chart, or on a prefix of it, is fixed.
    pub(crate) fn fix_by_class(&mut self, word: &Element) {
        let inv = stalk::invariant_of(word);
        let mut thoughts: Keyed<()> = Keyed::new();
        for p in inv.prefixes() {
            for (id, _) in self.by_chart.link(&p.key()) {
                if let Some(addr) = Contemplation::ordinal_of_key(&id).and_then(|o| self.address_at(o)) {
                    thoughts.insert(addr, ());
                }
            }
        }
        let keys: Vec<Vec<u8>> = thoughts.scoped().map(|(k, _)| k).collect();
        for k in keys {
            self.fix(&k, true);
        }
    }

    /// ONE K-STEP OF EVERY FRONT: nothing where `k` has not moved since the
    /// last step; otherwise each arrival from an earlier `K` that has not
    /// walked and does not stop walks every edge it stands on but the one it
    /// came by, exactly one edge.
    pub fn transport(&mut self, k: u64) {
        if self.stepped_k == Some(k) {
            return;
        }
        self.stepped_k = Some(k);
        let ready: Vec<(Vec<u8>, Arrival)> = self
            .fronts
            .scoped()
            .filter(|(_, a)| a.k < k && !a.walked && !a.stopped)
            .map(|(key, a)| (key, a.clone()))
            .collect();
        for (key, from) in ready {
            if let Some(a) = self.fronts.get_mut(&key) {
                a.walked = true;
            }
            let edges: Vec<Departure> = match &from.departs {
                Some(d) => d.clone(),
                None => {
                    let mut out = Vec::new();
                    for ek in self.complex.link(&from.at, 1) {
                        let Some(s) = self.complex.simplex(&ek) else { continue };
                        let Some(far) = s.vertices.iter().find(|v| **v != from.at).cloned() else { continue };
                        match self.complex.edge(&from.at, &far) {
                            Some((r, c)) => out.push((far, r, c, ek)),
                            None => self.transport.ungauged += 1,
                        }
                    }
                    out
                }
            };
            for (far, r, c, ek) in edges {
                if ek == from.by {
                    continue;
                }
                let (rotor, carriage) = (r.geo(&from.rotor), c.geo(&from.carriage));
                let fkey = arrival_key(&from.source, &far);
                match self.fronts.get(&fkey).cloned() {
                    None => {
                        self.classify_arrival(&far, (&rotor, &carriage));
                        self.fronts.insert(
                            &fkey,
                            Arrival {
                                source: from.source.clone(),
                                at: far,
                                rotor,
                                carriage,
                                by: ek,
                                prev: from.at.clone(),
                                k,
                                walked: false,
                                stopped: false,
                                departs: None,
                                cover: None,
                            },
                        );
                    }
                    // A RING: the far thought reached by another path — closed
                    // once per front per edge, though both its ends walk it.
                    Some(there) => {
                        let ring = arrival_key(&from.source, &ek);
                        if self.ring_edges.contains(&ring) {
                            continue;
                        }
                        self.ring_edges.insert(&ring, ());
                        self.close_ring(&from.at, &ek, &fkey, &there, (&rotor, &carriage));
                    }
                }
            }
        }
    }

    /// A NEW ARRIVAL AGAINST THE THOUGHT'S OWN SECTION: a disagreement is a
    /// class, written into the thought's horizon and counted.
    fn classify_arrival(&mut self, at: &[u8], arrived: (&Rotor, &element::Versor)) {
        let Some(own) = self.complex.sections.get(at).cloned() else {
            self.transport.unsectioned += 1;
            return;
        };
        let agrees = own.gamma.projective_eq(arrived.0) && element::same_versor(&own.carriage, arrived.1);
        if agrees {
            return;
        }
        match disagreement((&own.gamma, &own.carriage), arrived) {
            Some((_, r)) => {
                self.transport.classified += 1;
                if let Some(b) = self.bubbles.get_mut(at) {
                    horizon_put(&mut b.horizon, r, Standing::Held);
                }
            }
            None => self.transport.unclassed += 1,
        }
    }

    /// TWO ARRIVALS AT ONE THOUGHT close a ring. Holonomy 1 collapses it:
    /// each 2-simplex on the closing edge fills through the one fill path,
    /// and the merge runs where the component closes. Holonomy ≠ 1 holds its
    /// class in both thoughts' horizons and stops the front there.
    fn close_ring(
        &mut self,
        near: &[u8],
        edge: &[u8],
        there_key: &[u8],
        there: &Arrival,
        arrived: (&Rotor, &element::Versor),
    ) {
        let flat = there.rotor.projective_eq(arrived.0) && element::same_versor(&there.carriage, arrived.1);
        let on_ring = |rk: &Vec<u8>| self.complex.vertices_at(rk).is_some_and(|vs| vs.iter().any(|v| v.as_slice() == near));
        let rings: Vec<Vec<u8>> = self.complex.link(&there.at, 2).into_iter().filter(on_ring).collect();
        // 62.1 — the questions punctured over this ring before, standing as
        // holes in the complex.
        let holes: Vec<Vec<u8>> = self.complex.holes_on(&there.at).into_iter().filter(on_ring).collect();
        if flat {
            self.transport.collapsed += 1;
            // R1 — a ring held before that collapses now answers its entry.
            self.docket_ring_closed(edge);
            for rk in rings {
                if matches!(self.fill_meet(&rk), CoboundaryOutcome::Attached(Fill::Filled(..))) {
                    self.transport.collapsed_filled += 1;
                }
            }
            // 62.1 — SEAL: the arrivals now agree around the ring, so each
            // hole it holds is capped over its standing boundary and the
            // class it kept is killed — the question comprehended.
            for hole in holes {
                if let crate::transducer::cech::Capped::Sealed { cap: Fill::Filled(..), .. } = self.answer_hole(&hole) {
                    self.transport.collapsed_filled += 1;
                }
            }
            self.decide_neighbors(&there.at);
            return;
        }
        self.transport.held += 1;
        // 62.1 — ASCEND AND PUNCTURE: each cell of the held ring is sealed
        // through the one fill path, and one that reads a class is opened into
        // a hole keeping it — the question the mind now carries.
        for rk in &rings {
            self.ask(rk);
        }
        match disagreement(arrived, (&there.rotor, &there.carriage)) {
            Some((kappa, r)) => {
                for v in [near, there.at.as_slice()] {
                    if let Some(b) = self.bubbles.get_mut(v) {
                        horizon_put(&mut b.horizon, r.clone(), Standing::Held);
                    }
                }
                // 78.37 — THE DOCKET: the held class, its cover beside it, and
                // the ring's handles so a collapse or re-fill answers it (R1).
                let cover = self.ring_cover(&there.source, [near, there.at.as_slice()]);
                let entry = crate::chronometry::Cite::Class { class: Box::new(kappa), rotor: Box::new(r), cover };
                let mut handles = rings;
                handles.push(edge.to_vec());
                self.docket_ring(&handles, &entry);
                self.docket_put(entry);
            }
            None => self.transport.unclassed += 1,
        }
        if let Some(a) = self.fronts.get_mut(there_key) {
            a.stopped = true;
        }
    }

    /// 78.37(1) — the transport's reading: the arrivals at the last step's
    /// `K` (the fronts advanced, edges this k-step), the sources standing,
    /// and what it did since the contemplation began.
    pub fn transport_reading(&self) -> (u64, u64, TransportTally) {
        let at = self.stepped_k;
        let advanced = self.fronts.scoped().filter(|(_, a)| Some(a.k) == at && !a.by.is_empty()).count() as u64;
        let sources = self.fronts.scoped().filter(|(_, a)| a.by.is_empty()).count() as u64;
        (advanced, sources, self.transport)
    }
}
