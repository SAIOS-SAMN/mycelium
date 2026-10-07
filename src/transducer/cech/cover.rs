// 78.123 A — A COVER GENERIC OVER ITS COEFFICIENT GROUP: vertices, the
// coefficient on each edge, the holonomy around any cycle, and a cell of any
// degree read by the cocycle condition on every triangle it holds.
//
//! The same reading the minds' complex makes of a cycle (13, 58.1 — the
//! transitions around a triangle compose to the identity, or the holonomy is
//! the exact obstruction), made over any [`Coefficient`] group: the versors,
//! or the additive lane group a domain's ledger tree carries (78.123). A cell
//! is filled exactly where every triangle among its vertices closes; one that
//! does not is OBSTRUCTED, named by that triangle and its holonomy — the
//! obstruction localized, never a whole tree walked to find it.
//!
//! An edge is held once, oriented by its vertices' coordinates (the lesser
//! first), and read back inverted when crossed the other way: the cochain is
//! antisymmetric by construction (`g_ba = g_ab⁻¹`).

use super::coefficient::Coefficient;
use crate::network::nerve::Keyed;

/// A COVER: the coefficient on each edge between its vertices.
#[derive(Clone, Debug)]
pub struct Cover<C: Coefficient> {
    edges: Keyed<C>,
}

impl<C: Coefficient> Default for Cover<C> {
    fn default() -> Self {
        Cover { edges: Keyed::new() }
    }
}

/// WHY A CYCLE WAS NOT READ.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CycleUnread {
    /// A step of the cycle crosses no edge the cover holds.
    EdgeAbsent { from: Vec<u8>, to: Vec<u8> },
    /// A composition or inverse passed its carrier (rule 11).
    Carrier,
    /// Fewer than two vertices: no cycle.
    Degenerate,
}

/// WHAT A CELL READS.
#[derive(Clone, Debug)]
pub enum CellReading<C> {
    /// Every triangle among its vertices closes: the cell glues.
    Filled,
    /// A triangle does not close — the first, in its vertices' order — and
    /// the exact holonomy around it: the obstruction, localized.
    Obstructed { triangle: [Vec<u8>; 3], holonomy: C },
    /// A cycle of it could not be read, and why.
    Unread(CycleUnread),
}

/// The edge's one key — its vertices' coordinates, the lesser first, each by
/// its length — and whether `a → b` runs that way.
fn edge_key(a: &[u8], b: &[u8]) -> (Vec<u8>, bool) {
    let forward = a <= b;
    let (lo, hi) = if forward { (a, b) } else { (b, a) };
    let mut k = Vec::with_capacity(lo.len() + hi.len() + 8);
    crate::transducer::address::put_key_count(&mut k, lo.len());
    k.extend_from_slice(lo);
    k.extend_from_slice(hi);
    (k, forward)
}

impl<C: Coefficient> Cover<C> {
    pub fn new() -> Self {
        Cover::default()
    }

    /// The coefficient `g` on the edge `a → b`; `None` where its inverse —
    /// held, when `b` is the lesser — passes its carrier.
    pub fn set_edge(&mut self, a: &[u8], b: &[u8], g: C) -> Option<()> {
        let (k, forward) = edge_key(a, b);
        let held = if forward { g } else { g.inverse()? };
        self.edges.insert(&k, held);
        Some(())
    }

    /// The coefficient crossing `a → b`: as held, or its inverse the other
    /// way; `None` where no edge stands or the inverse passes its carrier.
    pub fn edge(&self, a: &[u8], b: &[u8]) -> Option<C> {
        let (k, forward) = edge_key(a, b);
        let g = self.edges.get(&k)?;
        if forward {
            Some(g.clone())
        } else {
            g.inverse()
        }
    }

    /// THE HOLONOMY around a cycle — its vertices in order, the last joined
    /// back to the first — the coefficients composed in the order crossed.
    pub fn holonomy(&self, cycle: &[&[u8]]) -> Result<C, CycleUnread> {
        if cycle.len() < 2 {
            return Err(CycleUnread::Degenerate);
        }
        let mut acc = C::identity();
        for (i, from) in cycle.iter().enumerate() {
            let to = cycle[(i + 1) % cycle.len()];
            let g = self
                .edge(from, to)
                .ok_or_else(|| CycleUnread::EdgeAbsent { from: from.to_vec(), to: to.to_vec() })?;
            acc = acc.compose(&g).ok_or(CycleUnread::Carrier)?;
        }
        Ok(acc)
    }

    /// A CELL READ BY THE COCYCLE CONDITION: every triangle among its
    /// vertices, in their coordinates' order, closes — or the first that
    /// does not, with its holonomy. A cell of one edge is filled where the
    /// edge stands; a vertex alone, always.
    pub fn fill(&self, cell: &[Vec<u8>]) -> CellReading<C> {
        let mut v: Vec<&[u8]> = cell.iter().map(Vec::as_slice).collect();
        // Rule 4: ordered by the vertices' own bytes, so the triangles are
        // read in one order.
        v.sort_unstable();
        v.dedup();
        if let [a, b] = v.as_slice() {
            return match self.edge(a, b) {
                Some(_) => CellReading::Filled,
                None => CellReading::Unread(CycleUnread::EdgeAbsent { from: a.to_vec(), to: b.to_vec() }),
            };
        }
        for i in 0..v.len() {
            for j in i + 1..v.len() {
                for l in j + 1..v.len() {
                    match self.holonomy(&[v[i], v[j], v[l]]) {
                        Ok(h) if h.is_identity() => {}
                        Ok(h) => {
                            return CellReading::Obstructed { triangle: [v[i].to_vec(), v[j].to_vec(), v[l].to_vec()], holonomy: h }
                        }
                        Err(why) => return CellReading::Unread(why),
                    }
                }
            }
        }
        CellReading::Filled
    }
}
