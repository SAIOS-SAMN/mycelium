// 42 — THE KERNEL'S READS: the small accessors a caller outside the
// book asks the court for. Split from `kernel.rs` at the §1.1 600-line
// reading bound when the awaiting docket was added (21 step 3); the
// items are moved, not rewritten (§1.6).

use super::*;

impl Kernel {
    pub(crate) fn mark_persist(&mut self) {
        self.persist_due = true;
    }

    pub fn take_persist(&mut self) -> bool {
        let due = self.persist_due;
        self.persist_due = false;
        due
    }

    /// THE STAMP A SNAPSHOT IS TAKEN AT: the head's height and `K`, read in
    /// the same hold as the snapshot, so a later note names what the
    /// written snapshot held (ACT1, 78.21(2)).
    pub fn snapshot_at(&self) -> (u64, u64) {
        (self.chain.head_height(), self.chain.head_k())
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
        self.offered.get(addr).is_some()
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

}
