// 78.46 — LIFT AND DRAG: a neighbouring generator against an orbit's own
// mode blade, split exactly over ℚ into the part that commutes with the
// orbit and the part that anticommutes with it.
//
//! Inside one anastomosis a neighbour's circuit reaches the orbit through
//! the contact's transport (A.16). Its mode is a bivector blade `B_n`; the
//! orbit's own mode is `B_o`, a blade whose square is a non-zero scalar, so
//! conjugation by `B_o` is an involution on bivectors and every `B_n` splits
//! exactly:
//!
//! ```text
//!     B_∥ = ½ (B_n + B_o B_n B_o⁻¹)      B_o B_∥ =  B_∥ B_o      DRAG
//!     B_⊥ = ½ (B_n − B_o B_n B_o⁻¹)      B_o B_⊥ = −B_⊥ B_o      LIFT
//! ```
//!
//! DRAG acts in the orbit's own mode: the rate along it, never its plane.
//! LIFT is the commutator's content, `[B_o, B_n] = 2 B_o B_⊥`: a rotation in a
//! plane neither generator is, the tilt of the orbit's plane (60.4). Neither
//! is a scalar and no angle is formed; the split is verified before it is
//! returned, and refused where `B_o` is no blade with a non-zero square.

use super::multivector::Mv;
use num_rational::BigRational;
use num_traits::Zero;

/// A NEIGHBOUR'S GENERATOR SPLIT AGAINST AN ORBIT'S MODE (78.46(2)).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiftDrag {
    /// `B_∥`, commuting with `B_o`: the rate along the orbit.
    pub drag: Mv,
    /// `B_⊥`, anticommuting with `B_o`: the tilt of the orbit's plane.
    pub lift: Mv,
}

/// HOW TWO ORBITS ARE ALIGNED (78.46(3)): read off the pair of parts, never a
/// scalar. `Aligned` where the lift is zero (commuting modes), `Orthogonal`
/// where the drag is, `Both` in general.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alignment {
    Aligned,
    Orthogonal,
    Both,
    /// The neighbour's generator is zero: no push or pull at all.
    Still,
}

impl LiftDrag {
    /// The alignment the two parts read.
    pub fn alignment(&self) -> Alignment {
        match (self.drag.is_zero(), self.lift.is_zero()) {
            (true, true) => Alignment::Still,
            (false, true) => Alignment::Aligned,
            (true, false) => Alignment::Orthogonal,
            (false, false) => Alignment::Both,
        }
    }
}

/// LIFT AND DRAG of the neighbour's generator `b_n` against the orbit's mode
/// blade `b_o` — both read at grade 2. `None` where `b_o`'s square is not a
/// non-zero scalar (no involution to split by) or the split does not verify:
/// the parts must re-sum to `B_n`, the drag commute with `B_o` and the lift
/// anticommute, exactly (rule 11: refused, never approximated).
pub fn lift_drag(b_o: &Mv, b_n: &Mv) -> Option<LiftDrag> {
    let (b_o, b_n) = (b_o.grade(2), b_n.grade(2));
    let square = b_o.geo(&b_o);
    let s = square.scalar_part();
    if s.is_zero() || !square.sub(&Mv::scalar(s.clone())).is_zero() {
        return None;
    }
    // `B_o⁻¹ = B_o / B_o²`, the square a non-zero scalar.
    let inverse = b_o.scale(&(BigRational::from_integer(1.into()) / &s));
    let conjugate = b_o.geo(&b_n).geo(&inverse);
    let half = BigRational::new(1.into(), 2.into());
    let drag = b_n.add(&conjugate).scale(&half);
    let lift = b_n.sub(&conjugate).scale(&half);
    let commutes = b_o.geo(&drag).sub(&drag.geo(&b_o)).is_zero();
    let anticommutes = b_o.geo(&lift).add(&lift.geo(&b_o)).is_zero();
    let resums = drag.add(&lift).sub(&b_n).is_zero();
    (commutes && anticommutes && resums).then_some(LiftDrag { drag, lift })
}
