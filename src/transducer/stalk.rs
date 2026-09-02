// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE ATLAS OF THE BASE MANIFOLD — charts on M = P/G for the bundle
//! P → M whose structure group is the substrate isometry group ℤ² ⋊ D4.
//!
//! The primary object is the principal bundle: the total space P is the
//! observed cochains, the base M is their orbit classes, and the fibre over
//! a class is the group itself — every placement, rotation, and reflection
//! is a point on the fibre above its class, connected to the base
//! representative by a transition g ∈ G ([`super::element::Transition`]).
//! Congruence is the exact statement π(Q₁) = π(Q₂) ⟺ ∃g: Q₂ = g·Q₁, and
//! [`super::element::transition`] computes that g.
//!
//! This module is only the atlas on M: to locate a cochain's base point
//! without enumerating the manifold, every support projects to a
//! STRUCTURAL INVARIANT VECTOR — quantities strictly invariant under the
//! whole group, computed with no group enumeration:
//!
//!   β₀ (components), β₁ (holes of the cubical subcomplex), |V| cells,
//!   |E| 4-adjacencies, |F| filled unit squares, the unordered bounding-box
//!   extent, and the tally of (degree, diagonal-degree) cell types.
//!
//! Because the vector is G-invariant, every point of one fibre projects to
//! the same chart, and different vectors imply different orbits. The chart
//! is never the classifier: the vector only narrows the candidate base
//! points (an empty stratum is immediate novelty), and the group element
//! decides — a spectrum can be shared by non-isometric complexes, and a
//! spectrum carries no transport; the transition g is what the bundle
//! keeps. The strata refine by axis in structural order (topology, then
//! volume, then extent, then cell types) — Vec-of-relations throughout; no
//! map, no sorted tape, no binary search over the manifold.

use super::element::{self, Transition};
use super::spectral::find;

/// A chain cell: (row, col, hue before, hue after).
type Cell = (i64, i64, u8, u8);

/// The invariant vector of a support, flattened in structural order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invariant(pub Vec<u64>);

/// Measure the invariant vector of a chain's support (hues are not part of
/// the base geometry: the base point is the geometry class, the hues are
/// fibre coordinates over it).
pub fn invariant(chain: &[Cell]) -> Invariant {
    let mut cells: Vec<(i64, i64)> = chain.iter().map(|(r, c, _, _)| (*r, *c)).collect();
    cells.sort_unstable();
    cells.dedup();
    let n = cells.len();
    let held = |r: i64, c: i64| cells.binary_search(&(r, c)).is_ok();

    // β₀ over 4-adjacency, |E|, and per-cell degrees.
    let mut parent: Vec<usize> = (0..n).collect();
    let mut degree = vec![0u64; n];
    let mut diagonal = vec![0u64; n];
    let mut edges = 0u64;
    let mut faces = 0u64;
    for (i, &(r, c)) in cells.iter().enumerate() {
        for (dr, dc) in [(0i64, 1i64), (1, 0)] {
            if let Ok(j) = cells.binary_search(&(r + dr, c + dc)) {
                edges += 1;
                degree[i] += 1;
                degree[j] += 1;
                let (ri, rj) = (find(&mut parent, i), find(&mut parent, j));
                if ri != rj {
                    parent[ri] = rj;
                }
            }
        }
        for (dr, dc) in [(1i64, 1i64), (1, -1), (-1, 1), (-1, -1)] {
            if held(r + dr, c + dc) {
                diagonal[i] += 1;
            }
        }
        if held(r, c + 1) && held(r + 1, c) && held(r + 1, c + 1) {
            faces += 1;
        }
    }
    let mut roots: Vec<usize> = (0..n).map(|i| find(&mut parent, i)).collect();
    roots.sort_unstable();
    roots.dedup();
    let b0 = roots.len() as u64;
    // χ = V − E + F for the cubical subcomplex; β₂ = 0 in the plane, so
    // β₁ = β₀ − χ.
    let chi = n as i64 - edges as i64 + faces as i64;
    let b1 = (b0 as i64 - chi).max(0) as u64;

    // The unordered bounding-box extent: rotation swaps rows and columns,
    // so only the sorted pair is invariant.
    let (mut h, mut w) = (0u64, 0u64);
    if n > 0 {
        let rows = cells.iter().map(|(r, _)| *r);
        let cols = cells.iter().map(|(_, c)| *c);
        h = (rows.clone().max().unwrap() - rows.min().unwrap() + 1) as u64;
        w = (cols.clone().max().unwrap() - cols.min().unwrap() + 1) as u64;
    }
    let (ext_min, ext_max) = (h.min(w), h.max(w));

    // The tally of cell types (degree, diagonal degree) — a Vec-of-relations,
    // sorted so the vector is canonical.
    let mut tally: Vec<(u64, u64, u64)> = Vec::new();
    for i in 0..n {
        match tally.iter_mut().find(|(d, g, _)| *d == degree[i] && *g == diagonal[i]) {
            Some((_, _, k)) => *k += 1,
            None => tally.push((degree[i], diagonal[i], 1)),
        }
    }
    tally.sort_unstable();

    let mut v = vec![b0, b1, n as u64, edges, faces, ext_min, ext_max, tally.len() as u64];
    for (d, g, k) in tally {
        v.push(d);
        v.push(g);
        v.push(k);
    }
    Invariant(v)
}

/// One stratum of the atlas: the branches refine by the next axis of the
/// invariant vector; the members are the indices whose vector ends here.
#[derive(Debug, Clone, Default)]
struct Stratum {
    branches: Vec<(u64, Stratum)>,
    members: Vec<usize>,
}

impl Stratum {
    fn admit(&mut self, axes: &[u64], index: usize) {
        match axes.split_first() {
            None => self.members.push(index),
            Some((&a, rest)) => {
                let branch = match self.branches.iter().position(|(v, _)| *v == a) {
                    Some(p) => &mut self.branches[p].1,
                    None => {
                        self.branches.push((a, Stratum::default()));
                        &mut self.branches.last_mut().unwrap().1
                    }
                };
                branch.admit(rest, index);
            }
        }
    }

    fn fiber(&self, axes: &[u64]) -> &[usize] {
        match axes.split_first() {
            None => &self.members,
            Some((&a, rest)) => self
                .branches
                .iter()
                .find(|(v, _)| *v == a)
                .map(|(_, s)| s.fiber(rest))
                .unwrap_or(&[]),
        }
    }

    /// Drop the member at `index`; every higher index shifts down by one
    /// (the indexed relation removed an entry). Strata left empty are pruned.
    fn remove(&mut self, index: usize) {
        self.members.retain(|m| *m != index);
        for m in self.members.iter_mut() {
            if *m > index {
                *m -= 1;
            }
        }
        for (_, s) in self.branches.iter_mut() {
            s.remove(index);
        }
        self.branches.retain(|(_, s)| !s.members.is_empty() || !s.branches.is_empty());
    }

    fn len(&self) -> usize {
        self.members.len() + self.branches.iter().map(|(_, s)| s.len()).sum::<usize>()
    }

    fn strata(&self, path: &mut Vec<u64>, out: &mut Vec<(Invariant, Vec<usize>)>) {
        if !self.members.is_empty() {
            out.push((Invariant(path.clone()), self.members.clone()));
        }
        for (v, s) in &self.branches {
            path.push(*v);
            s.strata(path, out);
            path.pop();
        }
    }
}

/// The atlas over an indexed relation of chains (a basis, a federation):
/// each index sits in the stratum over its support's invariant.
#[derive(Debug, Clone, Default)]
pub struct Stalks {
    root: Stratum,
}

impl Stalks {
    /// Chart an existing relation (rehydration: the atlas is measured from
    /// the held words, never persisted).
    pub fn over<'a>(chains: impl Iterator<Item = &'a [Cell]>) -> Stalks {
        let mut s = Stalks::default();
        for (i, chain) in chains.enumerate() {
            s.admit(&invariant(chain), i);
        }
        s
    }

    /// Place `index` in the stratum over `inv`.
    pub fn admit(&mut self, inv: &Invariant, index: usize) {
        self.root.admit(&inv.0, index);
    }

    /// The stratum over `inv`: every held index whose base point could be
    /// this one. The group element decides on these alone.
    pub fn fiber(&self, inv: &Invariant) -> &[usize] {
        self.root.fiber(&inv.0)
    }

    /// The relation removed its entry at `index`.
    pub fn remove(&mut self, index: usize) {
        self.root.remove(index);
    }

    pub fn clear(&mut self) {
        self.root = Stratum::default();
    }

    pub fn len(&self) -> usize {
        self.root.len()
    }

    pub fn is_empty(&self) -> bool {
        self.root.len() == 0
    }

    /// Every populated stratum as (invariant, members) — the open cover the
    /// sheaf reconciliation works over. Deterministic order (the trie's).
    pub fn strata(&self) -> Vec<(Invariant, Vec<usize>)> {
        let mut out = Vec::new();
        self.root.strata(&mut Vec::new(), &mut out);
        out
    }
}

/// Congruence lookup: the held chain (from `chains`, the relation the atlas
/// indexes) that is `chain`'s base point, with the transport g carrying
/// `chain` onto it — g · supp(chain) = supp(held). The chart narrows; the
/// group element decides and is returned, never discarded.
pub fn congruent<'a>(
    stalks: &Stalks,
    chains: impl Fn(usize) -> &'a [Cell],
    chain: &[Cell],
) -> Option<(usize, Transition)> {
    stalks
        .fiber(&invariant(chain))
        .iter()
        .copied()
        .find_map(|i| element::transition(chain, chains(i)).map(|g| (i, g)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::element::{from_residual, transport, D4};
    use crate::transducer::germinate::germinate;

    fn chain(cells: &[(i64, i64)]) -> Vec<Cell> {
        cells.iter().map(|&(r, c)| (r, c, 0u8, 1u8)).collect()
    }

    /// The invariant is exactly that: every D4 image and every translation
    /// of a support measures the same vector, with no group enumeration.
    #[test]
    fn the_vector_is_invariant_under_the_isometry_group() {
        let l = chain(&[(0, 0), (1, 0), (2, 0), (2, 1)]);
        let base = invariant(&l);
        for g in D4 {
            let moved: Vec<Cell> = l
                .iter()
                .map(|&(r, c, b, a)| {
                    let (rr, cc) = g(r, c);
                    (rr + 7, cc - 3, b, a)
                })
                .collect();
            assert_eq!(invariant(&moved), base, "isometry image shares the vector");
        }
    }

    /// Distinct orbits with equal cardinality separate on structure: a
    /// filled square (F=1, β₁=0) against a ring (F=0, β₁=1) against a bar.
    #[test]
    fn topology_separates_equal_volumes() {
        let square = chain(&[(0, 0), (0, 1), (1, 0), (1, 1)]);
        let bar = chain(&[(0, 0), (0, 1), (0, 2), (0, 3)]);
        let ring = chain(&[(0, 0), (0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1), (2, 2)]);
        assert_ne!(invariant(&square), invariant(&bar));
        assert_eq!(invariant(&square).0[4], 1, "one filled face");
        assert_eq!(invariant(&ring).0[1], 1, "one hole");
        assert_eq!(invariant(&ring).0[0], 1, "one component");
        assert_eq!(invariant(&bar).0[1], 0, "a bar has no hole");
    }

    /// The lookup finds a rotated sibling's base point through its chart and
    /// returns the transport that carries it there; an empty stratum is
    /// novelty; the atlas stays aligned after a removal.
    #[test]
    fn the_chart_finds_the_base_point_and_the_transport() {
        let held: Vec<Vec<Cell>> = vec![
            chain(&[(0, 0), (0, 1), (0, 2)]),         // 0: a bar
            chain(&[(0, 0), (0, 1), (1, 0)]),         // 1: an L
            chain(&[(0, 0), (0, 1), (1, 0), (1, 1)]), // 2: a square
        ];
        let mut stalks = Stalks::over(held.iter().map(|c| c.as_slice()));
        assert_eq!(stalks.len(), 3);
        let get = |i: usize| held[i].as_slice();
        let bar_rotated = chain(&[(5, 5), (6, 5), (7, 5)]);
        let (i, g) = congruent(&stalks, get, &bar_rotated).expect("the bar's orbit");
        assert_eq!(i, 0);
        assert_ne!(g.d4, 0, "a rotation connects the vertical bar to the base");
        assert_eq!(
            element::positions(&transport(&bar_rotated, &g)),
            element::positions(&held[0]),
            "g carries the sibling onto the base"
        );
        let l_reflected = chain(&[(3, 3), (3, 2), (4, 3)]);
        assert_eq!(congruent(&stalks, get, &l_reflected).map(|(i, _)| i), Some(1));
        let zigzag = chain(&[(0, 0), (0, 1), (1, 1), (1, 2)]);
        assert!(stalks.fiber(&invariant(&zigzag)).is_empty(), "an empty stratum is novelty");
        assert_eq!(congruent(&stalks, get, &zigzag), None);
        // Remove the L: the square shifts down to index 1 and still resolves.
        let held2: Vec<Vec<Cell>> = vec![held[0].clone(), held[2].clone()];
        stalks.remove(1);
        assert_eq!(stalks.len(), 2);
        let get2 = |i: usize| held2[i].as_slice();
        let sq = chain(&[(9, 9), (9, 10), (10, 9), (10, 10)]);
        assert_eq!(congruent(&stalks, get2, &sq).map(|(i, _)| i), Some(1));
        assert_eq!(congruent(&stalks, get2, &l_reflected), None);
    }

    /// Connecting the bundle: a rotated, differently-hued sibling attaches
    /// to the base as a section transported by g, the connection records
    /// (assignment, g), a second observation of the same section under the
    /// same g adds nothing, and the whole bundle round-trips its bytes.
    #[test]
    fn the_sibling_attaches_as_a_transported_section() {
        let mut base = from_residual(
            &germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap(),
        );
        let sibling = from_residual(
            &germinate(&[(0, 0, 3u8), (1, 0, 3)], &[(0, 0, 4u8), (1, 0, 4)]).unwrap(),
        );
        let g = element::transition(sibling.sub(), base.sub()).expect("congruent");
        assert_eq!(base.attach(&sibling, &g), 1);
        assert_eq!(base.sections().len(), 1, "the fibre gained a section");
        assert_eq!(base.transitions().len(), 1, "the connection holds (assignment, g)");
        assert_eq!(base.attach(&sibling, &g), 0, "already spoken");
        assert_eq!(base.transitions().len(), 1, "no duplicate relation");
        let bytes = base.to_bytes();
        let mut pos = 0usize;
        let back = crate::transducer::element::Element::from_bytes(&bytes, &mut pos).unwrap();
        assert_eq!(back.address(), base.address(), "the connection never renames the word");
        assert_eq!(back.transitions(), base.transitions());
        assert_eq!(back.sections(), base.sections());
        // A base of different geometry refuses the transport.
        let mut bent = from_residual(
            &germinate(&[(0, 0, 1u8), (0, 1, 1), (1, 0, 1)], &[(0, 0, 2u8), (0, 1, 2), (1, 0, 2)])
                .unwrap(),
        );
        assert_eq!(bent.attach(&sibling, &g), 0);
        assert!(bent.transitions().is_empty(), "a misfit records no connection");
    }
}
