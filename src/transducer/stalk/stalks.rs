// A6, M4 — THE STALKS: every word this mind holds, filed by chart, and the
// refinement that re-admits a stratum under a coordinate that arrived.

use super::*;

/// The atlas over a relation of words (a basis, a federation, a genome):
/// each word's ADDRESS sits in the stratum over its chart. Every address
/// also carries its chart in a keyed relation, so a removal follows one path.
#[derive(Debug, Clone, Default)]
pub struct Stalks {
    root: Stratum,
    charts: Keyed<Invariant>,
}

impl Stalks {
    /// Z10 — the bytes this atlas reaches: its chart index and its whole
    /// stratum tree.
    pub fn bytes(&self) -> usize {
        self.charts.key_bytes()
            + self.charts.scoped().map(|(_, i)| i.key().as_bytes().len()).sum::<usize>()
            + self.root.bytes()
    }

    /// Chart a relation of words by each word's own invariant.
    pub fn over_elements<'a>(words: impl Iterator<Item = &'a element::Element>) -> Stalks {
        let mut s = Stalks::default();
        for w in words {
            s.admit(&invariant_of(w), w.address());
        }
        s
    }

    /// Place `address` in the stratum over `inv`.
    pub fn admit(&mut self, inv: &Invariant, address: impl AsRef<[u8]>) {
        let address = address.as_ref();
        self.charts.insert(address, inv.clone());
        self.root.admit(&inv.0, address);
    }

    /// A second chart for the same address (the remainder face). The base
    /// chart in `charts` is not overwritten.
    pub fn admit_also(&mut self, inv: &Invariant, address: impl AsRef<[u8]>) {
        let address = address.as_ref();
        if self.charts.get(address).is_some_and(|c| c == inv) {
            return;
        }
        self.root.admit(&inv.0, address);
    }

    /// Retract `address` from one stratum without touching its base chart.
    pub fn retract(&mut self, inv: &Invariant, address: &[u8]) {
        if self.charts.get(address).is_some_and(|c| c == inv) {
            return;
        }
        self.root.remove(&inv.0, address);
    }

    /// The stratum over `inv`: every held address whose base point could
    /// be this one. The versor decides on these alone.
    pub fn fibre(&self, inv: &Invariant) -> Vec<Vec<u8>> {
        self.root.fibre(&inv.0)
    }

    /// Peel `inv` to a coarser chart (drop the deepest axis).
    pub fn peel(inv: &Invariant) -> Invariant {
        inv.peel()
    }

    /// Members at prefix depth `d` of `inv`. A read on the trie; chart-space
    /// neighbourhood is prefix depth, never a norm.
    pub fn fibre_at_depth(&self, inv: &Invariant, d: usize) -> Vec<Vec<u8>> {
        let n = d.min(inv.0.len());
        self.root.fibre(&inv.0[..n])
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
    // `Bubble::expand_with`, which read outward from an idea's chart to meet
    // whatever the trie held nearby. An idea's overlaps are the charts its own
    // vertices stand on, which are coordinates it already holds, so the
    // expansion reads `fibre` at those charts and this outward read has no caller.
    // `fibre_under` below is the remaining trie collect, and it answers a
    // question about one chart rather than paging a stratum.

    /// Every address under `inv`, including finer charts. A trie collect.
    pub fn fibre_under(&self, inv: &Invariant) -> Vec<Vec<u8>> {
        self.root.fibre_under(&inv.0)
    }

    /// The chart a held address sits on.
    pub fn chart_of(&self, address: &[u8]) -> Option<&Invariant> {
        self.charts.get(address)
    }

    /// The relation removed the word at `address`: one path, nothing shifts.
    pub fn remove(&mut self, address: &[u8]) {
        if let Some(inv) = self.charts.remove(address) {
            self.root.remove(&inv.0, address);
        }
    }

    /// Whether this address is charted here.
    pub fn contains(&self, address: &[u8]) -> bool {
        self.charts.contains(address)
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

    /// 78.153 — every populated stratum with the COUNT of its members, on a
    /// stroke: one read per stratum visited, and no member's address copied
    /// (`strata` copies every one). `None` where the stroke ran out first.
    pub fn strata_counted(&self, budget: &mut crate::physics::Budget) -> Option<Vec<(Invariant, usize)>> {
        let mut out = Vec::new();
        self.root.strata_counted(&mut Vec::new(), &mut out, &mut || budget.spend()).then_some(out)
    }

    /// Append ONE opened axis as a coordinate (3.7, I6): the axis a
    /// rewrite opened, written out.
    ///
    /// It took an arbitrary key, TRUNCATED it to eight bytes and packed
    /// those into one `u64` — a hash fragment, which `refine_axes` says in
    /// its own line it must never be, and which no reader could invert.
    /// Its one caller already held a `u64` and was padding it into a
    /// thirty-two byte buffer for this reader to cut back down.
    pub fn refine_axis(inv: &Invariant, axis: u64) -> Invariant {
        Self::refine_axes(inv, &[ChartAxis::Count(axis)])
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
        let members = self.fibre(inv);
        for address in members {
            let next = Self::refine_axes(inv, &coordinate_of(&address));
            self.root.remove(&inv.0, &address);
            self.root.admit(&next.0, &address);
            self.charts.insert(&address, next);
        }
    }
}
