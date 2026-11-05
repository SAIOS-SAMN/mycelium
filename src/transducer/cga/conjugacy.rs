// AI-GENERATED CONTENT — see DISCLAIMER.md.
//! 54.7 — THE CONJUGACY CLASS OF A VALUE OF `Cl(n+1, 1)`, read off its
//! grade-2 generator, and the holonomy coordinate the library's carriers
//! answer it by.

use super::group::weight_of;
use super::multivector::Mv;
use crate::network::nerve::coord::{BivectorKind, Holonomy};
use crate::transducer::address::Address;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

/// THE CONJUGACY CLASS OF A VALUE: its invariants and the coordinates they
/// are written as (A21), never a digest. The value, its writing and its own
/// key are the exact algebra's (`algebra::Mv`); the class reads the value's
/// action (`weight_of`) and is filed under the library's coordinates, so it
/// stands here.
pub trait Conjugacy {
    /// THE CONJUGACY INVARIANTS OF A TRANSITION, READ OFF ITS GRADE-2
    /// GENERATOR (54.7, as ruled 2026-09-21).
    ///
    /// A transition is `Ω = R · T(τ)` and its generator is the bivector
    /// `B = ⟨Ω⟩₂`. In `Cl(n+1,1)` that bivector is NOT one kind of thing:
    ///
    /// ```text
    ///     B = B_ell + B_hyp ,   B_ell = Σ b_ij (eᵢ∧eⱼ)
    ///                           B_hyp = Σ b_i₋ (eᵢ∧e₋)
    /// ```
    ///
    /// `B_ell` generates an ELLIPTIC rotation stabilising a timelike
    /// geodesic; `B_hyp` generates a HYPERBOLIC transvection, a pure
    /// geodesic translation; a null `B` generates a PARABOLIC limit
    /// rotation stabilising a point on the asymptotic sphere. The three are
    /// not conjugate to one another, and the sign of
    ///
    /// ```text
    ///     I₁ = ⟨B²⟩₀ = ‖B_ell‖² − ‖B_hyp‖²
    /// ```
    ///
    /// is what tells them apart.
    ///
    /// WHY THIS CANNOT BE DONE IN `Cl(4,0)`. `Rotor::class_invariants` reads
    /// `c[0]` and `c[15]` — grade 0 and grade 4 — and discards all six
    /// bivector components. That is not an oversight there: `Cl(4,0)` is
    /// POSITIVE DEFINITE, every bivector squares negative, every plane is
    /// carried to every other by a conjugation, and the conjugacy classes of
    /// `Spin(4)` genuinely are the two angles alone.
    ///
    /// It adds everything here. Under a signature with `e₋² = −1` the sign
    /// of `I₁` is a real coordinate, and a rotation, a boost and a limit
    /// rotation are three classes rather than one — which is why the
    /// transition's two halves must be composed into ONE element before the
    /// class is taken.
    ///
    /// ```text
    ///     I₂ = ‖⟨B²⟩₄‖²      the screw coupling, 2(B_ell · B_hyp) squared
    /// ```
    ///
    /// `I₂ = 0` is a planar rotation or a pure transvection; `I₂ ≠ 0` is a
    /// helicoidal screw, the two halves coupled.
    ///
    /// EXACT AND SCALE-FREE. `B ↦ λB` sends `⟨B²⟩₀ ↦ λ²⟨B²⟩₀` and
    /// `w = ΩΩ̃ ↦ λ²w`, so `I₁/w` is invariant; `‖⟨B²⟩₄‖² ↦ λ⁴‖·‖²`, so
    /// `I₂/w²` is. `None` when `Ω` is not a versor, which is a refusal and
    /// never a zero class (rule 11).
    fn class_invariants(&self) -> Option<(BigRational, BigRational, BivectorKind)>;

    /// 54.7 R4 — THE HOLONOMY CLASS OF A MOTION, AS THE COORDINATE THE
    /// TORSION AND AXIS CARRIERS ANSWER. The same bytes as
    /// [`Conjugacy::class_key`], declared as `coord::Holonomy`.
    fn holonomy_key(&self) -> Option<Holonomy>;

    /// The class as canonical coordinates — written by the coordinate
    /// itself, so the bytes here and the bytes a carrier is keyed by are
    /// one construction and not two agreeing (rule 12).
    fn class_key(&self) -> Option<Address>;
}

impl Conjugacy for Mv {
    fn class_invariants(&self) -> Option<(BigRational, BigRational, BivectorKind)> {
        // `I₁/w`, `I₂/w²`: scale-free ratios read as the class (Axiom C).
        let w = weight_of(self)?;
        if w.is_zero() {
            return None;
        }
        let b = self.grade(2);
        let bb = b.geo(&b);
        let i1 = bb.scalar_part() / &w;
        let quad = bb.grade(4);
        let i2 = quad.geo(&quad.reverse()).scalar_part() / (&w * &w);
        // THE SIGN IS THE TYPE. With `eᵢ² = +1` for every generator but
        // `e₋² = −1`, all anticommuting:
        //
        //   ELLIPTIC   B = e₁∧e₂ :  B² = −1  < 0
        //   HYPERBOLIC B = eᵢ∧e₋ :  B² = +1  > 0
        //   PARABOLIC  B = −½ d e_∞ : e_∞² = 0 and d ⊥ e_∞, so B² = 0
        //
        // TRIVIAL IS NOT PARABOLIC (54.7): the identity turns nothing, and
        // `I₁ = 0` alone cannot tell the two apart, so the bivector itself
        // is asked.
        let kind = if b.is_zero() {
            BivectorKind::Trivial
        } else if i1.is_negative() {
            BivectorKind::Elliptic
        } else if i1.is_positive() {
            BivectorKind::Hyperbolic
        } else {
            BivectorKind::Parabolic
        };
        Some((i1, i2, kind))
    }

    fn holonomy_key(&self) -> Option<Holonomy> {
        let (i1, i2, kind) = self.class_invariants()?;
        Some(Holonomy { i1, i2, kind })
    }

    fn class_key(&self) -> Option<Address> {
        use crate::network::nerve::Coordinate;
        let h = self.holonomy_key()?;
        Some(Address::from_bytes(h.encode().as_bytes()))
    }
}
