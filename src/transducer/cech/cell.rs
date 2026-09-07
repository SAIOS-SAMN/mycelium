// 13 — THE CELL: a simplex of the Čech complex, the transition on it, and
// the fill that closes it.
//
//! A transition is the element that carries one chart's section onto
//! another's; a holonomy is the composite around a triangle; a `Fill` is
//! what closes a cell — a gauge, a class, or nothing.

use super::*;

/// The TRANSITION carrying chart X's view to chart Y's, from each chart's germ for
/// the shared object: `g_XY = g_Y ∘ g_X⁻¹` (the change of view). Antisymmetric by
/// construction — `g_YX = g_XY⁻¹`. `None` if X's germ is not invertible (a
/// coboundary/residual chart has no group view-change; that overlap is handled by
/// the additive coboundary, not this group transition).
pub fn transition(g_x: &Element, g_y: &Element) -> Option<Element> {
    Some(g_y.compose(&g_x.solution()?))
}

/// The HOLONOMY around a triangle of charts — the composite transition
/// `g_ab ∘ g_bc ∘ g_ca`. Identity ⇔ the charts GLUE (one sheaf); otherwise it is
/// the exact obstruction (the equivocation) the charts must reconcile.
pub fn holonomy(g_ab: &Element, g_bc: &Element, g_ca: &Element) -> Element {
    g_ab.compose(g_bc).compose(g_ca)
}

/// Does a triangle of charts GLUE — the Čech cocycle `δg = 0`? True iff the
/// holonomy around the overlap is trivial (the transitions are the coboundary of a
/// consistent global assignment). False = CURVATURE (δσ ≠ 0), a dialect.
pub fn glues(g_ab: &Element, g_bc: &Element, g_ca: &Element) -> bool {
    holonomy(g_ab, g_bc, g_ca).is_identity()
}

/// Realise a group transition as an alphabet rotor. None is a remainder:
/// the edge is honest and no cell is formed over it.
pub fn rotor_of(g: &Element) -> Option<Rotor> {
    if let Some(letters) = g.letters() {
        return Some(letters.spelling.word.rotor());
    }
    None
}

pub(crate) fn nonzero(v: &[u64; 4]) -> bool {
    v.iter().any(|&x| x > 0)
}

pub(crate) fn cell_key(verts: &[Vec<u8>]) -> crate::transducer::a21::Address {
    let mut buf = vec![crate::transducer::a21::TAG_CELL];
    buf.extend_from_slice(&(verts.len() as u32).to_le_bytes());
    for v in verts {
        crate::transducer::a21::put_bytes(&mut buf, v);
    }
    crate::transducer::a21::Address::new(buf)
}

pub(crate) fn sorted_verts(verts: impl IntoIterator<Item = impl AsRef<[u8]>>) -> Vec<Vec<u8>> {
    let mut verts: Vec<Vec<u8>> = verts.into_iter().map(|v| v.as_ref().to_vec()).collect();
    verts.sort_unstable();
    verts.dedup();
    verts
}

/// State of a cell: open work, a coboundary gauge, a finite-order congruence,
/// or a class at this degree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fill {
    Open,
    Filled(Rotor),
    Congruent(Rotor, u32),
    Class(Rotor),
    /// A fill attempted on a cell that has already transitioned. Not stored
    /// on the cell; the return of `fill` when there is no new state.
    Already,
}

pub(crate) fn word_of_rotor(r: &Rotor) -> crate::transducer::alphabet::Word {
    use crate::transducer::alphabet::{spell, Word};
    let one = crate::transducer::alphabet::Q::from_integer(num_bigint::BigInt::from(1));
    let zero = crate::transducer::alphabet::Q::from_integer(num_bigint::BigInt::from(0));
    let e0 = r.carry(&[one.clone(), zero.clone(), zero.clone(), zero.clone()]);
    let e1 = r.carry(&[zero.clone(), one, zero.clone(), zero]);
    if let (Some(a), Some(b)) = (e0, e1) {
        let block = [[a[0].clone(), b[0].clone()], [a[1].clone(), b[1].clone()]];
        if let Some(sp) = spell(&block) {
            return sp.word;
        }
    }
    Word::identity()
}

/// Transition between two sections on a cell: identity or coboundary is
/// Extended, finite Word::order is Congruent, otherwise Class.
pub fn extend(section_a: &Section, section_b: &Section, cell: &Cell) -> Fill {
    #[cfg(test)]
    crate::network::nerve::work::count_extend();
    let Some(inv) = section_a.gamma.inverse() else {
        return Fill::Open;
    };
    let trans = section_b.gamma.geo(&inv);
    if cell.degree() <= 1 {
        let Some(coeff_inv) = cell.coefficient.inverse() else {
            return Fill::Open;
        };
        let g = trans.geo(&coeff_inv);
        if g.is_identity() {
            return Fill::Filled(trans);
        }
        return match word_of_rotor(&g).order() {
            crate::transducer::alphabet::Order::Finite(ell) => Fill::Congruent(g, ell),
            crate::transducer::alphabet::Order::Infinite => Fill::Class(g),
        };
    }
    if cell.coefficient.is_identity() {
        return Fill::Filled(Rotor::identity());
    }
    match word_of_rotor(&cell.coefficient).order() {
        crate::transducer::alphabet::Order::Finite(ell) => Fill::Congruent(cell.coefficient.clone(), ell),
        crate::transducer::alphabet::Order::Infinite => Fill::Class(cell.coefficient.clone()),
    }
}

/// One cell of the Čech complex. Keyed by the hash of its sorted vertices.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub vertices: Vec<Vec<u8>>,
    pub coefficient: Rotor,
    pub volume: [u64; 4],
    pub fill: Fill,
}

impl Cell {
    pub fn degree(&self) -> usize {
        self.vertices.len().saturating_sub(1)
    }

    pub fn key(&self) -> crate::transducer::a21::Address {
        cell_key(&self.vertices)
    }

    pub fn contains(&self, v: &[u8]) -> bool {
        self.vertices.iter().any(|x| x.as_slice() == v)
    }
}

/// Degree-0 section: the representative word after in-hypha gluing, and its gauge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    pub addr: crate::transducer::a21::Address,
    pub gamma: Rotor,
}
