// 78.15(4), DF1 — THE DUAL FRAME OF A SET OF CONSTRUCTORS: each constructor's
// irreducible direction relative to the whole, built by product, and
// `descend`'s reader — it contracts the meet to its faces.
//
//! `s^a = (−1)^a (s₀ ∧ … ŝ_a … ∧ s_k) M⁻¹`, `M = s₀ ∧ … ∧ s_k`, `M⁻¹ = M̃ /
//! (M M̃)`: a product construction, no matrix inverse. `s^a · s_b = δ_ab`,
//! and `s^a · s^b` are the entries of `G⁻¹`. Read as a sphere, `s^a` is
//! orthogonal to every constructor but `a` — the POLAR of the face opposite
//! `a`. Contracting the meet by it, `s^a ⌋ M = ±` the face opposite `a`
//! (the wedge of the others): the meet released to its boundary, which is
//! `descend`'s shape, the exact inverse of `ascend` multiplying the new
//! parts out to the meet (AD1 on content).
//!
//! No dual frame where `M M̃ = 0`. Where the meet is ZERO the constructors are
//! dependent — REDUNDANCY — and every constructor whose removal restores a
//! non-zero wedge is named, none chosen (`A₂`'s three roots, all three).
//! Where the meet is not zero and its weight is, the span is DEGENERATE: a
//! null direction, the affine case.

use super::multivector::{one, Mv};
use num_traits::Zero;

/// THE DUAL FRAME, OR WHY THERE IS NONE.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DualFrame {
    /// `s^a`, one per constructor, in the constructors' order.
    Frame(Vec<Mv>),
    /// The meet is zero: the constructors whose removal restores a non-zero
    /// wedge, the minimal dependent set's members (every one named).
    Redundant(Vec<usize>),
    /// The meet stands and its weight is zero: a null direction in the span.
    Degenerate,
}

/// THE OUTER PRODUCT OF VECTORS, one at a time (grade by grade).
pub fn wedge(vs: &[Mv]) -> Mv {
    vs.iter().zip(1u32..).fold(Mv::scalar(one()), |w, (v, k)| w.geo(v).grade(k))
}

/// THE DUAL FRAME OF `constructors` (DF1).
pub fn dual_frame(constructors: &[Mv]) -> DualFrame {
    let m = wedge(constructors);
    let others = |a: usize| -> Vec<Mv> {
        constructors.iter().enumerate().filter(|&(b, _)| b != a).map(|(_, s)| s.clone()).collect()
    };
    if m.is_zero() {
        return DualFrame::Redundant(
            (0..constructors.len()).filter(|&a| !wedge(&others(a)).is_zero()).collect(),
        );
    }
    let weight = m.geo(&m.reverse()).scalar_part();
    if weight.is_zero() {
        return DualFrame::Degenerate;
    }
    let m_inv = m.reverse().scale(&(one() / weight));
    DualFrame::Frame(
        (0..constructors.len())
            .map(|a| {
                let sign = match a % 2 {
                    0 => one(),
                    _ => -one(),
                };
                wedge(&others(a)).geo(&m_inv).grade(1).scale(&sign)
            })
            .collect(),
    )
}

/// `s^a ⌋ M`: the meet contracted by one dual constructor — the face
/// opposite `a`, up to sign — `descend`'s release of the cell to its
/// boundary.
pub fn contract_to_face(dual: &Mv, meet: &Mv, grade: u32) -> Mv {
    match grade {
        0 => Mv::zero(),
        g => dual.geo(meet).grade(g - 1),
    }
}
