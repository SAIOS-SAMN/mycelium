// 78.52 ROW 2 — HYPHAL FASCICLES: a transition is no rigid vector; it is a
// fascicle — two round dual spheres whose reflections, one then the other,
// stream a body along a conformal streamline, `V = s₂ s₁` (ruled 2026-09-28,
// the operator's word; the name ruled the same day).
//
//! A FASCICLE `(s₁, s₂)` — two dual spheres, `sᵢ² = rᵢ² ∈ ℚ` (a plane is a
//! sphere through `e_∞`, its square its normal's) — composes the versor
//! `V = s₂ s₁`: reflect in `s₁`, then in `s₂`. Its reverse is `s₁ s₂`, its
//! weight `V Ṽ = s₁² s₂² ∈ ℚ`, and it acts on anything by the sandwich read
//! at unit weight, `Ad_V(X) = V X Ṽ / (s₁² s₂²)` — no root, no series, no
//! matrix.
//!
//! THE STREAMLINE a fascicle flows along is read off its two spheres'
//! inversive distance `δ² = ⟨s₁ s₂⟩₀² / (s₁² s₂²)` (GR1):
//! - `δ² < 1` ELLIPTIC — the spheres cut; the stream turns about their
//!   circle `s₁ ∧ s₂`, by twice their dihedral angle (`cos² φ = δ²`).
//! - `δ² = 1` PARABOLIC — the spheres kiss; the stream flows about the
//!   contact, which it fixes.
//! - `δ² > 1` HYPERBOLIC — the spheres are apart; the stream breathes along
//!   the orthogonal trajectories (concentric: a dilation by `r₂² / r₁²`).
//!
//! THE SLIDE IS A FASCICLE: the two parallel planes `d` and `d + ½|d|² e_∞`
//! — spheres through `e_∞`, kissing there, a parabolic fascicle — compose
//! `|d|² (1 − ½ d e_∞)` exactly. The translator is that fascicle read at unit
//! first weight ([`Fascicle::parallel`], `cga::translator_at`), never a
//! formula written on its own. A fascicle kissing at a FINITE point is
//! conformal but not rigid — it carries no body onto its slid copy — so the
//! slide perception sees is the fascicle kissing at `e_∞` (78.52(5)).

use crate::transducer::cga::{self, Mv};
use num_rational::BigRational;
use num_traits::{One, Zero};

type Q = BigRational;

/// THE STREAMLINE a fascicle flows along, read off its inversive distance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Streamline {
    /// `δ² < 1`: the spheres cut; the stream turns about their circle.
    Elliptic,
    /// `δ² = 1`: the spheres kiss; the stream flows about the contact.
    Parabolic,
    /// `δ² > 1`: the spheres are apart; the stream breathes.
    Hyperbolic,
}

impl Streamline {
    /// The streamline of an inversive distance `δ²`.
    pub fn of(delta2: &Q) -> Streamline {
        match delta2.cmp(&Q::one()) {
            std::cmp::Ordering::Less => Streamline::Elliptic,
            std::cmp::Ordering::Equal => Streamline::Parabolic,
            std::cmp::Ordering::Greater => Streamline::Hyperbolic,
        }
    }
}

/// A HYPHAL FASCICLE: two dual spheres, reflected in in order — first
/// `first`, then `second`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fascicle {
    pub first: Mv,
    pub second: Mv,
}

impl Fascicle {
    /// THE FASCICLE'S VERSOR `V = s₂ s₁`.
    pub fn versor(&self) -> Mv {
        self.second.geo(&self.first)
    }

    /// THE FASCICLE'S WEIGHT `V Ṽ = s₁² s₂²`.
    pub fn weight(&self) -> Q {
        self.first.norm2() * self.second.norm2()
    }

    /// ITS INVERSIVE DISTANCE `δ² = ⟨s₁ s₂⟩₀² / (s₁² s₂²)`; `None` for a null
    /// sphere (a point), which has no weight.
    pub fn inversive(&self) -> Option<Q> {
        let w = self.weight();
        if w.is_zero() {
            return None;
        }
        let dot = self.first.geo(&self.second).scalar_part();
        Some(&dot * &dot / w)
    }

    /// THE STREAMLINE it flows along.
    pub fn streamline(&self) -> Option<Streamline> {
        self.inversive().map(|d| Streamline::of(&d))
    }

    /// `Ad_V(X) = V X Ṽ / (s₁² s₂²)` — the action read at unit weight; `None`
    /// for a fascicle with no weight.
    pub fn act(&self, x: &Mv) -> Option<Mv> {
        let w = self.weight();
        if w.is_zero() {
            return None;
        }
        Some(self.versor().sandwich(x).scale(&(Q::one() / w)))
    }

    /// THE SLIDE BY `d` AS A FASCICLE: the plane through the origin normal to
    /// `d`, then the parallel plane half-way along it, `d + ½|d|² e_∞` —
    /// kissing at `e_∞`, parabolic. `None` for `d = 0`, which slides nowhere.
    pub fn parallel(d: &[Q]) -> Option<Fascicle> {
        let first = cga::base_vector(d);
        let dd = first.norm2();
        if dd.is_zero() {
            return None;
        }
        let second = first.add(&cga::e_inf().scale(&(dd * Q::new(1.into(), 2.into()))));
        Some(Fascicle { first, second })
    }

    /// THE STREAM, `k` steps along the streamline: `Vᵏ`, each power held at
    /// its canonical ray.
    pub fn stream(&self, k: u64) -> Mv {
        let v = self.versor();
        let mut power = Mv::scalar(Q::one());
        for _ in 0..k {
            let next = v.geo(&power);
            power = match next.canonical() {
                Some(ray) => ray,
                None => next,
            };
        }
        power
    }
}

/// THE STREAMLINE OF A MOTION held as one versor (78.52 Row 2): a 2-versor —
/// a scalar and one simple bivector, `B ∧ B = 0` — is a fascicle's product,
/// and its inversive distance reads `δ² = ⟨V⟩₀² / (V Ṽ)`. `None` where `V`
/// is no 2-versor (a loxodromic composite, two fascicles) or has no weight.
pub fn streamline_of(v: &Mv) -> Option<Streamline> {
    let (s, b) = (v.scalar_part(), v.grade(2));
    if !v.sub(&Mv::scalar(s.clone())).sub(&b).is_zero() || !b.geo(&b).grade(4).is_zero() {
        return None;
    }
    let w = v.geo(&v.reverse()).scalar_part();
    if w.is_zero() {
        return None;
    }
    Some(Streamline::of(&(&s * &s / w)))
}
