// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-CLIENT §4.3 — THE BREATH AND ITS PINCH. The breath has two axes (the
//! operator, 2026-10-21: "breath has two axis (if not more) — contract /
//! dilate (inhale / exhale): you are looking for torsion on 'pinch' of that
//! radial construct"; "pinch is not on the same axis: imagine an orthogonal
//! pinch"). Where a translation `P_a` meets a transversion `K_b` — what the
//! mind carries forward meeting what it carries back through the inversion —
//! their commutator generates both at once (78.39(1)):
//!
//! ```text
//!     [P_a, K_b]  ∝  ⟨a, b⟩ D  +  M_ab
//! ```
//!
//! - THE RADIAL AXIS, `⟨a, b⟩ D`: the dilation generator `D = e_∞ ∧ e_o`,
//!   scaled by what of `a` and `b` lies along each other — the contraction
//!   and the dilation, the inhale and the exhale;
//! - THE PINCH, `M_ab`: a rotation in the plane `a ∧ b` — what of `a` and
//!   `b` stands ORTHOGONAL to each other — at right angles to the radial
//!   axis, so it is read on either half of the breath. Its torsion is the
//!   pinch's TWIST.
//!
//! Formed in the algebra itself, never by the formula: the two generators are
//! `a e_∞` and `b e_o`, their commutator is taken by the geometric product,
//! and it is split exactly — its coefficient on `D` read by the product with
//! `D` (`D² = 1`), the pinch the rest of its bivector. No exponential: the
//! twist is the pinch's exact Cayley rotor (`cayley`), over ℚ. `a ∥ b`
//! breathes with no pinch; `a ⊥ b` pinches with no breath.

use super::embedding::{e_inf, e_o};
use super::multivector::{Mv, E1, E2, E3};
use num_rational::BigRational;
use num_traits::Zero;

type Q = BigRational;

/// ONE BREATH, SPLIT: its radial coefficient on `D = e_∞ ∧ e_o`, and its
/// pinch — the bivector orthogonal to the radial axis.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Breath {
    pub radial: Q,
    pub pinch: Mv,
}

impl Breath {
    /// THE PINCH'S TWIST: the exact rotor its bivector generates (the Cayley
    /// chart). `None` where there is no pinch — `a ∥ b`, or either vector
    /// zero — or the chart gives no versor.
    pub fn twist(&self) -> Option<Mv> {
        if self.pinch.is_zero() {
            return None;
        }
        super::spin::cayley(&self.pinch)
    }
}

/// A Euclidean vector of the sensor's axes, `e1, e2, e3` in order. `None` past
/// the three.
fn vector(x: &[Q]) -> Option<Mv> {
    let axes = [E1, E2, E3];
    if x.len() > axes.len() {
        return None;
    }
    Some(x.iter().zip(axes).fold(Mv::scalar(Q::zero()), |acc, (c, e)| acc.add(&Mv::blade(e, c.clone()))))
}

/// THE BREATH OF A TRANSLATION BY `a` MEETING A TRANSVERSION BY `b`: the
/// commutator of their generators, split into its radial coefficient and its
/// orthogonal pinch. `None` where a vector passes the sensor's axes, or the
/// commutator leaves its grade (it does not, for vectors of the sensor: the
/// split is read, and refused were it otherwise).
pub fn breath(a: &[Q], b: &[Q]) -> Option<Breath> {
    let (va, vb) = (vector(a)?, vector(b)?);
    let p = va.geo(&e_inf());
    let k = vb.geo(&e_o());
    let c = p.geo(&k).sub(&k.geo(&p));
    let bivector = c.grade(2);
    if !c.sub(&bivector).is_zero() {
        return None;
    }
    let d = e_inf().geo(&e_o()).grade(2);
    let d2 = d.geo(&d).scalar_part();
    if d2.is_zero() {
        return None;
    }
    let radial = bivector.geo(&d).scalar_part() / &d2;
    let pinch = bivector.sub(&d.scale(&radial));
    Some(Breath { radial, pinch })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(n: i64) -> Q {
        Q::from_integer(n.into())
    }

    /// THE PINCH IS THE ORTHOGONAL PLANE `a ∧ b`, and the radial axis the
    /// part of `a` and `b` along each other — read off the commutator, never
    /// assumed: the pinch is a multiple of the wedge, its square a negative
    /// scalar (a rotation's plane), and it carries no part of `D`.
    #[test]
    fn the_breath_splits_into_its_radial_axis_and_its_orthogonal_pinch() {
        let (a, b) = ([q(1), q(2), q(0)], [q(3), q(-1), q(2)]);
        let br = breath(&a, &b).expect("the sensor's vectors breathe");
        let wedge = vector(&a).expect("a").geo(&vector(&b).expect("b")).grade(2);
        let ratio = br.pinch.geo(&wedge.reverse()).scalar_part() / wedge.geo(&wedge.reverse()).scalar_part();
        assert_eq!(br.pinch, wedge.scale(&ratio), "the pinch lies in the plane a ∧ b");
        assert!(!ratio.is_zero(), "and is not nothing");
        let d = e_inf().geo(&e_o()).grade(2);
        assert!(br.pinch.geo(&d).sub(&d.geo(&br.pinch)).is_zero(), "the pinch commutes with D: orthogonal to the radial axis");
        let dot = a.iter().zip(&b).fold(q(0), |s, (x, y)| s + x * y);
        assert_eq!(br.radial.is_zero(), dot.is_zero(), "the radial axis is what of a and b lies along each other");
    }

    /// `a ∥ b` BREATHES WITH NO PINCH; `a ⊥ b` PINCHES WITH NO BREATH.
    #[test]
    fn parallel_breathes_and_orthogonal_pinches() {
        let along = breath(&[q(2), q(0), q(0)], &[q(3), q(0), q(0)]).expect("breathes");
        assert!(!along.radial.is_zero() && along.pinch.is_zero() && along.twist().is_none(), "parallel: radial only");
        let across = breath(&[q(1), q(0), q(0)], &[q(0), q(1), q(0)]).expect("breathes");
        assert!(across.radial.is_zero() && !across.pinch.is_zero(), "orthogonal: the pinch only");
        let twist = across.twist().expect("the pinch twists");
        assert!(crate::transducer::cga::is_versor(&twist), "its twist is an exact rotor");
    }

    /// THE RADIAL AXIS IS SIGNED: a translation and a transversion turned
    /// against each other breathe the other way, the pinch reversed with
    /// them.
    #[test]
    fn the_radial_axis_reverses_with_its_pair() {
        let (a, b) = ([q(1), q(1), q(0)], [q(2), q(0), q(1)]);
        let out = breath(&a, &b).expect("breathes");
        let back = breath(&a, &[q(-2), q(0), q(-1)]).expect("breathes");
        assert_eq!(back.radial, -out.radial.clone(), "the other half of the breath");
        assert_eq!(back.pinch, out.pinch.scale(&q(-1)), "its pinch reversed with it");
    }
}
