// A6, M4 — THE STALKS: every word this mind holds, filed by chart, and the
// refinement that re-admits a stratum under a coordinate that arrived.

use super::*;

/// The atlas over a relation of words (a basis, a federation, a genome):
/// each word's ADDRESS sits in the stratum over its chart. Every address
/// also carries its chart in a keyed relation, so a removal walks one path.
#[derive(Debug, Clone, Default)]
pub struct Stalks {
    root: Stratum,
    charts: Keyed<Invariant>,
}

impl Stalks {
    /// Chart a relation of words by each word's own invariant.
    pub fn over_elements<'a>(words: impl Iterator<Item = &'a element::Element>) -> Stalks {
        let mut s = Stalks::default();
        for w in words {
            s.admit(&invariant_of(w), w.address());
        }
        s
    }

    /// Place `addr` in the stratum over `inv`.
    pub fn admit(&mut self, inv: &Invariant, addr: impl AsRef<[u8]>) {
        let addr = addr.as_ref();
        self.charts.insert(addr, inv.clone());
        self.root.admit(&inv.0, addr);
    }

    /// A second chart for the same address (the remainder face). The base
    /// chart in `charts` is not overwritten.
    pub fn admit_also(&mut self, inv: &Invariant, addr: impl AsRef<[u8]>) {
        let addr = addr.as_ref();
        if self.charts.get(addr).is_some_and(|c| c == inv) {
            return;
        }
        self.root.admit(&inv.0, addr);
    }

    /// Retract `addr` from one stratum without touching its base chart.
    pub fn retract(&mut self, inv: &Invariant, addr: &[u8]) {
        if self.charts.get(addr).is_some_and(|c| c == inv) {
            return;
        }
        self.root.remove(&inv.0, addr);
    }

    /// The stratum over `inv`: every held address whose base point could
    /// be this one. The versor decides on these alone.
    pub fn fiber(&self, inv: &Invariant) -> Vec<Vec<u8>> {
        self.root.fiber(&inv.0)
    }

    /// Peel `inv` to a coarser chart (drop the deepest axis).
    pub fn peel(inv: &Invariant) -> Invariant {
        inv.peel()
    }

    /// Members at prefix depth `d` of `inv`. A read on the trie; chart-space
    /// neighbourhood is prefix depth, never a norm.
    pub fn fiber_at_depth(&self, inv: &Invariant, d: usize) -> Vec<Vec<u8>> {
        let n = d.min(inv.0.len());
        self.root.fiber(&inv.0[..n])
    }

    /// One-step outward charts from `inv`: the child axes of this node.
    pub fn child_charts(&self, inv: &Invariant) -> Vec<Invariant> {
        self.root
            .child_axes(&inv.0)
            .into_iter()
            .map(|a| {
                let mut axes = inv.0.clone();
                axes.push(a);
                Invariant(axes)
            })
            .collect()
    }

    // `each_child(inv, after, budget, f)` stood here: a budgeted page of one
    // stratum's child branches, resumed from a cursor, returning the last key
    // visited. It existed for `AtlasRead::children`, whose only caller was
    // `Bubble::expand_with`, which walked outward from an idea's chart to meet
    // whatever the trie held nearby. An idea's overlaps are the charts its own
    // vertices stand on, which are coordinates it already holds, so the
    // expansion reads `fiber` at those charts and this walk has no caller.
    // `fiber_under` below is the remaining trie collect, and it answers a
    // question about one chart rather than paging a stratum.

    /// Every address under `inv`, including finer charts. A trie collect.
    pub fn fiber_under(&self, inv: &Invariant) -> Vec<Vec<u8>> {
        self.root.fiber_under(&inv.0)
    }

    /// The chart a held address sits on.
    pub fn chart_of(&self, addr: &[u8]) -> Option<&Invariant> {
        self.charts.get(addr)
    }

    /// The relation removed the word at `addr`: one path, nothing shifts.
    pub fn remove(&mut self, addr: &[u8]) {
        if let Some(inv) = self.charts.remove(addr) {
            self.root.remove(&inv.0, addr);
        }
    }

    /// Whether this address is charted here.
    pub fn contains(&self, addr: &[u8]) -> bool {
        self.charts.contains(addr)
    }

    pub fn clear(&mut self) {
        self.root = Stratum::default();
        self.charts = Keyed::new();
    }

    pub fn len(&self) -> usize {
        self.root.len()
    }

    pub fn is_empty(&self) -> bool {
        self.root.len() == 0
    }

    /// Every populated stratum as (chart, members) — the open cover the
    /// sheaf reconciliation works over. Deterministic order (the trie's).
    pub fn strata(&self) -> Vec<(Invariant, Vec<Vec<u8>>)> {
        let mut out = Vec::new();
        self.root.strata(&mut Vec::new(), &mut out);
        out
    }

    /// Append class-key bytes as one axis (rewrite peel invert). Prefer
    /// [`refine_axes`] for A7.
    pub fn refine(inv: &Invariant, class_key: impl AsRef<[u8]>) -> Invariant {
        let class_key = class_key.as_ref();
        let mut b = [0u8; 8];
        let n = class_key.len().min(8);
        b[..n].copy_from_slice(&class_key[..n]);
        let coordinate = u64::from_le_bytes(b);
        Self::refine_axes(inv, &[ChartAxis::ConnectionClass(coordinate)])
    }

    /// Append exact axes (A7). Never a hash fragment.
    pub fn refine_axes(inv: &Invariant, axes: &[ChartAxis]) -> Invariant {
        let mut out = inv.0.clone();
        out.extend_from_slice(axes);
        Invariant(out)
    }

    /// Re-admit members of `inv` under the refined chart, splitting the
    /// stratum by the new axes. Prefix only: other charts are untouched.
    pub fn split_stratum(&mut self, inv: &Invariant, mut coordinate_of: impl FnMut(&[u8]) -> Vec<ChartAxis>) {
        let members = self.fiber(inv);
        for addr in members {
            let next = Self::refine_axes(inv, &coordinate_of(&addr));
            self.root.remove(&inv.0, &addr);
            self.root.admit(&next.0, &addr);
            self.charts.insert(&addr, next);
        }
    }
}
