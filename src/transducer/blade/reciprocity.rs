// RX1 — THE TRANSPOSITION OF CONGRUENCE: a relation between two shapes is a
// transition only where it reads the same from both sides.
//
//! `T_AB` carries `a` onto `b` by `V`, fits through `m` and leaves the residue
//! `r_AB⁽ᵏ⁾ = h_B⁽ᵏ⁾ − Ad⁽ᵏ⁾_AB(h_A⁽ᵏ⁾)` above `m`. Read from `b`'s side, the
//! same relation is the reverse: `V_BA ∝ Ṽ_AB`, the same order, and — since
//! `Ad_BA ∘ Ad_AB` is the identity — the residue transposed,
//!
//! ```text
//!     r_BA⁽ᵏ⁾ = −Ad⁽ᵏ⁾_BA(r_AB⁽ᵏ⁾)
//! ```
//!
//! on the plane `Ad_BA` carries it onto. The reverse is the Clifford
//! reverse for rotor and reflection alike: an odd member's sandwich reverses
//! orientation by itself, so no parity is branched on. A pair that fails is
//! two relations, not one, and is refused by what failed.

use super::carry::Transition;
use super::continuous::PlaneCarriage;

/// WHY A PAIR IS NOT ONE RELATION (RX1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotReciprocal {
    /// No member of the reverse coset is the reverse of this transition.
    NoReverse,
    /// The two directions fit through different orders.
    OrderAsymmetric,
    /// The residue at this order does not transpose.
    Residue(super::tower::Harmonic),
}

/// RX1 — `ab` READ BACK FROM `b`'S SIDE: the member of `ba` (the coset from
/// `b` to `a`) that is `ab`'s reverse as a ray, where the two fit through one
/// order and every residue transposes onto the other's. `n` is the base's
/// axis count.
pub fn reciprocal<'a>(ab: &Transition, ba: &'a [Transition], n: usize) -> Result<&'a Transition, NotReciprocal> {
    let reverse = ab.versor.reverse();
    let back = ba.iter().find(|t| t.versor.projective_eq(&reverse)).ok_or(NotReciprocal::NoReverse)?;
    if back.through != ab.through {
        return Err(NotReciprocal::OrderAsymmetric);
    }
    // Each side's residues, transposed, are the other's: read both ways, so
    // a residue standing on one side alone is refused by its own order.
    transposes(ab, back, n)?;
    transposes(back, ab, n)?;
    Ok(back)
}

/// Every residue of `from`, carried by `to`'s transport and negated, is a
/// residue `to` holds on that plane at that order.
fn transposes(from: &Transition, to: &Transition, n: usize) -> Result<(), NotReciprocal> {
    for r in &from.residue {
        let carriage = PlaneCarriage::of(&to.rotation, &to.rho, n, r.plane).ok_or(NotReciprocal::Residue(r.order))?;
        let carried = carriage.carry(&r.vector, r.order);
        let expected = [-carried[0].clone(), -carried[1].clone()];
        if !to.residue.iter().any(|s| s.order == r.order && s.plane == carriage.image && s.vector == expected) {
            return Err(NotReciprocal::Residue(r.order));
        }
    }
    Ok(())
}
