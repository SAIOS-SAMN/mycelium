// 78.75 — THE FLOW: a motion read as where it stands on its own one-parameter
// flow, never as a step between two samples (ruled
// 2026-10-03: a continuous motion is never read in steps).
//
//! Every mode of a motion lies on the flow its plane generates. On one simple
//! plane `B`, with `k = B²` a rational scalar, the mode is the ray
//!
//! ```text
//!     a + b·B          (a, b) ∈ ℚ², read projectively
//! ```
//!
//! and two points of one flow compose by the geometric product, in the plane,
//! exactly:
//!
//! ```text
//!     (a₁ + b₁B)(a₂ + b₂B) = (a₁a₂ + k·b₁b₂) + (a₁b₂ + a₂b₁)·B
//! ```
//!
//! ONE LAW FOR EVERY KIND, THE METRIC'S SIGN DECIDING. Writing `t = b/a`:
//!
//! ```text
//!     t₁ ⊕ t₂ = (t₁ + t₂) / (1 + k·t₁t₂)
//! ```
//!
//! — the turn's tangent addition where `k < 0`, the boost's where `k > 0`, and
//! where `k = 0` (a null plane: the translation, and the white-hole source
//! kissing at a finite point) plain addition: the slivers, `1/7 ⊕ 2/5 = 19/35`.
//! No angle is formed, no root is taken, no frame is a unit: `t` is where the
//! motion stands on its flow, and a frame is only where it was observed.
//!
//! THE POLE IS HELD, NOT REFUSED. `a = 0` is the 180° turn — `t` infinite, the
//! Cayley chart's pole — and stays the point `(0 : 1)`; composition never
//! divides.
//!
//! RETURN AND NEARNESS, EXACT. The flow has returned where `b = 0`. How far it
//! stands from its return is read off the action's value on the plane,
//! `c = (a² + k·b²)/(a² − k·b²)` — `cos` of the turn, `cosh` of the boost, the
//! value MR1 reads off `(R + R⁻¹)/2` — as `|1 − c|`, a rational; on a null
//! plane `c = 1` always and the distance is `|t|` against the plane's own
//! reference generator.

use super::multivector::{one, Mv};
use num_rational::BigRational;
use num_traits::{Signed, Zero};

type Q = BigRational;

/// A POINT ON ONE PLANE'S FLOW: the plane's reference generator (a simple
/// bivector, any scale), its square `k`, and the point `a + b·B` on it, held
/// projectively — `(a, b)` and `(λa, λb)` are one point.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowPoint {
    plane: Mv,
    k: Q,
    a: Q,
    b: Q,
}

/// WHAT KIND OF FLOW a plane carries, read off the sign of its square.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowKind {
    /// `B² < 0`: it turns, and can return.
    Turn,
    /// `B² > 0`: it boosts, and never returns.
    Boost,
    /// `B² = 0`: a null flow — a translation or a source at a finite point —
    /// whose parameter adds.
    Null,
}

impl FlowPoint {
    /// THE POINT `t` ALONG THE FLOW OF `plane`: `1 + t·B`. `None` where `plane`
    /// is no simple bivector (`B ∧ B ≠ 0`, or `B²` not a scalar) or is zero.
    pub fn at(plane: &Mv, t: &Q) -> Option<FlowPoint> {
        Self::of_parts(plane, one(), t.clone())
    }

    /// THE IDENTITY ON `plane`'s flow: the point it starts from.
    pub fn origin(plane: &Mv) -> Option<FlowPoint> {
        Self::of_parts(plane, one(), Q::zero())
    }

    fn of_parts(plane: &Mv, a: Q, b: Q) -> Option<FlowPoint> {
        let plane = plane.grade(2);
        if plane.is_zero() {
            return None;
        }
        let square = plane.geo(&plane);
        let k = square.scalar_part();
        if !square.sub(&Mv::scalar(k.clone())).is_zero() {
            return None;
        }
        Some(FlowPoint { plane, k, a, b }.normal())
    }

    /// A MOTION'S POINT ON ITS OWN FLOW, where it lies on one simple plane: its
    /// ray `s + K` (a scalar and one simple bivector, nothing else) read
    /// against the plane `K` itself — the 180° turn, `K` alone, at `(0 : 1)`.
    /// `None` where the motion carries a grade-4 part (two planes at once, an
    /// isoclinic set or a screw: each plane is its own flow, read by its mode)
    /// or is no bivector-and-scalar at all.
    pub fn of_motion(motion: &Mv) -> Option<FlowPoint> {
        let (s, k) = (motion.scalar_part(), motion.grade(2));
        if !motion.sub(&Mv::scalar(s.clone())).sub(&k).is_zero() {
            return None;
        }
        Self::of_parts(&k, s, one())
    }

    /// THE SAME POINT READ AGAINST ANOTHER REFERENCE of the same plane: `None`
    /// where `reference` is not a rational multiple of this plane.
    pub fn against(&self, reference: &Mv) -> Option<FlowPoint> {
        let r = super::group::projective_ratio(&self.plane, reference)?;
        Self::of_parts(reference, self.a.clone(), &self.b * r)
    }

    /// The point's canonical writing: `(1, t)` off the pole, `(0, 1)` on it.
    fn normal(self) -> FlowPoint {
        match (self.a.is_zero(), self.b.is_zero()) {
            (true, true) => self,
            (true, false) => FlowPoint { a: Q::zero(), b: one(), ..self },
            (false, _) => {
                let t = &self.b / &self.a;
                FlowPoint { a: one(), b: t, ..self }
            }
        }
    }

    /// The plane's reference generator.
    pub fn plane(&self) -> &Mv {
        &self.plane
    }

    /// The plane's square `B²`.
    pub fn square(&self) -> &Q {
        &self.k
    }

    /// What kind of flow the plane carries.
    pub fn kind(&self) -> FlowKind {
        match () {
            _ if self.k.is_negative() => FlowKind::Turn,
            _ if self.k.is_positive() => FlowKind::Boost,
            _ => FlowKind::Null,
        }
    }

    /// WHERE THE MOTION STANDS on its flow, `t = b/a`; `None` at the pole (the
    /// 180° turn), where `t` is infinite and the point is held as `(0 : 1)`.
    pub fn parameter(&self) -> Option<Q> {
        (!self.a.is_zero()).then(|| &self.b / &self.a)
    }

    /// THE POINT AS THE VERSOR IT IS: `a + b·B`.
    pub fn versor(&self) -> Mv {
        Mv::scalar(self.a.clone()).add(&self.plane.scale(&self.b))
    }

    /// TWO POINTS OF ONE FLOW COMPOSED, exactly: `(a₁a₂ + k b₁b₂) + (a₁b₂ +
    /// a₂b₁) B`. `None` where the other point lies on another plane — two
    /// flows compose only as a motion, never as one parameter.
    pub fn then(&self, other: &FlowPoint) -> Option<FlowPoint> {
        let o = other.against(&self.plane)?;
        let a = &self.a * &o.a + &self.k * &self.b * &o.b;
        let b = &self.a * &o.b + &o.a * &self.b;
        Some(FlowPoint { plane: self.plane.clone(), k: self.k.clone(), a, b }.normal())
    }

    /// THE POINT RUN BACKWARD: `a − b·B`, the flow's own inverse.
    pub fn back(&self) -> FlowPoint {
        FlowPoint { b: -&self.b, ..self.clone() }.normal()
    }

    /// HAS THE FLOW RETURNED: `b = 0`, the identity ray (a turn through 360°
    /// lands on `−1`, the same ray).
    pub fn returned(&self) -> bool {
        self.b.is_zero() && !self.a.is_zero()
    }

    /// THE ACTION'S VALUE ON THE PLANE, `c = (a² + k b²)/(a² − k b²)`: `cos` of
    /// the turn, `cosh` of the boost, `1` on a null flow. `None` where the
    /// denominator vanishes — a boost's point on the light cone of its plane,
    /// which is no versor.
    pub fn c(&self) -> Option<Q> {
        let (a2, kb2) = (&self.a * &self.a, &self.k * &self.b * &self.b);
        let den = &a2 - &kb2;
        (!den.is_zero()).then(|| (a2 + kb2) / den)
    }

    /// HOW FAR THE FLOW STANDS FROM ITS RETURN, exact: `|1 − c|` on a turn or a
    /// boost, `|t|` against the reference on a null flow. Zero only at the
    /// return. Comparable along one plane's flow, never across two.
    pub fn distance(&self) -> Option<Q> {
        match self.kind() {
            FlowKind::Null => self.parameter().map(|t| t.abs()),
            FlowKind::Turn | FlowKind::Boost => self.c().map(|c| (one() - c).abs()),
        }
    }
}
