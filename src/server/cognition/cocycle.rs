// 13, 23.7 — THE COCYCLES a mind has filed: one per chart simplex, keyed
// by the class it obstructs.
//
//! A cocycle is filed at the receipt that produced it, and a second filing
//! on the same simplex supersedes the first only when the incumbent
//! demonstrably PRECEDES it on the chain (45.3). Two that cannot be ordered
//! are both kept: causal order is partial, and a tie is not a vote.

use super::*;

/// A STANDING COCYCLE: the gauge versor between two charts, the k-step it
/// was stamped at, the two class keys it joins, and the address that
/// speaks it. A newer stamp supersedes; the chain keeps the old.
pub type CocycleRow = (element::Versor, u64, [u8; 32], [u8; 32], Address);

impl Kernel {
    pub(crate) fn cocycle_key(
        chart: &crate::transducer::stalk::Invariant,
        a: &[u8],
        b: &[u8],
    ) -> Vec<u8> {
        let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
        let mut buf = vec![crate::transducer::a21::TAG_COCYCLE];
        crate::transducer::a21::put_bytes(&mut buf, &chart.key());
        crate::transducer::a21::put_bytes(&mut buf, lo);
        crate::transducer::a21::put_bytes(&mut buf, hi);
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
        let at = self.chain.head_hash();
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
        at: [u8; 32],
    ) {
        if a == b {
            return;
        }
        let key = Self::cocycle_key(chart, &a, &b);
        let tu = self.height();
        if let Some((old, old_tu, _, _, _)) = self.cocycle_at.get(&key).cloned() {
            if element::same_versor(&old, &g) {
                return;
            }
            let held = self.cocycle_receipt.get(&key).copied();
            let order = held.and_then(|h| {
                let mut budget = self.physics.stroke();
                self.chain.precedes_hashes(&h, &at, &mut budget)
            });
            match order {
                // The incumbent precedes: the newcomer supersedes it, and
                // the incumbent goes onto the chain that keeps what stood.
                Some(true) => self.dialect_chain.push((key.clone(), old, old_tu)),
                // Ordered the other way, unordered, or unmeasured: the
                // newcomer is kept on the chain and the incumbent stands.
                // Nothing is discarded and nothing is chosen.
                _ => {
                    self.dialect_chain.push((key.clone(), g, tu));
                    return;
                }
            }
        }
        self.cocycle_receipt.insert(&key, at);
        self.cocycle_at.insert(key.clone(), (g, tu, a, b, class_key));
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
        for (key, _) in rows.fiber(crate::network::nerve::ROOT) {
            if let Some(row) = self.cocycle_at.remove(&key) {
                self.dialect_chain.push((key, row.0.clone(), row.1));
                saved.push(row);
            }
        }
        for (g, _, a, b, ck) in saved {
            self.push_cocycle(new_a, a, b, g.clone(), ck.clone());
            self.push_cocycle(new_b, a, b, g, ck);
        }
    }

}
