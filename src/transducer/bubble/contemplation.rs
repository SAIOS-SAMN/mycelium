// 25 — THE CONTEMPLATION: every bubble a mind is holding at once, and the
// incidence between them.
//
//! `Contemplation` is the complex of held thoughts: the bubbles, their
//! remainders, the horizon of classes seeded from the syllabus, and the
//! cavity. Holding a form admits a vertex and links it to what it meets.

use super::*;

thread_local! {
    /// Live allocator bytes every `hold_form` on this thread has added.
    pub(crate) static HELD_BYTES: std::cell::Cell<i128> = const { std::cell::Cell::new(0) };
}

/// Live bytes jemalloc holds, or `None` where the counter does not read.
/// The epoch is advanced first, because jemalloc's stats are a snapshot
/// refreshed on demand and a read without it returns the previous one.
/// Z4's bisect: live bytes at a named point inside the hold, printed only
/// under `MYC_TRACE`. Instrument, never a decision.
pub(crate) fn mark(at: &str) {
    if std::env::var("MYC_TRACE").is_ok_and(|v| v != "0") {
        if let Some(b) = live_bytes() {
            eprintln!("  hold mark {at} live {b}");
        }
    }
}

pub(crate) fn live_bytes() -> Option<i128> {
    tikv_jemalloc_ctl::epoch::advance().ok()?;
    Some(tikv_jemalloc_ctl::stats::allocated::read().ok()? as i128)
}

/// The complex over bubbles. by_chart is the incidence trie that makes
/// every connection a read.
///
/// X3: THE BUBBLES ARE THE OBJECT, and every walk of them below reads them
/// with `scoped`. A contemplation IS the set of thoughts one mind holds at
/// once — there is no coarser coordinate above it to descend from, and a
/// bubble left out of a cohesion, a component count or a shed is a thought
/// the mind holds and does not account for. The carriers INSIDE a bubble —
/// its cover, its remainders, its nerve's sections — are scoped by the
/// bubble the same way, and `by_chart` is the one carrier here that is
/// read by a real coordinate: `link(chart)`, the incidence read this type
/// exists for.
/// HOW LONG A SPEAKING FORM DELIBERATED, and which path it came by.
///
/// Lane 𝒯 is the scene-state span the bundle stood over, and a span of zero
/// has two entirely different causes: a form that was HELD and has not yet
/// aged, and a form that was never held at all because it closed in the
/// stroke it was extracted in. Reported as one `0` the two are unreadable
/// on the wire, with no way to tell which path produced it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Deliberation {
    /// Held in contemplation for this many receipts of TUniversal.
    Held(u64),
    /// Never held: minted in the stroke it was extracted in.
    Fresh,
}

impl Deliberation {
    /// The span as lane 𝒯 carries it. A form that was never held stood over
    /// no receipts, which is a reading of zero and not a placeholder.
    pub fn span(self) -> u64 {
        match self {
            Deliberation::Held(t) => t,
            Deliberation::Fresh => 0,
        }
    }

    /// The word a journal line names the path by.
    pub fn label(self) -> &'static str {
        match self {
            Deliberation::Held(_) => "held",
            Deliberation::Fresh => "fresh",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Contemplation {
    pub bubbles: Keyed<Bubble>,
    /// THE CHARTS' INCIDENCE: chart key → the ORDINALS of the bubbles
    /// standing there (Z4 item 2). The inner key is the ordinal's two
    /// bytes, which is where the ordinal is held: a keyed inner trie
    /// removes one entry by its own coordinate at retraction, where a list
    /// would be scanned.
    ///
    /// IT HELD THE BUBBLE'S ADDRESS, ONCE PER CHART OF ITS COVER, and a
    /// form's address is its every vertex, member and gauge written out
    /// (law 2): 7.3 MB on the 590-vertex rule, measured at e0f4e0d, copied
    /// into every entry. The address is written ONCE now, on the bubble —
    /// `bubbles` is keyed by it — and every index names the bubble by the
    /// ordinal instead.
    pub by_chart: Keyed<Keyed<()>>,
    /// THE CITATION FIBRATION (25 door 1): cited address → the ORDINALS of
    /// the bubbles citing it. A bubble is pinned by `glues`, and `glues` is
    /// written at the hold and emptied nowhere, so a pin outlives every
    /// reason for it; this is the carrier that lets a pin be asked about.
    ///
    /// MEASURED, on the 60-task slice: 21 of 51 held bubbles cite, 21 of 21
    /// pins are live, 0 are dead. A dead-pin sweep would retire none of
    /// them. What retires them is the CITED address lapsing, and that is a
    /// keyed read here rather than a walk of every bubble asking each what
    /// it cites (rule 1, rule 17).
    pub(crate) cited: Keyed<Keyed<()>>,
    /// Bubble address → its ordinal in this contemplation. The shape
    /// `Form.by_address` already has.
    pub(crate) ordinals: Keyed<u16>,
    /// Ordinal → the bubble's address, or `None` where the bubble was
    /// retracted. A RETRACTED ORDINAL IS NEVER REUSED within one
    /// contemplation, so a stale index entry names nothing rather than
    /// naming another bubble — which is the one way an ordinal could lie.
    pub(crate) addresses: Vec<Option<std::sync::Arc<[u8]>>>,
    pub complex: Complex,
    /// Bubbles whose cover still has work; written when dirty is set.
    pub(crate) dirty: Keyed<()>,
    /// The ring's resume cursor: `None` is a walk that has not started,
    /// which is an absence and not an empty key (rule 11, X3).
    pub(crate) expand_cursor: Option<Vec<u8>>,
    /// The dirty ring's orbit (45.7, S13.3): the circuits `expand_all`
    /// completed and the strokes the last one took — the mind's local time
    /// beside K, written once per circuit as `[{tag}] orbit ring`.
    pub(crate) ring: crate::physics::Orbit,
    pub fills: u64,
    pub torsion_merges: u64,
    pub classes: u64,
    pub formed: u64,
    /// Bubbles retired to a cavity chart, address kept, remainder in the tape.
    pub(crate) cavity: Keyed<Vec<Link>>,
    pub(crate) pending_rewrites: Vec<Element>,
    /// TUniversal the entity last stamped; T-room and potential[2] read it.
    pub(crate) now_k: u64,
    /// Z4d — THE APERTURE OF STROKES ON TUNIVERSAL, as a window of the
    /// heights this contemplation has been stamped at. The span of it is
    /// what a held thought's escrow BUYS, so the mind's short-term memory
    /// forgets by a measured horizon and not by a constant.
    ///
    /// The dialect keeps the same window on the VOICE'S K
    /// (`Epigenome::aperture_k`) and this is its twin on TUniversal —
    /// 45.4's two clocks, kept apart, each measuring its own carrier's
    /// horizon. Crossing them is the defect that doc already names.
    pub(crate) stroke_tu: Vec<u64>,
    /// Mesh-seeded classes, keyed by class key, Standing::Held. Not persisted.
    pub(crate) seeded: Keyed<(Rotor, Order, Standing)>,
    /// EXPANSION 4 — THE VORTICES BY THEIR CONJUGACY STRATUM: the
    /// depth-one coordinate of the class vector a vortex is filed under →
    /// the bubbles standing there. A vortex is decided by a ruled word
    /// whose rotor is conjugate to the loop's holonomy, and a word knows
    /// its own conjugacy class and nothing finer, so that class IS the
    /// coordinate this index is keyed by: a keyed read, never a walk of
    /// every bubble asking each (rule 1).
    ///
    /// Written where a bubble's incidence is admitted and wherever a pivot
    /// rewrote a form (`refile_vortex`); not persisted, rebuilt as the
    /// other indices are.
    pub(crate) by_vortex: Keyed<Keyed<()>>,
}

/// P9 — THE CANDIDATE TRANSPORTS OVER ONE OPEN DEMAND, read from the
/// bodies it stands for (50.5) and never stored beside them (law 12).
///
/// A demand's links are the two sides of one transition split by which hue
/// they occupy — `before` where the hue it came from is set, `after` where
/// the hue it goes to is — which is the same split `ring_answer` makes in
/// `family/memo.rs`. `carriage_with_core` is the transport between the two
/// bodies: the versor where one exists, and the homeomorphic core's where
/// the shapes differ by a surgery (`Surgery::factor`), which is the case an
/// uncarried body is in.
///
/// The candidate is the ROTOR of that transport. `peel_versor` answers it
/// where the seventeen spell the versor, and where they do not the peel's
/// own `Standing` demand carries the canonical holonomy it stopped at
/// (16.16 step 6) — which is 37.7's object and is the candidate that
/// matters. Every other outcome names no rotor and contributes none:
/// nothing is stood in for a transport that was not measured (rule 11).
pub(crate) fn candidates_over(bodies: &[Link]) -> Keyed<Rotor> {
    let mut out: Keyed<Rotor> = Keyed::new();
    let before: Vec<Link> = bodies.iter().copied().filter(|c| c.2 != 0).collect();
    let after: Vec<Link> = bodies.iter().copied().filter(|c| c.3 != 0).collect();
    if before.is_empty() || after.is_empty() {
        return out;
    }
    let lift = |links: &[Link]| {
        crate::transducer::family::body_of(
            links.iter().map(|&(r, c, _, h)| (r, c, 0i64, h)).collect::<Vec<_>>(),
        )
    };
    let (a, b) = (lift(&before), lift(&after));
    let Some(car) = crate::transducer::family::carriage_with_core(&a, &b) else {
        return out;
    };
    let rotor = match crate::transducer::family::peel_versor(&car.v) {
        Some(Ok(w)) => w.rotor(),
        Some(Err(crate::transducer::sentence::Demand::Standing { rotor, .. })) => *rotor,
        _ => return out,
    };
    out.insert(rotor.key(), rotor);
    out
}

impl Contemplation {
    pub fn new() -> Contemplation {
        Contemplation::default()
    }

    /// WHERE A SPEAKING FORM CAME FROM, and how long it deliberated: the span
    /// its idea has stood over, in receipts — lane 𝒯, the same
    /// `held_since(now, moved_k)` that `expand_with` hands `pivot_loop` as
    /// its `t_extent`. One quantity, one reader (law 12).
    ///
    /// A form this contemplation does not hold stood over NO receipts: it
    /// was minted in the stroke it was extracted in. That is a reading of
    /// zero, not a placeholder for one, and a different fact from a held
    /// form aged zero, so the two are two readings. `moved_k` is stamped
    /// from `now_k` and this clock only advances, so the subtraction cannot
    /// go backwards while the contemplation is the one holding both.
    ///
    /// `None` where the clock went backwards — a K below the mark is a broken
    /// counter, and a span read off one is a measurement this contemplation
    /// cannot make, never a zero (rule 11).
    pub fn span_of(&self, form: &crate::transducer::sentence::Form) -> Option<Deliberation> {
        let key = super::idea::idea_key(form);
        match self.bubbles.get(key.as_bytes()) {
            // Held nowhere: minted in the stroke it was extracted in, so it
            // stood over no receipts. That is a reading of zero.
            None => Some(Deliberation::Fresh),
            Some(b) => super::idea::held_since(self.now_k, b.moved_k).map(Deliberation::Held),
        }
    }

    /// The TUniversal this contemplation was last stamped with — the mind's
    /// OWN K (45.4), never the mesh's height, which advances only on
    /// admitted receipts (45.2) and so stands at 0 on a mesh that admits
    /// nothing, where every bubble's extent would read the same and
    /// `retire_on_t` could tell none apart. See [`Deliberation`] for what a
    /// span read against it means.
    pub fn now_k(&self) -> u64 {
        self.now_k
    }

    pub fn set_now(&mut self, k: u64) {
        self.now_k = k;
        // The window is the last aperture of stamps, in stamp order.
        let n = crate::physics::aperture_cycle().max(1) as usize;
        self.stroke_tu.push(k);
        if self.stroke_tu.len() > n {
            self.stroke_tu.remove(0);
        }
    }

    /// Z4d — THE SPAN AN ESCROW BUYS: the measured aperture of this
    /// contemplation's own strokes on TUniversal.
    ///
    /// `None` until a whole aperture has been stamped. An unmeasured
    /// horizon is held, never guessed (rule 15), so nothing lapses in the
    /// first aperture after boot and no thought is paid to a height
    /// nothing measured.
    pub fn aperture_tu(&self) -> Option<u64> {
        let n = crate::physics::aperture_cycle().max(1) as usize;
        if self.stroke_tu.len() < n {
            return None;
        }
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — FIFO drain head: the window is filled in stamp order, so its head is the oldest stamp in it"
        )]
        let first = *self.stroke_tu.first()?;
        self.now_k.checked_sub(first)
    }

    pub fn len(&self) -> usize {
        self.bubbles.len()
    }

    /// No bubble is held: nothing is under contemplation.
    pub fn is_empty(&self) -> bool {
        self.bubbles.is_empty()
    }

    /// True when a live idea glued this address (a dialect entry so marked
    /// is not shed).
    pub fn glues_word(&self, addr: &[u8]) -> bool {
        self.bubbles
            .scoped()
            .any(|(_, b)| b.glues.contains(addr))
    }

    /// Record that idea `bubble` glued crystallized form `word`.
    pub fn mark_glue(&mut self, bubble: &[u8], word: impl AsRef<[u8]>) {
        if let Some(b) = self.bubbles.get_mut(bubble) {
            b.glues.insert(word, ());
        }
    }

    // P10 — `cohesion` WAS HERE AND IS DELETED, with the scalar it summed.
    //
    // It accumulated each idea's `volume` per chart — `v[l] =
    // v[l].saturating_add(add[l])` over every bubble standing there — and
    // called the total a chart's congruent mass. Mass is repetition: an
    // idea re-held ten times added ten times, and the reading said a chart
    // was cohesive because the mind had said one thing on it often. That
    // is not what agreement between two ideas is. Two ideas agree when the
    // transition between their transports is the identity, which is a
    // reading of the two and not a count of them.
    //
    // NOTHING READ IT. Measured across the tree at this commit: no caller
    // in `src`, tests included — the `cohesion_over` of `server::cognition`
    // is the court's rank reading and a different function. So the scalar
    // is removed by deleting its only reader rather than by giving a dead
    // carrier a better formula, which is R6's ruling applied again (law 12:
    // a carrier nothing reads is deleted, not improved).
    //
    // What replaces it is not written until something asks: the transition
    // between two candidates is `Vⱼ⁻¹Vₖ` and flatness is that quotient
    // being the identity, which `candidates` holds the operands for.

    pub fn cavity_len(&self) -> usize {
        self.cavity.len()
    }

    /// The links every cavity entry holds together. One entry is a whole
    /// remainder, so the entry count says how many retirements happened and
    /// this says what they kept — and a thousand entries of one link and ten
    /// of a thousand are the same count and not the same carrier.
    pub fn cavity_links(&self) -> usize {
        self.cavity.scoped().map(|(_, r)| r.len()).sum()
    }

    /// WHAT THE CONTEMPLATION ACTUALLY HOLDS (§8.1, measurement only).
    ///
    /// `ideas` counts BUBBLES, and a bubble is not a unit of memory. These
    /// are the carriers a bubble holds that no other line reports, and they
    /// are the ones with weight in them:
    /// - `nerve.sections` — a `Section` carries a γ and a carriage, and a
    ///   versor is 1024 bytes (`Acting::carriage`).
    /// - `nerve` simplices across every degree, each carrying a `Rotor`.
    /// - `horizon` — one `(Rotor, Order, Standing)` per class the idea met,
    ///   and `paint_standing` writes one per standing loop of every form
    ///   held. A rotor is sixteen `BigRational`, each of them heap.
    /// - `bundle` — the training pairs, COPIED per bubble.
    /// - `certificates` and `vortex`, which hold certificates carrying class
    ///   keys and moves.
    ///
    /// Counts and never bytes: a size in bytes would be a number this code
    /// cannot derive exactly, and rule 11 refuses an invented one. What the
    /// counts do is say WHICH carrier grows, which is the question.
    pub fn weight(&self) -> [usize; 5] {
        let mut out = [0usize; 5];
        for (_, b) in self.bubbles.scoped() {
            out[0] += b.nerve.sections.len();
            out[1] += b.nerve.degrees.iter().map(|d| d.len()).sum::<usize>();
            out[2] += b.horizon.len();
            out[3] += b.bundle.as_ref().map_or(0, |p| p.len());
            out[4] += b.certificates.len()
                + b.vortex.scoped().map(|(_, v)| v.len()).sum::<usize>();
        }
        out
    }

    /// THE CAVITY ENTRIES A CHART CAN RE-HOLD, read at that chart. The same
    /// descent `pivot_on` makes, exposed so the keying can be asserted
    /// without the drive: a cavity keyed by the bubble alone answers nothing
    /// here, which is why the drain had to copy the whole carrier instead.
    ///
    /// `None` for a chart with no key — an empty prefix names no coordinate,
    /// and a caller whose chart turned out empty must say which reading it
    /// wanted (rule 11, `Prefix::new`).
    pub fn cavity_keys_under(
        &self,
        inv: &crate::transducer::stalk::Invariant,
    ) -> Option<Vec<Vec<u8>>> {
        let k = inv.key();
        Some(self.cavity.fiber_at(k.as_bytes())?.map(|(k, _)| k).collect())
    }

    pub fn remainders(&self) -> Vec<&[Link]> {
        self.bubbles.scoped().map(|(_, b)| b.remainder.as_slice()).collect()
    }

    /// THE BOUNDARY MEASURE of the open thoughts (law 5, L7a): the
    /// four-lane measure of the held remainders, read per lane and never
    /// summed — a sum over lanes was the L1 norm of the convergence
    /// engine's coherence functional, the scalar the doctrine refuses. An
    /// unmeasured space is an open thought (∂g ≠ 0), not infinite
    /// discordance; no thought is `[0; 4]`.
    pub fn boundary_measure(&self) -> [u64; 4] {
        let mut acc: Vec<Link> = Vec::new();
        for r in self.remainders() {
            acc.extend(r.iter().copied());
        }
        acc.sort_unstable();
        acc.dedup();
        element::measure_lanes(&acc)
    }

    pub(crate) fn retract_incidence(&mut self, key: &[u8], b: &Bubble) {
        let Some(&ord) = self.ordinals.get(key) else {
            self.complex.drop_vertex(key);
            return;
        };
        let ob = ord.to_le_bytes();
        let charts: Vec<Vec<u8>> = b.cover.scoped().map(|(k, _)| k).collect();
        for ck in charts {
            if let Some(m) = self.by_chart.get_mut(&ck) {
                m.remove(&ob);
            }
        }
        self.unfile_vortex(key, &b.vortex_strata());
        for (addr, ()) in b.glues.scoped() {
            if let Some(m) = self.cited.get_mut(&addr) {
                m.remove(&ob);
            }
        }
        // THE ORDINAL IS SPENT, NOT FREED. Its slot is emptied and the
        // ordinal is never issued again, so an entry this retraction
        // missed names nothing instead of naming whatever arrived next.
        if let Some(slot) = self.addresses.get_mut(ord as usize) {
            *slot = None;
        }
        self.ordinals.remove(key);
        self.complex.drop_vertex(key);
    }

    /// The bubble leaves the vortex index under each of `strata`.
    /// THE TRANSLATION HAPPENS HERE AND NOWHERE ELSE. Every caller names
    /// a bubble by its ADDRESS, which is the identity; the index holds the
    /// ordinal, which is a coordinate of this carrier alone. One place
    /// converts, so no caller can hold an ordinal it did not get from here.
    pub(crate) fn unfile_vortex(&mut self, key: &[u8], strata: &[Vec<u8>]) {
        let Some(&ord) = self.ordinals.get(key) else {
            return;
        };
        let ob = ord.to_le_bytes();
        for ck in strata {
            if let Some(m) = self.by_vortex.get_mut(ck) {
                m.remove(&ob);
            }
        }
    }

    /// THE VORTICES A BUBBLE HOLDS, FILED AT THEIR CONJUGACY STRATA. A
    /// pivot rewrites the form in place (`take_form`) and may leave a
    /// vortex on it that was not there when the bubble's incidence was
    /// admitted; the index is re-read off the bubble at every such site. An
    /// entry a collapse consumed leaves with it (`collapse_on`, and the
    /// ruling path in `seed_horizon_inner`).
    pub(crate) fn refile_vortex(&mut self, key: &[u8]) {
        let strata: Vec<Vec<u8>> = match self.bubbles.get(key) {
            Some(b) => b.vortex_strata(),
            None => return,
        };
        let ord = self.ordinal_for(&std::sync::Arc::from(key)).to_le_bytes();
        for ck in strata {
            self.by_vortex.nested_insert(&ck, ord, ());
        }
    }

    /// EXPANSION 4 — COLLAPSE EVERY VORTEX A ROTOR IS A CANDIDATE OF. A
    /// vortex holds several certified discharges of one loop; a ruled word
    /// whose rotor is one of its candidates decides it, and the differences
    /// between that candidate and the others — rotations the mind measured
    /// (37.7) — ride out to be adjoined, each with the chart the deciding
    /// word was met on.
    ///
    /// Read at the ROTOR'S OWN CONJUGACY CLASS, which is the depth-one
    /// coordinate of the class vector a vortex is filed under: the
    /// candidate is filed at the loop's own class (`vortex_put`), and the
    /// deciding word's class key is that key's stratum, so this is a
    /// projection and not a guess. Neither the arriving word's chart, nor
    /// the loop's AFTER-class — the zero class for a closing candidate —
    /// nor the candidate's own element key, which is an identity and not a
    /// coordinate, would meet one.
    pub(crate) fn collapse_on(
        &mut self,
        rotors: &[Rotor],
        chart: &Invariant,
        by: &'static str,
    ) -> Vec<(Rotor, Invariant)> {
        let mut out = Vec::new();
        for r in rotors {
            // A rotor with no class stands at no stratum and decides
            // nothing (15.3); no key of its bytes stands in.
            let Some(stratum) = r.class_key() else {
                continue;
            };
            let rk = r.key();
            // index: the bubbles filed at this conjugacy stratum, each by
            // its ORDINAL, resolved to its address here — an ordinal that
            // names nothing is a retracted bubble and yields no holder
            // (Z4 item 2).
            let holders: Vec<Vec<u8>> = self
                .by_vortex
                .link(stratum.as_bytes())
                .filter_map(|(k, _)| Self::ordinal_of_key(&k))
                .filter_map(|o| self.address_at(o).map(<[u8]>::to_vec))
                .collect();
            for bk in holders {
                let held: Vec<Vec<u8>> = match self.bubbles.get(&bk) {
                    Some(b) => b.vortex_strata(),
                    // The bubble is gone; its entry leaves.
                    None => {
                        self.unfile_vortex(&bk, std::slice::from_ref(&stratum.clone().into_vec()));
                        continue;
                    }
                };
                let collapsed = self
                    .bubbles
                    .get_mut(&bk)
                    .and_then(|b| b.collapse_vortex(stratum.as_bytes(), rk.as_bytes(), by));
                match collapsed {
                    // A collapse cleared the form's vortices: the bubble
                    // leaves the index under every stratum it stood at and
                    // is refiled under what remains.
                    Some(syms) => {
                        self.unfile_vortex(&bk, &held);
                        self.refile_vortex(&bk);
                        for s in syms {
                            out.push((s, chart.clone()));
                        }
                    }
                    // No vortex here stands at this stratum any more: the
                    // entry is stale and leaves.
                    None => {
                        if !held.iter().any(|c| c.as_slice() == stratum.as_bytes()) {
                            self.unfile_vortex(
                                &bk,
                                std::slice::from_ref(&stratum.clone().into_vec()),
                            );
                        }
                    }
                }
            }
        }
        out
    }

    /// LINEAGE THROUGH MUTATION: the entry follows the form it holds.
    ///
    /// A bubble's key is its form's address, and a pivot REWRITES THE FORM IN
    /// PLACE — `expand_all` takes the slot by the key it entered with and
    /// `expand_with` calls `take_form` inside it. Left where it was, the
    /// bubble keeps its `moved_k` and everything it has built while its key
    /// stops matching the form it now holds, and every reader that looks it
    /// up by the current form misses it: `span_of` answers "not held" for a
    /// bubble that has been deliberating for thousands of receipts, and
    /// `hold_form` inserts a SECOND bubble for the same lineage while the
    /// first is orphaned under a key nothing computes any more. A form with
    /// an open remainder speaks only once it closes, and closing mutates it.
    ///
    /// The entry moves with its inception stamp and its accumulated state, so
    /// a resolved sentence carries the true extent of its deliberation —
    /// the bubble that deliberated is the bubble that speaks.
    ///
    /// Where the destination is already held, two lineages have converged on
    /// one form. The survivor is the one already under that key, and it takes
    /// the EARLIER inception of the two: the deliberation began when the
    /// first of them began, and a later stamp would shorten a span that was
    /// actually stood.
    pub(crate) fn rekey(&mut self, from: &[u8], to: &[u8]) {
        if from == to {
            return;
        }
        let Some(moving) = self.bubbles.remove(from) else {
            return;
        };
        self.retract_incidence(from, &moving);
        let was_dirty = self.dirty.remove(from).is_some();
        match self.bubbles.get_mut(to) {
            Some(standing) => {
                standing.moved_k = standing.moved_k.min(moving.moved_k);
            }
            None => {
                let shared: std::sync::Arc<[u8]> = std::sync::Arc::from(to);
                self.admit_incidence(&shared, &moving);
                self.bubbles.insert_shared(shared, moving);
            }
        }
        if was_dirty {
            self.dirty.insert(to, ());
        }
    }

    /// THE INCIDENCE A BUBBLE IS ADMITTED UNDER: its cover's charts and its
    /// vortices' strata, each naming the bubble by its address. Every entry
    /// copies that address, which is the form's vertex key and grows with
    /// the form, so an index is written here only where a reader takes it
    /// (`held_at`, `link_new`, the vortex collapse); an index nothing reads
    /// is that address copied once per entry for no reading (law 12).
    /// THE ORDINAL OF A BUBBLE, assigned on its first admission and held
    /// while the contemplation lives. Never reused after retraction.
    /// Z4c: the caller hands the ONE copy of the address. `Arc::from(&[u8])`
    /// would copy it here — 7 MB per hold on the 590-vertex rule — and the
    /// caller always has the address as an object, never only as a slice.
    pub(crate) fn ordinal_for(&mut self, key: &std::sync::Arc<[u8]>) -> u16 {
        if let Some(&o) = self.ordinals.get(key) {
            return o;
        }
        let o = self.addresses.len() as u16;
        self.addresses.push(Some(std::sync::Arc::clone(key)));
        self.ordinals.insert_shared(std::sync::Arc::clone(key), o);
        o
    }

    /// The address an ordinal names, or `None` where it names nothing —
    /// a retracted bubble, or an ordinal this contemplation never issued.
    pub(crate) fn address_at(&self, ord: u16) -> Option<&[u8]> {
        self.addresses.get(ord as usize)?.as_deref()
    }

    /// The ordinal an index entry's key IS.
    pub(crate) fn ordinal_of_key(k: &[u8]) -> Option<u16> {
        Some(u16::from_le_bytes(k.try_into().ok()?))
    }

    pub(crate) fn admit_incidence(&mut self, key: &std::sync::Arc<[u8]>, b: &Bubble) {
        let ord = self.ordinal_for(key);
        let ord = ord.to_le_bytes();
        let key = &ord[..];
        for (ck, _) in b.cover.scoped() {
            match self.by_chart.get_mut(&ck) {
                Some(m) => {
                    m.insert(key, ());
                }
                None => {
                    let mut m = Keyed::new();
                    m.insert(key, ());
                    self.by_chart.insert(&ck, m);
                }
            }
        }
        for ck in b.vortex_strata() {
            self.by_vortex.nested_insert(&ck, key, ());
        }
        // The citation fibration: this bubble is filed under every address
        // it cites, by ordinal.
        for (addr, ()) in b.glues.scoped() {
            self.cited.nested_insert(&addr, key, ());
        }
    }

    /// THE HELD STRATUM AT A CHART (24.1 as ruled 2026-09-09): the Word
    /// vertices of the held forms on `inv`, read at the coordinate the
    /// caller supplies — the chart names its bubbles (`by_chart`), and each
    /// bubble's form names the vertices it holds on that chart
    /// (`Form.by_class`, 18.2). Nothing is materialised ahead of the
    /// question, no vertex list is walked, and no second carrier of held
    /// words exists beside this contemplation.
    pub fn held_at(&self, inv: &Invariant) -> Vec<&Vertex> {
        let chart = inv.key();
        let Some(ideas) = self.by_chart.get(chart.as_bytes()) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        // index: the bubbles this chart names; then the vertices each
        // form's by_class names on it
        for (ok, _) in ideas.scoped() {
            let Some(bk) = Self::ordinal_of_key(&ok).and_then(|o| self.address_at(o)) else {
                continue;
            };
            let Some(form) = self.bubbles.get(bk).and_then(|b| b.form.as_ref()) else {
                continue;
            };
            // 54.7 R5 — ASKED AT THE COORDINATE IT ANSWERS. `by_class` is a
            // chart index and this read is written as one; a key of any
            // other system does not reach it.
            let Ok(ck) = Key::<coord::Chart>::from_address(&chart) else {
                return Vec::new();
            };
            let Some(on) = form.by_class.at(&ck) else {
                continue;
            };
            for &i in on {
                if let Some(v @ Vertex::Word { .. }) = form.vertices.get(i as usize) {
                    out.push(v);
                }
            }
        }
        out
    }

    pub(crate) fn rotor_between(&self, a: &[u8], b: &[u8]) -> Option<Rotor> {
        let ba = self.bubbles.get(a)?;
        let bb = self.bubbles.get(b)?;
        let ra = ba.rotor()?;
        let rb = bb.rotor()?;
        Some(rb.geo(&ra.inverse()?))
    }

    /// Link of one new bubble: edges to bubbles sharing a chart (by_chart
    /// peel), higher simplices where shared remainder volume is nonzero.
    pub(crate) fn link_new(&mut self, key: impl AsRef<[u8]>) {
        let key = key.as_ref().to_vec();
        let (charts, vol0) = {
            let Some(b) = self.bubbles.get(&key) else {
                return;
            };
            (
                b.cover.scoped().map(|(k, _)| k).collect::<Vec<_>>(),
                b.volume,
            )
        };
        self.complex.set_section(
            &key,
            // A bubble's section carries a rotation and no placement, so
            // the carriage is the identity — which is TRUE of it, not a
            // value stood in for one it has (L11e).
            Section::at(crate::transducer::address::Address::from(key.clone()), Rotor::identity()),
            vol0,
        );
        let mut neighbors = Keyed::new();
        for ck in &charts {
            for (id, _) in self.by_chart.link(ck) {
                let Some(addr) = Self::ordinal_of_key(&id).and_then(|o| self.address_at(o)) else {
                    continue;
                };
                if addr != key.as_slice() {
                    neighbors.insert(addr, ());
                }
            }
        }
        let nbs: Vec<Vec<u8>> = neighbors.scoped().map(|(k, _)| k).collect();
        let mut rems: Keyed<Vec<Link>> = Keyed::new();
        // Overlap is read in the SCENE: the chain carried through its own
        // translator (§2.1). Two bubbles holding one chain at two
        // translators are one substance said at two places, and their
        // canonical links coincide at the origin while their placed links
        // do not.
        if let Some(b) = self.bubbles.get(&key) {
            rems.insert(&key, b.remainder.placed());
        }
        for n in &nbs {
            if let Some(b) = self.bubbles.get(n) {
                rems.insert(n, b.remainder.placed());
            }
            let vol = volume_of(&rems, &[key.as_slice(), n.as_slice()]);
            match self.rotor_between(&key, n) {
                Some(rotor) => self.complex.set_edge(
                    &key,
                    n,
                    rotor,
                    crate::transducer::element::identity_versor(),
                    vol,
                ),
                None => {
                    let both_remainder = self.bubbles.get(&key).is_some_and(|b| b.form.is_none())
                        && self.bubbles.get(n).is_some_and(|b| b.form.is_none());
                    if both_remainder && vol.iter().any(|&x| x > 0) {
                        let _ = self.merge(&[key.clone(), n.clone()]);
                        break;
                    }
                }
            }
        }
        self.complex.coboundary_on_link(&key, |verts| volume_of(&rems, verts));
        self.decide_neighbors(&key);
    }

    /// Hold a form as an idea (chapter 30 step 5): the form is the carrier,
    /// and this contemplation is the one carrier of held forms (24.1 as
    /// ruled 2026-09-09): its words are read at a chart when a cite asks,
    /// through the form's own `by_class`, which is derived here as the
    /// form is taken into custody. The train is the germinating bundle;
    /// remainders are per-pair residual chains. idea_key is the form's
    /// vertex key.
    /// Z4's instrument (§8.1): the allocator bytes a hold ADDS, read
    /// between `hold_form`'s entry and its exit and totalled per stroke.
    ///
    /// IT IS THE CLAIM AND NOT THE OUTCOME (rule 13). "The fleet grows at
    /// 492 MB a minute" is an outcome and names nothing; "one hold of a
    /// 712-vertex form costs N bytes" is the claim, and a fix either moves
    /// N or it did not fix anything. The `grew` lane beside it is the
    /// arm's total; this is the hold's own share of it.
    ///
    /// The reading is jemalloc's `allocated` — LIVE bytes, so a hold that
    /// allocates and frees within itself reads zero and a hold that
    /// retains reads what it retained. Nothing in the engine branches on
    /// it and `hold_form` returns `()`.
    pub fn held_bytes() -> i128 {
        HELD_BYTES.with(|c| c.get())
    }

    pub fn reset_held_bytes() {
        HELD_BYTES.with(|c| c.set(0));
    }

    /// Admit a remainder as a bubble with its cover a peel of its invariant.
    pub fn hold(&mut self, remainder: Vec<Link>) {
        if remainder.is_empty() {
            return;
        }
        // A remainder saying two things at one site has no A5 chain and no
        // address; it is not held (U14).
        let Some(remainder) = element::place_chain(&remainder) else {
            return;
        };
        let key = remainder_addr(&remainder);
        let inv = stalk::invariant(&remainder);
        let cover = cover_of(&inv);
        // The volume is the remainder's own measure (L7b): a T extent of
        // zero is zero, and no lane is floored so that a gate downstream
        // would pass — the pivot loop runs on every dirty idea (25.1), T
        // paces nothing (law 9).
        let volume = element::measure_lanes(&remainder);
        let mut horizon = Keyed::new();
        self.paint_seeded(&mut horizon);
        let bubble = Bubble {
            remainder,
            nerve: Complex::new(),
            cover,
            horizon,
            potential: volume,
            volume,
            dirty: true,
            moved_k: self.now_k,
            escrow: volume,
            until: self.aperture_tu().and_then(|span| self.now_k.checked_add(span)),
            glues: Keyed::new(),
            glued_by: Keyed::new(),
            torsion: Keyed::new(),
            certificates: Keyed::new(),
            vortex: Keyed::new(),
            candidates: Keyed::new(),
            // A bubble with no form has no rotor to derive (rule 11).
            rotor: None,
            form: None,
            bundle: None,
            remainders: Keyed::new(),
        };
        if let Some(old) = self.bubbles.get(key.as_bytes()).cloned() {
            self.retract_incidence(key.as_bytes(), &old);
        }
        self.admit_incidence(&key.shared(), &bubble);
        self.bubbles.insert_shared(key.shared(), bubble);
        self.dirty.insert_shared(key.shared(), ());
        self.link_new(&key);
    }

    pub(crate) fn component_of(&self, start: &[u8]) -> Vec<Vec<u8>> {
        let mut seen = Keyed::new();
        let mut stack = vec![start.to_vec()];
        seen.insert(start, ());
        while let Some(v) = stack.pop() {
            for ck in self.complex.link(&v, 1) {
                let Some((fill, vertices)) = self.complex.simplex(&ck).map(|c| {
                    (matches!(c.fill, Fill::Filled(..)), c.vertices.clone())
                }) else {
                    continue;
                };
                if !fill {
                    continue;
                }
                for u in &vertices {
                    if !seen.contains(u) {
                        seen.insert(u, ());
                        stack.push(u.clone());
                    }
                }
            }
        }
        seen.scoped().map(|(k, _)| k).collect()
    }

    pub(crate) fn component_closed(&self, verts: &[Vec<u8>]) -> bool {
        if verts.is_empty() {
            return false;
        }
        let mut held = Keyed::new();
        for v in verts {
            held.insert(v, ());
        }
        // index: vertices of this component; each simplex from that
        // vertex's link
        for v in verts {
            for sk in self.complex.link_of(v) {
                let Some(simplex) = self.complex.simplex(&sk) else {
                    continue;
                };
                if simplex.degree() == 0 {
                    continue;
                }
                let all_in = simplex.vertices.iter().all(|u| held.contains(u));
                if all_in && !matches!(simplex.fill, Fill::Filled(..)) {
                    return false;
                }
            }
        }
        true
    }

    pub(crate) fn form_thought(&self, verts: &[Vec<u8>]) -> Option<Element> {
        let mut forms = Vec::new();
        for v in verts {
            let b = self.bubbles.get(v)?;
            if !b.remainder.is_empty() {
                return None;
            }
            forms.push(b.form.as_ref()?);
        }
        // The component arrives in KEY ORDER (`component_of` reads it off a
        // keyed fiber), so the head is its least vertex and every node
        // forms the thought on the same one.
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — canonical key order: the component is a keyed fiber, so its head is the least address in it"
        )]
        let &f = forms.first()?;
        if !f.is_cauterized() {
            return None;
        }
        let mag = form_mag(f);
        if mag.iter().all(|&x| x == 0) {
            return None;
        }
        sentence::as_element_form(f, mag)
    }

    /// CAUTERIZE (25.2): an idea whose form is closed — every loop the
    /// identity, filed torsion or an opened axis, every remainder empty —
    /// crystallizes. The tape carries its torsion, certificates and axes,
    /// M2 fixes the address, and its magnitude is the descent it measures
    /// over the bundle it germinated from (26), through the crystal that
    /// resolves what it cites; a form with no bundle carries what it glued.
    /// The bubble leaves the contemplation unless a live idea glued it
    /// (`retire_on_t` skips those). `None` when the form is not closed, a
    /// pair of the bundle is still open, or nothing is measured: a form that
    /// closes nothing is the citation, not a form (4), and stays a thought.
    pub fn crystallize(&mut self, key: &[u8], crystal: &dyn Crystal) -> Option<Element> {
        let b = self.bubbles.get(key)?;
        let f = b.form.as_ref()?;
        if !f.is_cauterized() || !f.remainder.is_empty() {
            return None;
        }
        if b.remainders.scoped().any(|(_, r)| !r.is_empty()) {
            return None;
        }
        let mag = match b.bundle.as_deref() {
            Some(bundle) => {
                let (pay, rems) = crate::transducer::family::form_deflation(f, bundle, crystal);
                if rems.iter().any(|r| !r.is_empty()) {
                    return None;
                }
                pay
            }
            None => form_mag(f),
        };
        let e = sentence::as_element_form(f, mag)?;
        let (loops, torsion, certificates, axes) =
            (f.loops.len(), f.torsion.len(), f.certificates.len(), f.axes.len());
        let retire = b.glued_by.is_empty();
        if retire {
            let old = b.clone();
            self.retract_incidence(key, &old);
            self.bubbles.remove(key);
            self.dirty.remove(key);
        }
        self.formed += 1;
        eprintln!(
            "  form crystallized {:02x}{:02x}… loops {loops} torsion {torsion} certificates {certificates} axes {axes} carried {mag:?}",
            e.address()[0],
            e.address()[1]
        );
        Some(e)
    }

    /// Classes on the contemplation, any degree.
    pub fn classes_held(
        &self,
    ) -> Vec<(crate::transducer::sentence::ClassVector, Rotor, usize)> {
        self.complex.classes()
    }

    pub fn component_count(&self) -> usize {
        let mut seen = Keyed::new();
        let mut n = 0usize;
        let keys: Vec<Vec<u8>> = self.bubbles.scoped().map(|(k, _)| k).collect();
        for k in keys {
            if seen.contains(&k) {
                continue;
            }
            n += 1;
            for v in self.component_of(&k) {
                seen.insert(v, ());
            }
        }
        n
    }

    pub fn horizon_len(&self) -> usize {
        let mut n = 0usize;
        for (_, b) in self.bubbles.scoped() {
            n += b.horizon.len();
        }
        n
    }

    pub fn potential(&self) -> [u64; 4] {
        let mut p = [0u64; 4];
        for (_, b) in self.bubbles.scoped() {
            for (t, d) in p.iter_mut().zip(&b.potential) {
                *t = t.saturating_add(*d);
            }
        }
        p
    }

}
