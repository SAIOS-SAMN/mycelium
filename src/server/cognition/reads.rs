// 42 — THE COURT'S READS: the small accessors a caller outside the
// book asks the court for. Split from `state.rs` at the §1.1 600-line
// reading bound when the awaiting docket was added (21 step 3); the
// items are moved, not rewritten (§1.6).

use super::*;

/// The kinds of word a court's language holds, in [`Court::basis_kinds`]'s
/// order.
pub const HELD_KINDS: [&str; 8] = ["sentences", "letters", "compositions", "connections", "colour words", "rewrites", "translations", "other"];

impl Court {
    pub(crate) fn mark_persist(&mut self) {
        self.persist_due = true;
    }

    pub fn take_persist(&mut self) -> bool {
        let due = self.persist_due;
        self.persist_due = false;
        due
    }

    /// 78.137 — a persist taken that did not land: still due.
    pub fn persist_failed(&mut self) {
        self.persist_due = true;
    }

    /// THE STAMP A SNAPSHOT IS TAKEN AT: the head's height and `K`, read in
    /// the same hold as the snapshot, so a later note names what the
    /// written snapshot held (ACT1, 78.21(2)).
    pub fn snapshot_at(&self) -> (u64, u64) {
        (self.chain.head_height(), self.chain.head_k())
    }

    /// 78.124 — THE CORE CLOCK at this court: its head's stamp
    /// `h:S:G:T:H:K.i`, or `None` before its first act. What a kernel's step
    /// clock is held to.
    pub fn core_stamp(&self) -> Option<crate::chronometry::CoreStamp> {
        self.chain.head().map(|r| crate::chronometry::CoreStamp::of(&r))
    }

    /// 78.152 TU-1 — THIS COURT'S KEEPER TICK: its head's height, its
    /// cumulative issuance per lane (`Treasury::issued`, the one exact
    /// carrier of measured change booked), and `K`, the orbits it closed.
    /// `None` before its first act, or where the issuance passed its
    /// carrier.
    pub fn keeper_tick(&self) -> Option<crate::chronometry::tuniversal::Tick> {
        let at = self.chain.head_stamp()?;
        let lanes = self.treasury.issued()?;
        Some(crate::chronometry::tuniversal::Tick { h: at.h, lanes, k: self.chain.next_step_index() })
    }

    /// 78.152 TU-6 PINPOINT — every receipt this court holds at a final
    /// point's tick: the mesh at that tick as this ledger holds it. Empty
    /// where the tick is folded or never held.
    pub fn pinpoint(&self, p: &crate::chronometry::tuniversal::TPoint) -> Vec<crate::chronometry::KStepReceipt> {
        self.chain.at_tick(p.h)
    }

    /// 78.152 TU-6 UNPACK — a final point's coordinates by name, and the
    /// frame each receipt at its tick measured, per lane.
    pub fn unpack(
        &self,
        p: &crate::chronometry::tuniversal::TPoint,
    ) -> (crate::chronometry::tuniversal::ops::Unpacked, Vec<(crate::chronometry::Stamp, [u64; 4])>) {
        (p.unpack(), self.chain.at_tick(p.h).into_iter().map(|r| (r.stamp, r.mu)).collect())
    }

    /// 78.152 TU-6 UNROLL — this court's own hot line from its root up to a
    /// final point's tick, in line order, at most `n`: the era replay from a
    /// root to the point. The folded eras before the root are replayed by
    /// `ACT_RECEIPTS` (its replay arm) and are not read here.
    pub fn unroll(&self, p: &crate::chronometry::tuniversal::TPoint, n: usize) -> Vec<crate::chronometry::KStepReceipt> {
        self.chain.own().take_while(|r| r.stamp.h <= p.h).take(n).collect()
    }

    /// A snapshot taken at `at` was written whole. Called only after the
    /// write lands, so a write that failed records nothing.
    pub fn note_persisted(&mut self, at: (u64, u64)) {
        self.persisted = Some(at);
    }

    /// ACT1 — THIS COURT ON THE CLOCK (78.21, 78.27(1), EC1): its `K`, its
    /// head's stamp, the origin it adopted, per peer line the last height
    /// admitted and the gap to this head, where its snapshot and its last
    /// fold stand, and the three counts. Read at the question; nothing is
    /// stored beside the carriers but the counts the acts themselves keep.
    pub fn clock(&self, genesis: Option<([u8; 32], u32)>) -> crate::observe::CourtClock {
        let head = match self.chain.head() {
            Some(r) => {
                let [s, g, t, h] = r.mu;
                [r.stamp.h, s, g, t, h, r.stamp.k, r.stamp.i]
            }
            // Before genesis, or on a folded line whose head is the root.
            None => match self.chain.head_place() {
                Some(at) => [self.chain.head_height(), 0, 0, 0, 0, at.k, at.i],
                None => [0; 7],
            },
        };
        let own = self.chain.vk();
        let at = i64::try_from(head[0]).ok();
        let peers = self
            .chain
            .signers()
            .into_iter()
            .filter(|vk| *vk != own)
            .filter_map(|vk| {
                let h = self.chain.line_head(&vk)?.stamp.h;
                let gap = at?.checked_sub(i64::try_from(h).ok()?)?;
                Some((vk, h, gap))
            })
            .collect();
        crate::observe::CourtClock {
            k: self.chain.next_step_index(),
            head,
            genesis,
            peers,
            persisted: self.persisted,
            folded: self.chain.root().map(|r| (r.height, r.at.k)),
            unframed: self.unframed,
            orbits: crate::observe::Orbits { withheld: self.withheld, distance: self.chain.separation() },
            unstamped: self.unstamped(),
            rulings: self.rulings,
            ruling_refusals: self.ruling_refusals,
        }
    }

    /// 78.26(6), TP1 — the closed rings the glue reopened because a cover's
    /// gauge moved, `[filled, congruent]`.
    pub fn reopened(&self) -> [u64; 2] {
        self.reopened
    }

    /// RM1 — THE COURT'S REMAINDERS: every `Congruent` fill over its
    /// complexes, each at its complex's chart, and the re-fills; and the
    /// basis words carrying a torsion cycle `file_torsion` recorded (fibre
    /// spatial) — ℓ alone, κ and g dropped — to read against them.
    pub fn remainders(&self) -> (crate::transducer::cech::RemainderReading, u64) {
        let mut count = crate::transducer::cech::RemainderCount::default();
        for (chart, cx) in self.complexes.scoped() {
            for s in cx.congruent_standing() {
                count.add(&s, Some(&chart));
            }
            count.refilled(cx.refilled());
        }
        let torsion_words = self
            .basis
            .scoped()
            .filter(|(_, w)| w.element().torsion().iter().any(|t| t.fibre == crate::transducer::element::FIBER_SPATIAL))
            .count() as u64;
        let read = crate::transducer::cech::RemainderReading {
            refused: self.remainders_refused,
            past_carrier: self.remainders_past_carrier,
            misspoken: self.remainders_misspoken,
            unpriced: self.remainders_unpriced,
            unminted: self.remainders_unminted,
            unoffered: self.remainders_unoffered,
            coincident: self.remainders_coincident,
            refill_acts: self.refills.applied,
            refill_unshaped: self.refills.unshaped,
            refill_unlocated: self.refills.unlocated,
            refill_unspelled: self.refills.unspelled,
            refill_unopen: self.refills.unopen,
            refill_unsited: self.refills.unsited,
            refill_unfilled: self.refills.unfilled,
            ..count.read()
        };
        (read, torsion_words)
    }

    pub fn basis_len(&self) -> usize {
        self.basis.len()
    }

    /// MU-CLIENT §5.4 — WHAT THE COURT'S LANGUAGE HOLDS, BY KIND, in the order
    /// [`HELD_KINDS`] names: sentences (forms with faces), letters (forms
    /// with no face — a generator's carrier, citing nothing), compositions
    /// (forms with no face whose word cites held words — the dictionary's
    /// relations, D4), connections, colour words, rewrites, translations, and
    /// what is none of these. Read off each held word; nothing is counted
    /// where it is held.
    pub fn basis_kinds(&self) -> [usize; 8] {
        let mut out = [0usize; 8];
        for (_, held) in self.basis.scoped() {
            let e = held.element();
            let cites = || e.receipts().derivations.iter().any(|d| !d.generators.is_empty());
            let slot = match () {
                _ if e.is_rewrite() => 5,
                _ if e.is_translation() => 6,
                _ if e.is_colour_word() => 4,
                _ if e.is_connection() => 3,
                _ if e.is_sentence() => match (held.form().is_some_and(|f| !f.faces.is_empty()), cites()) {
                    (true, _) => 0,
                    (false, false) => 1,
                    (false, true) => 2,
                },
                _ => 7,
            };
            out[slot] += 1;
        }
        out
    }

    /// The root over the ruled cover. Measured here, where it is asked
    /// for, rather than rebuilt on every ruling (see `refresh_ruled_root`).
    pub fn ruled_root(&mut self) -> [u8; 32] {
        self.measure_ruled_root()
    }

    pub fn court_federation_len(&self) -> usize {
        self.court_federation.len()
    }

    /// The demand vertices listed so far (50.5, L7): a counter read at the
    /// wire and by the tests, derived from the listings, never a gate.
    pub fn demands_listed(&self) -> u64 {
        self.court_demands_listed
    }

    pub fn offered(&self, addr: &[u8]) -> bool {
        self.in_flight(addr)
    }

    pub fn dialect_len(&self) -> usize {
        self.cocycle_at.len()
    }

    pub fn dialect_chain_len(&self) -> usize {
        self.dialect_chain.len()
    }

    pub fn heard_len(&self) -> usize {
        self.heard.len()
    }

    pub fn field_hot_len(&self) -> usize {
        self.field.len()
    }

    pub fn field_cold_len(&self) -> usize {
        self.field_cold.len()
    }

    pub fn settlements_hot_len(&self) -> usize {
        self.settlements.len()
    }

    pub fn settlements_cold_len(&self) -> usize {
        self.settlements_cold.len()
    }

    /// STD1 — the room reads that did not answer: `(broken, past carrier)`.
    pub fn room_refusals(&self) -> (u64, u64) {
        use std::sync::atomic::Ordering::Relaxed;
        (self.room_broken.load(Relaxed), self.room_overflowed.load(Relaxed))
    }

    /// STD1 — the court's own operations and readings refused past the
    /// readings' carrier.
    pub fn overflowed(&self) -> u64 {
        self.overflowed
    }

}
