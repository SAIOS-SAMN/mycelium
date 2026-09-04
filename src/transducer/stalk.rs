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
//! This module is only the atlas on M. A residual word is charted by its
//! support's MODULI (`blade::moduli`): components and holes, the boundary's
//! turning count, boundary length over area, anisotropy, spread, the fourth
//! moment, the hue fractions — exact scale-free rationals invariant under
//! the whole group, computed with no group enumeration. A CONNECTION has no
//! support: it is charted by the conjugacy class of its linear block and
//! the cycle type of its gauge permutation, on the element itself. The
//! moduli are coordinates, never a classifier: they name the stratum (an
//! empty stratum is immediate novelty), and the versor decides and is
//! returned, never discarded. A stratum holding two words a versor relates
//! to the same point is the measurement that the coordinate set is too
//! coarse: an axis to extend, never a first-wins pick. Members are
//! ADDRESSES — the atlas is a function of the words held, never of the
//! order they were held in — reached through a keyed trie, no map, no
//! sorted tape, no sibling scan.

use super::alphabet::Rotor;
use super::element::{self, Versor};
use crate::network::nerve::Keyed;

/// A chain cell: (row, col, hue before, hue after).
type Cell = (i64, i64, u8, u8);

/// The moduli of a support, in axis order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Invariant(pub Vec<u64>);

impl Invariant {
    /// Drop the deepest axis: the coarser chart this one refines.
    pub fn peel(&self) -> Invariant {
        let mut axes = self.0.clone();
        axes.pop();
        Invariant(axes)
    }

    /// Every prefix of this chart, finest first, including the empty chart.
    /// Chart-space distance is prefix depth, never a norm.
    pub fn prefixes(&self) -> Vec<Invariant> {
        let mut out = Vec::with_capacity(self.0.len() + 1);
        let mut cur = self.clone();
        loop {
            out.push(cur.clone());
            if cur.0.is_empty() {
                break;
            }
            cur = cur.peel();
        }
        out
    }

    /// A Keyed key for this chart: the little-endian axes hashed so a
    /// thought bag and a cover digest name the same plane without a map.
    pub fn key(&self) -> [u8; 32] {
        let mut h = blake3::Hasher::new();
        for a in &self.0 {
            h.update(&a.to_le_bytes());
        }
        *h.finalize().as_bytes()
    }
}

/// Closure of one chart a thought has been probed through. Never a number:
/// the loop closed (Flat), returned twisted (Twisted), a Čech class on the
/// nerve (Class — the same coefficient the cochains speak), or found no
/// section (Cavity). Magnitudes are read from remainders, never stored here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Closure {
    Flat,
    Twisted(Versor),
    /// A Čech class at a degree, a Rotor of the alphabet.
    Class(Rotor, u8),
    Cavity,
}

impl Closure {
    pub fn is_flat(&self) -> bool {
        matches!(self, Closure::Flat)
    }
}

/// Measure the moduli of a chain's support (hues are fibre coordinates over
/// the base point, not part of the geometry class).
pub fn invariant(chain: &[Cell]) -> Invariant {
    let mut cells = element::positions(chain);
    cells.sort_unstable();
    cells.dedup();
    if cells.is_empty() {
        return Invariant(Vec::new());
    }
    let shape = super::blade::shape_of(&cells);
    Invariant(super::blade::moduli(&shape.blade))
}

/// THE INVARIANT OF A WORD, read on its kind. A residual word (a chain) is
/// charted by its support's moduli. A connection is charted on its fibres:
/// the conjugacy class of the affine block's linear part (trace and
/// determinant, exact rationals packed as the moduli are) and the cycle
/// type of the gauge permutation. The translation part is the versor's
/// business on the fibre, exactly as position is quotiented for a residual
/// word: two identical motions at different origins share the chart. A
/// leading axis of 0 keeps connection charts disjoint from residual charts,
/// whose first axis is a component count of at least one. A word that is
/// neither — no support, no group component — has the empty chart:
/// novelty, not a stratum.
pub fn invariant_of(word: &element::Element) -> Invariant {
    if word.is_sentence() {
        return super::sentence::chart_of(word);
    }
    if word.is_connection() {
        return connection_chart(word).unwrap_or_default();
    }
    if !word.sub().is_empty() {
        return invariant(word.sub());
    }
    Invariant(Vec::new())
}

fn connection_chart(word: &element::Element) -> Option<Invariant> {
    let m = word.fiber(element::FIBER_SPATIAL)?;
    if m.len() != element::GERM_DIM {
        return None;
    }
    let trace = &m[0][0] + &m[1][1];
    let det = &m[0][0] * &m[1][1] - &m[0][1] * &m[1][0];
    let mut axes = vec![0u64];
    axes.extend_from_slice(&super::blade::axes_of(&trace));
    axes.extend_from_slice(&super::blade::axes_of(&det));
    // The gauge permutation's cycle type, read as the one 1 per hue row —
    // a row without it is not a permutation, and the chart is refused.
    let n = element::GERM_DIM - 3;
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let mut image = Vec::with_capacity(n);
    for r in 0..n {
        image.push((0..n).find(|&c| m[3 + r][3 + c] == one)?);
    }
    let mut seen = vec![false; n];
    let mut cycles: Vec<u64> = Vec::new();
    for start in 0..n {
        if seen[start] {
            continue;
        }
        let (mut i, mut len) = (start, 0u64);
        while !seen[i] {
            seen[i] = true;
            len += 1;
            i = image[i];
        }
        if len > 1 {
            cycles.push(len);
        }
    }
    cycles.sort_unstable();
    axes.push(cycles.len() as u64);
    axes.extend(cycles);
    Some(Invariant(axes))
}

/// One stratum of the atlas: the branches refine by the next axis of the
/// chart, a keyed relation over the axis value (read by its bits, never
/// scanned among siblings); the members are the addresses whose chart
/// ends here.
#[derive(Debug, Clone, Default)]
struct Stratum {
    branches: Keyed<Stratum>,
    members: Keyed<()>,
}

/// An axis value as a key: the coordinate's bytes, big-endian, so the
/// trie's order is the axis's order.
fn axis_key(a: u64) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[24..].copy_from_slice(&a.to_be_bytes());
    k
}

fn axis_of(key: &[u8; 32]) -> u64 {
    u64::from_be_bytes(key[24..].try_into().unwrap())
}

impl Stratum {
    fn admit(&mut self, axes: &[u64], addr: [u8; 32]) {
        match axes.split_first() {
            None => {
                self.members.insert(addr, ());
            }
            Some((&a, rest)) => {
                let key = axis_key(a);
                match self.branches.get_mut(&key) {
                    Some(s) => s.admit(rest, addr),
                    None => {
                        let mut s = Stratum::default();
                        s.admit(rest, addr);
                        self.branches.insert(key, s);
                    }
                }
            }
        }
    }

    fn fiber(&self, axes: &[u64]) -> Vec<[u8; 32]> {
        match axes.split_first() {
            None => self.members.entries().into_iter().map(|(k, _)| *k).collect(),
            Some((&a, rest)) => self.branches.get(&axis_key(a)).map(|s| s.fiber(rest)).unwrap_or_default(),
        }
    }

    fn is_empty(&self) -> bool {
        self.members.is_empty() && self.branches.is_empty()
    }

    /// Drop one member along its own chart path; strata left empty are pruned.
    fn remove(&mut self, axes: &[u64], addr: &[u8; 32]) {
        match axes.split_first() {
            None => {
                self.members.remove(addr);
            }
            Some((&a, rest)) => {
                let key = axis_key(a);
                let empty = {
                    let Some(s) = self.branches.get_mut(&key) else { return };
                    s.remove(rest, addr);
                    s.is_empty()
                };
                if empty {
                    self.branches.remove(&key);
                }
            }
        }
    }

    fn len(&self) -> usize {
        let mut n = self.members.len();
        self.branches.for_each(|_, s| n += s.len());
        n
    }

    fn strata(&self, path: &mut Vec<u64>, out: &mut Vec<(Invariant, Vec<[u8; 32]>)>) {
        if !self.members.is_empty() {
            let members: Vec<[u8; 32]> = self.members.entries().into_iter().map(|(k, _)| *k).collect();
            out.push((Invariant(path.clone()), members));
        }
        let kids = self.branches.entries();
        for (k, s) in kids {
            path.push(axis_of(k));
            s.strata(path, out);
            path.pop();
        }
    }

    fn child_axes(&self, axes: &[u64]) -> Vec<u64> {
        match axes.split_first() {
            None => {
                let mut out = Vec::new();
                self.branches.for_each(|k, _| out.push(axis_of(k)));
                out
            }
            Some((&a, rest)) => self
                .branches
                .get(&axis_key(a))
                .map(|s| s.child_axes(rest))
                .unwrap_or_default(),
        }
    }

    fn branches_at(&self, axes: &[u64]) -> Option<&Keyed<Stratum>> {
        match axes.split_first() {
            None => Some(&self.branches),
            Some((&a, rest)) => self.branches.get(&axis_key(a)).and_then(|s| s.branches_at(rest)),
        }
    }

    fn collect_members(&self, out: &mut Vec<[u8; 32]>) {
        self.members.for_each(|k, _| out.push(*k));
        self.branches.for_each(|_, s| s.collect_members(out));
    }

    fn fiber_under(&self, axes: &[u64]) -> Vec<[u8; 32]> {
        match axes.split_first() {
            None => {
                let mut out = Vec::new();
                self.collect_members(&mut out);
                out
            }
            Some((&a, rest)) => self
                .branches
                .get(&axis_key(a))
                .map(|s| s.fiber_under(rest))
                .unwrap_or_default(),
        }
    }
}

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
    pub fn admit(&mut self, inv: &Invariant, addr: [u8; 32]) {
        self.charts.insert(addr, inv.clone());
        self.root.admit(&inv.0, addr);
    }

    /// A second chart for the same address (the remainder face). The base
    /// chart in `charts` is not overwritten.
    pub fn admit_also(&mut self, inv: &Invariant, addr: [u8; 32]) {
        if self.charts.get(&addr).is_some_and(|c| c == inv) {
            return;
        }
        self.root.admit(&inv.0, addr);
    }

    /// Retract `addr` from one stratum without touching its base chart.
    pub fn retract(&mut self, inv: &Invariant, addr: &[u8; 32]) {
        if self.charts.get(addr).is_some_and(|c| c == inv) {
            return;
        }
        self.root.remove(&inv.0, addr);
    }

    /// The stratum over `inv`: every held address whose base point could
    /// be this one. The versor decides on these alone.
    pub fn fiber(&self, inv: &Invariant) -> Vec<[u8; 32]> {
        self.root.fiber(&inv.0)
    }

    /// Peel `inv` to a coarser chart (drop the deepest axis).
    pub fn peel(inv: &Invariant) -> Invariant {
        inv.peel()
    }

    /// Members at prefix depth `d` of `inv`. A read on the trie; chart-space
    /// neighbourhood is prefix depth, never a norm.
    pub fn fiber_at_depth(&self, inv: &Invariant, d: usize) -> Vec<[u8; 32]> {
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

    /// Budgeted one-level children of `inv`, resumed from `after` (a child
    /// invariant key). The last visited child key is returned as the cursor.
    pub fn each_child(
        &self,
        inv: &Invariant,
        after: Option<&[u8; 32]>,
        budget: usize,
        mut f: impl FnMut(Invariant),
    ) -> Option<[u8; 32]> {
        let branches = self.root.branches_at(&inv.0);
        let Some(branches) = branches else {
            return after.copied();
        };
        let last = branches.for_each_from(after, budget, |k, _| {
            let a = axis_of(k);
            let mut axes = inv.0.clone();
            axes.push(a);
            f(Invariant(axes));
        });
        last
    }

    /// Every address under `inv`, including finer charts. A trie collect.
    pub fn fiber_under(&self, inv: &Invariant) -> Vec<[u8; 32]> {
        self.root.fiber_under(&inv.0)
    }

    /// The chart a held address sits on.
    pub fn chart_of(&self, addr: &[u8; 32]) -> Option<&Invariant> {
        self.charts.get(addr)
    }

    /// The relation removed the word at `addr`: one path, nothing shifts.
    pub fn remove(&mut self, addr: &[u8; 32]) {
        if let Some(inv) = self.charts.remove(addr) {
            self.root.remove(&inv.0, addr);
        }
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
    pub fn strata(&self) -> Vec<(Invariant, Vec<[u8; 32]>)> {
        let mut out = Vec::new();
        self.root.strata(&mut Vec::new(), &mut out);
        out
    }

    /// Append one axis: the class key as a new coordinate of the chart.
    pub fn refine(inv: &Invariant, class_key: [u8; 32]) -> Invariant {
        let coordinate = u64::from_le_bytes(class_key[..8].try_into().unwrap());
        let mut axes = inv.0.clone();
        axes.push(coordinate);
        Invariant(axes)
    }

    /// Re-admit members of `inv` under the refined chart, splitting the
    /// stratum by the new coordinate. Prefix only: other charts are untouched.
    pub fn split_stratum(&mut self, inv: &Invariant, mut coordinate_of: impl FnMut(&[u8; 32]) -> u64) {
        let members = self.fiber(inv);
        for addr in members {
            let mut key = [0u8; 32];
            key[..8].copy_from_slice(&coordinate_of(&addr).to_le_bytes());
            let next = Self::refine(inv, key);
            self.root.remove(&inv.0, &addr);
            self.root.admit(&next.0, addr);
            self.charts.insert(addr, next);
        }
    }
}

/// A PLACEMENT: a transition between two connections that is a pure
/// translation — identity linear block, identity gauge, no chain — is the
/// same word placed elsewhere, and the translation is the versor carrying
/// one onto the other (a section of the base, never a new base). None when
/// the transition moves anything but the origin.
pub fn placement_of(g: &element::Element) -> Option<Versor> {
    if !g.sub().is_empty() {
        return None;
    }
    let m = g.fiber(element::FIBER_SPATIAL)?;
    if m.len() != element::GERM_DIM {
        return None;
    }
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let linear_identity = m[0][0] == one && m[1][1] == one && m[0][1].is_zero_q() && m[1][0].is_zero_q();
    let gauge_identity = (3..element::GERM_DIM).all(|r| m[r][r] == one);
    let other_axes = g.axes().iter().any(|&a| a != element::FIBER_SPATIAL);
    if !linear_identity || !gauge_identity || other_axes {
        return None;
    }
    Some(super::cga::translator(&m[0][2], &m[1][2]))
}

trait ZeroQ {
    fn is_zero_q(&self) -> bool;
}
impl ZeroQ for num_rational::BigRational {
    fn is_zero_q(&self) -> bool {
        num_traits::Zero::is_zero(self)
    }
}

/// CONGRUENCE on elements: the held word that is `spoken`'s base point,
/// with the versor carrying `spoken` onto it. For a residual word the chart
/// narrows the candidates and the versor between shapes decides; exactly
/// one relating word is a base point, several is the measurement that the
/// chart is too coarse (multiplicity — an axis to extend, returned as no
/// base, never a first-wins pick). For a connection the fibre's
/// representative (the first held connection there) is the chart, and the
/// Čech transition decides: the identity is the same word; anything else
/// is a dialect or a section, never a new base.
pub fn congruent<'a>(
    stalks: &Stalks,
    at: impl Fn(&[u8; 32]) -> Option<&'a element::Element>,
    spoken: &element::Element,
) -> Option<([u8; 32], Versor)> {
    let fibre = stalks.fiber(&invariant_of(spoken));
    if spoken.is_connection() {
        let held = fibre.iter().find_map(|a| at(a).filter(|w| w.address() != spoken.address() && !w.axes().is_empty()).map(|w| (*a, w)))?;
        let g = super::cech::transition(held.1, spoken)?;
        return match (g.axes().as_slice(), g.sub()) {
            ([], []) => Some((held.0, element::identity_versor())),
            _ => placement_of(&g).map(|v| (held.0, v)),
        };
    }
    let shape = element::shape_of_chain(spoken.sub())?;
    let mut related = fibre
        .iter()
        .filter_map(|a| at(a).filter(|w| !w.sub().is_empty()).map(|w| (*a, w)))
        .filter_map(|(a, w)| element::transition_from_shape(&shape, w.sub()).map(|v| (a, v)));
    let first = related.next()?;
    match related.next() {
        None => Some(first),
        Some(_) => None, // multiplicity: the chart wants another axis
    }
}

/// Congruence over pre-lifted shapes for a chain: `shape_at` yields a held
/// word's shape (lifted once per snapshot), so a lookup lifts only the
/// chain it asks about. Exactly one relating word is the base point.
pub fn congruent_shaped<'a>(
    stalks: &Stalks,
    shape_at: impl Fn(&[u8; 32]) -> Option<&'a super::blade::Shape>,
    chain: &[Cell],
) -> Option<([u8; 32], Versor)> {
    let shape = element::shape_of_chain(chain)?;
    let fibre = stalks.fiber(&invariant(chain));
    let mut related = fibre
        .iter()
        .filter_map(|a| shape_at(a).and_then(|held| super::blade::versor_between(&shape, held)).map(|v| (*a, v)));
    let first = related.next()?;
    match related.next() {
        None => Some(first),
        Some(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::element::{from_connection, from_residual, identity_versor, Element};
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
        assert_eq!(&invariant(&square).0[..3], &[1, 0, 4], "one body, no hole, four turns");
        assert_eq!(&invariant(&ring).0[..3], &[1, 1, 8], "one body, one hole, eight turns");
    }

    /// The lookup finds a rotated sibling's base point through its chart and
    /// returns the versor; an empty stratum is novelty; the atlas holds
    /// addresses, so a removal walks one path and nothing shifts.
    #[test]
    fn the_chart_finds_the_base_point_and_the_versor() {
        let held: Vec<Element> = vec![
            from_residual(&chain(&[(0, 0), (0, 1), (0, 2)])),         // a bar
            from_residual(&chain(&[(0, 0), (0, 1), (1, 0)])),         // an L
            from_residual(&chain(&[(0, 0), (0, 1), (1, 0), (1, 1)])), // a square
        ];
        let mut stalks = Stalks::over_elements(held.iter());
        assert_eq!(stalks.len(), 3);
        let at = |a: &[u8; 32]| held.iter().find(|w| w.address() == *a);
        let bar_rotated = from_residual(&chain(&[(5, 5), (6, 5), (7, 5)]));
        let (a, v) = congruent(&stalks, at, &bar_rotated).expect("the bar's orbit");
        assert_eq!(a, held[0].address());
        assert!(!element::same_versor(&v, &identity_versor()), "a rotation carries the vertical bar onto the base");
        let l_reflected = from_residual(&chain(&[(3, 3), (3, 2), (4, 3)]));
        assert_eq!(congruent(&stalks, at, &l_reflected).map(|(a, _)| a), Some(held[1].address()));
        let zigzag = from_residual(&chain(&[(0, 0), (0, 1), (1, 1), (1, 2)]));
        assert!(stalks.fiber(&invariant_of(&zigzag)).is_empty(), "an empty stratum is novelty");
        assert_eq!(congruent(&stalks, at, &zigzag), None);
        stalks.remove(&held[1].address());
        assert_eq!(stalks.len(), 2);
        let sq = from_residual(&chain(&[(9, 9), (9, 10), (10, 9), (10, 10)]));
        assert_eq!(congruent(&stalks, at, &sq).map(|(a, _)| a), Some(held[2].address()));
        assert_eq!(congruent(&stalks, at, &l_reflected), None);
    }

    /// A connection is charted on its fibres: the linear class and the
    /// gauge's cycle type, never a translation and never the empty chart;
    /// residual charts and connection charts are disjoint families.
    #[test]
    fn a_connection_is_charted_on_its_fibres_not_on_an_empty_chain() {
        let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
        let shift = |dr: i64, dc: i64| -> Vec<Vec<num_rational::BigRational>> {
            vec![vec![q(1), q(0), q(dr)], vec![q(0), q(1), q(dc)], vec![q(0), q(0), q(1)]]
        };
        let a = from_connection(&shift(0, 2), &[(1, 2), (2, 1)], &Vec::new(), [4, 2, 0, 0]).expect("spells");
        let b = from_connection(&shift(5, -3), &[(1, 2), (2, 1)], &Vec::new(), [4, 2, 0, 0]).expect("spells");
        let c = from_connection(&shift(0, 2), &[(1, 2), (2, 3), (3, 1)], &Vec::new(), [4, 2, 0, 0]).expect("spells");
        assert!(!invariant_of(&a).0.is_empty(), "a connection never lands on the empty chart");
        assert_eq!(invariant_of(&a), invariant_of(&b), "two identical motions at different origins share the chart");
        assert_ne!(invariant_of(&a), invariant_of(&c), "a different cycle type is a different chart");
        assert_eq!(invariant_of(&a).0[0], 0, "the connection family's leading axis");
        let word = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
        assert!(invariant_of(&word).0[0] >= 1, "a residual chart leads with its component count");
        // Congruence on connections: the fibre's representative and the transition.
        let stalks = Stalks::over_elements([&a, &c].into_iter());
        let at = |x: &[u8; 32]| [&a, &c].into_iter().find(|w| w.address() == *x);
        assert_eq!(congruent(&stalks, at, &b).map(|(x, _)| x), Some(a.address()), "b is a's word, placed elsewhere");
        assert_eq!(congruent(&stalks, at, &c), None, "c has no other word on its chart");
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
        assert_eq!(base.attach(&sibling, &v), 0, "a second observation adds nothing");
        let bytes = base.to_bytes();
        let mut pos = 0usize;
        let back = Element::from_bytes(&bytes, &mut pos).expect("round trip");
        assert_eq!(back, base, "the bundle round-trips");
    }

    /// A class refines one axis; members re-home under the new prefix.
    #[test]
    fn refine_splits_the_stratum_by_the_new_coordinate() {
        let a = from_residual(&chain(&[(0, 0), (0, 1), (0, 2)]));
        let b = from_residual(&vec![(0, 0, 2, 3), (0, 1, 2, 3), (0, 2, 2, 3)]);
        let mut stalks = Stalks::over_elements([&a, &b].into_iter());
        let inv = invariant_of(&a);
        assert_eq!(stalks.fiber(&inv).len(), 2);
        let class_coord = 7u64;
        stalks.split_stratum(&inv, |addr| {
            if *addr == a.address() {
                0
            } else {
                class_coord
            }
        });
        assert!(stalks.fiber(&inv).is_empty(), "the coarse chart is empty after the split");
        let mut zero = [0u8; 32];
        zero[..8].copy_from_slice(&0u64.to_le_bytes());
        let mut cls = [0u8; 32];
        cls[..8].copy_from_slice(&class_coord.to_le_bytes());
        assert_eq!(stalks.fiber(&Stalks::refine(&inv, zero)), vec![a.address()]);
        assert_eq!(stalks.fiber(&Stalks::refine(&inv, cls)), vec![b.address()]);
        let peeled = Stalks::peel(&Stalks::refine(&inv, cls));
        assert_eq!(peeled, inv, "peel drops the class axis");
        assert_eq!(inv.prefixes().last().map(|p| p.0.is_empty()), Some(true));
    }
}
