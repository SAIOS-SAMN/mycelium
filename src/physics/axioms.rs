// 56 — THE AXIOM REGISTER: what this engine assumes, what it refuses to
// assume, and the reading that decides each (ruled 2026-09-22).
//
//! A.1–A.10 are `ipsal-autic`'s register, whose own numbering marks
//! A.1–A.4 as carried forward unchanged from the convergence-engine and
//! A.5–A.10 as its additions. They are NOT adopted wholesale. Three are
//! REFUSED, and the refusals matter more than the imports: each would
//! flatten something this engine measures, and each is refused here with
//! a test that says why rather than a paragraph that asserts it.
//!
//! An axiom that cannot fail a test is a coordinate standing for no
//! theorem (T21(iii)). Every entry below is either exercised or is
//! explicitly marked as a statement about work not yet built.

/// A.6 — DIMENSIONAL COMPLETENESS, UNBOUND (imported).
///
/// Every contradiction admits a higher-dimensional resolution. A.6 names
/// a MECHANISM — escalation — and never a bound.
///
/// Their register had to split this from the value `N = 11` after the two
/// were conflated (their audit #126): a mechanism read as a constant. That
/// is the error class of `Fill::Open` standing for absence, and it is why
/// `A.6-bind-11` is refused below while A.6 itself is kept.
pub const A6_DIMENSIONAL_COMPLETENESS: &str =
    "every contradiction admits a higher-dimensional resolution; \
     escalation is a mechanism, never a bound";

/// A.8 — TORSIONAL PERSISTENCE (imported).
///
/// Torsion is never destroyed. It is promoted, annihilated against an
/// opposing pair, or deposited as an inverse record; total information is
/// monotonically NON-DECREASING.
///
/// This is `drain`'s conservation law, which §55 had no invariant for. It
/// is stronger than the Lyapunov bound first reached for: strain cannot
/// be relaxed away at all, only moved or recorded. A drain that lowers
/// `Σμ` without exhibiting a partner or a deposit is refused.
///
/// ANNIHILATION IS WHY THE CARRIER HOLDS VERSORS. An opposing pair is
/// `Ω_a · Ω_b ∼ ±1`, and two opposite Burgers vectors carry the SAME `μ` —
/// a magnitude cannot identify a partner. `PathCarried.steps` therefore
/// holds the versors and `μ` is taken as a projection.
pub const A8_TORSIONAL_PERSISTENCE: &str =
    "torsion is promoted, annihilated in an opposing pair, or deposited; \
     total information is monotonically non-decreasing";

/// A.9 — METRIC AUTONOMY (imported).
///
/// Each entity computes its own local coherence metric. There is no
/// global metric, and two entities may assign different values to one
/// state without contradiction.
///
/// The form-scoped quantum is therefore correct BY AXIOM rather than by
/// the argument §54.7 reached for. The autonomy is the ENTITY's: a
/// per-chord metric would still be wrong, because a form is the entity
/// and a chord is not.
pub const A9_METRIC_AUTONOMY: &str =
    "each entity's coherence metric is its own; there is no global metric";

/// A.10 — SYMBIOTIC CO-DIMENSIONALITY (imported, with a correction).
///
/// Two manifolds of differing dimensionality may occupy the same spatial
/// references without collapse IF AND ONLY IF their interface is governed
/// by a luminal phase gradient. The tether is a shared boundary in the
/// cohomological sense, not a pointer and not a channel.
///
/// THE GRADIENT MUST BE LUMINAL — null, `L² = 0`. On a null boundary the
/// induced metric is degenerate, which is what lets two interiors share
/// boundary freedom without crushing each other. This engine's meets run
/// between `v² = −1` points: timelike, `μ > 0`. Calling that the luminal
/// gradient was an error (§55.7), and the corpus reads `parabolic 0`, so
/// the medium A.10 requires is at present absent.
pub const A10_CO_DIMENSIONALITY: &str =
    "two manifolds co-occupy without collapse iff their interface is a \
     luminal (null) phase gradient";

/// A.11 — HARMONIC MULTIPOLE COMPLETENESS (ours, 56.3).
///
/// The geometric state of a discrete spatial chart is completely and
/// irreducibly characterised by its ascending harmonic multipoles in `ℚ`.
/// No higher-rank moment may be collapsed into a lower-rank scalar trace
/// before the frame is evaluated.
///
/// ```text
///   RADIAL (scalar)   tr(M₂) = I_xx + I_yy
///                     governs isotropic dilation and the basepoint's
///                     radius. CARRIES ZERO ORIENTATIONAL AUTHORITY.
///
///   ORIENTATION       2nd harmonic, turns at 2φ: u = ((I_xx−I_yy)/2)e₁ + I_xy e₂
///   (tensor)          4th harmonic, turns at 4φ: h₄, which orients a body
///                     whose M₂ is ISOTROPIC — a square, a cross — that the
///                     second moment cannot tell from a disc.
///
///   SINGLE PRODUCER   computed once at `blade::lift`; no consumer may
///   (law 12)          recompute, approximate, or substitute a scalar.
/// ```
///
/// WHY AN AXIOM AND NOT A CONVENTION. `tr(M₂)` is an `SO(2)` invariant, so
/// a chart read through its trace alone is an ISOTROPIC POINT MASS. Two
/// point masses have no relative angle; the only relation left is the
/// geodesic through their centres, whose bivector is a simple blade with
/// `I₂ = 0` and `ℓ = 1`. The collapse does not lose precision — it loses a
/// SECTOR, silently, and reports `ℓ = 1` as a fact about the scene rather
/// than as an artefact of the reading.
pub const A11_HARMONIC_MULTIPOLE_COMPLETENESS: &str =
    "a chart is characterised by its ascending harmonic multipoles; the \
     trace is scale alone and carries no orientation";

/// A.4 — RATIONAL INNER PRODUCT, "positive-definite" (REFUSED).
///
/// False in this algebra. `Cl(4,1)` is indefinite, `e₋² = −1`, and the
/// sheet is `−t² + x² + y² + z² = −1`. Positive-definiteness would flatten
/// every boost generator into an elliptic rotation and erase the
/// non-compact sector — which is the sector this corpus entirely occupies.
pub const A4_POSITIVE_DEFINITE_REFUSED: &str =
    "refused: the metric is indefinite, e₋² = −1";

/// A.7 — HOLONOMIC INVARIANT as written (REFUSED).
///
/// "Closed paths always return to their topological starting class modulo
/// torsion", valued in `H¹(X, ℤ)` — the abelian, torsion-only picture. The
/// hyperbolic sector never returns: `cosh(kβ) > 1` for every `k ≥ 1`,
/// which is the measured `closes 0`. A.7 holds for the compact half alone,
/// and importing it whole asserts a closure measured to be absent.
pub const A7_ABELIAN_RETURN_REFUSED: &str =
    "refused for the non-compact sector: cosh(kβ) > 1 forbids return";

/// A.6-bind-11 — `N = 11` EXACTLY (REFUSED).
///
/// Sourced to an architecture document; their register is honest that it
/// is a constitutional BINDING and not a theorem of the algebra. A
/// hardcoded generator is refused whatever register it arrives in.
pub const A6_BIND_11_REFUSED: &str =
    "refused: a dimension count from an architecture document is not a theorem";

#[cfg(test)]
mod tests {
    use crate::transducer::blade;
    use num_traits::Zero;

    /// A.4 IS REFUSED BECAUSE THE METRIC SAYS SO. `e₋² = −1`, so the form
    /// is indefinite and no positive-definite inner product exists on it.
    /// Asserted on the algebra rather than on a comment, because the whole
    /// hyperbolic sector depends on the sign.
    #[test]
    fn a4_is_refused_the_metric_is_indefinite() {
        use crate::transducer::cga;
        let em = crate::transducer::element::Versor::blade(cga::EM, crate::core::Q::from_integer(1.into()));
        let sq = em.geo(&em).scalar_part();
        assert!(
            sq < crate::core::Q::zero(),
            "e₋² must be negative or the boost sector collapses into rotation; got {sq}"
        );
    }

    /// A.11's RADIAL/ORIENTATION SPLIT, on bodies that distinguish them.
    ///
    /// Two congruent bodies at different orientations share a trace and
    /// differ in the deviatoric. If the trace could orient, these would be
    /// distinguishable by it; they are not, which is the axiom's content.
    #[test]
    fn a11_the_trace_cannot_orient_and_the_deviatoric_can() {
        // A domino lying along the rows, and the same domino along the
        // columns: congruent, one a quarter turn from the other.
        let along_r = blade::shape_of(&[(0, 0, 0), (1, 0, 0)]);
        let along_c = blade::shape_of(&[(0, 0, 0), (0, 1, 0)]);

        let trace = |s: &blade::Shape| &s.blade.inertia[0] + &s.blade.inertia[1];
        assert_eq!(
            trace(&along_r),
            trace(&along_c),
            "the trace is an SO(2) invariant: it cannot tell the two apart"
        );

        let (ua, ub) = (
            blade::deviatoric(&along_r.blade).expect("a domino is anisotropic"),
            blade::deviatoric(&along_c.blade).expect("a domino is anisotropic"),
        );
        assert_ne!(ua, ub, "the deviatoric CAN tell them apart");

        // And the angle between them is exact, with no root taken.
        let (cos2, sin2) =
            blade::relative_double_angle(&ua, &ub).expect("congruent bodies have an angle");
        assert!(
            cos2 == crate::core::Q::from_integer((-1).into()) && sin2.is_zero(),
            "a quarter turn is 2φ = π: cos 2φ = −1, sin 2φ = 0; got ({cos2}, {sin2})"
        );
    }

    /// A.11's FOURTH HARMONIC, on the body the second moment is blind to.
    ///
    /// A square has `I_xx = I_yy` and `I_xy = 0`, so its deviatoric
    /// vanishes and `D` cannot orient it. The quartic must, or the axiom's
    /// second clause is empty.
    #[test]
    fn a11_the_quartic_orients_what_the_deviatoric_cannot() {
        let square = blade::shape_of(&[(0, 0, 0), (0, 1, 0), (1, 0, 0), (1, 1, 0)]);
        assert!(
            blade::deviatoric(&square.blade).is_none(),
            "a square's second moment is isotropic, so it has no principal axis"
        );
        assert!(
            blade::quartic_harmonic(&square.blade).is_some(),
            "a discrete square is not a disc: its FOURTH moment orients it"
        );
    }

    /// A NON-CONGRUENT PAIR HAS NO ANGLE. Two bodies whose anisotropies
    /// differ are not related by a rotation, and answering one anyway
    /// would be inventing a reading (rule 11).
    #[test]
    fn a11_non_congruent_bodies_have_no_rotation_between_them() {
        let domino = blade::shape_of(&[(0, 0, 0), (1, 0, 0)]);
        let tromino = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0)]);
        let (ua, ub) = (
            blade::deviatoric(&domino.blade).expect("anisotropic"),
            blade::deviatoric(&tromino.blade).expect("anisotropic"),
        );
        assert!(
            blade::relative_double_angle(&ua, &ub).is_none(),
            "different anisotropies are not a rotation apart"
        );
    }
}
