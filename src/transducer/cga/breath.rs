// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-CLIENT §4.3 — THE BREATH AND ITS PINCHES. The breath has two axes, if
//! not more (the operator, 2026-10-21: "breath has two axis (if not more) —
//! contract / dilate (inhale / exhale): you are looking for torsion on
//! 'pinch' of that radial construct"; "pinch is not on the same axis:
//! imagine an orthogonal pinch"; and the split "needs wider machinery" —
//! every plane of a motion, across motions, beyond three axes). Where two
//! generators meet — what the mind carries forward and what it carries back
//! through the inversion, or any two motions it holds — their commutator is
//! what they breathe (78.39(1)):
//!
//! ```text
//!     [P_a, K_b]  ∝  ⟨a, b⟩ D  +  M_ab          (the pair the spec names)
//!     [B₁, B₂]   =   radial · D  +  Σ planes     (any two generators)
//! ```
//!
//! - THE RADIAL AXIS: the commutator's coefficient on the dilation generator
//!   `D = e_∞ ∧ e_o` — the contraction and the dilation, inhale and exhale;
//! - THE PLANES: the rest of its bivector, split into its commuting simple
//!   planes at every exponent (`modes`, verified to re-sum), each read by
//!   its kind — an ELLIPTIC plane is a PINCH, a rotation orthogonal to the
//!   radial axis whose TWIST is its exact Cayley rotor; a PARABOLIC plane is
//!   a null carry; a HYPERBOLIC plane a boost.
//!
//! Formed in the algebra itself, never by a formula; read over every base
//! axis the algebra carries (`base_axis`, every dimension an exponent). No
//! exponential, no approximation: a commutator whose planes lie in an
//! extension of ℚ is refused by name.

use super::embedding::{e_inf, e_o};
use super::modes::{modes, Mode, Undecomposed};
use super::multivector::{base_axis, Mv};
use crate::network::nerve::coord::BivectorKind;
use num_rational::BigRational;
use num_traits::Zero;

type Q = BigRational;

/// WHY TWO GENERATORS DID NOT BREATHE, by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unbreathed {
    /// A vector past the axes the algebra's masks hold.
    Beyond,
    /// The commutator left the bivectors (it does not, for bivectors: the
    /// grade is read, and refused were it otherwise).
    Grade,
    /// Its planes do not split over ℚ, or the stroke ran out reading them.
    Unsplit(Undecomposed),
}

/// ONE BREATH, SPLIT: its radial coefficient on `D = e_∞ ∧ e_o`, the rest of
/// its bivector, and that rest's commuting planes by kind.
#[derive(Clone, Debug)]
pub struct Breath {
    pub radial: Q,
    pub rest: Mv,
    pub planes: Vec<Mode>,
}

impl Breath {
    /// THE PINCHES: the elliptic planes — rotations orthogonal to the radial
    /// axis.
    pub fn pinches(&self) -> impl Iterator<Item = &Mode> {
        self.planes.iter().filter(|m| m.kind == BivectorKind::Elliptic)
    }

    /// THE TWISTS: each pinch's exact Cayley rotor. A pinch the chart reads
    /// no versor for twists nothing here.
    pub fn twists(&self) -> Vec<Mv> {
        self.pinches().filter_map(|m| super::spin::cayley(&m.blade)).collect()
    }

    /// The planes of each kind, in the order elliptic, parabolic,
    /// hyperbolic.
    pub fn by_kind(&self) -> [u64; 3] {
        let mut out = [0u64; 3];
        for m in &self.planes {
            let i = match m.kind {
                BivectorKind::Elliptic => 0,
                BivectorKind::Parabolic => 1,
                BivectorKind::Hyperbolic => 2,
                // A trivial blade moves nothing: no plane.
                BivectorKind::Trivial => continue,
            };
            out[i] += 1;
        }
        out
    }
}

/// A Euclidean vector over the algebra's base axes, `x[i]` on axis `i + 1`.
/// `None` past the axes the masks hold.
fn vector(x: &[Q]) -> Option<Mv> {
    let mut v = Mv::scalar(Q::zero());
    for (i, c) in x.iter().enumerate() {
        v = v.add(&Mv::blade(base_axis(i + 1)?, c.clone()));
    }
    Some(v)
}

/// THE DILATION GENERATOR `D = e_∞ ∧ e_o`, the radial axis.
fn dilation() -> Mv {
    e_inf().geo(&e_o()).grade(2)
}

/// WHAT TWO GENERATORS BREATHE: their commutator, split into its radial
/// coefficient and its commuting planes by kind.
pub fn breathe(x: &Mv, y: &Mv, budget: &mut crate::physics::Budget) -> Result<Breath, Unbreathed> {
    let c = x.geo(y).sub(&y.geo(x));
    let bivector = c.grade(2);
    if !c.sub(&bivector).is_zero() {
        return Err(Unbreathed::Grade);
    }
    let d = dilation();
    let d2 = d.geo(&d).scalar_part();
    if d2.is_zero() {
        return Err(Unbreathed::Grade);
    }
    let radial = bivector.geo(&d).scalar_part() / &d2;
    let rest = bivector.sub(&d.scale(&radial));
    let planes = match rest.is_zero() {
        true => Vec::new(),
        false => modes(&rest, budget).map_err(Unbreathed::Unsplit)?,
    };
    Ok(Breath { radial, rest, planes })
}

/// THE BREATH OF A TRANSLATION BY `a` MEETING A TRANSVERSION BY `b` — the
/// pair the spec names: `breathe(a e_∞, b e_o)`, over every base axis.
pub fn breath(a: &[Q], b: &[Q], budget: &mut crate::physics::Budget) -> Result<Breath, Unbreathed> {
    let (va, vb) = (vector(a).ok_or(Unbreathed::Beyond)?, vector(b).ok_or(Unbreathed::Beyond)?);
    breathe(&va.geo(&e_inf()), &vb.geo(&e_o()), budget)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(n: i64) -> Q {
        Q::from_integer(n.into())
    }

    fn budget() -> crate::physics::Budget {
        crate::physics::Physics::default().stroke_n(1 << 16)
    }

    /// THE PINCH OF THE NAMED PAIR IS THE ORTHOGONAL PLANE `a ∧ b`, and the
    /// radial axis the part of `a` and `b` along each other — read off the
    /// commutator, never assumed: its one plane is elliptic, a multiple of
    /// the wedge, commuting with `D`.
    #[test]
    fn the_breath_splits_into_its_radial_axis_and_its_orthogonal_pinch() {
        let (a, b) = ([q(1), q(2), q(0)], [q(3), q(-1), q(2)]);
        let br = breath(&a, &b, &mut budget()).expect("the sensor's vectors breathe");
        let wedge = vector(&a).expect("a").geo(&vector(&b).expect("b")).grade(2);
        let ratio = br.rest.geo(&wedge.reverse()).scalar_part() / wedge.geo(&wedge.reverse()).scalar_part();
        assert_eq!(br.rest, wedge.scale(&ratio), "the rest lies in the plane a ∧ b");
        assert_eq!(br.by_kind(), [1, 0, 0], "one plane, elliptic: a pinch");
        let d = dilation();
        assert!(br.rest.geo(&d).sub(&d.geo(&br.rest)).is_zero(), "orthogonal to the radial axis");
        let dot = a.iter().zip(&b).fold(q(0), |s, (x, y)| s + x * y);
        assert_eq!(br.radial.is_zero(), dot.is_zero(), "the radial axis is what lies along each other");
    }

    /// `a ∥ b` BREATHES WITH NO PINCH; `a ⊥ b` PINCHES WITH NO BREATH.
    #[test]
    fn parallel_breathes_and_orthogonal_pinches() {
        let along = breath(&[q(2), q(0), q(0)], &[q(3), q(0), q(0)], &mut budget()).expect("breathes");
        assert!(!along.radial.is_zero() && along.planes.is_empty() && along.twists().is_empty(), "parallel: radial only");
        let across = breath(&[q(1), q(0), q(0)], &[q(0), q(1), q(0)], &mut budget()).expect("breathes");
        assert!(across.radial.is_zero() && across.by_kind() == [1, 0, 0], "orthogonal: one pinch");
        let twists = across.twists();
        assert!(twists.len() == 1 && crate::transducer::cga::is_versor(&twists[0]), "its twist is an exact rotor");
    }

    /// BEYOND THREE AXES: a pair reaching a fifth axis breathes there, its
    /// pinch in the plane the two span — no axis is the sensor's privilege.
    #[test]
    fn the_breath_reads_every_axis_the_algebra_carries() {
        let (a, b) = ([q(1), q(0), q(0), q(0), q(0)], [q(0), q(0), q(0), q(0), q(1)]);
        let br = breath(&a, &b, &mut budget()).expect("five axes breathe");
        assert!(br.radial.is_zero() && br.by_kind() == [1, 0, 0], "a pinch in the plane e1 ∧ e5");
        assert!(vector(&vec![q(1); usize::from(crate::transducer::cga::MAX_N) + 1]).is_none(), "past the masks, refused");
    }

    /// ACROSS MOTIONS: two turns in planes sharing one axis breathe a third
    /// plane — the commutator of two rotations is a rotation, a pinch, with no
    /// radial part; a turn with itself commutes and breathes nothing.
    #[test]
    fn two_motions_breathe_against_each_other() {
        use crate::transducer::cga::{E1, E2, E3};
        let plane = |i: usize, j: usize| Mv::blade(i | j, q(1));
        let br = breathe(&plane(E1, E2), &plane(E2, E3), &mut budget()).expect("two turns breathe");
        assert!(br.radial.is_zero() && br.by_kind() == [1, 0, 0], "the third plane, pinched");
        let commuting = breathe(&plane(E1, E2), &plane(E1, E2), &mut budget()).expect("a turn with itself");
        assert!(commuting.radial.is_zero() && commuting.planes.is_empty(), "a turn commutes with itself: no breath");
    }

    /// THE RADIAL AXIS IS SIGNED: a pair turned against each other breathes
    /// the other way, its planes reversed with it.
    #[test]
    fn the_radial_axis_reverses_with_its_pair() {
        let (a, b) = ([q(1), q(1), q(0)], [q(2), q(0), q(1)]);
        let out = breath(&a, &b, &mut budget()).expect("breathes");
        let back = breath(&a, &[q(-2), q(0), q(-1)], &mut budget()).expect("breathes");
        assert_eq!(back.radial, -out.radial.clone(), "the other half of the breath");
        assert_eq!(back.rest, out.rest.scale(&q(-1)), "its planes reversed with it");
    }
}
