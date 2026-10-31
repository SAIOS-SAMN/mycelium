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

/// 78.123 C — A COCHAIN: the coefficient crossing `a → b`, read off whatever
/// holds it — a [`Cover`] of any group, or the minds' `Complex` in the versor
/// group — so a cycle's holonomy is written ONCE ([`holonomy`]) and every
/// complex reads its cycles by it.
pub trait Cochain<C: Coefficient> {
    /// The coefficient crossing `a → b`, or `None` where no edge stands (or
    /// its inverse passes its carrier).
    fn crossing(&self, a: &[u8], b: &[u8]) -> Option<C>;
}

impl<C: Coefficient> Cochain<C> for Cover<C> {
    fn crossing(&self, a: &[u8], b: &[u8]) -> Option<C> {
        self.edge(a, b)
    }
}

/// THE HOLONOMY around a cycle of any cochain — its vertices in order, the
/// last joined back to the first — the coefficients composed in the order
/// crossed.
pub fn holonomy<C: Coefficient>(x: &impl Cochain<C>, cycle: &[&[u8]]) -> Result<C, CycleUnread> {
    if cycle.len() < 2 {
        return Err(CycleUnread::Degenerate);
    }
    let mut acc = C::identity();
    for (i, from) in cycle.iter().enumerate() {
        let to = cycle[(i + 1) % cycle.len()];
        let g = x.crossing(from, to).ok_or_else(|| CycleUnread::EdgeAbsent { from: from.to_vec(), to: to.to_vec() })?;
        acc = acc.compose(&g).ok_or(CycleUnread::Carrier)?;
    }
    Ok(acc)
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
        holonomy(self, cycle)
    }

    /// A CELL READ BY THE COCYCLE CONDITION: every triangle among its
    /// vertices, in the vertices' coordinate order, closes — or the first that
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

/// 78.123 B — A GAUGE: per vertex, a parent and the coefficient carrying the
/// vertex to it, so every vertex reaches its component's root by one
/// composed coefficient — a POTENTIAL. An edge between two components joins
/// them, the gauge set so the edge holds; an edge inside one component
/// closes a cycle, and its holonomy is read against the gauge exactly — a
/// cycle of ANY length, found by the potential and never by walking the
/// graph (the pivot's gauge, `solve_gauge`, over any coefficient group).
#[derive(Clone, Debug)]
pub struct Gauge<C: Coefficient> {
    up: Keyed<(Vec<u8>, C)>,
}

impl<C: Coefficient> Default for Gauge<C> {
    fn default() -> Self {
        Gauge { up: Keyed::new() }
    }
}

/// WHAT AN EDGE DID TO THE GAUGE.
#[derive(Clone, Debug)]
pub enum Gauged<C> {
    /// It joined two components.
    Joined,
    /// It closed a cycle whose holonomy is the identity.
    Closed,
    /// It closed a cycle whose holonomy is not: the exact holonomy.
    Obstructed(C),
    /// A coefficient passed its carrier: nothing changed.
    Unread,
}

impl<C: Coefficient> Gauge<C> {
    /// The root `v` reaches and the coefficient carrying `v` there; a vertex
    /// the gauge does not hold is its own root.
    pub fn find(&self, v: &[u8]) -> Option<(Vec<u8>, C)> {
        let mut at = v.to_vec();
        let mut acc = C::identity();
        while let Some((parent, g)) = self.up.get(&at) {
            acc = acc.compose(g)?;
            at = parent.clone();
        }
        Some((at, acc))
    }

    /// THE EDGE `a → b` carrying `g`: joined, closed, or obstructed by the
    /// exact holonomy of the cycle it closes (`g`, then `b → root`, then
    /// `root → a`).
    pub fn edge(&mut self, a: &[u8], b: &[u8], g: &C) -> Gauged<C> {
        let (Some((ra, ga)), Some((rb, gb))) = (self.find(a), self.find(b)) else { return Gauged::Unread };
        if ra != rb {
            // root(a) → a → b → root(b): ga⁻¹, then g, then gb.
            let Some(join) = ga.inverse().and_then(|i| i.compose(g)).and_then(|x| x.compose(&gb)) else { return Gauged::Unread };
            self.up.insert(&ra, (rb, join));
            return Gauged::Joined;
        }
        match g.compose(&gb).and_then(|x| ga.inverse().and_then(|i| x.compose(&i))) {
            Some(h) if h.is_identity() => Gauged::Closed,
            Some(h) => Gauged::Obstructed(h),
            None => Gauged::Unread,
        }
    }

    /// Every held step: a vertex, its parent, the coefficient between.
    pub fn steps(&self) -> Vec<(Vec<u8>, Vec<u8>, C)> {
        self.up.scoped().map(|(v, (p, g))| (v, p.clone(), g.clone())).collect()
    }

    /// One step written back (a restore).
    pub fn hold_step(&mut self, v: &[u8], parent: Vec<u8>, g: C) {
        self.up.insert(v, (parent, g));
    }
}
