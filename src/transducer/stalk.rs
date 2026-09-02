// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE ATLAS OF THE BASE MANIFOLD — charts on M = P/G for the bundle
//! P → M whose structure group is the rational conformal group
//! Spin(4,1)(ℚ).
//!
//! The primary object is the principal bundle: the total space P is the
//! observed cochains, the base M is their conformal orbit classes, and the
//! fibre over a class is the group itself — every placement, rotation,
//! reflection, and dilation is a point on the fibre above its class,
//! connected to the base representative by a versor V. Congruence is the
//! exact statement π(Q₁) = π(Q₂) ⟺ ∃V: Q₂ = V Q₁ Ṽ, and
//! [`super::element::transition`] solves that V in closed form.
//!
//! This module is only the atlas on M: to locate a cochain's base point
//! without enumerating the manifold, every support projects to its
//! SIMILARITY CHART (`blade::similarity_chart`): corner count, boundary
//! length over area, anisotropy, spread — exact rationals invariant under
//! the whole group, computed with no group enumeration. Different charts
//! imply different orbits. The chart is never the classifier: it only
//! narrows the candidate base points (an empty stratum is immediate
//! novelty), and the versor decides and is returned, never discarded. The
//! strata refine by axis — Vec-of-relations throughout; no map, no sorted
//! tape, no binary search over the manifold.

use super::element::{self, Versor};

/// A chain cell: (row, col, hue before, hue after).
type Cell = (i64, i64, u8, u8);

/// The chart of a support, flattened in structural order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invariant(pub Vec<u64>);

/// Measure the chart of a chain's support (hues are fibre coordinates over
/// the base point, not part of the geometry class).
pub fn invariant(chain: &[Cell]) -> Invariant {
    let mut cells = element::positions(chain);
    cells.sort_unstable();
    cells.dedup();
    if cells.is_empty() {
        return Invariant(Vec::new());
    }
    let shape = super::blade::shape_of(&cells);
    Invariant(super::blade::similarity_chart(&shape.blade))
}

/// One stratum of the atlas: the branches refine by the next axis of the
/// chart; the members are the indices whose chart ends here.
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

/// The atlas over an indexed relation of chains (a basis, a federation, a
/// genome): each index sits in the stratum over its support's chart.
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
    /// this one. The versor decides on these alone.
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

    /// Every populated stratum as (chart, members) — the open cover the
    /// sheaf reconciliation works over. Deterministic order (the trie's).
    pub fn strata(&self) -> Vec<(Invariant, Vec<usize>)> {
        let mut out = Vec::new();
        self.root.strata(&mut Vec::new(), &mut out);
        out
    }
}

/// Congruence lookup: the held chain (from `chains`, the relation the atlas
/// indexes) that is `chain`'s base point, with the versor carrying `chain`
/// onto it. The chart narrows; the versor decides and is returned.
pub fn congruent<'a>(
    stalks: &Stalks,
    chains: impl Fn(usize) -> &'a [Cell],
    chain: &[Cell],
) -> Option<(usize, Versor)> {
    let shape = element::shape_of_chain(chain)?;
    stalks
        .fiber(&invariant(chain))
        .iter()
        .copied()
        .find_map(|i| element::transition_from_shape(&shape, chains(i)).map(|v| (i, v)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::element::{from_residual, identity_versor};
    use crate::transducer::germinate::germinate;

    fn chain(cells: &[(i64, i64)]) -> Vec<Cell> {
        cells.iter().map(|&(r, c)| (r, c, 0u8, 1u8)).collect()
    }

    /// The chart is invariant under translation, rotation, reflection, and
    /// exact dilation, with no group enumeration.
    #[test]
    fn the_chart_is_invariant_under_the_conformal_group() {
        let l = chain(&[(0, 0), (1, 0), (2, 0), (2, 1)]);
        let base = invariant(&l);
        let moved = chain(&[(7, -3), (7, -2), (7, -1), (6, -1)]); // rotated + translated
        assert_eq!(invariant(&moved), base, "an isometry image shares the chart");
        let mirrored = chain(&[(0, 1), (1, 1), (2, 1), (2, 0)]);
        assert_eq!(invariant(&mirrored), base, "a reflection shares the chart");
        let big: Vec<Cell> = l
            .iter()
            .flat_map(|&(r, c, b, a)| {
                [(2 * r, 2 * c), (2 * r, 2 * c + 1), (2 * r + 1, 2 * c), (2 * r + 1, 2 * c + 1)]
                    .into_iter()
                    .map(move |(rr, cc)| (rr, cc, b, a))
            })
            .collect();
        assert_eq!(invariant(&big), base, "a dilation shares the chart");
    }

    /// Distinct orbits separate on structure: a square, a bar, and a ring.
    #[test]
    fn structure_separates_equal_volumes() {
        let square = chain(&[(0, 0), (0, 1), (1, 0), (1, 1)]);
        let bar = chain(&[(0, 0), (0, 1), (0, 2), (0, 3)]);
        let ring = chain(&[(0, 0), (0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1), (2, 2)]);
        assert_ne!(invariant(&square), invariant(&bar));
        assert_ne!(invariant(&square), invariant(&ring));
        assert_eq!(invariant(&square).0[0], 4, "four corners");
        assert_eq!(invariant(&ring).0[0], 8, "an outer and an inner loop of corners");
    }

    /// The lookup finds a rotated sibling's base point through its chart and
    /// returns the versor; an empty stratum is novelty; the atlas stays
    /// aligned after a removal.
    #[test]
    fn the_chart_finds_the_base_point_and_the_versor() {
        let held: Vec<Vec<Cell>> = vec![
            chain(&[(0, 0), (0, 1), (0, 2)]),         // 0: a bar
            chain(&[(0, 0), (0, 1), (1, 0)]),         // 1: an L
            chain(&[(0, 0), (0, 1), (1, 0), (1, 1)]), // 2: a square
        ];
        let mut stalks = Stalks::over(held.iter().map(|c| c.as_slice()));
        assert_eq!(stalks.len(), 3);
        let get = |i: usize| held[i].as_slice();
        let bar_rotated = chain(&[(5, 5), (6, 5), (7, 5)]);
        let (i, v) = congruent(&stalks, get, &bar_rotated).expect("the bar's orbit");
        assert_eq!(i, 0);
        assert!(!element::same_versor(&v, &identity_versor()), "a rotation carries the vertical bar onto the base");
        let l_reflected = chain(&[(3, 3), (3, 2), (4, 3)]);
        assert_eq!(congruent(&stalks, get, &l_reflected).map(|(i, _)| i), Some(1));
        let zigzag = chain(&[(0, 0), (0, 1), (1, 1), (1, 2)]);
        assert!(stalks.fiber(&invariant(&zigzag)).is_empty(), "an empty stratum is novelty");
        assert_eq!(congruent(&stalks, get, &zigzag), None);
        let held2: Vec<Vec<Cell>> = vec![held[0].clone(), held[2].clone()];
        stalks.remove(1);
        assert_eq!(stalks.len(), 2);
        let get2 = |i: usize| held2[i].as_slice();
        let sq = chain(&[(9, 9), (9, 10), (10, 9), (10, 10)]);
        assert_eq!(congruent(&stalks, get2, &sq).map(|(i, _)| i), Some(1));
        assert_eq!(congruent(&stalks, get2, &l_reflected), None);
    }

    /// Connecting the bundle: a rotated, differently-hued sibling attaches
    /// to the base as a section pulled back through V, the connection records
    /// (assignment, V), a second observation adds nothing, and the whole
    /// bundle round-trips its bytes.
    #[test]
    fn the_sibling_attaches_as_a_transported_section() {
        let mut base = from_residual(
            &germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap(),
        );
        let sibling = from_residual(
            &germinate(&[(0, 0, 3u8), (1, 0, 3)], &[(0, 0, 4u8), (1, 0, 4)]).unwrap(),
        );
        let v = element::transition(sibling.sub(), base.sub()).expect("congruent");
        assert_eq!(base.attach(&sibling, &v), 1);
        assert_eq!(base.sections().len(), 1, "the fibre gained a section");
        assert_eq!(base.transitions().len(), 1, "the connection holds (assignment, V)");
        assert_eq!(base.attach(&sibling, &v), 0, "already spoken");
        assert_eq!(base.transitions().len(), 1, "no duplicate relation");
        let bytes = base.to_bytes();
        let mut pos = 0usize;
        let back = crate::transducer::element::Element::from_bytes(&bytes, &mut pos).unwrap();
        assert_eq!(back.address(), base.address(), "the connection never renames the word");
        assert_eq!(back.transitions(), base.transitions());
        assert_eq!(back.sections(), base.sections());
        let mut bent = from_residual(
            &germinate(&[(0, 0, 1u8), (0, 1, 1), (1, 0, 1)], &[(0, 0, 2u8), (0, 1, 2), (1, 0, 2)])
                .unwrap(),
        );
        assert_eq!(bent.attach(&sibling, &v), 0, "a misfit records no connection");
        assert!(bent.transitions().is_empty());
    }

    /// The frame fiber: the same codomain chain at ρ = (2,2) and at ρ = (1,1)
    /// are different words (identity and substance carry ρ); the planar word
    /// keeps its identity bit-for-bit.
    #[test]
    fn the_frame_fiber_separates_scales_and_keeps_the_planar_word() {
        use crate::transducer::element::from_residual_framed;
        let g = germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap();
        let planar = from_residual(&g);
        let same = from_residual_framed(&g, &(3, 3, 3, 3));
        let scaled = from_residual_framed(&g, &(3, 3, 6, 6));
        assert_eq!(same.address(), planar.address(), "ρ = (1,1) is the planar word");
        assert_ne!(scaled.address(), planar.address(), "ρ = (2,2) is another word");
        assert_ne!(scaled.substance(), planar.substance(), "the substance carries ρ");
        assert_eq!(scaled.sub(), planar.sub(), "the support stays on the codomain");
    }
}
