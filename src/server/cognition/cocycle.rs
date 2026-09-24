// 13, 23.7 — THE COCYCLES a mind has filed: one per chart simplex, keyed
// by the class it obstructs.
//
//! A cocycle is filed at the receipt that produced it, and a second filing
//! on the same simplex supersedes the first only when the incumbent
//! PRECEDES it on the chain (45.3); two that cannot be ordered are kept.

use super::*;

/// A STANDING COCYCLE: the transition between two voices' sections on one
/// chart. A newer receipt supersedes it; the dialect chain keeps the old,
/// and two that neither precede the other are held concurrent (45.3).
#[derive(Clone)]
pub struct CocycleRow {
    /// The gauge versor taking `a`'s section to `b`'s.
    pub gauge: element::Versor,
    /// The height it was written at — a reading, never the order itself:
    /// supersession is decided by the receipt chain, not by this number.
    pub t_universal: u64,
    /// The voice the transition is FROM.
    pub a: [u8; 32],
    /// The voice the transition is TO.
    pub b: [u8; 32],
    /// The rotor class the transport belongs to, stored beside the versor
    /// so a restore reads the class and never a hash of the row (CYC2).
    pub class_key: Address,
}

impl Kernel {
    pub(crate) fn cocycle_key(
        chart: &crate::transducer::stalk::Invariant,
        a: &[u8],
        b: &[u8],
    ) -> Vec<u8> {
        let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
        let mut buf = vec![crate::transducer::address::TAG_COCYCLE];
        crate::transducer::address::put_bytes(&mut buf, &chart.key());
        crate::transducer::address::put_bytes(&mut buf, lo);
        crate::transducer::address::put_bytes(&mut buf, hi);
        buf
    }

    /// One versor per (chart, pair), superseded by CAUSAL ORDER (45.3).
    /// The chain keeps every transition that ever stood.
    ///
    /// This compared `tu < old_tu`: a scalar order over the book (W1). Two
    /// transitions that are CONCURRENT — neither receipt's frontier below
    /// the other's — were ordered by a height comparison and one was
    /// silently discarded, and heights of two organs are exactly the
    /// quantity 45.3 says is partial. Now the incumbent is superseded only
    /// when it demonstrably PRECEDES the newcomer; a newcomer that precedes
    /// the incumbent does not supersede it; and an unordered or unmeasured
    /// pair is HELD — both stand on the dialect chain, neither is chosen
    /// (rule 15) — until a receipt that dominates both can order them.
    pub(crate) fn push_cocycle(
        &mut self,
        chart: &crate::transducer::stalk::Invariant,
        a: [u8; 32],
        b: [u8; 32],
        g: element::Versor,
        class_key: Address,
    ) {
        // A mind before its genesis stands at no receipt: the transition
        // stands with UNMEASURED order against every other.
        let at = self.chain.head_stamp();
        self.push_cocycle_at(chart, a, b, g, class_key, at);
    }

    /// The same, standing at a named receipt. A transition learned from a
    /// peer stands at the receipt the peer sealed it under, not at this
    /// mind's head — which is what makes two of them concurrent.
    pub(super) fn push_cocycle_at(
        &mut self,
        chart: &crate::transducer::stalk::Invariant,
        a: [u8; 32],
        b: [u8; 32],
        g: element::Versor,
        class_key: Address,
        at: Option<crate::chronometry::Stamp>,
    ) {
        if a == b {
            return;
        }
        let key = Self::cocycle_key(chart, &a, &b);
        let tu = self.height();
        if let Some(prev) = self.cocycle_at.get(&key).cloned() {
            let old = prev.gauge;
            if element::same_versor(&old, &g) {
                return;
            }
            let held = self.cocycle_receipt.get(&key).copied();
            let order = held.zip(at).and_then(|(h, at)| {
                let mut budget = self.physics.stroke();
                self.chain.precedes(&h, &at, &mut budget)
            });
            // 78.27 (5): each transition on the chain stands at the receipt
            // it stood at — the incumbent at its own, the newcomer at the
            // one it arrived under — never at a tick.
            match order {
                // The incumbent precedes: the newcomer supersedes it, and
                // the incumbent goes onto the chain that keeps what stood.
                Some(true) => self.dialect_chain.push((key.clone(), old, held)),
                // Ordered the other way, unordered, or unmeasured: the
                // newcomer is kept on the chain and the incumbent stands.
                // Nothing is discarded and nothing is chosen.
                _ => {
                    self.dialect_chain.push((key.clone(), g, at));
                    return;
                }
            }
        }
        match at {
            Some(at) => {
                self.cocycle_receipt.insert(&key, at);
            }
            None => {
                self.cocycle_receipt.remove(&key);
            }
        }
        self.cocycle_at.insert(
            key.clone(),
            CocycleRow { gauge: g, t_universal: tu, a, b, class_key },
        );
        let ck = chart.key();
        match self.cocycle_by_chart.get_mut(&ck) {
            Some(m) => {
                m.insert(key, ());
            }
            None => {
                let mut m = crate::network::nerve::Keyed::new();
                m.insert(key, ());
                self.cocycle_by_chart.insert(ck, m);
            }
        }
    }

    pub(crate) fn rekey_cocycles(
        &mut self,
        old: &crate::transducer::stalk::Invariant,
        new_a: &crate::transducer::stalk::Invariant,
        new_b: &crate::transducer::stalk::Invariant,
    ) {
        let ck = old.key();
        let Some(rows) = self.cocycle_by_chart.remove(&ck) else {
            return;
        };
        let mut saved: Vec<CocycleRow> = Vec::new();
        for (key, _) in rows.scoped() {
            if let Some(row) = self.cocycle_at.remove(&key) {
                let stood = self.cocycle_receipt.get(&key).copied();
                self.dialect_chain.push((key, row.gauge.clone(), stood));
                saved.push(row);
            }
        }
        for row in saved {
            let CocycleRow { gauge, a, b, class_key, .. } = row;
            self.push_cocycle(new_a, a, b, gauge.clone(), class_key.clone());
            self.push_cocycle(new_b, a, b, gauge, class_key);
        }
    }

}
