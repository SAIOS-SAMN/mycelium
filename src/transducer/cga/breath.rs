// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-CLIENT §4.3 — THE BREATH AND ITS PINCHES. The breath has two axes, if
//! not more (ruled 2026-10-21: the breath contracts and dilates, and torsion
//! is sought on the pinch of that radial construct; the pinch lies on an
//! orthogonal axis, not the same one; and the split needs wider machinery —
//! every plane of a motion, across motions, beyond three axes). Where two
//! generators meet — what the mind carries forward and what it carries back
//! through the inversion, or any two motions it holds — the generators' commutator is
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
pub(super) fn vector(x: &[Q]) -> Option<Mv> {
    let mut v = Mv::scalar(Q::zero());
    for (i, c) in x.iter().enumerate() {
        v = v.add(&Mv::blade(base_axis(i + 1)?, c.clone()));
    }
    Some(v)
}

/// THE DILATION GENERATOR `D = e_∞ ∧ e_o`, the radial axis.
pub(super) fn dilation() -> Mv {
    e_inf().geo(&e_o()).grade(2)
}

/// WHAT TWO GENERATORS BREATHE: the generators' commutator, split into its radial
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
