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

    /// THE CAYLEY CHART'S BOUNDARY IS THE LIGHT CONE, and it is refused
    /// by name rather than by falling through a generic failure.
    ///
    /// `B = u ∧ e₋` has `B² = |u|²`, so `1 − B²` vanishes exactly on the
    /// unit sphere, and `|u| = tanh(θ/2)` puts that at `θ → ∞` — infinite
    /// rapidity. Every finite boost lies strictly inside; the boundary is
    /// luminal. A displacement one step inside the cone must still read.
    #[test]
    fn the_transvection_chart_covers_finite_boosts_and_ends_at_the_cone() {
        use crate::core::Q;
        use crate::transducer::cga;
        let q = |n: i64, d: i64| Q::new(n.into(), d.into());
        let planes = cga::TRANSVECTION_PLANES;
        let gen = |s: Q| crate::transducer::element::Versor::blade(planes[0], s);

        // Inside the cone: |u| = 1/2, a finite boost, carried exactly.
        assert!(
            cga::cayley(&gen(q(1, 2))).is_some(),
            "a finite rapidity is inside the Cayley chart"
        );
        // ON the cone: |u| = 1, θ → ∞. No finite versor represents it.
        assert!(
            cga::cayley(&gen(q(1, 1))).is_none(),
            "the unit sphere is infinite rapidity and has no finite versor"
        );
        // Just inside it, and still exact — the boundary is a point, not
        // a neighbourhood that swallows nearby readings.
        assert!(
            cga::cayley(&gen(q(99, 100))).is_some(),
            "the chart is open up to the cone, not merely near its centre"
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

    /// A.11's FRAME, MINTED: a body and its quarter turn give the rotor
    /// `1 + e₁₂`, whose projective order is 4 — the reading `ℓ = 1` on
    /// every word exists to be moved off.
    #[test]
    fn a11_a_quarter_turn_mints_an_order_four_rotor() {
        // An L-tromino and the same L turned a quarter: chiral enough
        // that only one of the four turns can carry it.
        let upright = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (1, 1, 0)]);
        let turned = blade::shape_of(&[(0, 0, 0), (0, 1, 0), (1, 0, 0)]);
        let theta = blade::relative_rotor(&upright.blade, &turned.blade)
            .expect("one proper turn carries an L onto its quarter turn");
        assert!(
            matches!(theta, blade::Frame::Turn(_)),
            "equal cardinality: the support settles the half turn, got {theta:?}"
        );
        assert!(
            !theta.rotor().projective_eq(&crate::transducer::element::identity_versor()),
            "a turned body must not mint the identity: got {theta:?}"
        );
        let order = crate::transducer::cga::finite_order(theta.rotor()).map(|l| l.get());
        assert!(
            matches!(order, Some(2) | Some(4)),
            "a proper turn of the square base closes at 2 or 4, got {order:?}"
        );
    }

    /// AND A BODY AGAINST ITSELF IS THE IDENTITY — the rotor is not
    /// invented where there is no turn.
    #[test]
    fn a11_a_body_against_itself_is_the_identity_turn() {
        let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (1, 1, 0)]);
        let theta = blade::relative_rotor(&l.blade, &l.blade).expect("a body carries onto itself");
        assert!(
            theta.rotor().projective_eq(&crate::transducer::element::identity_versor()),
            "no turn is no rotor, got {theta:?}"
        );
    }

    /// SIMILARITY, NOT CONGRUENCE. A body and its 2× dilation are related
    /// by NO isometry — 8 cells against 2 — yet their rotation is
    /// perfectly well defined and is the identity. Demanding equal cell
    /// counts and equal `‖u‖²` refused this, which modelled every
    /// transformation as rigid when the carriage carries a dilator `D(ρ)`.
    ///
    /// The half turn stays OPEN here: an even moment cannot close it and
    /// the cardinalities differ, so no cell test can. Saying so is the
    /// honest answer; picking one of the two silently is not.
    #[test]
    fn a11_a_dilated_body_keeps_its_frame() {
        let domino = blade::shape_of(&[(0, 0, 0), (1, 0, 0)]);
        // The same domino at twice the scale: a 2×4 block.
        let doubled = blade::shape_of(&[
            (0, 0, 0),
            (0, 1, 0),
            (1, 0, 0),
            (1, 1, 0),
            (2, 0, 0),
            (2, 1, 0),
            (3, 0, 0),
            (3, 1, 0),
        ]);
        let theta = blade::relative_rotor(&domino.blade, &doubled.blade)
            .expect("a dilation is a similarity and its rotation is defined");
        assert!(
            matches!(theta, blade::Frame::HalfOpen(_)),
            "differing cardinality leaves the half turn open, got {theta:?}"
        );
        assert!(
            theta.rotor().projective_eq(&crate::transducer::element::identity_versor()),
            "a pure dilation turns nothing, got {theta:?}"
        );
    }

    /// A REFLECTION IS NOT A ROTATION — on a body chiral enough to tell.
    ///
    /// THE THREE-CELL L WILL NOT DO, and this test first used it: the
    /// L-tromino is ACHIRAL, its mirror image IS one of its own four
    /// turns, and `relative_rotor` correctly found one. The smallest
    /// chiral polyomino takes four cells, so the L- and J-tetrominoes are
    /// the honest fixture — congruent as multisets, mirror images, and
    /// related by NO proper turn.
    #[test]
    fn a11_a_mirrored_body_mints_no_rotor() {
        let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
        let j = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, -1, 0)]);
        assert!(
            blade::relative_rotor(&l.blade, &j.blade).is_none(),
            "a reflection lies in O(2)\\SO(2) and is not a rotor"
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

    /// A.11's REACH, MEASURED RATHER THAN ASSUMED (ruled 2026-09-22).
    ///
    /// A claim was made that the trace/2nd/4th suite is blind to six of the
    /// nine crystallographic orders and that `relative_rotor` minting only
    /// `{1, 2, 4}` was the instrument's own limit. IT IS NOT, and this test
    /// is the arithmetic that says so.
    ///
    /// The 2D crystallographic restriction is `φ(ℓ) ≤ 2`, admitting
    /// `{1, 2, 3, 4, 6}`, and a SQUARE lattice narrows it to `{1, 2, 4}`. A
    /// body of unit cells on an ARC grid therefore cannot carry `C₃` or `C₆`
    /// symmetry, and no rotation of order 3 or 6 maps the lattice to itself,
    /// so no such relation between two grid bodies exists to be measured.
    /// `φ(ℓ) ≤ 4` giving `{1,2,3,4,5,6,8,10,12}` is the restriction in FOUR
    /// dimensions: those orders live on the off-plane channels of `Cl(4,0)`
    /// and are not planar shape orientations at all.
    ///
    /// So `{2, 4}` is exactly sufficient for the sensor plane, and what it
    /// covers is asserted here: the 2nd orients everything the lattice can
    /// turn except the `C₄` bodies, and the 4th orients those.
    #[test]
    fn a11_the_even_suite_covers_the_square_lattices_whole_order_set() {
        // C₁ — an L-tromino. Anisotropic at the second harmonic.
        let c1 = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (0, 1, 0)]);
        assert!(
            blade::deviatoric(&c1.blade).is_some(),
            "a generic body is oriented by the 2nd harmonic"
        );

        // C₂ — a domino. Also second-harmonic anisotropic.
        let c2 = blade::shape_of(&[(0, 0, 0), (1, 0, 0)]);
        assert!(
            blade::deviatoric(&c2.blade).is_some(),
            "a two-fold body is oriented by the 2nd harmonic"
        );

        // C₄ — the 2x2 square. The 2nd harmonic VANISHES on it, which is
        // the whole reason the 4th exists (A.11).
        let c4 = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (0, 1, 0), (1, 1, 0)]);
        assert!(
            blade::deviatoric(&c4.blade).is_none(),
            "a four-fold body is isotropic to the 2nd harmonic"
        );

        // AND THE ORDER SET IS CLOSED HERE. `{1, 2, 4}` is every rotation a
        // square lattice admits, so a suite that orients each of them is
        // complete for the sensor plane. A seventh harmonic would answer a
        // question the base cannot ask.
    }

    /// THE PARITY GAP IS REAL, AND IT IS NOT THE ORDER GAP (57.3).
    ///
    /// The one thing the even suite genuinely cannot do is separate `φ` from
    /// `φ + π`: every even-rank moment satisfies `u(−x) = u(x)`, so a body
    /// and its point reflection through the centroid carry the SAME
    /// deviatoric and the SAME quartic. This is independent of the order
    /// set — it holds for `C₁` bodies on a square lattice, which is most of
    /// them — and it is why `Frame::HalfOpen` exists.
    ///
    /// Closing it needs an ODD invariant. The 3rd harmonic is the candidate,
    /// and the reason it works is NOT that bodies have three-fold symmetry
    /// (a square lattice forbids that) but that a generic body has a
    /// non-vanishing third moment which changes sign under `x -> -x`.
    #[test]
    fn a11_no_even_moment_separates_a_body_from_its_point_reflection() {
        let body = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (0, 1, 0)]);
        // The same body reflected through the origin. Congruent to itself
        // under a half turn, and a half turn is what must be detected.
        let flipped = blade::shape_of(&[(0, 0, 0), (-1, 0, 0), (0, -1, 0)]);

        let (ua, ub) = (
            blade::deviatoric(&body.blade).expect("anisotropic"),
            blade::deviatoric(&flipped.blade).expect("anisotropic"),
        );
        assert_eq!(ua, ub, "the 2nd harmonic cannot see a half turn");

        let (qa, qb) = (
            blade::quartic_harmonic(&body.blade),
            blade::quartic_harmonic(&flipped.blade),
        );
        assert_eq!(qa, qb, "and neither can the 4th: both are even rank");
    }

    /// `I₂ ≠ 0` IS PROOF THE MOTION LEFT THE SENSOR PLANE (ruled
    /// 2026-09-22).
    ///
    /// THE SENSOR IS NOT THE ONTOLOGY. A reading that confined the engine's
    /// orders to `{1, 2, 4}` because the detector is a square raster
    /// mistook the display substrate for the manifold: a facet pitched out
    /// of the sensor plane projects to jagged cells, and the refraction
    /// rotor across its edge carries its own order whatever the raster can
    /// tile. `Cl(4,1)` admits the whole cyclotomic spectrum, `φ(ℓ) ≤ 4`.
    ///
    /// And the second Casimir says when that has happened. `⟨Ω⟩₄` is a
    /// grade-4 part, so it needs FOUR independent directions to be
    /// non-zero. Everything confined to one plane is proportional to `e₁₂`
    /// and `e₁₂ ∧ e₁₂ = 0`; a planar translation is `v ∧ e_∞` and two of
    /// them repeat `e_∞`; a planar rotation against a planar translation
    /// repeats `e₁` or `e₂`. Each is asserted below rather than argued.
    ///
    /// So a corpus reading `non-simple 3` is not carrying a curiosity: it is
    /// EMPIRICAL PROOF that three of its circuits compose content off the
    /// sensor plane, which is what a refraction between two non-parallel
    /// facets is.
    #[test]
    fn a_non_simple_bivector_requires_content_off_the_sensor_plane() {
        use crate::transducer::cga;
        let q = |n: i64| crate::core::Q::from_integer(n.into());

        // A planar rotation composed with a planar rotation: one blade, so
        // the wedge is identically zero.
        let r1 = cga::rotator(&q(1), &q(1));
        let r2 = cga::rotator(&q(2), &q(3));
        assert!(
            cga::bivector_is_simple(&r1.geo(&r2)),
            "two turns in one plane cannot couple: e₁₂ ∧ e₁₂ = 0"
        );

        // A planar rotation composed with an IN-PLANE translation. The
        // translator is `v ∧ e_∞` with `v` in `e₁e₂`, so every grade-4 term
        // repeats a basis vector.
        let t_in = cga::translator(&q(3), &q(5), &q(0));
        assert!(
            cga::bivector_is_simple(&r1.geo(&t_in)),
            "a turn and a slide inside the sensor plane stay simple"
        );

        // Two in-plane translations: `e_∞` repeats.
        let t_in2 = cga::translator(&q(-2), &q(7), &q(0));
        assert!(
            cga::bivector_is_simple(&t_in.geo(&t_in2)),
            "two in-plane slides stay simple"
        );

        // NOW LEAVE THE PLANE. A turn in `e₁e₂` against a displacement with
        // a third-axis component — which in this engine is the FRAME axis,
        // the corpus's own `e₃` (61.3) — spans a four-volume.
        let t_off = cga::translator(&q(0), &q(0), &q(1));
        assert!(
            !cga::bivector_is_simple(&r1.geo(&t_off)),
            "a turn against an off-plane displacement is a SCREW: I₂ ≠ 0"
        );

        // TWO TURNS SHARING AN AXIS DO NOT COUPLE, and this is not a
        // detail. `e₁₂ ∧ e₁₃ = 0` because `e₁` repeats, and in THREE
        // dimensions every pair of 2-planes shares an axis — so a
        // composition of two spatial rotations is simple no matter how the
        // facets are tilted relative to one another.
        let shares_an_axis = cga::rotator_on_blade(&q(1), &q(1), cga::E1 | cga::E3);
        assert!(
            cga::bivector_is_simple(&r1.geo(&shares_an_axis)),
            "two turns through a common axis stay simple: e₁₂ ∧ e₁₃ = 0"
        );

        // IT TAKES A FOURTH DIRECTION. Two turns couple only when their
        // planes are COMPLETELY orthogonal — isoclinic — which `Cl(4,1)`
        // has room for and three spatial dimensions do not.
        let isoclinic = cga::rotator_on_blade(&q(1), &q(1), cga::E3 | cga::EP);
        assert!(
            !cga::bivector_is_simple(&r1.geo(&isoclinic)),
            "e₁₂ against e₃e₊ spans a four-volume: this is the screw"
        );
    }

    /// T2 — THE INVERSIVE STRAIN IS EXACT, SIGNED AND SCALE-FREE (59.3).
    ///
    /// `μ` itself is irrational — its denominator wants `√(r_a² r_b²)` —
    /// so the pair `(num, den2)` carries it and every question is a
    /// comparison of cross products. What must hold: the three contact
    /// sides come out right, the horizon test agrees with them, and a
    /// global dilation moves nothing.
    #[test]
    fn the_strain_reads_contact_exactly_and_scale_free() {
        use crate::transducer::blade::{self, Contact, Strain};
        let q = |n: i64| crate::core::Q::from_integer(n.into());

        // A body against ITSELF is `δ = −1`, internal tangency: `‖ΔP‖ = 0`
        // and `num = −2r²`, so `num² = 4r⁴ = den2`. Deep inside contact.
        let a = blade::shape_of(&[(0, 0, 0), (1, 0, 0)]).blade;
        let s = Strain::between(&a, &a).expect("a body has a centroid");
        assert_eq!(s.side(), Contact::Through, "a body against itself is inside");
        assert!(s.num < q(0), "and its numerator is negative");

        // FAR APART. Two dominoes a long way from one another: `‖ΔP‖²`
        // dominates, `δ > 1`.
        let far = blade::shape_of(&[(0, 100, 0), (1, 100, 0)]).blade;
        let s_far = Strain::between(&a, &far).expect("two centroids");
        assert_eq!(s_far.side(), Contact::Apart, "distant bodies are separated");

        // THE HORIZON ORDERS THEM, and the ordering is the physics: a
        // distant pair is outside every horizon a near pair is inside.
        let near = blade::shape_of(&[(0, 2, 0), (1, 2, 0)]).blade;
        let s_near = Strain::between(&a, &near).expect("two centroids");
        for mu_c in [q(1), q(4), q(16)] {
            assert!(
                !(s_far.within(&mu_c) && !s_near.within(&mu_c)),
                "no horizon admits the far pair while refusing the near one"
            );
        }
        assert!(s_near.within(&q(16)), "a generous horizon admits the near pair");
        assert!(!s_far.within(&q(16)), "and still refuses the far one");

        // SCALE-FREE, which is Axiom C's whole point: `δ² = num²/den2` is
        // invariant under `x ↦ λx` because `num ↦ λ²num` and
        // `den2 ↦ λ⁴den2`. Doubling every coordinate must not move the
        // reading or the side.
        let a2 = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (0, 1, 0), (1, 1, 0)]).blade;
        let far2 = blade::shape_of(&[
            (0, 200, 0),
            (1, 200, 0),
            (0, 201, 0),
            (1, 201, 0),
        ])
        .blade;
        let scaled = Strain::between(&a2, &far2).expect("two centroids");
        assert_eq!(scaled.side(), s_far.side(), "a dilation does not change the side");

        // AND NO ROOT WAS TAKEN ANYWHERE: `den2` is a square and is never
        // rooted; it is strictly positive because `radius2` is.
        assert!(s.den2 > q(0), "the denominator squared is strictly positive");
        assert!(s_far.den2 > q(0));
    }

    /// T3 — THE STRAIN IS A BILINEAR FORM, NOT A PAIRWISE TEST.
    ///
    /// `Strain::between` subtracts centroids and squares, which reads as a
    /// pairwise computation and invites a pairwise walk. It is not one. On
    /// the conformal representative `s = P − ½r²e_∞` the numerator is an
    /// INNER PRODUCT:
    ///
    /// ```text
    ///     s_a · s_b = ½(r_a² + r_b² − ‖P_a − P_b‖²)
    ///     num       = −2 (s_a · s_b)
    /// ```
    ///
    /// which is LINEAR IN EACH ARGUMENT. That is what T3 needs and what a
    /// bucketing scheme would destroy: "which bodies lie within `A`'s
    /// horizon" is a linear functional evaluated against the carrier, so a
    /// keyed descent is possible exactly when the key's coarse strata are
    /// level sets of that functional. Quantising `⌊log₂ r⌋` or `P mod 2^k`
    /// answers it with a grid instead, which is `ring_of_support` at a
    /// coarser pitch — and `⌊log₂⌋` is not even exact in ℚ.
    ///
    /// The identity is asserted here so the linear form is a measured fact
    /// before anything is built on it.
    #[test]
    fn the_strain_numerator_is_an_inner_product_of_conformal_spheres() {
        use crate::transducer::blade::{self, Strain};
        use crate::transducer::cga;
        let q = |n: i64| crate::core::Q::from_integer(n.into());
        let two = q(2);

        let bodies = [
            blade::shape_of(&[(0, 0, 0), (1, 0, 0)]).blade,
            blade::shape_of(&[(0, 5, 0), (1, 5, 0), (0, 6, 0)]).blade,
            blade::shape_of(&[(3, 3, 0)]).blade,
        ];

        for a in &bodies {
            for b in &bodies {
                let s = Strain::between(a, b).expect("both centroids read");
                let (pa, pb) = (a.centroid().expect("a"), b.centroid().expect("b"));
                let sa = cga::sphere(&pa.0, &pa.1, &pa.2, &a.radius2());
                let sb = cga::sphere(&pb.0, &pb.1, &pb.2, &b.radius2());
                // The scalar part of the geometric product IS the inner
                // product of two vectors.
                let dot = sa.geo(&sb).scalar_part();
                assert_eq!(
                    s.num,
                    -&two * &dot,
                    "num = −2 (s_a · s_b): the strain is bilinear in the spheres"
                );
            }
        }
    }

    /// T3 — THE STRAIN DEPENDS ONLY ON THE RELATIVE MOTOR (ruled
    /// 2026-09-22).
    ///
    /// A body's nucleus is `s = M s₀ M̃` for a motor `M ∈ Spin⁺(4,1)(ℚ)`,
    /// and a versor is an orthogonal transformation of `ℚ^{4,1}`, so it
    /// preserves the inner product. Therefore
    ///
    /// ```text
    ///     (M s_a M̃) · (M s_b M̃) = s_a · s_b
    /// ```
    ///
    /// and `num = −2(s_a · s_b)` is a function of `M_a⁻¹ M_b` ALONE —
    /// never of where either body sits absolutely. Absolute position is a
    /// gauge artifact of the strain.
    ///
    /// WHAT THAT COLLAPSES. A spatial index would answer "which bodies are
    /// near me"; the spellable horizon answers "can my mind factor `g_ab`".
    /// Since the relative motor IS `g_ab`, those are one question, and T3
    /// is not an index bolted beside the physics — proximity is membership
    /// of the relative motor in the alphabet's spellable neighbourhood.
    ///
    /// Asserted here on a motor with BOTH halves — a turn composed with a
    /// displacement — because a translation-only fixture would pass on the
    /// commuting case and say nothing about the screw.
    #[test]
    fn the_strain_is_invariant_under_a_common_motor() {
        use crate::transducer::blade::{self, Strain};
        use crate::transducer::cga;
        let q = |n: i64| crate::core::Q::from_integer(n.into());

        let bodies = [
            blade::shape_of(&[(0, 0, 0), (1, 0, 0)]).blade,
            blade::shape_of(&[(4, 7, 0), (5, 7, 0), (4, 8, 0)]).blade,
            blade::shape_of(&[(-3, 2, 0)]).blade,
        ];
        let nucleus = |b: &blade::Blade| {
            let p = b.centroid().expect("a centroid");
            cga::sphere(&p.0, &p.1, &p.2, &b.radius2())
        };

        // A motor with a turn AND a displacement, so the two halves do not
        // commute and the test is not a translation identity in disguise.
        let motor = cga::translator(&q(11), &q(-4), &q(2)).geo(&cga::rotator(&q(1), &q(1)));

        // THE INVARIANCE IS PROJECTIVE, NOT ON THE NOSE, and a first
        // statement of it claimed otherwise. A versor preserves the inner
        // product only when it is ORTHOGONAL — `M M̃ = 1` — and in ℚ a
        // versor cannot be normalised without a square root, so every motor
        // here is a RAY. `rotator(1,1) = 1 + e₁₂` has `M M̃ = 2`, and the
        // sandwich scales the inner product by `λ² = 4`:
        //
        //     num ↦ λ² num,   den2 ↦ λ⁴ den2,   δ² = num²/den2 unchanged
        //
        // which is 57.2's ray exactly — "canonicalization is not a
        // homomorphism" — and is why Axiom C carries the ratio and never
        // evaluates it. The scale-free reading is invariant; the numerator
        // alone is not, and a design that keyed on `num` would key on the
        // gauge.
        let lambda = motor.geo(&motor.reverse()).scalar_part();
        assert_eq!(lambda, q(2), "the fixture's motor is a ray of norm 2, not a unit");
        for a in &bodies {
            for b in &bodies {
                let (sa, sb) = (nucleus(a), nucleus(b));
                let before = sa.geo(&sb).scalar_part();
                let after = motor.sandwich(&sa).geo(&motor.sandwich(&sb)).scalar_part();
                assert_eq!(
                    after,
                    &lambda * &lambda * &before,
                    "a common motor scales the strain by its own norm squared"
                );
                // AND THE SCALE-FREE READING SURVIVES EXACTLY. `num²/den2`
                // is what the horizon is taken on, and it does not move.
                let (ra2, rb2) = (sa.geo(&sa).scalar_part(), sb.geo(&sb).scalar_part());
                let (ra2m, rb2m) = (
                    motor.sandwich(&sa).geo(&motor.sandwich(&sa)).scalar_part(),
                    motor.sandwich(&sb).geo(&motor.sandwich(&sb)).scalar_part(),
                );
                let delta2 = |n: &crate::core::Q, x: &crate::core::Q, y: &crate::core::Q| {
                    let num = -q(2) * n;
                    &num * &num / (q(4) * x * y)
                };
                assert_eq!(
                    delta2(&before, &ra2, &rb2),
                    delta2(&after, &ra2m, &rb2m),
                    "δ² = num²/den2 is invariant under the motor, on the nose"
                );
            }
        }

        // AND THE SIGN CONVENTION THE WHOLE THING RESTS ON. `s² = +r² > 0`:
        // the nucleus is SPACELIKE, which is the dual-sphere chart where
        // contact is reachable. On the `v² = −1` hyperboloid `μ = sinh²(d/2)
        // ≥ 0` and tangency is unreachable — that is 59.2's finding, and a
        // statement that the nucleus has `s² = −r²` would put it back
        // there.
        for b in &bodies {
            let s = nucleus(b);
            let r2 = b.radius2();
            assert_eq!(s.geo(&s).scalar_part(), r2, "s² = +r², spacelike");
            assert!(r2 > q(0), "and every extracted body is invertible");
        }

        // A BODY AGAINST ITSELF IS THE IDENTITY MOTOR, and that is internal
        // tangency rather than "no relation": δ = −1, num = −2r², num² =
        // den2.
        let s = Strain::between(&bodies[0], &bodies[0]).expect("a centroid");
        assert_eq!(&s.num * &s.num, s.den2, "M_ab = 1 sits exactly on δ = −1");
    }

    /// THE HARMONIC CORE IS WHAT KEEPS THE NUCLEUS OFF THE NULL CONE
    /// (ruled 2026-09-22).
    ///
    /// The luminal sector is an ENVELOPE: the projective ray wraps the
    /// harmonic body, and it can only do so while the body holds itself
    /// open. `s² = +r²`, so `r² → 0` drops the nucleus onto the light cone,
    /// `den2 = 4 r_a² r_b² → 0`, and the inversive distance diverges — the
    /// interaction horizon stops being defined at all.
    ///
    /// THE SMALLEST POSSIBLE BODY IS THE TEST. One lamina is the least
    /// harmonic mass the sensor can deliver, and it is exactly where a
    /// point-mass reading would give `r² = 0`. It does not, because
    /// `blade::lift` INTEGRATES THE LAMINA'S OWN UNIT SQUARE and carries
    /// the parallel-axis term at every order — which is the shadow trace's
    /// retirement (A.11's single producer) doing structural work rather
    /// than cosmetic work. A second producer taking cells as point masses
    /// would have collapsed the nucleus onto the cone.
    #[test]
    fn the_smallest_body_still_holds_its_nucleus_off_the_cone() {
        use crate::transducer::blade;
        let q = |n: i64| crate::core::Q::from_integer(n.into());
        let sixth = q(1) / q(6);

        // A SINGLE LAMINA — the least mass there is.
        let one = blade::shape_of(&[(0, 0, 0)]).blade;
        let r2 = one.radius2();
        assert!(
            r2 >= sixth,
            "one lamina integrates its own square: r² = {r2} ≥ 1/6, never 0"
        );

        // A point-mass reading would put this at zero. It does not, so the
        // nucleus is spacelike and `den2` for a pair of them is bounded
        // away from zero.
        let s = blade::Strain::between(&one, &one).expect("a centroid");
        assert!(s.den2 >= q(4) * &sixth * &sixth, "den2 = 4 r_a² r_b² is bounded below");
        assert!(s.den2 > q(0), "so the horizon is always defined");
    }

    /// A MOTOR OF NEGATIVE NORM DOES NOT FLIP THE CONTACT SIDE (ruled
    /// 2026-09-22, and left open as an argument until measured).
    ///
    /// `Cl(4,1)` is INDEFINITE — `e₋² = −1` — so a versor built from an odd
    /// number of timelike reflections has `λ = M M̃ < 0`, which cannot
    /// happen in a definite signature. 64.5 left it open whether the side
    /// of the strain survives such a motor, because the sign of `num` is
    /// what `Strain::side` splits on before squaring.
    ///
    /// It does survive, and here is why: the sandwich `M x M̃` is `λ` times
    /// an orthogonal map, so the inner product picks up `λ²` — and `λ² > 0`
    /// for every `λ ∈ ℚ×`, negative ones included. The SIDE is therefore an
    /// absolute geometric invariant even through an indefinite reflection
    /// channel, while `num` itself remains gauge.
    #[test]
    fn a_negative_norm_motor_scales_the_strain_and_never_flips_its_side() {
        use crate::transducer::blade;
        use crate::transducer::cga::{self, Mv, E1, EM};
        let q = |n: i64| crate::core::Q::from_integer(n.into());

        // `e₁e₋`: one spacelike and one timelike direction, so
        // `M M̃ = e₁ e₋ e₋ e₁ = e₁ (−1) e₁ = −1`.
        let motor = Mv::blade(E1, q(1)).geo(&Mv::blade(EM, q(1)));
        let lambda = motor.geo(&motor.reverse()).scalar_part();
        assert_eq!(lambda, q(-1), "an indefinite signature admits a NEGATIVE versor norm");
        assert!(lambda < q(0), "which a definite signature never could");

        let bodies = [
            blade::shape_of(&[(0, 0, 0), (1, 0, 0)]).blade,
            blade::shape_of(&[(0, 9, 0), (1, 9, 0)]).blade,
            blade::shape_of(&[(0, 1, 0)]).blade,
        ];
        let nucleus = |b: &blade::Blade| {
            let p = b.centroid().expect("a centroid");
            cga::sphere(&p.0, &p.1, &p.2, &b.radius2())
        };

        for a in &bodies {
            for b in &bodies {
                let (sa, sb) = (nucleus(a), nucleus(b));
                let before = sa.geo(&sb).scalar_part();
                let after = motor.sandwich(&sa).geo(&motor.sandwich(&sb)).scalar_part();
                assert_eq!(
                    after,
                    &lambda * &lambda * &before,
                    "the inner product picks up λ², never λ"
                );
                // AND THAT IS THE WHOLE POINT: `λ² > 0` even for `λ < 0`,
                // so the sign the side is read from cannot invert.
                assert_eq!(
                    before > q(0),
                    after > q(0),
                    "a negative-norm motor does not move the contact side"
                );
            }
        }
        assert!(&lambda * &lambda > q(0), "λ² is positive for every λ in ℚ×");
    }
}
