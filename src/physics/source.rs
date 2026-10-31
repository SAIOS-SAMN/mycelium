// 78.75 — THE WHITE-HOLE SOURCE, AN ORGAN: the inversion of a refusal is a
// flow out of a point of the manifold's own, kept inside it (78.74, derived
// in `tests/capacitor.rs`, where it stood alone until now).
//
//! A GEOID `Σ` is the round a frame's pivots fix (`capacitor::shape_of`),
//! held as one vector `σ` — the dual of the pivots' outer product — so a
//! point lies on it by one scalar, `P·σ = 0`, and a motion keeps it exactly
//! where its generator commutes with `σ`.
//!
//! A REFUSAL IS A LIFT: two classes whose relative motion moves a pivot off
//! its fibre disagree, and the disagreement is a generator `L` that does
//! work. Read alone it is pressure — noise. Its INVERSION is a source:
//!
//! - the part of `L` that keeps the geoid, `K = ½(L + σLσ⁻¹)` — reflection
//!   in `σ` is the polarity switch, and what it leaves unchanged is the flow
//!   that stays inside;
//! - at a pivot `P` on `Σ`, the velocity `K` gives it, `v = P·K`;
//! - the null generator `N = P ∧ v`: `N² = 0`, it fixes `P` (the source) and
//!   it keeps `Σ` — a fascicle kissing at `P`, the white hole, whose slivers
//!   `1 + tN` compose by ADDING the slivers' parameters (78.75's null flow).
//!
//! Every claim is VERIFIED on the value before it is returned — null, fixes
//! `P`, keeps `σ` — and refused otherwise (rule 11): no source is asserted
//! that the algebra does not hold.

use crate::transducer::cga::{FlowPoint, Mv};
use num_rational::BigRational;
use num_traits::Zero;

type Q = BigRational;

/// THE GEOID AS ONE VECTOR: the dual of the pivots' round — its blade times
/// the pseudoscalar of its own algebra, grade one. `None` for a zero round or
/// a dual that is no vector.
pub fn geoid_vector(round: &Mv) -> Option<Mv> {
    if round.is_zero() {
        return None;
    }
    let pseudo = Mv::blade(crate::transducer::cga::pseudo(round.n())?, Q::from_integer(1.into()));
    let sigma = round.geo(&pseudo);
    let v = sigma.grade(1);
    (!v.is_zero() && sigma.sub(&v).is_zero()).then_some(v)
}

/// THE PART OF A GENERATOR THAT KEEPS THE GEOID: `½(L + σLσ⁻¹)`, the
/// generator's half that reflection in `σ` leaves unchanged — it commutes
/// with `σ`. `None` where `σ` has no inverse (a null vector: a point, not a
/// sphere).
pub fn keeping(sigma: &Mv, l: &Mv) -> Option<Mv> {
    let s2 = sigma.geo(sigma).scalar_part();
    if s2.is_zero() {
        return None;
    }
    let inverse = sigma.scale(&(Q::from_integer(1.into()) / s2));
    let mirrored = sigma.geo(&l.grade(2)).geo(&inverse).grade(2);
    Some(l.grade(2).add(&mirrored).scale(&Q::new(1.into(), 2.into())))
}

/// A WHITE-HOLE SOURCE: the point it emanates from and its null generator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Source {
    pub at: Mv,
    pub generator: Mv,
}

impl Source {
    /// THE SOURCE AT `p` ON THE GEOID `sigma`, pushed by `k`: `N = P ∧ (P·K)`,
    /// verified null, non-zero, fixing `P` and keeping `σ`. `None` where `P`
    /// is not on `Σ`, `K` does not move `P`, or any check fails.
    pub fn of(sigma: &Mv, p: &Mv, k: &Mv) -> Option<Source> {
        if !p.geo(sigma).scalar_part().is_zero() {
            return None;
        }
        let half = Q::new(1.into(), 2.into());
        let v = p.geo(k).sub(&k.geo(p)).grade(1).scale(&half);
        if v.is_zero() {
            return None;
        }
        let n = p.wedge_vector(1, &v);
        let null = !n.is_zero() && n.geo(&n).is_zero();
        let fixes = n.geo(p).sub(&p.geo(&n)).is_zero();
        let keeps = n.geo(sigma).sub(&sigma.geo(&n)).is_zero();
        (null && fixes && keeps).then(|| Source { at: p.clone(), generator: n })
    }

    /// THE SLIVER AT PARAMETER `t`: `1 + tN`, exact (`N² = 0`, so the flow is
    /// its own first order). Slivers compose by adding the slivers' parameters.
    pub fn sliver(&self, t: &Q) -> Mv {
        Mv::scalar(Q::from_integer(1.into())).add(&self.generator.scale(t))
    }

    /// The sliver as its point on the source's null flow.
    pub fn point(&self, t: &Q) -> Option<FlowPoint> {
        FlowPoint::at(&self.generator, t)
    }
}
