// 13 — THE SIMPLEX: a simplex of the Čech complex, the transition on it,
// and the fill that closes it.
//
//! A transition is the element that carries one chart's section onto
//! another's; a holonomy is the composite around a triangle; a `Fill` is
//! what closes a simplex — a gauge, a class, or nothing.

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
/// the edge is honest and no simplex is formed over it.
pub fn rotor_of(g: &Element) -> Option<Rotor> {
    // THE ROTOR THE ELEMENT CARRIES, wherever it carries it. A transition
    // is `g_y ∘ g_x⁻¹` — a composite, which has no tape — so the reading
    // is of the element and never of a tape.
    g.carried_rotor()
}

/// THE SECTION AN ELEMENT CARRIES: its rotation AND its placement, read
/// from one [`Element::letters`] so the two halves are the same element's.
///
/// [`rotor_of`] answers the rotation alone, and a [`Section`] built from
/// that alone has the identity carriage whatever the element's placement
/// is: a Γ that is a translation, a dilation, or a rotation about any
/// point but the origin then reads as its rotor, and every STANDING class
/// at the court is invisible. The reader exists so the caller has the
/// choice.
pub fn section_of(g: &Element) -> Option<(Rotor, Versor)> {
    let letters = g.letters()?;
    Some((letters.spelling.word.rotor(), letters.carriage))
}

pub(crate) fn nonzero(v: &[u64; 4]) -> bool {
    v.iter().any(|&x| x > 0)
}

/// The key of a simplex: its sorted vertices, written out (A21).
pub(crate) fn simplex_key(verts: &[Vec<u8>]) -> crate::transducer::address::Address {
    let mut buf = vec![crate::transducer::address::TAG_SIMPLEX];
    buf.extend_from_slice(&(verts.len() as u32).to_le_bytes());
    for v in verts {
        crate::transducer::address::put_bytes(&mut buf, v);
    }
    crate::transducer::address::Address::new(buf)
}

pub(crate) fn sorted_verts(verts: impl IntoIterator<Item = impl AsRef<[u8]>>) -> Vec<Vec<u8>> {
    let mut verts: Vec<Vec<u8>> = verts.into_iter().map(|v| v.as_ref().to_vec()).collect();
    verts.sort_unstable();
    verts.dedup();
    verts
}

/// State of a simplex: open work, a coboundary gauge, a finite-order
/// congruence, a class at this degree, or a rotation the alphabet does not
/// spell.
#[derive(Clone, Debug)]
pub enum Fill {
    Open,
    /// The gauge that closed the simplex: its rotor and its carriage,
    /// because a transition is `D(ρ)·T(t)·R` and a gauge that is a pure
    /// placement has the identity rotor and is not the identity.
    Filled(Rotor, Box<Versor>),
    Congruent(Rotor, u32),
    /// A standing class: the class vector of 12.6 AND the rotor it was read
    /// from. Both, because they are different objects — the vector holds
    /// the rotor's CONJUGACY INVARIANTS together with the placement and
    /// dilation residues, and the rotor is the element, which those
    /// invariants do not determine. The horizon keys pivots by a rotor's
    /// class and cannot be served by the vector alone; law 12 is untouched,
    /// since neither is derivable from the other.
    Class(Box<crate::transducer::sentence::ClassVector>, Box<Rotor>),
    /// 37.7 AT A SIMPLEX: the transition acts on the sensor plane and the
    /// peel stands on a prime outside the seventeen. Not a class — a class
    /// is one the alphabet names and no held generator closes; this is one
    /// the alphabet cannot name, and it is where an adjoined generator
    /// comes from at the court.
    ///
    /// A rotor that acts on ANOTHER plane is not this: it is spelled by a
    /// letter of the seventeen and carries a class with no sensor section.
    Unspellable(Box<Rotor>),
    /// A fill attempted on a simplex that has already transitioned. Not
    /// stored on the simplex; the return of `fill` when there is no new
    /// state.
    Already,
}

/// CLASSIFY A TRANSITION (12.6). One reader for both organs: the court
/// reaches it through [`extend`], the voice through its cover's meets.
///
/// A transition is `D(ρ)·T(t)·R`, and it arrives here already separated —
/// the rotor in `Cl(4,0)` over all six planes, the carriage `D·T` in
/// `Cl(3,1)`. Nothing is projected and nothing is re-factored.
///
/// `Congruent` requires BOTH residues gone. A quarter turn carried along a
/// translation has the same rotor order as one about a fixed point and is
/// not the same class: filing it at period four would put a theorem on the
/// simplex that the seam does not satisfy, because the seam does not
/// return after four turns — it walks. A rotation about a point away from
/// the origin is `T_p R T_p⁻¹ = T(p − Rp)·R`, so it stands until the tree
/// gauge of [`solve_gauge`] conjugates the base-point translation away.
///
/// The 12.6 order of the standing class: the rotor's conjugacy invariants
/// first, then the placement and dilation residues, then the plane blocks.
pub fn classify(
    rotor: &Rotor,
    carriage: &Versor,
    hue: crate::transducer::sentence::CycleType,
) -> Fill {
    use crate::transducer::alphabet::Order;
    use crate::transducer::element::{identity_versor, same_versor};
    let flat = same_versor(carriage, &identity_versor());
    if flat && rotor.is_identity() {
        return Fill::Filled(rotor.clone(), Box::new(identity_versor()));
    }
    // A17: an order of zero is not an order — a congruence is believed only
    // where the rotation CLOSES at ℓ — so it is not filed as one, and every
    // reader of a filed period may take `ℓ − 1` whole.
    if let (true, Some(Order::Finite(ell @ 1..))) = (flat, rotor.order()) {
        return Fill::Congruent(rotor.clone(), ell);
    }
    // 37.7's condition, discriminated from an off-plane rotor rather than
    // conflated with it: the rotor DOES act on the sensor plane (it has a
    // block there) and the peel of that block stands, which is a prime
    // outside the seventeen. A rotor with no sensor block is not
    // unspellable — it is a letter acting elsewhere, and it carries a class
    // with no sensor section.
    let plane = (
        crate::transducer::alphabet::PLANE_ROW,
        crate::transducer::alphabet::PLANE_COL,
    );
    let acts_on_sensor = crate::transducer::alphabet::rotor_block_on(rotor, plane).is_some();
    if acts_on_sensor && crate::transducer::alphabet::word_of_rotor(rotor).is_none() {
        return Fill::Unspellable(Box::new(rotor.clone()));
    }
    match crate::transducer::sentence::ClassVector::of_rotor(rotor, carriage.clone(), hue) {
        Some(kappa) => Fill::Class(Box::new(kappa), Box::new(rotor.clone())),
        // A rotor with no conjugacy invariants is not a versor (15.3): no
        // class, and unmeasured rather than flat (rule 11).
        None => Fill::Open,
    }
}

/// Transition between two sections on a simplex, classified by
/// [`classify`].
///
/// Both halves of the transition are composed: the rotors in `Cl(4,0)` and
/// the carriages in `Cl(3,1)`. A Γ that is a translation or a dilation is
/// the carriage half, so a reading of the rotor alone takes it for the
/// identity, files finite rotor orders correctly and sees no standing
/// class at all.
///
/// THE HUE IS THE SECTION'S, AND A SECTION CARRIES NONE. `CycleType::of`
/// an empty listing is `Identity`, and a section holds a representative
/// word and its gauge with no hue map beside it, so the empty listing is
/// what this simplex actually has rather than a claim about it. A section
/// that should carry a gauge is a row, not a default written here.
pub fn extend(section_a: &Section, section_b: &Section, simplex: &Simplex) -> Fill {
    #[cfg(test)]
    crate::network::nerve::work::count_extend();
    use crate::transducer::element::{identity_versor, same_versor};
    let hue = crate::transducer::sentence::CycleType::of(&[]);
    let (Some(inv_r), Some(inv_c)) =
        (section_a.gamma.inverse(), section_a.carriage.versor_inverse())
    else {
        return Fill::Open;
    };
    let trans_r = section_b.gamma.geo(&inv_r);
    let trans_c = section_b.carriage.geo(&inv_c);
    if simplex.degree() <= 1 {
        let (Some(cr), Some(cc)) =
            (simplex.coefficient.inverse(), simplex.carriage.versor_inverse())
        else {
            return Fill::Open;
        };
        let (g_r, g_c) = (trans_r.geo(&cr), trans_c.geo(&cc));
        // The sections agree with what the simplex expected: the simplex
        // is extended BY THE TRANSITION, which is the gauge it was waiting
        // for.
        if g_r.is_identity() && same_versor(&g_c, &identity_versor()) {
            return Fill::Filled(trans_r, Box::new(trans_c));
        }
        return classify(&g_r, &g_c, hue);
    }
    classify(&simplex.coefficient, &simplex.carriage, hue)
}

/// One simplex of the Čech complex. Keyed by its sorted vertices, written
/// out (A21).
#[derive(Clone, Debug)]
pub struct Simplex {
    pub vertices: Vec<Vec<u8>>,
    /// The rotation the simplex expects, over all six planes of `Cl(4,0)`.
    pub coefficient: Rotor,
    /// The placement and dilation the simplex expects, `D(ρ)·T(t)` in
    /// `Cl(3,1)`. It is a SECOND field and not a wider coefficient: the
    /// conformal algebra of a two-dimensional base holds one rotation
    /// plane, so a versor coefficient would carry the residues and lose
    /// five of the rotor's planes — `cech/tests/bianchi` builds its
    /// degree-3 class from a rotor on `e₁∧e₂` that has no versor at all.
    pub carriage: Versor,
    pub volume: [u64; 4],
    pub fill: Fill,
}

/// The modules not yet swept name a simplex by the retired word.
pub type Cell = Simplex;

impl Simplex {
    /// A simplex on n vertices is an (n−1)-simplex. One on NONE is no
    /// simplex and is never remembered by the complex, so n ≥ 1 wherever
    /// this is asked and the subtraction is total.
    pub fn degree(&self) -> usize {
        self.vertices.len() - 1
    }

    pub fn key(&self) -> crate::transducer::address::Address {
        simplex_key(&self.vertices)
    }

    pub fn contains(&self, v: &[u8]) -> bool {
        self.vertices.iter().any(|x| x.as_slice() == v)
    }
}

/// Degree-0 section: the representative word after in-hypha gluing, and its gauge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    pub addr: crate::transducer::address::Address,
    /// The chart's rotation, over all six planes.
    pub gamma: Rotor,
    /// The chart's placement and dilation (L11e). `extend` composes both
    /// halves, and a Γ that is a translation, a dilation, or a rotation
    /// about any point but the origin is this one: without it the
    /// transition reads as its rotor and every STANDING class at the court
    /// is invisible, while finite rotor orders still file, since
    /// conjugation by a translation preserves the rotor class.
    pub carriage: Versor,
}

impl Section {
    /// A section at a rotation with no placement and no dilation.
    pub fn at(addr: crate::transducer::address::Address, gamma: Rotor) -> Section {
        Section { addr, gamma, carriage: crate::transducer::element::identity_versor() }
    }
}
